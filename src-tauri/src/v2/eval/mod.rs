//! Tier 0 deterministic evaluation (S6, #37): normalization and matching
//! against canonical + authored variants. Instant, offline, and
//! code-enforced — the most common case never depends on AI judgment.
//! Answers Tier 0 cannot match stay pending for the Tier 1 evaluator
//! (S7, #38).

pub mod normalize;
pub mod tier0;

pub use normalize::{normalize, Leniency};
pub use tier0::{match_answer, Tier0Match};
