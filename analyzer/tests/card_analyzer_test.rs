use analyzer::card_analyzer::SimpleRawCardAnalyzer;

#[test]
fn test_level_detection_from_html() {
    let analyzer = SimpleRawCardAnalyzer::new();
    
    // ルリグカードのテスト用HTML
    let lrig_html = r#"
        <dt>カード種類</dt><dd>ルリグ</dd>
        <dt>クラス</dt><dd>ホシノ</dd>
        <dt>色</dt><dd>緑</dd>
        <dt>レベル</dt><dd>2</dd>
        <dt>グロウコスト</dt><dd>《緑》×１</dd>
        <dt>コスト</dt><dd>-</dd>
        <dt>リミット</dt><dd>5</dd>
    "#;
    
    let level = analyzer.detect_level_from_html(lrig_html);
    assert_eq!(level, Some("2".to_string()));
    
    // レベルが"-"の場合
    let no_level_html = r#"
        <dt>カード種類</dt><dd>アーツ</dd>
        <dt>クラス</dt><dd>-</dd>
        <dt>色</dt><dd>緑</dd>
        <dt>レベル</dt><dd>-</dd>
    "#;
    
    let no_level = analyzer.detect_level_from_html(no_level_html);
    assert_eq!(no_level, None);
}

#[test]
fn test_power_detection_from_html() {
    let analyzer = SimpleRawCardAnalyzer::new();
    
    // シグニカードのテスト用HTML
    let signi_html = r#"
        <dt>カード種類</dt><dd>シグニ</dd>
        <dt>クラス</dt><dd>奏武：ブルアカ</dd>
        <dt>色</dt><dd>緑</dd>
        <dt>レベル</dt><dd>1</dd>
        <dt>グロウコスト</dt><dd>-</dd>
        <dt>コスト</dt><dd>-</dd>
        <dt>リミット</dt><dd>-</dd>
        <dt>パワー</dt><dd>3000</dd>
    "#;
    
    let power = analyzer.detect_power_from_html(signi_html);
    assert_eq!(power, Some("3000".to_string()));
    
    // ルリグカード（パワーがないはず）
    let lrig_html = r#"
        <dt>カード種類</dt><dd>ルリグ</dd>
        <dt>クラス</dt><dd>ホシノ</dd>
        <dt>色</dt><dd>緑</dd>
        <dt>レベル</dt><dd>2</dd>
        <dt>グロウコスト</dt><dd>《緑》×１</dd>
        <dt>コスト</dt><dd>-</dd>
        <dt>リミット</dt><dd>5</dd>
        <dt>パワー</dt><dd>-</dd>
    "#;
    
    let lrig_power = analyzer.detect_power_from_html(lrig_html);
    assert_eq!(lrig_power, None); // ルリグはパワー検出対象外
}

#[test]
fn test_limit_detection_from_html() {
    let analyzer = SimpleRawCardAnalyzer::new();
    
    // ルリグカードのテスト用HTML
    let lrig_html = r#"
        <dt>カード種類</dt><dd>ルリグ</dd>
        <dt>クラス</dt><dd>ホシノ</dd>
        <dt>色</dt><dd>緑</dd>
        <dt>レベル</dt><dd>2</dd>
        <dt>グロウコスト</dt><dd>《緑》×１</dd>
        <dt>コスト</dt><dd>-</dd>
        <dt>リミット</dt><dd>5</dd>
    "#;
    
    let (limit, feature) = analyzer.detect_limit_from_html(lrig_html);
    assert_eq!(limit, Some("5".to_string()));
    
    // シグニカード（リミットがないはず）
    let signi_html = r#"
        <dt>カード種類</dt><dd>シグニ</dd>
        <dt>クラス</dt><dd>奏武：ブルアカ</dd>
        <dt>色</dt><dd>緑</dd>
        <dt>レベル</dt><dd>1</dd>
        <dt>グロウコスト</dt><dd>-</dd>
        <dt>コスト</dt><dd>-</dd>
        <dt>リミット</dt><dd>-</dd>
    "#;
    
    let (signi_limit, _feature) = analyzer.detect_limit_from_html(signi_html);
    assert_eq!(signi_limit, None); // シグニはリミット検出対象外
}

#[test]
fn test_timing_detection_from_html() {
    let analyzer = SimpleRawCardAnalyzer::new();
    
    // アーツカードのテスト用HTML
    let arts_html = r#"
        <dt>カード種類</dt><dd>アーツ</dd>
        <dt>クラス</dt><dd>-</dd>
        <dt>色</dt><dd>緑</dd>
        <dt>レベル</dt><dd>-</dd>
        <dt>グロウコスト</dt><dd>-</dd>
        <dt>コスト</dt><dd>《緑》×１</dd>
        <dt>リミット</dt><dd>-</dd>
        <dt>パワー</dt><dd>-</dd>
        <dt>リミット消費</dt><dd>-</dd>
        <dt>使用タイミング</dt><dd>メインフェイズ</dd>
    "#;
    
    let timing = analyzer.detect_timing_from_html(arts_html);
    assert_eq!(timing, Some("メインフェイズ".to_string()));
    
    // ピースカードのテスト
    let piece_html = r#"
        <dt>カード種類</dt><dd>ピース</dd>
        <dt>クラス</dt><dd>-</dd>
        <dt>色</dt><dd>赤</dd>
        <dt>レベル</dt><dd>-</dd>
        <dt>グロウコスト</dt><dd>-</dd>
        <dt>コスト</dt><dd>《赤》×１</dd>
        <dt>リミット</dt><dd>-</dd>
        <dt>パワー</dt><dd>-</dd>
        <dt>リミット消費</dt><dd>-</dd>
        <dt>使用タイミング</dt><dd>アタックフェイズ</dd>
    "#;
    
    let piece_timing = analyzer.detect_timing_from_html(piece_html);
    assert_eq!(piece_timing, Some("アタックフェイズ".to_string()));
}

#[test]
fn test_story_detection_from_html() {
    let analyzer = SimpleRawCardAnalyzer::new();
    
    // dissonaアイコンがある場合
    let dissona_html = r#"
        <dt>カード種類</dt><dd>シグニ</dd>
        <dt>クラス</dt><dd>奏武：ブルアカ</dd>
        <dt>色</dt><dd>緑</dd>
        <dt>レベル</dt><dd>1</dd>
        <dt>グロウコスト</dt><dd>-</dd>
        <dt>コスト</dt><dd>-</dd>
        <dt>リミット</dt><dd>-</dd>
        <dt>パワー</dt><dd>3000</dd>
        <dt>リミット消費</dt><dd>-</dd>
        <dt>使用タイミング</dt><dd>-</dd>
        <dt>その他</dt><dd>-</dd>
        <dt>ストーリー</dt><dd><img src="/path/to/icon_txt_dissona.png" alt="dissona"></dd>
    "#;
    
    let story = analyzer.detect_story_from_html(dissona_html);
    assert_eq!(story, Some("dissona".to_string()));
    
    // dissonaアイコンがない場合
    let no_story_html = r#"
        <dt>カード種類</dt><dd>シグニ</dd>
        <dt>クラス</dt><dd>奏武：ブルアカ</dd>
        <dt>色</dt><dd>緑</dd>
        <dt>レベル</dt><dd>1</dd>
        <dt>グロウコスト</dt><dd>-</dd>
        <dt>コスト</dt><dd>-</dd>
        <dt>リミット</dt><dd>-</dd>
        <dt>パワー</dt><dd>3000</dd>
        <dt>リミット消費</dt><dd>-</dd>
        <dt>使用タイミング</dt><dd>-</dd>
        <dt>その他</dt><dd>-</dd>
        <dt>ストーリー</dt><dd>-</dd>
    "#;
    
    let no_story = analyzer.detect_story_from_html(no_story_html);
    assert_eq!(no_story, None);
}

#[test]
fn test_cost_detection_from_html() {
    let analyzer = SimpleRawCardAnalyzer::new();
    
    // ルリグカード（グロウコスト）
    let lrig_html = r#"
        <dt>カード種類</dt><dd>ルリグ</dd>
        <dt>クラス</dt><dd>ホシノ</dd>
        <dt>色</dt><dd>緑</dd>
        <dt>レベル</dt><dd>2</dd>
        <dt>グロウコスト</dt><dd>《緑》×１</dd>
        <dt>コスト</dt><dd>-</dd>
    "#;
    
    let lrig_cost = analyzer.detect_cost_from_html(lrig_html);
    assert_eq!(lrig_cost, Some("g1".to_string())); // 変換後の形式
    
    // シグニカード（通常コスト）
    let signi_html = r#"
        <dt>カード種類</dt><dd>シグニ</dd>
        <dt>クラス</dt><dd>奏武：ブルアカ</dd>
        <dt>色</dt><dd>緑</dd>
        <dt>レベル</dt><dd>1</dd>
        <dt>グロウコスト</dt><dd>-</dd>
        <dt>コスト</dt><dd>《緑》×２</dd>
    "#;
    
    let signi_cost = analyzer.detect_cost_from_html(signi_html);
    assert_eq!(signi_cost, Some("g2".to_string())); // 変換後の形式
}

#[test]
fn test_card_type_conditional_extraction() {
    let analyzer = SimpleRawCardAnalyzer::new();
    
    // ルリグでリミットが取得できることを確認
    let lrig_html = r#"
        <dt>カード種類</dt><dd>ルリグ</dd>
        <dt>クラス</dt><dd>ホシノ</dd>
        <dt>色</dt><dd>緑</dd>
        <dt>レベル</dt><dd>2</dd>
        <dt>グロウコスト</dt><dd>《緑》×１</dd>
        <dt>コスト</dt><dd>-</dd>
        <dt>リミット</dt><dd>5</dd>
        <dt>パワー</dt><dd>-</dd>
    "#;
    
    let (lrig_limit, _detected_future) = analyzer.detect_limit_from_html(lrig_html);
    assert!(lrig_limit.is_some());
    
    let lrig_power = analyzer.detect_power_from_html(lrig_html);
    assert!(lrig_power.is_none()); // ルリグは条件に含まれない
    
    // シグニでパワーが取得できることを確認
    let signi_html = r#"
        <dt>カード種類</dt><dd>シグニ</dd>
        <dt>クラス</dt><dd>奏武：ブルアカ</dd>
        <dt>色</dt><dd>緑</dd>
        <dt>レベル</dt><dd>1</dd>
        <dt>グロウコスト</dt><dd>-</dd>
        <dt>コスト</dt><dd>-</dd>
        <dt>リミット</dt><dd>-</dd>
        <dt>パワー</dt><dd>3000</dd>
    "#;
    
    let signi_power = analyzer.detect_power_from_html(signi_html);
    assert!(signi_power.is_some());
    
    let (signi_limit, _detected_future) = analyzer.detect_limit_from_html(signi_html);
    assert!(signi_limit.is_none()); // シグニは条件に含まれない
}