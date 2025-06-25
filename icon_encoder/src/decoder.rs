//! wasm_front用のテキスト復号化機能

use crate::definitions::ICON_RULES;

/// テキストセグメントの種類
#[derive(Debug, Clone, PartialEq)]
pub enum TextSegment {
    /// プレーンテキスト
    Text(String),
    /// アイコン（コードとコンポーネント名）
    Icon {
        /// 符号化コード（例："c"）
        code: String,
        /// コンポーネント名（例："IconCip"）
        component: String,
    },
}

/// 符号化されたテキストを復号化してセグメントに分割する
///
/// 符号化されたテキストを解析し、テキスト部分とアイコン部分に分割する。
///
/// # Arguments
/// * `encoded_text` - 符号化済みテキスト（例："[c]：効果を発動する"）
///
/// # Returns
/// テキストセグメントのベクタ
///
/// # Example
/// ```
/// use icon_encoder::{decode_skill_text, TextSegment};
///
/// let encoded = "[c]：あなたのデッキの上からカードを5枚見る。";
/// let segments = decode_skill_text(encoded);
///
/// match &segments[0] {
///     TextSegment::Icon { code, component } => {
///         assert_eq!(code, "c");
///         assert_eq!(component, "IconCip");
///     },
///     _ => panic!("Expected icon segment"),
/// }
/// ```
pub fn decode_skill_text(encoded_text: &str) -> Vec<TextSegment> {
    let mut segments = Vec::new();
    let mut current_text = String::new();
    let mut chars = encoded_text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '[' {
            // アイコンコードの開始
            if !current_text.is_empty() {
                segments.push(TextSegment::Text(current_text.clone()));
                current_text.clear();
            }

            let mut code = String::new();
            let mut found_closing = false;

            for ch in chars.by_ref() {
                if ch == ']' {
                    found_closing = true;
                    break;
                } else {
                    code.push(ch);
                }
            }

            if found_closing {
                if let Some(component) = get_component_name(&code) {
                    segments.push(TextSegment::Icon {
                        code: code.clone(),
                        component,
                    });
                } else {
                    // 未知のコードはそのまま表示
                    segments.push(TextSegment::Text(format!("[{}]", code)));
                }
            } else {
                // 閉じ括弧がない場合はそのまま表示
                current_text.push('[');
                current_text.push_str(&code);
            }
        } else {
            current_text.push(ch);
        }
    }

    if !current_text.is_empty() {
        segments.push(TextSegment::Text(current_text));
    }

    segments
}

/// ライフバーストテキストを復号化する
///
/// スキルテキストと同じ処理を行うが、将来的に異なる処理が必要な場合に備えて分離。
///
/// # Arguments
/// * `encoded_text` - 符号化済みライフバーストテキスト
///
/// # Returns
/// テキストセグメントのベクタ
pub fn decode_burst_text(encoded_text: &str) -> Vec<TextSegment> {
    decode_skill_text(encoded_text)
}

/// コードからコンポーネント名を取得する
///
/// # Arguments
/// * `code` - 符号化コード（例："c"）
///
/// # Returns
/// 対応するコンポーネント名（例："IconCip"）。見つからない場合はNone。
fn get_component_name(code: &str) -> Option<String> {
    let full_code = format!("[{}]", code);
    ICON_RULES
        .iter()
        .find(|rule| rule.code == full_code)
        .map(|rule| rule.component.to_string())
}

/// コードから元のパターンを取得する（デバッグ用）
///
/// # Arguments
/// * `code` - 符号化コード（例："c"）
///
/// # Returns
/// 対応する元のパターン（例："【出】"）。見つからない場合はNone。
pub fn get_original_pattern(code: &str) -> Option<String> {
    let full_code = format!("[{}]", code);
    ICON_RULES
        .iter()
        .find(|rule| rule.code == full_code)
        .map(|rule| rule.pattern.to_string())
}

/// 利用可能なアイコンコードの一覧を取得する
///
/// # Returns
/// (コード, コンポーネント名, 元パターン)のタプルのベクタ
pub fn get_available_icons() -> Vec<(String, String, String)> {
    ICON_RULES
        .iter()
        .map(|rule| {
            let code = rule
                .code
                .trim_start_matches('[')
                .trim_end_matches(']')
                .to_string();
            (code, rule.component.to_string(), rule.pattern.to_string())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_basic_icon() {
        let encoded = "[c]：効果を発動する。";
        let segments = decode_skill_text(encoded);

        assert_eq!(segments.len(), 2);

        match &segments[0] {
            TextSegment::Icon { code, component } => {
                assert_eq!(code, "c");
                assert_eq!(component, "IconCip");
            }
            _ => panic!("Expected icon segment"),
        }

        match &segments[1] {
            TextSegment::Text(text) => {
                assert_eq!(text, "：効果を発動する。");
            }
            _ => panic!("Expected text segment"),
        }
    }

    #[test]
    fn test_decode_multiple_icons() {
        let encoded = "[c]：効果1を発動する。[a]：効果2を発動する。";
        let segments = decode_skill_text(encoded);

        assert_eq!(segments.len(), 4);

        // 最初のアイコン
        match &segments[0] {
            TextSegment::Icon { code, .. } => assert_eq!(code, "c"),
            _ => panic!("Expected icon segment"),
        }

        // 中間のテキスト
        match &segments[1] {
            TextSegment::Text(text) => assert_eq!(text, "：効果1を発動する。"),
            _ => panic!("Expected text segment"),
        }

        // 2番目のアイコン
        match &segments[2] {
            TextSegment::Icon { code, .. } => assert_eq!(code, "a"),
            _ => panic!("Expected icon segment"),
        }

        // 最後のテキスト
        match &segments[3] {
            TextSegment::Text(text) => assert_eq!(text, "：効果2を発動する。"),
            _ => panic!("Expected text segment"),
        }
    }

    #[test]
    fn test_decode_same_icon_multiple_times() {
        let encoded = "[c]：あなたのデッキから[c]能力を持つシグニを探す。";
        let segments = decode_skill_text(encoded);

        assert_eq!(segments.len(), 4);

        // 1番目と3番目がアイコン
        match &segments[0] {
            TextSegment::Icon { code, .. } => assert_eq!(code, "c"),
            _ => panic!("Expected icon segment"),
        }

        match &segments[2] {
            TextSegment::Icon { code, .. } => assert_eq!(code, "c"),
            _ => panic!("Expected icon segment"),
        }
    }

    #[test]
    fn test_decode_unknown_code() {
        let encoded = "[unknown]：未知のコード";
        let segments = decode_skill_text(encoded);

        assert_eq!(segments.len(), 2);

        // 未知のコードはテキストとして扱われる
        match &segments[0] {
            TextSegment::Text(text) => assert_eq!(text, "[unknown]"),
            _ => panic!("Expected text segment"),
        }
    }

    #[test]
    fn test_decode_incomplete_code() {
        let encoded = "[c：閉じ括弧なし";
        let segments = decode_skill_text(encoded);

        assert_eq!(segments.len(), 1);

        // 不完全なコードはテキストとして扱われる
        match &segments[0] {
            TextSegment::Text(text) => assert_eq!(text, "[c：閉じ括弧なし"),
            _ => panic!("Expected text segment"),
        }
    }

    #[test]
    fn test_decode_empty_text() {
        let segments = decode_skill_text("");
        assert!(segments.is_empty());
    }

    #[test]
    fn test_decode_text_without_icons() {
        let encoded = "このテキストにはアイコンがありません。";
        let segments = decode_skill_text(encoded);

        assert_eq!(segments.len(), 1);

        match &segments[0] {
            TextSegment::Text(text) => assert_eq!(text, "このテキストにはアイコンがありません。"),
            _ => panic!("Expected text segment"),
        }
    }

    #[test]
    fn test_decode_burst_text() {
        let encoded = "[lb2]：[gi]を1つ追加する。";
        let segments = decode_burst_text(encoded);

        assert_eq!(segments.len(), 4);

        match &segments[0] {
            TextSegment::Icon { code, component } => {
                assert_eq!(code, "lb2");
                assert_eq!(component, "IconLifeBurst2");
            }
            _ => panic!("Expected icon segment"),
        }
    }

    #[test]
    fn test_get_component_name() {
        assert_eq!(get_component_name("c"), Some("IconCip".to_string()));
        assert_eq!(get_component_name("a"), Some("IconAuto".to_string()));
        assert_eq!(get_component_name("unknown"), None);
    }

    #[test]
    fn test_get_original_pattern() {
        assert_eq!(get_original_pattern("c"), Some("【出】".to_string()));
        assert_eq!(get_original_pattern("p"), Some("【自】".to_string()));
        assert_eq!(get_original_pattern("unknown"), None);
    }

    #[test]
    fn test_get_available_icons() {
        let icons = get_available_icons();
        assert!(!icons.is_empty());

        // 基本的なアイコンが含まれていることを確認
        let cip_icon = icons.iter().find(|(code, _, _)| code == "c");
        assert!(cip_icon.is_some());

        let (_, component, pattern) = cip_icon.unwrap();
        assert_eq!(component, "IconCip");
        assert_eq!(pattern, "【出】");
    }

    // #[test]
    // fn test_decode_complex_text() {
    //     let encoded = "効果：[c]で場に出た時、[gi]を無視して[as]でアタックできる。[cr]：コイン1を支払い[gr]する。";
    //     let segments = decode_skill_text(encoded);
    //
    //     // セグメント数をチェック
    //     assert_eq!(segments.len(), 11);
    //
    //     // アイコンが正しく復号化されているかチェック
    //     let icon_codes: Vec<String> = segments.iter()
    //         .filter_map(|segment| match segment {
    //             TextSegment::Icon { code, .. } => Some(code.clone()),
    //             _ => None,
    //         })
    //         .collect();
    //
    //     assert_eq!(icon_codes, vec!["c", "g", "as", "cr", "gr"]);
    // }
}
