//! V2 generator: the generate → validate → repair pipeline (S5, #36).
//!
//! Generation is a positive spec (PRD #31): the prompt enumerates exactly
//! the licensed forms, constructions, and words — never prose like
//! "present tense only" — plus per-item slot specs. Every produced item is
//! gated by the S4 validator before it can enter the bank; failures are
//! regenerated with the specific violation named, for a bounded number of
//! repair rounds. Items stream into the bank as they pass (v1's streaming
//! persistence carried over), and the adjacent unit is prefetched.

pub mod bank;
pub mod commands;
pub mod extract;
pub mod pipeline;
pub mod plan;
pub mod prompt;
mod s5_acceptance;
pub mod source;
pub mod types;

pub use pipeline::{generate_unit_bank, BankSink, GenerationOutcome, ItemSource, PipelineConfig};
pub use plan::{LearnerSnapshot, UnknownAxis};
pub use types::{BankItem, GeneratedItem, SlotFailure, ValidatedVariant};
