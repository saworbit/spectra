// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2024-2025 Spectra Contributors
//
// This file is dual-licensed under the MIT and Apache 2.0 licenses.
// See LICENSE-MIT and LICENSE-APACHE in the repository root for full license texts.

use anyhow::Result;
use clap::Parser;
use humansize::{format_size, DECIMAL};
use jwalk::WalkDir;
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;

// Import core scanner
use spectra_core::{
    ExtensionStat, FileRecord as CoreFileRecord, ScanCache, ScanStats as CoreScanStats, Scanner,
};

mod analysis;
use analysis::{
    analyze_filename_risk, calculate_shannon_entropy, detect_outliers, RiskLevel, SemanticEngine,
};

mod governance;
use governance::engine::{Action, Policy, Rule};

mod watch;

/// S.P.E.C.T.R.A.
/// Scalable Platform for Enterprise Content Topology & Resource Analytics
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The root directory to scan
    #[arg(short, long, default_value = ".")]
    path: String,

    /// Output detailed JSON logs instead of human summary
    #[arg(short, long)]
    json: bool,

    /// Number of top largest files to track
    #[arg(short, long, default_value_t = 10)]
    limit: usize,

    /// Enable Phase 2 semantic analysis (entropy, risk scoring)
    #[arg(long)]
    analyze: bool,

    /// Enable AI-based content classification (requires 'semantic' feature)
    #[arg(long)]
    semantic: bool,

    /// URL of the Spectra Server for federation
    #[arg(long)]
    server: Option<String>,

    /// Enable Active Governance (Execute policies - defaults to dry-run)
    #[arg(long)]
    enforce: bool,

    /// Watch directory for real-time changes after scanning
    #[arg(long)]
    watch: bool,
}

// CLI-specific FileRecord WITH analysis fields
#[derive(Debug, Serialize)]
struct AnalyzedFileRecord {
    path: String,
    size_bytes: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    entropy: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    risk_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    semantic_tag: Option<String>,
    /// Whether this file is a statistical entropy outlier (IQR method)
    #[serde(skip_serializing_if = "Option::is_none")]
    entropy_outlier: Option<bool>,
}

// Conversion from core FileRecord to analyzed FileRecord
impl From<CoreFileRecord> for AnalyzedFileRecord {
    fn from(core: CoreFileRecord) -> Self {
        Self {
            path: core.path,
            size_bytes: core.size_bytes,
            entropy: None,
            risk_level: None,
            semantic_tag: None,
            entropy_outlier: None,
        }
    }
}

// CLI-specific stats structure WITH analyzed files
#[derive(Serialize, Debug, Default)]
struct CliScanStats {
    root_path: String,
    total_files: u64,
    total_folders: u64,
    total_size_bytes: u64,
    scan_duration_ms: u128,
    extensions: HashMap<String, ExtensionStat>,
    top_files: Vec<AnalyzedFileRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threads_used: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_hits: Option<usize>,
}

// Conversion from core ScanStats to CLI ScanStats
impl From<CoreScanStats> for CliScanStats {
    fn from(core: CoreScanStats) -> Self {
        Self {
            root_path: core.root_path,
            total_files: core.total_files,
            total_folders: core.total_folders,
            total_size_bytes: core.total_size_bytes,
            scan_duration_ms: core.scan_duration_ms,
            extensions: core.extensions,
            top_files: core.top_files.into_iter().map(Into::into).collect(),
            device_type: core.device_type.map(|d| format!("{:?}", d)),
            threads_used: core.threads_used,
            cache_hits: None,
        }
    }
}

// Helper: Fetch policies from server
fn fetch_policies(server_url: &str) -> Vec<Policy> {
    let url = format!("{}/api/v1/policies", server_url);
    match reqwest::blocking::get(&url) {
        Ok(response) => {
            if let Ok(policies) = response.json::<Vec<serde_json::Value>>() {
                // Parse server policies into our Policy format
                policies
                    .into_iter()
                    .filter_map(|p| {
                        Some(Policy {
                            name: p.get("name")?.as_str()?.to_string(),
                            rule: Rule {
                                extension: Some("log".to_string()), // Simplified parsing
                                min_size_bytes: None,
                                min_age_days: Some(90),
                            },
                            action: Action::Report, // Default to Report for safety
                        })
                    })
                    .collect()
            } else {
                println!("⚠️  Failed to parse policies from server");
                Vec::new()
            }
        }
        Err(e) => {
            println!("⚠️  Failed to fetch policies: {}", e);
            Vec::new()
        }
    }
}

// Helper: Upload snapshot to server
fn upload_snapshot(server_url: &str, stats: &CliScanStats) {
    let url = format!("{}/api/v1/ingest", server_url);
    let client = reqwest::blocking::Client::new();

    // Extract top extensions for the snapshot
    let mut sorted_exts: Vec<(&String, &ExtensionStat)> = stats.extensions.iter().collect();
    sorted_exts.sort_by(|a, b| b.1.size.cmp(&a.1.size));
    let top_extensions: Vec<(String, u64)> = sorted_exts
        .iter()
        .take(10)
        .map(|(ext, stat)| (ext.to_string(), stat.size))
        .collect();

    let snapshot = serde_json::json!({
        "agent_id": format!("agent_{}", chrono::Utc::now().timestamp()),
        "timestamp": chrono::Utc::now().timestamp(),
        "hostname": std::env::var("COMPUTERNAME").or_else(|_| std::env::var("HOSTNAME")).unwrap_or_else(|_| "unknown".to_string()),
        "total_size_bytes": stats.total_size_bytes,
        "file_count": stats.total_files,
        "top_extensions": top_extensions,
    });

    match client.post(&url).json(&snapshot).send() {
        Ok(response) => {
            if response.status().is_success() {
                println!("📤 Snapshot uploaded successfully to {}", server_url);
            } else {
                println!("⚠️  Server responded with status: {}", response.status());
            }
        }
        Err(e) => {
            println!("⚠️  Failed to upload snapshot: {}", e);
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let root_path = PathBuf::from(&args.path);

    if !args.json {
        println!(
            "🚀 SPECTRA: Profiling topology of '{}'...",
            root_path.display()
        );
    }

    // PHASE 3: Fetch Policies from Server (if connected)
    let mut policies = Vec::new();
    if let Some(server_url) = &args.server {
        if !args.json {
            println!("🌐 Fetching governance policies from {}...", server_url);
        }
        policies = fetch_policies(server_url);
        if !args.json && !policies.is_empty() {
            println!("📋 Loaded {} policies", policies.len());
            if !args.enforce {
                println!("⚠️  Running in DRY-RUN mode. Use --enforce to execute actions.");
            }
        }
    }

    // USE CORE SCANNER for basic scanning (Phase 1)
    // Device-aware I/O: thread count is auto-tuned based on SSD vs HDD
    let scanner = Scanner::new(root_path.clone(), args.limit);
    let core_stats = scanner.scan()?;

    // Convert to CLI stats structure with analysis fields
    let mut stats = CliScanStats::from(core_stats);

    // PHASE 3: Apply governance policies (if configured)
    if !policies.is_empty() {
        if !args.json {
            println!("⚙️  Evaluating {} governance policies...", policies.len());
        }

        for dir_entry in WalkDir::new(&root_path).into_iter().flatten() {
            if let Ok(meta) = dir_entry.metadata() {
                if meta.is_file() {
                    for policy in &policies {
                        if policy.evaluate(&dir_entry.path(), &meta) {
                            policy.execute(&dir_entry.path(), !args.enforce);
                        }
                    }
                }
            }
        }
    }

    // POST-SCAN ANALYSIS: The Semantic Bridge (Phase 2)
    if args.analyze || args.semantic {
        if !args.json {
            println!(
                "🧠 Running Semantic Analysis on Top {} Files...",
                stats.top_files.len()
            );
        }

        // Load entropy cache (#5 - Hash/entropy caching)
        let mut cache = ScanCache::load(&root_path);
        let mut cache_hits = 0usize;

        // Initialize Semantic Engine (only if --semantic flag is used)
        let semantic_engine = if args.semantic {
            Some(SemanticEngine::new())
        } else {
            None
        };

        for file_record in &mut stats.top_files {
            let p = PathBuf::from(&file_record.path);

            // 1. Calculate Entropy (with cache)
            if let Some(cached) = cache.get_entropy(&p, file_record.size_bytes) {
                file_record.entropy = Some(cached);
                cache_hits += 1;
            } else if let Ok(ent) = calculate_shannon_entropy(&p) {
                file_record.entropy = Some(ent);
                cache.put_entropy(&p, file_record.size_bytes, ent);
            }

            // 2. Heuristic Risk Analysis (Tier 1)
            let risk = analyze_filename_risk(&p);
            if risk != RiskLevel::None {
                file_record.risk_level = Some(risk.as_str().to_string());
            }

            // 3. Semantic Tag (Tier 2 - only if enabled and file is likely text)
            if let Some(engine) = &semantic_engine {
                if file_record.entropy.unwrap_or(10.0) < 6.0 {
                    if let Some(tags) = engine.classify(&p) {
                        if tags.confidence > 0.5 {
                            file_record.semantic_tag = Some(tags.category);
                        }
                    }
                }
            }
        }

        // 4. IQR-based entropy outlier detection (#4)
        let entropies: Vec<f32> = stats
            .top_files
            .iter()
            .filter_map(|f| f.entropy)
            .collect();

        if let Some(outlier_report) = detect_outliers(&entropies) {
            // Map outlier indices back to file records
            let mut entropy_idx = 0;
            for file_record in &mut stats.top_files {
                if file_record.entropy.is_some() {
                    if outlier_report.outlier_indices.contains(&entropy_idx) {
                        file_record.entropy_outlier = Some(true);
                    }
                    entropy_idx += 1;
                }
            }

            if !args.json {
                println!(
                    "📊 Entropy Stats: Q1={:.2} Median={:.2} Q3={:.2} IQR={:.2}",
                    outlier_report.q1,
                    outlier_report.median,
                    outlier_report.q3,
                    outlier_report.iqr
                );
                if !outlier_report.outlier_indices.is_empty() {
                    println!(
                        "⚠️  {} entropy outlier(s) detected (outside {:.2}-{:.2})",
                        outlier_report.outlier_indices.len(),
                        outlier_report.lower_fence,
                        outlier_report.upper_fence
                    );
                }
            }
        }

        // Save cache
        stats.cache_hits = Some(cache_hits);
        if let Err(e) = cache.save() {
            if !args.json {
                eprintln!("⚠️  Failed to save entropy cache: {}", e);
            }
        } else if !args.json && cache.entries_count() > 0 {
            println!(
                "💾 Cache: {} entries ({} hits this run)",
                cache.entries_count(),
                cache_hits
            );
        }
    }

    if args.json {
        println!("{}", serde_json::to_string_pretty(&stats)?);
    } else {
        print_human_report(&stats);
    }

    // PHASE 3: Upload Snapshot to Server (Time-Travel Analytics)
    if let Some(server_url) = &args.server {
        if !args.json {
            println!("📤 Uploading snapshot to {}...", server_url);
        }
        upload_snapshot(server_url, &stats);
    }

    // PHASE 5: Watch mode -- real-time filesystem monitoring (#8)
    if args.watch {
        println!(
            "\n👁️  Watching '{}' for changes (Ctrl+C to stop)...",
            root_path.display()
        );

        let watcher = watch::FileSystemWatcher::new(&root_path)
            .map_err(|e| anyhow::anyhow!("Failed to start watcher: {}", e))?;

        loop {
            let events = watcher.poll(std::time::Duration::from_secs(1));
            for event in events {
                for path in &event.paths {
                    println!("  {} {}", event.kind, path);
                }
            }
        }
    }

    Ok(())
}

fn print_human_report(stats: &CliScanStats) {
    println!("------------------------------------------------");
    println!(
        "✅ Scan Complete in {:.2}s",
        stats.scan_duration_ms as f64 / 1000.0
    );
    if let Some(device) = &stats.device_type {
        if let Some(threads) = stats.threads_used {
            println!("⚡ Device: {} | Threads: {}", device, threads);
        }
    }
    println!("------------------------------------------------");
    println!("📂 Location : {}", stats.root_path);
    println!("📄 Files    : {}", stats.total_files);
    println!(
        "💾 Total Size: {}",
        format_size(stats.total_size_bytes, DECIMAL)
    );
    println!("------------------------------------------------");

    println!("📊 Top Extensions by Volume:");
    // Quick sort to find top 5 extensions by size
    let mut sorted_exts: Vec<(&String, &ExtensionStat)> = stats.extensions.iter().collect();
    sorted_exts.sort_by(|a, b| b.1.size.cmp(&a.1.size));

    for (ext, data) in sorted_exts.iter().take(5) {
        println!(
            "   .{:<5} : {:>10} ({})",
            ext,
            format_size(data.size, DECIMAL),
            data.count
        );
    }

    println!("\n🐳 Top Largest Files:");
    for file in &stats.top_files {
        let mut info_parts = vec![format_size(file.size_bytes, DECIMAL)];

        // Add entropy if available
        if let Some(ent) = file.entropy {
            info_parts.push(format!("Entropy:{:.1}", ent));
        }

        // Add outlier flag
        if file.entropy_outlier == Some(true) {
            info_parts.push("⚠️OUTLIER".to_string());
        }

        // Add risk level if available
        if let Some(risk) = &file.risk_level {
            let risk_icon = match risk.as_str() {
                "Critical" => "🔴",
                "High" => "🟠",
                "Medium" => "🟡",
                "Low" => "🟢",
                _ => "⚪",
            };
            info_parts.push(format!("{} {}", risk_icon, risk));
        }

        // Add semantic tag if available
        if let Some(tag) = &file.semantic_tag {
            info_parts.push(format!("[{}]", tag));
        }

        println!("   {:<50}  {}", info_parts.join(" | "), file.path);
    }
    println!("------------------------------------------------");
}
