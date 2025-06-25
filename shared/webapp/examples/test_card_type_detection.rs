use webapp::analyze::wixoss::card::{detect_card_type, CardType};

fn main() {
    println!("Testing card type detection from HTML text");

    // テストケース
    let test_cases = vec![
        ("ルリグ", CardType::Lrig),
        ("アシストルリグ", CardType::LrigAssist),
        ("アーツ", CardType::Arts),
        ("キー", CardType::Key),
        ("シグニ", CardType::Signi),
        ("スペル", CardType::Spell),
        ("レゾナ", CardType::Resona),
        ("シグニクラフト", CardType::SigniCraft),
        ("アーツクラフト", CardType::ArtsCraft),
        ("シグニレゾナクラフト", CardType::ResonaCraft),
        ("スペルクラフト", CardType::SpellCraft),
        ("ピース", CardType::Piece),
        ("ピースリレー", CardType::PieceRelay),
        ("ピースクラフト", CardType::PieceCraft),
        ("コイン", CardType::Token),
        ("トークン", CardType::Token),
        ("不明なタイプ", CardType::Unknown),
    ];

    for (input, expected) in test_cases {
        let detected = detect_card_type(input);
        let db_id = detected.to_db_id();

        println!(
            "Input: '{}' => Detected: {:?} (DB ID: {}) | Expected: {:?} | Match: {}",
            input,
            detected,
            db_id,
            expected,
            detected == expected
        );
    }

    // HTMLタグ付きのテスト
    println!("\n--- Testing with HTML tags ---");
    let html_tests = vec![
        ("<span>ルリグ</span>", CardType::Lrig),
        ("ルリグ<br>", CardType::Lrig),
        ("ルリグ<br/>", CardType::Lrig),
        ("シグニ<br />", CardType::Signi),
        ("アーツ\n", CardType::Arts),
    ];

    for (input, expected) in html_tests {
        let detected = detect_card_type(input);
        let db_id = detected.to_db_id();

        println!(
            "Input: '{}' => Detected: {:?} (DB ID: {}) | Expected: {:?} | Match: {}",
            input,
            detected,
            db_id,
            expected,
            detected == expected
        );
    }
}
