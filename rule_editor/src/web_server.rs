use axum::{
    extract::{Query, State},
    http::{HeaderValue, Method},
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;

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
    let mut id_counter = 0;
    
    for (card_number, card_name, skill_text) in rows {
        let sentences: Vec<&str> = skill_text.split('。').collect();
        
        for sentence in sentences {
            if sentence.contains(&query.keyword) && !sentence.trim().is_empty() {
                id_counter += 1;
                results.push(SentenceResult {
                    id: format!("{}-{}", card_number, id_counter),
                    text: format!("{}。", sentence.trim()),
                    card_number: card_number.clone(),
                    card_name: card_name.clone(),
                });
            }
        }
    }
    
    Json(results)
}

async fn generate_pattern(
    State(_app_state): State<AppState>,
    Json(request): Json<PatternRequest>,
) -> Json<PatternSuggestion> {
    // Simple pattern generation without OpenAI for now
    let pattern = if request.keyword.contains("手札に加え") {
        r"手札に加え"
    } else if request.keyword.contains("バニッシュ") {
        r"バニッシュ"
    } else {
        &request.keyword
    };
    
    let features = if pattern.contains("手札に加え") {
        vec!["Salvage".to_string()]
    } else if pattern.contains("バニッシュ") {
        vec!["Banish".to_string()]
    } else {
        request.features
    };
    
    Json(PatternSuggestion {
        pattern: pattern.to_string(),
        explanation: format!("Generated pattern for keyword: {}", request.keyword),
        features,
    })
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