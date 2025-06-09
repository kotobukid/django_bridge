use analyzer::card_analyzer::SimpleRawCardAnalyzer;
use analyzer::raw_card_analyzer::RawCardAnalyzer;
use models::r#gen::django_models::RawCardDb;
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing card type detection in SimpleRawCardAnalyzer");
    
    // テスト用のHTMLサンプル（実際のHTMLから抜粋）
    let test_cases = vec![
        ("Test Lrig", "<td>種別</td><td>ルリグ</td>"),
        ("Test Arts", "<td>種別</td><td>アーツ</td>"),
        ("Test Signi", "<td>種別</td><td>シグニ</td>"),
        ("Test Spell", "<td>種別</td><td>スペル</td>"),
        ("Test LrigAssist", "<td>種別</td><td>アシストルリグ</td>"),
        ("Test Unknown", "<td>種別</td><td>その他</td>"),
    ];
    
    let analyzer = SimpleRawCardAnalyzer::new();
    
    for (name, html) in test_cases {
        // モックのRawCardDbを作成
        let raw_card = RawCardDb {
            id: 1,
            card_number: "TEST001".to_string(),
            name: name.to_string(),
            raw_html: html.to_string(),
            skill_text: "テストスキル".to_string(),
            life_burst_text: "".to_string(),
            source_url: "https://test.com".to_string(),
            scraped_at: Utc::now(),
            last_analyzed_at: None,
            is_analyzed: false,
            analysis_error: "".to_string(),
        };
        
        // 解析実行
        match analyzer.analyze(&raw_card).await {
            Ok(create_card) => {
                let type_name = match create_card.card_type {
                    0 => "Unknown",
                    1 => "Lrig",
                    2 => "LrigAssist", 
                    3 => "Arts",
                    4 => "Key",
                    5 => "Signi",
                    6 => "Spell",
                    _ => "Other",
                };
                println!("{}: card_type = {} ({})", name, create_card.card_type, type_name);
                println!("  HTML: {}", html);
                println!();
            }
            Err(e) => {
                println!("{}: Error - {:?}", name, e);
            }
        }
    }
    
    Ok(())
}