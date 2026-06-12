//! V2 curriculum schema (S2, #33). The licensing set is the keystone
//! artifact (PRD #31): every unit declares, as data, exactly which verb
//! forms, constructions, and vocabulary its exercises may use. Forms are
//! enumerated individually — never tense names like "present".

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// Canonical registry of individually addressable verb-form slots.
/// A licensing grant may only reference slots listed here; a typo'd or
/// invented slot is a load-time error. Persons exclude vosotros and voseo
/// (the curriculum teaches a neutral standard, MOC v2).
pub const FORM_SLOTS: &[&str] = &[
    // Non-finite
    "inf", "ger", "part",
    // Present indicative
    "pres.1sg", "pres.2sg", "pres.3sg", "pres.1pl", "pres.3pl",
    // Preterite
    "pret.1sg", "pret.2sg", "pret.3sg", "pret.1pl", "pret.3pl",
    // Imperfect
    "imperf.1sg", "imperf.2sg", "imperf.3sg", "imperf.1pl", "imperf.3pl",
    // Future
    "fut.1sg", "fut.2sg", "fut.3sg", "fut.1pl", "fut.3pl",
    // Conditional
    "cond.1sg", "cond.2sg", "cond.3sg", "cond.1pl", "cond.3pl",
    // Present subjunctive
    "subj.pres.1sg", "subj.pres.2sg", "subj.pres.3sg", "subj.pres.1pl", "subj.pres.3pl",
    // Imperfect subjunctive (-ra production form; -se is an accepted variant)
    "subj.imperf.1sg", "subj.imperf.2sg", "subj.imperf.3sg", "subj.imperf.1pl",
    "subj.imperf.3pl",
    // Imperatives (compound tenses are constructions over haber + part)
    "imp.aff.2sg", "imp.aff.3sg", "imp.aff.1pl", "imp.aff.3pl",
    "imp.neg.2sg", "imp.neg.3sg", "imp.neg.1pl", "imp.neg.3pl",
];

pub fn is_known_form_slot(form: &str) -> bool {
    FORM_SLOTS.contains(&form)
}

/// One enumerated verb form: a specific lemma in a specific paradigm cell.
/// `lemma` must name a registered power verb — conjugated-form licensing is
/// anchored to the power-verb registry; open-class vocabulary verbs are
/// licensed per form slot via [`VocabFormGrant`] instead.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct VerbFormGrant {
    pub lemma: String,
    pub form: String,
    pub surface: String,
}

/// Licenses a form slot for vocabulary (non-enumerated) verbs, optionally
/// restricted to conjugation classes ("ar", "er", "ir"). `classes: None`
/// means any licensed vocabulary verb.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct VocabFormGrant {
    pub form: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classes: Option<Vec<String>>,
}

/// What one unit (or the ambient set) newly licenses. Effective licensing
/// for a unit is the union of the ambient set, every ancestor's grant, and
/// the unit's own grant — grants are deltas, never restatements.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LicensingGrant {
    #[serde(default)]
    pub verb_forms: Vec<VerbFormGrant>,
    #[serde(default)]
    pub vocab_verb_forms: Vec<VocabFormGrant>,
    #[serde(default)]
    pub constructions: Vec<String>,
    #[serde(default)]
    pub vocab: Vec<String>,
}

/// A drill unit: one micro-skill, its prerequisites (DAG edges), and its
/// licensing grant. The id is the machine-readable skill tag
/// (`opener.quiero`, `clitic.both.se-lo`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    pub id: String,
    pub title: String,
    pub phase: u32,
    #[serde(default)]
    pub prereqs: Vec<String>,
    #[serde(default)]
    pub grant: LicensingGrant,
    /// What counts as exercising this unit's skill (S4, #35): groups of
    /// `form:lemma@slot` / `construction:tag` atoms — every group must be
    /// satisfied, a group is satisfied by any of its atoms. When absent,
    /// the loader derives a single any-of group from the unit's own grant;
    /// units that grant nothing (interleaves) must author it.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target: Vec<Vec<String>>,
    /// One-liner pointing at the source video/notes material.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// One resolved target-evidence atom: a specific enumerated verb form or a
/// construction tag the item's analysis must contain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum TargetAtom {
    Form { lemma: String, form: String },
    Construction(String),
}

/// A unit's resolved target-skill spec, in conjunctive normal form over
/// [`TargetAtom`]s. The validator's target-skill-exercised check is
/// satisfaction of every group.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TargetSpec {
    pub groups: Vec<Vec<TargetAtom>>,
}

/// Day-0 licensed material, available in every unit from the first
/// exercise on: articles, gender basics, plurals, negation with "no",
/// core cognate patterns. An explicit curated artifact (user story 25).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmbientSet {
    pub version: u32,
    pub grant: LicensingGrant,
    /// References into the cognate notes registry.
    #[serde(default)]
    pub cognate_patterns: Vec<String>,
}

/// One of the ~45 curriculum-citizen verbs (user story 29). `class` names
/// the paradigm family the verb exemplifies (e.g. "irregular-core",
/// "regular-ar", "stem.e-ie", "spelling.-car").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerVerb {
    pub lemma: String,
    pub english: String,
    pub class: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerVerbRegistry {
    pub version: u32,
    pub verbs: Vec<PowerVerb>,
}

/// A cognate transformation pattern, kept as reference material — never a
/// drill unit or stacking tag (user story 26; the six v1 cognate units are
/// evicted from the sequence and live here instead).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognateNote {
    pub id: String,
    pub pattern: String,
    pub description: String,
    #[serde(default)]
    pub examples: Vec<CognateExample>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognateExample {
    pub en: String,
    pub es: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognateNotes {
    pub version: u32,
    pub notes: Vec<CognateNote>,
}

/// Top-level shape of the units file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitsFile {
    pub curriculum_version: u32,
    pub units: Vec<Unit>,
}

/// A unit's full computed licensing set: ambient ∪ ancestor grants ∪ own
/// grant. Stored in the v2 database, versioned, and inspectable via the
/// `dump_licensing` dev command.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EffectiveLicensing {
    pub unit_id: String,
    pub curriculum_version: u32,
    pub ambient_version: u32,
    pub verb_forms: BTreeSet<VerbFormGrant>,
    pub vocab_verb_forms: BTreeSet<VocabFormGrant>,
    pub constructions: BTreeSet<String>,
    pub vocab: BTreeSet<String>,
}
