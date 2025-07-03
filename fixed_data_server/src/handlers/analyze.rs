use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyzeResponse {
    pub pronunciation: String,
    pub cards_analyzed: usize,
    pub success: bool,
    pub message: String,
}

pub async fn analyze_card(
    State(pool): State<PgPool>,
    Path(pronunciation): Path<String>,
) -> Result<Json<AnalyzeResponse>, StatusCode> {
    // Get card numbers for this pronunciation
    let card_numbers: Vec<String> =
        sqlx::query_scalar("SELECT code FROM wix_card WHERE pronunciation = $1")
            .bind(&pronunciation)
            .fetch_all(&pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if card_numbers.is_empty() {
        return Ok(Json(AnalyzeResponse {
            pronunciation: pronunciation.clone(),
            cards_analyzed: 0,
            success: false,
            message: "No cards found with this pronunciation".to_string(),
        }));
    }

    // Run analyzer for each card number
    let mut success_count = 0;
    let mut errors = Vec::new();

    for card_number in &card_numbers {
        let output = Command::new("cargo")
            .args(&["run", "-p", "analyzer", "--", "--card-number", card_number])
            .output()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if output.status.success() {
            success_count += 1;
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            errors.push(format!("{}: {}", card_number, error_msg));
        }
    }

    let response = AnalyzeResponse {
        pronunciation,
        cards_analyzed: success_count,
        success: success_count == card_numbers.len(),
        message: if errors.is_empty() {
            format!("Successfully analyzed {} cards", success_count)
        } else {
            format!(
                "Analyzed {} of {} cards. Errors: {}",
                success_count,
                card_numbers.len(),
                errors.join("; ")
            )
        },
    };

    Ok(Json(response))
}
