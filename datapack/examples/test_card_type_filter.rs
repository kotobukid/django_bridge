use datapack::{fetch_by_colors_features_and_card_types_native, get_all_cards, CardExport, CardType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing card type filtering functionality");
    
    let cards = get_all_cards()?;
    println!("Total cards: {}", cards.len());
    
    // 現在のデータでUnknownが0件かテスト
    let unknown_filter = vec![CardType::Unknown];
    let unknown_results = fetch_by_colors_features_and_card_types_native(
        &cards, 
        0, // 色フィルタなし
        &vec![], // 機能フィルタなし
        &unknown_filter
    );
    println!("Unknown card type filter results: {}", unknown_results.len());
    
    // 全部のカード種別でテスト
    let all_types = vec![
        CardType::Lrig, CardType::LrigAssist, CardType::Arts, CardType::Key,
        CardType::Signi, CardType::Spell, CardType::Resona, CardType::SigniCraft,
        CardType::ArtsCraft, CardType::ResonaCraft, CardType::SpellCraft,
        CardType::Piece, CardType::PieceRelay, CardType::PieceCraft, 
        CardType::Token, CardType::Unknown
    ];
    
    for card_type in &all_types {
        let filter = vec![card_type.clone()];
        let results = fetch_by_colors_features_and_card_types_native(
            &cards, 
            0, 
            &vec![], 
            &filter
        );
        println!("{} ({}): {} cards", 
            card_type.display_name(), 
            card_type.to_u8(), 
            results.len());
    }
    
    // Mock データでテスト
    println!("\n--- Testing with mock data ---");
    let mock_cards = vec![
        create_mock_card(1, "Test Lrig", 1),
        create_mock_card(2, "Test Arts", 3),
        create_mock_card(3, "Test Signi", 5),
        create_mock_card(4, "Test Spell", 6),
        create_mock_card(5, "Test Unknown", 0),
    ];
    
    println!("Mock cards created: {}", mock_cards.len());
    
    // Mock データでフィルタリングテスト
    for card_type in &all_types {
        let filter = vec![card_type.clone()];
        let results = fetch_by_colors_features_and_card_types_native(
            &mock_cards, 
            0, 
            &vec![], 
            &filter
        );
        println!("Mock {} ({}): {} cards", 
            card_type.display_name(), 
            card_type.to_u8(), 
            results.len());
        
        if results.len() > 0 {
            for card in &results {
                println!("  - {} (card_type: {})", card.name(), card.card_type());
            }
        }
    }
    
    Ok(())
}

fn create_mock_card(id: i32, name: &'static str, card_type: u8) -> CardExport {
    // CardExportの実際の構造に合わせてモックデータを作成
    // この部分は実際のCardExportの構造に依存する
    CardExport::from(&(
        id,                    // id
        name,                  // name
        "TEST001",             // code
        "てすと",              // pronunciation
        0u32,                  // color
        "0",                   // cost
        "1",                   // level
        "1",                   // limit
        "",                    // limit_ex
        "1000",                // power
        0u8,                   // has_burst
        "テストスキル",         // skill_text
        "",                    // burst_text
        1u8,                   // format
        "",                    // story
        "C",                   // rarity
        "https://test.com",    // url
        card_type,             // card_type
        1u8,                   // product
        0u8,                   // timing
        0i64,                  // feature_bits1
        0i64,                  // feature_bits2
        0u64,                  // klass_bits
        0i64,                  // burst_bits
        "",                    // ex1
    ))
}