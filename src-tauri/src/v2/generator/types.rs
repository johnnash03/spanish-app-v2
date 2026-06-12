//! Generator data shapes (S5, #36): what the model returns, what the
//! repair loop tracks, and what the bank persists.

use super::plan::{ItemPlan, ItemTags};
use crate::v2::validator::{ItemAnalysis, SlotSpec, Violation};
use serde::{Deserialize, Serialize};

/// One item as the generation model proposes it. `slot_id` ties it back to
/// the [`ItemPlan`] it was requested against.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneratedItem {
    pub slot_id: u32,
    pub source: String,
    pub canonical: String,
    #[serde(default)]
    pub variants: Vec<String>,
}

/// A variant that survived validation, kept with its analysis for
/// inspection (deterministic Tier-0 matching consumes `text`; S6).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidatedVariant {
    pub text: String,
    pub analysis: ItemAnalysis,
}

/// A fully validated bank item: canonical + validated variants + slot spec
/// + tags + the canonical's analysis (issue #36 acceptance shape).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankItem {
    pub id: String,
    pub unit_id: String,
    pub source: String,
    pub canonical: String,
    pub variants: Vec<ValidatedVariant>,
    pub slot: SlotSpec,
    pub tags: ItemTags,
    pub analysis: ItemAnalysis,
}

/// A slot that did not bank this round: carried into the next repair round
/// with its violations named, or abandoned when rounds run out.
#[derive(Debug, Clone)]
pub struct SlotFailure {
    pub plan: ItemPlan,
    /// The rejected attempt — `None` when the model produced nothing for
    /// the slot.
    pub attempt: Option<GeneratedItem>,
    pub violations: Vec<Violation>,
}
