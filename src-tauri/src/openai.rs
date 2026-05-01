use async_openai::{
    config::OpenAIConfig,
    types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs},
    Client,
};

#[tauri::command]
pub async fn openai_ping() -> Result<String, String> {
    let api_key = std::env::var("OPENAI_API_KEY")
        .map_err(|_| "OPENAI_API_KEY not set in environment (.env)".to_string())?;

    let client = Client::with_config(OpenAIConfig::new().with_api_key(api_key));

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o-mini")
        .max_tokens(16u32)
        .messages([ChatCompletionRequestUserMessageArgs::default()
            .content("Reply with exactly one word: hola")
            .build()
            .map_err(|e| e.to_string())?
            .into()])
        .build()
        .map_err(|e| e.to_string())?;

    let response = client
        .chat()
        .create(request)
        .await
        .map_err(|e| e.to_string())?;

    let content = response
        .choices
        .first()
        .and_then(|c| c.message.content.clone())
        .unwrap_or_default();

    Ok(content.trim().to_string())
}
