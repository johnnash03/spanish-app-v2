//! The OpenAI streaming [`ItemSource`]: one chat-completion stream per
//! round, items extracted and forwarded the moment their closing brace
//! arrives. The system prompt is the stable
//! [`super::prompt::STABLE_SYSTEM_PROMPT`], so the provider's prompt cache
//! holds it across units and rounds (v1 behavior).

use super::extract::extract_complete_items;
use super::pipeline::{GeneratorError, ItemSource};
use super::types::GeneratedItem;
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use futures_util::StreamExt;
use tokio::sync::mpsc::UnboundedSender;

/// Capability-tier default (PRD #31: frontier tier for generation). The
/// model identifier is configuration: override with `V2_GENERATOR_MODEL`.
const DEFAULT_GENERATOR_MODEL: &str = "gpt-4o-2024-08-06";

pub fn generator_model() -> String {
    std::env::var("V2_GENERATOR_MODEL").unwrap_or_else(|_| DEFAULT_GENERATOR_MODEL.to_string())
}

pub struct OpenAiItemSource {
    client: Client<OpenAIConfig>,
    model: String,
}

impl OpenAiItemSource {
    /// Reads `OPENAI_API_KEY` (and optionally `V2_GENERATOR_MODEL`) from
    /// the environment.
    pub fn from_env() -> Result<Self, GeneratorError> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| GeneratorError::Source("OPENAI_API_KEY not set".into()))?;
        Ok(Self {
            client: Client::with_config(OpenAIConfig::new().with_api_key(api_key)),
            model: generator_model(),
        })
    }
}

impl ItemSource for OpenAiItemSource {
    async fn stream_items(
        &self,
        system: &str,
        user: &str,
        tx: UnboundedSender<GeneratedItem>,
    ) -> Result<(), GeneratorError> {
        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.model)
            .temperature(0.7_f32)
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system)
                    .build()
                    .map_err(|e| GeneratorError::Source(e.to_string()))?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(user)
                    .build()
                    .map_err(|e| GeneratorError::Source(e.to_string()))?
                    .into(),
            ])
            .build()
            .map_err(|e| GeneratorError::Source(e.to_string()))?;

        let mut stream = self
            .client
            .chat()
            .create_stream(request)
            .await
            .map_err(|e| GeneratorError::Source(e.to_string()))?;

        let mut buffer = String::new();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| GeneratorError::Source(e.to_string()))?;
            for choice in &chunk.choices {
                if let Some(content) = &choice.delta.content {
                    buffer.push_str(content);
                    let (items, consumed) = extract_complete_items(&buffer);
                    for item in items {
                        // A closed receiver means the pipeline stopped;
                        // nothing useful is left to stream.
                        if tx.send(item).is_err() {
                            return Ok(());
                        }
                    }
                    buffer.drain(..consumed);
                }
            }
        }
        for item in extract_complete_items(&buffer).0 {
            let _ = tx.send(item);
        }
        Ok(())
    }
}
