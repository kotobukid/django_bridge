use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RulePattern {
    pub id: i32,
    pub keyword: String,
    pub pattern: String,
    pub features: serde_json::Value,
    pub positive_examples: serde_json::Value,
    pub negative_examples: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

impl RulePattern {
    pub fn features_as_vec(&self) -> Vec<String> {
        self.features
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn positive_examples_as_vec(&self) -> Vec<String> {
        self.positive_examples
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn negative_examples_as_vec(&self) -> Vec<String> {
        self.negative_examples
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default()
    }
}
