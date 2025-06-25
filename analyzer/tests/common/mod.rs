use analyzer::{Analyzer, create_production_analyzer};
use feature::CardFeature;

/// 本番のフィーチャー検出ルールを使用するアナライザーを作成
/// すべてのテストで共通して使用することで、実際の動作と一致したテストを実現
pub fn get_production_analyzer() -> Analyzer<CardFeature> {
    create_production_analyzer()
}

/// テキストからフィーチャーを検出するヘルパー関数
/// 本番と同じ前処理（置換）を適用してからフィーチャーを検出
pub fn detect_features(text: &str) -> std::collections::HashSet<CardFeature> {
    let analyzer = get_production_analyzer();
    analyzer.analyze(text)
}

/// 前処理も含めた完全な解析を実行するヘルパー関数
pub fn analyze_with_preprocessing(text: &str) -> (String, std::collections::HashSet<CardFeature>) {
    let analyzer = get_production_analyzer();
    analyzer.analyze_with_preprocessing(text)
}
