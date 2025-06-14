use analyzer::card_analyzer::{
    RawCardWithProduct, SimpleRawCardAnalyzer, analyze_and_save_card_with_product_id,
    analyze_raw_cards_with_product_batch,
};
use clap::Parser;
use dotenvy;
use dotenvy::from_filename;
use sqlx::PgPool;
use std::env;

/// WIXOSS カード解析ツール
///
/// RawCardテーブルから未解析のカードを取得し、解析してCardテーブルに保存します。
#[derive(Parser, Debug)]
#[command(name = "analyzer")]
#[command(about = "WIXOSS カード解析ツール")]
struct Args {
    /// 処理するカードの最大数（デフォルト: 100）
    #[arg(long, default_value = "100")]
    limit: i64,

    /// バッチサイズ（一度に処理するカード数、デフォルト: 10）
    #[arg(long, default_value = "10")]
    batch_size: usize,

    /// 詳細ログを出力
    #[arg(short, long)]
    verbose: bool,

    /// 特定のカード番号のみを解析
    #[arg(long)]
    card_number: Option<String>,

    /// 強制再解析（既に解析済みのカードも対象にする）
    #[arg(long)]
    force: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("WIXOSS カード解析ツール");

    let workspace_env = format!(
        "{}/.env",
        env::var("CARGO_WORKSPACE_DIR").unwrap_or_default()
    );
    let env_paths = [
        ".env",                 // カレントディレクトリ
        "../.env",              // 一つ上のディレクトリ
        "../../.env",           // 二つ上のディレクトリ（nested crateの場合）
        workspace_env.as_str(), // CARGO_WORKSPACE_DIRが設定されている場合
    ];

    for path in &env_paths {
        if std::path::Path::new(path).exists() {
            from_filename(path).ok();
            break;
        }
    }

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

    // Create database pool
    let pool = PgPool::connect(&db_url).await?;

    // Build query based on arguments
    let mut query = String::from(
        "SELECT id, card_number, name, raw_html, skill_text, life_burst_text, 
               source_url, scraped_at, last_analyzed_at, is_analyzed, analysis_error,
               product_id
        FROM wix_rawcard",
    );

    let mut conditions = Vec::new();

    if let Some(card_no) = &args.card_number {
        conditions.push(format!("card_number = '{}'", card_no));
    }

    if !args.force {
        conditions.push("is_analyzed = false".to_string());
    }

    if !conditions.is_empty() {
        query.push_str(&format!(" WHERE {}", conditions.join(" AND ")));
    }

    query.push_str(&format!(" ORDER BY scraped_at DESC LIMIT {}", args.limit));

    if args.verbose {
        println!("Query: {}", query);
    }

    // Query for cards to analyze
    let raw_cards: Vec<RawCardWithProduct> = sqlx::query_as::<_, RawCardWithProduct>(&query)
        .fetch_all(&pool)
        .await?;

    if raw_cards.is_empty() {
        println!("解析対象のカードが見つかりませんでした。");
        if !args.force {
            println!("既に解析済みのカードも含める場合は --force オプションを使用してください。");
        }
        return Ok(());
    }

    println!("解析対象カード数: {}", raw_cards.len());

    // Process cards in batches
    let mut total_success = 0;
    let mut total_errors = 0;

    for (batch_index, batch) in raw_cards.chunks(args.batch_size).enumerate() {
        println!(
            "\nバッチ {}/{} を処理中...",
            batch_index + 1,
            (raw_cards.len() + args.batch_size - 1) / args.batch_size
        );

        if args.verbose {
            // 詳細モード: 個別に処理
            let analyzer = SimpleRawCardAnalyzer::new();
            for raw_card_with_product in batch {
                println!(
                    "  解析中: {} - {} (product_id: {:?})",
                    raw_card_with_product.card_number,
                    raw_card_with_product.name,
                    raw_card_with_product.product_id
                );
                let raw_card = raw_card_with_product.to_raw_card_db();

                match analyzer
                    .analyze_with_product_id(&raw_card, raw_card_with_product.product_id)
                    .await
                {
                    Ok(create_card) => {
                        println!("    ✓ 解析成功");
                        if args.verbose {
                            println!(
                                "      - パワー: {}",
                                create_card.power.unwrap_or_else(|| "N/A".to_string())
                            );
                            println!(
                                "      - ライフバースト: {}",
                                if create_card.has_burst == 1 {
                                    "あり"
                                } else {
                                    "なし"
                                }
                            );
                            println!(
                                "      - 特徴ビット: {:#b} / {:#b}",
                                create_card.feature_bits1, create_card.feature_bits2
                            );
                        }

                        match analyze_and_save_card_with_product_id(
                            &raw_card,
                            raw_card_with_product.product_id,
                            &pool,
                        )
                        .await
                        {
                            Ok(card_id) => {
                                println!("    ✓ 保存完了 (ID: {})", card_id);
                                total_success += 1;
                            }
                            Err(e) => {
                                println!("    ✗ 保存失敗: {}", e);
                                total_errors += 1;
                            }
                        }
                    }
                    Err(e) => {
                        println!("    ✗ 解析失敗: {}", e);
                        total_errors += 1;
                    }
                }
            }
        } else {
            // バッチモード: 一括処理
            let results = analyze_raw_cards_with_product_batch(batch.to_vec(), &pool).await;

            for (i, result) in results.iter().enumerate() {
                let raw_card = &batch[i];
                match result {
                    Ok(card_id) => {
                        println!(
                            "  ✓ {} - {} (ID: {})",
                            raw_card.card_number, raw_card.name, card_id
                        );
                        total_success += 1;
                    }
                    Err(e) => {
                        println!("  ✗ {} - {} : {}", raw_card.card_number, raw_card.name, e);
                        total_errors += 1;
                    }
                }
            }
        }

        println!(
            "  バッチ完了: 成功 {}, エラー {}",
            batch
                .iter()
                .zip(if args.verbose {
                    vec![(); batch.len()]
                } else {
                    vec![(); batch.len()]
                })
                .filter(|(_, _)| true)
                .count()
                - total_errors.min(batch.len()),
            total_errors.min(batch.len())
        );
    }

    println!("\n=== 解析完了 ===");
    println!("成功: {}", total_success);
    println!("エラー: {}", total_errors);
    println!("合計: {}", raw_cards.len());

    if total_errors > 0 {
        println!("\nエラーの詳細は --verbose オプションで確認できます。");
    }

    Ok(())
}
