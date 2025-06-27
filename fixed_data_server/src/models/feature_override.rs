use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CardFeatureOverride {
    pub pronunciation: String,
    pub fixed_bits1: i64,
    pub fixed_bits2: i64,
    pub fixed_burst_bits: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOverrideRequest {
    pub pronunciation: String,
    pub features: Vec<String>,  // CardFeature enum names
    pub burst_features: Vec<String>,  // BurstFeature enum names
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverrideResponse {
    pub pronunciation: String,
    pub features: Vec<String>,
    pub burst_features: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyCheckResult {
    pub pronunciation: String,
    pub is_consistent: bool,
    pub rule_based_features: Vec<String>,
    pub override_features: Vec<String>,
    pub rule_based_burst_features: Vec<String>,
    pub override_burst_features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportExportData {
    pub overrides: Vec<CardFeatureOverride>,
    pub exported_at: DateTime<Utc>,
    pub version: String,
}