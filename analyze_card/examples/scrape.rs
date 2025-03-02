use analyze_card::{
    try_mkdir,
    ProductType,
    collect_card_detail_links,
    cache_product_index
};
use std::path::Path;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    try_mkdir(Path::new("./text_cache")).unwrap();

    // let product_type = ProductType::Starter(String::from("WDA-F03"));
    let product_type = ProductType::Booster(String::from("WX24-P4"));

    cache_product_index(&product_type, 1).await.unwrap();

    let links = collect_card_detail_links(&product_type).await;

    if let Ok(links) = links {
        links.into_iter().for_each(|link| {
            println!("{}", link);
        });
    }

    Ok(())
}