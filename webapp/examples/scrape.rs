use rand::Rng;
use serde_qs as qs;
use std::path::Path;
use tokio::time::{sleep, Duration};
use url::Url;
use webapp::analyze::{
    cache_product_index, collect_card_detail_links, try_mkdir, CardQuery, ProductType,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    try_mkdir(Path::new("./text_cache")).unwrap();

    // let product_type = ProductType::Starter(String::from("WDA-F03"));
    let product_type = ProductType::Booster(String::from("WX24-P4"));

    cache_product_index(&product_type, 1).await.unwrap();

    let links = collect_card_detail_links(&product_type).await;

    if let Ok(links) = links {
        for link in links {
            let card_no = extract_card_no(&link).unwrap();
            println!("{card_no}");

            // ランダムな待機時間（1000ms-3000ms）を生成
            let wait_time = rand::rng().random_range(1000..=3000);
            sleep(Duration::from_millis(wait_time)).await;

            let cq: CardQuery = CardQuery::from_card_no(card_no.into());
            let text: Option<String> = cq.download_card_detail("./text_cache/single").await;
            println!("{}", text.unwrap_or("detail download error".into()));
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
