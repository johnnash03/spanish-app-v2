//! Category → curriculum-skill attribution (S7, #38; user story 17).
//!
//! The model only ever names an [`ErrorCategory`]; this code decides which
//! curriculum skill tags the error counts against, choosing among the
//! attempt's own tags (target + stacked) by category family. V1 let the
//! model attribute errors directly and it routinely dinged the wrong tag
//! ("polluting mastery data", PRD #31); here a clitic error on a
//! question-unit item lands on the stacked clitic skill, never the
//! question skill. Every attributed tag is validated against the
//! curriculum unit registry at write time — a tag outside the registry is
//! an error, never a write.

use super::error_enum::ErrorCategory;
use crate::v2::curriculum::Curriculum;

/// The curriculum tag families a category implicates. Families are unit-id
/// prefixes; an empty slice means the category carries no structural
/// signal of its own (lexical/orthographic slips) and the error stays on
/// the attempt's target skill.
fn category_families(category: ErrorCategory) -> &'static [&'static str] {
    use ErrorCategory::*;
    match category {
        // Finite-verb production is what the opener units drill.
        VerbForm | TenseSelection | MoodSelection => &["opener."],
        CliticPlacement | CliticChoice => &["clitic."],
        // Constituent-order errors beyond clitic placement are the
        // question units' territory (inversion, wh-fronting).
        WordOrder => &["question."],
        LexicalChoice | AgreementGender | AgreementNumber | Omission | Addition
        | Orthography => &[],
    }
}

/// The skill tags a classified error counts against: the attempt's tags
/// (target first, then stacked) filtered to the category's families, or
/// the target skill alone when no tag matches — an error never attributes
/// to nothing, and never to a skill the item did not exercise.
pub fn attributed_skills(
    category: ErrorCategory,
    target_skill: &str,
    stacked: &[String],
) -> Vec<String> {
    let families = category_families(category);
    let candidates =
        std::iter::once(target_skill).chain(stacked.iter().map(String::as_str));
    let mut matched: Vec<String> = candidates
        .filter(|tag| families.iter().any(|f| tag.starts_with(f)))
        .map(String::from)
        .collect();
    matched.dedup();
    if matched.is_empty() {
        vec![target_skill.to_string()]
    } else {
        matched
    }
}

/// Write-time registry validation: every attributed tag must name a
/// curriculum unit. A failure here means an attempt carried a tag the
/// curriculum does not know — the write must not happen (v1's hallucinated
/// `error_tag`s are unrepresentable end to end).
pub fn validate_skills(tags: &[String], curriculum: &Curriculum) -> Result<(), String> {
    for tag in tags {
        if curriculum.unit(tag).is_none() {
            return Err(format!("skill tag `{tag}` is not in the curriculum registry"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2::curriculum;
    use crate::v2::eval::error_enum::ALL_CATEGORIES;

    fn s(tags: &[&str]) -> Vec<String> {
        tags.iter().map(|t| t.to_string()).collect()
    }

    #[test]
    fn clitic_error_on_a_question_item_attributes_to_the_stacked_clitic_skill() {
        // The v1 misattribution fix: "Cuando va a llamarlo" drilled
        // question.wh stacked over a clitic skill; a clitic error must
        // land on the clitic skill, not the question skill.
        let got = attributed_skills(
            ErrorCategory::CliticPlacement,
            "question.wh",
            &s(&["clitic.do.sg.attach", "opener.voy-a"]),
        );
        assert_eq!(got, vec!["clitic.do.sg.attach".to_string()]);
    }

    #[test]
    fn verb_form_error_attributes_to_the_opener_skill_among_the_items_tags() {
        let got = attributed_skills(
            ErrorCategory::VerbForm,
            "clitic.do.attach.mixed",
            &s(&["opener.quiero"]),
        );
        assert_eq!(got, vec!["opener.quiero".to_string()]);
    }

    #[test]
    fn structureless_categories_stay_on_the_target_skill() {
        for category in [
            ErrorCategory::LexicalChoice,
            ErrorCategory::AgreementGender,
            ErrorCategory::Orthography,
        ] {
            let got = attributed_skills(category, "opener.quiero", &s(&["question.wh"]));
            assert_eq!(got, vec!["opener.quiero".to_string()], "{category:?}");
        }
    }

    #[test]
    fn family_category_with_no_matching_tag_falls_back_to_the_target() {
        // A clitic-classified error on an item that exercises no clitic
        // skill: the target still takes it; nothing is invented.
        let got = attributed_skills(ErrorCategory::CliticChoice, "opener.puedo", &[]);
        assert_eq!(got, vec!["opener.puedo".to_string()]);
    }

    #[test]
    fn target_skill_in_the_family_attributes_to_itself() {
        let got = attributed_skills(
            ErrorCategory::CliticPlacement,
            "clitic.both.se-lo",
            &s(&["opener.quiero"]),
        );
        assert_eq!(got, vec!["clitic.both.se-lo".to_string()]);
    }

    #[test]
    fn every_family_prefix_names_real_curriculum_units() {
        // A family that matches no unit would silently disable its
        // categories' attribution — the registry must cover every family.
        let c = curriculum::load_embedded().unwrap();
        for category in ALL_CATEGORIES {
            for family in category_families(category) {
                assert!(
                    c.units.iter().any(|u| u.id.starts_with(family)),
                    "family `{family}` ({category:?}) matches no curriculum unit"
                );
            }
        }
    }

    #[test]
    fn attribution_from_real_item_tags_always_passes_registry_validation() {
        // Write-time validation is the safety net; attribution from
        // curriculum-known tags must always clear it.
        let c = curriculum::load_embedded().unwrap();
        for category in ALL_CATEGORIES {
            let got = attributed_skills(
                category,
                "question.wh",
                &s(&["clitic.do.sg.attach", "opener.voy-a"]),
            );
            validate_skills(&got, &c).unwrap();
        }
    }

    #[test]
    fn registry_validation_rejects_unknown_tags() {
        // V1's hallucinated tags ("gram.personal-a") must be unwritable.
        let c = curriculum::load_embedded().unwrap();
        let err = validate_skills(&s(&["gram.personal-a"]), &c).unwrap_err();
        assert!(err.contains("gram.personal-a"));
    }
}
