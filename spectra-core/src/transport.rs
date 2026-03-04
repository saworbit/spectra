use crate::ScanStats;
use serde::{Deserialize, Serialize};

/// A command that can be sent through any transport layer (IPC, HTTP, direct).
///
/// This provides a unified interface for CLI, Tauri, and HTTP clients
/// to invoke the same operations without duplicating logic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpectraCommand {
    /// Scan a directory and return statistics.
    Scan { path: String, limit: usize },
    /// Get available snapshot timestamps for an agent.
    GetHistory { agent_id: String },
    /// Calculate velocity between two timestamps.
    GetVelocity {
        agent_id: String,
        start: i64,
        end: i64,
    },
    /// Get the snapshot closest to a specific timestamp.
    GetSnapshot { agent_id: String, timestamp: i64 },
}

/// A response returned from any transport layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpectraResponse {
    ScanResult(ScanStats),
    History(Vec<i64>),
    Velocity(VelocityData),
    Snapshot(Option<SnapshotData>),
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VelocityData {
    pub agent_id: String,
    pub t_start: i64,
    pub t_end: i64,
    pub duration_seconds: i64,
    pub growth_bytes: i64,
    pub growth_files: i64,
    pub bytes_per_second: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotData {
    pub agent_id: String,
    pub timestamp: i64,
    pub total_size_bytes: u64,
    pub file_count: u64,
}

/// Trait for executing Spectra commands over any transport.
///
/// Implementations:
/// - `DirectExecutor`: In-process for CLI usage (scan only)
/// - HTTP transport: Via reqwest in the CLI crate
/// - Tauri IPC: Via Tauri commands in the app crate
pub trait Transport {
    fn execute(&self, cmd: SpectraCommand) -> Result<SpectraResponse, String>;
}

/// Direct in-process executor -- handles scan commands without network.
pub struct DirectExecutor;

impl Transport for DirectExecutor {
    fn execute(&self, cmd: SpectraCommand) -> Result<SpectraResponse, String> {
        match cmd {
            SpectraCommand::Scan { path, limit } => {
                let scanner = crate::Scanner::new(&path, limit);
                let stats = scanner.scan().map_err(|e| e.to_string())?;
                Ok(SpectraResponse::ScanResult(stats))
            }
            _ => Err("Command requires server connection".to_string()),
        }
    }
}

/// HTTP transport configuration -- actual HTTP calls are in the CLI crate
/// to avoid pulling reqwest into spectra-core.
pub struct HttpTransport {
    pub base_url: String,
}

impl HttpTransport {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }
}
