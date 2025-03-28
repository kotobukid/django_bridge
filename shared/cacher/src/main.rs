// use futures::future::join_all;
use models::product::Product;
// use rand::Rng;
// use serde_qs as qs;
use std::collections::HashMap;
use std::io::Write;
#[allow(unused_imports)]
use std::path::{Path, PathBuf};
#[allow(unused_imports)]
use tokio::time::{sleep, Duration};
// use url::Url;
use futures::future::join_all;
use tokio::task::JoinHandle;
use webapp::analyze::{cache_product_index, collect_card_detail_links, try_mkdir, CardQuery};
use webapp::analyze::{extract_number, find_one, ProductType};
use webapp::repositories::ProductRepository;

// use webapp::analyze::wixoss::Card;

// use clap::Parser;
use dotenvy::from_filename;
// use models::card::CreateCard;
use reqwest::{Client, Response};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use std::fmt::Display;
use std::fs::File;
// use std::future::Future;
use std::io::Read;
// use std::pin::Pin;
use std::sync::Arc;
// use tokio::sync::Mutex;
// use webapp::repositories::{CardRepository, CardTypeRepository};

#[derive(Clone, Debug)]
pub struct SearchQuery {
    search: String,
    keyword: String,
    product_type: String,
    product_no: String,
    card_page: String,
    card_kind: String,
    rarelity: String,
    tab_item: String,
    support_formats: String,
    keyword_target: String,
}

impl SearchQuery {
    fn to_filename(&self) -> String {
        match self.product_type.as_str() {
            "booster" | "starter" => {
                format!("{}/page_{}.html", self.product_no, self.card_page)
            }
            "special_card" => {
                format!("{}/page_{}.html", self.keyword, self.card_page)
            }
            "promotion_card" => {
                format!("promotion/page_{}.html", self.card_page)
            }
            _ => {
                panic!("unknown product type")
            }
        }
    }

    pub fn check_cache(&self, dir: PathBuf) -> Result<String, std::io::Error> {
        let path = dir.join(self.to_filename());

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

    pub fn to_hashmap(&self) -> HashMap<String, String> {
        let empty_product_no = String::from("");

        let product_no = match self.product_type.as_str() {
            "booster" | "starter" => self.product_no.clone(),
            "promotion_card" => empty_product_no,
            "special_card" => empty_product_no,
            _ => panic!("unknown product type"),
        };

        HashMap::from_iter(vec![
            ("search".into(), self.search.clone()),
            ("keyword".into(), self.keyword.clone()),
            ("product_type".into(), self.product_type.clone()),
            ("product_no".into(), product_no.clone()),
            ("card_page".into(), self.card_page.clone()),
            ("card_kind".into(), self.card_kind.clone()),
            ("rarelity".into(), self.rarelity.clone()),
            ("tab_item".into(), self.tab_item.clone()),
            ("support_formats".into(), self.support_formats.clone()),
            ("keyword_target".into(), self.keyword_target.clone()),
        ])
    }
}

pub struct ProductCacher {
    root_dir: PathBuf,
    product: Product,
}

impl Display for ProductCacher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ProductCacher {{ root_dir: {:?}, product: {:?} }}",
            self.root_dir, self.product
        )
    }
}

impl ProductCacher {
    fn new(root_dir: PathBuf, product: Product) -> Self {
        Self { root_dir, product }
    }

    fn get_cache_dir_path(&self, cache_root: PathBuf) -> PathBuf {
        let p_type = &self.product.product_type.as_str();

        let sub_path = match *p_type {
            "bo" => {
                format!("booster/{}", self.product.cache_path())
            }
            "st" => {
                format!("starter/{}", self.product.cache_path())
            }
            "sp" => {
                format!("special/{}", self.product.cache_path())
            }
            "pr" => String::from("promotion"),
            _ => panic!("unknown product type"),
        };

        cache_root.join(sub_path)
    }

    fn to_search_query(&self, card_page: i32) -> SearchQuery {
        match self.product.product_type.as_str() {
            "sp" => SearchQuery {
                search: "1".into(),
                keyword: self.product.product_code.clone(),
                product_type: "special_card".into(),
                product_no: "".into(),
                card_page: card_page.to_string(),
                card_kind: "".into(),
                rarelity: "".into(),
                tab_item: "".into(),
                support_formats: "2".into(),
                keyword_target:
                    "カードNo,カード名,カードタイプ,テキスト,イラストレーター,フレーバー".into(),
            },
            "st" | "bo" => {
                let t = match self.product.product_type.as_str() {
                    "st" => "starter",
                    "bo" => "booster",
                    _ => panic!("unknown product type"),
                };
                SearchQuery {
                    search: "1".into(),
                    keyword: "".into(),
                    product_type: t.into(),
                    product_no: self.product.product_code.clone(),
                    card_page: card_page.to_string(),
                    card_kind: "".into(),
                    rarelity: "".into(),
                    tab_item: "".into(),
                    support_formats: "2".into(),
                    keyword_target: "".into(),
                }
            }
            "pr" => SearchQuery {
                search: "1".into(),
                keyword: "".into(),
                product_type: "promotion_card".into(),
                product_no: "".into(),
                card_page: card_page.to_string(),
                card_kind: "".into(),
                rarelity: "".into(),
                tab_item: "".into(),
                support_formats: "2".into(),
                keyword_target: "".into(),
            },
            _ => panic!("unknown product type"),
        }
    }

    async fn cache_first_page(&self) -> Result<(), reqwest::Error> {
        match self.cache_target_page(1).await {
            Ok(page_max) => {
                println!("pages: {}", page_max);
                for i in 2..=page_max {
                    self.cache_target_page(i).await?;
                    sleep(Duration::from_secs(1)).await;
                }

                Ok(())
            }
            Err(e) => {
                println!("{:?}", e);
                Err(e)
            }
        }
    }
    async fn cache_target_page(&self, card_page: i32) -> Result<i32, reqwest::Error> {
        let r = self.root_dir.clone();

        let search_query = self.to_search_query(card_page);
        println!("{:?}", search_query);

        let main_element: Option<String> = match search_query.check_cache(r.clone()) {
            Ok(content_) => Some(content_),
            _ => {
                let form: HashMap<String, String> = search_query.to_hashmap();

                let client: Client = Client::new();

                let url = "https://www.takaratomy.co.jp/products/wixoss/card/card_list.php";

                let res: Response = client
                    .post(url)
                    .header(reqwest::header::COOKIE, "wixAge=conf;")
                    .query(&form)
                    .send()
                    .await?;

                let body: String = res.text().await.unwrap();

                let cache_filename: PathBuf = r.join(&search_query.to_filename());

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

        let mut page_max: i32 = 0;

        if let Some(count) = find_one(&main_element.unwrap(), "h3 p span".into()) {
            let count = extract_number(&count);

            if let Some(count) = count {
                page_max = (count / 21) + 1;

                // 再帰を採用しない
                // if card_page < pages {
                //     self.cache_target_page(card_page + 1).await?;
                // }
            }
        } else {
            println!("not found");
        };

        Ok(page_max)
    }
}

#[tokio::main]
async fn main() {
    let pool = Arc::new(create_db().await);
    let product_repo = ProductRepository::new(pool.clone());

    let cache_dir_root = PathBuf::new();
    let cache_dir = cache_dir_root.join("../webapp/text_cache");

    let products = product_repo.get_all().await.unwrap();

    // 謙虚にシリアルに
    for product in products {
        let pc = ProductCacher::new(cache_dir.clone(), product);
        pc.cache_first_page().await.unwrap();
    }
}

async fn create_db() -> Pool<Postgres> {
    from_filename("../.env").ok();

    let db_url = {
        let host = env::var("DB_HOST").expect("DB_HOST not found in .env");
        let port = env::var("DB_PORT").expect("DB_PORT not found in .env");
        let user = env::var("DB_USER").expect("DB_USER not found in .env");
        let password = env::var("DB_PASSWORD").expect("DB_PASSWORD not found in .env");
        let db_name = env::var("DB_NAME").expect("DB_NAME not found in .env");
        format!(
            "postgres://{}:{}@{}:{}/{}",
            user, password, host, port, db_name
        )
    };

    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(format!("{db_url}?connect_timeout=5").as_str())
        .await
        .expect("Failed to connect to database");
    pool
}
