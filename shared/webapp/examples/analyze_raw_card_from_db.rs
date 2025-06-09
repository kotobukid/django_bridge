use dotenvy;
use sqlx::PgPool;
use webapp::analyze::raw_card_integration::{WebAppRawCardAnalyzer, analyze_and_save_card};
use analyzer::raw_card_analyzer::RawCardAnalyzer;
use models::gen::django_models::RawCardDb;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenvy::dotenv().ok();
    
    // Get database URL
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    // Create database pool
    let pool = PgPool::connect(&database_url).await?;
    
    // Query for unanalyzed cards  
    let raw_cards: Vec<RawCardDb> = sqlx::query_as::<_, RawCardDb>(
        r#"
        SELECT id, card_number, name, raw_html, skill_text, life_burst_text, 
               source_url, scraped_at, last_analyzed_at, is_analyzed, analysis_error
        FROM wix_rawcard
        WHERE is_analyzed = false
        LIMIT 5
        "#
    )
    .fetch_all(&pool)
    .await?;
    
    println!("Found {} unanalyzed cards", raw_cards.len());
    
    if raw_cards.is_empty() {
        println!("No unanalyzed cards found. Creating a test card...");
        
        // Insert a test card - using regular query instead of macro
        println!("Creating test card...");
        // We'll skip the test card creation for now to avoid DB issues
        println!("Skipping test card creation for now.");
    } else {
        // Analyze the cards using WebAppRawCardAnalyzer
        let analyzer = WebAppRawCardAnalyzer::new();
        
        for raw_card in raw_cards {
            println!("\nAnalyzing card: {} - {}", raw_card.card_number, raw_card.name);
            
            match analyzer.analyze(&raw_card).await {
                Ok(create_card) => {
                    println!("  ✓ Analysis successful");
                    println!("    - Power: {}", create_card.power.unwrap_or_else(|| "N/A".to_string()));
                    println!("    - Has burst: {}", if create_card.has_burst == 1 { "Yes" } else { "No" });
                    println!("    - Feature bits: {:#b} / {:#b}", create_card.feature_bits1, create_card.feature_bits2);
                    
                    // Try to save to database
                    match analyze_and_save_card(&raw_card, &pool).await {
                        Ok(card_id) => println!("    - Saved as card ID: {}", card_id),
                        Err(e) => println!("    - Failed to save: {}", e),
                    }
                }
                Err(e) => {
                    println!("  ✗ Analysis failed: {}", e);
                }
            }
        }
    }
    
    Ok(())
}