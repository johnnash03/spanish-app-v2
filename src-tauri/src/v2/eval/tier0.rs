//! Tier 0 matcher (S6, #37): a learner answer against the item's
//! canonical + authored variants. A normalized match is correct instantly;
//! when leniency was needed, the remark names exactly which axes —
//! deterministically, never by model discretion (user story 14).

use super::normalize::{normalize, Leniency};

/// A successful Tier 0 match.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tier0Match {
    /// The accepted form (canonical or variant) the answer matched.
    pub matched: String,
    /// Deterministic informational remarks; empty when the answer matched
    /// without leniency.
    pub remarks: Vec<String>,
}

/// Matches `answer` against `canonical` and `variants`. Returns `None`
/// when no accepted form matches even under full leniency — such answers
/// route to Tier 1 (S7).
pub fn match_answer(answer: &str, canonical: &str, variants: &[String]) -> Option<Tier0Match> {
    const CLEAN: Leniency = Leniency {
        fold_case: false,
        strip_accents: false,
        drop_punctuation: false,
    };

    let answer_full = normalize(answer, Leniency::FULL);
    if answer_full.is_empty() {
        return None;
    }

    let candidates = std::iter::once(canonical).chain(variants.iter().map(String::as_str));

    // A clean match on any accepted form beats a lenient match on an
    // earlier one — no remark should fire when the learner typed a variant
    // perfectly.
    let answer_clean = normalize(answer, CLEAN);
    for candidate in candidates.clone() {
        if answer_clean == normalize(candidate, CLEAN) {
            return Some(Tier0Match {
                matched: candidate.to_string(),
                remarks: vec![],
            });
        }
    }

    for candidate in candidates {
        if answer_full == normalize(candidate, Leniency::FULL) {
            return Some(Tier0Match {
                matched: candidate.to_string(),
                remarks: vec![leniency_remark(answer, candidate)],
            });
        }
    }
    None
}

/// Names the leniency axes a matched answer leaned on, by re-running the
/// comparison with each axis disabled in turn: if equality breaks without
/// an axis, that axis was load-bearing.
fn leniency_remark(answer: &str, matched: &str) -> String {
    let differs_without = |leniency: Leniency| {
        normalize(answer, leniency) != normalize(matched, leniency)
    };
    let mut axes = vec![];
    if differs_without(Leniency {
        strip_accents: false,
        ..Leniency::FULL
    }) {
        axes.push("accents");
    }
    if differs_without(Leniency {
        fold_case: false,
        ..Leniency::FULL
    }) {
        axes.push("capitalization");
    }
    if differs_without(Leniency {
        drop_punctuation: false,
        ..Leniency::FULL
    }) {
        axes.push("punctuation");
    }
    let axes = match axes.as_slice() {
        [a] => (*a).to_string(),
        [a, b] => format!("{a} and {b}"),
        [a, b, c] => format!("{a}, {b} and {c}"),
        // Unreachable in practice: a lenient-only match always has at
        // least one load-bearing axis.
        _ => "orthography".to_string(),
    };
    format!("Correct — differs from “{matched}” only in {axes}.")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vars(texts: &[&str]) -> Vec<String> {
        texts.iter().map(|t| t.to_string()).collect()
    }

    #[test]
    fn match_table() {
        struct Case {
            name: &'static str,
            answer: &'static str,
            canonical: &'static str,
            variants: &'static [&'static str],
            want_matched: Option<&'static str>,
            want_remark_mentions: &'static [&'static str],
        }
        let cases = [
            Case {
                name: "exact canonical match, no remarks",
                answer: "Puedes verlos.",
                canonical: "Puedes verlos.",
                variants: &["Los puedes ver."],
                want_matched: Some("Puedes verlos."),
                want_remark_mentions: &[],
            },
            Case {
                name: "'Los puedes ver'-class clitic variant accepted via variant list",
                answer: "Los puedes ver.",
                canonical: "Puedes verlos.",
                variants: &["Los puedes ver."],
                want_matched: Some("Los puedes ver."),
                want_remark_mentions: &[],
            },
            Case {
                name: "whitespace-only difference is a clean match",
                answer: "  Puedes   verlos. ",
                canonical: "Puedes verlos.",
                variants: &[],
                want_matched: Some("Puedes verlos."),
                want_remark_mentions: &[],
            },
            Case {
                name: "accent-only slip matches with accent remark",
                answer: "Queria comer.",
                canonical: "Quería comer.",
                variants: &[],
                want_matched: Some("Quería comer."),
                want_remark_mentions: &["accents"],
            },
            Case {
                name: "capitalization-only slip",
                answer: "quiero comer.",
                canonical: "Quiero comer.",
                variants: &[],
                want_matched: Some("Quiero comer."),
                want_remark_mentions: &["capitalization"],
            },
            Case {
                name: "missing ¿? matches with punctuation remark",
                answer: "Puedes verlos",
                canonical: "¿Puedes verlos?",
                variants: &[],
                want_matched: Some("¿Puedes verlos?"),
                want_remark_mentions: &["punctuation"],
            },
            Case {
                name: "all three axes named together",
                answer: "queria comer",
                canonical: "Quería comer.",
                variants: &[],
                want_matched: Some("Quería comer."),
                want_remark_mentions: &["accents", "capitalization", "punctuation"],
            },
            Case {
                name: "lenient match against a variant remarks with the variant",
                answer: "los puedes ver",
                canonical: "Puedes verlos.",
                variants: &["Los puedes ver."],
                want_matched: Some("Los puedes ver."),
                want_remark_mentions: &["Los puedes ver.", "capitalization"],
            },
            Case {
                name: "ñ is not accent-lenient: anos ≠ años",
                answer: "Tiene dos anos.",
                canonical: "Tiene dos años.",
                variants: &[],
                want_matched: None,
                want_remark_mentions: &[],
            },
            Case {
                name: "different words do not match",
                answer: "Quiero dormir.",
                canonical: "Quiero comer.",
                variants: &[],
                want_matched: None,
                want_remark_mentions: &[],
            },
            Case {
                name: "empty answer never matches",
                answer: "   ",
                canonical: "Quiero comer.",
                variants: &[],
                want_matched: None,
                want_remark_mentions: &[],
            },
        ];

        for case in &cases {
            let got = match_answer(case.answer, case.canonical, &vars(case.variants));
            match case.want_matched {
                None => assert!(got.is_none(), "{}: expected no match, got {got:?}", case.name),
                Some(want) => {
                    let got = got.unwrap_or_else(|| panic!("{}: expected a match", case.name));
                    assert_eq!(got.matched, want, "{}", case.name);
                    if case.want_remark_mentions.is_empty() {
                        assert!(
                            got.remarks.is_empty(),
                            "{}: expected clean match, got remarks {:?}",
                            case.name,
                            got.remarks
                        );
                    } else {
                        assert_eq!(got.remarks.len(), 1, "{}", case.name);
                        for mention in case.want_remark_mentions {
                            assert!(
                                got.remarks[0].contains(mention),
                                "{}: remark {:?} missing {:?}",
                                case.name,
                                got.remarks[0],
                                mention
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn remark_does_not_name_axes_that_were_not_load_bearing() {
        // Accent slip only — the remark must not mention capitalization or
        // punctuation.
        let got = match_answer("Queria comer.", "Quería comer.", &[]).unwrap();
        assert!(!got.remarks[0].contains("capitalization"), "{:?}", got.remarks);
        assert!(!got.remarks[0].contains("punctuation"), "{:?}", got.remarks);
    }
}
