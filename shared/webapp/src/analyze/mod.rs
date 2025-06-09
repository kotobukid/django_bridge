pub mod wixoss;
pub mod raw_card_integration;

use async_recursion::async_recursion;
use reqwest::{Client, Response, Url};
use scraper::{Html, Selector};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

/// Error types for the analyze module
#[derive(Debug)]
pub enum AnalyzeError {
    /// IO error
    Io(std::io::Error),
    /// Request error
    Request(reqwest::Error),
    /// Parse error
    Parse(String),
    /// Cache error
    Cache(String),
    /// Parent path missing
    ParentPathMissing,
}

impl Display for AnalyzeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AnalyzeError::Io(err) => write!(f, "IO error: {}", err),
            AnalyzeError::Request(err) => write!(f, "Request error: {}", err),
            AnalyzeError::Parse(msg) => write!(f, "Parse error: {}", msg),
            AnalyzeError::Cache(msg) => write!(f, "Cache error: {}", msg),
            AnalyzeError::ParentPathMissing => write!(f, "Parent path missing"),
        }
    }
}

impl Error for AnalyzeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AnalyzeError::Io(err) => Some(err),
            AnalyzeError::Request(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for AnalyzeError {
    fn from(err: std::io::Error) -> Self {
        AnalyzeError::Io(err)
    }
}

impl From<reqwest::Error> for AnalyzeError {
    fn from(err: reqwest::Error) -> Self {
        AnalyzeError::Request(err)
    }
}

#[derive(Clone, Debug)]
pub struct SearchQuery {
    search: String,
    keyword: String,
    product_type: ProductType,
    card_page: String,
    card_kind: String,
    rarelity: String,
    tab_item: String,
    support_formats: String,
    keyword_target: String,
}

/// Type of product
#[derive(Clone, Debug, PartialEq)]
pub enum ProductType {
    /// Booster pack with a product number
    Booster(String),
    /// Starter deck with a product number
    Starter(String),
    /// Promotion card
    PromotionCard,
    /// Special card with a product number
    SpecialCard(String),
}

impl ProductType {
    /// Get the relative path for the product type
    ///
    /// # Returns
    ///
    /// The relative path for the product type
    fn get_path_relative(&self) -> String {
        match self {
            ProductType::Booster(product_no) => format!("booster/{}", product_no),
            ProductType::Starter(product_no) => format!("starter/{}", product_no),
            ProductType::PromotionCard => String::from("promotion"),
            ProductType::SpecialCard(product_no) => format!("special/{}", product_no),
        }
    }

    /// Get the product code
    ///
    /// # Returns
    ///
    /// The product code
    pub fn code(&self) -> String {
        match self {
            ProductType::Booster(code) => code.into(),
            ProductType::Starter(code) => code.into(),
            ProductType::PromotionCard => "promotion".into(),
            ProductType::SpecialCard(code) => code.into(),
        }
    }
}

/// Implementation of SearchQuery
impl SearchQuery {
    /// Create a new SearchQuery
    ///
    /// # Arguments
    ///
    /// * `product_type` - The type of product to search for
    /// * `card_page` - The page number to search for
    ///
    /// # Returns
    ///
    /// A new SearchQuery
    fn new(product_type: &ProductType, card_page: i32) -> SearchQuery {
        let keyword = match product_type {
            ProductType::SpecialCard(product_no) => product_no.clone(),
            _ => "".into(),
        };

        SearchQuery {
            search: "1".into(),
            keyword: keyword.clone(),
            product_type: product_type.clone(),
            card_page: card_page.to_string(),
            card_kind: "".into(),
            rarelity: "".into(),
            tab_item: "".into(),
            support_formats: "2".into(),
            keyword_target: "カードNo,カード名,カードタイプ,テキスト,イラストレーター,フレーバー"
                .into(),
        }
    }

    /// Get the product type as a string
    ///
    /// # Returns
    ///
    /// The product type as a string
    fn get_product_type(&self) -> String {
        match &self.product_type {
            ProductType::Booster(_product_no) => "booster".into(),
            ProductType::Starter(_product_no) => "starter".into(),
            ProductType::PromotionCard => "promotion_card".into(),
            ProductType::SpecialCard(_product_no) => "special_card".into(),
        }
    }

    /// Convert the SearchQuery to a HashMap
    ///
    /// # Returns
    ///
    /// A HashMap containing the SearchQuery parameters
    fn to_hashmap(&self) -> HashMap<String, String> {
        let empty_product_no = String::from("");

        let product_no = match &self.product_type {
            ProductType::Booster(product_no) => product_no,
            ProductType::Starter(product_no) => product_no,
            ProductType::PromotionCard => &empty_product_no,
            ProductType::SpecialCard(_) => &empty_product_no,
        };

        HashMap::from_iter(vec![
            ("search".into(), self.search.clone()),
            ("keyword".into(), self.keyword.clone()),
            ("product_type".into(), self.get_product_type()),
            ("product_no".into(), product_no.clone()),
            ("card_page".into(), self.card_page.clone()),
            ("card_kind".into(), self.card_kind.clone()),
            ("rarelity".into(), self.rarelity.clone()),
            ("tab_item".into(), self.tab_item.clone()),
            ("support_formats".into(), self.support_formats.clone()),
            ("keyword_target".into(), self.keyword_target.clone()),
        ])
    }

    /// Get the filename for the cache file
    ///
    /// # Returns
    ///
    /// The filename for the cache file
    fn to_filename(&self) -> String {
        match &self.product_type {
            ProductType::Booster(product_no) | ProductType::Starter(product_no) => {
                format!("{}-{}.html", product_no, self.card_page)
            }
            ProductType::SpecialCard(_) => {
                format!("{}-{}.html", self.keyword, self.card_page)
            }
            ProductType::PromotionCard => {
                format!("p{}.html", self.card_page)
            }
        }
    }

    /// Check if the cache file exists
    ///
    /// # Arguments
    ///
    /// * `dir` - The directory to check for the cache file
    ///
    /// # Returns
    ///
    /// * `Ok(String)` if the cache file exists
    /// * `Err(std::io::Error)` if the cache file does not exist
    fn cache_check(&self, dir: String) -> Result<String, std::io::Error> {
        let product_path = match &self.product_type {
            ProductType::Booster(product_no) => format!("booster/{}", product_no),
            ProductType::Starter(product_no) => format!("starter/{}", product_no),
            ProductType::PromotionCard => String::from("promotion"),
            ProductType::SpecialCard(product_no) => format!("special/{}", product_no),
        };
        let path: PathBuf =
            PathBuf::from(format!("{}/{}", dir, product_path)).join(&self.to_filename());

        if path.exists() {
            let mut file: File = File::open(&path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            Ok(contents)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Cache file not found",
            ))
        }
    }
}

/// Create a directory if it doesn't exist
///
/// # Arguments
///
/// * `rel_path` - The path to create
///
/// # Returns
///
/// * `Ok(())` if the directory was created or already exists
/// * `Err(std::io::Error)` if an error occurred
pub fn try_mkdir(rel_path: &Path) -> Result<(), std::io::Error> {
    if !rel_path.exists() {
        fs::create_dir_all(rel_path)?;
    }

    Ok(())
}

/// Cache product index data
///
/// # Arguments
///
/// * `product_type` - The type of product to cache
/// * `card_page` - The page number to cache
///
/// # Returns
///
/// * `Ok(())` if the data was successfully cached
/// * `Err(AnalyzeError)` if an error occurred
#[async_recursion]
pub async fn cache_product_index(
    product_type: &ProductType,
    card_page: i32,
) -> Result<(), AnalyzeError> {
    let p_no = product_type.get_path_relative();
    let url = "https://www.takaratomy.co.jp/products/wixoss/card/card_list.php";
    let search_query: SearchQuery = SearchQuery::new(product_type, card_page);

    let main: Option<String> = match search_query.cache_check("./text_cache".to_string()) {
        Ok(content_) => Some(content_),
        _ => {
            let form: HashMap<String, String> = search_query.to_hashmap();

            let client: Client = Client::new();
            let res: Response = client
                .post(url)
                .header(reqwest::header::COOKIE, "wixAge=conf;")
                .query(&form)
                .send()
                .await?;

            // Use ? operator instead of unwrap
            let body: String = res.text().await?;

            let cache_filename: PathBuf =
                PathBuf::from(format!("./text_cache/{}", p_no)).join(&search_query.to_filename());
            println!("CFN {:?}", cache_filename);

            if let Some(parent_path) = cache_filename.parent() {
                // Use ? operator instead of unwrap
                try_mkdir(parent_path)?;

                let content = find_one(&body, ".cardDip".into());

                if let Some(element) = &content {
                    let file: Result<File, std::io::Error> = File::create(&cache_filename);
                    if let Ok(mut file_) = file {
                        // Use ? operator instead of unwrap
                        file_.write_all(element.as_bytes())?;
                    }
                }
                content
            } else {
                None
            }
        }
    };

    // Handle the case where main is None
    let main = main.ok_or_else(|| AnalyzeError::Parse("Failed to get main content".into()))?;

    if let Some(count) = find_one(&main, "h3 p span".into()) {
        let count = extract_number(&count);

        if let Some(count) = count {
            let pages = (count / 21) + 1;

            if card_page < pages {
                // Use ? operator instead of unwrap
                cache_product_index(product_type, card_page + 1).await?;
            }
        }
    } else {
        println!("not found");
    }

    Ok(())
}

/// Find the first element matching a selector in HTML content
///
/// # Arguments
///
/// * `content` - The HTML content to search
/// * `selector` - The CSS selector to use
///
/// # Returns
///
/// * `Some(String)` if an element was found
/// * `None` if no element was found or the selector is invalid
pub fn find_one(content: &str, selector: String) -> Option<String> {
    let document: Html = Html::parse_document(content);

    // Handle invalid selectors gracefully
    let main_selector = match Selector::parse(selector.as_str()) {
        Ok(selector) => selector,
        Err(e) => {
            eprintln!("Invalid selector '{}': {}", selector, e);
            return None;
        }
    };

    document
        .select(&main_selector)
        .next()
        .map(|element| element.inner_html())
}

/// Collect card detail links from cached product index files
///
/// # Arguments
///
/// * `product_type` - The type of product to collect links for
///
/// # Returns
///
/// * `Ok(Vec<String>)` if links were found
/// * `Err(AnalyzeError)` if an error occurred
pub async fn collect_card_detail_links(product_type: &ProductType) -> Result<Vec<String>, AnalyzeError> {
    let product_root: String = product_type.get_path_relative();
    let path_s: String = format!("./text_cache/{}", product_root);
    let product_dir: &Path = Path::new(&path_s);

    // ディレクトリが存在しない場合は作成
    try_mkdir(product_dir)?;
    
    let files = fs::read_dir(product_dir)?;
    let mut all_text = String::new();
    
    // ファイルの読み込みエラーを適切に処理
    for entry in files {
        let entry = entry?;
        let file_path = entry.path();
        match fs::read_to_string(&file_path) {
            Ok(content) => all_text.push_str(&content),
            Err(e) => {
                eprintln!("ファイル読み込みエラー: {:?}: {}", file_path, e);
                // ファイル読み込みエラーは無視して続行
            }
        }
    }

    let parsed_html: Html = Html::parse_document(&all_text);
    // 静的なセレクタなので、パニックする可能性は低いが、より安全に
    let selector: Selector = Selector::parse("a.c-box")
        .map_err(|e| AnalyzeError::Parse(format!("セレクタパースエラー: {}", e)))?;
    
    let links: Vec<String> = parsed_html
        .select(&selector)
        .map(|element| element.value().attr("href").unwrap_or("").to_owned())
        .filter(|href| !href.is_empty())
        .collect();
    
    Ok(links)
}

/// 以前の実装との互換性のため、エラーを簡略化するヘルパー関数
pub async fn collect_card_detail_links_compat(product_type: &ProductType) -> Result<Vec<String>, ()> {
    collect_card_detail_links(product_type).await.map_err(|e| {
        eprintln!("カード詳細リンク収集エラー: {}", e);
    })
}

/// Find all elements matching a selector in HTML content
///
/// # Arguments
///
/// * `content` - The HTML content to search
/// * `selector` - The CSS selector to use
///
/// # Returns
///
/// A vector of inner HTML strings from the matching elements.
/// Returns an empty vector if the selector is invalid.
#[allow(dead_code)]
pub fn find_many(content: &str, selector: String) -> Vec<String> {
    let document: Html = Html::parse_document(content);

    // Handle invalid selectors gracefully
    let main_selector = match Selector::parse(selector.as_str()) {
        Ok(selector) => selector,
        Err(e) => {
            eprintln!("Invalid selector '{}': {}", selector, e);
            return Vec::new();
        }
    };

    let mut result: Vec<String> = Vec::new();
    for element in document.select(&main_selector) {
        result.push(element.inner_html());
    }
    result
}

/// Extract a number from a string
///
/// # Arguments
///
/// * `s` - The string to extract a number from
///
/// # Returns
///
/// * `Some(i32)` if a number was found
/// * `None` if no number was found
pub fn extract_number(s: &str) -> Option<i32> {
    let digits: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
    digits.parse().ok()
}

/// Query parameters for card details
#[derive(Debug, Deserialize)]
pub struct CardQuery {
    /// The card type (usually "card_detail")
    card: String,
    /// The card number
    card_no: String,
    /// The directory to cache the card details
    cache_dir: Box<Path>,
}

impl CardQuery {
    /// Get the relative filename for the cache file
    ///
    /// # Returns
    ///
    /// The relative filename for the cache file
    pub fn get_relative_filename(&self) -> String {
        let mut tokens: Vec<_> = self.card_no.split('-').collect();

        // Handle the case where there are no tokens
        if tokens.is_empty() {
            return format!("unknown/{}.html", self.card_no);
        }

        let id = tokens.last().unwrap_or(&"unknown").to_string();
        tokens.pop();
        let dir: String = if tokens.is_empty() {
            "unknown".to_string()
        } else {
            tokens.join("-")
        };

        format!("{}/{}.html", dir, id)
    }

    /// Create a new CardQuery
    ///
    /// # Arguments
    ///
    /// * `card_no` - The card number
    /// * `cache_dir` - The directory to cache the card details
    ///
    /// # Returns
    ///
    /// A new CardQuery
    pub fn new(card_no: String, cache_dir: Box<Path>) -> Self {
        Self {
            card_no,
            card: "card_detail".into(),
            cache_dir,
        }
    }

    /// Check if the cache file exists
    ///
    /// # Returns
    ///
    /// `true` if the cache file exists, `false` otherwise
    pub fn check_cache_file_exists(&self) -> bool {
        let cache_file: PathBuf = PathBuf::from(format!(
            "{}/{}",
            &self.cache_dir.display().to_string(),
            self.get_relative_filename()
        ));
        cache_file.exists()
    }

    /// Get the cached text
    ///
    /// # Returns
    ///
    /// * `Some(String)` if the cache file exists and could be read
    /// * `None` if the cache file does not exist or could not be read
    pub fn get_cache_text(&self) -> Option<String> {
        let cache_file: PathBuf = PathBuf::from(format!(
            "{}/{}",
            &self.cache_dir.display().to_string(),
            self.get_relative_filename()
        ));
        if cache_file.exists() {
            match File::open(&cache_file) {
                Ok(mut file) => {
                    let mut contents = String::new();
                    match file.read_to_string(&mut contents) {
                        Ok(_) => Some(contents),
                        Err(e) => {
                            eprintln!("Error reading cache file: {}", e);
                            None
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error opening cache file: {}", e);
                    None
                }
            }
        } else {
            None
        }
    }

    /// Convert the CardQuery to a HashMap
    ///
    /// # Returns
    ///
    /// A HashMap containing the CardQuery parameters
    pub fn to_hashmap(&self) -> HashMap<String, String> {
        HashMap::from_iter(vec![
            ("card_no".into(), self.card_no.clone()),
            ("card".into(), self.card.clone()),
        ])
    }

    /// Download card detail data
    ///
    /// # Returns
    ///
    /// * `Ok(String)` if the data was successfully downloaded
    /// * `Err(AnalyzeError)` if an error occurred
    pub async fn download_card_detail(&self) -> Result<String, AnalyzeError> {
        let cache_file: PathBuf = PathBuf::from(format!(
            "{}/{}",
            &self.cache_dir.display().to_string(),
            self.get_relative_filename()
        ));

        if cache_file.exists() {
            let mut file: File = File::open(&cache_file)
                .map_err(|e| AnalyzeError::Io(e))?;
            let mut contents = String::new();

            file.read_to_string(&mut contents)
                .map_err(|e| AnalyzeError::Io(e))?;
            Ok(contents)
        } else {
            let url = "https://www.takaratomy.co.jp/products/wixoss/card_list.php";
            let form: HashMap<String, String> = self.to_hashmap();
            let client: Client = Client::new();

            let response = client
                .post(url)
                .header(reqwest::header::COOKIE, "wixAge=conf;")
                .form(&form)
                .send()
                .await?;

            let body = response.text().await?;
            let body = format!("<html><body>{}", body);
            let content = find_one(&body, ".cardDetail".into());

            if let Some(body_) = content {
                write_to_cache(cache_file, body_.clone())
                    .map_err(|e| AnalyzeError::Cache(format!("Failed to write to cache: {:?}", e)))?;
                Ok(body_)
            } else {
                Err(AnalyzeError::Parse("Failed to find card detail".into()))
            }
        }
    }
}

/// Parse a card URL into a CardQuery
///
/// # Arguments
///
/// * `url_string` - The URL to parse
///
/// # Returns
///
/// * `Ok(CardQuery)` if the URL was successfully parsed
/// * `Err(AnalyzeError)` if an error occurred
pub fn parse_card_url(url_string: impl Display) -> Result<CardQuery, AnalyzeError> {
    let parsed_url: Url = Url::parse(&url_string.to_string())
        .map_err(|e| AnalyzeError::Parse(format!("Failed to parse URL: {}", e)))?;
    let query: &str = parsed_url.query().unwrap_or_default();
    serde_qs::from_str::<CardQuery>(query)
        .map_err(|e| AnalyzeError::Parse(format!("Failed to parse query: {}", e)))
}

/// Errors that can occur when caching data
#[derive(Debug)]
pub enum CacheError {
    /// The parent path of the cache file is missing
    ParentPathMissing,
    /// Failed to create the cache file
    FileCreationFailed(std::io::Error),
    /// Failed to write to the cache file
    WriteFailed(std::io::Error),
}

impl std::fmt::Display for CacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CacheError::ParentPathMissing => write!(f, "Parent path is missing"),
            CacheError::FileCreationFailed(err) => write!(f, "Failed to create file: {}", err),
            CacheError::WriteFailed(err) => write!(f, "Failed to write to file: {}", err),
        }
    }
}

impl std::error::Error for CacheError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CacheError::ParentPathMissing => None,
            CacheError::FileCreationFailed(err) => Some(err),
            CacheError::WriteFailed(err) => Some(err),
        }
    }
}

/// Writes data to a cache file
///
/// # Arguments
///
/// * `filename` - The path to the cache file
/// * `body` - The data to write to the cache file
///
/// # Returns
///
/// * `Ok(())` if the data was successfully written to the cache file
/// * `Err(CacheError)` if an error occurred
pub fn write_to_cache(filename: PathBuf, body: String) -> Result<(), CacheError> {
    if let Some(parent_path) = filename.parent() {
        std::fs::create_dir_all(parent_path).map_err(|_| CacheError::ParentPathMissing)?;
        let mut file = File::create(&filename).map_err(CacheError::FileCreationFailed)?;
        file.write_all(body.as_bytes())
            .map_err(CacheError::WriteFailed)?;
        Ok(())
    } else {
        Err(CacheError::ParentPathMissing)
    }
}
