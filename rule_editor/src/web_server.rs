use axum::{
    extract::{Query, State},
    http::{HeaderValue, Method},
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use std::collections::HashMap;
use feature::feature::{export_features, ExportedCardFeature};
use async_openai::{
    types::{
        CreateChatCompletionRequest, ChatCompletionRequestSystemMessage, 
        ChatCompletionRequestUserMessage, ChatCompletionRequestMessage,
        ChatCompletionToolChoiceOption, ChatCompletionRequestSystemMessageContent,
        ChatCompletionRequestUserMessageContent
    },
    Client, config::OpenAIConfig
};
use serde_json::json;

use crate::{models::rule_pattern::RulePattern, AppState};

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    keyword: String,
}

#[derive(Debug, Serialize)]
pub struct SentenceResult {
    pub id: String,
    pub text: String,
    pub card_number: String,
    pub card_name: String,
}

#[derive(Debug, Deserialize)]
pub struct PatternRequest {
    pub keyword: String,
    pub positive_examples: Vec<String>,
    pub negative_examples: Vec<String>,
    pub features: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct PatternSuggestion {
    pub pattern: String,
    pub explanation: String,
    pub features: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct FeatureResponse {
    pub features_by_tag: HashMap<String, Vec<ExportedCardFeature>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RegexPatternResponse {
    patterns: Vec<String>,
    explanation: String,
    success: bool,
}

#[derive(Debug, Deserialize)]
pub struct SavePatternRequest {
    pub keyword: String,
    pub pattern: String,
    pub features: Vec<String>,
    pub positive_examples: Vec<String>,
    pub negative_examples: Vec<String>,
}

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(serve_index))
        .route("/api/search", get(search_rawcards))
        .route("/api/generate-pattern", post(generate_pattern))
        .route("/api/patterns", get(get_patterns))
        .route("/api/patterns", post(save_pattern))
        .route("/api/export", post(export_patterns))
        .route("/api/features", get(get_features))
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST])
                .allow_headers(tower_http::cors::Any),
        )
        .with_state(app_state)
}

async fn serve_index() -> Html<String> {
    Html(include_str!("../ui/dist/index.html").to_string())
}

async fn search_rawcards(
    Query(query): Query<SearchQuery>,
    State(app_state): State<AppState>,
) -> Json<Vec<SentenceResult>> {
    let pool = &app_state.pool;
    
    let sql_query = r#"
        SELECT card_number, name, skill_text
        FROM wix_rawcard
        WHERE skill_text LIKE $1
        LIMIT 100
    "#;
    
    let search_pattern = format!("%{}%", query.keyword);
    let rows = sqlx::query_as::<_, (String, String, String)>(sql_query)
        .bind(&search_pattern)
        .fetch_all(pool.as_ref())
        .await
        .unwrap_or_default();
    
    let mut results = Vec::new();
    
    for (card_number, card_name, skill_text) in rows {
        if skill_text.contains(&query.keyword) {
            results.push(SentenceResult {
                id: card_number.clone(),
                text: skill_text,
                card_number: card_number.clone(),
                card_name: card_name.clone(),
            });
        }
    }
    
    Json(results)
}

async fn generate_pattern(
    State(app_state): State<AppState>,
    Json(request): Json<PatternRequest>,
) -> Json<PatternSuggestion> {
    println!("=== AI パターン生成リクエスト（元データ） ===");
    println!("Search Keyword: {}", request.keyword);
    println!("Selected Features (context): {:?}", request.features);
    println!("Positive Examples ({} items):", request.positive_examples.len());
    for (i, example) in request.positive_examples.iter().enumerate() {
        println!("  [{}] {}", i + 1, example);
    }
    println!("================================");
    
    // 前処理: ポジティブサンプルを句点で分割し、検索キーワードを含む文のみを抽出
    let mut filtered_sentences = Vec::new();
    
    for skill_text in &request.positive_examples {
        let sentences: Vec<&str> = skill_text.split('。').collect();
        
        for sentence in sentences {
            let trimmed = sentence.trim();
            if trimmed.is_empty() {
                continue;
            }
            
            if trimmed.contains(&request.keyword) {
                filtered_sentences.push(trimmed.to_string());
            }
        }
    }
    
    // CardFeatureの日本語ラベルから内部コードを逆引きする処理
    let feature_context = request.features.iter().map(|feature_label| {
        let internal_code = match feature_label.as_str() {
            "トラッシュ回収" => "Salvage",
            "バニッシュ" => "Banish", 
            "ドロー" => "Draw",
            "エナチャージ" => "Charge",
            "アサシン" => "Assassin",
            "ガード" => "Guard",
            _ => "Unknown"
        };
        format!("{}({})", internal_code, feature_label)
    }).collect::<Vec<String>>();
    
    println!("=== 前処理後（AI APIに送信予定のデータ） ===");
    println!("Target CardFeature (context): {:?}", feature_context);
    println!("Search Keyword: {}", request.keyword);
    println!("Filtered Sentences containing '{}' ({} items):", request.keyword, filtered_sentences.len());
    for (i, sentence) in filtered_sentences.iter().enumerate() {
        println!("  [{}] {}", i + 1, sentence);
    }
    
    if filtered_sentences.is_empty() {
        println!("⚠️  警告: 検索キーワード「{}」を含む文が見つかりませんでした", request.keyword);
        
        return Json(PatternSuggestion {
            pattern: request.keyword.clone(),
            explanation: "検索キーワードを含む例文が見つからないため、シンプルなパターンを返しました。".to_string(),
            features: request.features,
        });
    }
    
    // OpenAI APIを呼び出し
    match call_openai_for_pattern(&app_state.openai_client, &request.keyword, &feature_context, &filtered_sentences).await {
        Ok(ai_response) => {
            println!("=== OpenAI API Response ===");
            println!("Success: {}", ai_response.success);
            println!("Patterns: {:?}", ai_response.patterns);
            println!("Explanation: {}", ai_response.explanation);
            println!("============================");
            
            if ai_response.success && !ai_response.patterns.is_empty() {
                // 複数パターンがある場合は最初のパターンを使用
                // TODO: 複数パターンに対応した PatternSuggestion の設計変更を検討
                Json(PatternSuggestion {
                    pattern: ai_response.patterns[0].clone(),
                    explanation: ai_response.explanation,
                    features: request.features,
                })
            } else {
                Json(PatternSuggestion {
                    pattern: request.keyword.clone(),
                    explanation: "AIによるパターン生成に失敗しました。シンプルなパターンを返します。".to_string(),
                    features: request.features,
                })
            }
        }
        Err(e) => {
            println!("OpenAI API Error: {}", e);
            Json(PatternSuggestion {
                pattern: request.keyword.clone(),
                explanation: format!("APIエラーのため、シンプルなパターンを返します。エラー: {}", e),
                features: request.features,
            })
        }
    }
}

async fn call_openai_for_pattern(
    client: &Client<OpenAIConfig>,
    keyword: &str,
    feature_context: &[String],
    filtered_sentences: &[String],
) -> Result<RegexPatternResponse, Box<dyn std::error::Error + Send + Sync>> {
    // Tool Calling用のfunction schema
    let function = json!({
        "type": "function",
        "function": {
            "name": "generate_regex_patterns",
            "description": "Generate regex patterns for WIXOSS card feature detection",
            "parameters": {
                "type": "object",
                "properties": {
                    "patterns": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "Rust regex patterns (e.g. r\"手札に加える\")"
                    },
                    "explanation": {
                        "type": "string",
                        "description": "Reasoning for pattern choices and how they work"
                    },
                    "success": {
                        "type": "boolean",
                        "description": "Whether satisfactory patterns were generated"
                    }
                },
                "required": ["patterns", "explanation", "success"]
            }
        }
    });
    
    let system_prompt = "あなたはWIXOSSトレーディングカードゲームのテキスト解析専門家です。\
    カードのスキルテキストから特定の機能を検出するための正規表現パターンを生成してください。\n\n\
    重要な原則：\n\
    - 既存の成功例を参考にシンプルなパターンを作成してください\n\
    - 例文全体をマッチングしようとせず、核となる機能部分のみを検出\n\
    - 条件部分（【使用条件】等）は無視し、実際の効果部分に注目\n\
    - Rustの正規表現構文を使用\n\
    - パターンにはr\"\"は含めず、正規表現のみを返す\n\n\
    成功パターンの例：\n\
    - トラッシュ回収: \"手札に加える\"\n\
    - バニッシュ: \"バニッシュ\"\n\
    - ドロー: \"ドロー\"\n\
    - エナチャージ: \"エナチャージ\"\n\
    - アサシン: \"【アサシン】\"\n\
    - ガード: \"【ガード】\"\n\n\
    これらの例のように、機能を表す核心的なキーワードや記号に焦点を当ててください。";
    
    let user_prompt = format!(
        "対象機能: {}\n検索キーワード: {}\n\n\
        以下の例文を参考に、機能を検出するためのシンプルな正規表現パターンを生成してください：\n{}\n\n\
        重要な指針：\n\
        1. 例文から共通する核心的なキーワードを抽出してください\n\
        2. 【使用条件】や複雑な条件文は無視し、実際の効果部分に焦点を当ててください\n\
        3. 上記の成功例のように、機能を表す最小限のキーワードでパターンを作成してください\n\
        4. 例: 「○○を手札に加える」なら「手札に加える」、「○○をバニッシュする」なら「バニッシュ」\n\
        5. 能力名が【】で囲まれている場合は記号も含めてください（例：【アサシン】）\n\
        6. 複数の表現がある場合は、最もシンプルで一般的なパターンを優先してください",
        feature_context.join(", "),
        keyword,
        filtered_sentences.iter().enumerate()
            .map(|(i, s)| format!("{}. {}", i + 1, s))
            .collect::<Vec<String>>()
            .join("\n")
    );
    
    let request = CreateChatCompletionRequest {
        model: "gpt-4o-mini".to_string(),
        messages: vec![
            ChatCompletionRequestMessage::System(
                ChatCompletionRequestSystemMessage {
                    content: ChatCompletionRequestSystemMessageContent::Text(system_prompt.to_string()),
                    name: None,
                }
            ),
            ChatCompletionRequestMessage::User(
                ChatCompletionRequestUserMessage {
                    content: ChatCompletionRequestUserMessageContent::Text(user_prompt),
                    name: None,
                }
            ),
        ],
        tools: Some(vec![serde_json::from_value(function)?]),
        tool_choice: Some(ChatCompletionToolChoiceOption::Required),
        ..Default::default()
    };
    
    let response = client.chat().create(request).await?;
    
    if let Some(choice) = response.choices.first() {
        if let Some(tool_calls) = &choice.message.tool_calls {
            if let Some(tool_call) = tool_calls.first() {
                let regex_response: RegexPatternResponse = 
                    serde_json::from_str(&tool_call.function.arguments)?;
                return Ok(regex_response);
            }
        }
    }
    
    Err("No valid response from OpenAI".into())
}

async fn get_patterns(State(app_state): State<AppState>) -> Json<Vec<RulePattern>> {
    let pool = &app_state.pool;
    
    let query = r#"
        SELECT id, keyword, pattern, features, positive_examples, 
               negative_examples, created_at, updated_at, is_active
        FROM wix_rule_pattern
        WHERE is_active = true
        ORDER BY created_at DESC
    "#;
    
    let patterns = sqlx::query_as::<_, RulePattern>(query)
        .fetch_all(pool.as_ref())
        .await
        .unwrap_or_default();
    
    Json(patterns)
}

async fn save_pattern(
    State(app_state): State<AppState>,
    Json(request): Json<SavePatternRequest>,
) -> Json<serde_json::Value> {
    let pool = &app_state.pool;
    
    let query = r#"
        INSERT INTO wix_rule_pattern 
        (keyword, pattern, features, positive_examples, negative_examples)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
    "#;
    
    let features_json = serde_json::to_value(&request.features).unwrap();
    let positive_json = serde_json::to_value(&request.positive_examples).unwrap();
    let negative_json = serde_json::to_value(&request.negative_examples).unwrap();
    
    match sqlx::query_as::<_, (i32,)>(query)
        .bind(&request.keyword)
        .bind(&request.pattern)
        .bind(&features_json)
        .bind(&positive_json)
        .bind(&negative_json)
        .fetch_one(pool.as_ref())
        .await
    {
        Ok(row) => Json(serde_json::json!({"success": true, "id": row.0})),
        Err(e) => Json(serde_json::json!({"success": false, "error": e.to_string()})),
    }
}

async fn export_patterns(State(app_state): State<AppState>) -> Json<serde_json::Value> {
    match crate::export_patterns_direct(&app_state).await {
        Ok(code) => {
            // ファイルに書き出し
            let output_path = "shared/feature/src/generated_patterns.rs";
            match std::fs::write(output_path, &code) {
                Ok(_) => Json(serde_json::json!({
                    "success": true, 
                    "code": code,
                    "path": output_path
                })),
                Err(e) => Json(serde_json::json!({
                    "success": false, 
                    "error": format!("Failed to write file: {}", e)
                })),
            }
        }
        Err(e) => Json(serde_json::json!({
            "success": false,
            "error": e
        })),
    }
}

async fn get_features() -> Json<FeatureResponse> {
    let features_by_tag = export_features();
    Json(FeatureResponse { features_by_tag })
}