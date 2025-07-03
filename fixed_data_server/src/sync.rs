use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::env;
use tonic::metadata::AsciiMetadataValue;
use tonic::transport::{Channel, ClientTlsConfig};
use tonic::Request;
use tracing::{info, warn};

// Include the generated gRPC code
pub mod admin {
    tonic::include_proto!("admin");
}

use admin::{admin_sync_client::AdminSyncClient, FeatureOverride, PullRequest, PushResponse};

// PostgreSQL database model for compatibility with existing table schema
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PgCardFeatureOverride {
    pub pronunciation: String,
    pub fixed_bits1: Option<i64>,
    pub fixed_bits2: Option<i64>,
    pub fixed_burst_bits: Option<i64>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub note: Option<String>,
}

pub struct SyncClient {
    client: AdminSyncClient<Channel>,
    api_key: String,
}

impl SyncClient {
    pub async fn new() -> Result<Self> {
        let admin_url = env::var("ADMIN_BACKEND_URL")
            .unwrap_or_else(|_| "https://ik1-341-30725.vs.sakura.ne.jp:50051".to_string());

        let api_key = env::var("ADMIN_BACKEND_API_KEY")
            .context("ADMIN_BACKEND_API_KEY environment variable is required")?;

        info!("Connecting to admin backend at: {}", admin_url);

        // Configure TLS for the connection with multiple fallback strategies
        let channel = if admin_url.starts_with("https://") {
            info!("Configuring TLS connection for: {}", admin_url);

            // Strategy 1: Standard TLS with explicit certificate authority
            let endpoint = Channel::from_shared(admin_url.clone())?;

            // Try different TLS configurations with proper certificate validation
            let strategies: Vec<(&str, Box<dyn Fn() -> ClientTlsConfig>)> = vec![
                (
                    "TLS with system certificate store",
                    Box::new(|| {
                        ClientTlsConfig::new()
                            .domain_name("ik1-341-30725.vs.sakura.ne.jp")
                            .with_enabled_roots() // Use system certificate store (includes Let's Encrypt)
                    }),
                ),
                (
                    "Standard TLS with default settings",
                    Box::new(|| {
                        ClientTlsConfig::new().domain_name("ik1-341-30725.vs.sakura.ne.jp")
                    }),
                ),
            ];

            let mut last_error = None;

            for (strategy_name, tls_config_fn) in strategies {
                info!("Trying TLS strategy: {}", strategy_name);

                let tls_config = tls_config_fn();
                let channel_result = endpoint.clone().tls_config(tls_config)?.connect().await;

                match channel_result {
                    Ok(ch) => {
                        info!("✅ Successfully connected using: {}", strategy_name);
                        return Ok(Self {
                            client: AdminSyncClient::new(ch),
                            api_key,
                        });
                    }
                    Err(e) => {
                        warn!("❌ Failed with {}: {}", strategy_name, e);
                        last_error = Some(e);
                    }
                }
            }

            // As a last resort for development environments only
            if env::var("DEVELOPMENT_MODE").is_ok() {
                warn!("⚠️  DEVELOPMENT MODE: Attempting connection with relaxed certificate validation");
                warn!("⚠️  This should NEVER be used in production!");

                // This would be the only place where we might relax security,
                // and only when explicitly enabled for development
                return Err(anyhow::anyhow!(
                    "Even development mode TLS failed. Check certificate configuration."
                ));
            }

            return Err(anyhow::anyhow!(
                "All TLS strategies failed. Last error: {:?}",
                last_error
            ));
        } else {
            Channel::from_shared(admin_url.clone())?
                .connect()
                .await
                .with_context(|| format!("Failed to establish connection to: {}", admin_url))?
        };

        let client = AdminSyncClient::new(channel);

        Ok(Self { client, api_key })
    }

    fn add_auth_header<T>(&self, mut request: Request<T>) -> Request<T> {
        let api_key_value: AsciiMetadataValue =
            self.api_key.parse().expect("API key should be valid ASCII");
        request.metadata_mut().insert("api-key", api_key_value);
        request
    }

    pub async fn push_overrides(
        &mut self,
        overrides: Vec<FeatureOverride>,
    ) -> Result<PushResponse> {
        info!(
            "Pushing {} feature overrides to admin backend",
            overrides.len()
        );

        let stream = tokio_stream::iter(overrides);
        let request = Request::new(stream);
        let request = self.add_auth_header(request);

        let response = self
            .client
            .push_feature_overrides(request)
            .await
            .context("Failed to push feature overrides")?;

        let push_response = response.into_inner();
        info!(
            "Push completed: {} received, {} created, {} updated",
            push_response.items_received, push_response.items_created, push_response.items_updated
        );

        if !push_response.errors.is_empty() {
            warn!("Push completed with errors: {:?}", push_response.errors);
        }

        Ok(push_response)
    }

    pub async fn pull_overrides(
        &mut self,
        since: Option<DateTime<Utc>>,
    ) -> Result<Vec<FeatureOverride>> {
        info!("Pulling feature overrides from admin backend");

        let request = PullRequest {
            since: since.map(|dt| prost_types::Timestamp {
                seconds: dt.timestamp(),
                nanos: dt.timestamp_subsec_nanos() as i32,
            }),
            limit: None,
        };

        let request = Request::new(request);
        let request = self.add_auth_header(request);

        let mut stream = self
            .client
            .pull_feature_overrides(request)
            .await
            .context("Failed to pull feature overrides")?
            .into_inner();

        let mut overrides = Vec::new();
        while let Some(override_item) = stream.message().await? {
            overrides.push(override_item);
        }

        info!(
            "Pulled {} feature overrides from admin backend",
            overrides.len()
        );
        Ok(overrides)
    }

    pub async fn get_sync_status(&mut self, client_id: String) -> Result<admin::StatusResponse> {
        info!(
            "Sending get_sync_status request for client_id: {}",
            client_id
        );
        let request = admin::StatusRequest { client_id };
        let request = Request::new(request);
        let request = self.add_auth_header(request);

        let response = self
            .client
            .get_sync_status(request)
            .await
            .with_context(|| "Failed to execute get_sync_status gRPC call")?;

        info!("Received sync status response successfully");
        Ok(response.into_inner())
    }
}

// Data conversion functions between PostgreSQL and gRPC formats
pub mod conversion {
    use super::*;

    pub fn to_grpc_override(override_item: &PgCardFeatureOverride) -> FeatureOverride {
        FeatureOverride {
            pronunciation: override_item.pronunciation.clone(),
            fixed_bits1: override_item.fixed_bits1.unwrap_or(0),
            fixed_bits2: override_item.fixed_bits2.unwrap_or(0),
            fixed_burst_bits: override_item.fixed_burst_bits.unwrap_or(0),
            created_at: override_item.created_at.map(|dt| prost_types::Timestamp {
                seconds: dt.timestamp(),
                nanos: dt.timestamp_subsec_nanos() as i32,
            }),
            updated_at: override_item.updated_at.map(|dt| prost_types::Timestamp {
                seconds: dt.timestamp(),
                nanos: dt.timestamp_subsec_nanos() as i32,
            }),
            note: override_item.note.clone(),
        }
    }

    pub fn from_grpc_override(grpc_override: &FeatureOverride) -> PgCardFeatureOverride {
        PgCardFeatureOverride {
            pronunciation: grpc_override.pronunciation.clone(),
            fixed_bits1: Some(grpc_override.fixed_bits1),
            fixed_bits2: Some(grpc_override.fixed_bits2),
            fixed_burst_bits: Some(grpc_override.fixed_burst_bits),
            created_at: grpc_override.created_at.as_ref().map(|ts| {
                DateTime::from_timestamp(ts.seconds, ts.nanos as u32).unwrap_or_default()
            }),
            updated_at: grpc_override.updated_at.as_ref().map(|ts| {
                DateTime::from_timestamp(ts.seconds, ts.nanos as u32).unwrap_or_default()
            }),
            note: grpc_override.note.clone(),
        }
    }
}

// Sync operations
pub async fn sync_push_all(pool: &PgPool) -> Result<PushResponse> {
    let mut sync_client = SyncClient::new().await?;

    // Get all local overrides
    let local_overrides = sqlx::query_as::<_, PgCardFeatureOverride>(
        "SELECT pronunciation, fixed_bits1, fixed_bits2, fixed_burst_bits, 
                created_at, updated_at, note 
         FROM wix_card_feature_override 
         ORDER BY updated_at DESC",
    )
    .fetch_all(pool)
    .await
    .context("Failed to fetch local feature overrides")?;

    // Convert to gRPC format
    let grpc_overrides: Vec<FeatureOverride> = local_overrides
        .iter()
        .map(conversion::to_grpc_override)
        .collect();

    // Push to admin backend
    sync_client.push_overrides(grpc_overrides).await
}

pub async fn sync_pull_all(pool: &PgPool) -> Result<usize> {
    let mut sync_client = SyncClient::new().await?;

    // Pull all overrides from admin backend
    let remote_overrides = sync_client.pull_overrides(None).await?;

    let mut imported_count = 0;

    // Import each override (upsert logic with timestamp comparison)
    for grpc_override in remote_overrides {
        let local_override = conversion::from_grpc_override(&grpc_override);

        // Check if local version exists and is newer
        let existing = sqlx::query_scalar::<_, Option<chrono::DateTime<chrono::Utc>>>(
            "SELECT updated_at FROM wix_card_feature_override WHERE pronunciation = $1",
        )
        .bind(&local_override.pronunciation)
        .fetch_optional(pool)
        .await?;

        let should_update = match existing {
            Some(Some(existing_updated)) => {
                let remote_updated = local_override.updated_at.unwrap_or_default();
                remote_updated > existing_updated
            }
            Some(None) | None => true, // New record or no timestamp
        };

        if should_update {
            sqlx::query(
                "INSERT INTO wix_card_feature_override 
                 (pronunciation, fixed_bits1, fixed_bits2, fixed_burst_bits, created_at, updated_at, note)
                 VALUES ($1, $2, $3, $4, $5, $6, $7)
                 ON CONFLICT (pronunciation) 
                 DO UPDATE SET 
                    fixed_bits1 = EXCLUDED.fixed_bits1,
                    fixed_bits2 = EXCLUDED.fixed_bits2,
                    fixed_burst_bits = EXCLUDED.fixed_burst_bits,
                    updated_at = EXCLUDED.updated_at,
                    note = EXCLUDED.note"
            )
            .bind(&local_override.pronunciation)
            .bind(local_override.fixed_bits1)
            .bind(local_override.fixed_bits2)
            .bind(local_override.fixed_burst_bits)
            .bind(local_override.created_at)
            .bind(local_override.updated_at)
            .bind(local_override.note)
            .execute(pool)
            .await
            .context("Failed to insert/update feature override")?;

            imported_count += 1;
        }
    }

    info!(
        "Imported {} feature overrides from admin backend",
        imported_count
    );
    Ok(imported_count)
}
