//! アイコン置換ルールの定義とマクロ

#[derive(Debug, Clone, PartialEq)]
pub struct IconRule {
    /// 検出パターン（元のテキスト）
    pub pattern: &'static str,
    /// 一時符号（エンコード後の文字列）
    pub code: &'static str,
    /// コンポーネント名（フロントエンドで使用）
    pub component: &'static str,
}

/// アイコンルールを定義するマクロ
/// 
/// # 使用例
/// ```
/// # use icon_encoder::definitions::IconRule;
/// macro_rules! define_icon_rules {
///     ($([$pattern:literal, $code:literal, $component:literal]),* $(,)?) => {
///         pub const ICON_RULES: &[IconRule] = &[
///             $(IconRule {
///                 pattern: $pattern,
///                 code: $code,
///                 component: $component,
///             }),*
///         ];
///     };
/// }
/// 
/// define_icon_rules![
///     ["【出】", "[c]", "IconCip"],
///     ["【常】", "[a]", "IconAuto"],
/// ];
/// ```
#[macro_export]
macro_rules! define_icon_rules {
    ($([$pattern:literal, $code:literal, $component:literal]),* $(,)?) => {
        pub const ICON_RULES: &[IconRule] = &[
            $(IconRule {
                pattern: $pattern,
                code: $code,
                component: $component,
            }),*
        ];
    };
}

// WIXOSSカードゲームのアイコンルール定義
define_icon_rules![
    // タイミングアイコン
    ["【出】", "[c]", "IconCip"],           // Comes into play（CIP）
    ["【絆出】", "[bc]", "IconBondCip"],     // Bond comes into play
    ["【常】", "[a]", "IconAuto"],          // Auto
    ["【起】", "[ac]", "IconActivated"],    // Activated
    ["【自】", "[p]", "IconPassive"],       // Passive

    ["《ターン1回》", "[t1]", "IconOnceTurn"],
    ["《ゲーム1回》", "[g1]", "IconOnceGame"],
    
    ["【ライフバースト】", "[lb2]", "IconLifeBurst2"],  // 【】形式のライフバースト
    
    // ゲームメカニクスアイコン
    ["《ガードアイコン》", "[gi]", "IconGuard"],
    ["《チーム》", "[t]", "IconTeam"],
    ["《ライフバースト》", "[lb]", "IconLifeBurst"],
    ["《リコレクトアイコン》", "[rc]", "IconRecollect"],
    
    // コラボアイコン
    ["《プリパラ》", "[pp]", "IconPripara"],
    ["《にじさんじ》", "[nj]", "IconNijisanji"],
    ["《ディソナ》", "[ds]", "IconDissona"],
    ["《電音部》", "[dn]", "IconDenonbu"],
    ["《ブルーアーカイブ》", "[bl]", "IconBlueArchive"],
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_rules_defined() {
        assert!(!ICON_RULES.is_empty());
        
        // 基本的なルールが定義されていることを確認
        let cip_rule = ICON_RULES.iter().find(|r| r.pattern == "【出】");
        assert!(cip_rule.is_some());
        assert_eq!(cip_rule.unwrap().code, "[c]");
        assert_eq!(cip_rule.unwrap().component, "IconCip");
    }

    #[test]
    fn test_no_duplicate_patterns() {
        let mut patterns = std::collections::HashSet::new();
        for rule in ICON_RULES {
            assert!(patterns.insert(rule.pattern), "重複するパターンが見つかりました: {}", rule.pattern);
        }
    }

    #[test]
    fn test_no_duplicate_codes() {
        let mut codes = std::collections::HashSet::new();
        for rule in ICON_RULES {
            assert!(codes.insert(rule.code), "重複するコードが見つかりました: {}", rule.code);
        }
    }
}