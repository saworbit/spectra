use regex::RegexSet;
use std::path::Path;
use std::sync::OnceLock;

fn sensitive_patterns() -> &'static RegexSet {
    static PATTERNS: OnceLock<RegexSet> = OnceLock::new();
    PATTERNS.get_or_init(|| {
        RegexSet::new([
            r"(?i)password",
            r"(?i)secret",
            r"(?i)key",
            r"(?i)token",
            r"(?i)\.pem$",
            r"(?i)\.kdbx$", // KeePass
            r"(?i)backup",
            r"(?i)dump",
            r"(?i)\.p12$", // Certificate files
            r"(?i)\.pfx$", // Certificate files
            r"(?i)credentials",
            r"(?i)\.env$", // Environment files
            r"(?i)config", // Configuration files (may contain secrets)
            r"(?i)\.ssh",  // SSH keys
            r"(?i)wallet", // Cryptocurrency wallets
        ])
        .expect("failed to compile sensitive pattern regexes")
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskLevel {
    None,
    Low,
    Medium,
    High,
    Critical,
}

impl RiskLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            RiskLevel::None => "None",
            RiskLevel::Low => "Low",
            RiskLevel::Medium => "Medium",
            RiskLevel::High => "High",
            RiskLevel::Critical => "Critical",
        }
    }
}

pub fn analyze_filename_risk(path: &Path) -> RiskLevel {
    let filename = match path.file_name() {
        Some(n) => n.to_string_lossy(),
        None => return RiskLevel::None,
    };

    // Also check the full path for directory-based patterns
    let path_str = path.to_string_lossy().to_lowercase();
    let filename_lower = filename.to_lowercase();

    // Check if either filename or full path matches sensitive patterns
    if !sensitive_patterns().is_match(&filename) && !sensitive_patterns().is_match(&path_str) {
        return RiskLevel::None;
    }

    // Critical: Private keys, certificates, password files
    if filename_lower.ends_with(".pem")
        || filename_lower.ends_with(".p12")
        || filename_lower.ends_with(".pfx")
        || filename_lower.contains("password")
        || filename_lower.contains("secret")
        || path_str.contains(".ssh")  // Check full path for .ssh directory
        || filename_lower.contains("wallet")
    {
        return RiskLevel::Critical;
    }

    // High: Credentials, tokens, KeePass databases
    if filename_lower.contains("credential")
        || filename_lower.contains("token")
        || filename_lower.ends_with(".kdbx")
        || filename_lower == ".env"
    {
        return RiskLevel::High;
    }

    // Medium: Backups, dumps, config files (may contain sensitive data)
    if filename_lower.contains("backup")
        || filename_lower.contains("dump")
        || filename_lower.contains("config")
        || filename_lower.contains("key")
    {
        return RiskLevel::Medium;
    }

    RiskLevel::Low
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_critical_risk_files() {
        assert_eq!(
            analyze_filename_risk(&PathBuf::from("private.pem")),
            RiskLevel::Critical
        );
        assert_eq!(
            analyze_filename_risk(&PathBuf::from("passwords.txt")),
            RiskLevel::Critical
        );
        assert_eq!(
            analyze_filename_risk(&PathBuf::from(".ssh/id_rsa")),
            RiskLevel::Critical
        );
        assert_eq!(
            analyze_filename_risk(&PathBuf::from("my_secret_key.pem")),
            RiskLevel::Critical
        );
    }

    #[test]
    fn test_high_risk_files() {
        assert_eq!(
            analyze_filename_risk(&PathBuf::from("credentials.json")),
            RiskLevel::High
        );
        assert_eq!(
            analyze_filename_risk(&PathBuf::from(".env")),
            RiskLevel::High
        );
        assert_eq!(
            analyze_filename_risk(&PathBuf::from("database.kdbx")),
            RiskLevel::High
        );
    }

    #[test]
    fn test_medium_risk_files() {
        assert_eq!(
            analyze_filename_risk(&PathBuf::from("backup.zip")),
            RiskLevel::Medium
        );
        assert_eq!(
            analyze_filename_risk(&PathBuf::from("config.yaml")),
            RiskLevel::Medium
        );
    }

    #[test]
    fn test_safe_files() {
        assert_eq!(
            analyze_filename_risk(&PathBuf::from("document.pdf")),
            RiskLevel::None
        );
        assert_eq!(
            analyze_filename_risk(&PathBuf::from("main.rs")),
            RiskLevel::None
        );
    }
}
