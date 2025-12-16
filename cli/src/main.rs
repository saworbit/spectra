use anyhow::Result;
use clap::Parser;
use humansize::{format_size, DECIMAL};
use jwalk::WalkDir;
use serde::Serialize;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::path::PathBuf;
use std::time::Instant;

mod analysis;
use analysis::{analyze_filename_risk, calculate_shannon_entropy, RiskLevel, SemanticEngine};

mod governance;
use governance::engine::{Action, Policy, Rule};

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
}

// A struct to hold file info, sortable by size (for our Heap)
#[derive(Debug, Serialize)]
struct FileRecord {
    path: String,
    size_bytes: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    entropy: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    risk_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    semantic_tag: Option<String>,
}

// Manual Eq and PartialEq implementations that only compare path and size
// (needed for BinaryHeap, but we ignore the analysis fields)
impl PartialEq for FileRecord {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path && self.size_bytes == other.size_bytes
    }
}

impl Eq for FileRecord {}

// Implement ordering so the BinaryHeap knows how to sort (by size)
impl Ord for FileRecord {
    fn cmp(&self, other: &Self) -> Ordering {
        // We reverse comparison because we want a MinHeap to keep the "Largest" items
        // (We pop the smallest of the top X to make room for bigger ones)
        other.size_bytes.cmp(&self.size_bytes)
    }
}

impl PartialOrd for FileRecord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Serialize, Debug, Default)]
struct ExtensionStat {
    count: u64,
    size: u64,
}

#[derive(Serialize, Debug, Default)]
struct ScanStats {
    root_path: String,
    total_files: u64,
    total_folders: u64,
    total_size_bytes: u64,
    scan_duration_ms: u128,
    // Analytics
    extensions: HashMap<String, ExtensionStat>,
    top_files: Vec<FileRecord>,
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
fn upload_snapshot(server_url: &str, stats: &ScanStats) {
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

    let start_time = Instant::now();

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

    // We use a BinaryHeap to efficiently track the Top N largest files without sorting the whole list
    let mut top_files_heap = BinaryHeap::with_capacity(args.limit + 1);
    let mut stats = ScanStats {
        root_path: root_path.display().to_string(),
        ..Default::default()
    };

    // Parallel Walk
    for dir_entry in WalkDir::new(&root_path).into_iter().flatten() {
        if let Ok(meta) = dir_entry.metadata() {
            if meta.is_file() {
                let size = meta.len();
                stats.total_files += 1;
                stats.total_size_bytes += size;

                // 1. EXTENSION ANALYTICS
                if let Some(ext) = dir_entry.path().extension() {
                    let ext_string = ext.to_string_lossy().to_string().to_lowercase();
                    let entry = stats.extensions.entry(ext_string).or_default();
                    entry.count += 1;
                    entry.size += size;
                }

                // 2. TOP FILES ANALYTICS (The "Heavy Hitters")
                top_files_heap.push(FileRecord {
                    path: dir_entry.path().display().to_string(),
                    size_bytes: size,
                    entropy: None,
                    risk_level: None,
                    semantic_tag: None,
                });

                // If heap is too big, remove the smallest of the large files
                if top_files_heap.len() > args.limit {
                    top_files_heap.pop();
                }

                // 3. GOVERNANCE CHECK (Phase 3)
                if !policies.is_empty() {
                    for policy in &policies {
                        if policy.evaluate(&dir_entry.path(), &meta) {
                            policy.execute(&dir_entry.path(), !args.enforce);
                        }
                    }
                }
            } else if meta.is_dir() {
                stats.total_folders += 1;
            }
        }
    }

    // Extract top files from heap and sort them largest first
    stats.top_files = top_files_heap.into_sorted_vec();
    // Note: into_sorted_vec returns ascending order, so we reverse for display
    stats.top_files.reverse();

    // POST-SCAN ANALYSIS: The Semantic Bridge (Phase 2)
    if args.analyze || args.semantic {
        if !args.json {
            println!(
                "🧠 Running Semantic Analysis on Top {} Files...",
                stats.top_files.len()
            );
        }

        // Initialize Semantic Engine (only if --semantic flag is used)
        let semantic_engine = if args.semantic {
            Some(SemanticEngine::new())
        } else {
            None
        };

        for file_record in &mut stats.top_files {
            let p = PathBuf::from(&file_record.path);

            // 1. Calculate Entropy (Tier 1)
            if let Ok(ent) = calculate_shannon_entropy(&p) {
                file_record.entropy = Some(ent);
            }

            // 2. Heuristic Risk Analysis (Tier 1)
            let risk = analyze_filename_risk(&p);
            if risk != RiskLevel::None {
                file_record.risk_level = Some(risk.as_str().to_string());
            }

            // 3. Semantic Tag (Tier 2 - only if enabled and file is likely text)
            if let Some(engine) = &semantic_engine {
                // Only classify files with low entropy (likely text)
                if file_record.entropy.unwrap_or(10.0) < 6.0 {
                    if let Some(tags) = engine.classify(&p) {
                        if tags.confidence > 0.5 {
                            file_record.semantic_tag = Some(tags.category);
                        }
                    }
                }
            }
        }
    }

    // Finalize Data
    let duration = start_time.elapsed();
    stats.scan_duration_ms = duration.as_millis();

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

    Ok(())
}

fn print_human_report(stats: &ScanStats) {
    println!("------------------------------------------------");
    println!(
        "✅ Scan Complete in {:.2}s",
        stats.scan_duration_ms as f64 / 1000.0
    );
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
