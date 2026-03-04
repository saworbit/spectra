// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2024-2025 Spectra Contributors
//
// This file is dual-licensed under the MIT and Apache 2.0 licenses.
// See LICENSE-MIT and LICENSE-APACHE in the repository root for full license texts.

//! # Spectra Time-Travel & Velocity Engine
//!
//! Phase 3.5 Architecture: Persistent Time-Series Intelligence
//! This server stores filesystem snapshots and enables temporal analytics
//! to answer questions like:
//! - "How fast is the data growing?"
//! - "Who caused the spike last Tuesday?"
//! - "What did the filesystem look like at time T?"

use axum::{
    extract::{Path, Query, Request, State},
    http::{header, HeaderName, Method, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;
use tokio::net::TcpListener;
use tower_http::cors::{AllowOrigin, CorsLayer};

// --- Data Models ---

/// Snapshot captured by an agent at a specific point in time
#[derive(Serialize, Deserialize, Debug, Clone)]
struct AgentSnapshot {
    agent_id: String,
    timestamp: i64, // Unix Epoch (seconds)
    hostname: String,
    total_size_bytes: u64,
    file_count: u64,
    /// (Extension, Total Size, File Count) - Enhanced for granularity
    top_extensions: Vec<(String, u64, u64)>,
}

/// Velocity report showing data growth/shrinkage between two points in time
#[derive(Serialize, Deserialize, Debug)]
struct VelocityReport {
    agent_id: String,
    t_start: i64,
    t_end: i64,
    duration_seconds: i64,
    growth_bytes: i64, // Can be negative (shrinkage)
    growth_files: i64,
    bytes_per_second: f64, // The Velocity (delta_bytes / delta_time)
    extension_deltas: Vec<ExtensionDelta>,
}

/// Per-extension delta showing what contributed to the change
#[derive(Serialize, Deserialize, Debug)]
struct ExtensionDelta {
    extension: String,
    size_delta: i64,
    count_delta: i64,
}

/// Query parameters for time range selection
#[derive(Deserialize)]
struct TimeRange {
    start: i64,
    end: i64,
}

/// Query parameter for single timestamp
#[derive(Deserialize)]
struct TimestampQuery {
    #[serde(default)]
    timestamp: Option<i64>,
}

/// Time-bucketed aggregate for time-series visualization (#2)
#[derive(Serialize, Deserialize, Debug)]
struct TimeSeriesBucket {
    bucket_start: i64,
    bucket_end: i64,
    avg_size_bytes: f64,
    avg_file_count: f64,
    snapshot_count: u64,
}

/// Wrapper for aggregate response that signals truncation
#[derive(Serialize, Debug)]
struct AggregateResponse {
    buckets: Vec<TimeSeriesBucket>,
    /// True if the snapshot count hit the 10,000 cap and results may be incomplete.
    truncated: bool,
}

/// Query parameters for aggregation
#[derive(Deserialize)]
struct AggregateQuery {
    start: i64,
    end: i64,
    /// Bucket size in seconds (default: 3600 = 1 hour)
    #[serde(default = "default_bucket_size")]
    bucket_seconds: i64,
}

fn default_bucket_size() -> i64 {
    3600
}

/// Legacy policy structure (Phase 3.0 - kept for backward compatibility)
#[derive(Serialize, Deserialize, Debug)]
struct Policy {
    id: String,
    name: String,
    rules: Vec<String>,
    action: String,
}

// --- Database Logic ---

struct AppState {
    db: Surreal<surrealdb::engine::local::Db>,
}

// --- Middleware ---

/// API key authentication middleware.
async fn require_api_key(request: Request, next: Next) -> Result<Response, StatusCode> {
    let expected_key = std::env::var("SPECTRA_API_KEY").ok();

    let Some(expected) = expected_key else {
        return Ok(next.run(request).await);
    };

    let provided = request
        .headers()
        .get("x-api-key")
        .and_then(|v| v.to_str().ok());

    match provided {
        Some(key) if key == expected => Ok(next.run(request).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

// --- Handlers ---

/// POST /api/v1/ingest
///
/// Ingest a snapshot from an agent (The "Write" Path)
async fn ingest_snapshot(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AgentSnapshot>,
) -> Json<String> {
    let created: Result<Vec<AgentSnapshot>, _> =
        state.db.create("snapshots").content(&payload).await;

    match created {
        Ok(_) => {
            tracing::info!(
                "📡 Ingested Snapshot: {} @ {} ({}B, {} files)",
                payload.agent_id,
                payload.timestamp,
                payload.total_size_bytes,
                payload.file_count
            );
            Json("Snapshot stored".to_string())
        }
        Err(e) => {
            tracing::error!("Failed to store snapshot: {:?}", e);
            Json(format!("Error: {}", e))
        }
    }
}

/// GET /api/v1/history/:agent_id
///
/// Get available timestamps for an agent (For the Time Slider)
async fn get_agent_history(
    State(state): State<Arc<AppState>>,
    Path(agent_id): Path<String>,
) -> Json<Vec<i64>> {
    let query_result: Result<Vec<i64>, _> = state
        .db
        .query("SELECT VALUE timestamp FROM snapshots WHERE agent_id = $agent_id ORDER BY timestamp DESC")
        .bind(("agent_id", &agent_id))
        .await
        .and_then(|mut response| response.take(0));

    match query_result {
        Ok(timestamps) => {
            tracing::info!(
                "📅 Retrieved {} timestamps for agent {}",
                timestamps.len(),
                agent_id
            );
            Json(timestamps)
        }
        Err(e) => {
            tracing::error!("Failed to retrieve history: {:?}", e);
            Json(vec![])
        }
    }
}

/// GET /api/v1/velocity/:agent_id?start=<ts>&end=<ts>
///
/// Calculate Data Velocity between two points in time
async fn get_velocity(
    State(state): State<Arc<AppState>>,
    Path(agent_id): Path<String>,
    Query(range): Query<TimeRange>,
) -> Json<VelocityReport> {
    // Fetch the snapshot closest to the start time
    let start_snap_result: Result<Option<AgentSnapshot>, _> = state
        .db
        .query(
            "SELECT * FROM snapshots
             WHERE agent_id = $agent_id AND timestamp <= $ts
             ORDER BY timestamp DESC LIMIT 1",
        )
        .bind(("agent_id", &agent_id))
        .bind(("ts", range.start))
        .await
        .and_then(|mut response| response.take(0));

    // Fetch the snapshot closest to the end time
    let end_snap_result: Result<Option<AgentSnapshot>, _> = state
        .db
        .query(
            "SELECT * FROM snapshots
             WHERE agent_id = $agent_id AND timestamp <= $ts
             ORDER BY timestamp DESC LIMIT 1",
        )
        .bind(("agent_id", &agent_id))
        .bind(("ts", range.end))
        .await
        .and_then(|mut response| response.take(0));

    // Calculate velocity if both snapshots exist
    match (start_snap_result, end_snap_result) {
        (Ok(Some(start_snap)), Ok(Some(end_snap))) => {
            let size_diff =
                (end_snap.total_size_bytes as i64) - (start_snap.total_size_bytes as i64);
            let file_diff = (end_snap.file_count as i64) - (start_snap.file_count as i64);
            let duration = end_snap.timestamp - start_snap.timestamp;

            // Build a map of start extensions for O(1) lookup
            let mut start_ext_map: HashMap<String, (u64, u64)> = HashMap::new();
            for (ext, size, count) in &start_snap.top_extensions {
                start_ext_map.insert(ext.clone(), (*size, *count));
            }

            let mut extension_deltas = Vec::new();

            for (ext, end_size, end_count) in &end_snap.top_extensions {
                if let Some((start_size, start_count)) = start_ext_map.get(ext) {
                    extension_deltas.push(ExtensionDelta {
                        extension: ext.clone(),
                        size_delta: (*end_size as i64) - (*start_size as i64),
                        count_delta: (*end_count as i64) - (*start_count as i64),
                    });
                    start_ext_map.remove(ext);
                } else {
                    extension_deltas.push(ExtensionDelta {
                        extension: ext.clone(),
                        size_delta: *end_size as i64,
                        count_delta: *end_count as i64,
                    });
                }
            }

            for (ext, (start_size, start_count)) in start_ext_map {
                extension_deltas.push(ExtensionDelta {
                    extension: ext,
                    size_delta: -(start_size as i64),
                    count_delta: -(start_count as i64),
                });
            }

            extension_deltas.sort_by(|a, b| b.size_delta.abs().cmp(&a.size_delta.abs()));

            let velocity = if duration > 0 {
                size_diff as f64 / duration as f64
            } else {
                0.0
            };

            tracing::info!(
                "📈 Velocity calculated for {}: {:.2} bytes/sec ({} -> {})",
                agent_id,
                velocity,
                start_snap.timestamp,
                end_snap.timestamp
            );

            Json(VelocityReport {
                agent_id,
                t_start: start_snap.timestamp,
                t_end: end_snap.timestamp,
                duration_seconds: duration,
                growth_bytes: size_diff,
                growth_files: file_diff,
                bytes_per_second: velocity,
                extension_deltas,
            })
        }
        _ => {
            tracing::warn!(
                "⚠️  Insufficient data for velocity calculation: {} ({} to {})",
                agent_id,
                range.start,
                range.end
            );
            Json(VelocityReport {
                agent_id,
                t_start: 0,
                t_end: 0,
                duration_seconds: 0,
                growth_bytes: 0,
                growth_files: 0,
                bytes_per_second: 0.0,
                extension_deltas: vec![],
            })
        }
    }
}

/// GET /api/v1/snapshot/:agent_id?timestamp=<ts>  (#2 - Time-Travel)
///
/// Retrieve the full snapshot at or closest before a given timestamp.
/// If no timestamp is provided, returns the most recent snapshot.
async fn get_snapshot_at_time(
    State(state): State<Arc<AppState>>,
    Path(agent_id): Path<String>,
    Query(params): Query<TimestampQuery>,
) -> Json<Option<AgentSnapshot>> {
    let query = match params.timestamp {
        Some(ts) => {
            state
                .db
                .query(
                    "SELECT * FROM snapshots
                     WHERE agent_id = $agent_id AND timestamp <= $ts
                     ORDER BY timestamp DESC LIMIT 1",
                )
                .bind(("agent_id", &agent_id))
                .bind(("ts", ts))
                .await
        }
        None => {
            state
                .db
                .query(
                    "SELECT * FROM snapshots
                     WHERE agent_id = $agent_id
                     ORDER BY timestamp DESC LIMIT 1",
                )
                .bind(("agent_id", &agent_id))
                .await
        }
    };

    let result: Result<Option<AgentSnapshot>, _> = query.and_then(|mut response| response.take(0));

    match result {
        Ok(snap) => {
            if let Some(s) = &snap {
                tracing::info!(
                    "📸 Snapshot retrieved for {} @ {} ({}B, {} files)",
                    agent_id,
                    s.timestamp,
                    s.total_size_bytes,
                    s.file_count
                );
            }
            Json(snap)
        }
        Err(e) => {
            tracing::error!("Failed to retrieve snapshot: {:?}", e);
            Json(None)
        }
    }
}

/// GET /api/v1/aggregate/:agent_id?start=<ts>&end=<ts>&bucket_seconds=<n>  (#2 - Time-Travel)
///
/// Time-series aggregation with configurable bucket sizes.
/// Useful for summarizing long time periods without returning every snapshot.
/// Caps at 10,000 snapshots to bound memory; returns `truncated: true` if hit.
const AGGREGATE_SNAPSHOT_CAP: usize = 10_000;

async fn get_aggregate(
    State(state): State<Arc<AppState>>,
    Path(agent_id): Path<String>,
    Query(params): Query<AggregateQuery>,
) -> Json<AggregateResponse> {
    // Fetch snapshots in the time range (capped to bound memory).
    // LIMIT N+1 so we can detect whether the cap was hit.
    let limit = AGGREGATE_SNAPSHOT_CAP + 1;
    let result: Result<Vec<AgentSnapshot>, _> = state
        .db
        .query(
            "SELECT * FROM snapshots
             WHERE agent_id = $agent_id
               AND timestamp >= $start
               AND timestamp <= $end
             ORDER BY timestamp ASC
             LIMIT $limit",
        )
        .bind(("agent_id", &agent_id))
        .bind(("start", params.start))
        .bind(("end", params.end))
        .bind(("limit", limit as i64))
        .await
        .and_then(|mut response| response.take(0));

    match result {
        Ok(mut snapshots) => {
            let truncated = snapshots.len() > AGGREGATE_SNAPSHOT_CAP;
            if truncated {
                snapshots.truncate(AGGREGATE_SNAPSHOT_CAP);
                tracing::warn!(
                    "Aggregate for {} truncated at {} snapshots (range {} to {})",
                    agent_id,
                    AGGREGATE_SNAPSHOT_CAP,
                    params.start,
                    params.end
                );
            }

            let mut buckets: Vec<TimeSeriesBucket> = Vec::new();
            let bucket_size = params.bucket_seconds.max(60); // Minimum 1 minute

            let mut current_bucket_start = params.start;

            while current_bucket_start < params.end {
                let bucket_end = current_bucket_start + bucket_size;

                let in_bucket: Vec<&AgentSnapshot> = snapshots
                    .iter()
                    .filter(|s| s.timestamp >= current_bucket_start && s.timestamp < bucket_end)
                    .collect();

                if !in_bucket.is_empty() {
                    let count = in_bucket.len() as f64;
                    let avg_size = in_bucket
                        .iter()
                        .map(|s| s.total_size_bytes as f64)
                        .sum::<f64>()
                        / count;
                    let avg_files =
                        in_bucket.iter().map(|s| s.file_count as f64).sum::<f64>() / count;

                    buckets.push(TimeSeriesBucket {
                        bucket_start: current_bucket_start,
                        bucket_end,
                        avg_size_bytes: avg_size,
                        avg_file_count: avg_files,
                        snapshot_count: in_bucket.len() as u64,
                    });
                }

                current_bucket_start = bucket_end;
            }

            tracing::info!(
                "📊 Aggregated {} buckets for {} ({} to {}{})",
                buckets.len(),
                agent_id,
                params.start,
                params.end,
                if truncated { " [TRUNCATED]" } else { "" }
            );
            Json(AggregateResponse { buckets, truncated })
        }
        Err(e) => {
            tracing::error!("Failed to aggregate: {:?}", e);
            Json(AggregateResponse {
                buckets: vec![],
                truncated: false,
            })
        }
    }
}

/// GET /api/v1/policies
///
/// Legacy endpoint for Phase 3.0 governance
async fn get_policies(State(_state): State<Arc<AppState>>) -> Json<Vec<Policy>> {
    let global_policy = Policy {
        id: "pol_cleanup_logs".into(),
        name: "Cleanup Old Logs".into(),
        rules: vec![
            "extension == 'log'".into(),
            "days_since_modified > 90".into(),
        ],
        action: "DELETE".into(),
    };
    Json(vec![global_policy])
}

// --- Main ---

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let db = Surreal::new::<Mem>(()).await?;
    db.use_ns("spectra").use_db("telemetry").await?;

    // Create indexes for query performance.
    // Uses IF NOT EXISTS for idempotency with persistent backends.
    // Non-critical: log failures but don't abort startup.
    match db
        .query("DEFINE INDEX IF NOT EXISTS idx_snapshots_agent ON snapshots FIELDS agent_id")
        .await
    {
        Ok(_) => {}
        Err(e) => tracing::warn!("Index idx_snapshots_agent creation skipped: {}", e),
    }
    match db
        .query("DEFINE INDEX IF NOT EXISTS idx_snapshots_agent_time ON snapshots FIELDS agent_id, timestamp")
        .await
    {
        Ok(_) => {}
        Err(e) => tracing::warn!("Index idx_snapshots_agent_time creation skipped: {}", e),
    }

    tracing::info!("🗄️  Database initialized (in-memory mode) with indexes");

    let shared_state = Arc::new(AppState { db });

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list(
            std::env::var("SPECTRA_CORS_ORIGINS")
                .unwrap_or_else(|_| {
                    "http://localhost:1420,tauri://localhost,https://tauri.localhost".to_string()
                })
                .split(',')
                .filter_map(|s| s.trim().parse().ok()),
        ))
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE, HeaderName::from_static("x-api-key")]);

    if std::env::var("SPECTRA_API_KEY").is_ok() {
        tracing::info!("🔐 API key authentication enabled");
    } else {
        tracing::warn!(
            "⚠️  No SPECTRA_API_KEY set - running without authentication (development mode)"
        );
    }

    let app = Router::new()
        .route("/api/v1/ingest", post(ingest_snapshot))
        .route("/api/v1/history/:agent_id", get(get_agent_history))
        .route("/api/v1/velocity/:agent_id", get(get_velocity))
        .route("/api/v1/snapshot/:agent_id", get(get_snapshot_at_time))
        .route("/api/v1/aggregate/:agent_id", get(get_aggregate))
        .route("/api/v1/policies", get(get_policies))
        .layer(middleware::from_fn(require_api_key))
        .layer(cors)
        .with_state(shared_state);

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("🚀 Spectra Brain (Time-Travel Enabled) listening on port 3000");
    tracing::info!("📡 Endpoints:");
    tracing::info!("   POST   /api/v1/ingest");
    tracing::info!("   GET    /api/v1/history/:agent_id");
    tracing::info!("   GET    /api/v1/velocity/:agent_id?start=<ts>&end=<ts>");
    tracing::info!("   GET    /api/v1/snapshot/:agent_id?timestamp=<ts>");
    tracing::info!("   GET    /api/v1/aggregate/:agent_id?start=<ts>&end=<ts>&bucket_seconds=<n>");
    tracing::info!("   GET    /api/v1/policies");

    axum::serve(listener, app).await?;

    Ok(())
}
