use datapack::filter::*;

fn main() {
    println!("=== 共通フィルター関数テスト ===");

    // テスト1: filter_by_combined_bits (AND条件)
    println!("\n1. filter_by_combined_bits - AND条件");
    let assassin_bit = 1_i64 << 6; // アサシン
    let slancer_bit = 1_i64 << 12; // Sランサー
    let combined_bits = assassin_bit | slancer_bit;

    let and_cards = filter_by_combined_bits(combined_bits, 0, "and");
    println!("AND条件結果: {}件", and_cards.len());

    if !and_cards.is_empty() {
        for (i, card) in and_cards.iter().enumerate() {
            println!(
                "  {}. {} (bits1={}, bits2={})",
                i + 1,
                card.name(),
                card.feature_bits1(),
                card.feature_bits2()
            );
        }
    }

    // テスト2: filter_by_combined_bits (OR条件)
    println!("\n2. filter_by_combined_bits - OR条件");
    let or_cards = filter_by_combined_bits(combined_bits, 0, "or");
    println!("OR条件結果: {}件", or_cards.len());

    // テスト3: filter_by_features_and
    println!("\n3. filter_by_features_and");
    let features = [6, 0, 12, 0]; // アサシン + Sランサー
    let features_and_cards = filter_by_features_and(&features);
    println!("features_and結果: {}件", features_and_cards.len());

    if !features_and_cards.is_empty() {
        for (i, card) in features_and_cards.iter().enumerate() {
            println!(
                "  {}. {} (bits1={}, bits2={})",
                i + 1,
                card.name(),
                card.feature_bits1(),
                card.feature_bits2()
            );
        }
    }

    // テスト4: filter_by_f_shifts (個別)
    println!("\n4. filter_by_f_shifts - 個別テスト");
    let assassin_shifts = filter_by_f_shifts(6, 0);
    let slancer_shifts = filter_by_f_shifts(12, 0);

    println!("アサシンのみ: {}件", assassin_shifts.len());
    println!("Sランサーのみ: {}件", slancer_shifts.len());

    // テスト5: filter_by_f_bits
    println!("\n5. filter_by_f_bits");
    let f_bits_cards = filter_by_f_bits(combined_bits, 0);
    println!("f_bits結果: {}件", f_bits_cards.len());

    // テスト6: 結果の一致確認
    println!("\n6. 結果の一致確認");
    println!(
        "AND条件とfeatures_and: {} == {} -> {}",
        and_cards.len(),
        features_and_cards.len(),
        and_cards.len() == features_and_cards.len()
    );

    // 実際のカード名も比較
    if and_cards.len() == features_and_cards.len() && !and_cards.is_empty() {
        let and_names: Vec<_> = and_cards.iter().map(|c| c.name()).collect();
        let features_names: Vec<_> = features_and_cards.iter().map(|c| c.name()).collect();
        println!("カード名も一致: {}", and_names == features_names);
    }

    println!("\n=== 共通関数テスト完了 ===");
}
