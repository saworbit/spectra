use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::models::{FileRecord, ExtensionStat};

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
