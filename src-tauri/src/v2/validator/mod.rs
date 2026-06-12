//! V2 validator: the generation-time quality gate (S4, #35).
//!
//! "LLM analyzes, code judges" (PRD #31): an analyzer LLM call produces a
//! structured linguistic inventory per candidate item; deterministic code
//! judges it against the unit's effective licensing set, target spec, slot
//! spec, and existing bank. The LLM never decides whether an item is
//! licensed — it only describes the language, and any analysis failure
//! rejects the item (needless regeneration, never leaked grammar).

pub mod analyzer;
pub mod judge;
pub mod types;
mod v1_regression;

pub use analyzer::{Analyzer, AnalyzerError, OpenAiAnalyzer};
pub use judge::{judge, JudgeContext};
pub use types::*;

/// The gate's output for one candidate item.
#[derive(Debug)]
pub enum Verdict {
    /// Admitted to the bank; the analysis is kept for inspection.
    Pass { analysis: ItemAnalysis },
    /// Rejected; the violations feed the regeneration prompt.
    Rejected { violations: Vec<Violation> },
}

/// Analyze-then-judge for one candidate item. Analyzer failure is a
/// rejection carrying [`Violation::AnalysisFailed`] — there is no code
/// path from a failed analysis to a pass.
pub async fn validate<A: Analyzer>(
    analyzer: &A,
    item: &CandidateItem,
    ctx: &JudgeContext<'_>,
) -> Verdict {
    match analyzer.analyze(item).await {
        Ok(analysis) => {
            let violations = judge(item, &analysis, ctx);
            if violations.is_empty() {
                Verdict::Pass { analysis }
            } else {
                Verdict::Rejected { violations }
            }
        }
        Err(e) => Verdict::Rejected {
            violations: vec![Violation::AnalysisFailed {
                reason: e.to_string(),
            }],
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2::curriculum;
    use std::collections::BTreeSet;

    /// Stub analyzer: a canned result, no network.
    struct Stub(Result<ItemAnalysis, &'static str>);

    impl Analyzer for Stub {
        async fn analyze(&self, _item: &CandidateItem) -> Result<ItemAnalysis, AnalyzerError> {
            self.0
                .clone()
                .map_err(|e| AnalyzerError::Transport(e.into()))
        }
    }

    fn quiero_comer_item() -> CandidateItem {
        CandidateItem {
            source: "I want to eat.".into(),
            canonical: "Quiero comer.".into(),
        }
    }

    async fn run(stub: Stub) -> Verdict {
        let c = curriculum::load_embedded().unwrap();
        let registry = c.construction_registry();
        let window = BTreeSet::new();
        let ctx = JudgeContext {
            licensing: c.effective_licensing("opener.quiero").unwrap(),
            target: c.target_spec("opener.quiero").unwrap(),
            construction_registry: &registry,
            window: &window,
            existing: &[],
            slot: None,
        };
        validate(&stub, &quiero_comer_item(), &ctx).await
    }

    #[tokio::test]
    async fn licensed_analysis_passes_and_keeps_the_analysis() {
        let analysis = ItemAnalysis {
            verb_forms: vec![
                AnalyzedVerbForm {
                    lemma: "querer".into(),
                    form: "pres.1sg".into(),
                    surface: "quiero".into(),
                },
                AnalyzedVerbForm {
                    lemma: "comer".into(),
                    form: "inf".into(),
                    surface: "comer".into(),
                },
            ],
            constructions: vec!["opener.finite+inf".into()],
            content_lemmas: vec![],
        };
        match run(Stub(Ok(analysis.clone()))).await {
            Verdict::Pass { analysis: kept } => assert_eq!(kept, analysis),
            Verdict::Rejected { violations } => panic!("expected pass, got {violations:?}"),
        }
    }

    #[tokio::test]
    async fn analyzer_failure_rejects_never_passes() {
        match run(Stub(Err("model unreachable"))).await {
            Verdict::Rejected { violations } => {
                assert_eq!(violations.len(), 1);
                assert!(matches!(
                    &violations[0],
                    Violation::AnalysisFailed { reason } if reason.contains("model unreachable")
                ));
            }
            Verdict::Pass { .. } => panic!("a failed analysis must never pass"),
        }
    }

    #[tokio::test]
    async fn unlicensed_analysis_is_rejected_with_named_violations() {
        let analysis = ItemAnalysis {
            verb_forms: vec![AnalyzedVerbForm {
                lemma: "ser".into(),
                form: "pres.3sg".into(),
                surface: "es".into(),
            }],
            constructions: vec![],
            content_lemmas: vec![],
        };
        match run(Stub(Ok(analysis))).await {
            Verdict::Rejected { violations } => {
                assert!(violations.contains(&Violation::UnlicensedVerbForm {
                    lemma: "ser".into(),
                    form: "pres.3sg".into(),
                    surface: "es".into(),
                }));
            }
            Verdict::Pass { .. } => panic!("ser in unit 1 must be rejected"),
        }
    }
}
