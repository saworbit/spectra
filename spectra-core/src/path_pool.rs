use std::collections::HashMap;

/// A path pool that interns common directory prefixes to reduce memory usage.
///
/// When scanning millions of files, paths share long common prefixes
/// (e.g., `C:\Users\User\Documents\Projects\...`). This pool stores each
/// unique directory prefix once and references it by a compact u32 ID.
pub struct PathPool {
    prefixes: HashMap<String, u32>,
    reverse: Vec<String>,
}

impl PathPool {
    pub fn new() -> Self {
        Self {
            prefixes: HashMap::new(),
            reverse: Vec::new(),
        }
    }

    /// Intern a full path, returning a CompactPath with shared prefix.
    pub fn intern(&mut self, full_path: &str) -> CompactPath {
        let (dir, file) = match full_path.rfind(|c: char| c == '/' || c == '\\') {
            Some(pos) => (&full_path[..pos], &full_path[pos + 1..]),
            None => ("", full_path),
        };

        let prefix_id = if let Some(&id) = self.prefixes.get(dir) {
            id
        } else {
            let id = self.reverse.len() as u32;
            self.prefixes.insert(dir.to_string(), id);
            self.reverse.push(dir.to_string());
            id
        };

        CompactPath {
            prefix_id,
            filename: file.to_string(),
        }
    }

    /// Resolve a CompactPath back to a full path string.
    pub fn resolve(&self, compact: &CompactPath) -> String {
        let prefix = &self.reverse[compact.prefix_id as usize];
        if prefix.is_empty() {
            compact.filename.clone()
        } else {
            #[cfg(windows)]
            {
                format!("{}\\{}", prefix, compact.filename)
            }
            #[cfg(not(windows))]
            {
                format!("{}/{}", prefix, compact.filename)
            }
        }
    }

    /// Number of unique directory prefixes stored.
    pub fn prefix_count(&self) -> usize {
        self.prefixes.len()
    }

    /// Estimated memory savings (bytes) compared to storing full paths.
    pub fn estimated_savings(&self, total_paths: usize) -> usize {
        if total_paths == 0 || self.prefixes.is_empty() {
            return 0;
        }
        let avg_prefix_len: usize = self.reverse.iter().map(|s| s.len()).sum::<usize>()
            / self.reverse.len().max(1);
        avg_prefix_len * total_paths.saturating_sub(self.prefixes.len())
    }
}

impl Default for PathPool {
    fn default() -> Self {
        Self::new()
    }
}

/// A compact representation of a file path using an interned directory prefix.
#[derive(Debug, Clone)]
pub struct CompactPath {
    pub prefix_id: u32,
    pub filename: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intern_and_resolve() {
        let mut pool = PathPool::new();

        let p1 = pool.intern("/home/user/docs/file1.txt");
        let p2 = pool.intern("/home/user/docs/file2.txt");
        let p3 = pool.intern("/home/user/photos/pic.jpg");

        // Same directory prefix should share the same ID
        assert_eq!(p1.prefix_id, p2.prefix_id);
        assert_ne!(p1.prefix_id, p3.prefix_id);

        assert_eq!(pool.prefix_count(), 2);

        #[cfg(not(windows))]
        {
            assert_eq!(pool.resolve(&p1), "/home/user/docs/file1.txt");
            assert_eq!(pool.resolve(&p2), "/home/user/docs/file2.txt");
            assert_eq!(pool.resolve(&p3), "/home/user/photos/pic.jpg");
        }
    }

    #[test]
    fn test_no_directory() {
        let mut pool = PathPool::new();
        let p = pool.intern("standalone.txt");
        assert_eq!(pool.resolve(&p), "standalone.txt");
    }

    #[test]
    fn test_savings_estimate() {
        let mut pool = PathPool::new();
        // Simulate 1000 files in 5 directories
        for i in 0..1000 {
            let dir = format!("/very/long/path/to/directory/{}", i % 5);
            let path = format!("{}/file_{}.txt", dir, i);
            pool.intern(&path);
        }
        assert_eq!(pool.prefix_count(), 5);
        assert!(pool.estimated_savings(1000) > 0);
    }
}
