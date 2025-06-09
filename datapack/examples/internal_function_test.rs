use datapack::gen::cards;
use datapack::CardExport;

fn main() {
    println!("=== 内部関数とデータ構造テスト ===");

    // テスト1: CardExportの作成テスト
    println!("\n1. CardExport構造体テスト");

    if cards::CARD_LIST.len() > 0 {
        let first_card = &cards::CARD_LIST[0];
        let card_export = CardExport::from(first_card);

        println!("最初のカード:");
        println!("  ID: {}", card_export.id());
        println!("  名前: {}", card_export.name());
        println!("  コード: {}", card_export.code());
        println!("  feature_bits1: {}", card_export.feature_bits1());
        println!("  feature_bits2: {}", card_export.feature_bits2());
    }

    // テスト2: fetch_by_combined_bitsロジックを手動実装
    println!("\n2. AND条件フィルターロジックテスト");

    let assassin_bit = 1_i64 << 6; // アサシン
    let slancer_bit = 1_i64 << 12; // Sランサー
    let bit1 = assassin_bit | slancer_bit; // 組み合わせ
    let bit2 = 0_i64;

    println!("検索条件:");
    println!("  アサシンビット: {}", assassin_bit);
    println!("  Sランサービット: {}", slancer_bit);
    println!("  組み合わせbit1: {}", bit1);
    println!("  組み合わせbit2: {}", bit2);

    // AND条件での手動フィルタリング
    let and_cards: Vec<CardExport> = cards::CARD_LIST
        .iter()
        .filter(|c| {
            let feature_bits1 = c.20;
            let feature_bits2 = c.21;

            // AND条件: 指定されたビットが全て立っている
            (bit1 == 0 || (feature_bits1 & bit1) == bit1)
                && (bit2 == 0 || (feature_bits2 & bit2) == bit2)
        })
        .map(|c| CardExport::from(c))
        .collect();

    println!("\nAND条件結果:");
    println!("  該当カード数: {}", and_cards.len());

    if and_cards.len() > 0 {
        println!("  該当カード一覧:");
        for (i, card) in and_cards.iter().enumerate() {
            println!(
                "    {}. {} (bits1={}, bits2={})",
                i + 1,
                card.name(),
                card.feature_bits1(),
                card.feature_bits2()
            );
        }
    }

    // テスト3: OR条件での手動フィルタリング
    println!("\n3. OR条件フィルターロジックテスト");

    let or_cards: Vec<CardExport> = cards::CARD_LIST
        .iter()
        .filter(|c| {
            let feature_bits1 = c.20;
            let feature_bits2 = c.21;

            // OR条件: いずれかのビットが立っている
            (bit1 > 0 && (feature_bits1 & bit1) != 0) || (bit2 > 0 && (feature_bits2 & bit2) != 0)
        })
        .map(|c| CardExport::from(c))
        .collect();

    println!("OR条件結果:");
    println!("  該当カード数: {}", or_cards.len());

    // テスト4: 個別ビットでのフィルタリング比較
    println!("\n4. 個別ビット比較");

    let assassin_only: Vec<_> = cards::CARD_LIST
        .iter()
        .filter(|c| (c.20 & assassin_bit) != 0)
        .collect();

    let slancer_only: Vec<_> = cards::CARD_LIST
        .iter()
        .filter(|c| (c.20 & slancer_bit) != 0)
        .collect();

    println!("  アサシンのみ: {}件", assassin_only.len());
    println!("  Sランサーのみ: {}件", slancer_only.len());

    // テスト5: fetch_by_features_andロジックを手動実装
    println!("\n5. fetch_by_features_and ロジックテスト");

    let features = [6, 0, 12, 0]; // [アサシンshift1, アサシンshift2, Sランサーshift1, Sランサーshift2]

    let features_and_cards: Vec<CardExport> = cards::CARD_LIST
        .iter()
        .filter(|c| {
            let feature_bits1 = c.20;
            let feature_bits2 = c.21;

            // 全てのフィーチャーを満たすかチェック（AND条件）
            for i in (0..features.len()).step_by(2) {
                if i + 1 >= features.len() {
                    break;
                }

                let shift1 = features[i];
                let shift2 = features[i + 1];

                // 両方とも-1の場合はスキップ
                if shift1 < 0 && shift2 < 0 {
                    continue;
                }

                let bit1 = if shift1 >= 0 { 1_i64 << shift1 } else { 0 };
                let bit2 = if shift2 >= 0 { 1_i64 << shift2 } else { 0 };

                let has_feature = if bit1 > 0 && bit2 > 0 {
                    (feature_bits1 & bit1) != 0 && (feature_bits2 & bit2) != 0
                } else if bit1 > 0 {
                    (feature_bits1 & bit1) != 0
                } else if bit2 > 0 {
                    (feature_bits2 & bit2) != 0
                } else {
                    false
                };

                if !has_feature {
                    return false;
                }
            }

            true
        })
        .map(|c| CardExport::from(c))
        .collect();

    println!("fetch_by_features_and ロジック結果:");
    println!("  該当カード数: {}", features_and_cards.len());

    if features_and_cards.len() > 0 {
        println!("  該当カード一覧:");
        for (i, card) in features_and_cards.iter().enumerate() {
            println!(
                "    {}. {} (bits1={}, bits2={})",
                i + 1,
                card.name(),
                card.feature_bits1(),
                card.feature_bits2()
            );
        }
    }

    println!("\n=== 内部関数テスト完了 ===");
}
