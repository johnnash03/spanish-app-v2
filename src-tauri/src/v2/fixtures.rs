//! V1 data archived as test fixtures (PRD #31, Foundation).
//!
//! V1 attempt history is deliberately not migrated into the v2 database —
//! its error attribution is polluted and the curriculum is re-authored.
//! Instead, the full v1 yield is committed under `src-tauri/fixtures/`:
//!
//! - `v1_exercise_items.json` — every generated v1 item; input corpus for
//!   the S4 validator (#35).
//! - `v1_evaluations.json` — every v1 evaluation verdict, including the
//!   known unjust ones; regression seed for the S7 evaluator (#38).
//! - `v1_combined_exercises.json` — the v1 combined-track pool, kept for
//!   the same validator corpus.

#[cfg(test)]
pub const V1_EXERCISE_ITEMS: &str = include_str!("../../fixtures/v1_exercise_items.json");
#[cfg(test)]
pub const V1_EVALUATIONS: &str = include_str!("../../fixtures/v1_evaluations.json");
#[cfg(test)]
pub const V1_COMBINED_EXERCISES: &str =
    include_str!("../../fixtures/v1_combined_exercises.json");

// Fields mirror the archived v1 row shape in full, whether or not current
// tests read them — S4/S7 suites will.
#[cfg(test)]
#[derive(serde::Deserialize)]
#[allow(dead_code)]
pub struct V1ExerciseItem {
    pub id: String,
    pub source: String,
    pub canonical: String,
    pub primary_tag: String,
    /// JSON-encoded array of tags, exactly as stored in the v1 DB.
    pub stacked_tags: String,
    pub created_at: i64,
    pub category: Option<String>,
}

#[cfg(test)]
#[derive(serde::Deserialize)]
#[allow(dead_code)]
pub struct V1Evaluation {
    pub id: String,
    pub tag: String,
    pub item_id: String,
    pub correct: i64,
    pub learner_answer: String,
    pub timestamp: i64,
    pub session_id: Option<String>,
    pub eval_state: String,
    pub error_tag: Option<String>,
    /// JSON-encoded array of remark strings, exactly as stored in the v1 DB.
    pub remarks: String,
    pub explanation: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v1_exercise_items_fixture_parses_with_expected_count() {
        let items: Vec<V1ExerciseItem> = serde_json::from_str(V1_EXERCISE_ITEMS).unwrap();
        assert_eq!(items.len(), 4055, "all 4,055 v1 items must be archived");
        for item in &items {
            assert!(!item.id.is_empty());
            assert!(!item.canonical.is_empty());
            let tags: Result<Vec<String>, _> = serde_json::from_str(&item.stacked_tags);
            assert!(tags.is_ok(), "item {} has invalid stacked_tags", item.id);
        }
    }

    #[test]
    fn v1_evaluations_fixture_parses_with_expected_count() {
        let evals: Vec<V1Evaluation> = serde_json::from_str(V1_EVALUATIONS).unwrap();
        assert_eq!(evals.len(), 131, "all 131 v1 evaluations must be archived");
        for e in &evals {
            assert_eq!(e.eval_state, "evaluated");
            let remarks: Result<Vec<String>, _> = serde_json::from_str(&e.remarks);
            assert!(remarks.is_ok(), "evaluation {} has invalid remarks", e.id);
        }
    }

    #[test]
    fn v1_evaluations_include_wrong_verdicts() {
        // The regression seed is only useful if it contains the verdicts that
        // were wrong-marked — including the known unjust ones.
        let evals: Vec<V1Evaluation> = serde_json::from_str(V1_EVALUATIONS).unwrap();
        let wrong = evals.iter().filter(|e| e.correct == 0).count();
        assert!(wrong > 0, "fixture must contain wrong verdicts");
    }

    #[test]
    fn v1_combined_exercises_fixture_parses() {
        let items: Vec<serde_json::Value> =
            serde_json::from_str(V1_COMBINED_EXERCISES).unwrap();
        assert_eq!(items.len(), 30, "all 30 v1 combined exercises must be archived");
    }
}
