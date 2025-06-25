//! static_generator用のテキスト符号化機能

use crate::definitions::ICON_RULES;

/// スキルテキストを符号化する
/// 
/// アイコンパターン（例：【出】）を一時符号（例：[c]）に置換する。
/// 
/// # Arguments
/// * `text` - 元のスキルテキスト
/// 
/// # Returns
/// 符号化されたテキスト
/// 
/// # Example
/// ```
/// use icon_encoder::encode_skill_text;
/// 
/// let input = "【出】：あなたのデッキの上からカードを5枚見る。そのシグニの【出】能力は発動しない。";
/// let encoded = encode_skill_text(input);
/// assert_eq!(encoded, "[c]：あなたのデッキの上からカードを5枚見る。そのシグニの[c]能力は発動しない。");
/// ```
pub fn encode_skill_text(text: &str) -> String {
    let mut result = text.to_string();
    
    // 長いパターンから先に置換することで、部分的な置換を防ぐ
    let mut sorted_rules: Vec<_> = ICON_RULES.iter().collect();
    sorted_rules.sort_by(|a, b| b.pattern.len().cmp(&a.pattern.len()));
    
    for rule in sorted_rules {
        result = result.replace(rule.pattern, rule.code);
    }
    
    result
}

/// ライフバーストテキストを符号化する
/// 
/// スキルテキストと同じ処理を行うが、将来的に異なる処理が必要な場合に備えて分離。
/// 
/// # Arguments
/// * `text` - 元のライフバーストテキスト
/// 
/// # Returns
/// 符号化されたテキスト
pub fn encode_burst_text(text: &str) -> String {
    encode_skill_text(text)
}

/// 複数のテキストをまとめて符号化する
/// 
/// # Arguments
/// * `texts` - 符号化対象のテキストのスライス
/// 
/// # Returns
/// 符号化されたテキストのベクタ
pub fn encode_multiple_texts(texts: &[&str]) -> Vec<String> {
    texts.iter().map(|text| encode_skill_text(text)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_basic_timing_icons() {
        let input = "【出】：効果を発動する。";
        let expected = "[c]：効果を発動する。";
        assert_eq!(encode_skill_text(input), expected);
    }

    #[test]
    fn test_encode_multiple_same_icons() {
        let input = "【出】：あなたのデッキの上からカードを5枚見る。そのシグニの【出】能力は発動しない。";
        let expected = "[c]：あなたのデッキの上からカードを5枚見る。そのシグニの[c]能力は発動しない。";
        assert_eq!(encode_skill_text(input), expected);
    }

    #[test]
    fn test_encode_multiple_different_icons() {
        let input = "【出】：効果1を発動する。【自】：効果2を発動する。";
        let expected = "[c]：効果1を発動する。[p]：効果2を発動する。";
        assert_eq!(encode_skill_text(input), expected);
    }

    // #[test]
    // fn test_encode_with_abilities() {
    //     let input = "【アサシン】を持つシグニは【ガード】を無視する。";
    //     let expected = "[as]を持つシグニは[g]を無視する。";
    //     assert_eq!(encode_skill_text(input), expected);
    // }

    // #[test]
    // fn test_encode_with_mechanics() {
    //     let input = "《クラフト》：コイン1を支払って《グロウ》する。";
    //     let expected = "[cr]：コイン1を支払って[gr]する。";
    //     assert_eq!(encode_skill_text(input), expected);
    // }

    // #[test]
    // fn test_encode_with_collaboration() {
    //     let input = "《プリパラ》と《にじさんじ》のコラボカード。";
    //     let expected = "[pp]と[nj]のコラボカード。";
    //     assert_eq!(encode_skill_text(input), expected);
    // }

    #[test]
    fn test_encode_empty_text() {
        assert_eq!(encode_skill_text(""), "");
    }

    #[test]
    fn test_encode_text_without_icons() {
        let input = "このカードには特別な効果がありません。";
        assert_eq!(encode_skill_text(input), input);
    }

    #[test]
    fn test_encode_burst_text() {
        let input = "【ライフバースト】：《ガードアイコン》を1つ追加する。";
        let expected = "[lb2]：[gi]を1つ追加する。";
        assert_eq!(encode_burst_text(input), expected);
    }

    #[test]
    fn test_encode_multiple_texts() {
        let texts = &[
            "【出】：効果1",
            "【自】：効果2",
            "テキストなし"
        ];
        let expected = vec![
            "[c]：効果1".to_string(),
            "[p]：効果2".to_string(),
            "テキストなし".to_string()
        ];
        assert_eq!(encode_multiple_texts(texts), expected);
    }

    // #[test]
    // fn test_encode_priority_long_pattern_first() {
    //     // 長いパターンが優先されることを確認
    //     // もし【ガード】が先に処理されると【ガードアイコン】が正しく処理されない
    //     let input = "【ガードアイコン】と【ガード】の違い";
    //     let result = encode_skill_text(input);
    //
    //     // 【ガードアイコン】→[gi2]、【ガード】→[g] の順で処理されるべき
    //     assert_eq!(result, "[gi2]と[gi]の違い");
    //
    //     // 逆の順序（短いパターン優先）だと誤った結果になる
    //     assert_ne!(result, "[g]アイコン】と[g]の違い");
    // }
}