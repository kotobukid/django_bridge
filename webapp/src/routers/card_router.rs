use crate::repositories::{CardRepository, OnlyCardName};
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Json;
use axum::Router;
use serde::Serialize;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use crate::models::Card;

#[derive(Clone)]
struct RouterState {
    card_repo: CardRepository,
}

pub fn create_card_router(pool: Arc<Pool<Postgres>>) -> Router<AppState> {
    let card_repo = CardRepository::new(pool);
    let state = RouterState {
        card_repo
    };

    Router::new()
        .route("/", get(card_list))
        .route("/api/list.json", get(card_list_json))
        .with_state(state)
}

async fn card_list(State(state): State<RouterState>) -> impl IntoResponse {
    let cards = state.card_repo.get_all().await;

    match cards {
        Ok(card_list) => card_list
            .into_iter()
            .map(|card| card.name)  // OnlyCardNameから直接nameフィールドを取得
            .collect::<Vec<String>>()
            .join("\n"),
        Err(_) => "エラーが発生しました".to_string()
    }
}

#[derive(Serialize)]
struct CardListJson {
    cards: Vec<Card>,
}
async fn card_list_json(
    State(state): State<RouterState>,
) -> Result<Json<CardListJson>, StatusCode> {
    let cards = state
        .card_repo
        .get_all()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let cards = cards.into_iter().map(|card| card.into()).collect::<Vec<Card>>();
    let res: CardListJson = CardListJson { cards };
    Ok(Json(res))
}
