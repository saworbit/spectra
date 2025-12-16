use std::cmp::Ordering;
use serde::{Deserialize, Serialize};

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
