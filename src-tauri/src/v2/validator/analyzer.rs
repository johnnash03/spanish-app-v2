//! The LLM half of the validator: one analyzer call per candidate item,
//! producing the structured [`ItemAnalysis`] the judge consumes.
//!
//! The analyzer is deliberately licensing-blind: it receives the full
//! curriculum construction registry and the full form-slot registry, never
//! the unit's licensed subset, so it has nothing to be agreeable about —
//! it describes the sentence, and the judge does the deciding.

use super::types::{CandidateItem, ItemAnalysis};
use crate::v2::curriculum::types::FORM_SLOTS;
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs, ResponseFormat, ResponseFormatJsonSchema,
    },
    Client,
};
use thiserror::Error;

/// Capability-tier default (PRD #31: frontier tier for analysis). The
/// model identifier is configuration: override with `V2_ANALYZER_MODEL`.
const DEFAULT_ANALYZER_MODEL: &str = "gpt-4o-2024-08-06";

pub fn analyzer_model() -> String {
    std::env::var("V2_ANALYZER_MODEL").unwrap_or_else(|_| DEFAULT_ANALYZER_MODEL.to_string())
}

/// Glosses for every registered construction tag, embedded in the analyzer
/// prompt so the model labels sentences in the curriculum's closed tag
/// vocabulary. A curriculum construction without a gloss is a test failure
/// (`gloss_registry_covers_every_curriculum_construction`).
pub const CONSTRUCTION_GLOSSES: &[(&str, &str)] = &[
    // Ambient set
    ("art.def", "definite article (el/la/los/las) with a noun"),
    ("art.indef", "indefinite article (un/una/unos/unas) with a noun"),
    ("gender.agreement.basic", "article/adjective agrees with noun gender"),
    ("plural.formation.basic", "plural noun formation (-s/-es)"),
    ("neg.no.preverbal", "negation with `no` directly before the finite verb"),
    ("pron.subject.optional", "subject pronoun omitted (or explicitly present for emphasis)"),
    // Phase 1 — openers
    ("opener.finite+inf", "finite opener verb followed directly by an infinitive (quiero comer)"),
    ("neg.tampoco", "tampoco — negative `too/either`"),
    ("opener.tener-que", "tener que + infinitive (obligation)"),
    ("opener.ir-a+inf", "ir a + infinitive (near future)"),
    // Phase 2 — direct-object clitics
    ("clitic.do.sg.attach-to-inf", "singular direct-object clitic lo/la attached to an infinitive (quiero verlo)"),
    ("clitic.do.pl.attach-to-inf", "plural direct-object clitic los/las attached to an infinitive"),
    ("clitic.do.person.attach-to-inf", "personal direct-object clitic me/te/nos attached to an infinitive"),
    // Phase 3 — indirect & two-pronoun clitics
    ("clitic.io.attach-to-inf", "indirect-object clitic le/les attached to an infinitive"),
    ("clitic.both.attach-to-inf", "two clitics (indirect then direct: me lo / te la / nos los) attached to an infinitive"),
    ("clitic.both.se-lo-substitution", "le/les replaced by se before lo/la/los/las (quiero dárselo)"),
    // Phase 4 — questions
    ("question.yes-no.intonation", "yes/no question formed by intonation, written ¿…?"),
    ("question.wh.fronting", "fronted question word (qué/quién/dónde/cuándo/cómo/por qué/cuánto)"),
    ("question.embedded.after-saber", "embedded question after saber (saber si/qué/dónde…)"),
];

#[derive(Debug, Error)]
pub enum AnalyzerError {
    #[error("analyzer transport error: {0}")]
    Transport(String),
    #[error("analyzer returned no content")]
    EmptyResponse,
    #[error("analyzer output failed schema parse: {0}")]
    Parse(#[from] serde_json::Error),
}

/// One analyzer call per item. Implementations must never judge — only
/// describe. Errors always reject the item (fail-safe), never pass it.
pub trait Analyzer {
    fn analyze(
        &self,
        item: &CandidateItem,
    ) -> impl std::future::Future<Output = Result<ItemAnalysis, AnalyzerError>> + Send;
}

pub struct OpenAiAnalyzer {
    client: Client<OpenAIConfig>,
    model: String,
}

impl OpenAiAnalyzer {
    /// Reads `OPENAI_API_KEY` (and optionally `V2_ANALYZER_MODEL`) from the
    /// environment.
    pub fn from_env() -> Result<Self, AnalyzerError> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| AnalyzerError::Transport("OPENAI_API_KEY not set".into()))?;
        Ok(Self {
            client: Client::with_config(OpenAIConfig::new().with_api_key(api_key)),
            model: analyzer_model(),
        })
    }
}

impl Analyzer for OpenAiAnalyzer {
    async fn analyze(&self, item: &CandidateItem) -> Result<ItemAnalysis, AnalyzerError> {
        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.model)
            .response_format(ResponseFormat::JsonSchema {
                json_schema: ResponseFormatJsonSchema {
                    name: "item_analysis".into(),
                    description: Some(
                        "Structured linguistic inventory of a Spanish sentence".into(),
                    ),
                    schema: Some(analysis_schema()),
                    strict: Some(true),
                },
            })
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system_prompt())
                    .build()
                    .map_err(|e| AnalyzerError::Transport(e.to_string()))?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(user_prompt(item))
                    .build()
                    .map_err(|e| AnalyzerError::Transport(e.to_string()))?
                    .into(),
            ])
            .build()
            .map_err(|e| AnalyzerError::Transport(e.to_string()))?;

        let response = self
            .client
            .chat()
            .create(request)
            .await
            .map_err(|e| AnalyzerError::Transport(e.to_string()))?;

        let content = response
            .choices
            .first()
            .and_then(|c| c.message.content.as_deref())
            .ok_or(AnalyzerError::EmptyResponse)?;
        parse_analysis(content)
    }
}

/// Parses analyzer output. Schema enforcement happens server-side; this is
/// the client-side backstop, and any failure rejects the item.
pub fn parse_analysis(content: &str) -> Result<ItemAnalysis, AnalyzerError> {
    Ok(serde_json::from_str(content)?)
}

/// Strict JSON schema for [`ItemAnalysis`]. The `form` field is an enum
/// over the registered slots plus `"other"`, so an unplaceable form is
/// expressible (and then rejected by the judge) rather than hallucinated
/// into a real slot.
pub fn analysis_schema() -> serde_json::Value {
    let mut form_slots: Vec<&str> = FORM_SLOTS.to_vec();
    form_slots.push("other");
    serde_json::json!({
        "type": "object",
        "properties": {
            "verb_forms": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "lemma": { "type": "string" },
                        "form": { "type": "string", "enum": form_slots },
                        "surface": { "type": "string" }
                    },
                    "required": ["lemma", "form", "surface"],
                    "additionalProperties": false
                }
            },
            "constructions": {
                "type": "array",
                "items": { "type": "string" }
            },
            "content_lemmas": {
                "type": "array",
                "items": { "type": "string" }
            }
        },
        "required": ["verb_forms", "constructions", "content_lemmas"],
        "additionalProperties": false
    })
}

fn system_prompt() -> String {
    let glosses = CONSTRUCTION_GLOSSES
        .iter()
        .map(|(tag, gloss)| format!("- `{tag}` — {gloss}"))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "You are a Spanish linguistic analyzer. Given one Spanish sentence, produce a \
         complete structured inventory of it. You never judge whether the sentence is \
         appropriate — you only describe what it contains.\n\
         \n\
         1. `verb_forms`: every verb occurrence. For each, give the lemma (infinitive), \
         the paradigm slot, and the exact surface text. Use slot `other` whenever the \
         form does not fit a listed slot (e.g. compound tenses, vosotros forms). Never \
         force a form into the nearest slot.\n\
         2. `constructions`: every tag from the registry below that the sentence \
         exhibits. Use only registry tags, and list every one that applies. Ignore \
         phenomena the registry has no tag for.\n\
         3. `content_lemmas`: lemmas of content words that are not verbs — nouns, \
         adjectives, adverbs. Exclude function words (articles, pronouns, clitics, \
         prepositions, conjunctions, question words) and exclude verbs.\n\
         \n\
         Construction registry:\n{glosses}"
    )
}

fn user_prompt(item: &CandidateItem) -> String {
    format!(
        "English cue: {}\nSpanish sentence to analyze: {}",
        item.source, item.canonical
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2::curriculum;

    #[test]
    fn gloss_registry_covers_every_curriculum_construction() {
        // The analyzer can only label what it has a gloss for; a grant
        // with no gloss would make its construction invisible to analysis.
        let c = curriculum::load_embedded().unwrap();
        let glossed: std::collections::HashSet<&str> =
            CONSTRUCTION_GLOSSES.iter().map(|(tag, _)| *tag).collect();
        for tag in c.construction_registry() {
            assert!(glossed.contains(tag.as_str()), "construction `{tag}` has no gloss");
        }
    }

    #[test]
    fn schema_enumerates_form_slots_plus_other_escape() {
        let schema = analysis_schema();
        let form_enum = schema["properties"]["verb_forms"]["items"]["properties"]["form"]["enum"]
            .as_array()
            .unwrap();
        assert!(form_enum.iter().any(|v| v == "other"));
        assert!(form_enum.iter().any(|v| v == "pres.1sg"));
        assert_eq!(
            form_enum.len(),
            crate::v2::curriculum::types::FORM_SLOTS.len() + 1
        );
    }

    #[test]
    fn parse_accepts_schema_conformant_output() {
        let analysis = parse_analysis(
            r#"{
                "verb_forms": [
                    {"lemma": "querer", "form": "pres.1sg", "surface": "quiero"},
                    {"lemma": "comer", "form": "inf", "surface": "comer"}
                ],
                "constructions": ["opener.finite+inf"],
                "content_lemmas": ["ahora"]
            }"#,
        )
        .unwrap();
        assert_eq!(analysis.verb_forms.len(), 2);
        assert_eq!(analysis.constructions, vec!["opener.finite+inf"]);
    }

    #[test]
    fn parse_rejects_malformed_output() {
        assert!(matches!(
            parse_analysis("the sentence uses quiero"),
            Err(AnalyzerError::Parse(_))
        ));
        // Wrong shape (missing required field) also fails.
        assert!(matches!(
            parse_analysis(r#"{"verb_forms": "quiero"}"#),
            Err(AnalyzerError::Parse(_))
        ));
    }
}
