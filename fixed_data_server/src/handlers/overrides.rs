use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::{PgPool, Row};
use crate::models::{CardFeatureOverride, CreateOverrideRequest, OverrideResponse, ConsistencyCheckResult};
use feature::feature::{CardFeature, BurstFeature};
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
    // Convert feature names to bits
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
    .bind(bits1 as i64)
    .bind(bits2 as i64)
    .bind(burst_bits as i64)
    .bind(&request.note)
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

// Helper functions
fn convert_features_to_bits(features: &[String]) -> (u64, u64) {
    let mut bits1 = 0i64;
    let mut bits2 = 0i64;

    // Create all features and find matches by display name
    let all_features = CardFeature::create_vec();
    
    for feature_name in features {
        for feature in &all_features {
            if feature.to_string() == *feature_name {
                let (shift1, shift2) = feature.to_bit_shifts();
                bits1 |= 1_i64 << shift1;
                bits2 |= 1_i64 << shift2;
                break;
            }
        }
    }

    (bits1 as u64, bits2 as u64)
}

fn convert_burst_features_to_bits(features: &[String]) -> u64 {
    let mut bits = 0i64;

    // Create all burst features and find matches by display name
    let all_features = BurstFeature::create_vec();
    for feature_name in features {
        for feature in &all_features {
            if feature.to_string() == *feature_name {
                let shift = feature.to_bit_shift();
                bits |= 1_i64 << shift;
                break;
            }
        }
    }

    bits as u64
}

fn convert_bits_to_features(bits1: u64, bits2: u64) -> Vec<String> {
    let mut features = Vec::new();
    let bits1 = bits1 as i64;
    let bits2 = bits2 as i64;

    // Check each feature against the bits
    let all_features = CardFeature::create_vec();
    for feature in all_features {
        let (shift1, shift2) = feature.to_bit_shifts();
        let has_feature = if shift2 == 0 {
            (bits1 & (1_i64 << shift1)) != 0
        } else {
            (bits2 & (1_i64 << shift2)) != 0
        };
        
        if has_feature {
            features.push(feature.to_string());
        }
    }

    features
}

fn convert_burst_bits_to_features(bits: u64) -> Vec<String> {
    let mut features = Vec::new();
    let bits = bits as i64;

    // Check each burst feature against the bits
    let all_features = BurstFeature::create_vec();
    for feature in all_features {
        let shift = feature.to_bit_shift();
        if (bits & (1_i64 << shift)) != 0 {
            features.push(feature.to_string());
        }
    }

    features
}

fn convert_to_response(override_data: CardFeatureOverride) -> OverrideResponse {
    OverrideResponse {
        pronunciation: override_data.pronunciation,
        features: convert_bits_to_features(override_data.fixed_bits1 as u64, override_data.fixed_bits2 as u64),
        burst_features: convert_burst_bits_to_features(override_data.fixed_burst_bits as u64),
        created_at: override_data.created_at,
        updated_at: override_data.updated_at,
        note: override_data.note,
    }
}