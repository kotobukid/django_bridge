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
    ["【出】", "[c]", "IconCip"],               // Comes into play（CIP）
    ["【絆出】", "[bc]", "IconBondCip"],        // Bond comes into play
    ["【絆常】", "[ba]", "IconBondAuto"],       // Bond auto
    ["【絆自】", "[bp]", "IconBondPassive"],    // Bond auto
    ["【絆起】", "[bac]", "IconBondActivated"], // Bond activated
    ["【常】", "[a]", "IconAuto"],              // Auto
    ["【起】", "[ac]", "IconActivated"],        // Activated
    ["【自】", "[p]", "IconPassive"],           // Passive
    ["《ターン1回》", "[t1]", "IconOnceTurn"],
    ["《ターン2回》", "[t2]", "IconTwiceTurn"],
    ["《ゲーム1回》", "[g1]", "IconOnceGame"],
    ["《クロスアイコン》", "[cr]", "IconCross"],
    ["《ダウン》", "[d]", "IconDown"],
    ["《相手ターン》", "[ot]", "IconOpponentTurn"],
    ["《自分ターン》", "[mt]", "IconMyTurn"],
    ["【ライフバースト】", "[lb2]", "IconLifeBurst2"], // 【】形式のライフバースト
    ["《ガードアイコン》", "[gi]", "IconGuard"],

    ["【チーム】", "[t]", "IconTeam"],
    ["【チーム自】", "[tp]", "IconTeamPassive"],
    ["【チーム出】", "[tc]", "IconTeamCip"],
    ["【チーム起】", "[ta]", "IconTeamActivated"],
    ["【チーム常】", "[tu]", "IconTeamAuto"],

    ["【クロス自】", "[xp]", "IconCrossPassive"],
    ["《ヘブン》", "[h]", "IconHeaven"],
    
    ["《ライフバースト》", "[lb]", "IconLifeBurst"],
    ["《リコレクトアイコン》", "[rc]", "IconRecollect"],
    ["【使用条件】【ドリームチーム】", "[dt]", "IconDreamTeam"],
    ["【使用条件】【チーム】", "[pt]", "IconTeamPiece"],
    ["【使用条件】", "[l]", "IconLimitation"],
    ["【出現条件】", "[ap]", "IconAppear"],
    ["【ハーモニー】", "[h]", "IconHarmony"],
    ["【ライズ】", "[rs]", "IconRise"],
    ["【  】", "[du]", "IconDuty"], // カーニバルメモリア
    ["《白》", "[iw]", "InlineWhite"],
    ["《青》", "[iu]", "InlineBlue"],
    ["《黒》", "[ik]", "InlineBlack"],
    ["《赤》", "[ir]", "InlineRed"],
    ["《緑》", "[ig]", "InlineGreen"],
    ["《無》", "[il]", "InlineColorless"],

    ["《白2》", "[iw]", "InlineWhite"], // 凶天ガープ用
    ["《青2》", "[iu]", "InlineBlue"],
    ["《黒2》", "[ik]", "InlineBlack"],
    ["《赤2》", "[ir]", "InlineRed"],
    ["《緑2》", "[ig]", "InlineGreen"],
    ["《無2》", "[il]", "InlineColorless"],

    ["《コインアイコン》", "[ic]", "InlineCoin"],
    ["【コイン】", "[ico]", "LabelCoin"],
    ["《ライズアイコン》", "[ri]", "RiseIcon"],
    ["《白×0》", "[0w]", "WhiteZero"],
    ["《青×0》", "[0u]", "BlueZero"],
    ["《黒×0》", "[0k]", "BlackZero"],
    ["《赤×0》", "[0r]", "RedZero"],
    ["《緑×0》", "[0g]", "GreenZero"],
    ["《無×0》", "[0l]", "ColorlessZero"],

    ["《ディソナアイコン》", "[ds]", "IconDissona"],
    
    ["《メインフェイズアイコン》", "[pa]", "IconMainPhase"],
    ["《アタックフェイズアイコン》", "[pm]", "IconAttackPhase"],
    ["《アタックフェイズアイコン2_黒》", "[ak]", "IconAttackPhase2k"],
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
            assert!(
                patterns.insert(rule.pattern),
                "重複するパターンが見つかりました: {}",
                rule.pattern
            );
        }
    }

    #[test]
    fn test_no_duplicate_codes() {
        let mut codes = std::collections::HashSet::new();
        for rule in ICON_RULES {
            assert!(
                codes.insert(rule.code),
                "重複するコードが見つかりました: {}",
                rule.code
            );
        }
    }
}
