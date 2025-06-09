use datapack::{get_all_cards, CardType};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Debug: Investigating card type values in dataset");
    
    let cards = get_all_cards()?;
    println!("Total cards: {}", cards.len());
    
    // カード種別の分布を調査
    let mut card_type_counts: HashMap<u8, usize> = HashMap::new();
    
    // 最新のカードを表示（IDでソート）
    let mut cards_by_id = cards.clone();
    cards_by_id.sort_by_key(|c| c.id());
    
    println!("Latest 20 cards by ID:");
    for card in cards_by_id.iter().rev().take(20) {
        println!("Card: {} - card_type: {} - name: {}", 
            card.id(), card.card_type(), card.name());
    }
    
    println!("\nFirst 20 cards:");
    for card in &cards[..20.min(cards.len())] { // 最初の20件だけ調査
        let card_type = card.card_type();
        *card_type_counts.entry(card_type).or_insert(0) += 1;
        
        if card_type_counts.len() <= 10 { // 最初の10種類だけ詳細表示
            println!("Card: {} - card_type: {} - name: {}", 
                card.id(), card_type, card.name());
        }
    }
    
    println!("\nCard type distribution (first 100 cards):");
    for (card_type_u8, count) in card_type_counts.iter() {
        let card_type = CardType::from_u8(*card_type_u8);
        println!("card_type {}: {} ({} cards)", 
            card_type_u8, card_type.display_name(), count);
    }
    
    // 各CardType enumに対応するu8値をテスト
    println!("\nCardType enum mapping:");
    let all_types = vec![
        CardType::Lrig, CardType::LrigAssist, CardType::Arts, CardType::Key,
        CardType::Signi, CardType::Spell, CardType::Resona, CardType::SigniCraft,
        CardType::ArtsCraft, CardType::ResonaCraft, CardType::SpellCraft,
        CardType::Piece, CardType::PieceRelay, CardType::PieceCraft, 
        CardType::Token, CardType::Unknown
    ];
    
    for card_type in all_types {
        println!("{}: {} -> u8: {}", 
            card_type.display_name(), 
            format!("{:?}", card_type),
            card_type.to_u8());
    }
    
    Ok(())
}