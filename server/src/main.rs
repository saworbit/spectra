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

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

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
    bytes_per_second: f64, // The Velocity (Δ/Δt)
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

// --- Handlers ---

/// POST /api/v1/ingest
///
/// Ingest a snapshot from an agent (The "Write" Path)
///
/// This endpoint receives telemetry snapshots from Spectra Agents
/// and persists them to the time-series database for historical analysis.
async fn ingest_snapshot(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AgentSnapshot>,
) -> Json<String> {
    // Store the snapshot in the time-series database
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
///
/// Returns a list of Unix timestamps when snapshots are available,
/// allowing the GUI to render interactive timeline markers.
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
///
/// This is the core "Time-Travel Analytics" endpoint that computes:
/// - Total data growth/shrinkage (Δ bytes)
/// - File count change (Δ files)
/// - Velocity (bytes per second)
/// - Per-extension contribution breakdown
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

            // Calculate per-extension deltas
            let mut extension_deltas = Vec::new();

            // Process extensions in end snapshot
            for (ext, end_size, end_count) in &end_snap.top_extensions {
                if let Some((start_size, start_count)) = start_ext_map.get(ext) {
                    extension_deltas.push(ExtensionDelta {
                        extension: ext.clone(),
                        size_delta: (*end_size as i64) - (*start_size as i64),
                        count_delta: (*end_count as i64) - (*start_count as i64),
                    });
                    start_ext_map.remove(ext); // Mark as processed
                } else {
                    // New extension appeared
                    extension_deltas.push(ExtensionDelta {
                        extension: ext.clone(),
                        size_delta: *end_size as i64,
                        count_delta: *end_count as i64,
                    });
                }
            }

            // Process extensions that disappeared
            for (ext, (start_size, start_count)) in start_ext_map {
                extension_deltas.push(ExtensionDelta {
                    extension: ext,
                    size_delta: -(start_size as i64),
                    count_delta: -(start_count as i64),
                });
            }

            // Sort by absolute size impact (most significant first)
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
            // Fallback for missing data
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

/// GET /api/v1/policies
///
/// Legacy endpoint for Phase 3.0 governance (kept for backward compatibility)
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
    // Initialize tracing (structured logging)
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // Initialize In-Memory Database
    // For production: use Surreal::new::<RocksDb>("path/to/data.db")
    let db = Surreal::new::<Mem>(()).await?;
    db.use_ns("spectra").use_db("telemetry").await?;

    tracing::info!("🗄️  Database initialized (in-memory mode)");

    let shared_state = Arc::new(AppState { db });

    // Build the router with CORS enabled for React frontend
    let app = Router::new()
        .route("/api/v1/ingest", post(ingest_snapshot))
        .route("/api/v1/history/:agent_id", get(get_agent_history))
        .route("/api/v1/velocity/:agent_id", get(get_velocity))
        .route("/api/v1/policies", get(get_policies))
        .layer(CorsLayer::permissive()) // Allow GUI to connect from localhost
        .with_state(shared_state);

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("🚀 Spectra Brain (Time-Travel Enabled) listening on port 3000");
    tracing::info!("📡 Endpoints:");
    tracing::info!("   POST   /api/v1/ingest");
    tracing::info!("   GET    /api/v1/history/:agent_id");
    tracing::info!("   GET    /api/v1/velocity/:agent_id?start=<ts>&end=<ts>");
    tracing::info!("   GET    /api/v1/policies");

    axum::serve(listener, app).await?;

    Ok(())
}
