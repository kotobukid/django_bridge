use futures::future::join_all;
use rand::Rng;
use serde_qs as qs;
use std::path::Path;
use tokio::time::{sleep, Duration};
use url::Url;
use webapp::analyze::{
    cache_product_index, collect_card_detail_links, try_mkdir, CardQuery, ProductType,
};

use webapp::analyze::wixoss::Card;

use dotenvy::from_filename;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;

use clap::Parser;
use models::card::CreateCard;
use rayon::iter::ParallelIterator;
use std::sync::Arc;
use tokio::sync::Mutex;
use webapp::repositories::{CardRepository, CardTypeRepository};

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

async fn db(
    pool: Arc<Pool<Postgres>>,
    item: CreateCard,
) -> Result<models::card::Card, sqlx::Error> {
    let card_repo = CardRepository::new(pool.clone());
    Ok(card_repo.upsert(item).await?)
}

#[derive(Parser, Debug)]
struct Args {
    product_type: String, // `#[clap(short,` long)]    を付けなければ位置指定となる
    code: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let product_code = args.code;

    try_mkdir(Path::new("./text_cache")).unwrap();

    let product_type = match args.product_type.to_ascii_lowercase().as_str() {
        "starter" => ProductType::Starter(product_code),
        "booster" => ProductType::Booster(product_code),
        _ => {
            panic!("invalid product type");
        }
    };

    cache_product_index(&product_type, 1).await.unwrap();

    let links = collect_card_detail_links(&product_type).await;

    let pool = Arc::new(create_db().await);

    let card_type_repo = Arc::new(Mutex::new(CardTypeRepository::new(pool.clone())));
    let product_repo = Arc::new(Mutex::new(webapp::repositories::ProductRepository::new(
        pool.clone(),
    )));

    if let Ok(links) = links {
        let tasks: Vec<_> = links
            .iter()
            .map(|link| {
                let link = link.clone();
                let pool = pool.clone();
                let product_type = product_type.clone();
                let card_type_repo = Arc::clone(&card_type_repo);
                let product_repo = Arc::clone(&product_repo);

                tokio::spawn(async move {
                    let card_no = extract_card_no(&link).unwrap();
                    let dir = Path::new("./text_cache/single");
                    let cq: CardQuery =
                        CardQuery::new(card_no.clone().into(), Box::from(dir.to_path_buf()));

                    let text = if cq.check_cache_file_exists() {
                        println!("cache exists for {card_no}");
                        cq.get_cache_text()
                    } else {
                        println!("cache not found. downloading {card_no}");

                        // ランダムな待機時間（1000ms-3000ms）を生成
                        let wait_time = rand::thread_rng().gen_range(1000..=3000);
                        sleep(Duration::from_millis(wait_time)).await;

                        cq.download_card_detail().await
                    };

                    match text {
                        Some(text) => {
                            let c = Card::card_from_html(text.as_str());
                            match c {
                                Some(card) => {
                                    let ct = &card.card_type.code();

                                    let card_type_id = card_type_repo
                                        .lock()
                                        .await
                                        .find_by_code(ct)
                                        .await
                                        .unwrap_or(0);

                                    let product_id = product_repo
                                        .lock()
                                        .await
                                        .get_id_by_code(&product_type.code())
                                        .await
                                        .unwrap_or(0);

                                    let mut cc: CreateCard = card.into();
                                    cc.card_type = card_type_id.to_string().parse::<i32>().unwrap();
                                    cc.product = product_id.to_string().parse::<i32>().unwrap();

                                    db(pool, cc).await.unwrap();
                                }
                                None => eprintln!("card parse error[skip]: {}", card_no),
                            }
                        }
                        None => eprintln!("download error"),
                    }
                })
            })
            .collect();

        join_all(tasks).await;
    }

    Ok(())
}

fn extract_card_no(url_str: &str) -> Option<String> {
    // URLをパース
    let url = Url::parse(url_str).ok()?;

    // クエリ文字列を取得
    let query = url.query()?;

    // クエリパラメータをパース
    let params: std::collections::HashMap<String, String> = qs::from_str(query).ok()?;

    // card_noパラメータを取得
    params.get("card_no").map(|s| s.to_string())
}
