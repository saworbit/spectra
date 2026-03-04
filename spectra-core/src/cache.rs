use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CacheEntry {
    mtime_secs: u64,
    size_bytes: u64,
    entropy: Option<f32>,
}

/// Persistent cache for scan results (entropy, hashes) keyed by file metadata.
/// Invalidates automatically when a file's mtime or size changes.
/// Stored as JSON in `~/.spectra/cache/`.
///
/// NOTE: mtime is stored at second-level precision. Rapid edits within the same
/// second (e.g., automated pipelines) could reuse stale values. This is an
/// acceptable tradeoff for the common interactive use case.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ScanCache {
    version: u32,
    entries: HashMap<String, CacheEntry>,
    #[serde(skip)]
    cache_path: PathBuf,
    #[serde(skip)]
    dirty: bool,
}

fn home_dir() -> PathBuf {
    std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}

impl ScanCache {
    /// Load an existing cache for the given scan root, or create a new one.
    pub fn load(scan_root: &Path) -> Self {
        let cache_path = Self::cache_file_for(scan_root);
        if let Ok(data) = fs::read_to_string(&cache_path) {
            if let Ok(mut cache) = serde_json::from_str::<ScanCache>(&data) {
                if cache.version == 1 {
                    cache.cache_path = cache_path;
                    return cache;
                }
            }
        }
        ScanCache {
            version: 1,
            entries: HashMap::new(),
            cache_path,
            dirty: false,
        }
    }

    fn cache_file_for(scan_root: &Path) -> PathBuf {
        let cache_dir = home_dir().join(".spectra").join("cache");
        let _ = fs::create_dir_all(&cache_dir);

        // Simple hash of the scan root path for the filename
        let path_str = scan_root.to_string_lossy();
        let hash = path_str
            .bytes()
            .fold(0u64, |h, b| h.wrapping_mul(31).wrapping_add(b as u64));

        cache_dir.join(format!("scan_{:016x}.json", hash))
    }

    /// Look up cached entropy for a file. Returns None if not cached or stale.
    pub fn get_entropy(&self, path: &Path, size: u64) -> Option<f32> {
        let key = path.to_string_lossy().to_string();
        let entry = self.entries.get(&key)?;
        if entry.size_bytes != size {
            return None;
        }
        let meta = fs::metadata(path).ok()?;
        let mtime = meta.modified().ok()?;
        let secs = mtime.duration_since(SystemTime::UNIX_EPOCH).ok()?.as_secs();
        if secs != entry.mtime_secs {
            return None;
        }
        entry.entropy
    }

    /// Store entropy for a file, keyed by its current metadata.
    pub fn put_entropy(&mut self, path: &Path, size: u64, entropy: f32) {
        let key = path.to_string_lossy().to_string();
        let mtime_secs = fs::metadata(path)
            .and_then(|m| m.modified())
            .ok()
            .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);

        self.entries.insert(
            key,
            CacheEntry {
                mtime_secs,
                size_bytes: size,
                entropy: Some(entropy),
            },
        );
        self.dirty = true;
    }

    /// Persist the cache to disk.
    pub fn save(&self) -> std::io::Result<()> {
        if !self.dirty {
            return Ok(());
        }
        if let Some(parent) = self.cache_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let data = serde_json::to_string(self)?;
        fs::write(&self.cache_path, data)
    }

    pub fn entries_count(&self) -> usize {
        self.entries.len()
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_cache_put_and_get() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        {
            let mut f = fs::File::create(&file_path).unwrap();
            f.write_all(b"hello world").unwrap();
        }

        let mut cache = ScanCache::load(dir.path());
        assert!(cache.get_entropy(&file_path, 11).is_none());

        cache.put_entropy(&file_path, 11, 3.5);
        assert_eq!(cache.get_entropy(&file_path, 11), Some(3.5));

        // Wrong size should miss
        assert!(cache.get_entropy(&file_path, 999).is_none());
    }

    #[test]
    fn test_cache_save_and_reload() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        {
            let mut f = fs::File::create(&file_path).unwrap();
            f.write_all(b"hello").unwrap();
        }

        let mut cache = ScanCache::load(dir.path());
        cache.put_entropy(&file_path, 5, 2.0);
        cache.save().unwrap();

        let cache2 = ScanCache::load(dir.path());
        assert_eq!(cache2.get_entropy(&file_path, 5), Some(2.0));
    }
}
