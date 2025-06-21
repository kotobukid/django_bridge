#[cfg(test)]
mod field_extraction_tests {
    use crate::card_analyzer::SimpleRawCardAnalyzer;

    /// ルリグカードのテスト用HTML
    fn lrig_test_html() -> String {
        r#"
        <dt>カード種類</dt><dd>ルリグ</dd>
        <dt>クラス</dt><dd>ホシノ</dd>
        <dt>色</dt><dd>緑</dd>
        <dt>レベル</dt><dd>2</dd>
        <dt>グロウコスト</dt><dd>《緑》×１</dd>
        <dt>コスト</dt><dd>-</dd>
        <dt>リミット</dt><dd>5</dd>
        <dt>パワー</dt><dd>-</dd>
        <dt>リミット消費</dt><dd>-</dd>
        <dt>使用タイミング</dt><dd>-</dd>
        <dt>その他</dt><dd>-</dd>
        <dt>ストーリー</dt><dd>-</dd>
        "#.to_string()
    }

    /// シグニカードのテスト用HTML
    fn signi_test_html() -> String {
        r#"
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
        "#.to_string()
    }

    /// アーツカードのテスト用HTML
    fn arts_test_html() -> String {
        r#"
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
        <dt>その他</dt><dd>-</dd>
        <dt>ストーリー</dt><dd>-</dd>
        "#.to_string()
    }

    /// ピースカードのテスト用HTML
    fn piece_test_html() -> String {
        r#"
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
        <dt>その他</dt><dd>-</dd>
        <dt>ストーリー</dt><dd>-</dd>
        "#.to_string()
    }

    // フルテストは複雑なので、個別メソッドのテストに集中

    #[test]
    fn test_individual_field_detection_methods() {
        let analyzer = SimpleRawCardAnalyzer::new();
        
        // 個別メソッドのテスト
        println!("Testing individual field detection methods:");
        
        // レベル検出テスト
        let level = analyzer.detect_level_from_html(&lrig_test_html());
        assert_eq!(level, Some("2".to_string()));
        println!("✅ Level detection: {:?}", level);
        
        // リミット検出テスト（ルリグのみ）
        let (limit, _feature) = analyzer.detect_limit_from_html(&lrig_test_html());
        assert_eq!(limit, Some("5".to_string()));
        println!("✅ Limit detection: {:?}", limit);
        
        // パワー検出テスト（シグニのみ）
        let power = analyzer.detect_power_from_html(&signi_test_html());
        assert_eq!(power, Some("3000".to_string()));
        println!("✅ Power detection: {:?}", power);
        
        // タイミング検出テスト（アーツ/ピースのみ）
        let timing_arts = analyzer.detect_timing_from_html(&arts_test_html());
        assert_eq!(timing_arts, Some("メインフェイズ".to_string()));
        println!("✅ Timing detection (Arts): {:?}", timing_arts);
        
        let timing_piece = analyzer.detect_timing_from_html(&piece_test_html());
        assert_eq!(timing_piece, Some("アタックフェイズ".to_string()));
        println!("✅ Timing detection (Piece): {:?}", timing_piece);
    }

    #[test]
    fn test_card_type_conditional_extraction() {
        let analyzer = SimpleRawCardAnalyzer::new();
        
        // ルリグでリミットが取得できることを確認
        let (lrig_limit, _feature) = analyzer.detect_limit_from_html(&lrig_test_html());
        assert!(lrig_limit.is_some());
        
        // シグニでリミットが取得できないことを確認（シグニは条件に含まれない）
        let (signi_limit, _feature) = analyzer.detect_limit_from_html(&signi_test_html());
        assert!(signi_limit.is_none());
        
        // シグニでパワーが取得できることを確認
        let signi_power = analyzer.detect_power_from_html(&signi_test_html());
        assert!(signi_power.is_some());
        
        // ルリグでパワーが取得できないことを確認（ルリグは条件に含まれない）
        let lrig_power = analyzer.detect_power_from_html(&lrig_test_html());
        assert!(lrig_power.is_none());
        
        // アーツでタイミングが取得できることを確認
        let arts_timing = analyzer.detect_timing_from_html(&arts_test_html());
        assert!(arts_timing.is_some());
        
        // シグニでタイミングが取得できないことを確認（シグニは条件に含まれない）
        let signi_timing = analyzer.detect_timing_from_html(&signi_test_html());
        assert!(signi_timing.is_none());
        
        println!("✅ Card type conditional extraction test passed");
    }
}