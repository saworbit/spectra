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
