use datapack::parse_feature_name;
use feature::labels::FEATURE_LABELS;

fn main() {
    // 正常ケース
    let test_cases = vec!["ハンデス(強)", "ダブルクラッシュ", "アサシン", "Sランサー"];

    println!("Testing feature label parsing:");
    for name in test_cases {
        match parse_feature_name(name) {
            Ok(feature) => println!("✓ '{}' -> {:?}", name, feature),
            Err(e) => println!("✗ '{}' -> Error: {}", name, e),
        }
    }

    // エラーケース
    println!("\nTesting error case:");
    match parse_feature_name("ランダムハンデス") {
        Ok(feature) => println!("✗ 'ランダムハンデス' should fail but got: {:?}", feature),
        Err(e) => println!("✓ 'ランダムハンデス' -> Error: {}", e),
    }

    // すべてのラベルを表示
    println!("\nAll available feature labels:");
    let mut labels: Vec<_> = FEATURE_LABELS.keys().copied().collect();
    labels.sort();
    for label in labels {
        println!("  - {}", label);
    }
}
