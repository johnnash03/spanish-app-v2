//! The generate → validate → repair pipeline for one unit's bank (S5,
//! #36).
//!
//! Items stream out of the generation model and are validated the moment
//! they are complete; passing items are persisted immediately (streaming
//! persistence, v1 behavior). Failing items go into the next repair round
//! with their violations named verbatim. Rounds are bounded: a slot that
//! cannot be repaired is abandoned and logged, never banked.

use super::plan::{plan_bank, LearnerSnapshot};
use super::prompt::{build_generation_message, build_repair_message, STABLE_SYSTEM_PROMPT};
use super::types::{BankItem, GeneratedItem, SlotFailure, ValidatedVariant};
use crate::v2::curriculum::types::TargetSpec;
use crate::v2::curriculum::Curriculum;
use crate::v2::validator::{
    judge, validate, Analyzer, CandidateItem, ExistingItem, JudgeContext, Verdict,
};
use std::collections::BTreeSet;
use thiserror::Error;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

#[derive(Debug, Error)]
pub enum GeneratorError {
    #[error("unknown unit `{0}`")]
    UnknownUnit(String),
    #[error("generation source error: {0}")]
    Source(String),
    #[error("bank sink error: {0}")]
    Sink(String),
}

/// A source of generated items: the OpenAI streaming client in
/// production, canned rounds in tests. Implementations send each item the
/// moment it is complete — the pipeline validates and persists while the
/// stream is still running.
pub trait ItemSource {
    fn stream_items(
        &self,
        system: &str,
        user: &str,
        tx: UnboundedSender<GeneratedItem>,
    ) -> impl std::future::Future<Output = Result<(), GeneratorError>> + Send;
}

/// Where validated items land: the v2 SQLite bank in production, memory in
/// tests and the dev binary.
pub trait BankSink {
    fn persist(&self, item: &BankItem) -> Result<(), String>;
}

pub struct PipelineConfig {
    pub items_per_unit: usize,
    /// Repair rounds after the initial generation round.
    pub max_repair_rounds: usize,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            items_per_unit: 20,
            max_repair_rounds: 2,
        }
    }
}

#[derive(Debug)]
pub struct GenerationOutcome {
    pub banked: usize,
    /// Rounds actually run (1 = everything passed first try).
    pub rounds: usize,
    /// Slots that never produced a valid item, with their last violations.
    pub abandoned: Vec<SlotFailure>,
}

/// Runs the full pipeline for one unit. `existing` and `existing_sources`
/// seed the near-duplication context from any previously banked items.
pub async fn generate_unit_bank<S: ItemSource, A: Analyzer, K: BankSink>(
    source: &S,
    analyzer: &A,
    sink: &K,
    c: &Curriculum,
    unit_id: &str,
    learner: &LearnerSnapshot,
    mut existing: Vec<ExistingItem>,
    mut existing_sources: Vec<String>,
    cfg: &PipelineConfig,
) -> Result<GenerationOutcome, GeneratorError> {
    let plans = plan_bank(c, unit_id, learner, cfg.items_per_unit)
        .ok_or_else(|| GeneratorError::UnknownUnit(unit_id.to_string()))?;
    let licensing = c.effective_licensing(unit_id).expect("plan_bank checked");
    let target = c.target_spec(unit_id).expect("every unit has a target");
    let registry = c.construction_registry();

    let mut pending: Vec<SlotFailure> = plans
        .into_iter()
        .map(|plan| SlotFailure {
            plan,
            attempt: None,
            violations: vec![],
        })
        .collect();
    let mut banked = 0usize;
    let mut rounds = 0usize;

    for round in 0..=cfg.max_repair_rounds {
        if pending.is_empty() {
            break;
        }
        rounds += 1;

        let user_msg = if round == 0 {
            let plans: Vec<_> = pending.iter().map(|f| f.plan.clone()).collect();
            build_generation_message(c, unit_id, &plans, learner, &existing_sources)
        } else {
            build_repair_message(c, unit_id, &pending, learner, &existing_sources)
        }
        .expect("unit existence checked above");

        let (tx, mut rx) = unbounded_channel();
        let producer = source.stream_items(STABLE_SYSTEM_PROMPT, &user_msg, tx);

        // Consume while the producer is still streaming: validate each
        // item as it completes and persist it the moment it passes.
        let consumer = async {
            let mut resolved: BTreeSet<u32> = BTreeSet::new();
            let mut next_round: Vec<SlotFailure> = Vec::new();
            while let Some(item) = rx.recv().await {
                let Some(pos) = pending
                    .iter()
                    .position(|f| f.plan.slot_id == item.slot_id)
                    .filter(|_| !resolved.contains(&item.slot_id))
                else {
                    eprintln!(
                        "[gen {unit_id}] ignoring item for unrequested slot {}",
                        item.slot_id
                    );
                    continue;
                };
                resolved.insert(item.slot_id);
                let plan = pending[pos].plan.clone();

                let stacked_targets: Vec<(String, TargetSpec)> = plan
                    .tags
                    .stacked
                    .iter()
                    .filter_map(|s| c.target_spec(s).map(|t| (s.clone(), t.clone())))
                    .collect();
                let candidate = CandidateItem {
                    source: item.source.clone(),
                    canonical: item.canonical.clone(),
                };
                let ctx = JudgeContext {
                    licensing,
                    target,
                    construction_registry: &registry,
                    window: &plan.legal_window,
                    existing: &existing,
                    slot: Some(&plan.spec),
                    stacked_targets: &stacked_targets,
                };
                match validate(analyzer, &candidate, &ctx).await {
                    Verdict::Pass { analysis } => {
                        // Variants are judged under the same positive spec
                        // (licensing + target structure); a failing variant
                        // is dropped, never banked and never fatal.
                        let mut variants = Vec::new();
                        for text in &item.variants {
                            let v_candidate = CandidateItem {
                                source: item.source.clone(),
                                canonical: text.clone(),
                            };
                            let v_ctx = JudgeContext {
                                licensing,
                                target,
                                construction_registry: &registry,
                                window: &plan.legal_window,
                                existing: &[],
                                slot: None,
                                stacked_targets: &[],
                            };
                            match analyzer.analyze(&v_candidate).await {
                                Ok(v_analysis) => {
                                    let violations = judge(&v_candidate, &v_analysis, &v_ctx);
                                    if violations.is_empty() {
                                        variants.push(ValidatedVariant {
                                            text: text.clone(),
                                            analysis: v_analysis,
                                        });
                                    } else {
                                        eprintln!(
                                            "[gen {unit_id}] slot {} variant \"{text}\" dropped: {}",
                                            plan.slot_id,
                                            serde_json::to_string(&violations).unwrap()
                                        );
                                    }
                                }
                                Err(e) => eprintln!(
                                    "[gen {unit_id}] slot {} variant \"{text}\" dropped: \
                                     analysis failed: {e}",
                                    plan.slot_id
                                ),
                            }
                        }

                        let bank_item = BankItem {
                            id: item_id(plan.slot_id),
                            unit_id: unit_id.to_string(),
                            source: item.source.clone(),
                            canonical: item.canonical.clone(),
                            variants,
                            slot: plan.spec.clone(),
                            tags: plan.tags.clone(),
                            analysis,
                        };
                        sink.persist(&bank_item).map_err(GeneratorError::Sink)?;
                        banked += 1;
                        eprintln!(
                            "[gen {unit_id}] slot {} banked: \"{}\"",
                            plan.slot_id, item.canonical
                        );
                        existing.push(ExistingItem {
                            id: bank_item.id,
                            canonical: bank_item.canonical,
                        });
                        existing_sources.push(item.source.clone());
                    }
                    Verdict::Rejected { violations } => {
                        eprintln!(
                            "[gen {unit_id}] slot {} rejected (round {}): \"{}\" — {}",
                            plan.slot_id,
                            round + 1,
                            item.canonical,
                            serde_json::to_string(&violations).unwrap()
                        );
                        next_round.push(SlotFailure {
                            plan,
                            attempt: Some(item),
                            violations,
                        });
                    }
                }
            }
            Ok::<(Vec<SlotFailure>, BTreeSet<u32>), GeneratorError>((next_round, resolved))
        };

        let (produced, consumed) = tokio::join!(producer, consumer);
        produced?;
        let (mut next_round, resolved) = consumed?;

        // Slots the model never answered stay pending with no attempt.
        for f in std::mem::take(&mut pending) {
            if !resolved.contains(&f.plan.slot_id) {
                eprintln!(
                    "[gen {unit_id}] slot {} missing from round {} output",
                    f.plan.slot_id,
                    round + 1
                );
                next_round.push(SlotFailure {
                    plan: f.plan,
                    attempt: None,
                    violations: vec![],
                });
            }
        }
        pending = next_round;
    }

    if !pending.is_empty() {
        eprintln!(
            "[gen {unit_id}] abandoning {} slot(s) after {rounds} round(s)",
            pending.len()
        );
    }
    Ok(GenerationOutcome {
        banked,
        rounds,
        abandoned: pending,
    })
}

/// Unique-enough id for a single-user local bank.
fn item_id(slot_id: u32) -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("itm-{nanos:x}-{slot_id}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2::curriculum;
    use crate::v2::validator::{AnalyzedVerbForm, AnalyzerError, ItemAnalysis, Violation};
    use std::collections::BTreeMap;
    use std::sync::Mutex;

    /// Canned rounds: each `stream_items` call sends the next round's
    /// items and records the user message it was asked with.
    struct StubSource {
        rounds: Mutex<std::collections::VecDeque<Vec<GeneratedItem>>>,
        seen_messages: Mutex<Vec<String>>,
    }

    impl StubSource {
        fn new(rounds: Vec<Vec<GeneratedItem>>) -> Self {
            Self {
                rounds: Mutex::new(rounds.into()),
                seen_messages: Mutex::new(vec![]),
            }
        }
    }

    impl ItemSource for StubSource {
        async fn stream_items(
            &self,
            _system: &str,
            user: &str,
            tx: UnboundedSender<GeneratedItem>,
        ) -> Result<(), GeneratorError> {
            self.seen_messages.lock().unwrap().push(user.to_string());
            let round = self.rounds.lock().unwrap().pop_front().unwrap_or_default();
            for item in round {
                tx.send(item).expect("consumer alive");
            }
            Ok(())
        }
    }

    /// Canonical-text → canned analysis. Anything unmapped is an analyzer
    /// transport error (which must reject, fail-safe).
    struct StubAnalyzer(BTreeMap<String, ItemAnalysis>);

    impl Analyzer for StubAnalyzer {
        async fn analyze(&self, item: &CandidateItem) -> Result<ItemAnalysis, AnalyzerError> {
            self.0
                .get(&item.canonical)
                .cloned()
                .ok_or_else(|| AnalyzerError::Transport(format!("no analysis for `{}`", item.canonical)))
        }
    }

    #[derive(Default)]
    struct MemorySink(Mutex<Vec<BankItem>>);

    impl BankSink for MemorySink {
        fn persist(&self, item: &BankItem) -> Result<(), String> {
            self.0.lock().unwrap().push(item.clone());
            Ok(())
        }
    }

    fn avf(lemma: &str, form: &str, surface: &str) -> AnalyzedVerbForm {
        AnalyzedVerbForm {
            lemma: lemma.into(),
            form: form.into(),
            surface: surface.into(),
        }
    }

    /// A licensed, on-target opener.quiero analysis for `inf_lemma`.
    fn opener_analysis(inf_lemma: &str, surface: &str) -> ItemAnalysis {
        ItemAnalysis {
            verb_forms: vec![avf("querer", "pres.1sg", "quiero"), avf(inf_lemma, "inf", surface)],
            constructions: vec!["opener.finite+inf".into()],
            content_lemmas: vec![],
        }
    }

    fn negated(mut analysis: ItemAnalysis) -> ItemAnalysis {
        analysis.constructions.push("neg.no.preverbal".into());
        analysis
    }

    fn item(slot_id: u32, source: &str, canonical: &str, variants: &[&str]) -> GeneratedItem {
        GeneratedItem {
            slot_id,
            source: source.into(),
            canonical: canonical.into(),
            variants: variants.iter().map(|v| v.to_string()).collect(),
        }
    }

    /// Three opener.quiero slots: 0/1 affirmative, 2 negative (plan cadence).
    fn run_config() -> PipelineConfig {
        PipelineConfig {
            items_per_unit: 3,
            max_repair_rounds: 2,
        }
    }

    async fn run(
        source: &StubSource,
        analyzer: &StubAnalyzer,
        sink: &MemorySink,
        cfg: &PipelineConfig,
    ) -> GenerationOutcome {
        let c = curriculum::load_embedded().unwrap();
        generate_unit_bank(
            source,
            analyzer,
            sink,
            &c,
            "opener.quiero",
            &LearnerSnapshot::default(),
            vec![],
            vec![],
            cfg,
        )
        .await
        .unwrap()
    }

    #[tokio::test]
    async fn clean_round_banks_everything_with_validated_variants() {
        let source = StubSource::new(vec![vec![
            item(0, "I want to eat.", "Quiero comer.", &["Yo quiero comer."]),
            item(1, "I want to dance.", "Quiero bailar.", &[]),
            item(2, "I don't want to work.", "No quiero trabajar.", &[]),
        ]]);
        let analyzer = StubAnalyzer(
            [
                ("Quiero comer.".to_string(), opener_analysis("comer", "comer")),
                ("Yo quiero comer.".to_string(), {
                    let mut a = opener_analysis("comer", "comer");
                    a.constructions.push("pron.subject.optional".into());
                    a
                }),
                ("Quiero bailar.".to_string(), opener_analysis("bailar", "bailar")),
                (
                    "No quiero trabajar.".to_string(),
                    negated(opener_analysis("trabajar", "trabajar")),
                ),
            ]
            .into(),
        );
        let sink = MemorySink::default();

        let outcome = run(&source, &analyzer, &sink, &run_config()).await;
        assert_eq!(outcome.banked, 3);
        assert_eq!(outcome.rounds, 1);
        assert!(outcome.abandoned.is_empty());

        let banked = sink.0.lock().unwrap();
        assert_eq!(banked.len(), 3);
        let first = &banked[0];
        assert_eq!(first.unit_id, "opener.quiero");
        assert_eq!(first.canonical, "Quiero comer.");
        assert_eq!(first.variants.len(), 1);
        assert_eq!(first.variants[0].text, "Yo quiero comer.");
        assert_eq!(first.tags.target_skill, "opener.quiero");
        assert_eq!(first.slot.person.as_deref(), Some("1sg"));
        assert!(!first.analysis.verb_forms.is_empty(), "analysis kept for inspection");
    }

    #[tokio::test]
    async fn repair_round_regenerates_only_failures_with_violations_named() {
        // Round 1: slot 1 leaks `ser` (the canonical v1 failure shape).
        // Round 2 repairs it. Slots 0 and 2 must not be re-requested.
        let source = StubSource::new(vec![
            vec![
                item(0, "I want to eat.", "Quiero comer.", &[]),
                item(1, "I want to be famous.", "Quiero ser famoso.", &[]),
                item(2, "I don't want to work.", "No quiero trabajar.", &[]),
            ],
            vec![item(1, "I want to drink.", "Quiero beber.", &[])],
        ]);
        let analyzer = StubAnalyzer(
            [
                ("Quiero comer.".to_string(), opener_analysis("comer", "comer")),
                ("Quiero ser famoso.".to_string(), {
                    let mut a = opener_analysis("ser", "ser");
                    a.content_lemmas = vec!["famoso".into()];
                    a
                }),
                (
                    "No quiero trabajar.".to_string(),
                    negated(opener_analysis("trabajar", "trabajar")),
                ),
                ("Quiero beber.".to_string(), opener_analysis("beber", "beber")),
            ]
            .into(),
        );
        let sink = MemorySink::default();

        let outcome = run(&source, &analyzer, &sink, &run_config()).await;
        assert_eq!(outcome.banked, 3);
        assert_eq!(outcome.rounds, 2);
        assert!(outcome.abandoned.is_empty());

        // Streaming persistence: round-1 passes were banked before the
        // repair round ran.
        let banked = sink.0.lock().unwrap();
        assert_eq!(banked[0].canonical, "Quiero comer.");
        assert_eq!(banked[1].canonical, "No quiero trabajar.");
        assert_eq!(banked[2].canonical, "Quiero beber.");

        // The repair message named the violation verbatim and re-requested
        // only the failed slot.
        let messages = source.seen_messages.lock().unwrap();
        assert_eq!(messages.len(), 2);
        let repair = &messages[1];
        assert!(repair.contains("REPAIR ROUND"));
        assert!(repair.contains(r#""kind":"unlicensed_verb_form""#));
        assert!(repair.contains(r#""surface":"ser""#));
        assert!(repair.contains("- Item 1:"));
        assert!(!repair.contains("- Item 0:"));
        assert!(!repair.contains("- Item 2:"));
    }

    #[tokio::test]
    async fn bounded_retries_abandon_incorrigible_slots() {
        // The model insists on `ser` every round; the slot must be
        // abandoned after the configured rounds, never banked.
        let bad = || item(1, "I want to be famous.", "Quiero ser famoso.", &[]);
        let good0 = item(0, "I want to eat.", "Quiero comer.", &[]);
        let good2 = item(2, "I don't want to work.", "No quiero trabajar.", &[]);
        let source = StubSource::new(vec![
            vec![good0, bad(), good2],
            vec![bad()],
            vec![bad()],
        ]);
        let analyzer = StubAnalyzer(
            [
                ("Quiero comer.".to_string(), opener_analysis("comer", "comer")),
                ("Quiero ser famoso.".to_string(), opener_analysis("ser", "ser")),
                (
                    "No quiero trabajar.".to_string(),
                    negated(opener_analysis("trabajar", "trabajar")),
                ),
            ]
            .into(),
        );
        let sink = MemorySink::default();

        let outcome = run(&source, &analyzer, &sink, &run_config()).await;
        assert_eq!(outcome.banked, 2);
        assert_eq!(outcome.rounds, 3, "initial + both repair rounds");
        assert_eq!(outcome.abandoned.len(), 1);
        assert_eq!(outcome.abandoned[0].plan.slot_id, 1);
        assert!(outcome.abandoned[0]
            .violations
            .iter()
            .any(|v| matches!(v, Violation::UnlicensedVerbForm { lemma, .. } if lemma == "ser")));
        assert_eq!(sink.0.lock().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn unanswered_slots_are_re_requested_then_abandoned() {
        // The model only ever answers slot 0.
        let source = StubSource::new(vec![
            vec![item(0, "I want to eat.", "Quiero comer.", &[])],
            vec![],
            vec![],
        ]);
        let analyzer = StubAnalyzer(
            [("Quiero comer.".to_string(), opener_analysis("comer", "comer"))].into(),
        );
        let sink = MemorySink::default();

        let outcome = run(&source, &analyzer, &sink, &run_config()).await;
        assert_eq!(outcome.banked, 1);
        assert_eq!(outcome.abandoned.len(), 2);
        let messages = source.seen_messages.lock().unwrap();
        assert!(messages[1].contains("no item was produced for this slot"));
    }

    #[tokio::test]
    async fn invalid_variants_are_dropped_without_failing_the_item() {
        let source = StubSource::new(vec![vec![
            item(
                0,
                "I want to eat.",
                "Quiero comer.",
                &["Yo quiero comer.", "Quiero comer mucho."],
            ),
            item(1, "I want to dance.", "Quiero bailar.", &[]),
            item(2, "I don't want to work.", "No quiero trabajar.", &[]),
        ]]);
        let analyzer = StubAnalyzer(
            [
                ("Quiero comer.".to_string(), opener_analysis("comer", "comer")),
                ("Yo quiero comer.".to_string(), opener_analysis("comer", "comer")),
                ("Quiero comer mucho.".to_string(), {
                    // `mucho` is licensed nowhere in unit 1.
                    let mut a = opener_analysis("comer", "comer");
                    a.content_lemmas = vec!["mucho".into()];
                    a
                }),
                ("Quiero bailar.".to_string(), opener_analysis("bailar", "bailar")),
                (
                    "No quiero trabajar.".to_string(),
                    negated(opener_analysis("trabajar", "trabajar")),
                ),
            ]
            .into(),
        );
        let sink = MemorySink::default();

        let outcome = run(&source, &analyzer, &sink, &run_config()).await;
        assert_eq!(outcome.banked, 3);
        let banked = sink.0.lock().unwrap();
        let texts: Vec<&str> = banked[0].variants.iter().map(|v| v.text.as_str()).collect();
        assert_eq!(texts, vec!["Yo quiero comer."], "the unlicensed variant is dropped");
    }

    #[tokio::test]
    async fn near_duplicates_within_a_run_are_rejected_and_repaired() {
        let source = StubSource::new(vec![
            vec![
                item(0, "I want to eat.", "Quiero comer.", &[]),
                item(1, "I want to eat!", "Quiero comer.", &[]),
                item(2, "I don't want to work.", "No quiero trabajar.", &[]),
            ],
            vec![item(1, "I want to dance.", "Quiero bailar.", &[])],
        ]);
        let analyzer = StubAnalyzer(
            [
                ("Quiero comer.".to_string(), opener_analysis("comer", "comer")),
                ("Quiero bailar.".to_string(), opener_analysis("bailar", "bailar")),
                (
                    "No quiero trabajar.".to_string(),
                    negated(opener_analysis("trabajar", "trabajar")),
                ),
            ]
            .into(),
        );
        let sink = MemorySink::default();

        let outcome = run(&source, &analyzer, &sink, &run_config()).await;
        assert_eq!(outcome.banked, 3);
        assert_eq!(outcome.rounds, 2);
        let messages = source.seen_messages.lock().unwrap();
        assert!(messages[1].contains(r#""kind":"near_duplicate""#));
    }
}
