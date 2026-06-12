//! S4 acceptance regression: the validator, run over the v1 unit-1
//! exercise banks, must catch the documented v1 ordering leaks (PRD #31:
//! quiere/queremos/ser inside unit 1 banks — grammar from many phases
//! later served in the first unit).
//!
//! The analyses in `fixtures/v1_unit1_analyses.json` were produced by the
//! real analyzer (`cargo run --bin analyze_v1_unit1`) and committed, so
//! this suite is hermetic: it exercises the deterministic judgment of real
//! LLM output without network access.

#![cfg(test)]

use super::judge::{judge, JudgeContext};
use super::types::*;
use crate::v2::curriculum;
use serde::Deserialize;
use std::collections::BTreeSet;

const ANALYSES_JSON: &str = include_str!("../../../fixtures/v1_unit1_analyses.json");

#[derive(Deserialize)]
struct Fixture {
    analyzer_model: String,
    analyses: Vec<AnalyzedV1Item>,
}

#[derive(Deserialize)]
struct AnalyzedV1Item {
    item_id: String,
    /// The v1 bank tag; identical to the v2 unit id for these two units.
    primary_tag: String,
    source: String,
    canonical: String,
    analysis: ItemAnalysis,
}

fn fixture() -> Fixture {
    serde_json::from_str(ANALYSES_JSON).expect("committed analyses fixture parses")
}

/// Judges one archived v1 item against its unit's v2 licensing, exactly as
/// the generation gate would: empty window, empty bank, no slot spec.
fn judge_v1(item: &AnalyzedV1Item) -> Vec<Violation> {
    let c = curriculum::load_embedded().unwrap();
    let registry = c.construction_registry();
    let window = BTreeSet::new();
    judge(
        &CandidateItem {
            source: item.source.clone(),
            canonical: item.canonical.clone(),
        },
        &item.analysis,
        &JudgeContext {
            licensing: c.effective_licensing(&item.primary_tag).unwrap(),
            target: c.target_spec(&item.primary_tag).unwrap(),
            construction_registry: &registry,
            window: &window,
            existing: &[],
            slot: None,
        },
    )
}

fn by_canonical<'a>(fx: &'a Fixture, canonical: &str) -> &'a AnalyzedV1Item {
    fx.analyses
        .iter()
        .find(|a| a.canonical == canonical)
        .unwrap_or_else(|| panic!("fixture missing `{canonical}`"))
}

#[test]
fn fixture_covers_both_v1_unit1_banks() {
    let fx = fixture();
    assert!(!fx.analyzer_model.is_empty());
    assert_eq!(fx.analyses.len(), 40, "20 items per v1 unit-1 bank");
    for tag in ["opener.quiero", "opener.quiero.neg"] {
        assert_eq!(fx.analyses.iter().filter(|a| a.primary_tag == tag).count(), 20);
    }
}

#[test]
fn catches_the_documented_quiere_leak() {
    // v1 served 3sg `quiere` in the very first unit; v2 licenses it 9
    // units later (clitic.do.person.attach).
    let fx = fixture();
    let violations = judge_v1(by_canonical(&fx, "Quiere bailar."));
    assert!(
        violations.contains(&Violation::UnlicensedVerbForm {
            lemma: "querer".into(),
            form: "pres.3sg".into(),
            surface: "Quiere".into(),
        }),
        "got {violations:?}"
    );
}

#[test]
fn catches_the_documented_queremos_leak() {
    // 1pl `queremos` is licensed nowhere in Phases 1–4.
    let fx = fixture();
    let violations = judge_v1(by_canonical(&fx, "Queremos comparar las opciones."));
    assert!(
        violations.contains(&Violation::UnlicensedVerbForm {
            lemma: "querer".into(),
            form: "pres.1pl".into(),
            surface: "Queremos".into(),
        }),
        "got {violations:?}"
    );
}

#[test]
fn catches_the_documented_ser_leak() {
    // `ser` appeared inside the unit-1 negation bank; it is licensed
    // nowhere in Phases 1–4 (neither enumerated nor vocabulary).
    let fx = fixture();
    let violations = judge_v1(by_canonical(&fx, "No quiere ser diferente."));
    assert!(
        violations
            .iter()
            .any(|v| matches!(v, Violation::UnlicensedVerbForm { lemma, .. } if lemma == "ser")),
        "got {violations:?}"
    );
}

#[test]
fn every_non_1sg_querer_form_in_the_banks_is_rejected_by_name() {
    // The leak class, not just the headline examples: any querer form
    // other than the licensed `quiero` must produce a named violation.
    let fx = fixture();
    let mut leaks = 0;
    for item in &fx.analyses {
        for vf in &item.analysis.verb_forms {
            if vf.lemma == "querer" && vf.form != "pres.1sg" {
                leaks += 1;
                assert!(
                    judge_v1(item).contains(&Violation::UnlicensedVerbForm {
                        lemma: vf.lemma.clone(),
                        form: vf.form.clone(),
                        surface: vf.surface.clone(),
                    }),
                    "`{}` leak not caught",
                    item.canonical
                );
            }
        }
    }
    assert!(leaks >= 10, "the v1 banks are known to be leak-ridden, found {leaks}");
}

#[test]
fn clean_v1_items_pass_the_gate() {
    // The gate must not be vacuously strict: the handful of v1 items that
    // genuinely respect unit-1 licensing pass untouched.
    let fx = fixture();
    for canonical in ["Quiero comer.", "No quiero comer."] {
        let violations = judge_v1(by_canonical(&fx, canonical));
        assert!(violations.is_empty(), "`{canonical}` should pass, got {violations:?}");
    }
}

#[test]
fn rejections_are_machine_readable_for_the_repair_loop() {
    let fx = fixture();
    let violations = judge_v1(by_canonical(&fx, "Quiere bailar."));
    let json = serde_json::to_value(&violations).unwrap();
    let kinds: Vec<&str> = json
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v["kind"].as_str().unwrap())
        .collect();
    assert!(kinds.contains(&"unlicensed_verb_form"));
    // Every violation kind round-trips through serde — the repair loop can
    // parse what the judge emits.
    let parsed: Vec<Violation> = serde_json::from_value(json).unwrap();
    assert_eq!(parsed, violations);
}
