use webapp::admin_process;

use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::{routing::get, Router};
use dotenvy::from_filename;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;

use std::sync::Arc;
use std::time::Duration;
use webapp::routers::card_router::create_card_router;
use webapp::state::AppState;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
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

    let pool = Arc::new(pool);

    let a_routers = admin_process::create_admin_portal_router("/admin_operation/");
    let card_router = create_card_router(pool.clone());

    let app_state = AppState { db_pool: pool };

    let app = Router::new()
        .route("/", get(get_index))
        .nest("/card/", card_router)
        .nest("/admin_operation/", a_routers.0)
        .nest("/admin_proxy/", a_routers.1)
        .nest("/a_static/", a_routers.2)
        .with_state(app_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("Failed to bind port");
    println!("Server is running on http://localhost:3000");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_index() -> impl IntoResponse {
    (
        StatusCode::OK,
        Html(
            r#"<!doctype html><html><title>HOME</title><body>
            <a href="/card/">Card list</a>
            <br /><a href="/admin_operation/">manage Django Server</a>
    </body></html>"#,
        ),
    )
        .into_response()
}
