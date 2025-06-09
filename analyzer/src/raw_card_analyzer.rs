use crate::{AnalyzeRule, CardFeatureAnalyzer};
use feature::feature::CardFeature;
use models::card::CreateCard;
use models::r#gen::django_models::RawCardDb;
use std::collections::HashSet;

/// Error type for card analysis
#[derive(Debug, Clone)]
pub struct AnalysisError {
    pub message: String,
    pub card_id: i64,
}

impl AnalysisError {
    pub fn new(message: String, card_id: i64) -> Self {
        Self { message, card_id }
    }
}

impl std::fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Analysis error for card {}: {}",
            self.card_id, self.message
        )
    }
}

impl std::error::Error for AnalysisError {}

/// Trait for analyzing raw card data and converting to CreateCard
#[async_trait::async_trait]
pub trait RawCardAnalyzer {
    async fn analyze(&self, raw_card: &RawCardDb) -> Result<CreateCard, AnalysisError>;

    /// Analyze multiple cards in batch
    async fn analyze_batch(
        &self,
        raw_cards: &[RawCardDb],
    ) -> Vec<Result<CreateCard, AnalysisError>> {
        let mut results = Vec::new();
        for raw_card in raw_cards {
            results.push(self.analyze(raw_card).await);
        }
        results
    }
}

/// Default implementation of RawCardAnalyzer that uses webapp's analyze module
pub struct DefaultRawCardAnalyzer {
    feature_analyzer: CardFeatureAnalyzer,
}

impl DefaultRawCardAnalyzer {
    pub fn new() -> Result<Self, regex::Error> {
        Ok(Self {
            feature_analyzer: CardFeatureAnalyzer::new()?,
        })
    }

    /// Analyze skill text and life burst text to detect features
    fn analyze_features(&self, skill_text: &str, life_burst_text: &str) -> (i64, i64) {
        let mut all_features = HashSet::new();

        // Analyze normal skill text
        if !skill_text.is_empty() {
            let features = self.feature_analyzer.detect(skill_text);
            all_features.extend(features);
        }

        // Analyze life burst text
        if !life_burst_text.is_empty() {
            let features = self.feature_analyzer.detect(life_burst_text);
            all_features.extend(features);
        }

        // Convert features to bit representation
        let (bits1, bits2) = features_to_bits(&all_features);
        (bits1, bits2)
    }
}

/// Convert a set of features to two 64-bit integers
fn features_to_bits(features: &HashSet<CardFeature>) -> (i64, i64) {
    let mut bits1: i64 = 0;
    let mut bits2: i64 = 0;

    for feature in features {
        let (shift1, shift2) = feature.to_bit_shifts();
        if shift1 != 0 {
            bits1 |= shift1;
        }
        if shift2 != 0 {
            bits2 |= shift2;
        }
    }

    (bits1, bits2)
}

#[async_trait::async_trait]
impl RawCardAnalyzer for DefaultRawCardAnalyzer {
    async fn analyze(&self, raw_card: &RawCardDb) -> Result<CreateCard, AnalysisError> {
        // For now, we'll need to use the existing webapp analyze module
        // This is a bridge implementation that will use the HTML parsing logic

        // Detect features from skill texts
        let (feature_bits1, feature_bits2) =
            self.analyze_features(&raw_card.skill_text, &raw_card.life_burst_text);

        // Since we can't fully parse the HTML without the webapp module,
        // we'll create a minimal CreateCard for now
        // In the future, this should call into webapp's analyze::wixoss::Card::card_from_html

        Ok(CreateCard {
            name: raw_card.name.clone(),
            code: raw_card.card_number.clone(),
            pronunciation: raw_card.name.clone(), // Default to name for now
            color: 128,                           // Default colorless (bit 7)
            power: None,
            has_burst: if raw_card.life_burst_text.is_empty() {
                2
            } else {
                1
            }, // 1=has burst, 2=no burst
            cost: None,
            level: None,
            limit: None,
            limit_ex: None,
            burst_text: Some(raw_card.life_burst_text.clone()),
            format: 7, // Default to all-star
            story: None,
            rarity: None,
            timing: None,
            card_type: 0, // Will need to be determined from HTML
            product: 0,   // TODO: Get product ID from raw_card when field is available
            url: Some(raw_card.source_url.clone()),
            skill_text: Some(raw_card.skill_text.clone()),
            feature_bits1,
            feature_bits2,
            ex1: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_features_to_bits() {
        let mut features = HashSet::new();
        features.insert(CardFeature::Draw);
        features.insert(CardFeature::Banish);

        let (bits1, bits2) = features_to_bits(&features);

        // Get the actual bit shifts for the features
        let (draw_shift1, draw_shift2) = CardFeature::Draw.to_bit_shifts();
        let (banish_shift1, banish_shift2) = CardFeature::Banish.to_bit_shifts();

        // Check that the bits are set correctly
        assert_eq!(bits1, draw_shift1 | banish_shift1);
        assert_eq!(bits2, draw_shift2 | banish_shift2);
    }

    #[tokio::test]
    async fn test_analyze_empty_card() {
        let analyzer = DefaultRawCardAnalyzer::new().unwrap();

        let raw_card = RawCardDb {
            id: 1,
            card_number: "TEST-001".to_string(),
            name: "Test Card".to_string(),
            raw_html: "<html></html>".to_string(),
            skill_text: "".to_string(),
            life_burst_text: "".to_string(),
            source_url: "https://example.com".to_string(),
            scraped_at: chrono::Utc::now(),
            last_analyzed_at: None,
            is_analyzed: false,
            analysis_error: "".to_string(),
        };

        let result = analyzer.analyze(&raw_card).await;
        assert!(result.is_ok());

        let create_card = result.unwrap();
        assert_eq!(create_card.name, "Test Card");
        assert_eq!(create_card.code, "TEST-001");
        assert_eq!(create_card.has_burst, 2); // No burst
    }
}
