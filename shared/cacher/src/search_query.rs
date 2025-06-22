use crate::error::{CacherError, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct SearchQuery {
    pub search: String,
    pub keyword: String,
    pub product_type: String,
    pub product_no: String,
    pub card_page: String,
    pub card_kind: String,
    pub rarelity: String,
    pub tab_item: String,
    pub support_formats: String,
    pub keyword_target: String,
}

impl SearchQuery {
    pub fn new(product_type: &str, card_page: i32) -> Self {
        SearchQuery {
            search: "1".into(),
            keyword: String::new(),
            product_type: product_type.into(),
            product_no: String::new(),
            card_page: card_page.to_string(),
            card_kind: String::new(),
            rarelity: String::new(),
            tab_item: String::new(),
            support_formats: "".into(),
            keyword_target: String::new(),
        }
    }

    pub fn with_keyword(mut self, keyword: String) -> Self {
        self.keyword = keyword;
        self
    }

    pub fn with_product_no(mut self, product_no: String) -> Self {
        self.product_no = product_no;
        self
    }

    pub fn with_keyword_target(mut self, target: String) -> Self {
        self.keyword_target = target;
        self
    }

    pub fn to_filename(&self) -> String {
        match self.product_type.as_str() {
            "booster" | "starter" => format!("{}-{}.html", self.product_no, self.card_page),
            "special_card" => format!("{}-{}.html", self.keyword, self.card_page),
            "promotion_card" => format!("p{}.html", self.card_page),
            _ => panic!("Unknown product type: {}", self.product_type),
        }
    }

    pub fn check_cache(&self, cache_root: PathBuf) -> Result<String> {
        let path = cache_root.join(&self.product_type).join(self.to_filename());

        if path.exists() {
            println!("Cache found: {:?}", path);
            let mut file = File::open(&path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            Ok(contents)
        } else {
            println!("Cache not found: {:?}", path);
            Err(CacherError::CacheNotFound)
        }
    }

    pub fn to_hashmap(&self) -> HashMap<String, String> {
        let product_no = match self.product_type.as_str() {
            "booster" | "starter" => self.product_no.clone(),
            "promotion_card" | "special_card" => String::new(),
            _ => panic!("Unknown product type: {}", self.product_type),
        };

        HashMap::from_iter(vec![
            ("search".into(), self.search.clone()),
            ("keyword".into(), self.keyword.clone()),
            ("product_type".into(), self.product_type.clone()),
            ("product_no".into(), product_no),
            ("card_page".into(), self.card_page.clone()),
            ("card_kind".into(), self.card_kind.clone()),
            ("rarelity".into(), self.rarelity.clone()),
            ("tab_item".into(), self.tab_item.clone()),
            ("support_formats".into(), self.support_formats.clone()),
            ("keyword_target".into(), self.keyword_target.clone()),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_search_query_new() {
        let query = SearchQuery::new("booster", 1);

        assert_eq!(query.search, "1");
        assert_eq!(query.product_type, "booster");
        assert_eq!(query.card_page, "1");
        assert_eq!(query.support_formats, "");
        assert!(query.keyword.is_empty());
        assert!(query.product_no.is_empty());
    }

    #[test]
    fn test_search_query_builder_pattern() {
        let query = SearchQuery::new("special_card", 2)
            .with_keyword("WXDi-P01".to_string())
            .with_keyword_target("カードNo,カード名".to_string());

        assert_eq!(query.product_type, "special_card");
        assert_eq!(query.card_page, "2");
        assert_eq!(query.keyword, "WXDi-P01");
        assert_eq!(query.keyword_target, "カードNo,カード名");
    }

    #[test]
    fn test_to_filename_booster() {
        let query = SearchQuery::new("booster", 1).with_product_no("WXDi-P01".to_string());

        assert_eq!(query.to_filename(), "WXDi-P01-1.html");
    }

    #[test]
    fn test_to_filename_starter() {
        let query = SearchQuery::new("starter", 3).with_product_no("WX24-D1".to_string());

        assert_eq!(query.to_filename(), "WX24-D1-3.html");
    }

    #[test]
    fn test_to_filename_special_card() {
        let query = SearchQuery::new("special_card", 2).with_keyword("special".to_string());

        assert_eq!(query.to_filename(), "special-2.html");
    }

    #[test]
    fn test_to_filename_promotion_card() {
        let query = SearchQuery::new("promotion_card", 5);

        assert_eq!(query.to_filename(), "p5.html");
    }

    #[test]
    fn test_to_hashmap() {
        let query = SearchQuery::new("booster", 1).with_product_no("WXDi-P01".to_string());

        let hashmap = query.to_hashmap();

        assert_eq!(hashmap.get("search"), Some(&"1".to_string()));
        assert_eq!(hashmap.get("product_type"), Some(&"booster".to_string()));
        assert_eq!(hashmap.get("product_no"), Some(&"WXDi-P01".to_string()));
        assert_eq!(hashmap.get("card_page"), Some(&"1".to_string()));
        assert_eq!(hashmap.get("support_formats"), Some(&"2".to_string()));
    }

    #[test]
    fn test_to_hashmap_special_card() {
        let query = SearchQuery::new("special_card", 1).with_keyword("special".to_string());

        let hashmap = query.to_hashmap();

        // special_cardの場合、product_noは空になる
        assert_eq!(hashmap.get("product_no"), Some(&"".to_string()));
        assert_eq!(hashmap.get("keyword"), Some(&"special".to_string()));
    }

    #[test]
    fn test_check_cache_not_found() {
        let query = SearchQuery::new("booster", 1).with_product_no("TEST".to_string());

        let temp_dir = std::env::temp_dir().join("test_cache_not_found");

        match query.check_cache(temp_dir) {
            Err(CacherError::CacheNotFound) => (), // 期待される結果
            _ => panic!("Expected CacheNotFound error"),
        }
    }

    #[test]
    fn test_check_cache_found() -> Result<()> {
        let query = SearchQuery::new("booster", 1).with_product_no("TEST".to_string());

        let temp_dir = std::env::temp_dir().join("test_cache_found");
        let cache_dir = temp_dir.join("booster");
        let cache_file = cache_dir.join("TEST-1.html");

        // テスト用のキャッシュファイルを作成
        fs::create_dir_all(&cache_dir)?;
        fs::write(&cache_file, "test content")?;

        let result = query.check_cache(temp_dir.clone())?;
        assert_eq!(result, "test content");

        // クリーンアップ
        fs::remove_dir_all(&temp_dir).ok();

        Ok(())
    }
}
