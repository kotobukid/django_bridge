use cacher::ProductCacher;
use models::product::Product;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example product for demonstration
    use models::product::ProductDb;
    
    let product = Product(ProductDb {
        id: 1,
        name: "ディーヴァセレクション".to_string(),
        product_code: "WXDi-P01".to_string(),
        url: None,
        product_type: "bo".to_string(),
        sort_asc: 0,
    });

    let cache_dir = PathBuf::from("./test_cache");
    let cacher = ProductCacher::new(cache_dir, product);

    println!("Starting cache process for: {}", cacher);
    
    match cacher.cache_all_pages().await {
        Ok(_) => println!("Successfully cached all pages"),
        Err(e) => eprintln!("Error during caching: {}", e),
    }

    Ok(())
}