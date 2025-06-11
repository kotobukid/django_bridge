use analyzer::AnalyzeRule;
use analyzer::migration;
use analyzer::raw_card_analyzer::to_half;
use feature::CardFeature;
use std::collections::HashSet;

mod common;

#[test]
fn test_card_feature_detection_banish_pattern() {
    let card_skill_text = to_half(
        r#"
    【出現条件】《メインフェイズアイコン》レゾナではない＜凶蟲＞のシグニ２体をあなたの場からトラッシュに置く
    【常】：対戦相手は【チャーム】が付いているシグニの《起》能力を使用できない。
    【自】：対戦相手のシグニ１体が場に出たとき、対戦相手は自分のデッキの一番上のカードをそのシグニの【チャーム】にする。
    【自】：各アタックフェイズ開始時、対戦相手は【チャーム】が付いている自分のシグニ１体を対象とし、それをバニッシュする。
    "#,
    );

    // 本番のルールを使用
    let detected_features = common::detect_features(card_skill_text.as_str());

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
    let card_skill_text = to_half(
        r#"
    【常】：対戦相手は【チャーム】が付いているシグニの《起》能力を使用できない。
    【自】：対戦相手のシグニ１体が場に出たとき、対戦相手は自分のデッキの一番上のカードをそのシグニの【チャーム】にする。
    "#,
    );

    // 本番のルールを使用
    let detected_features = common::detect_features(card_skill_text.as_str());

    assert!(
        detected_features.contains(&CardFeature::Charm),
        "Should detect Charm feature from 【チャーム】 text"
    );
}

#[test]
fn test_card_feature_detection_acce_pattern() {
    // WXDi-P05-060
    let card_skill_text = to_half(
        r#"
    あなたの赤のシグニ１体を対象とし、それの下にカードが無い場合、このカードをそれの下に置く。

    【常】：このカードの上にある赤のシグニは「【自】：あなたのアタックフェイズ開始時、【エナチャージ１】をする。」を得る。
    【常】：このカードの上にある《コードアクセル　ヒャッハー》のパワーを＋2000する。 
    "#,
    );

    let detected_features = common::detect_features(card_skill_text.as_str());

    assert!(
        !detected_features.contains(&CardFeature::Acce),
        "Should NOT detect Acce feature from 'コードアクセル' text"
    );
    assert!(
        detected_features.contains(&CardFeature::Charge),
        "Should detect Enhance feature from 'エナチャージ1' text"
    );
    assert!(
        detected_features.contains(&CardFeature::PowerUp),
        "Should detect PowerUp feature from '+2000' text"
    );
}

#[test]
fn test_card_feature_detection_multiple_features() {
    let card_skill_text = to_half(
        r#"
    【ライフバースト】：カードを１枚引く。対戦相手のシグニ１体をバニッシュする。
    【常】：このシグニがアタックしたとき、対戦相手のライフクロス１枚をクラッシュする。
    "#,
    );

    // 本番のルールを使用（前処理も含む）
    let (_processed_text, detected_features) =
        common::analyze_with_preprocessing(card_skill_text.as_str());

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
    let card_skill_text = to_half(
        r#"対戦相手のシグニ１体を対象とし、それをバニッシュする。それが感染状態の場合、代わりにそれをトラッシュに置く。"#,
    );

    // 本番のルールを使用
    let detected_features = common::detect_features(card_skill_text.as_str());

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
    let analyzer = common::get_production_analyzer();

    let text1 = to_half("カードを１枚引く");
    let text2 = to_half("カードを３枚引く");
    let text3 = to_half("何もしない");

    assert!(
        analyzer
            .analyze(text1.as_str())
            .contains(&CardFeature::Draw)
    );
    assert!(
        analyzer
            .analyze(text2.as_str())
            .contains(&CardFeature::Draw)
    );
    assert!(
        !analyzer
            .analyze(text3.as_str())
            .contains(&CardFeature::Draw)
    );
}

#[test]
fn test_category_rules() {
    // 本番のアナライザーを使用
    let analyzer = common::get_production_analyzer();

    // Lethal系のテスト
    let assassin_text = to_half("このシグニはアサシンを持つ");
    let crush_text = to_half("このシグニはダブルクラッシュを持つ");

    let assassin_features = analyzer.analyze(assassin_text.as_str());
    assert!(assassin_features.contains(&CardFeature::Assassin));

    let crush_features = analyzer.analyze(crush_text.as_str());
    assert!(crush_features.contains(&CardFeature::DoubleCrush));

    // Offensive系のテスト
    let banish_text = to_half("対戦相手のシグニ１体をバニッシュする");
    let bounce_text = to_half("対戦相手のシグニ１体を対象とし、それを手札に戻す");

    let banish_features = analyzer.analyze(banish_text.as_str());
    assert!(banish_features.contains(&CardFeature::Banish));

    let bounce_features = analyzer.analyze(bounce_text.as_str());
    assert!(bounce_features.contains(&CardFeature::Bounce));

    // Unique系のテスト
    let charm_text = to_half("【チャーム】を付ける");
    let lifeburst_text = to_half("【ライフバースト】：カードを１枚引く");

    let charm_features = analyzer.analyze(charm_text.as_str());
    assert!(charm_features.contains(&CardFeature::Charm));

    let lb_features = analyzer.analyze(lifeburst_text.as_str());
    assert!(lb_features.contains(&CardFeature::LifeBurst));
}

#[test]
fn test_card_feature_analyzer() {
    // 本番のアナライザーを使用
    let analyzer = common::get_production_analyzer();

    let complex_text = to_half(
        r#"
    【ライフバースト】：カードを２枚引く。対戦相手のシグニ１体をバニッシュする。
    【常】：このシグニはアサシンを持つ。
    【自】：このシグニがアタックしたとき、【チャーム】を１つ付ける。
    "#,
    );

    let (_processed, features) = analyzer.analyze_with_preprocessing(complex_text.as_str());

    println!("検出されたフィーチャー: {:?}", features);

    // 基本的な検証 - 何らかのフィーチャーが検出されることを確認
    assert!(!features.is_empty(), "何らかのフィーチャーが検出される");

    // 期待されるフィーチャーが検出されることを確認
    assert!(features.contains(&CardFeature::Banish), "バニッシュを検出");
    assert!(
        features.contains(&CardFeature::LifeBurst),
        "ライフバーストを検出"
    );
    assert!(features.contains(&CardFeature::Draw), "ドローを検出");
    assert!(features.contains(&CardFeature::Assassin), "アサシンを検出");
    assert!(features.contains(&CardFeature::Charm), "チャームを検出");
}

#[test]
fn test_migration_compatibility() {
    use migration::*;

    let test_text = to_half(
        r#"
    【ライフバースト】：カードを１枚引く。対戦相手のシグニ１体をバニッシュする。
    このシグニがアタックすると正面のシグニとバトルをせず対戦相手にダメージを与える。
    "#,
    );

    // 移行されたアナライザーを作成
    let migrated_analyzer = create_migrated_analyzer().unwrap();
    let (_, migrated_features) = migrated_analyzer.analyze(test_text.as_str());

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
    // 本番のアナライザーを使用
    let analyzer = common::get_production_analyzer();

    let test_texts = vec![
        "【ライフバースト】：カードを１枚引く",
        "対戦相手のシグニ１体をバニッシュする",
        "このシグニはアサシンを持つ",
        "【チャーム】を付ける",
        "ダブルクラッシュを持つ",
    ];

    // 各テキストを個別に処理し、analyze_parallelメソッドを使用
    let mut all_features = HashSet::new();
    for text in test_texts {
        let features = analyzer.analyze_parallel(to_half(text).as_str());
        all_features.extend(features);
    }

    // 複数のフィーチャーが検出されることを確認
    assert!(
        !all_features.is_empty(),
        "並列処理で複数のフィーチャーが検出される"
    );
    assert!(
        all_features.len() >= 3,
        "少なくとも3つのフィーチャーが検出される"
    );

    println!("並列処理で検出されたフィーチャー: {:?}", all_features);
}

#[test]
fn test_enhanced_analyzer_features() {
    // 本番のアナライザーを使用
    let analyzer = common::get_production_analyzer();

    let test_text = to_half("対戦相手のシグニをバニッシュして【チャーム】を付ける");

    // 通常の解析
    let normal_features = analyzer.analyze(test_text.as_str());

    // 並列解析
    let parallel_features = analyzer.analyze_parallel(test_text.as_str());

    // 前処理付き解析
    let (_processed_text, preprocessing_features) =
        analyzer.analyze_with_preprocessing(test_text.as_str());

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
