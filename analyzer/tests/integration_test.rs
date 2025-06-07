use analyzer::*;
use feature::CardFeature;
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
    
    assert!(detected_features.contains(&CardFeature::Banish), "Should detect Banish feature from 'バニッシュする' text");
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
    
    assert!(detected_features.contains(&CardFeature::Charm), "Should detect Charm feature from 【チャーム】 text");
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
    
    assert!(detected_features.contains(&CardFeature::LifeBurst), "Should detect LifeBurst feature");
    assert!(detected_features.contains(&CardFeature::Draw), "Should detect Draw feature");
    assert!(detected_features.contains(&CardFeature::Banish), "Should detect Banish feature");
    assert!(detected_features.contains(&CardFeature::DoubleCrush), "Should detect Crash feature");
    assert_eq!(detected_features.len(), 4, "Should detect exactly 4 features");
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
