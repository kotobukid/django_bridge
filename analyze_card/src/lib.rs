mod wixoss;

use async_recursion::async_recursion;
use reqwest::{Client, Response, Url};
use scraper::{Html, Selector};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs;
use std::fs::{File, ReadDir};
use std::io::prelude::*;
use std::path::{Path, PathBuf};


fn main() {
    println!("Hello, world!");
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
}

#[derive(Clone, Debug, PartialEq)]
pub enum ProductType {
    Booster(String),
    Starter(String),
    PromotionCard,
    SpecialCard,
}

impl ProductType {
    fn get_path_relative(&self) -> String {
        match self {
            ProductType::Booster(product_no) => format!("booster/{}", product_no),
            ProductType::Starter(product_no) => format!("starter/{}", product_no),
            ProductType::PromotionCard => String::from("promotion"),
            ProductType::SpecialCard => String::from("special"),
        }
    }
}

impl SearchQuery {
    fn new(product_type: &ProductType, card_page: i32) -> SearchQuery {
        SearchQuery {
            search: "1".into(),
            keyword: "".into(),
            product_type: product_type.clone(),
            card_page: card_page.to_string(),
            card_kind: "".into(),
            rarelity: "".into(),
            tab_item: "".into(),
        }
    }

    fn get_product_type(&self) -> String {
        match &self.product_type {
            ProductType::Booster(_product_no) => "booster".into(),
            ProductType::Starter(_product_no) => "starter".into(),
            ProductType::PromotionCard => "-".into(),
            ProductType::SpecialCard => "-".into(),
        }
    }

    fn to_hashmap(&self) -> HashMap<String, String> {
        let empty_product_no = String::from("");

        let product_no = match &self.product_type {
            ProductType::Booster(product_no) => product_no,
            ProductType::Starter(product_no) => product_no,
            ProductType::PromotionCard => &empty_product_no,
            ProductType::SpecialCard => &empty_product_no,
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
        ])
    }

    fn to_filename(&self) -> String {
        format!(
            "{}/p{}.html",
            &self.product_type.get_path_relative(),
            &self.card_page
        )
    }

    fn cache_check(&self, dir: String) -> Result<String, std::io::Error> {
        let path: PathBuf = PathBuf::from(format!("{}/{}", dir, &self.to_filename()));
        if path.exists() {
            println!("cache found");
            let mut file: File = File::open(&path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            Ok(contents)
        } else {
            println!("cache not found");
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "An unexpected error occurred.",
            ))
        }
    }
}

pub fn try_mkdir(rel_path: &Path) -> Result<(), std::io::Error> {
    if !rel_path.exists() {
        fs::create_dir_all(rel_path)?;
    }

    Ok(())
}

#[async_recursion]
pub async fn cache_product_index(
    product_type: &ProductType,
    card_page: i32,
) -> Result<(), reqwest::Error> {
    let p_no = product_type.get_path_relative();
    println!("{} {}", p_no, card_page);

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

            let body: String = res.text().await.unwrap();

            let cache_filename: PathBuf =
                PathBuf::from(format!("./text_cache/{}", &search_query.to_filename()));

            if let Some(parent_path) = cache_filename.parent() {
                try_mkdir(parent_path).unwrap();

                let content = find_one(&body, ".cardDip".into());

                if let Some(element) = &content {
                    let file: Result<File, std::io::Error> = File::create(&cache_filename);
                    if let Ok(mut file_) = file {
                        file_.write_all(element.as_bytes()).unwrap();
                    }
                }
                content
            } else {
                None
            }
        }
    };

    if let Some(count) = find_one(&main.unwrap(), "h3 p span".into()) {
        let count = extract_number(&count);

        if let Some(count) = count {
            let pages = (count / 21) + 1;

            if card_page < pages {
                cache_product_index(product_type, card_page + 1)
                    .await
                    .unwrap();
            }
        }
    } else {
        println!("not found");
    }

    Ok(())
}

pub fn find_one(content: &str, selector: String) -> Option<String> {
    let document: Html = Html::parse_document(content);
    let main_selector: Selector = Selector::parse(selector.as_str()).unwrap();

    println!("{:?}", document.clone());
    document
        .select(&main_selector)
        .next()
        .map(|element| element.inner_html())
}

pub async fn collect_card_detail_links(product_type: &ProductType) -> Result<Vec<String>, ()> {
    let product_root: String = product_type.get_path_relative();
    let path_s: String = format!("./text_cache/{}", product_root);
    let product_dir: &Path = Path::new(&path_s);

    println!("{}", product_dir.display());

    try_mkdir(product_dir).unwrap();
    let files_result: std::io::Result<ReadDir> = fs::read_dir(product_dir);

    match files_result {
        Ok(files) => {
            let all_text: String = files
                .into_iter()
                .map(|f| {
                    let p: fs::DirEntry = f.unwrap();
                    let file_path: PathBuf = p.path();
                    let content: String = fs::read_to_string(file_path).unwrap();

                    content
                })
                .collect::<Vec<_>>()
                .join("");

            let parsed_html: Html = Html::parse_document(&all_text);
            let selector: Selector = Selector::parse("a.c-box").unwrap();
            let links: Vec<String> = parsed_html
                .select(&selector)
                .map(|element| element.value().attr("href").unwrap_or("").to_owned())
                .filter(|href| !href.is_empty())
                .collect();
            Ok(links)
        }
        Err(err) => {
            println!("{:?}", err);
            Err(())
        }
    }
}

#[allow(dead_code)]
pub fn find_many(content: &str, selector: String) -> Vec<String> {
    let document: Html = Html::parse_document(content);
    let main_selector: Selector = Selector::parse(selector.as_str()).unwrap();
    let mut result: Vec<String> = Vec::new();
    for element in document.select(&main_selector) {
        result.push(element.inner_html());
    }
    result
}

pub fn extract_number(s: &str) -> Option<i32> {
    let digits: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
    digits.parse().ok()
}

#[derive(Debug, Deserialize)]
pub struct CardQuery {
    card: String,
    card_no: String,
}

impl CardQuery {
    pub fn get_relative_filename(&self) -> String {
        let mut tokens: Vec<_> = self.card_no.split('-').collect();
        let id = tokens.last().unwrap().to_string();
        tokens.pop();
        let dir: String = tokens.join("-");

        format!("{}/{}.html", dir, id)
    }

    pub fn from_card_no(card_no: String) -> Self {
        Self {
            card_no,
            card: "card_detail".into(),
        }
    }
    pub fn to_hashmap(&self) -> HashMap<String, String> {
        HashMap::from_iter(vec![
            ("card_no".into(), self.card_no.clone()),
            ("card".into(), self.card.clone()),
        ])
    }

    pub async fn download_card_detail(&self, cache_dir: &'static str) -> Option<String> {
        let cache_file: PathBuf =
            PathBuf::from(format!("{}/{}", cache_dir, self.get_relative_filename()));

        println!("{:?}", cache_file);
        if cache_file.exists() {
            let mut file: File = File::open(&cache_file).expect("cache file open error");
            let mut contents = String::new();

            match file.read_to_string(&mut contents) {
                Ok(_) => Some(contents),
                _ => None,
            }
        } else {
            let url = "https://www.takaratomy.co.jp/products/wixoss/card_list.php";

            let form: HashMap<String, String> = self.to_hashmap();

            let client: Client = Client::new();
            let res: Result<Response, reqwest::Error> = client
                .post(url)
                .header(reqwest::header::COOKIE, "wixAge=conf;")
                .form(&form)
                .send()
                .await;

            match res {
                Ok(response) => {
                    let body: String = response.text().await.unwrap();
                    let body: String = format!("<html><body>{}", body);
                    let content = find_one(&body, ".cardDetail".into());

                    if let Some(body_) = content {
                        match write_to_cache(cache_file, body_.clone()) {
                            Ok(()) => Some(body_),
                            _ => None,
                        }
                    } else {
                        println!("{}", body);
                        None
                    }
                }
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
            }
        }
    }
}

pub fn parse_card_url(url_string: impl Display) -> Result<CardQuery, serde_qs::Error> {
    let parsed_url: Url = Url::parse(&url_string.to_string()).expect("Failed to parse the URL");
    let query: &str = parsed_url.query().unwrap_or_default();
    serde_qs::from_str::<CardQuery>(query)
}

pub enum CacheError {
    ParentPathMissing,
    FileCreationFailed(std::io::Error),
    WriteFailed(std::io::Error),
}

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
