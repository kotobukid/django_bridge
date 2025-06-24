use datapack::gen::cards;

fn main() {
    println!("=== フィーチャーフィルター動作テスト ===");

    // テスト1: 全カード数を確認
    println!("\n1. 全カード数の確認");
    let all_cards = cards::CARD_LIST;
    println!("総カード数: {}", all_cards.len());

    // テスト2: 単一フィーチャー（アサシン）
    println!("\n2. 単一フィーチャーテスト - アサシン");
    let assassin_shift = (6, 0); // アサシンのbit_shift
    let assassin_bit1 = 1_i64 << assassin_shift.0;
    let assassin_bit2 = 1_i64 << assassin_shift.1;

    println!("アサシン bit_shift: {:?}", assassin_shift);
    println!(
        "アサシン bits: bit1={}, bit2={}",
        assassin_bit1, assassin_bit2
    );

    let assassin_cards: Vec<&_> = all_cards
        .iter()
        .filter(|c| {
            let feature_bits1 = c.19;
            let feature_bits2 = c.20;

            // OR条件での検索
            if assassin_bit2 == 0 || assassin_bit2 == 1 {
                (feature_bits1 & assassin_bit1) != 0
            } else if assassin_bit1 == 0 || assassin_bit1 == 1 {
                (feature_bits2 & assassin_bit2) != 0
            } else {
                (feature_bits1 & assassin_bit1) == assassin_bit1
                    && (feature_bits2 & assassin_bit2) == assassin_bit2
            }
        })
        .collect();

    println!("アサシンを持つカード数: {}", assassin_cards.len());

    // サンプルカードを表示
    if assassin_cards.len() > 0 {
        println!("サンプルカード（最初の3件）:");
        for (i, card) in assassin_cards.iter().take(3).enumerate() {
            println!(
                "  {}. {}: bits1={}, bits2={}",
                i + 1,
                card.1,
                card.20,
                card.21
            );
        }
    }

    // テスト3: 単一フィーチャー（Sランサー）
    println!("\n3. 単一フィーチャーテスト - Sランサー");
    let slancer_shift = (12, 0); // Sランサーのbit_shift
    let slancer_bit1 = 1_i64 << slancer_shift.0;
    let slancer_bit2 = 1_i64 << slancer_shift.1;

    println!("Sランサー bit_shift: {:?}", slancer_shift);
    println!(
        "Sランサー bits: bit1={}, bit2={}",
        slancer_bit1, slancer_bit2
    );

    let slancer_cards: Vec<&_> = all_cards
        .iter()
        .filter(|c| {
            let feature_bits1 = c.19;
            let feature_bits2 = c.20;

            // OR条件での検索
            if slancer_bit2 == 0 || slancer_bit2 == 1 {
                (feature_bits1 & slancer_bit1) != 0
            } else if slancer_bit1 == 0 || slancer_bit1 == 1 {
                (feature_bits2 & slancer_bit2) != 0
            } else {
                (feature_bits1 & slancer_bit1) == slancer_bit1
                    && (feature_bits2 & slancer_bit2) == slancer_bit2
            }
        })
        .collect();

    println!("Sランサーを持つカード数: {}", slancer_cards.len());

    // サンプルカードを表示
    if slancer_cards.len() > 0 {
        println!("サンプルカード（最初の3件）:");
        for (i, card) in slancer_cards.iter().take(3).enumerate() {
            println!(
                "  {}. {}: bits1={}, bits2={}",
                i + 1,
                card.1,
                card.20,
                card.21
            );
        }
    }

    // テスト4: 複数フィーチャー（アサシン AND Sランサー）
    println!("\n4. 複数フィーチャーテスト - アサシン AND Sランサー");

    // 組み合わせたビットマスク
    let combined_bit1 = assassin_bit1 | slancer_bit1;
    let combined_bit2 = assassin_bit2 | slancer_bit2;

    println!(
        "組み合わせ bits: bit1={}, bit2={}",
        combined_bit1, combined_bit2
    );

    let combined_cards: Vec<&_> = all_cards
        .iter()
        .filter(|c| {
            let feature_bits1 = c.19;
            let feature_bits2 = c.20;

            // AND条件: 両方のビットが立っている
            (combined_bit1 == 0 || (feature_bits1 & combined_bit1) == combined_bit1)
                && (combined_bit2 == 0 || (feature_bits2 & combined_bit2) == combined_bit2)
        })
        .collect();

    println!(
        "アサシン AND Sランサーを持つカード数: {}",
        combined_cards.len()
    );

    // サンプルカードを表示
    if combined_cards.len() > 0 {
        println!("サンプルカード（全件）:");
        for (i, card) in combined_cards.iter().enumerate() {
            println!(
                "  {}. {}: bits1={}, bits2={}",
                i + 1,
                card.1,
                card.20,
                card.21
            );
        }
    } else {
        println!("該当カードなし");
    }

    // テスト5: OR条件での検索（参考）
    println!("\n5. 参考: OR条件 - アサシン OR Sランサー");

    let or_cards: Vec<&_> = all_cards
        .iter()
        .filter(|c| {
            let feature_bits1 = c.19;
            let feature_bits2 = c.20;

            // OR条件: どちらかのビットが立っている
            (assassin_bit1 > 0 && (feature_bits1 & assassin_bit1) != 0)
                || (slancer_bit1 > 0 && (feature_bits1 & slancer_bit1) != 0)
        })
        .collect();

    println!("アサシン OR Sランサーを持つカード数: {}", or_cards.len());

    println!("\n=== テスト完了 ===");
}
