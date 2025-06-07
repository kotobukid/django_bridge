use std::collections::HashSet;
use regex::Regex;

/// A trait for rules that analyze text and detect patterns
pub trait AnalyzeRule<T> {
    /// Detects patterns in the given text and returns a HashSet of found items
    fn detect(&self, text: &str) -> HashSet<T>;
}

/// A rule that detects individual digits in text and returns them as i32
pub struct DigitDetectRule {
    pattern: Regex,
}

impl DigitDetectRule {
    pub fn new() -> Self {
        Self {
            pattern: Regex::new(r"\d").unwrap(),
        }
    }
}

impl AnalyzeRule<i32> for DigitDetectRule {
    fn detect(&self, text: &str) -> HashSet<i32> {
        let mut result = HashSet::new();

        for cap in self.pattern.find_iter(text) {
            if let Ok(digit) = cap.as_str().parse::<i32>() {
                result.insert(digit);
            }
        }

        result
    }
}

/// A rule that detects even digits in text
pub struct EvenNumberRule {
    pattern: Regex,
}

impl EvenNumberRule {
    pub fn new() -> Self {
        Self {
            // Use \d to match individual digits
            pattern: Regex::new(r"\d").unwrap(),
        }
    }
}

impl AnalyzeRule<i32> for EvenNumberRule {
    fn detect(&self, text: &str) -> HashSet<i32> {
        let mut result = HashSet::new();

        for cap in self.pattern.find_iter(text) {
            if let Ok(digit) = cap.as_str().parse::<i32>() {
                // Check if the digit is even (0, 2, 4, 6, 8)
                if digit % 2 == 0 {
                    result.insert(digit);
                }
            }
        }

        result
    }
}

/// A rule that detects words with a specific length
pub struct WordLengthRule {
    pattern: Regex,
    length: usize,
}

impl WordLengthRule {
    pub fn new(length: usize) -> Self {
        Self {
            pattern: Regex::new(r"\b\w+\b").unwrap(),
            length,
        }
    }
}

impl AnalyzeRule<String> for WordLengthRule {
    fn detect(&self, text: &str) -> HashSet<String> {
        let mut result = HashSet::new();

        for cap in self.pattern.find_iter(text) {
            let word = cap.as_str();
            if word.len() == self.length {
                result.insert(word.to_string());
            }
        }

        result
    }
}

/// An analyzer that uses multiple rules to analyze text
pub struct Analyzer<T> {
    rules: Vec<Box<dyn AnalyzeRule<T>>>,
}

impl<T> Analyzer<T> {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: Box<dyn AnalyzeRule<T>>) {
        self.rules.push(rule);
    }

    pub fn analyze(&self, text: &str) -> HashSet<T> 
    where 
        T: Clone + Eq + std::hash::Hash,
    {
        let mut result = HashSet::new();

        for rule in &self.rules {
            let detected = rule.detect(text);
            result.extend(detected);
        }

        result
    }
}

/// Example usage of the analyzer
pub fn example_digit_analysis(text: &str) -> HashSet<i32> {
    let mut analyzer = Analyzer::new();
    analyzer.add_rule(Box::new(DigitDetectRule::new()));
    analyzer.analyze(text)
}

/// A rule that detects email addresses in text
pub struct EmailDetectRule {
    pattern: Regex,
}

impl EmailDetectRule {
    pub fn new() -> Self {
        Self {
            // Simple email regex pattern
            pattern: Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b").unwrap(),
        }
    }
}

impl AnalyzeRule<String> for EmailDetectRule {
    fn detect(&self, text: &str) -> HashSet<String> {
        let mut result = HashSet::new();

        for cap in self.pattern.find_iter(text) {
            result.insert(cap.as_str().to_string());
        }

        result
    }
}

/// Example usage with multiple rules
pub fn example_number_analysis(text: &str) -> HashSet<i32> {
    let mut analyzer = Analyzer::new();
    analyzer.add_rule(Box::new(DigitDetectRule::new()));
    analyzer.add_rule(Box::new(EvenNumberRule::new()));
    analyzer.analyze(text)
}

/// Example usage for email detection
pub fn example_email_analysis(text: &str) -> HashSet<String> {
    let mut analyzer = Analyzer::new();
    analyzer.add_rule(Box::new(EmailDetectRule::new()));
    analyzer.analyze(text)
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_detection() {
        let mut analyzer = Analyzer::new();
        analyzer.add_rule(Box::new(DigitDetectRule::new()));

        let flags: HashSet<i32> = analyzer.analyze("019834");

        assert!(flags.contains(&0));
        assert!(flags.contains(&1));
        assert!(flags.contains(&3));
        assert!(flags.contains(&4));
        assert!(flags.contains(&8));
        assert!(flags.contains(&9));
    }

    #[test]
    fn test_even_number_detection() {
        let mut analyzer = Analyzer::new();
        analyzer.add_rule(Box::new(EvenNumberRule::new()));

        let flags: HashSet<i32> = analyzer.analyze("123 456 789");

        assert!(flags.contains(&2));
        assert!(flags.contains(&4));
        assert!(flags.contains(&6));
        assert!(flags.contains(&8));
        assert!(!flags.contains(&1));
        assert!(!flags.contains(&3));
    }

    #[test]
    fn test_word_length_detection() {
        let mut analyzer = Analyzer::new();
        analyzer.add_rule(Box::new(WordLengthRule::new(3)));

        let words: HashSet<String> = analyzer.analyze("The quick brown fox jumps over the lazy dog");

        assert!(words.contains("The"));
        assert!(words.contains("fox"));
        assert!(words.contains("the"));
        assert!(!words.contains("quick"));
        assert!(!words.contains("brown"));
    }

    #[test]
    fn test_email_detection() {
        let mut analyzer = Analyzer::new();
        analyzer.add_rule(Box::new(EmailDetectRule::new()));

        let emails: HashSet<String> = analyzer.analyze("Contact us at test@example.com or support@company.org for more information.");

        assert!(emails.contains("test@example.com"));
        assert!(emails.contains("support@company.org"));
        assert_eq!(emails.len(), 2);

        // Test with no emails
        let empty: HashSet<String> = analyzer.analyze("This text contains no email addresses.");
        assert_eq!(empty.len(), 0);
    }
}
