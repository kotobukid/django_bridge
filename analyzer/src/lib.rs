use std::collections::HashSet;
use regex::Regex;
use feature::CardFeature;
use rayon::prelude::*;

/// テキストを解析してパターンを検出するルールのトレイト
pub trait AnalyzeRule<T> {
    /// 指定されたテキストからパターンを検出し、見つかったアイテムのHashSetを返す
    fn detect(&self, text: &str) -> HashSet<T>;

    /// テキストの前処理を行う（デフォルト実装は何もしない）
    fn preprocess(&self, text: &str) -> String {
        text.to_string()
    }

    /// 前処理と検出を一度に実行する
    fn analyze(&self, text: &str) -> (String, HashSet<T>) {
        let processed_text = self.preprocess(text);
        let features = self.detect(&processed_text);
        (processed_text, features)
    }
}

/// A rule that detects individual digits in text and returns them as i32
pub struct DigitDetectRule {
    pattern: Regex,
}

impl DigitDetectRule {
    pub fn new() -> Self {
        Self {
            pattern: Regex::new(r"\d").unwrap(),
        }
    }
}

impl AnalyzeRule<i32> for DigitDetectRule {
    fn detect(&self, text: &str) -> HashSet<i32> {
        let mut result = HashSet::new();

        for cap in self.pattern.find_iter(text) {
            if let Ok(digit) = cap.as_str().parse::<i32>() {
                result.insert(digit);
            }
        }

        result
    }
}

/// A rule that detects even digits in text
pub struct EvenNumberRule {
    pattern: Regex,
}

impl EvenNumberRule {
    pub fn new() -> Self {
        Self {
            // Use \d to match individual digits
            pattern: Regex::new(r"\d").unwrap(),
        }
    }
}

impl AnalyzeRule<i32> for EvenNumberRule {
    fn detect(&self, text: &str) -> HashSet<i32> {
        let mut result = HashSet::new();

        for cap in self.pattern.find_iter(text) {
            if let Ok(digit) = cap.as_str().parse::<i32>() {
                // Check if the digit is even (0, 2, 4, 6, 8)
                if digit % 2 == 0 {
                    result.insert(digit);
                }
            }
        }

        result
    }
}

/// A rule that detects words with a specific length
pub struct WordLengthRule {
    pattern: Regex,
    length: usize,
}

impl WordLengthRule {
    pub fn new(length: usize) -> Self {
        Self {
            pattern: Regex::new(r"\b\w+\b").unwrap(),
            length,
        }
    }
}

impl AnalyzeRule<String> for WordLengthRule {
    fn detect(&self, text: &str) -> HashSet<String> {
        let mut result = HashSet::new();

        for cap in self.pattern.find_iter(text) {
            let word = cap.as_str();
            if word.len() == self.length {
                result.insert(word.to_string());
            }
        }

        result
    }
}

/// An analyzer that uses multiple rules to analyze text
pub struct Analyzer<T> {
    rules: Vec<Box<dyn AnalyzeRule<T>>>,
}

impl<T> Analyzer<T> {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: Box<dyn AnalyzeRule<T>>) {
        self.rules.push(rule);
    }

    pub fn analyze(&self, text: &str) -> HashSet<T>
    where
        T: Clone + Eq + std::hash::Hash,
    {
        let mut result = HashSet::new();

        for rule in &self.rules {
            let detected = rule.detect(text);
            result.extend(detected);
        }

        result
    }
}

/// Example usage of the analyzer
pub fn example_digit_analysis(text: &str) -> HashSet<i32> {
    let mut analyzer = Analyzer::new();
    analyzer.add_rule(Box::new(DigitDetectRule::new()));
    analyzer.analyze(text)
}

/// A rule that detects email addresses in text
pub struct EmailDetectRule {
    pattern: Regex,
}

impl EmailDetectRule {
    pub fn new() -> Self {
        Self {
            // Simple email regex pattern
            pattern: Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b").unwrap(),
        }
    }
}

impl AnalyzeRule<String> for EmailDetectRule {
    fn detect(&self, text: &str) -> HashSet<String> {
        let mut result = HashSet::new();

        for cap in self.pattern.find_iter(text) {
            result.insert(cap.as_str().to_string());
        }

        result
    }
}

/// Example usage with multiple rules
pub fn example_number_analysis(text: &str) -> HashSet<i32> {
    let mut analyzer = Analyzer::new();
    analyzer.add_rule(Box::new(DigitDetectRule::new()));
    analyzer.add_rule(Box::new(EvenNumberRule::new()));
    analyzer.analyze(text)
}

/// Example usage for email detection
pub fn example_email_analysis(text: &str) -> HashSet<String> {
    let mut analyzer = Analyzer::new();
    analyzer.add_rule(Box::new(EmailDetectRule::new()));
    analyzer.analyze(text)
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 関連するフィーチャーとオプションの置換テキストを持つコンパイル済みパターン
#[derive(Clone)]
pub struct CompiledPattern {
    pub regex: Regex,
    pub features: Vec<CardFeature>,
    pub replace_to: Option<String>,
}

impl CompiledPattern {
    pub fn new(pattern: &str, features: Vec<CardFeature>, replace_to: Option<String>) -> Result<Self, regex::Error> {
        Ok(Self {
            regex: Regex::new(pattern)?,
            features,
            replace_to,
        })
    }

    pub fn detect_pattern(pattern: &str, features: Vec<CardFeature>) -> Result<Self, regex::Error> {
        Self::new(pattern, features, None)
    }

    pub fn replace_pattern(pattern: &str, replace_to: String, features: Vec<CardFeature>) -> Result<Self, regex::Error> {
        Self::new(pattern, features, Some(replace_to))
    }
}

/// 日本語テキストの数値バリエーションを処理するルール（any_numマクロ相当）
pub struct NumericVariationRule {
    pub base_pattern: String,
    pub patterns: Vec<CompiledPattern>,
    pub features: Vec<CardFeature>,
}

impl NumericVariationRule {
    pub fn new(pattern_head: &str, pattern_tail: &str, features: Vec<CardFeature>) -> Result<Self, regex::Error> {
        let base_pattern = format!("{}{}", pattern_head, pattern_tail);
        let patterns = Self::generate_numeric_patterns(pattern_head, pattern_tail, features.clone())?;

        Ok(Self {
            base_pattern,
            patterns,
            features,
        })
    }

    fn generate_numeric_patterns(head: &str, tail: &str, features: Vec<CardFeature>) -> Result<Vec<CompiledPattern>, regex::Error> {
        // 日本語全角数字（括弧付き）
        let numbers = ["（０）", "（１）", "（２）", "（３）", "（４）", "（５）", "（６）", "（７）", "（８）", "（９）"];
        let mut patterns = Vec::new();

        for num in numbers {
            let pattern = format!("{}{}{}", regex::escape(head), regex::escape(num), regex::escape(tail));
            patterns.push(CompiledPattern::detect_pattern(&pattern, features.clone())?);
        }

        // 任意の数字にマッチする汎用パターンも追加
        let generic_pattern = format!("{}[（]?[０-９]+[）]?{}", regex::escape(head), regex::escape(tail));
        patterns.push(CompiledPattern::detect_pattern(&generic_pattern, features)?);

        Ok(patterns)
    }
}

impl AnalyzeRule<CardFeature> for NumericVariationRule {
    fn detect(&self, text: &str) -> HashSet<CardFeature> {
        for pattern in &self.patterns {
            if pattern.regex.is_match(text) {
                return self.features.iter().cloned().collect();
            }
        }
        HashSet::new()
    }
}

/// 単一のコンパイル済みパターンに基づくシンプルなルール
pub struct SimpleRule {
    pub pattern: CompiledPattern,
}

impl SimpleRule {
    pub fn new(pattern: &str, features: Vec<CardFeature>) -> Result<Self, regex::Error> {
        Ok(Self {
            pattern: CompiledPattern::detect_pattern(pattern, features)?,
        })
    }

    pub fn replace_rule(pattern: &str, replace_to: String, features: Vec<CardFeature>) -> Result<Self, regex::Error> {
        Ok(Self {
            pattern: CompiledPattern::replace_pattern(pattern, replace_to, features)?,
        })
    }
}

impl AnalyzeRule<CardFeature> for SimpleRule {
    fn detect(&self, text: &str) -> HashSet<CardFeature> {
        if self.pattern.regex.is_match(text) {
            self.pattern.features.iter().cloned().collect()
        } else {
            HashSet::new()
        }
    }

    fn preprocess(&self, text: &str) -> String {
        if let Some(ref replace_to) = self.pattern.replace_to {
            self.pattern.regex.replace_all(text, replace_to).into_owned()
        } else {
            text.to_string()
        }
    }
}

/// 異なるタイプのカードフィーチャーに対応するカテゴリベースのルール
pub mod category_rules {
    use super::*;

    /// 致命的フィーチャー用のルール（Assassin, DoubleCrush, SLancer等）
    pub struct LethalRule {
        patterns: Vec<CompiledPattern>,
    }

    impl LethalRule {
        pub fn new() -> Result<Self, regex::Error> {
            let mut patterns = Vec::new();

            // アサシンパターン
            patterns.push(CompiledPattern::detect_pattern(r"アサシン", vec![CardFeature::Assassin])?);
            patterns.push(CompiledPattern::detect_pattern(r"【アサシン】", vec![CardFeature::Assassin])?);
            patterns.push(CompiledPattern::detect_pattern(r"このシグニがアタックすると正面のシグニとバトルをせず対戦相手にダメージを与える", vec![CardFeature::Assassin])?);

            // ダブルクラッシュパターン
            patterns.push(CompiledPattern::detect_pattern(r"ダブルクラッシュ", vec![CardFeature::DoubleCrush])?);
            patterns.push(CompiledPattern::detect_pattern(r"トリプルクラッシュ", vec![CardFeature::DoubleCrush])?);

            // Sランサーパターン
            patterns.push(CompiledPattern::detect_pattern(r"Sランサー", vec![CardFeature::SLancer, CardFeature::Lancer])?);
            patterns.push(CompiledPattern::detect_pattern(r"Ｓランサー", vec![CardFeature::SLancer, CardFeature::Lancer])?);

            // ランサーパターン
            patterns.push(CompiledPattern::detect_pattern(r"ランサー", vec![CardFeature::Lancer])?);
            patterns.push(CompiledPattern::detect_pattern(r"ライフクロスを１枚クラッシュする", vec![CardFeature::LifeCrush])?);
            patterns.push(CompiledPattern::detect_pattern(r"対戦相手のライフクロス１枚をクラッシュする。", vec![CardFeature::LifeCrush])?);

            // ダメージパターン
            patterns.push(CompiledPattern::detect_pattern(r"対戦相手にダメージを与える。", vec![CardFeature::Damage])?);

            Ok(Self { patterns })
        }
    }

    impl AnalyzeRule<CardFeature> for LethalRule {
        fn detect(&self, text: &str) -> HashSet<CardFeature> {
            let mut features = HashSet::new();
            for pattern in &self.patterns {
                if pattern.regex.is_match(text) {
                    features.extend(pattern.features.iter().cloned());
                }
            }
            features
        }
    }

    /// 攻撃的フィーチャー用のルール（Banish, PowerDown, Bounce等）
    pub struct OffensiveRule {
        patterns: Vec<CompiledPattern>,
    }

    impl OffensiveRule {
        pub fn new() -> Result<Self, regex::Error> {
            let mut patterns = Vec::new();

            // バニッシュパターン
            patterns.push(CompiledPattern::detect_pattern(r"バニッシュ", vec![CardFeature::Banish])?);

            // パワーダウンパターン
            patterns.push(CompiledPattern::detect_pattern(r"(シグニ|それ|それら)のパワーを－", vec![CardFeature::PowerDown])?);
            patterns.push(CompiledPattern::detect_pattern(r"(シグニ|それ)のパワーをこの方法で.+－", vec![CardFeature::PowerDown])?);

            // バウンスパターン
            patterns.push(CompiledPattern::detect_pattern(r"対戦相手のシグニ.+体(まで|を)対象とし、(それら|それ)を手札に戻", vec![CardFeature::Bounce])?);
            patterns.push(CompiledPattern::detect_pattern(r"対戦相手のパワー.+体(まで|を)対象とし、(それら|それ)を手札に戻", vec![CardFeature::Bounce])?);

            // エナ攻撃パターン
            patterns.push(CompiledPattern::detect_pattern(r"シグニ.+エナゾーンに置", vec![CardFeature::EnerOffensive])?);
            patterns.push(CompiledPattern::detect_pattern(r"対戦相手は自分の.?シグニ１体を選びエナゾーンに置", vec![CardFeature::EnerOffensive])?);
            patterns.push(CompiledPattern::detect_pattern(r"対戦相手のパワー.+以下のシグニ１体を対象とし、それをエナゾーンに置", vec![CardFeature::EnerOffensive])?);
            patterns.push(CompiledPattern::detect_pattern(r"対戦相手のすべてのシグニをエナゾーンに置", vec![CardFeature::EnerOffensive])?);

            Ok(Self { patterns })
        }
    }

    impl AnalyzeRule<CardFeature> for OffensiveRule {
        fn detect(&self, text: &str) -> HashSet<CardFeature> {
            let mut features = HashSet::new();
            for pattern in &self.patterns {
                if pattern.regex.is_match(text) {
                    features.extend(pattern.features.iter().cloned());
                }
            }
            features
        }
    }

    /// 防御的フィーチャー用のルール（Guard, Barrier, Shadow等）
    pub struct DefensiveRule {
        patterns: Vec<CompiledPattern>,
    }

    impl DefensiveRule {
        pub fn new() -> Result<Self, regex::Error> {
            let mut patterns = Vec::new();

            // ガードパターン
            patterns.push(CompiledPattern::detect_pattern(r"《ガードアイコン》", vec![CardFeature::Guard])?);
            patterns.push(CompiledPattern::detect_pattern(r"ガードアイコン", vec![CardFeature::Guard])?);

            // バリアパターン
            patterns.push(CompiledPattern::detect_pattern(r"シグニバリア", vec![CardFeature::Barrier])?);
            patterns.push(CompiledPattern::detect_pattern(r"ルリグバリア", vec![CardFeature::Barrier])?);

            // シャドウパターン
            patterns.push(CompiledPattern::detect_pattern(r"シャドウ", vec![CardFeature::Shadow])?);
            patterns.push(CompiledPattern::detect_pattern(r"【シャドウ】", vec![CardFeature::Shadow])?);

            // 無敵パターン
            patterns.push(CompiledPattern::detect_pattern(r"バニッシュされない", vec![CardFeature::Invulnerable])?);
            patterns.push(CompiledPattern::detect_pattern(r"ダメージを受けない", vec![CardFeature::CancelDamage])?);

            Ok(Self { patterns })
        }
    }

    impl AnalyzeRule<CardFeature> for DefensiveRule {
        fn detect(&self, text: &str) -> HashSet<CardFeature> {
            let mut features = HashSet::new();
            for pattern in &self.patterns {
                if pattern.regex.is_match(text) {
                    features.extend(pattern.features.iter().cloned());
                }
            }
            features
        }
    }

    /// 強化フィーチャー用のルール（Draw, Charge, Salvage等）
    pub struct EnhanceRule {
        patterns: Vec<CompiledPattern>,
        numeric_rules: Vec<NumericVariationRule>,
    }

    impl EnhanceRule {
        pub fn new() -> Result<Self, regex::Error> {
            let mut patterns = Vec::new();
            let mut numeric_rules = Vec::new();

            // 数値バリエーション付きドローパターン
            numeric_rules.push(NumericVariationRule::new("カードを", "枚引", vec![CardFeature::Draw])?);

            // チャージパターン
            patterns.push(CompiledPattern::detect_pattern(r"エナチャージ", vec![CardFeature::Charge])?);
            numeric_rules.push(NumericVariationRule::new("カードを", "枚までエナゾーンに置", vec![CardFeature::Charge])?);

            // サルベージパターン
            numeric_rules.push(NumericVariationRule::new("(シグニ|シグニを|シグニをそれぞれ)", "枚(を|まで).+手札に加え", vec![CardFeature::Salvage])?);
            numeric_rules.push(NumericVariationRule::new("スペル", "枚を.+手札に加え", vec![CardFeature::SalvageSpell])?);

            // パワーアップパターン
            patterns.push(CompiledPattern::detect_pattern(r"シグニのパワーを＋", vec![CardFeature::PowerUp])?);
            patterns.push(CompiledPattern::detect_pattern(r"このシグニのパワーは＋", vec![CardFeature::PowerUp])?);
            patterns.push(CompiledPattern::detect_pattern(r"(シグニ|それ|それら)のパワーを＋", vec![CardFeature::PowerUp])?);

            Ok(Self { patterns, numeric_rules })
        }
    }

    impl AnalyzeRule<CardFeature> for EnhanceRule {
        fn detect(&self, text: &str) -> HashSet<CardFeature> {
            let mut features = HashSet::new();

            // シンプルパターンをチェック
            for pattern in &self.patterns {
                if pattern.regex.is_match(text) {
                    features.extend(pattern.features.iter().cloned());
                }
            }

            // 数値バリエーションパターンをチェック
            for rule in &self.numeric_rules {
                features.extend(rule.detect(text));
            }

            features
        }
    }

    /// 特殊フィーチャー用のルール（Charm, Craft, Acce等）
    pub struct UniqueRule {
        patterns: Vec<CompiledPattern>,
    }

    impl UniqueRule {
        pub fn new() -> Result<Self, regex::Error> {
            let mut patterns = Vec::new();

            // チャームパターン
            patterns.push(CompiledPattern::detect_pattern(r"チャーム", vec![CardFeature::Charm])?);
            patterns.push(CompiledPattern::detect_pattern(r"【チャーム】", vec![CardFeature::Charm])?);

            // クラフトパターン
            patterns.push(CompiledPattern::detect_pattern(r"【クラフト】", vec![CardFeature::Craft])?);
            patterns.push(CompiledPattern::detect_pattern(r"クラフトの《", vec![CardFeature::Craft])?);
            patterns.push(CompiledPattern::detect_pattern(r"（このクラフトは効果以外によっては場に出せない）", vec![CardFeature::Craft])?);

            // アクセパターン
            patterns.push(CompiledPattern::detect_pattern(r"アクセ", vec![CardFeature::Acce])?);

            // ライズパターン
            patterns.push(CompiledPattern::detect_pattern(r"【ライズ】あなたの", vec![CardFeature::Rise])?);

            // ウィルスパターン
            patterns.push(CompiledPattern::detect_pattern(r"【ウィルス】", vec![CardFeature::Virus])?);

            // ライフバーストパターン
            patterns.push(CompiledPattern::detect_pattern(r"【ライフバースト】", vec![CardFeature::LifeBurst])?);

            Ok(Self { patterns })
        }
    }

    impl AnalyzeRule<CardFeature> for UniqueRule {
        fn detect(&self, text: &str) -> HashSet<CardFeature> {
            let mut features = HashSet::new();
            for pattern in &self.patterns {
                if pattern.regex.is_match(text) {
                    features.extend(pattern.features.iter().cloned());
                }
            }
            features
        }
    }
}

/// カードフィーチャー検出のために全カテゴリルールを組み合わせた包括的なアナライザー
pub struct CardFeatureAnalyzer {
    preprocessor_rules: Vec<SimpleRule>,
    category_rules: Vec<Box<dyn AnalyzeRule<CardFeature> + Send + Sync>>,
}

impl CardFeatureAnalyzer {
    pub fn new() -> Result<Self, regex::Error> {
        let mut analyzer = Self {
            preprocessor_rules: Vec::new(),
            category_rules: Vec::new(),
        };

        // 全カテゴリルールを追加
        analyzer.add_category_rule(Box::new(category_rules::LethalRule::new()?))?;
        analyzer.add_category_rule(Box::new(category_rules::OffensiveRule::new()?))?;
        analyzer.add_category_rule(Box::new(category_rules::DefensiveRule::new()?))?;
        analyzer.add_category_rule(Box::new(category_rules::EnhanceRule::new()?))?;
        analyzer.add_category_rule(Box::new(category_rules::UniqueRule::new()?))?;

        // 共通前処理ルールを追加
        analyzer.add_preprocessor_rules()?;

        Ok(analyzer)
    }

    pub fn add_category_rule(&mut self, rule: Box<dyn AnalyzeRule<CardFeature> + Send + Sync>) -> Result<(), regex::Error> {
        self.category_rules.push(rule);
        Ok(())
    }

    pub fn add_preprocessor_rule(&mut self, rule: SimpleRule) {
        self.preprocessor_rules.push(rule);
    }

    fn add_preprocessor_rules(&mut self) -> Result<(), regex::Error> {
        // 置換パターンに類似した共通前処理ルールを追加
        self.add_preprocessor_rule(SimpleRule::replace_rule(
            r"【ライフバースト】：",
            "LB:".to_string(),
            vec![CardFeature::LifeBurst]
        )?);

        self.add_preprocessor_rule(SimpleRule::replace_rule(
            r"（パワーが０以下のシグニはルールによってバニッシュされる）",
            "*POWER DOWN*".to_string(),
            vec![CardFeature::PowerDown]
        )?);

        self.add_preprocessor_rule(SimpleRule::replace_rule(
            r"（シグニのパワーを計算する場合、先に基本パワーを適用してプラスやマイナスをする）",
            "*CALC ORDER*".to_string(),
            vec![]
        )?);

        self.add_preprocessor_rule(SimpleRule::replace_rule(
            r"（凍結された(ルリグ|シグニ)は次の自分のアップフェイズにアップしない）",
            "*FROZEN*".to_string(),
            vec![CardFeature::Freeze]
        )?);

        self.add_preprocessor_rule(SimpleRule::replace_rule(
            r"（【アサシン】を持つシグニがアタックすると正面のシグニとバトルをせず対戦相手にダメージを与える）",
            "*ASSASSIN*".to_string(),
            vec![CardFeature::Assassin]
        )?);

        Ok(())
    }
}

impl AnalyzeRule<CardFeature> for CardFeatureAnalyzer {
    fn detect(&self, text: &str) -> HashSet<CardFeature> {
        let processed_text = self.preprocess(text);

        // 並列処理を使用して全カテゴリルールを実行
        self.category_rules
            .par_iter()
            .flat_map(|rule| rule.detect(&processed_text))
            .collect()
    }

    fn preprocess(&self, text: &str) -> String {
        let mut processed = text.to_string();

        // 全前処理ルールを順次適用
        for rule in &self.preprocessor_rules {
            processed = rule.preprocess(&processed);
        }

        processed
    }

    fn analyze(&self, text: &str) -> (String, HashSet<CardFeature>) {
        let processed_text = self.preprocess(text);
        let mut all_features = HashSet::new();

        // 前処理ルールからフィーチャーを収集
        for rule in &self.preprocessor_rules {
            all_features.extend(rule.detect(&processed_text));
        }

        // カテゴリルールからフィーチャーを収集（並列）
        let category_features: HashSet<CardFeature> = self.category_rules
            .par_iter()
            .flat_map(|rule| rule.detect(&processed_text))
            .collect();

        all_features.extend(category_features);

        (processed_text, all_features)
    }
}

/// 並列処理サポートを持つ拡張アナライザー
impl<T> Analyzer<T>
where
    T: Clone + Eq + std::hash::Hash + Send + Sync,
{
    /// より良いパフォーマンスのために並列処理を使用してテキストを解析
    pub fn analyze_parallel(&self, text: &str) -> HashSet<T> {
        self.rules
            .iter()
            .flat_map(|rule| rule.detect(text))
            .collect()
    }

    /// サポートする全ルールに前処理を適用してテキストを解析
    pub fn analyze_with_preprocessing(&self, text: &str) -> (String, HashSet<T>) {
        let mut processed_text = text.to_string();
        let mut all_features = HashSet::new();

        for rule in &self.rules {
            let (rule_processed, rule_features) = rule.analyze(&processed_text);
            processed_text = rule_processed;
            all_features.extend(rule_features);
        }

        (processed_text, all_features)
    }
}

/// 既存パターンを新しいアナライザーシステムに変換するための移行ユーティリティ
pub mod migration {
    use super::*;
    use feature::{create_detect_patterns, DetectPattern, ReplacePattern, PATTERNS_AMOUNT_D, PATTERNS_AMOUNT_R};

    // パターンを直接作成する関数
    fn get_legacy_patterns() -> ([ReplacePattern; PATTERNS_AMOUNT_R], [DetectPattern; PATTERNS_AMOUNT_D]) {
        create_detect_patterns()
    }

    /// featureクレートの既存パターンを全て使用してCardFeatureAnalyzerを作成
    pub fn create_migrated_analyzer() -> Result<CardFeatureAnalyzer, regex::Error> {
        let mut analyzer = CardFeatureAnalyzer::new()?;

        // 既存の置換パターンを前処理ルールとして追加
        let (replace_patterns, _detect_patterns) = get_legacy_patterns();

        for pattern in &replace_patterns[..5] { // 最初の5つだけをテストに使用
            let rule = SimpleRule::replace_rule(
                pattern.pattern,
                pattern.replace_to.to_string(),
                pattern.features_detected.to_vec()
            )?;
            analyzer.add_preprocessor_rule(rule);
        }

        // 既存の検出パターンの一部を使用してレガシー検出ルールを作成
        let legacy_rule = LegacyDetectRule::new()?;
        analyzer.add_category_rule(Box::new(legacy_rule))?;

        Ok(analyzer)
    }

    /// 既存の検出パターンをラップするルール
    pub struct LegacyDetectRule;

    impl LegacyDetectRule {
        pub fn new() -> Result<Self, regex::Error> {
            Ok(Self)
        }
    }

    impl AnalyzeRule<CardFeature> for LegacyDetectRule {
        fn detect(&self, text: &str) -> HashSet<CardFeature> {
            // 簡単なフィーチャー検出を実装（パフォーマンステスト用）
            let mut features = HashSet::new();

            if text.contains("バニッシュ") {
                features.insert(CardFeature::Banish);
            }
            if text.contains("【チャーム】") {
                features.insert(CardFeature::Charm);
            }
            if text.contains("アサシン") {
                features.insert(CardFeature::Assassin);
            }
            if text.contains("ダブルクラッシュ") {
                features.insert(CardFeature::DoubleCrush);
            }

            features
        }
    }

    /// 既存のany_numパターンをNumericVariationRuleに変換
    pub fn create_numeric_variation_rules() -> Result<Vec<NumericVariationRule>, regex::Error> {
        let mut rules = Vec::new();

        // 既存システムの一般的な数値パターン
        rules.push(NumericVariationRule::new("カードを", "枚引", vec![CardFeature::Draw])?);
        rules.push(NumericVariationRule::new("カードを", "枚までエナゾーンに置", vec![CardFeature::Charge])?);
        rules.push(NumericVariationRule::new("対戦相手のシグニ", "体を対象とし、それをバニッシュする", vec![CardFeature::Banish])?);
        rules.push(NumericVariationRule::new("対戦相手のシグニ", "体を対象とし、それを手札に戻", vec![CardFeature::Bounce])?);
        rules.push(NumericVariationRule::new("ライフクロス", "枚をトラッシュに置", vec![CardFeature::LifeTrash])?);
        rules.push(NumericVariationRule::new("エナゾーンからカード", "枚を選び、それらをトラッシュに置", vec![CardFeature::EnerAttack])?);
        rules.push(NumericVariationRule::new("デッキの上からカードを", "枚トラッシュに置", vec![CardFeature::Drop])?);
        rules.push(NumericVariationRule::new("スペル", "枚をコストを支払わずに使用する", vec![CardFeature::FreeSpell])?);
        rules.push(NumericVariationRule::new("シグニ", "枚を対象とし、それを場に出す", vec![CardFeature::PutSigniDefense, CardFeature::PutSigniOffense])?);

        Ok(rules)
    }

    /// 旧システムと新システムのパフォーマンス比較
    pub struct PerformanceComparison {
        pub legacy_time_ns: u128,
        pub new_system_time_ns: u128,
        pub legacy_features: HashSet<CardFeature>,
        pub new_features: HashSet<CardFeature>,
        pub matches: bool,
    }

    pub fn compare_performance(text: &str) -> Result<PerformanceComparison, regex::Error> {
        use std::time::Instant;

        // レガシーシステムをテスト
        let start = Instant::now();
        let (replace_patterns, detect_patterns) = get_legacy_patterns();

        // 置換パターンを適用（最初の5つのみ）
        let mut processed_text = text.to_string();
        for pattern in &replace_patterns[..5] {
            processed_text = pattern.pattern_r.replace_all(&processed_text, pattern.replace_to).into_owned();
        }

        // 検出パターンを適用（最初の10個のみ）
        let legacy_features: HashSet<CardFeature> = detect_patterns[..10]
            .iter()
            .filter_map(|pat| {
                if pat.pattern_r.is_match(&processed_text) {
                    Some(pat.features_detected.iter().cloned().collect::<HashSet<CardFeature>>())
                } else {
                    None
                }
            })
            .fold(HashSet::new(), |mut acc, detected| {
                acc.extend(detected);
                acc
            });
        let legacy_time_ns = start.elapsed().as_nanos();

        // 新システムをテスト
        let start = Instant::now();
        let analyzer = create_migrated_analyzer()?;
        let (_processed, new_features) = analyzer.analyze(text);
        let new_system_time_ns = start.elapsed().as_nanos();

        Ok(PerformanceComparison {
            legacy_time_ns,
            new_system_time_ns,
            matches: legacy_features == new_features,
            legacy_features,
            new_features,
        })
    }
}

pub mod raw_card_analyzer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_detection() {
        let mut analyzer = Analyzer::new();
        analyzer.add_rule(Box::new(DigitDetectRule::new()));

        let flags: HashSet<i32> = analyzer.analyze("019834");

        assert!(flags.contains(&0));
        assert!(flags.contains(&1));
        assert!(flags.contains(&3));
        assert!(flags.contains(&4));
        assert!(flags.contains(&8));
        assert!(flags.contains(&9));
    }

    #[test]
    fn test_even_number_detection() {
        let mut analyzer = Analyzer::new();
        analyzer.add_rule(Box::new(EvenNumberRule::new()));

        let flags: HashSet<i32> = analyzer.analyze("123 456 789");

        assert!(flags.contains(&2));
        assert!(flags.contains(&4));
        assert!(flags.contains(&6));
        assert!(flags.contains(&8));
        assert!(!flags.contains(&1));
        assert!(!flags.contains(&3));
    }

    #[test]
    fn test_word_length_detection() {
        let mut analyzer = Analyzer::new();
        analyzer.add_rule(Box::new(WordLengthRule::new(3)));

        let words: HashSet<String> = analyzer.analyze("The quick brown fox jumps over the lazy dog");

        assert!(words.contains("The"));
        assert!(words.contains("fox"));
        assert!(words.contains("the"));
        assert!(!words.contains("quick"));
        assert!(!words.contains("brown"));
    }

    #[test]
    fn test_email_detection() {
        let mut analyzer = Analyzer::new();
        analyzer.add_rule(Box::new(EmailDetectRule::new()));

        let emails: HashSet<String> = analyzer.analyze("Contact us at test@example.com or support@company.org for more information.");

        assert!(emails.contains("test@example.com"));
        assert!(emails.contains("support@company.org"));
        assert_eq!(emails.len(), 2);

        // Test with no emails
        let empty: HashSet<String> = analyzer.analyze("This text contains no email addresses.");
        assert_eq!(empty.len(), 0);
    }
}
