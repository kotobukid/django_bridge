use analyzer::raw_card_analyzer::RawCardAnalyzer;
use models::r#gen::django_models::RawCardDb;
use sqlx::postgres::PgPoolOptions;
use std::env;
use webapp::analyze::raw_card_integration::WebAppRawCardAnalyzer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 環境変数からデータベースURLを取得
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:password@localhost/wixossdb".to_string());

    println!("Connecting to database: {}", database_url);

    // データベース接続プールを作成
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("Connected to database successfully");

    // RawCardを1件取得してテスト
    let raw_card: RawCardDb = sqlx::query_as("SELECT * FROM wix_rawcard LIMIT 1")
        .fetch_one(&pool)
        .await?;

    println!(
        "Retrieved RawCard: ID={}, Name={}",
        raw_card.id, raw_card.name
    );
    println!(
        "HTML snippet: {}",
        &raw_card.raw_html[..200.min(raw_card.raw_html.len())]
    );

    // WebAppアナライザーを作成
    let analyzer = WebAppRawCardAnalyzer::new();

    // 解析実行
    println!("\nAnalyzing card...");
    match analyzer.analyze(&raw_card).await {
        Ok(create_card) => {
            println!("Analysis successful!");
            println!("Card name: {}", create_card.name);
            println!("Card type: {} (DB ID)", create_card.card_type);
            println!("Color: {}", create_card.color);
            println!("Has burst: {}", create_card.has_burst);

            // card_typeが0でない場合は成功
            if create_card.card_type != 0 {
                println!("✅ Card type detection successful! (was previously 0)");
            } else {
                println!("⚠️  Card type is still 0 (Unknown)");
            }
        }
        Err(e) => {
            println!("Analysis failed: {:?}", e);
        }
    }

    // より多くのサンプルでテスト
    println!("\n--- Testing multiple cards ---");
    let raw_cards: Vec<RawCardDb> = sqlx::query_as("SELECT * FROM wix_rawcard LIMIT 10")
        .fetch_all(&pool)
        .await?;

    let mut type_counts = std::collections::HashMap::new();

    for raw_card in raw_cards {
        if let Ok(create_card) = analyzer.analyze(&raw_card).await {
            *type_counts.entry(create_card.card_type).or_insert(0) += 1;
        }
    }

    println!("Card type distribution in sample:");
    for (card_type, count) in type_counts {
        let type_name = match card_type {
            0 => "Unknown",
            1 => "Lrig",
            2 => "LrigAssist",
            3 => "Arts",
            4 => "Key",
            5 => "Signi",
            6 => "Spell",
            7 => "Resona",
            8 => "SigniCraft",
            9 => "ArtsCraft",
            10 => "ResonaCraft",
            11 => "SpellCraft",
            12 => "Piece",
            13 => "PieceRelay",
            14 => "PieceCraft",
            15 => "Token",
            _ => "Other",
        };
        println!("  {}: {} cards", type_name, count);
    }

    Ok(())
}
