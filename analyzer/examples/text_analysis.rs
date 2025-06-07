use analyzer::{Analyzer, DigitDetectRule, EmailDetectRule, EvenNumberRule, WordLengthRule};
use std::collections::HashSet;

fn main() {
    let text = "The quick brown fox jumps over 42 lazy dogs in 2023. Contact us at example@email.com or support@example.org for more information.";

    println!("Analyzing text: \"{}\"", text);

    // Example 1: Detect all digits
    let mut digit_analyzer = Analyzer::new();
    digit_analyzer.add_rule(Box::new(DigitDetectRule::new()));
    let digits: HashSet<i32> = digit_analyzer.analyze(text);

    println!("\nDetected digits:");
    for digit in digits {
        println!("  - {}", digit);
    }

    // Example 2: Detect even numbers
    let mut even_analyzer = Analyzer::new();
    even_analyzer.add_rule(Box::new(EvenNumberRule::new()));
    let even_numbers: HashSet<i32> = even_analyzer.analyze(text);

    println!("\nDetected even numbers:");
    for number in even_numbers {
        println!("  - {}", number);
    }

    // Example 3: Detect words of specific lengths
    let mut word_analyzer = Analyzer::new();
    word_analyzer.add_rule(Box::new(WordLengthRule::new(3)));
    word_analyzer.add_rule(Box::new(WordLengthRule::new(4)));
    let words: HashSet<String> = word_analyzer.analyze(text);

    println!("\nDetected words with 3 or 4 characters:");
    for word in words {
        println!("  - {}", word);
    }

    // Example 4: Combining multiple rules of the same type
    let mut combined_analyzer = Analyzer::new();
    combined_analyzer.add_rule(Box::new(DigitDetectRule::new()));
    combined_analyzer.add_rule(Box::new(EvenNumberRule::new()));
    let numbers: HashSet<i32> = combined_analyzer.analyze(text);

    println!("\nAll detected numbers (digits and even numbers):");
    for number in numbers {
        println!("  - {}", number);
    }

    // Example 5: Detect email addresses
    let mut email_analyzer = Analyzer::new();
    email_analyzer.add_rule(Box::new(EmailDetectRule::new()));
    let emails: HashSet<String> = email_analyzer.analyze(text);

    println!("\nDetected email addresses:");
    for email in emails {
        println!("  - {}", email);
    }
}
