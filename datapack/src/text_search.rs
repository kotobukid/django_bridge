use crate::CardExport;

/// 入力文字の特性を表す列挙型
#[derive(Debug, Clone, PartialEq)]
pub enum InputType {
    /// 英数字とハイフンのみ（カードコード向け）
    AlphaNumeric,
    /// カタカナのみ（読み向け）
    Katakana,
    /// ひらがな含む（カタカナ変換して読み向け）
    Hiragana,
    /// 混在（全フィールド検索）
    Mixed,
    /// 空文字列
    Empty,
}

/// 検索対象フィールドを指定する構造体
#[derive(Debug, Clone, PartialEq)]
pub struct SearchFields {
    pub name: bool,
    pub code: bool,
    pub pronunciation: bool,
}

impl SearchFields {
    pub fn all() -> Self {
        Self { name: true, code: true, pronunciation: true }
    }
    
    pub fn name_only() -> Self {
        Self { name: true, code: false, pronunciation: false }
    }
    
    pub fn code_and_name() -> Self {
        Self { name: true, code: true, pronunciation: false }
    }
    
    pub fn pronunciation_and_name() -> Self {
        Self { name: true, code: false, pronunciation: true }
    }
}

/// 文字列を正規化する関数
/// - 全角英数字を半角に変換
/// - ひらがなをカタカナに変換
/// - 連続するスペースを単一のスペースに圧縮
/// - 前後の空白を削除
pub fn normalize_text(input: &str) -> String {
    let mut result = String::new();
    let mut prev_was_space = false;
    
    for ch in input.chars() {
        match ch {
            // 全角英数字を半角に変換
            'Ａ'..='Ｚ' => {
                result.push(char::from(ch as u8 - 'Ａ' as u8 + b'A'));
                prev_was_space = false;
            }
            'ａ'..='ｚ' => {
                result.push(char::from(ch as u8 - 'ａ' as u8 + b'a'));
                prev_was_space = false;
            }
            '０'..='９' => {
                result.push(char::from(ch as u8 - '０' as u8 + b'0'));
                prev_was_space = false;
            }
            // ひらがなをカタカナに変換（包括的処理）
            ch if ch >= 'あ' && ch <= 'ゖ' => {
                let katakana_code = ch as u32 - 'あ' as u32 + 'ア' as u32;
                if let Some(katakana_char) = char::from_u32(katakana_code) {
                    result.push(katakana_char);
                } else {
                    result.push(ch);
                }
                prev_was_space = false;
            }
            // スペース処理（全角・半角両対応、連続圧縮）
            ' ' | '　' => {
                if !prev_was_space && !result.is_empty() {
                    result.push(' ');
                    prev_was_space = true;
                }
            }
            // その他の文字はそのまま
            _ => {
                result.push(ch);
                prev_was_space = false;
            }
        }
    }
    
    // 最後がスペースの場合は削除
    result.trim_end().to_string()
}

/// キーワードを分割する関数
/// スペース（半角・全角）で分割し、空文字列を除去
pub fn split_keywords(normalized_text: &str) -> Vec<String> {
    normalized_text
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

/// 入力文字列の特性を判定する関数
pub fn detect_input_type(input: &str) -> InputType {
    if input.trim().is_empty() {
        return InputType::Empty;
    }
    
    let mut has_alphanumeric = false;
    let mut has_katakana = false;
    let mut has_hiragana = false;
    let mut has_other = false;
    
    for ch in input.chars() {
        match ch {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' => has_alphanumeric = true,
            'Ａ'..='Ｚ' | 'ａ'..='ｚ' | '０'..='９' => has_alphanumeric = true,
            'ア'..='ン' | 'ァ'..='ヶ' => has_katakana = true,
            'あ'..='ん' | 'ぁ'..='ゖ' => has_hiragana = true,
            ' ' | '　' => {}, // スペースは無視
            _ => has_other = true,
        }
    }
    
    if has_hiragana {
        InputType::Hiragana
    } else if has_other || (has_alphanumeric && has_katakana) {
        InputType::Mixed
    } else if has_katakana {
        InputType::Katakana
    } else if has_alphanumeric {
        InputType::AlphaNumeric
    } else {
        InputType::Mixed
    }
}

/// 入力特性に基づいて検索対象フィールドを決定する関数
pub fn determine_search_fields(input_type: &InputType) -> SearchFields {
    match input_type {
        InputType::Empty => SearchFields::all(), // 空の場合は全フィールド
        InputType::AlphaNumeric => SearchFields::code_and_name(), // 英数字はcodeとname
        InputType::Katakana => SearchFields::pronunciation_and_name(), // カタカナは読みとname
        InputType::Hiragana => SearchFields::pronunciation_and_name(), // ひらがなは読みとname
        InputType::Mixed => SearchFields::all(), // 混在は全フィールド
    }
}

/// 単一フィールドに対してキーワードマッチングを行う関数
/// 全てのキーワードが含まれている場合にtrueを返す（AND条件）
pub fn field_matches_keywords(field_value: &str, keywords: &[String]) -> bool {
    if keywords.is_empty() {
        return true;
    }
    
    let normalized_field = normalize_text(field_value).to_lowercase();
    
    keywords.iter().all(|keyword| {
        let normalized_keyword = keyword.to_lowercase();
        normalized_field.contains(&normalized_keyword)
    })
}

/// カードがテキスト検索条件にマッチするかを判定する関数
/// 指定されたフィールドのいずれかがマッチすればtrueを返す（OR条件）
pub fn card_matches_text_search(
    card: &CardExport,
    keywords: &[String],
    search_fields: &SearchFields,
) -> bool {
    if keywords.is_empty() {
        return true;
    }
    
    let mut matches = false;
    
    if search_fields.name {
        matches |= field_matches_keywords(&card.name(), keywords);
    }
    
    if search_fields.code {
        matches |= field_matches_keywords(&card.code(), keywords);
    }
    
    if search_fields.pronunciation {
        matches |= field_matches_keywords(&card.pronunciation(), keywords);
    }
    
    matches
}

/// テキスト検索を実行する包括的な関数
/// 入力の正規化、特性判定、検索フィールド決定、マッチング処理を一括実行
pub fn search_cards_by_text_optimized(
    cards: &[CardExport],
    search_text: &str,
) -> Vec<CardExport> {
    // 入力が空の場合は全カードを返す
    if search_text.trim().is_empty() {
        return cards.to_vec();
    }
    
    // 1. 入力を正規化
    let normalized_input = normalize_text(search_text);
    
    // 2. キーワードに分割
    let keywords = split_keywords(&normalized_input);
    
    // 3. 入力特性を判定
    let input_type = detect_input_type(&normalized_input);
    
    // 4. 検索対象フィールドを決定
    let search_fields = determine_search_fields(&input_type);
    
    // 5. フィルタリング実行
    cards
        .iter()
        .filter(|card| card_matches_text_search(card, &keywords, &search_fields))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_text() {
        // 全角英数字を半角に変換
        assert_eq!(normalize_text("ＡＢＣ１２３"), "ABC123");
        
        // ひらがなをカタカナに変換
        assert_eq!(normalize_text("あいうえお"), "アイウエオ");
        assert_eq!(normalize_text("がぎぐげご"), "ガギグゲゴ");
        assert_eq!(normalize_text("ばびぶべぼ"), "バビブベボ");
        
        // スペース正規化
        assert_eq!(normalize_text("hello　　world"), "hello world");
        assert_eq!(normalize_text("  trim  test  "), "trim test");
        
        // 混在パターン
        assert_eq!(normalize_text("Ａｂｃ　あいう　１２３"), "Abc アイウ 123");
    }

    #[test]
    fn test_split_keywords() {
        assert_eq!(split_keywords("hello world"), vec!["hello", "world"]);
        assert_eq!(split_keywords("one"), vec!["one"]);
        assert_eq!(split_keywords(""), Vec::<String>::new());
        assert_eq!(split_keywords("  space  test  "), vec!["space", "test"]);
    }

    #[test]
    fn test_detect_input_type() {
        assert_eq!(detect_input_type("ABC123"), InputType::AlphaNumeric);
        assert_eq!(detect_input_type("WX24-P1"), InputType::AlphaNumeric);
        assert_eq!(detect_input_type("アイウエオ"), InputType::Katakana);
        assert_eq!(detect_input_type("あいうえお"), InputType::Hiragana);
        assert_eq!(detect_input_type("ABC アイウ"), InputType::Mixed);
        assert_eq!(detect_input_type(""), InputType::Empty);
        assert_eq!(detect_input_type("  "), InputType::Empty);
    }

    #[test]
    fn test_determine_search_fields() {
        let alpha_fields = determine_search_fields(&InputType::AlphaNumeric);
        assert!(alpha_fields.code && alpha_fields.name && !alpha_fields.pronunciation);
        
        let katakana_fields = determine_search_fields(&InputType::Katakana);
        assert!(!katakana_fields.code && katakana_fields.name && katakana_fields.pronunciation);
        
        let mixed_fields = determine_search_fields(&InputType::Mixed);
        assert!(mixed_fields.code && mixed_fields.name && mixed_fields.pronunciation);
    }

    #[test]
    fn test_field_matches_keywords() {
        let keywords = vec!["test".to_string(), "card".to_string()];
        
        assert!(field_matches_keywords("This is a test card", &keywords));
        assert!(field_matches_keywords("CARD TEST NAME", &keywords));
        assert!(!field_matches_keywords("This is a test", &keywords)); // "card"が含まれない
        assert!(field_matches_keywords("Test Card", &keywords));
        
        // 空のキーワード
        assert!(field_matches_keywords("any text", &[]));
    }

    #[test]
    fn test_card_matches_text_search() {
        let card = CardExport::from(&(
            1,                    // id
            "Test Card Name",     // name
            "WX24-001",          // code
            "テストカード",        // pronunciation
            0,                   // color
            "1",                 // cost
            "1",                 // level
            "3",                 // limit
            "",                  // limit_ex
            "1000",              // power
            0,                   // has_burst
            "Test skill",        // skill_text
            "",                  // burst_text
            1,                   // format
            "",                  // story
            "C",                 // rarity
            "",                  // url
            5,                   // card_type (Signi)
            1,                   // product
            0,                   // timing
            0,                   // feature_bits1
            0,                   // feature_bits2
            0,                   // klass_bits
            0,                   // burst_bits
            "",                  // ex1
        ));

        let keywords = vec!["Test".to_string()];
        
        // name検索
        let name_fields = SearchFields { name: true, code: false, pronunciation: false };
        assert!(card_matches_text_search(&card, &keywords, &name_fields));
        
        // code検索
        let code_fields = SearchFields { name: false, code: true, pronunciation: false };
        assert!(!card_matches_text_search(&card, &keywords, &code_fields)); // codeに"Test"は含まれない
        
        // pronunciation検索
        let keywords_jp = vec!["テスト".to_string()];
        let pronunciation_fields = SearchFields { name: false, code: false, pronunciation: true };
        assert!(card_matches_text_search(&card, &keywords_jp, &pronunciation_fields));
    }

    #[test]
    fn test_search_cards_by_text_optimized() {
        let cards = vec![
            CardExport::from(&(
                1, "Fire Dragon", "WX24-001", "ファイアドラゴン", 0, "3", "2", "3", "", "2000", 0, "", "", 1, "", "R", "", 5, 1, 0, 0, 0, 0, 0, "",
            )),
            CardExport::from(&(
                2, "Water Spirit", "WX24-002", "ウォータースピリット", 0, "2", "1", "2", "", "1500", 0, "", "", 1, "", "C", "", 5, 1, 0, 0, 0, 0, 0, "",
            )),
        ];

        // 英語名での検索
        let result = search_cards_by_text_optimized(&cards, "Fire");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name(), "Fire Dragon");

        // カタカナでの検索
        let result = search_cards_by_text_optimized(&cards, "ファイア");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name(), "Fire Dragon");

        // コードでの検索
        let result = search_cards_by_text_optimized(&cards, "WX24-002");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name(), "Water Spirit");

        // 複数キーワード
        let result = search_cards_by_text_optimized(&cards, "Water Spirit");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name(), "Water Spirit");

        // マッチしない検索
        let result = search_cards_by_text_optimized(&cards, "NonExistent");
        assert_eq!(result.len(), 0);

        // 空文字列
        let result = search_cards_by_text_optimized(&cards, "");
        assert_eq!(result.len(), 2);
    }
}