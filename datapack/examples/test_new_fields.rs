use datapack::get_all_cards;

fn main() {
    let cards = get_all_cards().unwrap();
    println!("Testing new fields in CardExport:");

    // レベルやパワーが設定されているカードを探す
    let cards_with_level: Vec<_> = cards
        .iter()
        .filter(|c| !c.level().is_empty())
        .take(3)
        .collect();

    let cards_with_power: Vec<_> = cards
        .iter()
        .filter(|c| !c.power().is_empty())
        .take(3)
        .collect();

    let cards_with_limit: Vec<_> = cards
        .iter()
        .filter(|c| !c.limit().is_empty())
        .take(3)
        .collect();

    println!("\n=== Cards with Level ===");
    for card in cards_with_level {
        println!(
            "Name: {}, Level: {}, Type: {}",
            card.name(),
            card.level(),
            card.card_type()
        );
    }

    println!("\n=== Cards with Power ===");
    for card in cards_with_power {
        println!(
            "Name: {}, Power: {}, Type: {}",
            card.name(),
            card.power(),
            card.card_type()
        );
    }

    println!("\n=== Cards with Limit ===");
    for card in cards_with_limit {
        println!(
            "Name: {}, Limit: {}, Type: {}",
            card.name(),
            card.limit(),
            card.card_type()
        );
    }

    // 統計
    let level_count = cards.iter().filter(|c| !c.level().is_empty()).count();
    let power_count = cards.iter().filter(|c| !c.power().is_empty()).count();
    let limit_count = cards.iter().filter(|c| !c.limit().is_empty()).count();

    println!("\n=== Statistics ===");
    println!("Total cards: {}", cards.len());
    println!("Cards with level: {level_count}");
    println!("Cards with power: {power_count}");
    println!("Cards with limit: {limit_count}");
}
