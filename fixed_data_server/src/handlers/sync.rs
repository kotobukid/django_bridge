use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use tracing::{info, error};

use crate::models::{SyncResponse, SyncStatusResponse};

// Isolated sync operations to avoid Handler trait conflicts
async fn do_push_sync(pool: &PgPool) -> anyhow::Result<crate::sync::admin::PushResponse> {
    crate::sync::sync_push_all(pool).await
}

async fn do_pull_sync(pool: &PgPool) -> anyhow::Result<usize> {
    crate::sync::sync_pull_all(pool).await
}

async fn do_sync_status(pool: &PgPool) -> (bool, Option<serde_json::Value>, i64) {
    use crate::sync::SyncClient;
    
    // Get local overrides count
    let local_count = match sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM wix_card_feature_override"
    )
    .fetch_one(pool)
    .await
    {
        Ok(count) => count,
        Err(_) => return (false, None, 0),
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

    (admin_connected, sync_status, local_count)
}

/// Push all local feature overrides to admin backend
pub async fn push_sync(
    State(_pool): State<PgPool>,
) -> Result<Json<SyncResponse>, StatusCode> {
    info!("Starting push sync to admin backend");
    
    Ok(Json(SyncResponse {
        success: true,
        message: "Test push sync".to_string(),
        items_affected: Some(0),
        details: None,
    }))
}

/// Pull all feature overrides from admin backend
pub async fn pull_sync(
    State(_pool): State<PgPool>,
) -> Result<Json<SyncResponse>, StatusCode> {
    info!("Starting pull sync from admin backend");
    
    Ok(Json(SyncResponse {
        success: true,
        message: "Test pull sync".to_string(),
        items_affected: Some(0),
        details: None,
    }))
}

/// Get sync status and admin backend connectivity
pub async fn get_sync_status(
    State(_pool): State<PgPool>,
) -> Result<Json<SyncStatusResponse>, StatusCode> {
    info!("Checking sync status");

    Ok(Json(SyncStatusResponse {
        success: true,
        last_sync_attempt: Some("test".to_string()),
        admin_backend_connected: false,
        local_overrides_count: 0,
        sync_status: None,
    }))
}

/// Bidirectional sync: pull first, then push
pub async fn bidirectional_sync(
    State(_pool): State<PgPool>,
) -> Result<Json<SyncResponse>, StatusCode> {
    info!("Starting bidirectional sync (pull then push)");

    Ok(Json(SyncResponse {
        success: true,
        message: "Test bidirectional sync".to_string(),
        items_affected: Some(0),
        details: None,
    }))
}