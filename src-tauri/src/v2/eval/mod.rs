//! V2 evaluation. Tier 0 (S6, #37): deterministic normalization and
//! matching against canonical + authored variants — instant, offline, and
//! code-enforced, so the most common case never depends on AI judgment.
//! Tier 1 (S7, #38): decomposed LLM judgment for answers Tier 0 cannot
//! match — the model judges, code decides.

pub mod error_enum;
pub mod normalize;
pub mod skill_map;
pub mod tier0;
pub mod tier1;
mod v1_regression;

pub use error_enum::{ErrorCategory, ALL_CATEGORIES};
pub use skill_map::{attributed_skills, validate_skills};
pub use normalize::{normalize, Leniency};
pub use tier0::{match_answer, Tier0Match};
pub use tier1::{
    resolve, target_description, EvalInput, Evaluator, OpenAiEvaluator, Tier1Analysis,
    Tier1Error, Tier1Outcome,
};
