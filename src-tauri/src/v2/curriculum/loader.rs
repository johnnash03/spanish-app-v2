//! Curriculum loader (S2, #33). Parses the curriculum data files and
//! refuses to start on structural rot: DAG cycles, unknown references,
//! cognate units smuggled back into the sequence, ambiguous grants, or
//! non-monotonic licensing. Computes each unit's effective licensing set.

use super::types::*;
use std::collections::{BTreeMap, HashMap, HashSet};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurriculumError {
    #[error("failed to parse {file}: {source}")]
    Parse {
        file: &'static str,
        #[source]
        source: serde_json::Error,
    },
    #[error("duplicate unit id `{0}`")]
    DuplicateUnitId(String),
    #[error("unit `{unit}` lists unknown prerequisite `{prereq}`")]
    UnknownPrereq { unit: String, prereq: String },
    #[error("prerequisite cycle: {}", .0.join(" -> "))]
    Cycle(Vec<String>),
    #[error(
        "cognate unit `{0}` in unit sequence; cognate patterns are reference \
         notes, never drill units or stacking tags (PRD #31)"
    )]
    CognateUnit(String),
    #[error("unknown verb-form slot `{form}` in {context}")]
    UnknownFormSlot { context: String, form: String },
    #[error(
        "unit `{unit}` grants a conjugated form of `{lemma}`, which is not in \
         the power-verb registry"
    )]
    UnknownPowerVerb { unit: String, lemma: String },
    #[error(
        "licensing element {element} granted by both `{first}` and `{second}`; \
         every element must have exactly one granting source"
    )]
    DuplicateGrant {
        element: String,
        first: String,
        second: String,
    },
    #[error(
        "non-monotonic licensing: unit `{unit}` is missing {element} licensed \
         by its prerequisite `{prereq}`"
    )]
    NonMonotonic {
        unit: String,
        prereq: String,
        element: String,
    },
    #[error("ambient set references unknown cognate pattern `{0}`")]
    UnknownCognatePattern(String),
}

/// The loaded, validated curriculum. Construction is only possible through
/// [`load`], so holding a `Curriculum` is proof the data passed every check.
#[derive(Debug)]
pub struct Curriculum {
    pub version: u32,
    pub ambient: AmbientSet,
    pub power_verbs: PowerVerbRegistry,
    pub cognate_notes: CognateNotes,
    /// Units in authored order.
    pub units: Vec<Unit>,
    effective: BTreeMap<String, EffectiveLicensing>,
}

impl Curriculum {
    pub fn unit(&self, id: &str) -> Option<&Unit> {
        self.units.iter().find(|u| u.id == id)
    }

    pub fn effective_licensing(&self, unit_id: &str) -> Option<&EffectiveLicensing> {
        self.effective.get(unit_id)
    }

    pub fn effective_licensing_all(&self) -> impl Iterator<Item = &EffectiveLicensing> {
        self.effective.values()
    }
}

pub fn load(
    units_json: &str,
    ambient_json: &str,
    power_verbs_json: &str,
    cognate_notes_json: &str,
) -> Result<Curriculum, CurriculumError> {
    let units_file: UnitsFile = serde_json::from_str(units_json).map_err(|e| {
        CurriculumError::Parse { file: "units", source: e }
    })?;
    let ambient: AmbientSet = serde_json::from_str(ambient_json).map_err(|e| {
        CurriculumError::Parse { file: "ambient_set", source: e }
    })?;
    let power_verbs: PowerVerbRegistry =
        serde_json::from_str(power_verbs_json).map_err(|e| CurriculumError::Parse {
            file: "power_verbs",
            source: e,
        })?;
    let cognate_notes: CognateNotes =
        serde_json::from_str(cognate_notes_json).map_err(|e| CurriculumError::Parse {
            file: "cognate_notes",
            source: e,
        })?;

    let units = &units_file.units;

    validate_unit_ids(units)?;
    validate_cognate_eviction(units)?;
    validate_prereq_refs(units)?;
    validate_form_slots(units, &ambient)?;
    validate_power_verb_refs(units, &ambient, &power_verbs)?;
    validate_grant_uniqueness(units, &ambient)?;
    validate_cognate_pattern_refs(&ambient, &cognate_notes)?;

    let topo_order = topological_order(units)?;
    let effective = compute_effective_sets(
        units,
        &ambient,
        units_file.curriculum_version,
        &topo_order,
    );
    check_monotonicity(units, &effective)?;

    Ok(Curriculum {
        version: units_file.curriculum_version,
        ambient,
        power_verbs,
        cognate_notes,
        units: units_file.units,
        effective,
    })
}

fn validate_unit_ids(units: &[Unit]) -> Result<(), CurriculumError> {
    let mut seen = HashSet::new();
    for u in units {
        if !seen.insert(u.id.as_str()) {
            return Err(CurriculumError::DuplicateUnitId(u.id.clone()));
        }
    }
    Ok(())
}

fn validate_cognate_eviction(units: &[Unit]) -> Result<(), CurriculumError> {
    // The six v1 cognate units were tagged `lex.cognate.*`; reject any tag
    // with a `cognate` segment so they cannot re-enter under a new prefix.
    for u in units {
        if u.id.split('.').any(|seg| seg == "cognate") {
            return Err(CurriculumError::CognateUnit(u.id.clone()));
        }
    }
    Ok(())
}

fn validate_prereq_refs(units: &[Unit]) -> Result<(), CurriculumError> {
    let known: HashSet<&str> = units.iter().map(|u| u.id.as_str()).collect();
    for u in units {
        for p in &u.prereqs {
            if !known.contains(p.as_str()) {
                return Err(CurriculumError::UnknownPrereq {
                    unit: u.id.clone(),
                    prereq: p.clone(),
                });
            }
        }
    }
    Ok(())
}

fn validate_grant_form_slots(
    grant: &LicensingGrant,
    context: &str,
) -> Result<(), CurriculumError> {
    for vf in &grant.verb_forms {
        if !is_known_form_slot(&vf.form) {
            return Err(CurriculumError::UnknownFormSlot {
                context: context.to_string(),
                form: vf.form.clone(),
            });
        }
    }
    for vvf in &grant.vocab_verb_forms {
        if !is_known_form_slot(&vvf.form) {
            return Err(CurriculumError::UnknownFormSlot {
                context: context.to_string(),
                form: vvf.form.clone(),
            });
        }
    }
    Ok(())
}

fn validate_form_slots(units: &[Unit], ambient: &AmbientSet) -> Result<(), CurriculumError> {
    validate_grant_form_slots(&ambient.grant, "ambient set")?;
    for u in units {
        validate_grant_form_slots(&u.grant, &format!("unit `{}`", u.id))?;
    }
    Ok(())
}

fn validate_power_verb_refs(
    units: &[Unit],
    ambient: &AmbientSet,
    registry: &PowerVerbRegistry,
) -> Result<(), CurriculumError> {
    let known: HashSet<&str> = registry.verbs.iter().map(|v| v.lemma.as_str()).collect();
    let check = |grant: &LicensingGrant, owner: &str| -> Result<(), CurriculumError> {
        for vf in &grant.verb_forms {
            if !known.contains(vf.lemma.as_str()) {
                return Err(CurriculumError::UnknownPowerVerb {
                    unit: owner.to_string(),
                    lemma: vf.lemma.clone(),
                });
            }
        }
        Ok(())
    };
    check(&ambient.grant, "ambient set")?;
    for u in units {
        check(&u.grant, &u.id)?;
    }
    Ok(())
}

/// Every licensing element must be granted exactly once across the ambient
/// set and all units. A re-grant means the authored ordering is ambiguous
/// ("which unit teaches this?") and is the practical way licensing
/// monotonicity breaks during authoring.
fn validate_grant_uniqueness(
    units: &[Unit],
    ambient: &AmbientSet,
) -> Result<(), CurriculumError> {
    let mut sources: HashMap<String, String> = HashMap::new();
    let mut claim = |element: String, owner: &str| -> Result<(), CurriculumError> {
        if let Some(first) = sources.get(&element) {
            return Err(CurriculumError::DuplicateGrant {
                element,
                first: first.clone(),
                second: owner.to_string(),
            });
        }
        sources.insert(element, owner.to_string());
        Ok(())
    };

    let mut claim_grant = |grant: &LicensingGrant, owner: &str| -> Result<(), CurriculumError> {
        for vf in &grant.verb_forms {
            claim(format!("verb form `{} {}`", vf.lemma, vf.form), owner)?;
        }
        for vvf in &grant.vocab_verb_forms {
            let classes = vvf
                .classes
                .as_ref()
                .map(|c| c.join(","))
                .unwrap_or_else(|| "*".to_string());
            claim(format!("vocab form slot `{}` [{}]", vvf.form, classes), owner)?;
        }
        for c in &grant.constructions {
            claim(format!("construction `{}`", c), owner)?;
        }
        for w in &grant.vocab {
            claim(format!("vocab `{}`", w), owner)?;
        }
        Ok(())
    };

    claim_grant(&ambient.grant, "ambient set")?;
    for u in units {
        claim_grant(&u.grant, &u.id)?;
    }
    Ok(())
}

fn validate_cognate_pattern_refs(
    ambient: &AmbientSet,
    notes: &CognateNotes,
) -> Result<(), CurriculumError> {
    let known: HashSet<&str> = notes.notes.iter().map(|n| n.id.as_str()).collect();
    for p in &ambient.cognate_patterns {
        if !known.contains(p.as_str()) {
            return Err(CurriculumError::UnknownCognatePattern(p.clone()));
        }
    }
    Ok(())
}

/// DFS three-color cycle detection; returns unit ids in topological order
/// (prerequisites before dependents) or the offending cycle path.
fn topological_order(units: &[Unit]) -> Result<Vec<String>, CurriculumError> {
    let by_id: HashMap<&str, &Unit> = units.iter().map(|u| (u.id.as_str(), u)).collect();

    #[derive(Clone, Copy, PartialEq)]
    enum Color {
        White,
        Gray,
        Black,
    }
    let mut color: HashMap<&str, Color> =
        units.iter().map(|u| (u.id.as_str(), Color::White)).collect();
    let mut order: Vec<String> = Vec::with_capacity(units.len());

    fn visit<'a>(
        id: &'a str,
        by_id: &HashMap<&'a str, &'a Unit>,
        color: &mut HashMap<&'a str, Color>,
        order: &mut Vec<String>,
        path: &mut Vec<&'a str>,
    ) -> Result<(), CurriculumError> {
        color.insert(id, Color::Gray);
        path.push(id);
        for p in &by_id[id].prereqs {
            match color[p.as_str()] {
                Color::Black => {}
                Color::Gray => {
                    // Cycle: slice the current path from the repeated node.
                    let start = path.iter().position(|n| *n == p.as_str()).unwrap();
                    let mut cycle: Vec<String> =
                        path[start..].iter().map(|s| s.to_string()).collect();
                    cycle.push(p.clone());
                    return Err(CurriculumError::Cycle(cycle));
                }
                Color::White => visit(p.as_str(), by_id, color, order, path)?,
            }
        }
        path.pop();
        color.insert(id, Color::Black);
        order.push(id.to_string());
        Ok(())
    }

    for u in units {
        if color[u.id.as_str()] == Color::White {
            visit(u.id.as_str(), &by_id, &mut color, &mut order, &mut Vec::new())?;
        }
    }
    Ok(order)
}

/// Effective licensing per unit: ambient ∪ (every ancestor's grant) ∪ own
/// grant, computed in topological order so each prerequisite's effective
/// set is final before its dependents union it in.
fn compute_effective_sets(
    units: &[Unit],
    ambient: &AmbientSet,
    curriculum_version: u32,
    topo_order: &[String],
) -> BTreeMap<String, EffectiveLicensing> {
    let by_id: HashMap<&str, &Unit> = units.iter().map(|u| (u.id.as_str(), u)).collect();
    let mut effective: BTreeMap<String, EffectiveLicensing> = BTreeMap::new();

    for id in topo_order {
        let unit = by_id[id.as_str()];
        let mut eff = EffectiveLicensing {
            unit_id: id.clone(),
            curriculum_version,
            ambient_version: ambient.version,
            verb_forms: ambient.grant.verb_forms.iter().cloned().collect(),
            vocab_verb_forms: ambient.grant.vocab_verb_forms.iter().cloned().collect(),
            constructions: ambient.grant.constructions.iter().cloned().collect(),
            vocab: ambient.grant.vocab.iter().cloned().collect(),
        };
        for p in &unit.prereqs {
            let pe = &effective[p];
            eff.verb_forms.extend(pe.verb_forms.iter().cloned());
            eff.vocab_verb_forms.extend(pe.vocab_verb_forms.iter().cloned());
            eff.constructions.extend(pe.constructions.iter().cloned());
            eff.vocab.extend(pe.vocab.iter().cloned());
        }
        eff.verb_forms.extend(unit.grant.verb_forms.iter().cloned());
        eff.vocab_verb_forms
            .extend(unit.grant.vocab_verb_forms.iter().cloned());
        eff.constructions
            .extend(unit.grant.constructions.iter().cloned());
        eff.vocab.extend(unit.grant.vocab.iter().cloned());

        effective.insert(id.clone(), eff);
    }
    effective
}

/// Invariant check: along every prerequisite edge, the dependent's
/// effective set must contain everything the prerequisite licenses. The
/// union construction makes this hold for sets computed here; the check
/// guards stored artifacts and future changes to the computation.
pub fn check_monotonicity(
    units: &[Unit],
    effective: &BTreeMap<String, EffectiveLicensing>,
) -> Result<(), CurriculumError> {
    for u in units {
        let ue = &effective[&u.id];
        for p in &u.prereqs {
            let pe = &effective[p];
            let missing: Option<String> = pe
                .verb_forms
                .iter()
                .find(|vf| !ue.verb_forms.contains(*vf))
                .map(|vf| format!("verb form `{} {}`", vf.lemma, vf.form))
                .or_else(|| {
                    pe.vocab_verb_forms
                        .iter()
                        .find(|v| !ue.vocab_verb_forms.contains(*v))
                        .map(|v| format!("vocab form slot `{}`", v.form))
                })
                .or_else(|| {
                    pe.constructions
                        .iter()
                        .find(|c| !ue.constructions.contains(*c))
                        .map(|c| format!("construction `{}`", c))
                })
                .or_else(|| {
                    pe.vocab
                        .iter()
                        .find(|w| !ue.vocab.contains(*w))
                        .map(|w| format!("vocab `{}`", w))
                });
            if let Some(element) = missing {
                return Err(CurriculumError::NonMonotonic {
                    unit: u.id.clone(),
                    prereq: p.clone(),
                    element,
                });
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const AMBIENT: &str = r#"{
        "version": 1,
        "grant": {
            "constructions": ["neg.no", "art.def"],
            "vocab": ["no", "el", "la"]
        },
        "cognate_patterns": ["cognate.tion"]
    }"#;

    const POWER_VERBS: &str = r#"{
        "version": 1,
        "verbs": [
            {"lemma": "querer", "english": "to want", "class": "stem.e-ie"},
            {"lemma": "poder", "english": "to be able to", "class": "stem.o-ue"}
        ]
    }"#;

    const COGNATE_NOTES: &str = r#"{
        "version": 1,
        "notes": [
            {"id": "cognate.tion", "pattern": "-tion → -ción",
             "description": "English -tion nouns become -ción (feminine).",
             "examples": [{"en": "information", "es": "información"}]}
        ]
    }"#;

    fn units_file(units_json: &str) -> String {
        format!(r#"{{"curriculum_version": 1, "units": {}}}"#, units_json)
    }

    fn load_units(units_json: &str) -> Result<Curriculum, CurriculumError> {
        load(&units_file(units_json), AMBIENT, POWER_VERBS, COGNATE_NOTES)
    }

    #[test]
    fn loads_minimal_curriculum() {
        let c = load_units(
            r#"[{
                "id": "opener.quiero",
                "title": "Quiero + infinitive",
                "phase": 1,
                "grant": {
                    "verb_forms": [{"lemma": "querer", "form": "pres.1sg", "surface": "quiero"}],
                    "vocab_verb_forms": [{"form": "inf"}],
                    "constructions": ["opener.modal-inf"],
                    "vocab": ["comer", "esperar"]
                }
            }]"#,
        )
        .unwrap();
        assert_eq!(c.version, 1);
        assert_eq!(c.units.len(), 1);
        assert_eq!(c.ambient.version, 1);
        assert_eq!(c.power_verbs.verbs.len(), 2);
        assert!(c.unit("opener.quiero").is_some());
    }

    #[test]
    fn rejects_duplicate_unit_ids() {
        let err = load_units(
            r#"[{"id": "a", "title": "A", "phase": 1},
                {"id": "a", "title": "A again", "phase": 1}]"#,
        )
        .unwrap_err();
        assert!(matches!(err, CurriculumError::DuplicateUnitId(id) if id == "a"));
    }

    #[test]
    fn rejects_unknown_prereq() {
        let err = load_units(
            r#"[{"id": "a", "title": "A", "phase": 1, "prereqs": ["ghost"]}]"#,
        )
        .unwrap_err();
        assert!(matches!(
            err,
            CurriculumError::UnknownPrereq { unit, prereq } if unit == "a" && prereq == "ghost"
        ));
    }

    #[test]
    fn rejects_prereq_cycle_and_names_it() {
        let err = load_units(
            r#"[{"id": "a", "title": "A", "phase": 1, "prereqs": ["c"]},
                {"id": "b", "title": "B", "phase": 1, "prereqs": ["a"]},
                {"id": "c", "title": "C", "phase": 1, "prereqs": ["b"]}]"#,
        )
        .unwrap_err();
        match err {
            CurriculumError::Cycle(path) => {
                assert!(path.len() >= 4, "cycle path should name every node: {:?}", path);
                assert_eq!(path.first(), path.last());
            }
            other => panic!("expected Cycle, got {:?}", other),
        }
    }

    #[test]
    fn rejects_self_prereq() {
        let err = load_units(
            r#"[{"id": "a", "title": "A", "phase": 1, "prereqs": ["a"]}]"#,
        )
        .unwrap_err();
        assert!(matches!(err, CurriculumError::Cycle(_)));
    }

    #[test]
    fn rejects_cognate_unit_in_sequence() {
        // The exact v1 tag shape that caused the flood.
        let err = load_units(
            r#"[{"id": "lex.cognate.tion", "title": "-tion words", "phase": 0}]"#,
        )
        .unwrap_err();
        assert!(matches!(err, CurriculumError::CognateUnit(id) if id == "lex.cognate.tion"));
    }

    #[test]
    fn rejects_unknown_form_slot() {
        // "pres" alone is a tense name, not an enumerated form — exactly
        // what the schema exists to forbid.
        let err = load_units(
            r#"[{"id": "a", "title": "A", "phase": 1,
                 "grant": {"verb_forms": [{"lemma": "querer", "form": "pres", "surface": "quiere"}]}}]"#,
        )
        .unwrap_err();
        assert!(matches!(
            err,
            CurriculumError::UnknownFormSlot { form, .. } if form == "pres"
        ));
    }

    #[test]
    fn rejects_verb_form_grant_for_unregistered_lemma() {
        let err = load_units(
            r#"[{"id": "a", "title": "A", "phase": 1,
                 "grant": {"verb_forms": [{"lemma": "bailar", "form": "pres.1sg", "surface": "bailo"}]}}]"#,
        )
        .unwrap_err();
        assert!(matches!(
            err,
            CurriculumError::UnknownPowerVerb { lemma, .. } if lemma == "bailar"
        ));
    }

    #[test]
    fn rejects_element_granted_by_two_units() {
        let err = load_units(
            r#"[{"id": "a", "title": "A", "phase": 1,
                 "grant": {"verb_forms": [{"lemma": "querer", "form": "pres.1sg", "surface": "quiero"}]}},
                {"id": "b", "title": "B", "phase": 1,
                 "grant": {"verb_forms": [{"lemma": "querer", "form": "pres.1sg", "surface": "quiero"}]}}]"#,
        )
        .unwrap_err();
        assert!(matches!(err, CurriculumError::DuplicateGrant { .. }));
    }

    #[test]
    fn rejects_unit_regranting_ambient_material() {
        let err = load_units(
            r#"[{"id": "a", "title": "A", "phase": 1,
                 "grant": {"vocab": ["no"]}}]"#,
        )
        .unwrap_err();
        assert!(matches!(
            err,
            CurriculumError::DuplicateGrant { first, .. } if first == "ambient set"
        ));
    }

    #[test]
    fn rejects_unknown_cognate_pattern_reference() {
        let ambient = r#"{
            "version": 1,
            "grant": {},
            "cognate_patterns": ["cognate.ghost"]
        }"#;
        let err = load(
            &units_file(r#"[{"id": "a", "title": "A", "phase": 1}]"#),
            ambient,
            POWER_VERBS,
            COGNATE_NOTES,
        )
        .unwrap_err();
        assert!(matches!(
            err,
            CurriculumError::UnknownCognatePattern(p) if p == "cognate.ghost"
        ));
    }

    // --- effective licensing ---

    /// a → b → c plus a → d; c also depends on d (diamond-ish shape).
    const DAG: &str = r#"[
        {"id": "a", "title": "A", "phase": 1,
         "grant": {"verb_forms": [{"lemma": "querer", "form": "pres.1sg", "surface": "quiero"}],
                    "vocab_verb_forms": [{"form": "inf"}],
                    "constructions": ["opener.modal-inf"],
                    "vocab": ["comer"]}},
        {"id": "b", "title": "B", "phase": 1, "prereqs": ["a"],
         "grant": {"constructions": ["clitic.do.attach"], "vocab": ["lo"]}},
        {"id": "d", "title": "D", "phase": 1, "prereqs": ["a"],
         "grant": {"verb_forms": [{"lemma": "poder", "form": "pres.1sg", "surface": "puedo"}]}},
        {"id": "c", "title": "C", "phase": 2, "prereqs": ["b", "d"],
         "grant": {"vocab": ["ahora"]}}
    ]"#;

    #[test]
    fn effective_set_unions_ambient_ancestors_and_own_grant() {
        let c = load_units(DAG).unwrap();
        let eff = c.effective_licensing("c").unwrap();

        // Own grant
        assert!(eff.vocab.contains("ahora"));
        // From direct prereq b
        assert!(eff.constructions.contains("clitic.do.attach"));
        assert!(eff.vocab.contains("lo"));
        // From transitive ancestor a
        assert!(eff
            .verb_forms
            .iter()
            .any(|vf| vf.lemma == "querer" && vf.form == "pres.1sg"));
        assert!(eff.vocab_verb_forms.iter().any(|v| v.form == "inf"));
        // From the other branch d
        assert!(eff
            .verb_forms
            .iter()
            .any(|vf| vf.lemma == "poder" && vf.form == "pres.1sg"));
        // Ambient everywhere
        assert!(eff.constructions.contains("neg.no"));
        assert!(eff.vocab.contains("no"));
        assert_eq!(eff.curriculum_version, 1);
        assert_eq!(eff.ambient_version, 1);
    }

    #[test]
    fn prereq_grants_do_not_leak_into_unrelated_units() {
        let c = load_units(DAG).unwrap();
        let eff_b = c.effective_licensing("b").unwrap();
        // b does not depend on d, so puedo must not be licensed there.
        assert!(!eff_b.verb_forms.iter().any(|vf| vf.lemma == "poder"));
    }

    #[test]
    fn unit_with_empty_grant_is_legal_and_inherits_everything() {
        let c = load_units(
            r#"[{"id": "a", "title": "A", "phase": 1,
                 "grant": {"vocab": ["comer"]}},
                {"id": "mixed", "title": "Interleave", "phase": 1, "prereqs": ["a"]}]"#,
        )
        .unwrap();
        let eff = c.effective_licensing("mixed").unwrap();
        assert!(eff.vocab.contains("comer"));
        assert!(eff.constructions.contains("neg.no"));
    }

    #[test]
    fn effective_sets_are_monotone_along_every_edge() {
        let c = load_units(DAG).unwrap();
        check_monotonicity(&c.units, &c.effective).unwrap();
    }

    #[test]
    fn monotonicity_check_rejects_corrupted_sets() {
        let c = load_units(DAG).unwrap();
        let mut corrupted = c.effective.clone();
        // Strip an inherited element from c's stored set: now c lacks
        // something its prerequisite b licenses.
        corrupted.get_mut("c").unwrap().vocab.remove("lo");
        let err = check_monotonicity(&c.units, &corrupted).unwrap_err();
        assert!(matches!(
            err,
            CurriculumError::NonMonotonic { unit, prereq, .. }
                if unit == "c" && prereq == "b"
        ));
    }
}
