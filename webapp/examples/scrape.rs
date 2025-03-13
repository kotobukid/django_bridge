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

use std::sync::Arc;
use webapp::models::card::CreateCard;
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
) -> Result<webapp::models::card::Card, sqlx::Error> {
    let card_repo = CardRepository::new(pool.clone());
    Ok(card_repo.upsert(item).await?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    try_mkdir(Path::new("./text_cache")).unwrap();

    // let product_type = ProductType::Booster(String::from("WXDi-P15"));
    // let product_type = ProductType::Booster(String::from("WXDi-P11"));
    let product_type = ProductType::Booster(String::from("WXDi-D09"));
    // let product_type = ProductType::Booster(String::from("WX24-P4"));

    cache_product_index(&product_type, 1).await.unwrap();

    let links = collect_card_detail_links(&product_type).await;

    let pool = Arc::new(create_db().await);
    let mut card_type_repo = CardTypeRepository::new(pool.clone());
    let _ = card_type_repo.create_cache().await;

    let mut product_repo = webapp::repositories::ProductRepository::new(pool.clone());
    product_repo.create_cache().await;

    if let Ok(links) = links {
        for link in links {
            let pool = pool.clone();
            let card_no = extract_card_no(&link).unwrap();

            let dir = Path::new("./text_cache/single");
            let cq: CardQuery =
                CardQuery::new(card_no.clone().into(), Box::from(dir.to_path_buf()));

            let text = if cq.check_cache_file_exists() {
                println!("cache exists {card_no}");
                cq.get_cache_text()
            } else {
                // ランダムな待機時間（1000ms-3000ms）を生成
                let wait_time = rand::rng().random_range(1000..=3000);
                sleep(Duration::from_millis(wait_time)).await;

                cq.download_card_detail().await
            };
            match text {
                Some(text) => {
                    // let t = Card::detect_card_type(text.as_str());
                    let c = Card::card_from_html(text.as_str());
                    println!("{:?}", c);
                    match c {
                        Some(card) => {
                            let ct = &card.card_type.code();
                            let card_type_id = card_type_repo.find_by_code(ct).await;
                            let card_type_id = card_type_id.unwrap_or(0);

                            let product_id = product_repo
                                .get_id_by_code(&product_type.code())
                                .await
                                .unwrap_or(0);
                            let mut cc: CreateCard = card.into();
                            cc.card_type = card_type_id.to_string().parse::<i32>().unwrap();
                            cc.product = product_id.to_string().parse::<i32>().unwrap();

                            db(pool, cc).await?;
                        }
                        None => {
                            eprintln!("card parse error[skip]: {}", card_no);
                        }
                    }
                }
                None => {
                    panic!("download error");
                }
            }
        }
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
