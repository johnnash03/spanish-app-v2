//! S7 acceptance regression: the Tier 1 evaluator, run over all 131
//! archived v1 evaluations, must correct the documented unjust v1 verdicts
//! (PRD #31: "grammatically correct answers were marked wrong with
//! confidently false explanations — 'Los puedes ver' rejected as invalid
//! Spanish") while keeping genuinely wrong answers wrong, now classified
//! into the closed error enum.
//!
//! The analyses in `fixtures/v1_eval_tier1.json` were produced by the real
//! evaluator (`cargo run --bin evaluate_v1`) and committed, so this suite
//! is hermetic: it exercises the deterministic resolution of real LLM
//! output without network access.

#![cfg(test)]

use super::error_enum::ErrorCategory;
use super::tier1::{resolve, Tier1Analysis, Tier1Outcome};
use crate::v2::curriculum;
use serde::Deserialize;

const FIXTURE_JSON: &str = include_str!("../../../fixtures/v1_eval_tier1.json");

#[derive(Deserialize)]
struct Fixture {
    evaluator_model: String,
    evaluations: Vec<EvaluatedV1Attempt>,
}

/// Deserializing the fixture is itself the closed-enum guarantee: every
/// error category in 131 real evaluator outputs parses into
/// [`ErrorCategory`], or the whole suite fails.
#[derive(Deserialize)]
struct EvaluatedV1Attempt {
    v1_id: String,
    v1_tag: String,
    v1_correct: i64,
    answer: String,
    analysis: Tier1Analysis,
}

fn fixture() -> Fixture {
    serde_json::from_str(FIXTURE_JSON).expect("committed Tier 1 fixture parses")
}

fn by_answer<'a>(fx: &'a Fixture, answer: &str) -> Vec<&'a EvaluatedV1Attempt> {
    let found: Vec<&EvaluatedV1Attempt> =
        fx.evaluations.iter().filter(|e| e.answer == answer).collect();
    assert!(!found.is_empty(), "fixture missing `{answer}`");
    found
}

/// Resolves one archived attempt with the production resolution code. The
/// v1 tag stands in as the target skill; correct/dodge resolution never
/// consults the registry, so pre-v2 tags are fine on those paths.
fn resolve_v1(e: &EvaluatedV1Attempt) -> Result<Tier1Outcome, super::Tier1Error> {
    let c = curriculum::load_embedded().unwrap();
    resolve(&e.analysis, &e.v1_tag, &e.v1_tag, &[], &c)
}

/// Real, meaning-conveying Spanish that the v1 evaluator was capable of
/// rejecting — the documented unjust verdicts plus the same leniency
/// violations wherever they recur in the archive. Each must now resolve
/// correct — or dodge, which the learner also sees as correct (with a
/// nudge) and which never marks good Spanish wrong.
const UNJUST_V1_VERDICTS: &[&str] = &[
    // The PRD's headline case: valid clitic-before-finite placement,
    // rejected as invalid Spanish.
    "Los puedes ver",
    // Punctuation-leniency violations (rules v1 wrote and then broke).
    "Lo entiendes",
    "Comes aqui",
    "Como estas",
    // Accent-leniency violations, including on question words and verb
    // endings.
    "Entiendo por que hablas",
    "Como le envias dinero",
    "Por que lo quiere",
    // Optional-subject-pronoun phrasing punished as an omission.
    "Quiere bailar",
    "quiere bailar",
    // Valid near-future phrasing rejected for not being the canonical
    // future tense.
    "Sabe si va a venir",
];

#[test]
fn fixture_covers_all_131_v1_evaluations() {
    let fx = fixture();
    assert!(!fx.evaluator_model.is_empty());
    assert_eq!(fx.evaluations.len(), 131, "every archived v1 evaluation is re-judged");
    let v1_wrong = fx.evaluations.iter().filter(|e| e.v1_correct == 0).count();
    assert_eq!(v1_wrong, 67, "the archive's wrong-marked verdicts are all present");
    for e in &fx.evaluations {
        assert!(!e.v1_id.is_empty());
    }
}

#[test]
fn every_documented_unjust_verdict_now_evaluates_correct() {
    let fx = fixture();
    for answer in UNJUST_V1_VERDICTS {
        for e in by_answer(&fx, answer) {
            match resolve_v1(e).unwrap() {
                Tier1Outcome::Correct | Tier1Outcome::Dodge { .. } => {}
                Tier1Outcome::Wrong { category, evidence, .. } => panic!(
                    "`{answer}` ({}) is still judged wrong: {category:?} ({evidence})",
                    e.v1_id
                ),
            }
        }
    }
}

#[test]
fn los_puedes_ver_is_grammatical_meaning_conveying_spanish() {
    // The headline injustice, asserted at the judgment level: both
    // decomposed verdicts that decide correctness are true.
    let fx = fixture();
    for e in by_answer(&fx, "Los puedes ver") {
        assert!(e.analysis.grammatical.verdict, "grammatical: {:?}", e.analysis.grammatical);
        assert!(
            e.analysis.conveys_meaning.verdict,
            "conveys meaning: {:?}",
            e.analysis.conveys_meaning
        );
    }
}

#[test]
fn genuinely_wrong_answers_stay_wrong_in_the_closed_enum() {
    // V1's wrong-marking was not all unjust; the fix must not be blanket
    // leniency. Each case names the categories a sound classification may
    // use.
    let cases: &[(&str, &[ErrorCategory])] = &[
        // Invented 1pl form of querer.
        ("Quieromos cancelar la reunion", &[ErrorCategory::VerbForm]),
        ("Quieromos comparar las opciones", &[ErrorCategory::VerbForm]),
        // lo for la ("I want to see her").
        ("Quiero verlo", &[ErrorCategory::CliticChoice]),
        // Laísmo: la for le ("He talks to her").
        ("La habla", &[ErrorCategory::CliticChoice]),
        // Real misspelling, not an accent slip.
        ("Realmente concoces a Maria", &[ErrorCategory::Orthography]),
        // n where ñ belongs is not accent leniency (anos ≠ años).
        ("Puedes intentarlo manana", &[ErrorCategory::Orthography]),
    ];
    let fx = fixture();
    for (answer, allowed) in cases {
        for e in by_answer(&fx, answer) {
            let correct =
                e.analysis.grammatical.verdict && e.analysis.conveys_meaning.verdict;
            assert!(!correct, "`{answer}` must stay wrong");
            let finding = e.analysis.error.as_ref().unwrap_or_else(|| {
                panic!("`{answer}` has no error classification")
            });
            assert!(
                allowed.contains(&finding.category),
                "`{answer}` classified {:?}, expected one of {allowed:?}",
                finding.category
            );
        }
    }
}

#[test]
fn every_wrong_judgment_carries_classification_hint_and_explanation() {
    // User stories 17 and 18, over the full archive: a wrong answer is
    // never a bare verdict.
    let fx = fixture();
    for e in &fx.evaluations {
        let correct = e.analysis.grammatical.verdict && e.analysis.conveys_meaning.verdict;
        if !correct {
            let finding = e.analysis.error.as_ref().unwrap_or_else(|| {
                panic!("wrong `{}` ({}) has no classification", e.answer, e.v1_id)
            });
            assert!(!finding.evidence.is_empty(), "`{}` has no evidence span", e.answer);
            assert!(
                e.analysis.hint.as_deref().is_some_and(|h| !h.is_empty()),
                "`{}` has no hint",
                e.answer
            );
            assert!(
                e.analysis.explanation.as_deref().is_some_and(|x| !x.is_empty()),
                "`{}` has no explanation",
                e.answer
            );
        }
    }
}

#[test]
fn structure_dodges_resolve_to_the_nudge_path_on_real_output() {
    // "Los puedes ver" against an attach-to-infinitive target is the
    // canonical dodge: real Spanish, target sidestepped — correct with a
    // nudge, never wrong (user story 15).
    let fx = fixture();
    for e in by_answer(&fx, "Los puedes ver") {
        if !e.analysis.uses_target_structure.verdict {
            match resolve_v1(e).unwrap() {
                Tier1Outcome::Dodge { nudge } => {
                    assert!(nudge.contains("Correct"), "nudge affirms the Spanish: {nudge}")
                }
                other => panic!("expected dodge, got {other:?}"),
            }
        }
    }
}

// V1's other documented injustice — empty submissions marked correct — is
// settled by code before any model call: an empty answer is wrong at
// submit time and never reaches Tier 1 (the fixture shows why: handed an
// empty answer, the model invents one from the cue). The guarantee is
// tested where it lives, in
// `session::store::tests::empty_answers_are_wrong_by_code_and_never_reach_tier1`.
