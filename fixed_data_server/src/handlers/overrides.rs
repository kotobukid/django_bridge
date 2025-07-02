use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::{PgPool, Row};
use crate::models::{CardFeatureOverride, CreateOverrideRequest, OverrideResponse, ConsistencyCheckResult};
use feature::feature::{CardFeature, BurstFeature, HashSetToBits, BurstHashSetToBits};
use std::collections::HashSet;

pub async fn list_overrides(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<OverrideResponse>>, StatusCode> {
    let overrides = sqlx::query_as::<_, CardFeatureOverride>(
        "SELECT * FROM wix_card_feature_override ORDER BY pronunciation"
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let responses: Vec<OverrideResponse> = overrides
        .into_iter()
        .map(|o| convert_to_response(o))
        .collect();

    Ok(Json(responses))
}

pub async fn list_override_pronunciations(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<String>>, StatusCode> {
    let pronunciations: Vec<String> = sqlx::query(
        "SELECT DISTINCT pronunciation FROM wix_card_feature_override ORDER BY pronunciation"
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .into_iter()
    .map(|row| row.get("pronunciation"))
    .collect();

    Ok(Json(pronunciations))
}

pub async fn get_override(
    State(pool): State<PgPool>,
    Path(pronunciation): Path<String>,
) -> Result<Json<OverrideResponse>, StatusCode> {
    let override_data = sqlx::query_as::<_, CardFeatureOverride>(
        "SELECT * FROM wix_card_feature_override WHERE pronunciation = $1"
    )
    .bind(&pronunciation)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(convert_to_response(override_data)))
}

pub async fn create_or_update_override(
    State(pool): State<PgPool>,
    Json(request): Json<CreateOverrideRequest>,
) -> Result<Json<OverrideResponse>, StatusCode> {
    // Convert features to bit flags
    let (bits1, bits2) = convert_features_to_bits(&request.features);
    let burst_bits = convert_burst_features_to_bits(&request.burst_features);

    let override_data = sqlx::query_as::<_, CardFeatureOverride>(
        r#"
        INSERT INTO wix_card_feature_override 
        (pronunciation, fixed_bits1, fixed_bits2, fixed_burst_bits, note, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        ON CONFLICT (pronunciation) DO UPDATE SET
            fixed_bits1 = EXCLUDED.fixed_bits1,
            fixed_bits2 = EXCLUDED.fixed_bits2,
            fixed_burst_bits = EXCLUDED.fixed_burst_bits,
            note = EXCLUDED.note,
            updated_at = CURRENT_TIMESTAMP
        RETURNING *
        "#
    )
    .bind(&request.pronunciation)
    .bind(bits1)
    .bind(bits2)
    .bind(burst_bits)
    .bind(request.note.as_deref().unwrap_or(""))
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(convert_to_response(override_data)))
}

pub async fn update_override(
    State(pool): State<PgPool>,
    Path(pronunciation): Path<String>,
    Json(mut request): Json<CreateOverrideRequest>,
) -> Result<Json<OverrideResponse>, StatusCode> {
    // Use pronunciation from URL path
    request.pronunciation = pronunciation;
    
    // Convert features to bit flags
    let (bits1, bits2) = convert_features_to_bits(&request.features);
    let burst_bits = convert_burst_features_to_bits(&request.burst_features);

    let override_data = sqlx::query_as::<_, CardFeatureOverride>(
        r#"
        INSERT INTO wix_card_feature_override 
        (pronunciation, fixed_bits1, fixed_bits2, fixed_burst_bits, note, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        ON CONFLICT (pronunciation) DO UPDATE SET
            fixed_bits1 = EXCLUDED.fixed_bits1,
            fixed_bits2 = EXCLUDED.fixed_bits2,
            fixed_burst_bits = EXCLUDED.fixed_burst_bits,
            note = EXCLUDED.note,
            updated_at = CURRENT_TIMESTAMP
        RETURNING *
        "#
    )
    .bind(&request.pronunciation)
    .bind(bits1)
    .bind(bits2)
    .bind(burst_bits)
    .bind(request.note.as_deref().unwrap_or(""))
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(convert_to_response(override_data)))
}

pub async fn delete_override(
    State(pool): State<PgPool>,
    Path(pronunciation): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query(
        "DELETE FROM wix_card_feature_override WHERE pronunciation = $1"
    )
    .bind(&pronunciation)
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn check_consistency(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<ConsistencyCheckResult>>, StatusCode> {
    // Get all overrides with corresponding card data
    let results = sqlx::query(
        r#"
        SELECT DISTINCT
            o.pronunciation,
            o.fixed_bits1,
            o.fixed_bits2,
            o.fixed_burst_bits,
            c.feature_bits1,
            c.feature_bits2,
            c.burst_bits
        FROM wix_card_feature_override o
        INNER JOIN wix_card c ON c.pronunciation = o.pronunciation
        ORDER BY o.pronunciation
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let consistency_results: Vec<ConsistencyCheckResult> = results
        .into_iter()
        .map(|r| {
            let pronunciation: String = r.get("pronunciation");
            let fixed_bits1: i64 = r.get("fixed_bits1");
            let fixed_bits2: i64 = r.get("fixed_bits2");
            let fixed_burst_bits: i64 = r.get("fixed_burst_bits");
            let feature_bits1: i64 = r.get("feature_bits1");
            let feature_bits2: i64 = r.get("feature_bits2");
            let burst_bits: i64 = r.get("burst_bits");

            let is_consistent = 
                fixed_bits1 == feature_bits1 &&
                fixed_bits2 == feature_bits2 &&
                fixed_burst_bits == burst_bits;

            ConsistencyCheckResult {
                pronunciation,
                is_consistent,
                rule_based_features: convert_bits_to_features(feature_bits1 as u64, feature_bits2 as u64),
                override_features: convert_bits_to_features(fixed_bits1 as u64, fixed_bits2 as u64),
                rule_based_burst_features: convert_burst_bits_to_features(burst_bits as u64),
                override_burst_features: convert_burst_bits_to_features(fixed_burst_bits as u64),
            }
        })
        .collect();

    Ok(Json(consistency_results))
}

// Helper functions using feature crate dynamically
fn convert_features_to_bits(features: &[String]) -> (i64, i64) {
    let feature_set: HashSet<CardFeature> = features
        .iter()
        .filter_map(|f| {
            // Try to match by display string (Japanese label)
            CardFeature::create_vec()
                .into_iter()
                .find(|feature| format!("{}", feature) == *f)
        })
        .collect();
    
    feature_set.to_bits()
}

fn convert_burst_features_to_bits(features: &[String]) -> i64 {
    let feature_set: HashSet<BurstFeature> = features
        .iter()
        .filter_map(|f| {
            // Try to match by display string (Japanese label)
            BurstFeature::create_vec()
                .into_iter()
                .find(|feature| format!("{}", feature) == *f)
        })
        .collect();
    
    feature_set.to_burst_bits()
}

fn convert_bits_to_features(bits1: u64, bits2: u64) -> Vec<String> {
    let all_features = CardFeature::create_vec();
    let mut features = Vec::new();
    
    for feature in all_features {
        let (shift1, shift2) = feature.to_bit_shifts();
        let bit_is_set = if shift1 != 0 {
            // Check bits1
            bits1 & (1u64 << shift1) != 0
        } else if shift2 != 0 {
            // Check bits2  
            bits2 & (1u64 << shift2) != 0
        } else {
            false
        };
        
        if bit_is_set {
            features.push(format!("{}", feature));
        }
    }
    
    features
}

fn convert_burst_bits_to_features(bits: u64) -> Vec<String> {
    let all_features = BurstFeature::create_vec();
    let mut features = Vec::new();
    
    for feature in all_features {
        let shift = feature.to_bit_shift();
        if bits & (1u64 << shift) != 0 {
            features.push(format!("{}", feature));
        }
    }
    
    features
}

fn convert_to_response(override_data: CardFeatureOverride) -> OverrideResponse {
    OverrideResponse {
        pronunciation: override_data.pronunciation,
        features: convert_bits_to_features(
            override_data.fixed_bits1 as u64, 
            override_data.fixed_bits2 as u64
        ),
        burst_features: convert_burst_bits_to_features(
            override_data.fixed_burst_bits as u64
        ),
        created_at: override_data.created_at,
        updated_at: override_data.updated_at,
        note: override_data.note,
    }
}