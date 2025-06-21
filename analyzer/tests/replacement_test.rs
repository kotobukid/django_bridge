use feature::create_detect_patterns;

#[test]
fn test_replacement_patterns() {
    let (replace_patterns, _) = create_detect_patterns();
    
    // Test case 1: Self assassin pattern (with half-width parentheses after to_half)
    let test_text1 = "(このシグニがアタックすると正面のシグニとバトルをせず対戦相手にダメージを与える)";
    let mut found_self_assassin = false;
    
    for pattern in &replace_patterns {
        if pattern.pattern_r.is_match(test_text1) {
            println!("Pattern '{}' matched for self-assassin text", pattern.pattern);
            println!("Replace to: '{}'", pattern.replace_to);
            
            let replaced = pattern.pattern_r.replace_all(test_text1, pattern.replace_to);
            println!("Result: '{}'", replaced);
            
            if pattern.replace_to == "*SELF ASSASSIN*" {
                found_self_assassin = true;
                assert_eq!(replaced, "*SELF ASSASSIN*");
            }
        }
    }
    
    assert!(found_self_assassin, "Self-assassin pattern should match and replace");
    
    // Test case 2: Assassin ability pattern
    let test_text2 = "(【アサシン】を持つシグニがアタックすると正面のシグニとバトルをせず対戦相手にダメージを与える)";
    let mut found_assassin = false;
    
    for pattern in &replace_patterns {
        if pattern.pattern_r.is_match(test_text2) {
            println!("\nPattern '{}' matched for assassin ability text", pattern.pattern);
            println!("Replace to: '{}'", pattern.replace_to);
            
            let replaced = pattern.pattern_r.replace_all(test_text2, pattern.replace_to);
            println!("Result: '{}'", replaced);
            
            if pattern.replace_to == "*ASSASSIN*" {
                found_assassin = true;
                assert_eq!(replaced, "*ASSASSIN*");
            }
        }
    }
    
    assert!(found_assassin, "Assassin ability pattern should match and replace");
    
    // Test case 3: Text with context (should preserve context)
    let test_text3 = "【自】：このシグニがアタックしたとき、(このシグニがアタックすると正面のシグニとバトルをせず対戦相手にダメージを与える)";
    
    for pattern in &replace_patterns {
        if pattern.pattern_r.is_match(test_text3) && pattern.replace_to == "*SELF ASSASSIN*" {
            let replaced = pattern.pattern_r.replace_all(test_text3, pattern.replace_to);
            println!("\nContext preservation test:");
            println!("Original: '{}'", test_text3);
            println!("Replaced: '{}'", replaced);
            
            assert!(replaced.contains("【自】：このシグニがアタックしたとき、"));
            assert!(replaced.contains("*SELF ASSASSIN*"));
        }
    }
}

#[test]
fn test_apply_all_replacements() {
    let (replace_patterns, _) = create_detect_patterns();
    
    // Function to apply all replacements to a text
    fn apply_replacements(text: &str, patterns: &[feature::ReplacePattern]) -> String {
        let mut result = text.to_string();
        for pattern in patterns {
            if pattern.pattern_r.is_match(&result) {
                result = pattern.pattern_r.replace_all(&result, pattern.replace_to).to_string();
            }
        }
        result
    }
    
    // Test with a complex skill text
    let skill_text = "【自】：このシグニがアタックしたとき、(このシグニがアタックすると正面のシグニとバトルをせず対戦相手にダメージを与える)。追加で【エナチャージ1】をする。";
    
    let replaced = apply_replacements(skill_text, &replace_patterns);
    
    println!("\nFull replacement test:");
    println!("Original: '{}'", skill_text);
    println!("Replaced: '{}'", replaced);
    
    assert!(replaced.contains("*SELF ASSASSIN*"));
    assert!(!replaced.contains("(このシグニがアタックすると正面のシグニとバトルをせず対戦相手にダメージを与える)"));
}