//! Tier 1 decomposed evaluation (S7, #38): the LLM judges, code decides.
//!
//! For an answer Tier 0 cannot match, one evaluator call produces three
//! independent judgments with evidence — (a) grammatical Spanish? (b)
//! conveys the cue's meaning? (c) uses the target structure? — plus, when
//! the answer is wrong, a classification into the closed error enum with
//! an evidence span, a hint, and a pedagogical explanation. Deterministic
//! code then resolves the verdict: correct = a ∧ b; correct-but-¬c is the
//! structure dodge (nudge, zero credit, re-serve); anything else is wrong.
//! The model never sees the canonical answer — v1's evaluator anchored on
//! it and rejected real Spanish ("Los puedes ver"); here grammar and
//! meaning are judged on the answer's own terms (user stories 13, 15–17).

use super::error_enum::{ErrorCategory, ALL_CATEGORIES};
use super::skill_map::{attributed_skills, validate_skills};
use crate::v2::curriculum::types::TargetAtom;
use crate::v2::curriculum::Curriculum;
use crate::v2::validator::analyzer::CONSTRUCTION_GLOSSES;
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs, ResponseFormat, ResponseFormatJsonSchema,
    },
    Client,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Capability-tier default (PRD #31: frontier tier for evaluation). The
/// model identifier is configuration: override with `V2_EVALUATOR_MODEL`.
const DEFAULT_EVALUATOR_MODEL: &str = "gpt-4o-2024-08-06";

pub fn evaluator_model() -> String {
    std::env::var("V2_EVALUATOR_MODEL").unwrap_or_else(|_| DEFAULT_EVALUATOR_MODEL.to_string())
}

/// One decomposed judgment: a boolean verdict and the evidence it rests
/// on, in the model's words.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Judgment {
    pub verdict: bool,
    pub evidence: String,
}

/// The classified error of a wrong answer: one closed-enum category and
/// the span of the learner's answer that evidences it. The schema locks
/// `category` to the twelve wire names — a hallucinated tag cannot parse.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorFinding {
    pub category: ErrorCategory,
    pub evidence: String,
}

/// The evaluator call's full structured output. Judgments are always
/// present; `error`, `hint`, and `explanation` are null unless the answer
/// is wrong.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tier1Analysis {
    /// The model's first field (alphabetically first in the schema, so it
    /// is generated before any judgment): the answer with only accents,
    /// casing, and ¿¡/punctuation corrected. Forcing the restoration to be
    /// written down before judging is what makes the leniency rules stick
    /// — without it the model kept reading "envias" as a conjugation
    /// error.
    pub accent_restored_answer: String,
    pub grammatical: Judgment,
    pub conveys_meaning: Judgment,
    pub uses_target_structure: Judgment,
    pub error: Option<ErrorFinding>,
    pub hint: Option<String>,
    pub explanation: Option<String>,
}

/// The code-decided resolution of a Tier 1 analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tier1Outcome {
    /// Grammatical, conveys the meaning, exercises the target.
    Correct,
    /// Good Spanish that avoids the target structure: shown as correct
    /// with the nudge, worth zero mastery credit, and the skill re-serves
    /// (user stories 15, 16).
    Dodge { nudge: String },
    /// Wrong, with the closed-enum classification and the curriculum
    /// skills the error attributes to (registry-validated).
    Wrong {
        category: ErrorCategory,
        evidence: String,
        hint: Option<String>,
        explanation: Option<String>,
        skills: Vec<String>,
    },
}

#[derive(Debug, Error)]
pub enum Tier1Error {
    #[error("evaluator transport error: {0}")]
    Transport(String),
    #[error("evaluator returned no content")]
    EmptyResponse,
    #[error("evaluator output failed schema parse: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("wrong answer carries no error classification")]
    MissingError,
    #[error("skill attribution failed registry validation: {0}")]
    Registry(String),
}

/// Resolves an analysis into a verdict — pure code, no model discretion.
/// A wrong answer without an error finding, or an attribution outside the
/// curriculum registry, is an error: the attempt stays pending rather than
/// writing a verdict the data model cannot trust.
pub fn resolve(
    analysis: &Tier1Analysis,
    target_skill: &str,
    target_title: &str,
    stacked: &[String],
    curriculum: &Curriculum,
) -> Result<Tier1Outcome, Tier1Error> {
    let correct = analysis.grammatical.verdict && analysis.conveys_meaning.verdict;
    if correct {
        if analysis.uses_target_structure.verdict {
            return Ok(Tier1Outcome::Correct);
        }
        return Ok(Tier1Outcome::Dodge {
            nudge: format!(
                "Correct Spanish — but this one drills “{target_title}”. Try a version that uses it."
            ),
        });
    }
    let finding = analysis.error.as_ref().ok_or(Tier1Error::MissingError)?;
    let skills = attributed_skills(finding.category, target_skill, stacked);
    validate_skills(&skills, curriculum).map_err(Tier1Error::Registry)?;
    Ok(Tier1Outcome::Wrong {
        category: finding.category,
        evidence: finding.evidence.clone(),
        hint: analysis.hint.clone(),
        explanation: analysis.explanation.clone(),
        skills,
    })
}

/// What the evaluator is asked to judge. Deliberately canonical-free.
#[derive(Debug, Clone)]
pub struct EvalInput {
    /// The English cue the learner translated.
    pub cue: String,
    /// The learner's answer, verbatim.
    pub answer: String,
    /// Human-readable description of the item's target structure.
    pub target_description: String,
}

/// One evaluator call per pending attempt. Implementations judge the
/// decomposed questions only; they never compute the verdict. Errors
/// always leave the attempt pending (fail-safe), never resolve it.
pub trait Evaluator {
    fn evaluate(
        &self,
        input: &EvalInput,
    ) -> impl std::future::Future<Output = Result<Tier1Analysis, Tier1Error>> + Send;
}

/// Renders a unit's target structure for the evaluator prompt: the unit
/// title plus its target atoms, glossed where the construction registry
/// has a gloss.
pub fn target_description(curriculum: &Curriculum, unit_id: &str) -> Option<String> {
    let unit = curriculum.unit(unit_id)?;
    let spec = curriculum.target_spec(unit_id)?;
    let mut parts: Vec<String> = vec![];
    for group in &spec.groups {
        let rendered: Vec<String> = group
            .iter()
            .map(|atom| match atom {
                TargetAtom::Form { lemma, form } => format!("the form {lemma}@{form}"),
                TargetAtom::Construction(tag) => CONSTRUCTION_GLOSSES
                    .iter()
                    .find(|(t, _)| t == tag)
                    .map(|(_, gloss)| (*gloss).to_string())
                    .unwrap_or_else(|| format!("the construction `{tag}`")),
            })
            .collect();
        parts.push(rendered.join(" or "));
    }
    Some(format!("{} — {}", unit.title, parts.join("; and ")))
}

pub struct OpenAiEvaluator {
    client: Client<OpenAIConfig>,
    model: String,
}

impl OpenAiEvaluator {
    /// Reads `OPENAI_API_KEY` (and optionally `V2_EVALUATOR_MODEL`) from
    /// the environment.
    pub fn from_env() -> Result<Self, Tier1Error> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| Tier1Error::Transport("OPENAI_API_KEY not set".into()))?;
        Ok(Self {
            client: Client::with_config(OpenAIConfig::new().with_api_key(api_key)),
            model: evaluator_model(),
        })
    }
}

impl Evaluator for OpenAiEvaluator {
    async fn evaluate(&self, input: &EvalInput) -> Result<Tier1Analysis, Tier1Error> {
        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.model)
            // Judgment must be reproducible: the same answer gets the same
            // verdict, today and in the regression suite.
            .temperature(0.0)
            .response_format(ResponseFormat::JsonSchema {
                json_schema: ResponseFormatJsonSchema {
                    name: "tier1_analysis".into(),
                    description: Some(
                        "Decomposed evaluation of a learner's Spanish answer".into(),
                    ),
                    schema: Some(tier1_schema()),
                    strict: Some(true),
                },
            })
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system_prompt())
                    .build()
                    .map_err(|e| Tier1Error::Transport(e.to_string()))?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(user_prompt(input))
                    .build()
                    .map_err(|e| Tier1Error::Transport(e.to_string()))?
                    .into(),
            ])
            .build()
            .map_err(|e| Tier1Error::Transport(e.to_string()))?;

        let response = self
            .client
            .chat()
            .create(request)
            .await
            .map_err(|e| Tier1Error::Transport(e.to_string()))?;

        let content = response
            .choices
            .first()
            .and_then(|c| c.message.content.as_deref())
            .ok_or(Tier1Error::EmptyResponse)?;
        parse_tier1(content)
    }
}

/// Parses evaluator output. Schema enforcement happens server-side; this
/// is the client-side backstop, and any failure leaves the attempt
/// pending.
pub fn parse_tier1(content: &str) -> Result<Tier1Analysis, Tier1Error> {
    Ok(serde_json::from_str(content)?)
}

/// Strict JSON schema for [`Tier1Analysis`]. `error.category` enumerates
/// exactly the twelve closed wire names — the model cannot output a
/// category outside the enum.
pub fn tier1_schema() -> serde_json::Value {
    let categories: Vec<String> = ALL_CATEGORIES.iter().map(|c| c.wire_name()).collect();
    let judgment = serde_json::json!({
        "type": "object",
        "properties": {
            "verdict": { "type": "boolean" },
            "evidence": { "type": "string" }
        },
        "required": ["verdict", "evidence"],
        "additionalProperties": false
    });
    serde_json::json!({
        "type": "object",
        "properties": {
            "accent_restored_answer": { "type": "string" },
            "grammatical": judgment,
            "conveys_meaning": judgment,
            "uses_target_structure": judgment,
            "error": {
                "type": ["object", "null"],
                "properties": {
                    "category": { "type": "string", "enum": categories },
                    "evidence": { "type": "string" }
                },
                "required": ["category", "evidence"],
                "additionalProperties": false
            },
            "hint": { "type": ["string", "null"] },
            "explanation": { "type": ["string", "null"] }
        },
        "required": [
            "accent_restored_answer",
            "grammatical",
            "conveys_meaning",
            "uses_target_structure",
            "error",
            "hint",
            "explanation"
        ],
        "additionalProperties": false
    })
}

/// One short definition per closed category, embedded in the prompt so the
/// model classifies in the enum's intended senses.
fn category_glosses() -> String {
    ALL_CATEGORIES
        .iter()
        .map(|c| {
            let gloss = match c {
                ErrorCategory::VerbForm => {
                    "wrong conjugated form of the right verb (person, number, or an invented form like “quieromos”)"
                }
                ErrorCategory::CliticPlacement => {
                    "an object pronoun in a position Spanish does not license (valid alternative placements are NOT errors)"
                }
                ErrorCategory::CliticChoice => {
                    "the wrong object pronoun (lo for la, la for le, nos for les)"
                }
                ErrorCategory::AgreementGender => {
                    "gender agreement failure between article/adjective and noun"
                }
                ErrorCategory::AgreementNumber => "number agreement failure",
                ErrorCategory::LexicalChoice => {
                    "the wrong word: a verb, content word, or preposition that changes the meaning"
                }
                ErrorCategory::MoodSelection => "indicative/subjunctive (or other mood) selection error",
                ErrorCategory::TenseSelection => "the wrong tense for the cue's time reference",
                ErrorCategory::WordOrder => "constituents in an order Spanish does not allow",
                ErrorCategory::Omission => {
                    "a required element is missing (que after tener, personal a, a required pronoun)"
                }
                ErrorCategory::Addition => "a superfluous element that does not belong",
                ErrorCategory::Orthography => {
                    "a real misspelling — beyond accents, casing, and punctuation, which are never errors"
                }
            };
            format!("- `{}` — {gloss}", c.wire_name())
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn system_prompt() -> String {
    format!(
        "You evaluate one learner answer in a Spanish translation drill. First restore, \
         then make three INDEPENDENT judgments, each with a short evidence note. You are \
         never shown an expected answer, and you never decide the overall verdict — code \
         does that.\n\
         \n\
         0. `accent_restored_answer`: rewrite the learner's answer with only accents \
         (á é í ó ú), casing, and ¿¡/end punctuation corrected — change no letters and \
         no words. Never restore ñ: n where ñ belongs (“manana”) is a real misspelling, \
         must stay as written, and makes the answer ungrammatical. Every judgment below \
         is about THIS restored string, never the raw answer.\n\
         1. `grammatical`: is the answer well-formed Spanish on its own terms? Judge the \
         sentence the learner wrote, not the sentence you would have written. Leniency \
         rules, enforced absolutely: missing or wrong accents and diacritics on ANY word \
         (como for cómo, estas for estás, que for qué), casing, and missing ¿¡ or end \
         punctuation are NEVER errors — deterministic code already forgives them; mentally \
         restore them before judging. The test: if the answer becomes well-formed Spanish \
         by only correcting accents, casing, and punctuation, `grammatical` is true and \
         there is no error. Examples: “Como estas” is “¿Cómo estás?” → true; “Por que lo \
         quiere” is “¿Por qué lo quiere?” → true; “Entiendo por que hablas” is “Entiendo \
         por qué hablas” → true. This applies to verb endings too: a missing accent on a \
         correctly chosen form (“envio” for “envío”, “hablaria” for “hablaría”) is never \
         a conjugation error. Spanish drops subject pronouns by default: an answer \
         with or without an explicit subject pronoun is equally grammatical — never \
         require one. Any valid Spanish phrasing counts, including object pronouns before \
         a finite verb (“Los puedes ver”) — do not require any particular construction. \
         Real misspellings — wrong or missing letters beyond accents, including n where ñ \
         belongs — DO make it ungrammatical.\n\
         2. `conveys_meaning`: does the answer express the English cue's meaning? Accept \
         reasonable synonyms and alternative phrasings. Reject changes of person, number, \
         polarity, tense, or referent that alter who does what to whom. Judge this \
         completely independently of the target structure: an answer that ignores the \
         target structure can still convey the meaning perfectly.\n\
         3. `uses_target_structure`: does the answer exercise the named target structure? \
         A grammatically fine answer that sidesteps the target (e.g. a different opener, \
         or pronoun placement the unit is not drilling) gets `false` here — that is a \
         signal, not an error. This judgment must never influence the other two.\n\
         \n\
         Every `evidence` note quotes the specific span at issue and says in a few words \
         why — never just the whole answer.\n\
         \n\
         If `grammatical` and `conveys_meaning` are not BOTH true, you MUST also fill:\n\
         - `error`: the single most pedagogically important mistake, classified into \
         exactly one category from the list below, with `evidence` quoting the exact span \
         of the learner's answer that shows it.\n\
         - `hint`: one short sentence that points the learner toward the fix without \
         giving the corrected sentence away.\n\
         - `explanation`: two or three sentences of pedagogy about this specific mistake \
         — why it is wrong and what rule governs the correct form.\n\
         Otherwise set `error`, `hint`, and `explanation` to null.\n\
         \n\
         Error categories:\n{}",
        category_glosses()
    )
}

fn user_prompt(input: &EvalInput) -> String {
    format!(
        "English cue: {}\nLearner's answer: {}\nTarget structure: {}",
        input.cue, input.answer, input.target_description
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2::curriculum;

    fn judgment(verdict: bool, evidence: &str) -> Judgment {
        Judgment {
            verdict,
            evidence: evidence.into(),
        }
    }

    fn analysis(a: bool, b: bool, c: bool) -> Tier1Analysis {
        Tier1Analysis {
            accent_restored_answer: "restored".into(),
            grammatical: judgment(a, "a"),
            conveys_meaning: judgment(b, "b"),
            uses_target_structure: judgment(c, "c"),
            error: None,
            hint: None,
            explanation: None,
        }
    }

    fn resolve_with(analysis: &Tier1Analysis) -> Result<Tier1Outcome, Tier1Error> {
        let c = curriculum::load_embedded().unwrap();
        resolve(
            analysis,
            "question.wh",
            "Wh-questions",
            &["clitic.do.sg.attach".to_string()],
            &c,
        )
    }

    #[test]
    fn correct_is_grammatical_and_conveys_meaning_and_uses_target() {
        assert_eq!(
            resolve_with(&analysis(true, true, true)).unwrap(),
            Tier1Outcome::Correct
        );
    }

    #[test]
    fn correct_but_structure_avoiding_resolves_to_the_dodge_with_a_nudge() {
        match resolve_with(&analysis(true, true, false)).unwrap() {
            Tier1Outcome::Dodge { nudge } => {
                assert!(nudge.contains("Wh-questions"), "nudge names the target: {nudge}");
                assert!(nudge.contains("Correct"), "the learner is told they were right: {nudge}");
            }
            other => panic!("expected dodge, got {other:?}"),
        }
    }

    #[test]
    fn wrong_resolves_with_classification_and_code_attributed_skills() {
        let mut a = analysis(false, true, true);
        a.error = Some(ErrorFinding {
            category: ErrorCategory::CliticPlacement,
            evidence: "va a llamarlo".into(),
        });
        a.hint = Some("Where does the pronoun go?".into());
        a.explanation = Some("Clitics attach to the infinitive here.".into());
        match resolve_with(&a).unwrap() {
            Tier1Outcome::Wrong {
                category,
                evidence,
                hint,
                explanation,
                skills,
            } => {
                assert_eq!(category, ErrorCategory::CliticPlacement);
                assert_eq!(evidence, "va a llamarlo");
                assert!(hint.is_some() && explanation.is_some());
                // Attribution is code's, not the model's: the clitic error
                // lands on the stacked clitic skill.
                assert_eq!(skills, vec!["clitic.do.sg.attach".to_string()]);
            }
            other => panic!("expected wrong, got {other:?}"),
        }
    }

    #[test]
    fn ungrammatical_but_meaning_conveying_is_still_wrong() {
        // Correct = a AND b; one false is enough.
        let mut a = analysis(false, true, true);
        a.error = Some(ErrorFinding {
            category: ErrorCategory::VerbForm,
            evidence: "quieromos".into(),
        });
        assert!(matches!(
            resolve_with(&a).unwrap(),
            Tier1Outcome::Wrong { category: ErrorCategory::VerbForm, .. }
        ));
        let mut a = analysis(true, false, true);
        a.error = Some(ErrorFinding {
            category: ErrorCategory::LexicalChoice,
            evidence: "podemos".into(),
        });
        assert!(matches!(resolve_with(&a).unwrap(), Tier1Outcome::Wrong { .. }));
    }

    #[test]
    fn wrong_without_a_classification_does_not_resolve() {
        // Fail-safe: no verdict is written from an analysis the data model
        // cannot trust; the attempt stays pending.
        assert!(matches!(
            resolve_with(&analysis(false, false, false)),
            Err(Tier1Error::MissingError)
        ));
    }

    #[test]
    fn parse_accepts_schema_conformant_output() {
        let a = parse_tier1(
            r#"{
                "accent_restored_answer": "¿Lo entiendes?",
                "grammatical": {"verdict": true, "evidence": "well-formed"},
                "conveys_meaning": {"verdict": true, "evidence": "matches the cue"},
                "uses_target_structure": {"verdict": false, "evidence": "clitic precedes the finite verb"},
                "error": null,
                "hint": null,
                "explanation": null
            }"#,
        )
        .unwrap();
        assert!(a.grammatical.verdict);
        assert!(!a.uses_target_structure.verdict);
        assert!(a.error.is_none());
    }

    #[test]
    fn parse_rejects_categories_outside_the_closed_enum() {
        // V1's hallucinated tags must fail the parse, never coerce.
        let err = parse_tier1(
            r#"{
                "accent_restored_answer": "x",
                "grammatical": {"verdict": false, "evidence": "x"},
                "conveys_meaning": {"verdict": true, "evidence": "x"},
                "uses_target_structure": {"verdict": true, "evidence": "x"},
                "error": {"category": "gram.personal-a", "evidence": "x"},
                "hint": null,
                "explanation": null
            }"#,
        );
        assert!(matches!(err, Err(Tier1Error::Parse(_))));
    }

    #[test]
    fn schema_locks_the_category_to_the_twelve_wire_names() {
        let schema = tier1_schema();
        let cats = schema["properties"]["error"]["properties"]["category"]["enum"]
            .as_array()
            .unwrap();
        assert_eq!(cats.len(), 12);
        assert!(cats.iter().any(|c| c == "clitic-placement"));
        assert!(!cats.iter().any(|c| c == "gram.personal-a"));
    }

    #[test]
    fn prompt_never_contains_a_canonical_answer_field() {
        // The decomposed prompt judges the answer on its own terms; there
        // is nothing to anchor on.
        let p = user_prompt(&EvalInput {
            cue: "Can you see them?".into(),
            answer: "Los puedes ver".into(),
            target_description: "clitic attached to the infinitive".into(),
        });
        assert!(!p.to_lowercase().contains("canonical"));
        assert!(!p.to_lowercase().contains("expected answer"));
    }

    #[test]
    fn target_description_renders_title_and_glossed_atoms() {
        let c = curriculum::load_embedded().unwrap();
        let d = target_description(&c, "clitic.do.sg.attach").unwrap();
        assert!(d.contains("lo / la attached to the infinitive"), "{d}");
        assert!(target_description(&c, "nope").is_none());
    }
}
