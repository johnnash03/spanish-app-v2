//! Generator prompts (S5, #36). The system prompt is a stable string —
//! never varies between units or rounds, so the provider's prompt cache
//! holds it (v1 behavior carried over). All per-unit material — the
//! enumerated licensing set, the slot specs, the repair list — lives in
//! the user message, and it is always the enumerated positive spec, never
//! prose like "present tense only".

use super::plan::{ItemPlan, LearnerSnapshot};
use super::types::{GeneratedItem, SlotFailure};
use crate::v2::curriculum::Curriculum;
use crate::v2::validator::analyzer::CONSTRUCTION_GLOSSES;
use crate::v2::validator::{Polarity, SentenceType, SlotSpec};
use std::collections::BTreeSet;

/// Stable system prompt prefix — identical for every unit and every repair
/// round, enabling prompt caching.
pub static STABLE_SYSTEM_PROMPT: &str = r#"You are a Spanish exercise author for a translation practice app. You write English → Spanish translation items for one drill unit at a time.

THE POSITIVE SPEC — THE ONLY RULE THAT MATTERS:
The user message enumerates everything this learner has been taught: specific conjugated verb forms, grammatical constructions, and words. The Spanish you write must be assembled exclusively from that material. There is no "basic" Spanish that is always safe — if a form, construction, or word is not in the lists, it does not exist for this learner. Every item is machine-checked against the lists; anything outside them is rejected and comes back to you for repair.

ITEM SPECS:
Each requested item carries a spec. Satisfy every field:
- person: at least one verb in the sentence carries that grammatical person.
- polarity=negative: the sentence is negated (using a licensed negation).
- type=question: written as a Spanish question with ¿…? (and an English question cue).
- must use the word «w»: that word must appear. It is the item's single new element — keep every structure around it simple and well-worn.
- stack with: the sentence must genuinely exercise that prior skill too, not merely the unit's target.
- sense: build the English cue around that sense of the verb.
Every item must exercise the unit's target skill. Items must differ meaningfully from one another and from the existing cues listed — different verbs, different words, different sentence shapes, never the same sentence re-worded.

VARIANTS:
For each item, list as "variants" every alternative Spanish answer a competent learner should get instant credit for:
- clitic-placement alternates (lo quiero ver / quiero verlo) when both orders use licensed constructions,
- the explicit subject-pronoun rendering of the canonical (Yo quiero comer),
- genuine synonyms — but only from the listed words and forms.
NEVER list a variant that avoids the unit's target structure: a correct paraphrase that dodges the target skill must be left out deliberately. Variants obey the same positive spec as the canonical answer.

STYLE:
- Neutral everyday English cues; neutral Latin American Spanish (tú/ustedes, never vosotros or vos).
- Canonical answers omit subject pronouns by default.
- Sentences as short as the spec allows. Vocabulary is never the challenge unless the spec schedules a word.

OUTPUT:
A raw JSON array — no markdown fence, no wrapper object, no commentary. One object per requested item:
{"slot_id": <requested slot id>, "source": "<English cue>", "canonical": "<Spanish answer>", "variants": ["<alternative>", ...]}"#;

/// Human-readable spec line for one planned item, shared by the initial
/// and repair messages so the model sees identical constraints each round.
fn spec_line(c: &Curriculum, plan: &ItemPlan) -> String {
    let mut parts: Vec<String> = Vec::new();
    let spec: &SlotSpec = &plan.spec;
    if let Some(p) = &spec.person {
        parts.push(format!("person={p}"));
    }
    parts.push(match spec.polarity {
        Some(Polarity::Negative) => "polarity=negative".into(),
        _ => "polarity=affirmative".into(),
    });
    parts.push(match spec.sentence_type {
        Some(SentenceType::Question) => "type=question".into(),
        _ => "type=declarative".into(),
    });
    if let Some(w) = &spec.required_lemma {
        parts.push(format!("must use the word «{w}»"));
    }
    if let Some(s) = &spec.sense {
        parts.push(format!("sense: {s}"));
    }
    for skill in &plan.tags.stacked {
        let title = c.unit(skill).map(|u| u.title.as_str()).unwrap_or("");
        parts.push(format!("stack with: {skill} — {title}"));
    }
    format!("- Item {}: {}", plan.slot_id, parts.join(", "))
}

/// The enumerated licensing context for a unit: verb forms, open form
/// slots, constructions with glosses, and the legal word pool (licensed
/// vocabulary plus the learner's consolidated words).
fn licensing_sections(
    c: &Curriculum,
    unit_id: &str,
    learner: &LearnerSnapshot,
    plans: &[&ItemPlan],
) -> Option<String> {
    let unit = c.unit(unit_id)?;
    let licensing = c.effective_licensing(unit_id)?;
    let mut msg = format!("Unit: {} — {}\n", unit.id, unit.title);

    msg.push_str("\nLICENSED VERB FORMS (complete list — no other verb form exists):\n");
    for vf in &licensing.verb_forms {
        msg.push_str(&format!("- {} ({}, {})\n", vf.surface, vf.lemma, vf.form));
    }
    if !licensing.vocab_verb_forms.is_empty() {
        msg.push_str("Open form slots — any licensed vocabulary verb may also appear as:\n");
        for g in &licensing.vocab_verb_forms {
            match &g.classes {
                Some(classes) => msg.push_str(&format!(
                    "- {} (only -{} verbs)\n",
                    g.form,
                    classes.join("/-")
                )),
                None => msg.push_str(&format!("- {}\n", g.form)),
            }
        }
    }

    msg.push_str("\nLICENSED CONSTRUCTIONS (complete list):\n");
    for (tag, gloss) in CONSTRUCTION_GLOSSES {
        if licensing.constructions.contains(*tag) {
            msg.push_str(&format!("- {tag} — {gloss}\n"));
        }
    }

    let mut words: BTreeSet<&str> = licensing.vocab.iter().map(String::as_str).collect();
    words.extend(learner.consolidated.iter().map(String::as_str));
    msg.push_str("\nLICENSED WORDS (complete list — articles, pronouns, vocabulary):\n");
    msg.push_str(&words.into_iter().collect::<Vec<_>>().join(", "));
    msg.push('\n');

    let scheduled: Vec<&&ItemPlan> = plans
        .iter()
        .filter(|p| p.spec.required_lemma.is_some())
        .collect();
    if !scheduled.is_empty() {
        msg.push_str(
            "\nWINDOW WORDS (new vocabulary — each may appear ONLY in the item that requires it):\n",
        );
        for p in scheduled {
            msg.push_str(&format!(
                "- {} (item {} only)\n",
                p.spec.required_lemma.as_deref().unwrap_or(""),
                p.slot_id
            ));
        }
    }
    Some(msg)
}

fn existing_section(existing_sources: &[String]) -> String {
    if existing_sources.is_empty() {
        return String::new();
    }
    let mut msg = String::from("\nEXISTING CUES TO AVOID (write nothing close to these):\n");
    for s in existing_sources {
        msg.push_str(&format!("- \"{s}\"\n"));
    }
    msg
}

/// The initial generation request for a unit's bank.
pub fn build_generation_message(
    c: &Curriculum,
    unit_id: &str,
    plans: &[ItemPlan],
    learner: &LearnerSnapshot,
    existing_sources: &[String],
) -> Option<String> {
    let refs: Vec<&ItemPlan> = plans.iter().collect();
    let mut msg = licensing_sections(c, unit_id, learner, &refs)?;
    msg.push_str(&format!("\nITEMS TO WRITE ({}):\n", plans.len()));
    for plan in plans {
        msg.push_str(&spec_line(c, plan));
        msg.push('\n');
    }
    msg.push_str(&existing_section(existing_sources));
    Some(msg)
}

/// A repair-round request: full licensing context again (the model must
/// re-see the positive spec), then only the failed slots, each with its
/// rejected attempt and the judge's violations quoted verbatim — the
/// machine-readable contract from
/// [`crate::v2::validator::Violation`]'s serialization.
pub fn build_repair_message(
    c: &Curriculum,
    unit_id: &str,
    failures: &[SlotFailure],
    learner: &LearnerSnapshot,
    existing_sources: &[String],
) -> Option<String> {
    let refs: Vec<&ItemPlan> = failures.iter().map(|f| &f.plan).collect();
    let mut msg = licensing_sections(c, unit_id, learner, &refs)?;
    msg.push_str(&format!(
        "\nREPAIR ROUND — {} item(s) were rejected by the validator. Rewrite ONLY these \
         items: same slot specs, new sentences that fix the named violations. Same JSON \
         output format, same slot_ids.\n",
        failures.len()
    ));
    for f in failures {
        msg.push_str(&spec_line(c, &f.plan));
        msg.push('\n');
        match &f.attempt {
            Some(GeneratedItem { source, canonical, .. }) => {
                msg.push_str(&format!(
                    "  rejected attempt: \"{canonical}\" (cue: \"{source}\")\n  violations: {}\n",
                    serde_json::to_string(&f.violations).expect("violations serialize"),
                ));
            }
            None => msg.push_str("  no item was produced for this slot — write it now.\n"),
        }
    }
    msg.push_str(&existing_section(existing_sources));
    Some(msg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2::curriculum;
    use crate::v2::generator::plan::plan_bank;
    use crate::v2::validator::Violation;

    #[test]
    fn system_prompt_is_stable_and_carries_the_contract() {
        assert!(STABLE_SYSTEM_PROMPT.contains("POSITIVE SPEC"));
        assert!(STABLE_SYSTEM_PROMPT.contains("VARIANTS"));
        assert!(STABLE_SYSTEM_PROMPT.contains("slot_id"));
        assert!(
            STABLE_SYSTEM_PROMPT.contains("NEVER list a variant that avoids"),
            "structure-avoiding variants are deliberately excluded (PRD)"
        );
        assert!(
            !STABLE_SYSTEM_PROMPT.contains("present tense"),
            "the system prompt must never describe licensing in prose"
        );
    }

    #[test]
    fn generation_message_enumerates_the_positive_spec() {
        let c = curriculum::load_embedded().unwrap();
        let learner = LearnerSnapshot::default();
        let plans = plan_bank(&c, "opener.quiero", &learner, 6).unwrap();
        let msg =
            build_generation_message(&c, "opener.quiero", &plans, &learner, &[]).unwrap();

        // Enumerated forms, not tense names.
        assert!(msg.contains("- quiero (querer, pres.1sg)"));
        assert!(msg.contains("Open form slots"));
        // Licensed construction with its gloss; unlicensed ones absent.
        assert!(msg.contains("opener.finite+inf —"));
        assert!(!msg.contains("clitic.do.sg.attach-to-inf"));
        // The word pool and the per-item specs.
        assert!(msg.contains("esperar"));
        assert!(msg.contains("ITEMS TO WRITE (6):"));
        assert!(msg.contains("- Item 0: person=1sg, polarity=affirmative, type=declarative"));
        assert!(msg.contains("polarity=negative"));
    }

    #[test]
    fn generation_message_lists_consolidated_words_and_scheduled_window_words() {
        let c = curriculum::load_embedded().unwrap();
        let learner = LearnerSnapshot {
            consolidated: ["gato".to_string()].into(),
            early_window: ["mensaje".to_string()].into(),
            mastered_skills: ["opener.quiero".to_string()].into(),
        };
        let plans = plan_bank(&c, "opener.quiero", &learner, 6).unwrap();
        let msg =
            build_generation_message(&c, "opener.quiero", &plans, &learner, &[]).unwrap();
        assert!(msg.contains("gato"), "consolidated words join the pool");
        assert!(msg.contains("WINDOW WORDS"));
        assert!(msg.contains("- mensaje (item 0 only)"));
        assert!(msg.contains("must use the word «mensaje»"));
    }

    #[test]
    fn generation_message_includes_existing_cues_to_avoid() {
        let c = curriculum::load_embedded().unwrap();
        let learner = LearnerSnapshot::default();
        let plans = plan_bank(&c, "opener.quiero", &learner, 3).unwrap();
        let msg = build_generation_message(
            &c,
            "opener.quiero",
            &plans,
            &learner,
            &["I want to eat.".to_string()],
        )
        .unwrap();
        assert!(msg.contains("EXISTING CUES TO AVOID"));
        assert!(msg.contains("\"I want to eat.\""));
    }

    #[test]
    fn repair_message_names_the_violations_verbatim() {
        let c = curriculum::load_embedded().unwrap();
        let learner = LearnerSnapshot::default();
        let plans = plan_bank(&c, "opener.quiero", &learner, 6).unwrap();
        let failures = vec![
            SlotFailure {
                plan: plans[3].clone(),
                attempt: Some(GeneratedItem {
                    slot_id: 3,
                    source: "I want to visit the university.".into(),
                    canonical: "Quiero visitar la universidad.".into(),
                    variants: vec![],
                }),
                violations: vec![Violation::UnlicensedVocab {
                    lemma: "universidad".into(),
                }],
            },
            SlotFailure {
                plan: plans[5].clone(),
                attempt: None,
                violations: vec![],
            },
        ];
        let msg =
            build_repair_message(&c, "opener.quiero", &failures, &learner, &[]).unwrap();

        // Full positive spec re-stated, then only the failed slots.
        assert!(msg.contains("LICENSED VERB FORMS"));
        assert!(msg.contains("REPAIR ROUND — 2 item(s)"));
        assert!(msg.contains("- Item 3:"));
        assert!(msg.contains("\"Quiero visitar la universidad.\""));
        // The serialized violation is the repair contract — quoted verbatim.
        assert!(msg.contains(r#"{"kind":"unlicensed_vocab","lemma":"universidad"}"#));
        assert!(msg.contains("- Item 5:"));
        assert!(msg.contains("no item was produced for this slot"));
        assert!(!msg.contains("- Item 0:"), "healthy slots are not re-requested");
    }
}
