pub const WIXOSS_BASE_URL: &str = "https://www.takaratomy.co.jp/products/wixoss/card/card_list.php";
pub const WIXOSS_COOKIE: &str = "wixAge=conf;";
pub const CARDS_PER_PAGE: i32 = 21;
pub const REQUEST_DELAY_SECS: u64 = 1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wixoss_base_url() {
        assert!(!WIXOSS_BASE_URL.is_empty());
        assert!(WIXOSS_BASE_URL.starts_with("https://"));
        assert!(WIXOSS_BASE_URL.contains("takaratomy.co.jp"));
        assert!(WIXOSS_BASE_URL.contains("wixoss"));
        assert!(WIXOSS_BASE_URL.ends_with(".php"));
    }

    #[test]
    fn test_wixoss_cookie() {
        assert!(!WIXOSS_COOKIE.is_empty());
        assert!(WIXOSS_COOKIE.contains("wixAge"));
        assert!(WIXOSS_COOKIE.contains("conf"));
        assert!(WIXOSS_COOKIE.ends_with(";"));
    }

    #[test]
    fn test_cards_per_page() {
        assert!(CARDS_PER_PAGE > 0);
        assert_eq!(CARDS_PER_PAGE, 21);
        // 実際のWIXOSSサイトの1ページあたりのカード数と一致することを確認
    }

    #[test]
    fn test_request_delay_secs() {
        assert!(REQUEST_DELAY_SECS > 0);
        assert_eq!(REQUEST_DELAY_SECS, 1);
        // サーバーに負荷をかけないための適切な遅延時間
    }

    #[test]
    fn test_url_validity() {
        // URLのフォーマットが正しいかを簡単にチェック
        let url = WIXOSS_BASE_URL;
        assert!(url.parse::<url::Url>().is_ok(), "Invalid URL format");
    }
}