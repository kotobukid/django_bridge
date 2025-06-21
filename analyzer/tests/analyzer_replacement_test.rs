use analyzer::card_analyzer::SimpleRawCardAnalyzer;
use analyzer::raw_card_analyzer::RawCardAnalyzer;
use models::r#gen::django_models::RawCardDb;
use chrono::Utc;

#[tokio::test]
async fn test_analyzer_applies_replacements() {
    let analyzer = SimpleRawCardAnalyzer::new();
    
    // Create a test RawCard with self-assassin text
    let raw_card = RawCardDb {
        id: 1,
        card_number: "TEST-001".to_string(),
        name: "テストカード".to_string(),
        raw_html: r#"<dt>カード種類</dt><dd>シグニ</dd>"#.to_string(),
        skill_text: "【自】：このシグニがアタックしたとき、（このシグニがアタックすると正面のシグニとバトルをせず対戦相手にダメージを与える）。追加で【エナチャージ1】をする。".to_string(),
        life_burst_text: "".to_string(),
        source_url: "http://example.com".to_string(),
        scraped_at: Utc::now(),
        last_analyzed_at: None,
        is_analyzed: false,
        analysis_error: "".to_string(),
    };
    
    // Analyze the card
    let result = analyzer.analyze(&raw_card).await.unwrap();
    
    // Check that the replacement was applied
    assert!(result.skill_text.is_some());
    let skill_text = result.skill_text.unwrap();
    
    println!("Original skill text: {}", raw_card.skill_text);
    println!("Processed skill text: {}", skill_text);
    
    // The text should contain the replacement marker
    assert!(skill_text.contains("*SELF ASSASSIN*"));
    
    // The original parenthetical text should be replaced
    assert!(!skill_text.contains("このシグニがアタックすると正面のシグニとバトルをせず対戦相手にダメージを与える"));
    
    // Other parts should be preserved
    assert!(skill_text.contains("【自】:このシグニがアタックしたとき、"));
    assert!(skill_text.contains("。追加で【エナチャージ1】をする。"));
}

#[tokio::test]
async fn test_analyzer_multiple_replacements() {
    let analyzer = SimpleRawCardAnalyzer::new();
    
    // Create a test RawCard with multiple patterns to replace
    let raw_card = RawCardDb {
        id: 2,
        card_number: "TEST-002".to_string(),
        name: "テストカード2".to_string(),
        raw_html: r#"<dt>カード種類</dt><dd>シグニ</dd>"#.to_string(),
        skill_text: "【アサシン】（【アサシン】を持つシグニがアタックすると正面のシグニとバトルをせず対戦相手にダメージを与える）".to_string(),
        life_burst_text: "【ライフバースト】：（このシグニがアタックすると正面のシグニとバトルをせず対戦相手にダメージを与える）".to_string(),
        source_url: "http://example.com".to_string(),
        scraped_at: Utc::now(),
        last_analyzed_at: None,
        is_analyzed: false,
        analysis_error: "".to_string(),
    };
    
    // Analyze the card
    let result = analyzer.analyze(&raw_card).await.unwrap();
    
    // Check skill text replacement
    let skill_text = result.skill_text.unwrap();
    println!("Skill text: {}", skill_text);
    assert!(skill_text.contains("*ASSASSIN*"));
    assert!(skill_text.contains("【アサシン】"));
    
    // Check burst text replacement
    let burst_text = result.burst_text.unwrap();
    println!("Burst text: {}", burst_text);
    assert!(burst_text.contains("*SELF ASSASSIN*"));
    assert!(burst_text.contains("【ライフバースト】:"));
}