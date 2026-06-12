//! Validator data shapes (S4, #35): the analyzer's structured inventory
//! and the machine-readable violations the judge emits for the repair loop.

use serde::{Deserialize, Serialize};

/// One verb occurrence as the analyzer saw it: lemma + paradigm cell
/// (a form slot from [`crate::v2::curriculum::types::FORM_SLOTS`], or
/// `"other"` when the form fits no registered slot) + the surface string.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnalyzedVerbForm {
    pub lemma: String,
    pub form: String,
    pub surface: String,
}

/// The structured linguistic inventory of one candidate item's canonical
/// answer. Produced by the analyzer LLM call under a strict schema; the
/// judge treats it as the complete description of the sentence.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ItemAnalysis {
    pub verb_forms: Vec<AnalyzedVerbForm>,
    /// Construction tags from the curriculum's construction registry.
    pub constructions: Vec<String>,
    /// Lemmas of non-verb content words (nouns, adjectives, adverbs).
    /// Function words ride construction tags and are never listed.
    pub content_lemmas: Vec<String>,
}

/// A candidate exercise item as the generator proposes it: the English
/// cue and the canonical Spanish answer under judgment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandidateItem {
    pub source: String,
    pub canonical: String,
}

/// An already-banked item, for near-duplication checks.
#[derive(Debug, Clone)]
pub struct ExistingItem {
    pub id: String,
    pub canonical: String,
}

/// Sentence polarity a slot spec may demand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Polarity {
    Affirmative,
    Negative,
}

/// Sentence type a slot spec may demand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SentenceType {
    Declarative,
    Question,
}

/// The generator's per-item slot specification (PRD #31): the axes this
/// item was asked to vary on. The judge checks the produced item actually
/// conforms. All fields optional — `None` means the axis was unspecified.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SlotSpec {
    /// Grammatical person of the target verb form ("1sg", "3pl", …).
    pub person: Option<String>,
    pub polarity: Option<Polarity>,
    pub sentence_type: Option<SentenceType>,
}

/// One named, machine-readable rule violation. The serialized form is the
/// contract with the repair loop: regeneration prompts quote these fields
/// verbatim, so every variant names the offending element precisely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Violation {
    /// A verb form outside the unit's licensing set.
    UnlicensedVerbForm {
        lemma: String,
        form: String,
        surface: String,
    },
    /// A registered construction the unit does not license.
    UnlicensedConstruction { construction: String },
    /// A content word outside the licensed vocabulary and the active
    /// window.
    UnlicensedVocab { lemma: String },
    /// A construction tag no unit or ambient grant registers — an analyzer
    /// vocabulary error, rejected fail-safe.
    UnknownConstructionTag { construction: String },
    /// The canonical answer does not exercise the unit's target skill
    /// (user story 52). `unmet` lists one unsatisfied target group in
    /// `form:`/`construction:` atom syntax.
    TargetSkillNotExercised {
        target_skill: String,
        unmet: Vec<String>,
    },
    /// Too similar to an item already in the bank (user story 50).
    NearDuplicate { of_item_id: String },
    /// The item does not conform to its slot spec (`slot` names the axis).
    SlotMismatch {
        slot: String,
        expected: String,
        found: String,
    },
    /// A verb form slot outside the registry (including the analyzer's
    /// `"other"` escape hatch) — rejected fail-safe.
    UnrecognizedFormSlot {
        lemma: String,
        form: String,
        surface: String,
    },
    /// The analyzer call itself failed (transport, schema, parse). Always
    /// a rejection — mis-analysis must never silently pass.
    AnalysisFailed { reason: String },
}
