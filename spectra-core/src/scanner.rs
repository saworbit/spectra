use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::path::PathBuf;
use std::time::Instant;
use jwalk::WalkDir;
use anyhow::Result;
use crate::models::{FileRecord, ExtensionStat};
use crate::stats::ScanStats;

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

        for entry in WalkDir::new(&self.root) {
            if let Ok(dir_entry) = entry {
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
