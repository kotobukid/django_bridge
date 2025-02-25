// use axum::response::IntoResponse;
// use axum::Router;
// use axum::routing::get;
// use sqlm::state::AppState;
//
// pub fn create_card_router() -> Router<AppState> {
//     Router::new()
//         .route("/", get(card_list))
// }
//
// async fn card_list() -> impl IntoResponse {
//     "card list".into_response()
// }