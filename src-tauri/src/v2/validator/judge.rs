//! The deterministic half of the validator: set-membership judgment of an
//! [`ItemAnalysis`] against a unit's [`EffectiveLicensing`], target spec,
//! slot spec, and existing bank. No model discretion — every check is
//! plain code over enumerated data.

use super::types::*;
use crate::v2::curriculum::types::{
    is_known_form_slot, EffectiveLicensing, TargetAtom, TargetSpec,
};
use std::collections::BTreeSet;

/// Tokens this similar (Jaccard) to an existing item are a near-duplicate.
const NEAR_DUP_JACCARD: f64 = 0.8;

/// Everything the judge consults besides the item and its analysis.
pub struct JudgeContext<'a> {
    pub licensing: &'a EffectiveLicensing,
    /// The unit's resolved target-skill spec
    /// ([`crate::v2::curriculum::Curriculum::target_spec`]).
    pub target: &'a TargetSpec,
    /// The curriculum-wide construction tag registry
    /// ([`crate::v2::curriculum::Curriculum::construction_registry`]).
    /// Tags outside it are analyzer errors, not licensing questions.
    pub construction_registry: &'a BTreeSet<String>,
    /// Active vocabulary-window lemmas (licensed in addition to the
    /// curriculum vocab; empty when judging curriculum-only content).
    pub window: &'a BTreeSet<String>,
    /// Already-banked items, for near-duplication.
    pub existing: &'a [ExistingItem],
    /// The slot spec this item was generated against, if any.
    pub slot: Option<&'a SlotSpec>,
    /// Resolved target specs of the skills this item was asked to stack
    /// (skill id → spec, resolved by the pipeline from the curriculum).
    /// Each must be exercised, exactly like the unit's own target.
    pub stacked_targets: &'a [(String, TargetSpec)],
}

/// Judges one analyzed item. An empty result is a pass; otherwise every
/// violation names the specific offending element for the repair loop.
pub fn judge(item: &CandidateItem, analysis: &ItemAnalysis, ctx: &JudgeContext) -> Vec<Violation> {
    let mut violations = Vec::new();

    for vf in &analysis.verb_forms {
        if !is_known_form_slot(&vf.form) {
            // Includes the analyzer's "other" escape hatch: a form it
            // could not place in the registry is rejected, never waved
            // through (fail-safe).
            violations.push(Violation::UnrecognizedFormSlot {
                lemma: vf.lemma.clone(),
                form: vf.form.clone(),
                surface: vf.surface.clone(),
            });
        } else if !verb_form_licensed(vf, ctx) {
            violations.push(Violation::UnlicensedVerbForm {
                lemma: vf.lemma.clone(),
                form: vf.form.clone(),
                surface: vf.surface.clone(),
            });
        }
    }

    for lemma in &analysis.content_lemmas {
        if !ctx.licensing.vocab.contains(lemma) && !ctx.window.contains(lemma) {
            violations.push(Violation::UnlicensedVocab {
                lemma: lemma.clone(),
            });
        }
    }

    for c in &analysis.constructions {
        if !ctx.construction_registry.contains(c) {
            violations.push(Violation::UnknownConstructionTag {
                construction: c.clone(),
            });
        } else if !ctx.licensing.constructions.contains(c) {
            violations.push(Violation::UnlicensedConstruction {
                construction: c.clone(),
            });
        }
    }

    for group in &ctx.target.groups {
        if !group.iter().any(|atom| atom_satisfied(atom, analysis)) {
            violations.push(Violation::TargetSkillNotExercised {
                target_skill: ctx.licensing.unit_id.clone(),
                unmet: group.iter().map(render_atom).collect(),
            });
        }
    }

    for (skill, spec) in ctx.stacked_targets {
        for group in &spec.groups {
            if !group.iter().any(|atom| atom_satisfied(atom, analysis)) {
                violations.push(Violation::StackedSkillNotExercised {
                    skill: skill.clone(),
                    unmet: group.iter().map(render_atom).collect(),
                });
            }
        }
    }

    if let Some(dup) = find_near_duplicate(&item.canonical, ctx.existing) {
        violations.push(Violation::NearDuplicate {
            of_item_id: dup.id.clone(),
        });
    }

    if let Some(slot) = ctx.slot {
        check_slot(item, analysis, slot, &mut violations);
    }

    violations
}

/// A verb form is licensed either as an enumerated grant (power verbs) or
/// by riding an open vocab form slot: the slot must be granted for the
/// verb's conjugation class and the lemma itself must be licensed
/// vocabulary or an active window word.
fn verb_form_licensed(vf: &AnalyzedVerbForm, ctx: &JudgeContext) -> bool {
    if ctx
        .licensing
        .verb_forms
        .iter()
        .any(|g| g.lemma == vf.lemma && g.form == vf.form)
    {
        return true;
    }

    let slot_granted = ctx.licensing.vocab_verb_forms.iter().any(|g| {
        g.form == vf.form
            && g.classes.as_ref().is_none_or(|classes| {
                classes
                    .iter()
                    .any(|c| conjugation_class(&vf.lemma) == Some(c.as_str()))
            })
    });
    slot_granted && (ctx.licensing.vocab.contains(&vf.lemma) || ctx.window.contains(&vf.lemma))
}

/// Conjugation class by infinitive ending; `None` for non-infinitive
/// lemmas, which can never ride a class-restricted slot.
fn conjugation_class(lemma: &str) -> Option<&'static str> {
    if lemma.ends_with("ar") {
        Some("ar")
    } else if lemma.ends_with("er") {
        Some("er")
    } else if lemma.ends_with("ir") || lemma.ends_with("ír") {
        Some("ir")
    } else {
        None
    }
}

fn atom_satisfied(atom: &TargetAtom, analysis: &ItemAnalysis) -> bool {
    match atom {
        TargetAtom::Form { lemma, form } => analysis
            .verb_forms
            .iter()
            .any(|vf| vf.lemma == *lemma && vf.form == *form),
        TargetAtom::Construction(tag) => analysis.constructions.iter().any(|c| c == tag),
    }
}

fn render_atom(atom: &TargetAtom) -> String {
    match atom {
        TargetAtom::Form { lemma, form } => format!("form:{lemma}@{form}"),
        TargetAtom::Construction(tag) => format!("construction:{tag}"),
    }
}

fn find_near_duplicate<'a>(
    canonical: &str,
    existing: &'a [ExistingItem],
) -> Option<&'a ExistingItem> {
    let candidate: BTreeSet<String> = normalized_tokens(canonical).collect();
    existing.iter().find(|e| {
        let banked: BTreeSet<String> = normalized_tokens(&e.canonical).collect();
        let intersection = candidate.intersection(&banked).count();
        let union = candidate.union(&banked).count();
        union > 0 && intersection as f64 / union as f64 >= NEAR_DUP_JACCARD
    })
}

/// Lowercased, deaccented, punctuation-free word tokens — the same
/// normalization stance as Tier-0 evaluation (accents and ¿¡ never
/// distinguish items).
fn normalized_tokens(text: &str) -> impl Iterator<Item = String> + '_ {
    text.split_whitespace().filter_map(|raw| {
        let token: String = raw
            .chars()
            .filter(|c| c.is_alphanumeric())
            .flat_map(|c| c.to_lowercase())
            .map(|c| match c {
                'á' => 'a',
                'é' => 'e',
                'í' => 'i',
                'ó' => 'o',
                'ú' | 'ü' => 'u',
                'ñ' => 'n',
                other => other,
            })
            .collect();
        (!token.is_empty()).then_some(token)
    })
}

fn check_slot(
    item: &CandidateItem,
    analysis: &ItemAnalysis,
    slot: &SlotSpec,
    violations: &mut Vec<Violation>,
) {
    if let Some(person) = &slot.person {
        let suffix = format!(".{person}");
        if !analysis.verb_forms.iter().any(|vf| vf.form.ends_with(&suffix)) {
            let found: Vec<&str> = analysis
                .verb_forms
                .iter()
                .filter_map(|vf| vf.form.rsplit_once('.').map(|(_, p)| p))
                .collect();
            violations.push(Violation::SlotMismatch {
                slot: "person".into(),
                expected: person.clone(),
                found: if found.is_empty() {
                    "none".into()
                } else {
                    found.join(", ")
                },
            });
        }
    }

    if let Some(polarity) = slot.polarity {
        let negative = analysis.constructions.iter().any(|c| c.starts_with("neg."));
        let found = if negative {
            Polarity::Negative
        } else {
            Polarity::Affirmative
        };
        if found != polarity {
            violations.push(Violation::SlotMismatch {
                slot: "polarity".into(),
                expected: format!("{polarity:?}").to_lowercase(),
                found: format!("{found:?}").to_lowercase(),
            });
        }
    }

    if let Some(lemma) = &slot.required_lemma {
        let appears = analysis.content_lemmas.contains(lemma)
            || analysis.verb_forms.iter().any(|vf| &vf.lemma == lemma);
        if !appears {
            violations.push(Violation::ScheduledWordMissing {
                lemma: lemma.clone(),
            });
        }
    }

    if let Some(sentence_type) = slot.sentence_type {
        let question = item.canonical.contains('¿') || item.canonical.contains('?');
        let found = if question {
            SentenceType::Question
        } else {
            SentenceType::Declarative
        };
        if found != sentence_type {
            violations.push(Violation::SlotMismatch {
                slot: "sentence_type".into(),
                expected: format!("{sentence_type:?}").to_lowercase(),
                found: format!("{found:?}").to_lowercase(),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2::curriculum::{self, Curriculum};

    fn avf(lemma: &str, form: &str, surface: &str) -> AnalyzedVerbForm {
        AnalyzedVerbForm {
            lemma: lemma.into(),
            form: form.into(),
            surface: surface.into(),
        }
    }

    fn item(canonical: &str) -> CandidateItem {
        CandidateItem {
            source: "(cue)".into(),
            canonical: canonical.into(),
        }
    }

    /// Judges against the embedded curriculum with the given extras.
    fn judge_full(
        c: &Curriculum,
        unit_id: &str,
        canonical: &str,
        analysis: &ItemAnalysis,
        window: &BTreeSet<String>,
        existing: &[ExistingItem],
        slot: Option<&SlotSpec>,
    ) -> Vec<Violation> {
        let registry = c.construction_registry();
        judge(
            &item(canonical),
            analysis,
            &JudgeContext {
                licensing: c.effective_licensing(unit_id).unwrap(),
                target: c.target_spec(unit_id).unwrap(),
                construction_registry: &registry,
                window,
                existing,
                slot,
                stacked_targets: &[],
            },
        )
    }

    fn judge_unit(unit_id: &str, canonical: &str, analysis: &ItemAnalysis) -> Vec<Violation> {
        let c = curriculum::load_embedded().unwrap();
        judge_full(&c, unit_id, canonical, analysis, &BTreeSet::new(), &[], None)
    }

    /// "Quiero comer." in opener.quiero — fully licensed, on target.
    fn quiero_comer() -> ItemAnalysis {
        ItemAnalysis {
            verb_forms: vec![avf("querer", "pres.1sg", "quiero"), avf("comer", "inf", "comer")],
            constructions: vec!["opener.finite+inf".into()],
            content_lemmas: vec![],
        }
    }

    #[test]
    fn licensed_on_target_item_passes_clean() {
        assert!(judge_unit("opener.quiero", "Quiero comer.", &quiero_comer()).is_empty());
    }

    #[test]
    fn flags_verb_form_outside_the_licensing_set() {
        // The canonical v1 unit-1 leak: 3sg `quiere` in the opener.quiero
        // bank, where only `quiero` (pres.1sg) is licensed.
        let analysis = ItemAnalysis {
            verb_forms: vec![avf("querer", "pres.3sg", "quiere"), avf("bailar", "inf", "bailar")],
            constructions: vec!["opener.finite+inf".into()],
            content_lemmas: vec![],
        };
        let violations = judge_unit("opener.quiero", "Quiere bailar.", &analysis);
        assert!(violations.contains(&Violation::UnlicensedVerbForm {
            lemma: "querer".into(),
            form: "pres.3sg".into(),
            surface: "quiere".into(),
        }));
        // …and 3sg querer is not the unit's target skill either.
        assert!(violations.iter().any(|v| matches!(
            v,
            Violation::TargetSkillNotExercised { target_skill, unmet }
                if target_skill == "opener.quiero"
                    && unmet == &vec!["form:querer@pres.1sg".to_string()]
        )));
    }

    #[test]
    fn vocab_verb_rides_a_licensed_form_slot() {
        // "Quiero esperar." — esperar is unit-1 *vocab*, licensed in the
        // open `inf` slot granted by the same unit.
        let mut licensed = quiero_comer();
        licensed.verb_forms[1] = avf("esperar", "inf", "esperar");
        assert!(judge_unit("opener.quiero", "Quiero esperar.", &licensed).is_empty());

        // "Quiero nadar." — the slot is licensed but nadar enters the
        // vocabulary only at opener.voy-a.
        let mut unlicensed = quiero_comer();
        unlicensed.verb_forms[1] = avf("nadar", "inf", "nadar");
        assert_eq!(
            judge_unit("opener.quiero", "Quiero nadar.", &unlicensed),
            vec![Violation::UnlicensedVerbForm {
                lemma: "nadar".into(),
                form: "inf".into(),
                surface: "nadar".into(),
            }]
        );

        // Same lemma in the active window instead: licensed.
        let c = curriculum::load_embedded().unwrap();
        let window: BTreeSet<String> = ["nadar".to_string()].into();
        assert!(judge_full(
            &c,
            "opener.quiero",
            "Quiero nadar.",
            &unlicensed,
            &window,
            &[],
            None
        )
        .is_empty());
    }

    #[test]
    fn vocab_verb_slot_respects_class_restriction() {
        // A conjugated slot restricted to -ar verbs licenses esperar but
        // not beber, even with both lemmas in the vocabulary.
        let c = curriculum::loader::load_units_for_tests(
            r#"[{
                "id": "a", "title": "A", "phase": 1,
                "grant": {
                    "vocab_verb_forms": [{"form": "pres.3sg", "classes": ["ar"]}],
                    "constructions": ["opener.modal-inf"],
                    "vocab": ["esperar", "beber"]
                },
                "target": [["construction:opener.modal-inf"]]
            }]"#,
        );
        let base = ItemAnalysis {
            constructions: vec!["opener.modal-inf".into()],
            ..Default::default()
        };

        let mut ar = base.clone();
        ar.verb_forms = vec![avf("esperar", "pres.3sg", "espera")];
        assert!(judge_full(&c, "a", "Espera.", &ar, &BTreeSet::new(), &[], None).is_empty());

        let mut er = base.clone();
        er.verb_forms = vec![avf("beber", "pres.3sg", "bebe")];
        assert_eq!(
            judge_full(&c, "a", "Bebe.", &er, &BTreeSet::new(), &[], None).len(),
            1
        );
    }

    #[test]
    fn flags_known_but_unlicensed_construction() {
        // Clitic attachment is a real curriculum construction, but it is
        // licensed in phase 2 — not in unit 1.
        let mut analysis = quiero_comer();
        analysis.constructions.push("clitic.do.sg.attach-to-inf".into());
        assert_eq!(
            judge_unit("opener.quiero", "Quiero comerlo.", &analysis),
            vec![Violation::UnlicensedConstruction {
                construction: "clitic.do.sg.attach-to-inf".into(),
            }]
        );
    }

    #[test]
    fn flags_construction_tag_outside_the_registry_as_bad_analysis() {
        // A tag no unit grants is an analyzer vocabulary error, not a
        // licensing question — it must fail safe as invalid analysis.
        let mut analysis = quiero_comer();
        analysis.constructions.push("ser.copula".into());
        assert_eq!(
            judge_unit("opener.quiero", "Quiero comer.", &analysis),
            vec![Violation::UnknownConstructionTag {
                construction: "ser.copula".into(),
            }]
        );
    }

    #[test]
    fn flags_content_lemma_outside_vocab_and_window() {
        // The v1 corporate-abstract leak shape: "…la universidad" with
        // universidad never licensed and not in the window.
        let mut analysis = quiero_comer();
        analysis.content_lemmas = vec!["universidad".into()];
        assert_eq!(
            judge_unit("opener.quiero", "Quiero visitar la universidad.", &analysis),
            vec![Violation::UnlicensedVocab {
                lemma: "universidad".into(),
            }]
        );

        // The same lemma in the active window is licensed.
        let c = curriculum::load_embedded().unwrap();
        let window: BTreeSet<String> = ["universidad".to_string()].into();
        assert!(judge_full(
            &c,
            "opener.quiero",
            "Quiero visitar la universidad.",
            &analysis,
            &window,
            &[],
            None
        )
        .is_empty());
    }

    // --- target skill ---

    #[test]
    fn tampoco_unit_requires_tampoco_not_just_any_negation() {
        let mut tampoco_item = quiero_comer();
        tampoco_item.constructions.push("neg.tampoco".into());
        assert!(judge_unit("opener.tampoco", "Tampoco quiero comer.", &tampoco_item).is_empty());

        // Plain preverbal `no` is ambient-licensed but off target here —
        // mixed-polarity practice of it lives in every other unit.
        let mut no_item = quiero_comer();
        no_item.constructions.push("neg.no.preverbal".into());
        let violations = judge_unit("opener.tampoco", "No quiero comer.", &no_item);
        assert_eq!(violations.len(), 1);
        assert!(matches!(
            &violations[0],
            Violation::TargetSkillNotExercised { target_skill, unmet }
                if target_skill == "opener.tampoco" && unmet.len() == 1
        ));

        // As is an affirmative item.
        let affirmative = quiero_comer();
        let violations = judge_unit("opener.tampoco", "Quiero comer.", &affirmative);
        assert!(matches!(
            &violations[0],
            Violation::TargetSkillNotExercised { target_skill, .. }
                if target_skill == "opener.tampoco"
        ));
    }

    // --- near-duplication ---

    #[test]
    fn flags_near_duplicate_of_banked_item() {
        let existing = [
            ExistingItem {
                id: "item-1".into(),
                canonical: "Quiero comer ahora.".into(),
            },
            ExistingItem {
                id: "item-2".into(),
                canonical: "Debo trabajar mucho.".into(),
            },
        ];
        let c = curriculum::load_embedded().unwrap();

        // Identical up to accents/punctuation/case.
        let dup = judge_full(
            &c,
            "opener.quiero",
            "¿quiero comer ahora?",
            &quiero_comer(),
            &BTreeSet::new(),
            &existing,
            None,
        );
        assert!(dup.contains(&Violation::NearDuplicate {
            of_item_id: "item-1".into()
        }));

        // A genuinely different sentence passes.
        let fresh = judge_full(
            &c,
            "opener.quiero",
            "Quiero esperar aquí.",
            &quiero_comer(),
            &BTreeSet::new(),
            &existing,
            None,
        );
        assert!(!fresh
            .iter()
            .any(|v| matches!(v, Violation::NearDuplicate { .. })));
    }

    // --- slot conformance ---

    #[test]
    fn flags_slot_mismatches_on_every_axis() {
        let c = curriculum::load_embedded().unwrap();
        let slot = SlotSpec {
            person: Some("3sg".into()),
            polarity: Some(Polarity::Negative),
            sentence_type: Some(SentenceType::Question),
            ..Default::default()
        };
        // "Quiero comer." — 1sg, affirmative, declarative: misses all three.
        let violations = judge_full(
            &c,
            "opener.quiero",
            "Quiero comer.",
            &quiero_comer(),
            &BTreeSet::new(),
            &[],
            Some(&slot),
        );
        let slots: Vec<&str> = violations
            .iter()
            .filter_map(|v| match v {
                Violation::SlotMismatch { slot, .. } => Some(slot.as_str()),
                _ => None,
            })
            .collect();
        assert_eq!(slots, vec!["person", "polarity", "sentence_type"]);
    }

    #[test]
    fn conforming_slot_spec_passes() {
        let c = curriculum::load_embedded().unwrap();
        let slot = SlotSpec {
            person: Some("1sg".into()),
            polarity: Some(Polarity::Affirmative),
            sentence_type: Some(SentenceType::Declarative),
            ..Default::default()
        };
        assert!(judge_full(
            &c,
            "opener.quiero",
            "Quiero comer.",
            &quiero_comer(),
            &BTreeSet::new(),
            &[],
            Some(&slot),
        )
        .is_empty());
    }

    // --- stacked skills (S5) ---

    #[test]
    fn flags_stacked_skill_the_sentence_does_not_exercise() {
        // Spec asked for opener.tampoco stacked on top; the produced
        // sentence has no tampoco, so the stack is missing. Judged inside
        // opener.mixed, which inherits the tampoco grant — stacking only
        // ever draws from ancestors, so the stack is always licensed.
        let c = curriculum::load_embedded().unwrap();
        let registry = c.construction_registry();
        let window = BTreeSet::new();
        let stacked = vec![(
            "opener.tampoco".to_string(),
            c.target_spec("opener.tampoco").unwrap().clone(),
        )];
        let violations = judge(
            &item("Quiero comer."),
            &quiero_comer(),
            &JudgeContext {
                licensing: c.effective_licensing("opener.mixed").unwrap(),
                target: c.target_spec("opener.quiero").unwrap(),
                construction_registry: &registry,
                window: &window,
                existing: &[],
                slot: None,
                stacked_targets: &stacked,
            },
        );
        assert!(violations.iter().any(|v| matches!(
            v,
            Violation::StackedSkillNotExercised { skill, unmet }
                if skill == "opener.tampoco" && !unmet.is_empty()
        )));

        // A tampoco sentence satisfies both the target and the stack.
        let mut negated = quiero_comer();
        negated.constructions.push("neg.tampoco".into());
        let clean = judge(
            &item("Tampoco quiero comer."),
            &negated,
            &JudgeContext {
                licensing: c.effective_licensing("opener.mixed").unwrap(),
                target: c.target_spec("opener.quiero").unwrap(),
                construction_registry: &registry,
                window: &window,
                existing: &[],
                slot: None,
                stacked_targets: &stacked,
            },
        );
        assert!(clean.is_empty(), "got {clean:?}");
    }

    // --- one-unknown rule: scheduled word (S5) ---

    #[test]
    fn flags_missing_scheduled_window_word() {
        let c = curriculum::load_embedded().unwrap();
        let window: BTreeSet<String> = ["mensaje".to_string()].into();
        let slot = SlotSpec {
            required_lemma: Some("mensaje".into()),
            ..Default::default()
        };

        // Item came back without the scheduled word.
        let violations = judge_full(
            &c,
            "opener.quiero",
            "Quiero comer.",
            &quiero_comer(),
            &window,
            &[],
            Some(&slot),
        );
        assert_eq!(
            violations,
            vec![Violation::ScheduledWordMissing {
                lemma: "mensaje".into()
            }]
        );

        // With the word present (as a content lemma) the item passes.
        let mut with_word = quiero_comer();
        with_word.content_lemmas = vec!["mensaje".into()];
        assert!(judge_full(
            &c,
            "opener.quiero",
            "Quiero leer el mensaje.",
            &with_word,
            &window,
            &[],
            Some(&slot),
        )
        .is_empty());
    }

    #[test]
    fn scheduled_verb_counts_via_its_lemma() {
        // A scheduled window *verb* shows up in verb_forms, not
        // content_lemmas (the analyzer excludes verbs there).
        let c = curriculum::load_embedded().unwrap();
        let window: BTreeSet<String> = ["nadar".to_string()].into();
        let slot = SlotSpec {
            required_lemma: Some("nadar".into()),
            ..Default::default()
        };
        let mut analysis = quiero_comer();
        analysis.verb_forms[1] = avf("nadar", "inf", "nadar");
        assert!(judge_full(
            &c,
            "opener.quiero",
            "Quiero nadar.",
            &analysis,
            &window,
            &[],
            Some(&slot),
        )
        .is_empty());
    }

    // --- fail-safe ---

    #[test]
    fn unplaceable_verb_form_is_rejected_not_waved_through() {
        // The analyzer's "other" escape: a compound tense it cannot place
        // in the slot registry must reject the item.
        let mut analysis = quiero_comer();
        analysis
            .verb_forms
            .push(avf("comer", "other", "hemos comido"));
        let violations = judge_unit("opener.quiero", "Hemos comido.", &analysis);
        assert!(violations.contains(&Violation::UnrecognizedFormSlot {
            lemma: "comer".into(),
            form: "other".into(),
            surface: "hemos comido".into(),
        }));
    }

    #[test]
    fn violations_serialize_machine_readably() {
        // The serialized shape is the repair-loop contract.
        let v = Violation::UnlicensedVerbForm {
            lemma: "querer".into(),
            form: "pres.3sg".into(),
            surface: "quiere".into(),
        };
        let json = serde_json::to_value(&v).unwrap();
        assert_eq!(json["kind"], "unlicensed_verb_form");
        assert_eq!(json["surface"], "quiere");
    }
}
