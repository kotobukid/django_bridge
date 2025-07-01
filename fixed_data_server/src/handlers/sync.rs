use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde::Serialize;
use sqlx::PgPool;
use tracing::{info, error};

use crate::sync::{sync_push_all, sync_pull_all, SyncClient};

#[derive(Serialize)]
pub struct SyncResponse {
    pub success: bool,
    pub message: String,
    pub items_affected: Option<usize>,
    pub details: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct SyncStatusResponse {
    pub success: bool,
    pub last_sync_attempt: Option<String>,
    pub admin_backend_connected: bool,
    pub local_overrides_count: i64,
    pub sync_status: Option<serde_json::Value>,
}

/// Push all local feature overrides to admin backend
pub async fn push_sync(
    State(pool): State<PgPool>,
) -> Result<Json<SyncResponse>, StatusCode> {
    info!("Starting push sync to admin backend");

    match sync_push_all(&pool).await {
        Ok(push_response) => {
            let details = serde_json::json!({
                "items_received": push_response.items_received,
                "items_created": push_response.items_created,
                "items_updated": push_response.items_updated,
                "error_count": push_response.errors.len()
            });

            let message = if push_response.errors.is_empty() {
                format!("Successfully pushed {} items to admin backend", push_response.items_received)
            } else {
                format!("Pushed {} items with {} errors", push_response.items_received, push_response.errors.len())
            };

            Ok(Json(SyncResponse {
                success: true,
                message,
                items_affected: Some(push_response.items_received as usize),
                details: Some(details),
            }))
        }
        Err(e) => {
            error!("Push sync failed: {}", e);
            Ok(Json(SyncResponse {
                success: false,
                message: format!("Push sync failed: {}", e),
                items_affected: None,
                details: None,
            }))
        }
    }
}

/// Pull all feature overrides from admin backend
pub async fn pull_sync(
    State(pool): State<PgPool>,
) -> Result<Json<SyncResponse>, StatusCode> {
    info!("Starting pull sync from admin backend");

    match sync_pull_all(&pool).await {
        Ok(imported_count) => {
            Ok(Json(SyncResponse {
                success: true,
                message: format!("Successfully imported {} items from admin backend", imported_count),
                items_affected: Some(imported_count),
                details: None,
            }))
        }
        Err(e) => {
            error!("Pull sync failed: {}", e);
            Ok(Json(SyncResponse {
                success: false,
                message: format!("Pull sync failed: {}", e),
                items_affected: None,
                details: None,
            }))
        }
    }
}

/// Get sync status and admin backend connectivity
pub async fn get_sync_status(
    State(pool): State<PgPool>,
) -> Result<Json<SyncStatusResponse>, StatusCode> {
    info!("Checking sync status");

    // Get local overrides count
    let local_count = match sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM wix_card_feature_override"
    )
    .fetch_one(&pool)
    .await
    {
        Ok(count) => count,
        Err(e) => {
            error!("Failed to count local overrides: {}", e);
            return Ok(Json(SyncStatusResponse {
                success: false,
                last_sync_attempt: None,
                admin_backend_connected: false,
                local_overrides_count: 0,
                sync_status: None,
            }));
        }
    };

    // Try to connect to admin backend and get status
    let (admin_connected, sync_status) = match SyncClient::new().await {
        Ok(mut client) => {
            let client_id = std::env::var("SYNC_CLIENT_ID")
                .unwrap_or_else(|_| "wx_db_local".to_string());

            match client.get_sync_status(client_id).await {
                Ok(status) => {
                    let status_json = serde_json::json!({
                        "server_time_seconds": status.server_time.map(|ts| ts.seconds).unwrap_or(0),
                        "total_feature_overrides": status.total_feature_overrides,
                        "total_confirmed_features": status.total_confirmed_features,
                        "sync_status_count": status.sync_status.len()
                    });
                    (true, Some(status_json))
                }
                Err(e) => {
                    error!("Failed to get admin backend status: {}", e);
                    (false, None)
                }
            }
        }
        Err(e) => {
            error!("Failed to connect to admin backend: {}", e);
            (false, None)
        }
    };

    Ok(Json(SyncStatusResponse {
        success: true,
        last_sync_attempt: Some(chrono::Utc::now().to_rfc3339()),
        admin_backend_connected: admin_connected,
        local_overrides_count: local_count,
        sync_status,
    }))
}

/// Bidirectional sync: pull first, then push
pub async fn bidirectional_sync(
    State(pool): State<PgPool>,
) -> Result<Json<SyncResponse>, StatusCode> {
    info!("Starting bidirectional sync (pull then push)");

    // First, pull remote changes
    let pull_result = sync_pull_all(&pool).await;
    let imported_count = match pull_result {
        Ok(count) => count,
        Err(e) => {
            error!("Pull phase of bidirectional sync failed: {}", e);
            return Ok(Json(SyncResponse {
                success: false,
                message: format!("Pull phase failed: {}", e),
                items_affected: None,
                details: None,
            }));
        }
    };

    // Then, push local changes
    let push_result = sync_push_all(&pool).await;
    match push_result {
        Ok(push_response) => {
            let details = serde_json::json!({
                "pull": {
                    "imported_count": imported_count
                },
                "push": {
                    "items_received": push_response.items_received,
                    "items_created": push_response.items_created,
                    "items_updated": push_response.items_updated,
                    "error_count": push_response.errors.len()
                }
            });

            let message = format!(
                "Bidirectional sync completed: imported {} items, pushed {} items",
                imported_count, push_response.items_received
            );

            Ok(Json(SyncResponse {
                success: true,
                message,
                items_affected: Some(imported_count + push_response.items_received as usize),
                details: Some(details),
            }))
        }
        Err(e) => {
            error!("Push phase of bidirectional sync failed: {}", e);
            Ok(Json(SyncResponse {
                success: false,
                message: format!("Pull succeeded ({} items), but push failed: {}", imported_count, e),
                items_affected: Some(imported_count),
                details: Some(serde_json::json!({
                    "pull": { "imported_count": imported_count },
                    "push": { "error": e.to_string() }
                })),
            }))
        }
    }
}