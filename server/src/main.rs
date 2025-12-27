// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2024-2025 Spectra Contributors
//
// This file is dual-licensed under the MIT and Apache 2.0 licenses.
// See LICENSE-MIT and LICENSE-APACHE in the repository root for full license texts.

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::TcpListener;

// --- Data Models ---

#[derive(Serialize, Deserialize, Debug)]
struct AgentSnapshot {
    agent_id: String,
    timestamp: i64,
    hostname: String,
    total_size_bytes: u64,
    file_count: u64,
    top_extensions: Vec<(String, u64)>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Policy {
    id: String,
    name: String,
    rules: Vec<String>,
    action: String,
}

// --- State Management ---

struct AppState {
    // In a real app, this is a SurrealDB client
    // db: Surreal<Client>,
}

// --- Handlers ---

async fn ingest_snapshot(
    State(_state): State<Arc<AppState>>,
    Json(payload): Json<AgentSnapshot>,
) -> Json<String> {
    // 1. Save Snapshot to DB (Time-Series)
    println!("📡 Received Telemetry from Agent: {}", payload.agent_id);
    println!("   📊 Size: {} bytes", payload.total_size_bytes);

    // 2. Logic to compare with previous snapshot for "Growth Alert" would go here

    Json("Ack".to_string())
}

async fn get_policies(State(_state): State<Arc<AppState>>) -> Json<Vec<Policy>> {
    // Distribute global governance rules
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
async fn main() {
    let shared_state = Arc::new(AppState {});

    let app = Router::new()
        .route("/api/v1/ingest", post(ingest_snapshot))
        .route("/api/v1/policies", get(get_policies))
        .with_state(shared_state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Spectra Server listening on port 3000");
    axum::serve(listener, app).await.unwrap();
}
