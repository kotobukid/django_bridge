use analyzer::category_rules;
use analyzer::migration;
use analyzer::*;
use feature::CardFeature;
use rayon::prelude::*;
use std::collections::HashSet;

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

    let mut analyzer: Analyzer<CardFeature> = Analyzer::new();
    analyzer.add_rule(Box::new(BanishDetectRule::new()));

    let detected_features = analyzer.analyze(card_skill_text);

    assert!(
        detected_features.contains(&CardFeature::Banish),
        "Should detect Banish feature from 'バニッシュする' text"
    );
}

#[test]
fn test_card_feature_detection_charm_pattern() {
    let card_skill_text = r#"
    【常】：対戦相手は【チャーム】が付いているシグニの《起》能力を使用できない。
    【自】：対戦相手のシグニ１体が場に出たとき、対戦相手は自分のデッキの一番上のカードをそのシグニの【チャーム】にする。
    "#;

    let mut analyzer: Analyzer<CardFeature> = Analyzer::new();
    analyzer.add_rule(Box::new(CharmDetectRule::new()));

    let detected_features = analyzer.analyze(card_skill_text);

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

    let mut analyzer: Analyzer<CardFeature> = Analyzer::new();
    analyzer.add_rule(Box::new(LifeBurstDetectRule::new()));
    analyzer.add_rule(Box::new(DrawDetectRule::new()));
    analyzer.add_rule(Box::new(BanishDetectRule::new()));
    analyzer.add_rule(Box::new(CrashDetectRule::new()));

    let detected_features = analyzer.analyze(card_skill_text);

    assert!(
        detected_features.contains(&CardFeature::LifeBurst),
        "Should detect LifeBurst feature"
    );
    assert!(
        detected_features.contains(&CardFeature::Draw),
        "Should detect Draw feature"
    );
    assert!(
        detected_features.contains(&CardFeature::Banish),
        "Should detect Banish feature"
    );
    assert!(
        detected_features.contains(&CardFeature::DoubleCrush),
        "Should detect Crash feature"
    );
    assert_eq!(
        detected_features.len(),
        4,
        "Should detect exactly 4 features"
    );
}

/// Rule to detect Banish pattern
pub struct BanishDetectRule {
    pattern: regex::Regex,
}

impl BanishDetectRule {
    pub fn new() -> Self {
        Self {
            pattern: regex::Regex::new(r"バニッシュ").unwrap(),
        }
    }
}

impl AnalyzeRule<CardFeature> for BanishDetectRule {
    fn detect(&self, text: &str) -> HashSet<CardFeature> {
        let mut result = HashSet::new();
        if self.pattern.is_match(text) {
            result.insert(CardFeature::Banish);
        }
        result
    }
}

/// Rule to detect Charm pattern
pub struct CharmDetectRule {
    pattern: regex::Regex,
}

impl CharmDetectRule {
    pub fn new() -> Self {
        Self {
            pattern: regex::Regex::new(r"【チャーム】").unwrap(),
        }
    }
}

impl AnalyzeRule<CardFeature> for CharmDetectRule {
    fn detect(&self, text: &str) -> HashSet<CardFeature> {
        let mut result = HashSet::new();
        if self.pattern.is_match(text) {
            result.insert(CardFeature::Charm);
        }
        result
    }
}

/// Rule to detect LifeBurst pattern
pub struct LifeBurstDetectRule {
    pattern: regex::Regex,
}

impl LifeBurstDetectRule {
    pub fn new() -> Self {
        Self {
            pattern: regex::Regex::new(r"【ライフバースト】").unwrap(),
        }
    }
}

impl AnalyzeRule<CardFeature> for LifeBurstDetectRule {
    fn detect(&self, text: &str) -> HashSet<CardFeature> {
        let mut result = HashSet::new();
        if self.pattern.is_match(text) {
            result.insert(CardFeature::LifeBurst);
        }
        result
    }
}

/// Rule to detect Draw pattern
pub struct DrawDetectRule {
    pattern: regex::Regex,
}

impl DrawDetectRule {
    pub fn new() -> Self {
        Self {
            pattern: regex::Regex::new(r"カードを.*?引く").unwrap(),
        }
    }
}

impl AnalyzeRule<CardFeature> for DrawDetectRule {
    fn detect(&self, text: &str) -> HashSet<CardFeature> {
        let mut result = HashSet::new();
        if self.pattern.is_match(text) {
            result.insert(CardFeature::Draw);
        }
        result
    }
}

/// Rule to detect Crash pattern
pub struct CrashDetectRule {
    pattern: regex::Regex,
}

impl CrashDetectRule {
    pub fn new() -> Self {
        Self {
            pattern: regex::Regex::new(r"クラッシュ").unwrap(),
        }
    }
}

impl AnalyzeRule<CardFeature> for CrashDetectRule {
    fn detect(&self, text: &str) -> HashSet<CardFeature> {
        let mut result = HashSet::new();
        if self.pattern.is_match(text) {
            result.insert(CardFeature::DoubleCrush);
        }
        result
    }
}

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
    let mut analyzer: Analyzer<CardFeature> = Analyzer::new();

    // カスタムルールを追加
    analyzer.add_rule(Box::new(BanishDetectRule::new()));
    analyzer.add_rule(Box::new(CharmDetectRule::new()));

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
