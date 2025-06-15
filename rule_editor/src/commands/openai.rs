use crate::AppState;
use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct PatternRequest {
    pub keyword: String,
    pub positive_examples: Vec<String>,
    pub negative_examples: Vec<String>,
    pub features: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatternSuggestion {
    pub pattern: String,
    pub explanation: String,
    pub features: Vec<String>,
}

#[tauri::command]
pub async fn generate_pattern(
    request: PatternRequest,
    state: State<'_, AppState>,
) -> Result<PatternSuggestion, String> {
    let client = &state.openai_client;
    
    // プロンプトの構築
    let system_prompt = r#"あなたはWIXOSSカードゲームのテキストパターンを解析する専門家です。
与えられたキーワードと例文から、適切な日本語正規表現パターンを生成してください。

重要な記号：
- 【】: 能力名を囲む（例：【アサシン】、【ライフバースト】）
- 《》: アイコンやマーカーを囲む（例：《ガードアイコン》、《白》）
- 「」: カード名を囲む
- ：（全角コロン）: タイミング指定（例：出：、自：）

パターンは以下の形式で返してください：
{
  "pattern": "正規表現パターン",
  "explanation": "このパターンの説明",
  "features": ["検出されるCardFeature名"]
}"#;

    let mut positive_text = String::from("マッチすべき例：\n");
    for example in &request.positive_examples {
        positive_text.push_str(&format!("- {}\n", example));
    }
    
    let mut negative_text = String::from("\nマッチすべきでない例：\n");
    for example in &request.negative_examples {
        negative_text.push_str(&format!("- {}\n", example));
    }
    
    let user_prompt = format!(
        "キーワード: 「{}」\n\n{}{}\n\n検出すべきCardFeature: {:?}\n\n適切な正規表現パターンを生成してください。",
        request.keyword, positive_text, negative_text, request.features
    );
    
    let messages = vec![
        ChatCompletionRequestMessage::System(
            ChatCompletionRequestSystemMessageArgs::default()
                .content(system_prompt)
                .build()
                .map_err(|e| format!("Failed to build system message: {}", e))?
        ),
        ChatCompletionRequestMessage::User(
            ChatCompletionRequestUserMessageArgs::default()
                .content(user_prompt)
                .build()
                .map_err(|e| format!("Failed to build user message: {}", e))?
        ),
    ];
    
    let model = std::env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-4o-mini".to_string());
    
    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages(messages)
        .temperature(0.3)
        .response_format(async_openai::types::ChatCompletionResponseFormat::JsonObject {
            json_object: async_openai::types::ChatCompletionResponseFormatJsonObject {
                r#type: async_openai::types::ChatCompletionResponseFormatJsonObjectType::JsonObject,
            },
        })
        .build()
        .map_err(|e| format!("Failed to build request: {}", e))?;
    
    let response = client
        .chat()
        .create(request)
        .await
        .map_err(|e| format!("OpenAI API error: {}", e))?;
    
    let content = response
        .choices
        .first()
        .and_then(|choice| choice.message.content.clone())
        .ok_or_else(|| "No response from OpenAI".to_string())?;
    
    // JSONレスポンスをパース
    let suggestion: PatternSuggestion = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse OpenAI response: {}", e))?;
    
    Ok(suggestion)
}