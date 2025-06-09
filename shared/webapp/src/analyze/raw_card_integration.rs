use crate::analyze::wixoss::Card;
use analyzer::raw_card_analyzer::{AnalysisError, RawCardAnalyzer};
use models::card::CreateCard;
use models::gen::django_models::RawCardDb;

/// WebApp integrated RawCardAnalyzer that uses the full HTML parsing capabilities
pub struct WebAppRawCardAnalyzer;

impl WebAppRawCardAnalyzer {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl RawCardAnalyzer for WebAppRawCardAnalyzer {
    async fn analyze(&self, raw_card: &RawCardDb) -> Result<CreateCard, AnalysisError> {
        // Parse the HTML to get a Card struct
        let card = match Card::card_from_html(&raw_card.raw_html) {
            Some(card) => card,
            None => {
                return Err(AnalysisError::new(
                    "Failed to parse card HTML".to_string(),
                    raw_card.id,
                ));
            }
        };

        // Convert Card to CreateCard
        let mut create_card: CreateCard = card.into();

        // Override some fields with data from RawCardDb
        // TODO: Use raw_card.product_id when field is available
        create_card.url = Some(raw_card.source_url.clone());

        // Ensure the code matches what was scraped
        if create_card.code != raw_card.card_number {
            return Err(AnalysisError::new(
                format!(
                    "Card number mismatch: expected {}, got {}",
                    raw_card.card_number, create_card.code
                ),
                raw_card.id,
            ));
        }

        Ok(create_card)
    }
}

/// Utility function to analyze a single raw card and save to database
/// NOTE: This function is currently disabled due to cyclic dependency removal
pub async fn analyze_and_save_card(
    raw_card: &RawCardDb,
    pool: &sqlx::PgPool,
) -> Result<i64, Box<dyn std::error::Error>> {
    // let analyzer = WebAppRawCardAnalyzer::new();

    // Analyze the raw card using Card::card_from_html directly
    let card = match Card::card_from_html(&raw_card.raw_html) {
        Some(card) => card,
        None => {
            return Err(format!("Failed to parse card HTML for card {}", raw_card.id).into());
        }
    };
    
    // Convert Card to CreateCard
    let mut create_card: CreateCard = card.into();
    create_card.url = Some(raw_card.source_url.clone());
    
    // Ensure the code matches what was scraped
    if create_card.code != raw_card.card_number {
        return Err(format!(
            "Card number mismatch: expected {}, got {}",
            raw_card.card_number, create_card.code
        ).into());
    }

    // Save to database using the repository
    use crate::repositories::CardRepository;
    use std::sync::Arc;
    let card_repo = CardRepository::new(Arc::new(pool.clone()));
    let card_result = card_repo.upsert(create_card).await?;
    let card_id = card_result.id;

    // Update the raw card to mark it as analyzed
    sqlx::query(
        "UPDATE wix_rawcard SET is_analyzed = true, last_analyzed_at = NOW() WHERE id = $1",
    )
    .bind(raw_card.id)
    .execute(pool)
    .await?;

    Ok(card_id)
}

/// Batch analyze multiple raw cards
pub async fn analyze_raw_cards_batch(
    raw_cards: Vec<RawCardDb>,
    pool: &sqlx::PgPool,
) -> Vec<Result<i64, Box<dyn std::error::Error>>> {
    // let analyzer = WebAppRawCardAnalyzer::new();
    let mut results = Vec::new();

    for raw_card in raw_cards {
        let result: Result<i64, Box<dyn std::error::Error>> = async {
            // Analyze using Card::card_from_html directly
            let card = match Card::card_from_html(&raw_card.raw_html) {
                Some(card) => card,
                None => {
                    return Err(format!("Failed to parse card HTML for card {}", raw_card.id).into());
                }
            };
            
            // Convert Card to CreateCard
            let mut create_card: CreateCard = card.into();
            create_card.url = Some(raw_card.source_url.clone());
            
            // Ensure the code matches what was scraped
            if create_card.code != raw_card.card_number {
                return Err(format!(
                    "Card number mismatch: expected {}, got {}",
                    raw_card.card_number, create_card.code
                ).into());
            }

            use crate::repositories::CardRepository;
            use std::sync::Arc;
            let card_repo = CardRepository::new(Arc::new(pool.clone()));
            let card_result = card_repo.upsert(create_card).await?;
            let card_id = card_result.id;

            // Mark as analyzed
            sqlx::query(
                "UPDATE wix_rawcard SET is_analyzed = true, last_analyzed_at = NOW() WHERE id = $1",
            )
            .bind(raw_card.id)
            .execute(pool)
            .await?;

            Ok(card_id)
        }
        .await;

        // If there was an error, update the raw card with the error message
        if let Err(ref e) = result {
            let _ = sqlx::query("UPDATE wix_rawcard SET analysis_error = $1 WHERE id = $2")
                .bind(e.to_string())
                .bind(raw_card.id)
                .execute(pool)
                .await;
        }

        results.push(result);
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_webapp_analyzer_creation() {
        let _analyzer = WebAppRawCardAnalyzer::new();
        // Basic creation test
        assert!(true);
    }
}
