#[cfg(test)]
mod regex_tests {
    use regex::Regex;

    #[test]
    fn test_rust_regex_with_fullwidth_digits() {
        // Test cases
        let test_cases = vec![
            ("カードを２枚引く", r"カードを\d+枚引く", "Rust \\d+"),
            ("カードを２枚引く", r"カードを[0-9]+枚引く", "Rust [0-9]+"),
            ("カードを２枚引く", r"カードを[０-９]+枚引く", "Rust [０-９]+"),
            ("【エナチャージ１】", r"エナチャージ\d+", "Rust \\d+ for エナチャージ"),
            ("【エナチャージ１】", r"エナチャージ[0-9]+", "Rust [0-9]+ for エナチャージ"),
            ("【エナチャージ１】", r"エナチャージ[０-９]+", "Rust [０-９]+ for エナチャージ"),
        ];

        println!("\nTesting Rust regex patterns with full-width numbers:");
        println!("{}", "=".repeat(60));

        for (text, pattern, description) in test_cases {
            let re = Regex::new(pattern).unwrap();
            let has_match = re.is_match(text);
            
            println!("\nText: '{}'", text);
            println!("Pattern: '{}'", pattern);
            println!("Description: {}", description);
            println!("Match: {}", has_match);
            
            if let Some(m) = re.find(text) {
                println!("Matched text: '{}'", m.as_str());
            }
        }
    }

    #[test]
    fn test_individual_digit_matching() {
        let digit_re = Regex::new(r"\d").unwrap();
        let ascii_digit_re = Regex::new(r"[0-9]").unwrap();
        
        // Note: Rust's \d DOES match full-width digits
        assert!(digit_re.is_match("２"), "\\d DOES match full-width ２");
        assert!(!ascii_digit_re.is_match("２"), "[0-9] should NOT match full-width ２");
        
        // Test half-width digit
        assert!(digit_re.is_match("2"), "\\d should match half-width 2");
        assert!(ascii_digit_re.is_match("2"), "[0-9] should match half-width 2");
    }

    #[test]
    fn test_feature_detection_with_fullwidth() {
        // Test the actual pattern from feature detection
        let pattern = Regex::new(r"カードを\d+枚引").unwrap();
        
        let fullwidth_text = "カードを２枚引く";
        let halfwidth_text = "カードを2枚引く";
        
        // Note: Rust's \d DOES match full-width digits, so both should match
        assert!(pattern.is_match(fullwidth_text), "Pattern DOES match full-width text because \\d matches full-width digits");
        assert!(pattern.is_match(halfwidth_text), "Pattern should match half-width text");
    }

    #[test]
    fn test_to_half_conversion() {
        use analyzer::raw_card_analyzer::to_half;
        
        // Test number conversion
        assert_eq!(to_half("０１２３４５６７８９"), "0123456789");
        assert_eq!(to_half("カードを２枚引く"), "カードを2枚引く");
        assert_eq!(to_half("【エナチャージ１】"), "【エナチャージ1】");
        
        // Test with feature detection pattern
        let pattern = Regex::new(r"カードを\d+枚引").unwrap();
        let original = "カードを２枚引く";
        let converted = to_half(original);
        
        // Both should match because Rust's \d matches full-width digits
        assert!(pattern.is_match(original), "Pattern DOES match original full-width text");
        assert!(pattern.is_match(&converted), "Pattern should match converted half-width text");
    }
}