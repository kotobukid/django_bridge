use analyze_card::{
    CardQuery,
};

#[tokio::main]
async fn main() {
    let cq: CardQuery = CardQuery::from_card_no("WX24-P4-001U".into());
    let text: Option<String> = cq.download_card_detail("./text_cache/single").await;
    println!("{}", text.unwrap_or("detail download error".into()))
}