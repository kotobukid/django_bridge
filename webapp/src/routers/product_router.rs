use crate::models::card::Product;
use crate::repositories::ProductRepository;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Json;
use axum::Router;
use serde::Serialize;
use sqlx::{Error, Pool, Postgres};
use std::sync::Arc;

#[derive(Clone)]
struct RouterState {
    repo: ProductRepository,
}

pub fn create_product_router(pool: Arc<Pool<Postgres>>) -> Router<AppState> {
    let repo = ProductRepository::new(pool);
    let state = RouterState { repo };

    Router::new()
        .route("/", get(product_list))
        .route("/api/list.json", get(product_list_json))
        .with_state(state)
}

async fn product_list(State(state): State<RouterState>) -> impl IntoResponse {
    let products: Result<Vec<Product>, Error> = state.repo.get_all().await;

    match products {
        Ok(product_list) => product_list
            .into_iter()
            .map(|product| product.name.clone())
            .collect::<Vec<String>>()
            .join("\n"),
        Err(_) => "エラーが発生しました".to_string(),
    }
}

#[derive(Serialize)]
struct CardListJson {
    products: Vec<Product>,
}
async fn product_list_json(
    State(state): State<RouterState>,
) -> Result<Json<CardListJson>, StatusCode> {
    let cards = state
        .repo
        .get_all()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let res: CardListJson = CardListJson { products: cards };
    Ok(Json(res))
}
