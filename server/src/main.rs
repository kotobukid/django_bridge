use axum::Router;
use dotenvy::from_filename;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use tower_http::services::ServeDir;

use std::sync::Arc;
use std::time::Duration;
#[allow(unused_imports)]
use axum::routing::get;
use webapp::routers::{
    admin_process::create_admin_portal_router, card_router::create_card_router,
    product_router::create_product_router,
};
use webapp::state::AppState;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    from_filename("../.env").ok();

    let web_port: u16 = env::var("WEB_PORT").unwrap_or("8000".to_string()).parse().unwrap();
    let django_admin_port: u16 = env::var("DJANGO_ADMIN_PORT").unwrap_or("8200".to_string()).parse().unwrap();

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

    let pool = Arc::new(pool);

    let a_routers = create_admin_portal_router(django_admin_port, web_port);
    let card_router = create_card_router(pool.clone());
    let product_router = create_product_router(pool.clone());
    let api_router = Router::new()
        .nest("/card/", card_router)
        .nest("/product/", product_router);

    let app_state = AppState { 
        db_pool: pool,
        django_admin_port
    };

    let app = Router::new()
        // .route("/hello", get(hello_handler))
        // .nest("/card/", card_router)
        // .nest("/product/", product_router)
        .route("/healthz", get(|| async { "OK" }))
        .nest("/api/", api_router)
        .nest("/admin_operation/", a_routers.0)
        .nest("/admin_proxy/", a_routers.1)
        .nest("/a_static/", a_routers.2)
        .fallback_service(ServeDir::new("../front/dist"))
        .with_state(app_state);

    let web_addr = format!("0.0.0.0:{}", web_port);
    let listener = tokio::net::TcpListener::bind(&web_addr)
        .await
        .expect("Failed to bind port");
    println!("Server is running on http://{}", web_addr);
    axum::serve(listener, app).await?;

    Ok(())
}

#[allow(dead_code)]
async fn hello_handler() -> &'static str {
    "Hello World"
}