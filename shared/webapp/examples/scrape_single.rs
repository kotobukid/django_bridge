use std::path::Path;
use webapp::analyze::CardQuery;

#[tokio::main]
async fn main() {
    let card_no = "WX24-P4-001U";
    let dir = Path::new("./text_cache/single");
    let cq: CardQuery = CardQuery::new(card_no.into(), Box::from(dir.to_path_buf()));
    let text: Option<String> = cq.download_card_detail().await.ok();
    println!("{}", text.unwrap_or("detail download error".into()))
}
