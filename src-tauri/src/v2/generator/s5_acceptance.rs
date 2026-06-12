//! S5 acceptance regression (#36): the committed output of a real
//! end-to-end pipeline run (`cargo run --bin generate_unit -- opener.quiero 20`)
//! must hold up under re-judgment — a validated Phase 1 bank with zero
//! licensing violations, every item carrying canonical + validated
//! variants + slot spec + tags.
//!
//! The fixture makes this hermetic: it exercises deterministic judgment of
//! real generator/analyzer output without network access. Re-run the
//! binary to refresh it after prompt or pipeline changes.

#![cfg(test)]

use super::types::BankItem;
use crate::v2::curriculum;
use crate::v2::validator::{judge, CandidateItem, JudgeContext, Polarity, Violation};
use serde::Deserialize;
use std::collections::BTreeSet;

const BANK_JSON: &str = include_str!("../../../fixtures/generated_bank_opener_quiero.json");

#[derive(Deserialize)]
struct Fixture {
    unit_id: String,
    generator_model: String,
    analyzer_model: String,
    banked: usize,
    items: Vec<BankItem>,
}

fn fixture() -> Fixture {
    serde_json::from_str(BANK_JSON).expect("committed bank fixture parses")
}

/// Re-judges an analysis exactly as the pipeline's gate did (empty window:
/// the run used a default learner snapshot).
fn judge_analysis(
    candidate: &CandidateItem,
    analysis: &crate::v2::validator::ItemAnalysis,
    slot: Option<&crate::v2::validator::SlotSpec>,
) -> Vec<Violation> {
    let c = curriculum::load_embedded().unwrap();
    let registry = c.construction_registry();
    let window = BTreeSet::new();
    judge(
        candidate,
        analysis,
        &JudgeContext {
            licensing: c.effective_licensing("opener.quiero").unwrap(),
            target: c.target_spec("opener.quiero").unwrap(),
            construction_registry: &registry,
            window: &window,
            existing: &[],
            slot,
            stacked_targets: &[],
        },
    )
}

#[test]
fn real_phase1_bank_has_zero_licensing_violations() {
    let fx = fixture();
    assert_eq!(fx.unit_id, "opener.quiero");
    assert!(!fx.generator_model.is_empty());
    assert!(!fx.analyzer_model.is_empty());
    assert_eq!(fx.items.len(), fx.banked);
    assert!(fx.banked >= 15, "a usable bank, got {}", fx.banked);

    for item in &fx.items {
        let candidate = CandidateItem {
            source: item.source.clone(),
            canonical: item.canonical.clone(),
        };
        let violations = judge_analysis(&candidate, &item.analysis, Some(&item.slot));
        assert!(
            violations.is_empty(),
            "`{}` re-judged dirty: {violations:?}",
            item.canonical
        );
    }
}

#[test]
fn every_item_carries_validated_variants_slot_spec_and_tags() {
    let fx = fixture();
    for item in &fx.items {
        // Tags and slot spec persisted per item.
        assert_eq!(item.tags.target_skill, "opener.quiero");
        assert_eq!(item.slot.person.as_deref(), Some("1sg"));
        assert!(item.slot.polarity.is_some());
        assert!(!item.id.is_empty());
        assert!(!item.analysis.verb_forms.is_empty(), "analysis kept for inspection");

        // Every stored variant was validated: re-judging its analysis
        // (licensing + target structure) stays clean.
        for v in &item.variants {
            let candidate = CandidateItem {
                source: item.source.clone(),
                canonical: v.text.clone(),
            };
            let violations = judge_analysis(&candidate, &v.analysis, None);
            assert!(
                violations.is_empty(),
                "variant `{}` of `{}` re-judged dirty: {violations:?}",
                v.text,
                item.canonical
            );
        }
    }
    let with_variants = fx.items.iter().filter(|i| !i.variants.is_empty()).count();
    assert!(
        with_variants * 2 >= fx.items.len(),
        "most items should carry at least one accepted variant, got {with_variants}/{}",
        fx.items.len()
    );
}

#[test]
fn bank_varies_along_the_planned_slot_axes() {
    // User story 7: the slot plan must actually have produced variety —
    // both polarities present, and more than a handful of distinct verbs.
    let fx = fixture();
    let polarities: BTreeSet<&str> = fx
        .items
        .iter()
        .filter_map(|i| i.slot.polarity)
        .map(|p| match p {
            Polarity::Affirmative => "aff",
            Polarity::Negative => "neg",
        })
        .collect();
    assert_eq!(polarities.len(), 2, "both polarities must appear in the bank");

    let infinitives: BTreeSet<&str> = fx
        .items
        .iter()
        .flat_map(|i| i.analysis.verb_forms.iter())
        .filter(|vf| vf.form == "inf")
        .map(|vf| vf.lemma.as_str())
        .collect();
    assert!(
        infinitives.len() >= 5,
        "bank collapsed onto too few verbs: {infinitives:?}"
    );
}
