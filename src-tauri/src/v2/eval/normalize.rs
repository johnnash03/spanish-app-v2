//! Tier 0 answer normalization (S6, #37). The leniency policy of user
//! story 14 — accents, capitalization, and ¿¡/punctuation never block a
//! match — is enforced here, in code, and is not model-discretionary.

/// Which leniency axes to apply. Tier 0 matching always applies all of
/// them; the matcher also runs partial combinations to name, in its
/// deterministic remark, exactly which axes a learner's answer leaned on.
#[derive(Debug, Clone, Copy)]
pub struct Leniency {
    pub fold_case: bool,
    pub strip_accents: bool,
    pub drop_punctuation: bool,
}

impl Leniency {
    pub const FULL: Leniency = Leniency {
        fold_case: true,
        strip_accents: true,
        drop_punctuation: true,
    };
}

/// Normalizes an answer under the given leniency axes. Whitespace is
/// always trimmed and collapsed — spacing differences are never
/// meaningful. `ñ` is a distinct letter, not an accented `n`, and is
/// deliberately preserved by accent stripping.
pub fn normalize(text: &str, leniency: Leniency) -> String {
    let mut out = String::with_capacity(text.len());
    let mut pending_space = false;
    for ch in text.chars() {
        let ch = if leniency.strip_accents {
            strip_accent(ch)
        } else {
            ch
        };
        if ch.is_whitespace() {
            pending_space = !out.is_empty();
            continue;
        }
        if leniency.drop_punctuation && !ch.is_alphanumeric() {
            continue;
        }
        if pending_space {
            out.push(' ');
            pending_space = false;
        }
        if leniency.fold_case {
            out.extend(ch.to_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

/// Removes the acute accent / diaeresis from Spanish vowels (both cases).
/// `ñ`/`Ñ` pass through untouched.
fn strip_accent(ch: char) -> char {
    match ch {
        'á' => 'a',
        'é' => 'e',
        'í' => 'i',
        'ó' => 'o',
        'ú' | 'ü' => 'u',
        'Á' => 'A',
        'É' => 'E',
        'Í' => 'I',
        'Ó' => 'O',
        'Ú' | 'Ü' => 'U',
        _ => ch,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_normalization_table() {
        let cases: &[(&str, &str)] = &[
            // identity on already-plain text
            ("quiero comer", "quiero comer"),
            // case folding
            ("Quiero Comer", "quiero comer"),
            // accent stripping, both cases
            ("Quería comer", "queria comer"),
            ("ÉL QUERÍA", "el queria"),
            ("pingüino", "pinguino"),
            // ñ is a letter, never stripped
            ("mañana", "mañana"),
            ("AÑO", "año"),
            // inverted and terminal punctuation dropped
            ("¿Puedes verlos?", "puedes verlos"),
            ("¡No lo quiero!", "no lo quiero"),
            ("Quiero comer.", "quiero comer"),
            // commas, quotes, dashes dropped
            ("No, no quiero.", "no no quiero"),
            ("\u{201c}Lo s\u{e9}\u{201d} \u{2014} dijo", "lo se dijo"),
            // whitespace trimmed and collapsed
            ("  quiero   comer  ", "quiero comer"),
            ("quiero\tcomer\n", "quiero comer"),
            // punctuation-only gap still separates words
            ("quiero, comer", "quiero comer"),
            // empty and punctuation-only inputs
            ("", ""),
            ("¿?¡!.", ""),
        ];
        for (input, want) in cases {
            assert_eq!(
                normalize(input, Leniency::FULL),
                *want,
                "normalize({input:?})"
            );
        }
    }

    #[test]
    fn partial_leniency_axes_apply_independently() {
        let no_accent_strip = Leniency {
            strip_accents: false,
            ..Leniency::FULL
        };
        assert_eq!(
            normalize("¿Quería comer?", no_accent_strip),
            "quería comer"
        );

        let no_case_fold = Leniency {
            fold_case: false,
            ..Leniency::FULL
        };
        assert_eq!(normalize("¿Quería comer?", no_case_fold), "Queria comer");

        let no_punct_drop = Leniency {
            drop_punctuation: false,
            ..Leniency::FULL
        };
        assert_eq!(
            normalize("¿Quería comer?", no_punct_drop),
            "¿queria comer?"
        );
    }
}
