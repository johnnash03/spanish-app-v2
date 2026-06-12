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
    #[error("invalid target atom `{atom}` in unit `{unit}`: {reason}")]
    InvalidTarget {
        unit: String,
        atom: String,
        reason: String,
    },
    #[error(
        "unit `{0}` has no target spec and grants nothing to derive one \
         from; author a `target` (interleaves must name what they drill)"
    )]
    UntargetableUnit(String),
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
    targets: BTreeMap<String, TargetSpec>,
}

impl Curriculum {
    pub fn unit(&self, id: &str) -> Option<&Unit> {
        self.units.iter().find(|u| u.id == id)
    }

    pub fn effective_licensing(&self, unit_id: &str) -> Option<&EffectiveLicensing> {
        self.effective.get(unit_id)
    }

    /// The unit's resolved target-skill spec (authored or grant-derived).
    pub fn target_spec(&self, unit_id: &str) -> Option<&TargetSpec> {
        self.targets.get(unit_id)
    }

    pub fn effective_licensing_all(&self) -> impl Iterator<Item = &EffectiveLicensing> {
        self.effective.values()
    }

    /// Every construction tag any grant registers (ambient or unit) — the
    /// closed tag vocabulary the validator's analyzer must describe
    /// sentences in.
    pub fn construction_registry(&self) -> std::collections::BTreeSet<String> {
        self.ambient
            .grant
            .constructions
            .iter()
            .chain(self.units.iter().flat_map(|u| u.grant.constructions.iter()))
            .cloned()
            .collect()
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
    let targets = resolve_targets(units, &effective)?;

    Ok(Curriculum {
        version: units_file.curriculum_version,
        ambient,
        power_verbs,
        cognate_notes,
        units: units_file.units,
        effective,
        targets,
    })
}

/// Resolves every unit's target-skill spec. Authored atoms must name
/// elements inside the unit's effective licensing — a target the unit's
/// own exercises could never legally satisfy is an authoring error. With
/// no authored target, a single any-of group over the unit's own grant
/// (its newly licensed forms and constructions) is derived.
fn resolve_targets(
    units: &[Unit],
    effective: &BTreeMap<String, EffectiveLicensing>,
) -> Result<BTreeMap<String, TargetSpec>, CurriculumError> {
    let mut targets = BTreeMap::new();
    for u in units {
        let eff = &effective[&u.id];
        let spec = if u.target.is_empty() {
            let group: Vec<TargetAtom> = u
                .grant
                .verb_forms
                .iter()
                .map(|vf| TargetAtom::Form {
                    lemma: vf.lemma.clone(),
                    form: vf.form.clone(),
                })
                .chain(
                    u.grant
                        .constructions
                        .iter()
                        .map(|c| TargetAtom::Construction(c.clone())),
                )
                .collect();
            if group.is_empty() {
                return Err(CurriculumError::UntargetableUnit(u.id.clone()));
            }
            TargetSpec { groups: vec![group] }
        } else {
            let mut groups = Vec::with_capacity(u.target.len());
            for authored in &u.target {
                let mut group = Vec::with_capacity(authored.len());
                for atom in authored {
                    group.push(parse_target_atom(&u.id, atom, eff)?);
                }
                groups.push(group);
            }
            TargetSpec { groups }
        };
        targets.insert(u.id.clone(), spec);
    }
    Ok(targets)
}

fn parse_target_atom(
    unit: &str,
    atom: &str,
    eff: &EffectiveLicensing,
) -> Result<TargetAtom, CurriculumError> {
    let invalid = |reason: &str| CurriculumError::InvalidTarget {
        unit: unit.to_string(),
        atom: atom.to_string(),
        reason: reason.to_string(),
    };

    if let Some(tag) = atom.strip_prefix("construction:") {
        if !eff.constructions.contains(tag) {
            return Err(invalid("construction is not licensed for this unit"));
        }
        Ok(TargetAtom::Construction(tag.to_string()))
    } else if let Some(spec) = atom.strip_prefix("form:") {
        let (lemma, form) = spec
            .split_once('@')
            .ok_or_else(|| invalid("expected form:<lemma>@<form-slot>"))?;
        if !eff
            .verb_forms
            .iter()
            .any(|vf| vf.lemma == lemma && vf.form == form)
        {
            return Err(invalid("verb form is not licensed for this unit"));
        }
        Ok(TargetAtom::Form {
            lemma: lemma.to_string(),
            form: form.to_string(),
        })
    } else {
        Err(invalid("expected a `form:` or `construction:` prefix"))
    }
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

/// Loads a curriculum from inline units JSON over a minimal ambient set,
/// power-verb registry, and cognate notes — for tests (here and in the
/// validator) that need a curriculum the committed data doesn't exhibit.
#[cfg(test)]
pub fn load_units_for_tests(units_json: &str) -> Curriculum {
    let ambient = r#"{
        "version": 1,
        "grant": {
            "constructions": ["neg.no", "art.def"],
            "vocab": ["no", "el", "la"]
        },
        "cognate_patterns": []
    }"#;
    let power_verbs = r#"{
        "version": 1,
        "verbs": [
            {"lemma": "querer", "english": "to want", "class": "stem.e-ie"},
            {"lemma": "poder", "english": "to be able to", "class": "stem.o-ue"}
        ]
    }"#;
    let cognate_notes = r#"{"version": 1, "notes": []}"#;
    let units_file = format!(r#"{{"curriculum_version": 1, "units": {}}}"#, units_json);
    load(&units_file, ambient, power_verbs, cognate_notes)
        .expect("test curriculum must load")
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
         "grant": {"constructions": ["clitic.both.attach"], "vocab": ["ahora"]}}
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
                 "grant": {"constructions": ["opener.modal-inf"], "vocab": ["comer"]}},
                {"id": "mixed", "title": "Interleave", "phase": 1, "prereqs": ["a"],
                 "target": [["construction:opener.modal-inf"]]}]"#,
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

    // --- target specs (S4, #35) ---

    #[test]
    fn authored_target_resolves_to_atoms() {
        let c = load_units(
            r#"[{
                "id": "a", "title": "A", "phase": 1,
                "grant": {
                    "verb_forms": [{"lemma": "querer", "form": "pres.1sg", "surface": "quiero"}],
                    "constructions": ["opener.modal-inf"]
                },
                "target": [["construction:neg.no", "construction:opener.modal-inf"],
                           ["form:querer@pres.1sg"]]
            }]"#,
        )
        .unwrap();
        let spec = c.target_spec("a").unwrap();
        assert_eq!(spec.groups.len(), 2);
        assert_eq!(
            spec.groups[0],
            vec![
                TargetAtom::Construction("neg.no".into()),
                TargetAtom::Construction("opener.modal-inf".into()),
            ]
        );
        assert_eq!(
            spec.groups[1],
            vec![TargetAtom::Form { lemma: "querer".into(), form: "pres.1sg".into() }]
        );
    }

    #[test]
    fn missing_target_defaults_to_any_own_grant_element() {
        let c = load_units(
            r#"[{
                "id": "a", "title": "A", "phase": 1,
                "grant": {
                    "verb_forms": [{"lemma": "querer", "form": "pres.1sg", "surface": "quiero"}],
                    "constructions": ["opener.modal-inf"]
                }
            }]"#,
        )
        .unwrap();
        let spec = c.target_spec("a").unwrap();
        // One any-of group spanning the unit's own grant.
        assert_eq!(spec.groups.len(), 1);
        assert!(spec.groups[0].contains(&TargetAtom::Form {
            lemma: "querer".into(),
            form: "pres.1sg".into()
        }));
        assert!(spec.groups[0].contains(&TargetAtom::Construction("opener.modal-inf".into())));
    }

    #[test]
    fn rejects_untargetable_unit() {
        // An interleave grants nothing, so a target cannot be derived; it
        // must be authored.
        let err = load_units(
            r#"[{"id": "a", "title": "A", "phase": 1,
                 "grant": {"constructions": ["opener.modal-inf"], "vocab": ["comer"]}},
                {"id": "mixed", "title": "Interleave", "phase": 1, "prereqs": ["a"]}]"#,
        )
        .unwrap_err();
        assert!(matches!(err, CurriculumError::UntargetableUnit(id) if id == "mixed"));
    }

    #[test]
    fn rejects_target_atom_the_unit_does_not_license() {
        let err = load_units(
            r#"[{
                "id": "a", "title": "A", "phase": 1,
                "grant": {"constructions": ["opener.modal-inf"]},
                "target": [["form:poder@pres.1sg"]]
            }]"#,
        )
        .unwrap_err();
        assert!(matches!(
            err,
            CurriculumError::InvalidTarget { unit, atom, .. }
                if unit == "a" && atom == "form:poder@pres.1sg"
        ));
    }

    #[test]
    fn rejects_malformed_target_atom() {
        let err = load_units(
            r#"[{
                "id": "a", "title": "A", "phase": 1,
                "grant": {"constructions": ["opener.modal-inf"]},
                "target": [["opener.modal-inf"]]
            }]"#,
        )
        .unwrap_err();
        assert!(matches!(err, CurriculumError::InvalidTarget { .. }));
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
