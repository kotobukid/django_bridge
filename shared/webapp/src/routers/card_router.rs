//! カードルーター
//! 
//! カード一覧の取得APIを提供

use crate::repositories::CardRepository;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use models::card::Card;
use serde::Serialize;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use thiserror::Error;

/// カードルーターのエラー型
#[derive(Debug, Error)]
pub enum CardRouterError {
    #[error("データベースエラー: {0}")]
    Database(#[from] sqlx::Error),
    #[error("シリアライゼーションエラー: {0}")]
    Serialization(#[from] serde_json::Error),
}

/// ルーター状態
#[derive(Clone)]
struct RouterState {
    card_repo: CardRepository,
}

/// カードルーターを作成
/// 
/// # 引数
/// 
/// * `pool` - データベース接続プール
/// 
/// # 戻り値
/// 
/// 設定されたAxumルーター
pub fn create_card_router(pool: Arc<Pool<Postgres>>) -> Router<AppState> {
    let card_repo = CardRepository::new(pool);
    let state = RouterState { card_repo };

    Router::new()
        .route("/", get(card_list))
        .route("/list.json", get(card_list_json))
        .with_state(state)
}

/// カード一覧をテキスト形式で取得
/// 
/// # 引数
/// 
/// * `state` - ルーター状態
/// 
/// # 戻り値
/// 
/// カード名を改行区切りで連結した文字列
async fn card_list(State(state): State<RouterState>) -> impl IntoResponse {
    println!("カード一覧テキスト形式の取得を開始");
    
    match state.card_repo.get_all().await {
        Ok(card_list) => {
            let result = card_list
                .into_iter()
                .map(|card| card.name)
                .collect::<Vec<String>>()
                .join("\n");
            println!("カード一覧テキスト形式の取得が成功: {}件", result.lines().count());
            result
        }
        Err(e) => {
            eprintln!("カード一覧取得エラー: {:?}", e);
            "カード一覧の取得に失敗しました".to_string()
        }
    }
}

/// JSON形式のカード一覧レスポンス
#[derive(Serialize)]
pub struct CardListJson {
    /// カード配列
    cards: Vec<Card>,
    /// 総件数
    total: usize,
}

impl CardListJson {
    /// 新しいCardListJsonを作成
    pub fn new(cards: Vec<Card>) -> Self {
        let total = cards.len();
        Self { cards, total }
    }

    /// JSON文字列に変換（エラーハンドリング付き）
    pub fn to_json(&self) -> Result<String, CardRouterError> {
        serde_json::to_string(&self).map_err(CardRouterError::from)
    }
}
/// カード一覧をJSON形式で取得
/// 
/// # 引数
/// 
/// * `state` - ルーター状態
/// 
/// # 戻り値
/// 
/// JSON形式のカード一覧またはエラーステータス
async fn card_list_json(
    State(state): State<RouterState>,
) -> Result<Json<CardListJson>, StatusCode> {
    println!("カード一覧JSON形式の取得を開始");
    
    let cards = state
        .card_repo
        .get_all()
        .await
        .map_err(|e| {
            eprintln!("カード一覧取得エラー: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        
    let cards: Vec<Card> = cards
        .into_iter()
        .map(|card| card.into())
        .collect();
        
    let total = cards.len();
    println!("カード一覧JSON形式の取得が成功: {}件", total);
    
    let res = CardListJson::new(cards);
    Ok(Json(res))
}
