use analyzer::card_analyzer::SimpleRawCardAnalyzer;
use analyzer::raw_card_analyzer::RawCardAnalyzer;
use chrono::Utc;
use models::r#gen::django_models::RawCardDb;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a test RawCardDb instance
    let raw_card = RawCardDb {
        id: 1,
        card_number: "WXDi-P01-001".to_string(),
        name: "幻装　タケノコ".to_string(),
        raw_html: r#"<div class="cardDetail">Test HTML</div>"#.to_string(),
        skill_text: "【常】：あなたのターンの間、このシグニのパワーは＋3000される。".to_string(),
        life_burst_text: "【ライフバースト】：カードを1枚引く。".to_string(),
        source_url: "https://example.com/card/WXDi-P01-001".to_string(),
        scraped_at: Utc::now(),
        last_analyzed_at: None,
        is_analyzed: false,
        analysis_error: "".to_string(),
    };

    // Create analyzer
    let analyzer = SimpleRawCardAnalyzer::new();

    // Analyze the card
    match analyzer.analyze(&raw_card).await {
        Ok(create_card) => {
            println!("Analysis successful!");
            println!("Card name: {}", create_card.name);
            println!("Card code: {}", create_card.code);
            println!("Feature bits1: {:#b}", create_card.feature_bits1);
            println!("Feature bits2: {:#b}", create_card.feature_bits2);
            println!("Skill text: {}", create_card.skill_text.unwrap_or_default());
            println!("Burst text: {}", create_card.burst_text.unwrap_or_default());
        }
        Err(e) => {
            println!("Analysis failed: {e}");
        }
    }

    Ok(())
}
