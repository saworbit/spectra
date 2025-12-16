use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub enum Action {
    Report,
    Delete,
    Archive { target_path: String },
}

#[derive(Debug, Deserialize, Clone)]
pub struct Rule {
    pub extension: Option<String>,
    pub min_size_bytes: Option<u64>,
    pub min_age_days: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct Policy {
    pub name: String,
    pub rule: Rule,
    pub action: Action,
}

impl Policy {
    pub fn evaluate(&self, path: &Path, metadata: &std::fs::Metadata) -> bool {
        // 1. Check Extension
        if let Some(target_ext) = &self.rule.extension {
            if let Some(ext) = path.extension() {
                if ext.to_string_lossy().to_lowercase() != *target_ext {
                    return false;
                }
            } else {
                return false;
            }
        }

        // 2. Check Size
        if let Some(min_size) = self.rule.min_size_bytes {
            if metadata.len() < min_size {
                return false;
            }
        }

        // 3. Check Age
        if let Some(days) = self.rule.min_age_days {
            if let Ok(modified) = metadata.modified() {
                if let Ok(elapsed) = modified.elapsed() {
                    if elapsed.as_secs() < days * 86400 {
                        return false; // Too young
                    }
                }
            }
        }

        true // All conditions met
    }

    pub fn execute(&self, path: &Path, dry_run: bool) {
        if dry_run {
            println!("[DRY RUN] Would execute {:?} on {:?}", self.action, path);
            return;
        }

        match &self.action {
            Action::Report => println!("🚩 Violation: {:?} matches '{}'", path, self.name),
            Action::Delete => {
                // SAFETY: Double check before deletion in production code!
                match std::fs::remove_file(path) {
                    Ok(_) => println!("🗑️ Deleted: {:?}", path),
                    Err(e) => eprintln!("❌ Failed to delete {:?}: {}", path, e),
                }
            }
            Action::Archive { target_path } => {
                println!("📦 Archiving {:?} to {}", path, target_path);
                // Implementation: Move file to target_path
            }
        }
    }
}
