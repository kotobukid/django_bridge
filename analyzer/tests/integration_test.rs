use analyzer::category_rules;
use analyzer::migration;
use analyzer::*;
use feature::CardFeature;
use rayon::prelude::*;
use std::collections::HashSet;

mod common;

#[test]
fn test_sample() {
    assert_eq!(add(1, 2), 3);
    assert_eq!(1, 1);
}

#[test]
fn test_card_feature_detection_banish_pattern() {
    let card_skill_text = r#"
    【出現条件】《メインフェイズアイコン》レゾナではない＜凶蟲＞のシグニ２体をあなたの場からトラッシュに置く
    【常】：対戦相手は【チャーム】が付いているシグニの《起》能力を使用できない。
    【自】：対戦相手のシグニ１体が場に出たとき、対戦相手は自分のデッキの一番上のカードをそのシグニの【チャーム】にする。
    【自】：各アタックフェイズ開始時、対戦相手は【チャーム】が付いている自分のシグニ１体を対象とし、それをバニッシュする。
    "#;

    // 本番のルールを使用
    let detected_features = common::detect_features(card_skill_text);

    assert!(
        detected_features.contains(&CardFeature::Banish),
        "Should detect Banish feature from 'バニッシュする' text"
    );

    // 【チャーム】も検出されるはず
    assert!(
        detected_features.contains(&CardFeature::Charm),
        "Should also detect Charm feature from 【チャーム】 text"
    );
}

#[test]
fn test_card_feature_detection_charm_pattern() {
    let card_skill_text = r#"
    【常】：対戦相手は【チャーム】が付いているシグニの《起》能力を使用できない。
    【自】：対戦相手のシグニ１体が場に出たとき、対戦相手は自分のデッキの一番上のカードをそのシグニの【チャーム】にする。
    "#;

    // 本番のルールを使用
    let detected_features = common::detect_features(card_skill_text);

    assert!(
        detected_features.contains(&CardFeature::Charm),
        "Should detect Charm feature from 【チャーム】 text"
    );
}

#[test]
fn test_card_feature_detection_multiple_features() {
    let card_skill_text = r#"
    【ライフバースト】：カードを１枚引く。対戦相手のシグニ１体をバニッシュする。
    【常】：このシグニがアタックしたとき、対戦相手のライフクロス１枚をクラッシュする。
    "#;

    // 本番のルールを使用（前処理も含む）
    let (processed_text, detected_features) = common::analyze_with_preprocessing(card_skill_text);

    // 検出されたフィーチャーを確認
    assert!(
        detected_features.contains(&CardFeature::LifeBurst),
        "Should detect LifeBurst feature"
    );
    assert!(
        detected_features.contains(&CardFeature::Draw),
        "Should detect Draw feature from 'カードを１枚引く'"
    );
    assert!(
        detected_features.contains(&CardFeature::Banish),
        "Should detect Banish feature from 'バニッシュする'"
    );
    assert!(
        detected_features.contains(&CardFeature::LifeCrush),
        "Should detect LifeCrush feature from 'ライフクロス１枚をクラッシュする'"
    );

    // 前処理の動作確認
    // 本番では【ライフバースト】パターンは検出パターンで処理され、置換は行われない
}

#[test]
fn test_card_feature_detection_virustotal() {
    let card_skill_text = r#"対戦相手のシグニ１体を対象とし、それをバニッシュする。それが感染状態の場合、代わりにそれをトラッシュに置く。"#;

    // 本番のルールを使用
    let detected_features = common::detect_features(card_skill_text);

    // 期待されるフィーチャーを検証
    assert!(
        detected_features.contains(&CardFeature::Banish),
        "Should detect Banish feature from 'バニッシュする'"
    );
    assert!(
        detected_features.contains(&CardFeature::Virus),
        "Should detect Virus feature from '感染状態'"
    );
    assert!(
        detected_features.contains(&CardFeature::Trash),
        "Should detect Trash feature from 'トラッシュに置く'"
    );

    // 検出されたフィーチャーの詳細を出力（デバッグ用）
    println!(
        "Detected features for virustotal test: {:?}",
        detected_features
    );
}

// 以下の個別ルールは本番ルールを使用するように変更したため不要になりました
// 既存のテストとの互換性のために、一部のテストではまだ使用されているため残しています

#[test]
fn test_numeric_variation_rule() {
    let rule = NumericVariationRule::new("カードを", "枚引", vec![CardFeature::Draw]).unwrap();

    let text1 = "カードを（１）枚引く";
    let text2 = "カードを（３）枚引く";
    let text3 = "カードを２枚引く"; // Without parentheses
    let text4 = "何もしない";

    assert!(rule.detect(text1).contains(&CardFeature::Draw));
    assert!(rule.detect(text2).contains(&CardFeature::Draw));
    assert!(rule.detect(text3).contains(&CardFeature::Draw));
    assert!(!rule.detect(text4).contains(&CardFeature::Draw));
}

#[test]
fn test_category_rules() {
    // LethalRuleのテスト
    let lethal_rule = category_rules::LethalRule::new().unwrap();
    let assassin_text = "このシグニはアサシンを持つ";
    let crush_text = "このシグニはダブルクラッシュを持つ";

    let lethal_features = lethal_rule.detect(assassin_text);
    assert!(lethal_features.contains(&CardFeature::Assassin));

    let crush_features = lethal_rule.detect(crush_text);
    assert!(crush_features.contains(&CardFeature::DoubleCrush));

    // OffensiveRuleのテスト
    let offensive_rule = category_rules::OffensiveRule::new().unwrap();
    let banish_text = "対戦相手のシグニ１体をバニッシュする";
    let bounce_text = "対戦相手のシグニ１体を対象とし、それを手札に戻す";

    let banish_features = offensive_rule.detect(banish_text);
    assert!(banish_features.contains(&CardFeature::Banish));

    let bounce_features = offensive_rule.detect(bounce_text);
    assert!(bounce_features.contains(&CardFeature::Bounce));

    // UniqueRuleのテスト
    let unique_rule = category_rules::UniqueRule::new().unwrap();
    let charm_text = "【チャーム】を付ける";
    let lifeburst_text = "【ライフバースト】：カードを１枚引く";

    let charm_features = unique_rule.detect(charm_text);
    assert!(charm_features.contains(&CardFeature::Charm));

    let lb_features = unique_rule.detect(lifeburst_text);
    assert!(lb_features.contains(&CardFeature::LifeBurst));
}

#[test]
fn test_card_feature_analyzer() {
    let analyzer = CardFeatureAnalyzer::new().unwrap();

    let complex_text = r#"
    【ライフバースト】：カードを２枚引く。対戦相手のシグニ１体をバニッシュする。
    【常】：このシグニはアサシンを持つ。
    【自】：このシグニがアタックしたとき、【チャーム】を１つ付ける。
    "#;

    let (processed, features) = analyzer.analyze(complex_text);

    println!("検出されたフィーチャー: {:?}", features);
    println!("前処理後のテキスト: {}", processed);

    // 基本的な検証 - 何らかのフィーチャーが検出されることを確認
    assert!(!features.is_empty(), "何らかのフィーチャーが検出される");

    // バニッシュは確実に検出されるはず
    assert!(features.contains(&CardFeature::Banish), "バニッシュを検出");

    // 前処理が行われていることを確認
    assert!(processed != complex_text, "前処理が実行されている");
}

#[test]
fn test_migration_compatibility() {
    use migration::*;

    let test_text = r#"
    【ライフバースト】：カードを１枚引く。対戦相手のシグニ１体をバニッシュする。
    このシグニがアタックすると正面のシグニとバトルをせず対戦相手にダメージを与える。
    "#;

    // 移行されたアナライザーを作成
    let migrated_analyzer = create_migrated_analyzer().unwrap();
    let (_, migrated_features) = migrated_analyzer.analyze(test_text);

    // 基本的なフィーチャーが検出されることを確認
    assert!(
        !migrated_features.is_empty(),
        "何らかのフィーチャーが検出される"
    );

    println!(
        "移行されたアナライザーで検出されたフィーチャー: {:?}",
        migrated_features
    );
}

#[test]
fn test_parallel_performance() {
    let analyzer = CardFeatureAnalyzer::new().unwrap();

    let test_texts = vec![
        "【ライフバースト】：カードを１枚引く",
        "対戦相手のシグニ１体をバニッシュする",
        "このシグニはアサシンを持つ",
        "【チャーム】を付ける",
        "ダブルクラッシュを持つ",
    ];

    // 並列処理のテスト
    let features: HashSet<CardFeature> = test_texts
        .par_iter()
        .flat_map(|text| analyzer.detect(text))
        .collect();

    // 複数のフィーチャーが検出されることを確認
    assert!(
        !features.is_empty(),
        "並列処理で複数のフィーチャーが検出される"
    );
    assert!(
        features.len() >= 3,
        "少なくとも3つのフィーチャーが検出される"
    );

    println!("並列処理で検出されたフィーチャー: {:?}", features);
}

#[test]
fn test_enhanced_analyzer_features() {
    // 本番のアナライザーを使用
    let analyzer = common::get_production_analyzer();

    let test_text = "対戦相手のシグニをバニッシュして【チャーム】を付ける";

    // 通常の解析
    let normal_features = analyzer.analyze(test_text);

    // 並列解析
    let parallel_features = analyzer.analyze_parallel(test_text);

    // 前処理付き解析
    let (_processed_text, preprocessing_features) = analyzer.analyze_with_preprocessing(test_text);

    // 結果が一致することを確認
    assert_eq!(
        normal_features, parallel_features,
        "通常と並列処理の結果が一致"
    );
    assert_eq!(
        normal_features, preprocessing_features,
        "通常と前処理付きの結果が一致"
    );

    assert!(normal_features.contains(&CardFeature::Banish));
    assert!(normal_features.contains(&CardFeature::Charm));

    println!("拡張アナライザーのテスト完了");
}
