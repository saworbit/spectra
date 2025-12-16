use anyhow::Result;
use jwalk::WalkDir;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::path::PathBuf;
use std::time::Instant;

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
}

pub struct Scanner {
    root: PathBuf,
    top_limit: usize,
}

impl Scanner {
    pub fn new(root: impl Into<PathBuf>, top_limit: usize) -> Self {
        Self {
            root: root.into(),
            top_limit,
        }
    }

    /// Executes the parallel scan and returns the aggregated statistics.
    pub fn scan(&self) -> Result<ScanStats> {
        let start_time = Instant::now();

        let mut stats = ScanStats {
            root_path: self.root.display().to_string(),
            ..Default::default()
        };

        // Heap to track top N files efficiently
        let mut top_files_heap = BinaryHeap::with_capacity(self.top_limit + 1);

        for dir_entry in WalkDir::new(&self.root).into_iter().flatten() {
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
    use tempfile::tempdir;

    #[test]
    fn test_scanner_aggregates_correctly() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Hello World").unwrap(); // ~12 bytes

        let scanner = Scanner::new(dir.path(), 5);
        let stats = scanner.scan().unwrap();

        assert_eq!(stats.total_files, 1);
        assert!(stats.total_size_bytes > 0);
        assert_eq!(stats.extensions.get("txt").unwrap().count, 1);
        assert_eq!(stats.top_files.len(), 1);
    }
}
