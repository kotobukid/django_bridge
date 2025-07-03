use datapack::filter::*;

fn main() {
    println!("=== 共通フィルター関数テスト（WASM関数相当） ===");

    // テスト1: get_by_id は直接関数として残っているのでスキップ
    println!("\n1. get_by_id テスト - スキップ（WASM固有関数）");

    // テスト2: filter_by_f_bits（基本的なケース）
    println!("\n2. filter_by_f_bits テスト");
    println!("全カード取得（0, 0）:");
    let all_cards = filter_by_f_bits(0, 0);
    println!("結果: {}件のカード", all_cards.len());

    // テスト3: filter_by_f_shifts（アサシン）
    println!("\n3. filter_by_f_shifts テスト - アサシン");
    let assassin_cards = filter_by_f_shifts(6, 0); // アサシンのshift値
    println!("アサシンカード: {}件", assassin_cards.len());

    // テスト4: filter_by_f_shifts（Sランサー）
    println!("\n4. filter_by_f_shifts テスト - Sランサー");
    let slancer_cards = filter_by_f_shifts(12, 0); // Sランサーのshift値
    println!("Sランサーカード: {}件", slancer_cards.len());

    // テスト5: filter_by_features_and
    println!("\n5. filter_by_features_and テスト");
    let features_array = [6, 0, 12, 0]; // アサシン(6,0) + Sランサー(12,0)
    println!("入力配列: {features_array:?}");
    let and_result = filter_by_features_and(&features_array);
    println!("AND結果: {}件", and_result.len());

    if !and_result.is_empty() {
        println!("該当カード:");
        for (i, card) in and_result.iter().enumerate() {
            println!(
                "  {}. {} (bits1={}, bits2={})",
                i + 1,
                card.name(),
                card.feature_bits1(),
                card.feature_bits2()
            );
        }
    }

    // テスト6: filter_by_combined_bits（AND条件）
    println!("\n6. filter_by_combined_bits テスト - AND条件");
    let assassin_bit = 1_i64 << 6; // アサシン
    let slancer_bit = 1_i64 << 12; // Sランサー
    let combined_bits = assassin_bit | slancer_bit; // 組み合わせ

    println!("アサシンビット: {assassin_bit}");
    println!("Sランサービット: {slancer_bit}");
    println!("組み合わせビット: {combined_bits}");

    let and_cards = filter_by_combined_bits(combined_bits, 0, "and");
    println!("AND条件結果: {}件", and_cards.len());

    if !and_cards.is_empty() {
        println!("該当カード:");
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

    // テスト7: filter_by_combined_bits（OR条件）
    println!("\n7. filter_by_combined_bits テスト - OR条件");
    let or_cards = filter_by_combined_bits(combined_bits, 0, "or");
    println!("OR条件結果: {}件", or_cards.len());

    // テスト8: 無効な条件での動作確認
    println!("\n8. 境界値テスト");
    let edge_case1 = filter_by_combined_bits(0, 0, "and");
    println!("(0,0) AND結果: {}件", edge_case1.len());

    let edge_case2 = filter_by_combined_bits(-1, -1, "invalid");
    println!("(-1,-1) invalid結果: {}件", edge_case2.len());

    // テスト9: 結果の一致確認
    println!("\n9. 結果の一致確認");
    println!("filter_by_features_and と filter_by_combined_bits (AND):");
    println!("  features_and: {}件", and_result.len());
    println!("  combined_bits: {}件", and_cards.len());
    println!("  一致: {}", and_result.len() == and_cards.len());

    // テスト10: パフォーマンステスト
    println!("\n10. パフォーマンステスト");
    let start = std::time::Instant::now();
    for _ in 0..100 {
        let _cards = filter_by_combined_bits(combined_bits, 0, "and");
    }
    let duration = start.elapsed();
    println!("100回実行時間: {duration:?}");

    println!("\n=== 共通フィルター関数テスト完了 ===");
}
