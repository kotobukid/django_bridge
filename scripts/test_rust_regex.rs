use regex::Regex;

fn main() {
    // Test cases
    let test_cases = vec![
        ("カードを２枚引く", r"カードを\d+枚引く", "Rust \\d+"),
        ("カードを２枚引く", r"カードを[0-9]+枚引く", "Rust [0-9]+"),
        ("カードを２枚引く", r"カードを[０-９]+枚引く", "Rust [０-９]+"),
        ("【エナチャージ１】", r"エナチャージ\d+", "Rust \\d+ for エナチャージ"),
        ("【エナチャージ１】", r"エナチャージ[0-9]+", "Rust [0-9]+ for エナチャージ"),
        ("【エナチャージ１】", r"エナチャージ[０-９]+", "Rust [０-９]+ for エナチャージ"),
    ];

    println!("Testing Rust regex patterns with full-width numbers:");
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

    // Test individual characters
    println!("\n\nDirect character tests:");
    println!("{}", "=".repeat(60));
    
    let digit_re = Regex::new(r"\d").unwrap();
    let ascii_digit_re = Regex::new(r"[0-9]").unwrap();
    
    println!("'２' matches \\d: {}", digit_re.is_match("２"));
    println!("'２' matches [0-9]: {}", ascii_digit_re.is_match("２"));
    println!("'2' matches \\d: {}", digit_re.is_match("2"));
    println!("'2' matches [0-9]: {}", ascii_digit_re.is_match("2"));
}