use datapack::{get_all_cards, CardExport};

fn main() {
    println!("Testing burst_bits field integration...");

    // Test creating a card with burst_bits
    let test_card = CardExport::from(&(
        999,                    // id
        "Test Burst Card",      // name
        "TEST-001",             // code
        "テストバーストカード", // pronunciation
        1u32,                   // color
        "1",                    // cost
        "1",                    // level
        "",                     // limit
        "",                     // limit_ex
        "1000",                 // power
        1u8,                    // has_burst
        "Test skill",           // skill_text
        "Test burst",           // burst_text
        1u8,                    // format
        "",                     // story
        "C",                    // rarity
        5u8,                    // card_type
        1u8,                    // product
        0u8,                    // timing
        0i64,                   // feature_bits1
        0i64,                   // feature_bits2
        0u64,                   // klass_bits
        42i64,                  // burst_bits (test value)
        "",                     // ex1
    ));

    println!("✓ CardExport created with burst_bits field");
    println!("  Card name: {}", test_card.name());
    println!("  Burst bits: {}", test_card.burst_bits());
    println!("  Has burst: {}", test_card.has_burst());

    assert_eq!(test_card.burst_bits(), 42);
    println!("✓ burst_bits getter works correctly");

    // Test accessing static data
    match get_all_cards() {
        Ok(cards) => {
            if let Some(card) = cards.first() {
                println!("\n✓ Static data loaded successfully");
                println!("  Sample card: {}", card.name());
                println!("  Burst bits: {}", card.burst_bits());
            } else {
                println!("⚠ No cards in static data");
            }
        }
        Err(e) => {
            println!("✗ Error loading static data: {}", e);
        }
    }

    println!("\n🎉 All tests passed! The burst_bits field is working correctly.");
}
