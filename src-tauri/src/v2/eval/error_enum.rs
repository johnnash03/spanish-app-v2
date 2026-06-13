//! The closed error enum (S7, #38; user story 17). Every wrong answer is
//! classified into exactly these categories — the Tier 1 schema locks the
//! model's output to them, so a hallucinated tag is unrepresentable. V1's
//! free-text `error_tag` invented tags that existed nowhere in the
//! registry; this enum is the structural fix.

use serde::{Deserialize, Serialize};

/// One of the twelve closed error categories. The PRD names eleven
/// (verb-form … orthography); `addition` is the twelfth — the v1 log shows
/// superfluous-word errors ("Cuantos muchos libros") that `omission`
/// cannot carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ErrorCategory {
    /// Wrong conjugated form of the right verb: person, number, or an
    /// invented form ("quieromos").
    VerbForm,
    /// A clitic in an unlicensed position (not a valid alternative
    /// placement — those are correct).
    CliticPlacement,
    /// The wrong clitic pronoun (lo for la, nos for les, laísmo/loísmo).
    CliticChoice,
    /// Gender agreement failure (article/adjective vs noun).
    AgreementGender,
    /// Number agreement failure.
    AgreementNumber,
    /// The wrong word: a verb or content word that changes the meaning
    /// (poder for tener que, saber for conocer, wrong preposition).
    LexicalChoice,
    /// Indicative/subjunctive (or other mood) selection error.
    MoodSelection,
    /// The wrong tense for the cue's time reference.
    TenseSelection,
    /// Constituents in an order Spanish does not allow.
    WordOrder,
    /// A required element is missing (que after tener, personal a, a
    /// required clitic).
    Omission,
    /// A superfluous element that does not belong in the sentence.
    Addition,
    /// A spelling error beyond the deterministic leniency axes (accents,
    /// case, ¿¡ punctuation never reach this category — Tier 0 code
    /// forgives them).
    Orthography,
}

/// Every category, in declaration order. The Tier 1 JSON schema enumerates
/// exactly these wire names.
pub const ALL_CATEGORIES: [ErrorCategory; 12] = [
    ErrorCategory::VerbForm,
    ErrorCategory::CliticPlacement,
    ErrorCategory::CliticChoice,
    ErrorCategory::AgreementGender,
    ErrorCategory::AgreementNumber,
    ErrorCategory::LexicalChoice,
    ErrorCategory::MoodSelection,
    ErrorCategory::TenseSelection,
    ErrorCategory::WordOrder,
    ErrorCategory::Omission,
    ErrorCategory::Addition,
    ErrorCategory::Orthography,
];

impl ErrorCategory {
    /// The serialized wire name ("verb-form"), as the schema and the
    /// attempt log store it.
    pub fn wire_name(&self) -> String {
        serde_json::to_value(self)
            .expect("category serializes")
            .as_str()
            .expect("category is a string")
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_enum_is_closed_at_twelve_kebab_case_categories() {
        assert_eq!(ALL_CATEGORIES.len(), 12);
        let names: Vec<String> = ALL_CATEGORIES.iter().map(|c| c.wire_name()).collect();
        for expected in [
            "verb-form",
            "clitic-placement",
            "clitic-choice",
            "agreement-gender",
            "agreement-number",
            "lexical-choice",
            "mood-selection",
            "tense-selection",
            "word-order",
            "omission",
            "addition",
            "orthography",
        ] {
            assert!(names.contains(&expected.to_string()), "missing {expected}");
        }
    }

    #[test]
    fn categories_round_trip_through_serde() {
        for c in ALL_CATEGORIES {
            let json = serde_json::to_string(&c).unwrap();
            let back: ErrorCategory = serde_json::from_str(&json).unwrap();
            assert_eq!(back, c);
        }
    }

    #[test]
    fn unknown_categories_are_unrepresentable() {
        // V1's hallucinated tags must fail to parse, never coerce.
        for bogus in ["\"gram.personal-a\"", "\"verb_form\"", "\"spelling\""] {
            assert!(serde_json::from_str::<ErrorCategory>(bogus).is_err());
        }
    }
}
