//! Additional streaming functionality tests to improve coverage for streaming.rs
//! Focuses on StreamingCheckpoint functionality and edge cases

use batless::{BatlessConfig, StreamingCheckpoint};
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_streaming_checkpoint_creation() {
    let config = BatlessConfig::default();

    let checkpoint = StreamingCheckpoint::new("test_file.txt".to_string(), 10, 250, 1, &config);

    assert_eq!(checkpoint.line_number, 10);
    assert_eq!(checkpoint.bytes_processed, 250);
    assert_eq!(checkpoint.chunk_number, 1);
    assert_eq!(checkpoint.file_path, "test_file.txt");
    assert!(!checkpoint.timestamp.is_empty(), "Timestamp should be set");
    assert!(
        !checkpoint.config_hash.is_empty(),
        "Config hash should be set"
    );
    assert_eq!(checkpoint.schema_version, config.schema_version);
}

#[test]
fn test_streaming_checkpoint_compatibility() {
    let config = BatlessConfig::default()
        .with_streaming_json(true)
        .with_streaming_chunk_size(10);

    // Create a checkpoint
    let checkpoint = StreamingCheckpoint::new("test_file.txt".to_string(), 15, 500, 2, &config);

    // Test compatibility with same config
    assert!(
        checkpoint.is_compatible(&config),
        "Checkpoint should be compatible with same config"
    );

    // Test incompatibility with different config parameters
    let different_config = config.clone().with_max_lines(5000);
    assert!(
        !checkpoint.is_compatible(&different_config),
        "Checkpoint should be incompatible with different max_lines"
    );

    let different_config2 = config.clone().with_include_tokens(true);
    assert!(
        !checkpoint.is_compatible(&different_config2),
        "Checkpoint should be incompatible with different token setting"
    );

    let different_config3 = config.with_language(Some("Python".to_string()));
    assert!(
        !checkpoint.is_compatible(&different_config3),
        "Checkpoint should be incompatible with different language"
    );
}

#[test]
fn test_streaming_checkpoint_save_load() {
    let temp_dir = TempDir::new().unwrap();
    let checkpoint_path = temp_dir.path().join("checkpoint.json");

    let config = BatlessConfig::default();
    let original_checkpoint =
        StreamingCheckpoint::new("test_file.txt".to_string(), 25, 1000, 3, &config);

    // Save checkpoint
    batless::StreamingProcessor::save_checkpoint(&original_checkpoint, &checkpoint_path)
        .expect("Should save checkpoint successfully");

    // Verify file was created
    assert!(
        checkpoint_path.exists(),
        "Checkpoint file should be created"
    );

    // Load checkpoint
    let loaded_checkpoint = batless::StreamingProcessor::load_checkpoint(&checkpoint_path)
        .expect("Should load checkpoint successfully");

    // Verify all fields match
    assert_eq!(loaded_checkpoint.file_path, original_checkpoint.file_path);
    assert_eq!(
        loaded_checkpoint.line_number,
        original_checkpoint.line_number
    );
    assert_eq!(
        loaded_checkpoint.bytes_processed,
        original_checkpoint.bytes_processed
    );
    assert_eq!(
        loaded_checkpoint.chunk_number,
        original_checkpoint.chunk_number
    );
    assert_eq!(
        loaded_checkpoint.config_hash,
        original_checkpoint.config_hash
    );
    assert_eq!(
        loaded_checkpoint.schema_version,
        original_checkpoint.schema_version
    );
    assert_eq!(loaded_checkpoint.timestamp, original_checkpoint.timestamp);
}

#[test]
fn test_streaming_checkpoint_load_nonexistent() {
    let nonexistent_path = Path::new("/nonexistent/checkpoint.json");

    let result = batless::StreamingProcessor::load_checkpoint(nonexistent_path);
    assert!(
        result.is_err(),
        "Loading nonexistent checkpoint should fail"
    );
}

#[test]
fn test_streaming_checkpoint_load_invalid_json() {
    let temp_dir = TempDir::new().unwrap();
    let checkpoint_path = temp_dir.path().join("invalid.json");

    // Write invalid JSON
    std::fs::write(&checkpoint_path, "invalid json content").expect("Should write invalid JSON");

    let result = batless::StreamingProcessor::load_checkpoint(&checkpoint_path);
    assert!(result.is_err(), "Loading invalid JSON should fail");
}

#[test]
fn test_streaming_processor_with_stdin() {
    let config = BatlessConfig::default().with_streaming_json(true);

    // Test that stdin doesn't support checkpoints
    let checkpoint = StreamingCheckpoint::new("-".to_string(), 0, 0, 0, &config);

    let result = batless::StreamingProcessor::process_streaming("-", &config, Some(checkpoint));
    assert!(result.is_err(), "Stdin with checkpoint should fail");

    if let Err(error) = result {
        assert!(
            error.to_string().contains("Resume/checkpoint") || error.to_string().contains("stdin"),
            "Error should mention checkpoint/stdin incompatibility: {}",
            error
        );
    }
}

#[test]
fn test_streaming_processor_checkpoint_file_mismatch() {
    let config = BatlessConfig::default().with_streaming_json(true);

    // Create checkpoint for different file
    let checkpoint =
        StreamingCheckpoint::new("different_file.txt".to_string(), 10, 500, 1, &config);

    // Try to use with different file path
    let result = batless::StreamingProcessor::process_streaming(
        "current_file.txt",
        &config,
        Some(checkpoint),
    );
    assert!(result.is_err(), "Mismatched file path should fail");

    if let Err(error) = result {
        assert!(
            error.to_string().contains("file path") || error.to_string().contains("doesn't match"),
            "Error should mention file path mismatch: {}",
            error
        );
    }
}

#[test]
fn test_streaming_processor_incompatible_checkpoint() {
    let config = BatlessConfig::default()
        .with_streaming_json(true)
        .with_max_lines(1000);

    // Create checkpoint with different config
    let different_config = config.clone().with_max_lines(2000);
    let checkpoint =
        StreamingCheckpoint::new("test_file.txt".to_string(), 10, 500, 1, &different_config);

    // Try to use incompatible checkpoint
    let result =
        batless::StreamingProcessor::process_streaming("test_file.txt", &config, Some(checkpoint));
    assert!(result.is_err(), "Incompatible checkpoint should fail");

    if let Err(error) = result {
        assert!(
            error.to_string().contains("incompatible")
                || error.to_string().contains("configuration"),
            "Error should mention incompatibility: {}",
            error
        );
    }
}

#[test]
fn test_streaming_get_schema() {
    let schema = batless::StreamingProcessor::get_streaming_schema();

    assert!(schema.is_object(), "Schema should be a JSON object");
    assert!(
        schema["$schema"].is_string(),
        "Schema should have $schema field"
    );
    assert!(
        schema["title"].is_string(),
        "Schema should have title field"
    );
    assert!(
        schema["description"].is_string(),
        "Schema should have description field"
    );
    assert_eq!(schema["type"], "object", "Schema type should be object");
}

#[test]
fn test_streaming_checkpoint_config_hash_changes() {
    let config1 = BatlessConfig::default().with_max_lines(1000);

    let config2 = BatlessConfig::default().with_max_lines(2000);

    let checkpoint1 = StreamingCheckpoint::new("test.txt".to_string(), 0, 0, 0, &config1);
    let checkpoint2 = StreamingCheckpoint::new("test.txt".to_string(), 0, 0, 0, &config2);

    // Config hashes should be different
    assert_ne!(
        checkpoint1.config_hash, checkpoint2.config_hash,
        "Different configs should produce different hashes"
    );
}

#[test]
fn test_streaming_checkpoint_serialization() {
    let config = BatlessConfig::default();
    let checkpoint = StreamingCheckpoint::new("test_file.txt".to_string(), 42, 1024, 5, &config);

    // Test serialization to JSON
    let json_str = serde_json::to_string(&checkpoint).expect("Should serialize to JSON");
    assert!(
        json_str.contains("test_file.txt"),
        "JSON should contain file path"
    );
    assert!(json_str.contains("42"), "JSON should contain line number");
    assert!(
        json_str.contains("1024"),
        "JSON should contain bytes processed"
    );
    assert!(json_str.contains("5"), "JSON should contain chunk number");

    // Test deserialization from JSON
    let deserialized: StreamingCheckpoint =
        serde_json::from_str(&json_str).expect("Should deserialize from JSON");

    assert_eq!(deserialized.file_path, checkpoint.file_path);
    assert_eq!(deserialized.line_number, checkpoint.line_number);
    assert_eq!(deserialized.bytes_processed, checkpoint.bytes_processed);
    assert_eq!(deserialized.chunk_number, checkpoint.chunk_number);
}
