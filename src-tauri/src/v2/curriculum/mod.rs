//! V2 curriculum: licensing-set schema, loader, seed data, and the
//! effective-licensing dump command (S2, #33).
//!
//! The committed JSON files under `data/` are the source of truth:
//!
//! - `units.sample.json` — hand-authored sample units (MOC v2 Phases 1–2
//!   subset), enough to exercise the schema and loader. The real Phase 1–4
//!   authoring, with learner sign-off, is S3 (#34).
//! - `ambient_set.json` — the day-0 licensed base. Draft until S3.
//! - `power_verbs.json` — draft power-verb registry; the final ~45-verb
//!   list with paradigm-class rationale is settled in S3.
//! - `cognate_notes.json` — the five cognate transformation patterns from
//!   the six evicted v1 cognate units (the sixth was an interleave drill,
//!   which has no meaning as a note). Reference material only.

pub mod loader;
pub mod store;
pub mod types;

pub use loader::{load, Curriculum, CurriculumError};

use std::sync::Arc;

const UNITS_JSON: &str = include_str!("data/units.sample.json");
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
