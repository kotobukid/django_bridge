use crate::config::{CARDS_PER_PAGE, REQUEST_DELAY_SECS, WIXOSS_BASE_URL, WIXOSS_COOKIE};
use crate::error::{CacherError, Result};
use crate::search_query::SearchQuery;
use models::product::Product;
use reqwest::Client;
use std::fmt;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use tokio::time::{sleep, Duration};
use webapp::analyze::{extract_number, find_one, try_mkdir};

pub struct ProductCacher {
    root_dir: PathBuf,
    product: Product,
    client: Client,
}

impl fmt::Display for ProductCacher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ProductCacher {{ root_dir: {:?}, product: {} }}",
            self.root_dir, self.product.product_code
        )
    }
}

impl ProductCacher {
    pub fn new(root_dir: PathBuf, product: Product) -> Self {
        Self {
            root_dir,
            product,
            client: Client::new(),
        }
    }

    fn get_cache_dir_path(&self) -> Result<PathBuf> {
        let product_type = match self.product.product_type.as_str() {
            "bo" => "booster",
            "st" => "starter",
            "sp" => "special_card",
            "pr" => "promotion_card",
            _ => {
                return Err(CacherError::UnknownProductType(
                    self.product.product_type.clone(),
                ))
            }
        };

        Ok(self.root_dir.join(product_type))
    }

    fn create_search_query(&self, card_page: i32) -> Result<SearchQuery> {
        let query = match self.product.product_type.as_str() {
            "sp" => SearchQuery::new("special_card", card_page)
                .with_keyword(self.product.product_code.clone())
                .with_keyword_target(
                    "カードNo,カード名,カードタイプ,テキスト,イラストレーター,フレーバー".into(),
                ),
            "st" => SearchQuery::new("starter", card_page)
                .with_product_no(self.product.product_code.clone()),
            "bo" => SearchQuery::new("booster", card_page)
                .with_product_no(self.product.product_code.clone()),
            "pr" => SearchQuery::new("promotion_card", card_page),
            _ => {
                return Err(CacherError::UnknownProductType(
                    self.product.product_type.clone(),
                ))
            }
        };

        Ok(query)
    }

    pub async fn cache_all_pages(&self) -> Result<()> {
        let page_count = self.cache_page(1).await?;
        println!(
            "Total pages for {}: {}",
            self.product.product_code, page_count
        );

        for page in 2..=page_count {
            self.cache_page(page).await?;
            sleep(Duration::from_secs(REQUEST_DELAY_SECS)).await;
        }

        Ok(())
    }

    async fn cache_page(&self, page_number: i32) -> Result<i32> {
        let search_query = self.create_search_query(page_number)?;
        println!(
            "Caching page {} for {}",
            page_number, self.product.product_code
        );

        let content = match search_query.check_cache(self.root_dir.clone()) {
            Ok(cached_content) => cached_content,
            Err(CacherError::CacheNotFound) => {
                let fetched_content = self.fetch_page(&search_query).await?;
                self.save_to_cache(&search_query, &fetched_content)?;
                fetched_content
            }
            Err(e) => return Err(e),
        };

        let page_count = self.extract_page_count(&content)?;
        Ok(page_count)
    }

    async fn fetch_page(&self, search_query: &SearchQuery) -> Result<String> {
        let form = search_query.to_hashmap();

        let response = self
            .client
            .post(WIXOSS_BASE_URL)
            .header(reqwest::header::COOKIE, WIXOSS_COOKIE)
            .query(&form)
            .send()
            .await?;

        let body = response.text().await?;

        let main_content = find_one(&body, ".cardDip".into())
            .ok_or_else(|| CacherError::ParseError("Could not find .cardDip element".into()))?;

        Ok(main_content)
    }

    fn save_to_cache(&self, search_query: &SearchQuery, content: &str) -> Result<()> {
        let cache_path = self
            .root_dir
            .join(&search_query.product_type)
            .join(search_query.to_filename());

        if let Some(parent) = cache_path.parent() {
            try_mkdir(parent)?;
        }

        let mut file = File::create(&cache_path)?;
        file.write_all(content.as_bytes())?;

        println!("Cached to: {cache_path:?}");
        Ok(())
    }

    fn extract_page_count(&self, content: &str) -> Result<i32> {
        let count_element = find_one(content, "h3 p span".into())
            .ok_or_else(|| CacherError::ParseError("Could not find item count".into()))?;

        let total_items = extract_number(&count_element)
            .ok_or_else(|| CacherError::ParseError("Could not extract item count".into()))?;

        let page_count = (total_items + CARDS_PER_PAGE - 1) / CARDS_PER_PAGE;
        Ok(page_count)
    }

    pub async fn extract_card_links(&self) -> Result<Vec<String>> {
        let cache_dir = self.get_cache_dir_path()?;
        let links = Vec::new();

        let entries = fs::read_dir(&cache_dir)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("html") {
                // TODO: Parse HTML and extract card detail links
                println!("Processing: {path:?}");
            }
        }

        Ok(links)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use models::product::Product;
    use std::fs;

    fn create_test_product(product_type: &str, product_code: &str) -> Product {
        use models::product::ProductDb;

        Product(ProductDb {
            id: 1,
            name: "Test Product".to_string(),
            product_code: product_code.to_string(),
            url: None,
            product_type: product_type.to_string(),
            sort_asc: 0,
        })
    }

    #[test]
    fn test_get_cache_dir_path() {
        let temp_dir = std::env::temp_dir().join("test_product_cacher");

        // ブースターパックのテスト
        let product = create_test_product("bo", "WXDi-P01");
        let cacher = ProductCacher::new(temp_dir.clone(), product);
        let cache_dir = cacher.get_cache_dir_path().unwrap();
        assert_eq!(cache_dir, temp_dir.join("booster"));

        // スターターデッキのテスト
        let product = create_test_product("st", "WX24-D1");
        let cacher = ProductCacher::new(temp_dir.clone(), product);
        let cache_dir = cacher.get_cache_dir_path().unwrap();
        assert_eq!(cache_dir, temp_dir.join("starter"));

        // スペシャルカードのテスト
        let product = create_test_product("sp", "SPECIAL");
        let cacher = ProductCacher::new(temp_dir.clone(), product);
        let cache_dir = cacher.get_cache_dir_path().unwrap();
        assert_eq!(cache_dir, temp_dir.join("special_card"));

        // プロモーションカードのテスト
        let product = create_test_product("pr", "PROMO");
        let cacher = ProductCacher::new(temp_dir.clone(), product);
        let cache_dir = cacher.get_cache_dir_path().unwrap();
        assert_eq!(cache_dir, temp_dir.join("promotion_card"));
    }

    #[test]
    fn test_get_cache_dir_path_unknown_type() {
        let temp_dir = std::env::temp_dir().join("test_product_cacher");
        let product = create_test_product("unknown", "TEST");
        let cacher = ProductCacher::new(temp_dir, product);

        match cacher.get_cache_dir_path() {
            Err(CacherError::UnknownProductType(t)) => assert_eq!(t, "unknown"),
            _ => panic!("Expected UnknownProductType error"),
        }
    }

    #[test]
    fn test_create_search_query_booster() -> Result<()> {
        let temp_dir = std::env::temp_dir().join("test_product_cacher");
        let product = create_test_product("bo", "WXDi-P01");
        let cacher = ProductCacher::new(temp_dir, product);

        let query = cacher.create_search_query(1)?;

        assert_eq!(query.product_type, "booster");
        assert_eq!(query.product_no, "WXDi-P01");
        assert_eq!(query.card_page, "1");
        assert_eq!(query.search, "1");
        assert_eq!(query.support_formats, "");

        Ok(())
    }

    #[test]
    fn test_create_search_query_starter() -> Result<()> {
        let temp_dir = std::env::temp_dir().join("test_product_cacher");
        let product = create_test_product("st", "WX24-D1");
        let cacher = ProductCacher::new(temp_dir, product);

        let query = cacher.create_search_query(2)?;

        assert_eq!(query.product_type, "starter");
        assert_eq!(query.product_no, "WX24-D1");
        assert_eq!(query.card_page, "2");

        Ok(())
    }

    #[test]
    fn test_create_search_query_special_card() -> Result<()> {
        let temp_dir = std::env::temp_dir().join("test_product_cacher");
        let product = create_test_product("sp", "SPECIAL");
        let cacher = ProductCacher::new(temp_dir, product);

        let query = cacher.create_search_query(3)?;

        assert_eq!(query.product_type, "special_card");
        assert_eq!(query.keyword, "SPECIAL");
        assert_eq!(query.card_page, "3");
        assert_eq!(
            query.keyword_target,
            "カードNo,カード名,カードタイプ,テキスト,イラストレーター,フレーバー"
        );

        Ok(())
    }

    #[test]
    fn test_create_search_query_promotion_card() -> Result<()> {
        let temp_dir = std::env::temp_dir().join("test_product_cacher");
        let product = create_test_product("pr", "PROMO");
        let cacher = ProductCacher::new(temp_dir, product);

        let query = cacher.create_search_query(4)?;

        assert_eq!(query.product_type, "promotion_card");
        assert_eq!(query.card_page, "4");

        Ok(())
    }

    #[test]
    fn test_create_search_query_unknown_type() {
        let temp_dir = std::env::temp_dir().join("test_product_cacher");
        let product = create_test_product("unknown", "TEST");
        let cacher = ProductCacher::new(temp_dir, product);

        match cacher.create_search_query(1) {
            Err(CacherError::UnknownProductType(t)) => assert_eq!(t, "unknown"),
            _ => panic!("Expected UnknownProductType error"),
        }
    }

    #[test]
    fn test_extract_page_count() -> Result<()> {
        let temp_dir = std::env::temp_dir().join("test_product_cacher");
        let product = create_test_product("bo", "TEST");
        let cacher = ProductCacher::new(temp_dir, product);

        // 42件のアイテムがある場合のテスト（21件/ページなので2ページ）
        let content_42_items = r#"<h3><p><span>42</span>件のカードが見つかりました</p></h3>"#;
        let page_count = cacher.extract_page_count(content_42_items)?;
        assert_eq!(page_count, 2);

        // 21件のアイテムがある場合のテスト（ちょうど1ページ）
        let content_21_items = r#"<h3><p><span>21</span>件のカードが見つかりました</p></h3>"#;
        let page_count = cacher.extract_page_count(content_21_items)?;
        assert_eq!(page_count, 1);

        // 43件のアイテムがある場合のテスト（3ページ必要）
        let content_43_items = r#"<h3><p><span>43</span>件のカードが見つかりました</p></h3>"#;
        let page_count = cacher.extract_page_count(content_43_items)?;
        assert_eq!(page_count, 3);

        Ok(())
    }

    #[test]
    fn test_extract_page_count_no_match() {
        let temp_dir = std::env::temp_dir().join("test_product_cacher");
        let product = create_test_product("bo", "TEST");
        let cacher = ProductCacher::new(temp_dir, product);

        let content_no_match = r#"<div>No count information</div>"#;

        match cacher.extract_page_count(content_no_match) {
            Err(CacherError::ParseError(_)) => (), // 期待される結果
            _ => panic!("Expected ParseError"),
        }
    }

    #[test]
    fn test_save_to_cache() -> Result<()> {
        let temp_dir = std::env::temp_dir().join("test_save_cache");
        let product = create_test_product("bo", "TEST");
        let cacher = ProductCacher::new(temp_dir.clone(), product);

        let search_query = SearchQuery::new("booster", 1).with_product_no("TEST".to_string());

        let content = "test cache content";
        cacher.save_to_cache(&search_query, content)?;

        // ファイルが作成されたかチェック
        let cache_path = temp_dir.join("booster").join("TEST-1.html");
        assert!(cache_path.exists());

        // ファイル内容をチェック
        let saved_content = fs::read_to_string(&cache_path)?;
        assert_eq!(saved_content, content);

        // クリーンアップ
        fs::remove_dir_all(&temp_dir).ok();

        Ok(())
    }

    #[test]
    fn test_display() {
        let temp_dir = std::env::temp_dir().join("test_display");
        let product = create_test_product("bo", "WXDi-P01");
        let cacher = ProductCacher::new(temp_dir.clone(), product);

        let display_string = format!("{}", cacher);
        assert!(display_string.contains("WXDi-P01"));
        assert!(display_string.contains("ProductCacher"));
    }
}
