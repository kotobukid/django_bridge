use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use chrono::Utc;
use sqlx::PgPool;
use crate::models::{CardFeatureOverride, ImportExportData};

pub async fn export_all(
    State(pool): State<PgPool>,
) -> Result<Json<ImportExportData>, StatusCode> {
    let overrides = sqlx::query_as::<_, CardFeatureOverride>(
        "SELECT * FROM wix_card_feature_override ORDER BY pronunciation"
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let export_data = ImportExportData {
        overrides,
        exported_at: Utc::now(),
        version: "1.0".to_string(),
    };

    Ok(Json(export_data))
}

pub async fn import_data(
    State(pool): State<PgPool>,
    Json(data): Json<ImportExportData>,
) -> Result<Json<ImportResult>, StatusCode> {
    let mut imported = 0;
    let mut errors = Vec::new();

    for override_data in data.overrides {
        let result = sqlx::query(
            r#"
            INSERT INTO wix_card_feature_override 
            (pronunciation, fixed_bits1, fixed_bits2, fixed_burst_bits, note, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (pronunciation) DO UPDATE SET
                fixed_bits1 = EXCLUDED.fixed_bits1,
                fixed_bits2 = EXCLUDED.fixed_bits2,
                fixed_burst_bits = EXCLUDED.fixed_burst_bits,
                note = EXCLUDED.note,
                updated_at = EXCLUDED.updated_at
            "#
        )
        .bind(&override_data.pronunciation)
        .bind(override_data.fixed_bits1)
        .bind(override_data.fixed_bits2)
        .bind(override_data.fixed_burst_bits)
        .bind(&override_data.note)
        .bind(override_data.created_at)
        .bind(override_data.updated_at)
        .execute(&pool)
        .await;

        match result {
            Ok(_) => imported += 1,
            Err(e) => errors.push(format!("{}: {}", override_data.pronunciation, e)),
        }
    }

    let success = errors.is_empty();
    let result = ImportResult {
        imported,
        errors: if errors.is_empty() { None } else { Some(errors) },
        success,
    };

    Ok(Json(result))
}

#[derive(Debug, serde::Serialize)]
pub struct ImportResult {
    pub imported: usize,
    pub errors: Option<Vec<String>>,
    pub success: bool,
}