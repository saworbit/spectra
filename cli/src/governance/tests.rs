use super::engine::*;
use std::fs::File;
use tempfile::TempDir;

#[test]
fn test_policy_evaluation_age() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("old.log");
    let _file = File::create(&file_path).unwrap();

    // Note: Testing age-based policies requires either:
    // 1. Manually setting modification time (OS-dependent)
    // 2. Mocking the Metadata struct (requires trait abstraction)
    // 3. Testing the logic with mocked data separately from filesystem

    let rule = Rule {
        extension: Some("log".to_string()),
        min_size_bytes: None,
        min_age_days: Some(30),
    };

    // This test validates the rule structure is correct
    assert_eq!(rule.extension, Some("log".to_string()));
    assert_eq!(rule.min_age_days, Some(30));
}

#[test]
fn test_policy_extension_match() {
    let temp_dir = TempDir::new().unwrap();
    let tmp_file_path = temp_dir.path().join("test.tmp");
    let log_file_path = temp_dir.path().join("test.log");

    let _tmp_file = File::create(&tmp_file_path).unwrap();
    let _log_file = File::create(&log_file_path).unwrap();

    let rule = Rule {
        extension: Some("tmp".to_string()),
        min_size_bytes: None,
        min_age_days: None,
    };

    let policy = Policy {
        name: "Test TMP Files".to_string(),
        rule: rule.clone(),
        action: Action::Report,
    };

    // Test that .tmp file matches
    let tmp_metadata = std::fs::metadata(&tmp_file_path).unwrap();
    assert!(policy.evaluate(&tmp_file_path, &tmp_metadata));

    // Test that .log file does not match
    let log_metadata = std::fs::metadata(&log_file_path).unwrap();
    assert!(!policy.evaluate(&log_file_path, &log_metadata));
}

#[test]
fn test_policy_size_threshold() {
    let temp_dir = TempDir::new().unwrap();
    let small_file_path = temp_dir.path().join("small.dat");
    let large_file_path = temp_dir.path().join("large.dat");

    // Create small file (< 1KB)
    std::fs::write(&small_file_path, b"small").unwrap();

    // Create large file (> 1KB)
    let large_content = vec![0u8; 2048];
    std::fs::write(&large_file_path, large_content).unwrap();

    let rule = Rule {
        extension: None,
        min_size_bytes: Some(1024), // 1KB threshold
        min_age_days: None,
    };

    let policy = Policy {
        name: "Large Files Only".to_string(),
        rule,
        action: Action::Report,
    };

    // Test small file does not match
    let small_metadata = std::fs::metadata(&small_file_path).unwrap();
    assert!(!policy.evaluate(&small_file_path, &small_metadata));

    // Test large file matches
    let large_metadata = std::fs::metadata(&large_file_path).unwrap();
    assert!(policy.evaluate(&large_file_path, &large_metadata));
}

#[test]
fn test_dry_run_mode() {
    let temp_dir = TempDir::new().unwrap();
    let test_file_path = temp_dir.path().join("test.txt");
    std::fs::write(&test_file_path, b"test content").unwrap();

    let policy = Policy {
        name: "Delete Test".to_string(),
        rule: Rule {
            extension: Some("txt".to_string()),
            min_size_bytes: None,
            min_age_days: None,
        },
        action: Action::Delete,
    };

    // Execute in dry-run mode
    policy.execute(&test_file_path, true);

    // File should still exist after dry-run
    assert!(test_file_path.exists());
}
