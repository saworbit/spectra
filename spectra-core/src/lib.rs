// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2024-2025 Spectra Contributors
//
// This file is dual-licensed under the MIT and Apache 2.0 licenses.
// See LICENSE-MIT and LICENSE-APACHE in the repository root for full license texts.

use anyhow::Result;
use jwalk::WalkDir;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::path::{Path, PathBuf};
use std::time::Instant;

pub mod cache;
pub mod path_pool;
pub mod transport;

pub use cache::ScanCache;
pub use path_pool::PathPool;

// --- Device-Aware I/O (#6) ---

/// Device type for I/O thread tuning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceType {
    SSD,
    HDD,
    Unknown,
}

/// Detect whether the given path resides on an SSD or HDD.
///
/// On Windows, `canonicalize` produces extended-length paths that won't match
/// sysinfo mount points directly. This function normalizes:
/// - `\\?\C:\...` -> `c:\...`  (extended-length local path)
/// - `\\?\UNC\server\share\...` -> `\\server\share\...`  (extended-length UNC)
pub fn detect_device_type(path: &Path) -> DeviceType {
    use sysinfo::Disks;

    let disks = Disks::new_with_refreshed_list();
    let canonical = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    let mut path_str = canonical.to_string_lossy().to_lowercase();

    // Normalize Windows extended-length path prefixes:
    // \\?\UNC\server\share -> \\server\share  (must check before \\?\)
    // \\?\C:\...           -> c:\...
    if path_str.starts_with(r"\\?\unc\") {
        path_str = format!(r"\\{}", &path_str[8..]);
    } else if path_str.starts_with(r"\\?\") {
        path_str = path_str[4..].to_string();
    }

    let mut best_match: Option<(usize, DeviceType)> = None;

    for disk in &disks {
        let mount_str = disk.mount_point().to_string_lossy().to_lowercase();
        if path_str.starts_with(&*mount_str) {
            let len = mount_str.len();
            let device = match disk.kind() {
                sysinfo::DiskKind::SSD => DeviceType::SSD,
                sysinfo::DiskKind::HDD => DeviceType::HDD,
                _ => DeviceType::Unknown,
            };
            if best_match.is_none() || len > best_match.unwrap().0 {
                best_match = Some((len, device));
            }
        }
    }

    best_match.map(|(_, d)| d).unwrap_or(DeviceType::Unknown)
}

/// Recommended thread count based on device type.
/// SSDs benefit from full parallelism; HDDs are seek-limited.
/// Always capped at the available CPU count.
pub fn recommended_threads(device: DeviceType) -> usize {
    let cpus = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);

    match device {
        DeviceType::SSD => cpus,
        DeviceType::HDD => 2.min(cpus),
        DeviceType::Unknown => (cpus / 2).max(1).min(cpus),
    }
}

// --- Progress Streaming (#1) ---

/// Progress information emitted during scanning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    pub files_scanned: u64,
    pub folders_scanned: u64,
    pub bytes_scanned: u64,
}

// --- Data Models ---

/// Represents a file on disk, sortable by size for "Top N" calculations.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct FileRecord {
    pub path: String,
    pub size_bytes: u64,
}

// Reverse ordering for MinHeap (to keep largest items)
impl Ord for FileRecord {
    fn cmp(&self, other: &Self) -> Ordering {
        other.size_bytes.cmp(&self.size_bytes)
    }
}

impl PartialOrd for FileRecord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ExtensionStat {
    pub count: u64,
    pub size: u64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ScanStats {
    pub root_path: String,
    pub total_files: u64,
    pub total_folders: u64,
    pub total_size_bytes: u64,
    pub scan_duration_ms: u128,
    pub extensions: HashMap<String, ExtensionStat>,
    pub top_files: Vec<FileRecord>,
    /// Device type detected for the scanned path.
    #[serde(default)]
    pub device_type: Option<DeviceType>,
    /// Number of threads used for this scan.
    #[serde(default)]
    pub threads_used: Option<usize>,
}

// --- Scanner ---

pub struct Scanner {
    root: PathBuf,
    top_limit: usize,
    num_threads: usize,
    device: DeviceType,
    progress_callback: Option<Box<dyn Fn(ScanProgress) + Send>>,
}

impl Scanner {
    pub fn new(root: impl Into<PathBuf>, top_limit: usize) -> Self {
        let root = root.into();
        let device = detect_device_type(&root);
        let threads = recommended_threads(device);

        Self {
            root,
            top_limit,
            num_threads: threads,
            device,
            progress_callback: None,
        }
    }

    /// Override the auto-detected thread count.
    pub fn with_threads(mut self, threads: usize) -> Self {
        self.num_threads = threads.max(1);
        self
    }

    /// Set a progress callback for streaming scan updates.
    /// Called approximately every 1000 items processed.
    pub fn with_progress<F: Fn(ScanProgress) + Send + 'static>(mut self, callback: F) -> Self {
        self.progress_callback = Some(Box::new(callback));
        self
    }

    /// Executes the parallel scan and returns the aggregated statistics.
    /// Thread count is automatically tuned based on device type (SSD vs HDD).
    pub fn scan(&self) -> Result<ScanStats> {
        let start_time = Instant::now();

        let mut stats = ScanStats {
            root_path: self.root.display().to_string(),
            device_type: Some(self.device),
            threads_used: Some(self.num_threads),
            ..Default::default()
        };

        let mut top_files_heap = BinaryHeap::with_capacity(self.top_limit + 1);
        let mut item_counter = 0u64;

        let walker = WalkDir::new(&self.root)
            .parallelism(jwalk::Parallelism::RayonNewPool(self.num_threads));

        for dir_entry in walker.into_iter().flatten() {
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

                    // 2. TOP FILES ANALYTICS
                    top_files_heap.push(FileRecord {
                        path: dir_entry.path().display().to_string(),
                        size_bytes: size,
                    });

                    if top_files_heap.len() > self.top_limit {
                        top_files_heap.pop();
                    }
                } else if meta.is_dir() {
                    stats.total_folders += 1;
                }

                // Emit progress every 1000 items
                item_counter += 1;
                if item_counter.is_multiple_of(1000) {
                    if let Some(cb) = &self.progress_callback {
                        cb(ScanProgress {
                            files_scanned: stats.total_files,
                            folders_scanned: stats.total_folders,
                            bytes_scanned: stats.total_size_bytes,
                        });
                    }
                }
            }
        }

        stats.scan_duration_ms = start_time.elapsed().as_millis();

        // Finalize top files (sort descending)
        stats.top_files = top_files_heap.into_sorted_vec();
        stats.top_files.reverse();

        Ok(stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};
    use std::sync::Arc;
    use tempfile::tempdir;

    #[test]
    fn test_scanner_aggregates_correctly() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Hello World").unwrap();

        let scanner = Scanner::new(dir.path(), 5);
        let stats = scanner.scan().unwrap();

        assert_eq!(stats.total_files, 1);
        assert!(stats.total_size_bytes > 0);
        assert_eq!(stats.extensions.get("txt").unwrap().count, 1);
        assert_eq!(stats.top_files.len(), 1);
        assert!(stats.device_type.is_some());
        assert!(stats.threads_used.is_some());
    }

    #[test]
    fn test_device_detection() {
        let device = detect_device_type(Path::new("."));
        // Just verify it doesn't panic -- result varies by hardware
        println!("Detected device type: {:?}", device);
    }

    #[test]
    fn test_thread_recommendations() {
        assert!(recommended_threads(DeviceType::SSD) >= 1);
        assert!(recommended_threads(DeviceType::HDD) <= 2);
        assert!(recommended_threads(DeviceType::Unknown) >= 1);
    }

    #[test]
    fn test_progress_callback() {
        let dir = tempdir().unwrap();
        for i in 0..50 {
            let p = dir.path().join(format!("file_{}.txt", i));
            let mut f = File::create(p).unwrap();
            writeln!(f, "content {}", i).unwrap();
        }

        let progress_count = Arc::new(AtomicU64::new(0));
        let counter = progress_count.clone();

        let scanner = Scanner::new(dir.path(), 5).with_progress(move |_progress| {
            counter.fetch_add(1, AtomicOrdering::Relaxed);
        });

        let stats = scanner.scan().unwrap();
        assert_eq!(stats.total_files, 50);
    }
}
