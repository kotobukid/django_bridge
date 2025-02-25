use std::sync::Arc;
use axum::extract::State;
use crate::state::AppState;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use sqlx::{Pool, Postgres};
use crate::models::OnlyCardNameRepository;

#[derive(Clone)]
struct RouterState {
    card_repo: OnlyCardNameRepository,
}

pub fn create_card_router(pool: Arc<Pool<Postgres>>) -> Router<AppState> {
    let only_card_name_repo = OnlyCardNameRepository::new(pool);
    let state = RouterState { card_repo: only_card_name_repo };

    Router::new().route("/", get(card_list))
        .with_state(state)
}

async fn card_list(State(state): State<RouterState>) -> impl IntoResponse {
    state.card_repo
        .get_all()
        .await
        .into_iter()
        .map(|card| card.to_string())
        .collect::<Vec<String>>().join("\n")
}
