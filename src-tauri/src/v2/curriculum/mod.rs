//! V2 curriculum: licensing-set schema, loader, seed data, and the
//! effective-licensing dump command (S2, #33).
//!
//! The committed JSON files under `data/` are the source of truth:
//!
//! - `units.json` — the authored curriculum, MOC v2 Phases 1–4 (S3, #34):
//!   openers, direct-object clitics, indirect/two-pronoun clitics, and
//!   question formation. Later phases are authored in subsequent slices.
//! - `ambient_set.json` — the finalized day-0 licensed base.
//! - `power_verbs.json` — the final 45-verb registry; paradigm-class
//!   coverage rationale lives in `data/README.md`.
//! - `cognate_notes.json` — the five cognate transformation patterns from
//!   the six evicted v1 cognate units (the sixth was an interleave drill,
//!   which has no meaning as a note). Reference material only.
//!
//! Authoring decisions, MOC deviations, and the learner sign-off checklist
//! are documented in `data/README.md`.

pub mod loader;
pub mod store;
pub mod types;

pub use loader::{load, Curriculum, CurriculumError};

use std::sync::Arc;

const UNITS_JSON: &str = include_str!("data/units.json");
const AMBIENT_JSON: &str = include_str!("data/ambient_set.json");
const POWER_VERBS_JSON: &str = include_str!("data/power_verbs.json");
const COGNATE_NOTES_JSON: &str = include_str!("data/cognate_notes.json");

/// Loads and validates the committed curriculum. Called at startup; an
/// error here is fatal by design — the app must not run on a curriculum
/// that fails DAG or licensing validation.
pub fn load_embedded() -> Result<Curriculum, CurriculumError> {
    load(UNITS_JSON, AMBIENT_JSON, POWER_VERBS_JSON, COGNATE_NOTES_JSON)
}

/// The validated curriculum, managed as Tauri state.
pub struct CurriculumState(pub Arc<Curriculum>);

pub fn dump_effective(
    curriculum: &Curriculum,
    unit_id: &str,
) -> Result<serde_json::Value, String> {
    match curriculum.effective_licensing(unit_id) {
        Some(eff) => Ok(serde_json::to_value(eff).expect("EffectiveLicensing serializes")),
        None => {
            let known: Vec<&str> = curriculum.units.iter().map(|u| u.id.as_str()).collect();
            Err(format!(
                "unknown unit `{}`; known units: {}",
                unit_id,
                known.join(", ")
            ))
        }
    }
}

/// Dev command: inspect a unit's effective licensing set (user story 24;
/// acceptance criterion "effective licensing set inspectable per unit").
/// Also available offline via `cargo run --bin dump_licensing -- <unit-id>`.
#[tauri::command]
pub fn dump_effective_licensing(
    unit_id: String,
    state: tauri::State<'_, CurriculumState>,
) -> Result<serde_json::Value, String> {
    dump_effective(&state.0, &unit_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_seed_curriculum_loads_and_validates() {
        let c = load_embedded().expect("committed curriculum data must pass validation");
        assert!(!c.units.is_empty());
        assert!(c.version >= 1);
    }

    #[test]
    fn seed_has_no_cognate_units_but_has_cognate_notes() {
        let c = load_embedded().unwrap();
        assert!(c.units.iter().all(|u| !u.id.contains("cognate")));
        assert_eq!(
            c.cognate_notes.notes.len(),
            5,
            "the five v1 cognate patterns live on as notes"
        );
        assert!(c.cognate_notes.version >= 1);
    }

    #[test]
    fn seed_power_verb_registry_is_versioned_and_covers_classes() {
        let c = load_embedded().unwrap();
        assert!(c.power_verbs.version >= 1);
        let classes: std::collections::HashSet<&str> = c
            .power_verbs
            .verbs
            .iter()
            .map(|v| v.class.as_str())
            .collect();
        for class in ["irregular-core", "regular-ar", "regular-er", "regular-ir"] {
            assert!(classes.contains(class), "registry missing class {class}");
        }
        assert!(c.power_verbs.verbs.iter().any(|v| v.lemma == "decir"));
    }

    #[test]
    fn seed_ambient_set_carries_day0_material() {
        let c = load_embedded().unwrap();
        assert!(c.ambient.version >= 1);
        let grant = &c.ambient.grant;
        assert!(grant.constructions.iter().any(|x| x.starts_with("neg.")));
        assert!(grant.constructions.iter().any(|x| x.starts_with("art.")));
        assert!(grant.vocab.contains(&"no".to_string()));
        assert!(!c.ambient.cognate_patterns.is_empty());
    }

    #[test]
    fn every_seed_unit_has_an_effective_licensing_set() {
        let c = load_embedded().unwrap();
        for u in &c.units {
            assert!(
                c.effective_licensing(&u.id).is_some(),
                "unit {} has no effective licensing set",
                u.id
            );
        }
    }

    // --- S3 (#34): authored Phase 1–4 curriculum ---

    #[test]
    fn s3_authors_all_eighteen_phase_1_to_4_units() {
        let c = load_embedded().unwrap();
        let expected: [(&str, u32); 18] = [
            ("opener.quiero", 1),
            ("opener.quiero.neg", 1),
            ("opener.puedo", 1),
            ("opener.debo", 1),
            ("opener.tengo-que", 1),
            ("opener.voy-a", 1),
            ("opener.mixed", 1),
            ("clitic.do.sg.attach", 2),
            ("clitic.do.pl.attach", 2),
            ("clitic.do.person.attach", 2),
            ("clitic.do.attach.mixed", 2),
            ("clitic.io.attach", 3),
            ("clitic.both.attach", 3),
            ("clitic.both.se-lo", 3),
            ("clitic.both.mixed", 3),
            ("question.yes-no", 4),
            ("question.wh", 4),
            ("question.embedded", 4),
        ];
        assert_eq!(c.units.len(), expected.len(), "exactly the 18 MOC Phase 1–4 units");
        for (id, phase) in expected {
            let u = c.unit(id).unwrap_or_else(|| panic!("missing unit `{id}`"));
            assert_eq!(u.phase, phase, "unit `{id}` phase");
        }
    }

    #[test]
    fn s3_power_verb_list_is_final_45_with_full_class_coverage() {
        let c = load_embedded().unwrap();
        assert_eq!(c.power_verbs.verbs.len(), 45);

        let mut lemmas = std::collections::HashSet::new();
        for v in &c.power_verbs.verbs {
            assert!(lemmas.insert(v.lemma.as_str()), "duplicate lemma {}", v.lemma);
        }

        // Every paradigm family the PRD names must have an exemplar:
        // regular families, spelling-change classes, stem-change families
        // across all three conjugations, and the irregular core.
        let classes: std::collections::HashSet<&str> = c
            .power_verbs
            .verbs
            .iter()
            .map(|v| v.class.as_str())
            .collect();
        for class in [
            "irregular-core",
            "regular-ar",
            "regular-er",
            "regular-ir",
            "spelling.-car",
            "spelling.-gar",
            "spelling.-zar",
            "spelling.-cer",
            "spelling.-cir",
            "spelling.-gir",
            "spelling.-guir",
            "spelling.-uir",
            "stem.e-ie.ar",
            "stem.e-ie.er",
            "stem.e-ie.ir",
            "stem.o-ue.ar",
            "stem.o-ue.er",
            "stem.o-ue.ir",
            "stem.e-i.ir",
            "stem.u-ue.ar",
        ] {
            assert!(classes.contains(class), "registry missing class {class}");
        }
    }

    #[test]
    fn s3_ambient_set_is_day0_grammar_only() {
        let c = load_embedded().unwrap();
        assert_eq!(c.ambient.version, 2, "finalized ambient set");
        let grant = &c.ambient.grant;
        // No verb forms day-0: the first licensed verb form is `quiero`,
        // granted by the first unit. Ambient is articles, pronouns,
        // particles, and pattern constructions only.
        assert!(grant.verb_forms.is_empty());
        assert!(grant.vocab_verb_forms.is_empty());
        for pron in ["yo", "tú", "él", "ella", "usted", "nosotros", "ellos", "ustedes"] {
            assert!(grant.vocab.contains(&pron.to_string()), "missing pronoun {pron}");
        }
        assert!(grant
            .constructions
            .contains(&"pron.subject.optional".to_string()));
        assert_eq!(c.ambient.cognate_patterns.len(), 5);
    }

    #[test]
    fn s3_interleave_units_grant_nothing_new() {
        let c = load_embedded().unwrap();
        for id in ["opener.mixed", "clitic.do.attach.mixed", "clitic.both.mixed"] {
            let g = &c.unit(id).unwrap().grant;
            assert!(
                g.verb_forms.is_empty()
                    && g.vocab_verb_forms.is_empty()
                    && g.constructions.is_empty()
                    && g.vocab.is_empty(),
                "interleave unit `{id}` must be a pure interleave"
            );
        }
    }

    #[test]
    fn s3_opener_mixed_licenses_all_five_opener_forms() {
        let c = load_embedded().unwrap();
        let eff = c.effective_licensing("opener.mixed").unwrap();
        for (lemma, surface) in [
            ("querer", "quiero"),
            ("poder", "puedo"),
            ("deber", "debo"),
            ("tener", "tengo"),
            ("ir", "voy"),
        ] {
            assert!(
                eff.verb_forms
                    .iter()
                    .any(|vf| vf.lemma == lemma && vf.form == "pres.1sg" && vf.surface == surface),
                "opener.mixed missing {surface}"
            );
        }
        assert!(eff.constructions.contains("neg.tampoco"));
        assert!(eff.vocab_verb_forms.iter().any(|v| v.form == "inf"));
    }

    #[test]
    fn s3_questions_license_tu_forms_of_all_five_openers() {
        let c = load_embedded().unwrap();
        let eff = c.effective_licensing("question.yes-no").unwrap();
        for surface in ["quieres", "puedes", "debes", "tienes", "vas"] {
            assert!(
                eff.verb_forms
                    .iter()
                    .any(|vf| vf.form == "pres.2sg" && vf.surface == surface),
                "question.yes-no missing {surface}"
            );
        }
    }

    #[test]
    fn s3_embedded_questions_can_ask_about_third_parties() {
        // "I want to know if she wants to come": saber (own grant), quiere
        // (inherited from clitic.do.person.attach), venir (inherited from
        // opener.puedo along the opener chain).
        let c = load_embedded().unwrap();
        let eff = c.effective_licensing("question.embedded").unwrap();
        for (lemma, form) in [("saber", "inf"), ("querer", "pres.3sg"), ("venir", "inf")] {
            assert!(
                eff.verb_forms
                    .iter()
                    .any(|vf| vf.lemma == lemma && vf.form == form),
                "question.embedded missing {lemma} {form}"
            );
        }
    }

    #[test]
    fn s3_se_lo_unit_inherits_the_full_two_pronoun_chain() {
        let c = load_embedded().unwrap();
        let eff = c.effective_licensing("clitic.both.se-lo").unwrap();
        for construction in [
            "clitic.do.sg.attach-to-inf",
            "clitic.io.attach-to-inf",
            "clitic.both.attach-to-inf",
            "clitic.both.se-lo-substitution",
        ] {
            assert!(
                eff.constructions.contains(construction),
                "clitic.both.se-lo missing construction {construction}"
            );
        }
        // "quiero dárselo": dar enumerated at clitic.io.attach.
        assert!(eff
            .verb_forms
            .iter()
            .any(|vf| vf.lemma == "dar" && vf.form == "inf"));
    }

    #[test]
    fn s3_clitic_pronouns_are_never_vocab_grants() {
        // la/los/las would collide with the ambient articles, and clitics
        // are function words; they ride their construction tags instead.
        let c = load_embedded().unwrap();
        for u in &c.units {
            for clitic in ["lo", "la", "los", "las", "me", "te", "nos", "le", "les", "se"] {
                assert!(
                    !u.grant.vocab.contains(&clitic.to_string()),
                    "unit `{}` grants clitic `{}` as vocab",
                    u.id,
                    clitic
                );
            }
        }
    }

    #[test]
    fn dump_returns_json_for_known_unit_and_error_for_unknown() {
        let c = load_embedded().unwrap();
        let dump = dump_effective(&c, "opener.mixed").unwrap();
        assert_eq!(dump["unit_id"], "opener.mixed");
        // The interleave unit grants nothing itself but inherits the
        // opener constructions through its prerequisites.
        let constructions = dump["constructions"].as_array().unwrap();
        assert!(constructions.iter().any(|c| c == "opener.finite+inf"));

        let err = dump_effective(&c, "nope").unwrap_err();
        assert!(err.contains("unknown unit"));
        assert!(err.contains("opener.quiero"), "error should list known units");
    }
}
