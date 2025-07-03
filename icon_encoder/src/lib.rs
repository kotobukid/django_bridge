//! # Icon Encoder
//!
//! WIXOSSカードゲームのスキルテキスト内のアイコン記号を符号化・復号化するライブラリ。
//!
//! ## 概要
//!
//! このライブラリは以下の機能を提供します：
//!
//! - **符号化** (Encoding): アイコンパターン（例：`【出】`）を一時符号（例：`[c]`）に変換
//! - **復号化** (Decoding): 符号化されたテキストをパースして、アイコンコンポーネント情報を抽出
//!
//! ## 使用例
//!
//! ### Static Generator での符号化
//!
//! ```rust
//! use icon_encoder::encode_skill_text;
//!
//! let original = "【出】：あなたのデッキの上からカードを5枚見る。";
//! let encoded = encode_skill_text(original);
//! assert_eq!(encoded, "[c]：あなたのデッキの上からカードを5枚見る。");
//! ```
//!
//! ### WASM Frontend での復号化
//!
//! ```rust
//! use icon_encoder::{decode_skill_text, TextSegment};
//!
//! let encoded = "[c]：効果を発動する。";
//! let segments = decode_skill_text(encoded);
//!
//! for segment in segments {
//!     match segment {
//!         TextSegment::Text(text) => {
//!             // プレーンテキストをレンダリング
//!             println!("Text: {}", text);
//!         },
//!         TextSegment::Icon { code, component } => {
//!             // アイコンコンポーネントをレンダリング
//!             println!("Icon: {} ({})", code, component);
//!         }
//!     }
//! }
//! ```
//!
//! ## アーキテクチャ
//!
//! ```text
//! [Static Generator] ──encode_skill_text()──> [Encoded Data] ──decode_skill_text()──> [WASM Frontend]
//!                                                     │
//!                                               【出】→[c]→IconCip
//! ```

pub mod decoder;
pub mod definitions;
pub mod encoder;

// Public re-exports for convenience
pub use decoder::{
    decode_burst_text, decode_skill_text, get_available_icons, get_original_pattern, TextSegment,
};
pub use definitions::{IconRule, ICON_RULES};
pub use encoder::{encode_burst_text, encode_multiple_texts, encode_skill_text};

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_encode_decode_roundtrip() {
        let original_texts = vec![
            "【出】：効果を発動する。",
            "【自】：毎ターン発動。【起】：手動で発動。",
            "【アサシン】と【ガード】を持つ。",
        ];

        for original in original_texts {
            // エンコード
            let encoded = encode_skill_text(original);

            // デコード
            let segments = decode_skill_text(&encoded);

            // 復元されたテキストを構築
            let mut restored = String::new();
            for segment in segments {
                match segment {
                    TextSegment::Text(text) => restored.push_str(&text),
                    TextSegment::Icon { code, .. } => {
                        if let Some(pattern) = get_original_pattern(&code) {
                            restored.push_str(&pattern);
                        } else {
                            restored.push_str(&format!("[{code}]"));
                        }
                    }
                }
            }

            assert_eq!(original, restored, "Round-trip failed for: {original}");
        }
    }

    #[test]
    fn test_all_defined_icons_work() {
        for rule in ICON_RULES {
            // エンコードテスト
            let test_text = format!("テスト{}テスト", rule.pattern);
            let encoded = encode_skill_text(&test_text);
            let expected_encoded = format!("テスト{}テスト", rule.code);
            assert_eq!(
                encoded, expected_encoded,
                "Encoding failed for: {}",
                rule.pattern
            );

            // デコードテスト
            let segments = decode_skill_text(&encoded);
            assert!(
                segments.len() >= 3,
                "Expected at least 3 segments for: {}",
                rule.pattern
            );

            // 中央のセグメントがアイコンであることを確認
            match &segments[1] {
                TextSegment::Icon { component, .. } => {
                    assert_eq!(
                        component, rule.component,
                        "Component mismatch for: {}",
                        rule.pattern
                    );
                }
                _ => panic!("Expected icon segment for: {}", rule.pattern),
            }
        }
    }

    #[test]
    fn test_complex_mixed_content() {
        let complex_text = "【自】：《ガードアイコン》を無視。";

        // エンコード
        let encoded = encode_skill_text(complex_text);
        assert_eq!(encoded, "[p]：[gi]を無視。");

        // デコード
        let segments = decode_skill_text(&encoded);

        // アイコンセグメントの数を確認
        let icon_count = segments
            .iter()
            .filter(|s| matches!(s, TextSegment::Icon { .. }))
            .count();
        assert_eq!(icon_count, 2, "Expected 5 icons in complex text");

        // 各アイコンの内容を確認
        let icons: Vec<String> = segments
            .iter()
            .filter_map(|s| match s {
                TextSegment::Icon { code, .. } => Some(code.clone()),
                _ => None,
            })
            .collect();
        assert_eq!(icons, vec!["p", "gi"]);
    }

    #[test]
    fn test_edge_cases() {
        // 空文字列
        assert_eq!(encode_skill_text(""), "");
        assert!(decode_skill_text("").is_empty());

        // アイコンなしのテキスト
        let no_icons = "このテキストにはアイコンがありません。";
        assert_eq!(encode_skill_text(no_icons), no_icons);

        let segments = decode_skill_text(no_icons);
        assert_eq!(segments.len(), 1);
        match &segments[0] {
            TextSegment::Text(text) => assert_eq!(text, no_icons),
            _ => panic!("Expected text segment"),
        }

        // アイコンのみ
        let icon_only = "【出】";
        let encoded = encode_skill_text(icon_only);
        assert_eq!(encoded, "[c]");

        let segments = decode_skill_text(&encoded);
        assert_eq!(segments.len(), 1);
        match &segments[0] {
            TextSegment::Icon { code, component } => {
                assert_eq!(code, "c");
                assert_eq!(component, "IconCip");
            }
            _ => panic!("Expected icon segment"),
        }
    }

    #[test]
    fn test_get_available_icons_completeness() {
        let available = get_available_icons();

        // 定義されたルール数と一致することを確認
        assert_eq!(available.len(), ICON_RULES.len());

        // 基本的なアイコンが含まれていることを確認
        let codes: Vec<String> = available.iter().map(|(code, _, _)| code.clone()).collect();
        assert!(codes.contains(&"c".to_string())); // 【出】
        assert!(codes.contains(&"a".to_string())); // 【自】
        assert!(codes.contains(&"gi".to_string())); // 【ガード】
    }
}
