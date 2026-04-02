//! Additional streaming functionality tests to improve coverage for streaming.rs
//! Focuses on StreamingCheckpoint functionality and edge cases

use batless::{BatlessConfig, StreamingCheckpoint};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use tempfile::{NamedTempFile, TempDir};

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

// =========================================================================
// ME-4: --chunk-strategy CLI integration tests
// =========================================================================

fn run_batless_cli(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_batless"))
        .args(args)
        .output()
        .expect("Failed to execute batless")
}

fn create_streaming_test_file(content: &str, extension: &str) -> NamedTempFile {
    let mut file = tempfile::Builder::new()
        .suffix(extension)
        .tempfile()
        .unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file
}

#[test]
fn test_chunk_strategy_line_default() {
    // A Rust file with 3 functions totalling > 30 lines
    let content = "pub fn alpha() -> i32 {\n    let x = 1;\n    let y = 2;\n    let z = 3;\n    let w = 4;\n    let v = 5;\n    x + y + z + w + v\n}\n\npub fn beta() -> i32 {\n    let a = 10;\n    let b = 20;\n    let c = 30;\n    let d = 40;\n    let e = 50;\n    a + b + c + d + e\n}\n\npub fn gamma() -> i32 {\n    let m = 100;\n    let n = 200;\n    let o = 300;\n    let p = 400;\n    let q = 500;\n    m + n + o + p + q\n}\n\npub fn delta() {\n    // intentionally empty\n}\n";
    let file = create_streaming_test_file(content, ".rs");
    let path = file.path().to_str().unwrap();

    let output = run_batless_cli(&[
        "--streaming-json",
        "--mode=json",
        "--streaming-chunk-size=10",
        path,
    ]);

    assert!(
        output.status.success(),
        "batless --streaming-json --streaming-chunk-size=10 should succeed"
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(!stdout.is_empty(), "Output should not be empty");

    // Parse NDJSON: each non-empty line is a JSON chunk
    let chunks: Vec<serde_json::Value> = stdout
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).expect("Each NDJSON line should be valid JSON"))
        .collect();

    assert!(
        chunks.len() > 1,
        "File with >30 lines and chunk-size=10 should produce multiple chunks, got {}",
        chunks.len()
    );

    // Collect all lines across chunks and verify the full file is covered
    let total_lines_in_chunks: usize = chunks
        .iter()
        .map(|c| c["lines"].as_array().map(|a| a.len()).unwrap_or(0))
        .sum();

    let file_total_lines = content.lines().count();
    assert_eq!(
        total_lines_in_chunks, file_total_lines,
        "All file lines should appear across chunks: expected {}, got {}",
        file_total_lines, total_lines_in_chunks
    );
}

#[test]
fn test_chunk_strategy_semantic() {
    // Same style Rust file — use semantic chunking
    let content = "pub fn alpha() -> i32 {\n    let x = 1;\n    let y = 2;\n    let z = 3;\n    x + y + z\n}\n\npub fn beta() -> i32 {\n    let a = 10;\n    let b = 20;\n    a + b\n}\n\npub fn gamma() -> i32 {\n    let m = 100;\n    let n = 200;\n    m + n\n}\n\npub fn delta() {\n    // empty\n}\n\npub fn epsilon() -> bool {\n    true\n}\n";
    let file = create_streaming_test_file(content, ".rs");
    let path = file.path().to_str().unwrap();

    let output = run_batless_cli(&[
        "--streaming-json",
        "--mode=json",
        "--streaming-chunk-size=5",
        "--chunk-strategy=semantic",
        path,
    ]);

    assert!(
        output.status.success(),
        "batless --chunk-strategy=semantic should succeed"
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(!stdout.is_empty(), "Output should not be empty");

    let chunks: Vec<serde_json::Value> = stdout
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).expect("Each NDJSON line should be valid JSON"))
        .collect();

    assert!(!chunks.is_empty(), "Should produce at least one chunk");

    // With semantic chunking a chunk may extend beyond the nominal chunk size
    // to avoid splitting in the middle of a declaration
    let any_extended = chunks
        .iter()
        .any(|c| c["lines"].as_array().map(|a| a.len() > 5).unwrap_or(false));
    assert!(
        any_extended,
        "At least one semantic chunk should extend beyond the nominal chunk size of 5"
    );

    // All file lines should still be covered
    let total_lines_in_chunks: usize = chunks
        .iter()
        .map(|c| c["lines"].as_array().map(|a| a.len()).unwrap_or(0))
        .sum();

    let file_total_lines = content.lines().count();
    assert_eq!(
        total_lines_in_chunks, file_total_lines,
        "All file lines should appear across semantic chunks: expected {}, got {}",
        file_total_lines, total_lines_in_chunks
    );
}

#[test]
fn test_chunk_strategy_unsupported_language_falls_back() {
    // Plain text — no AST support, should fall back to line-based chunking
    let lines: Vec<String> = (1..=20).map(|i| format!("plain text line {}", i)).collect();
    let content = lines.join("\n") + "\n";
    let file = create_streaming_test_file(&content, ".txt");
    let path = file.path().to_str().unwrap();

    let output = run_batless_cli(&[
        "--streaming-json",
        "--mode=json",
        "--streaming-chunk-size=5",
        "--chunk-strategy=semantic",
        path,
    ]);

    assert!(
        output.status.success(),
        "batless --chunk-strategy=semantic on a .txt file should succeed (fallback to line-based)"
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        !stdout.is_empty(),
        "Output should not be empty even when falling back"
    );

    // Must parse as NDJSON without errors
    let chunks: Vec<serde_json::Value> = stdout
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).expect("Each NDJSON line should be valid JSON"))
        .collect();

    assert!(!chunks.is_empty(), "Should produce at least one chunk");

    // All lines should be covered
    let total_lines_in_chunks: usize = chunks
        .iter()
        .map(|c| c["lines"].as_array().map(|a| a.len()).unwrap_or(0))
        .sum();

    let file_total_lines = lines.len();
    assert_eq!(
        total_lines_in_chunks, file_total_lines,
        "All text lines should appear across fallback chunks: expected {}, got {}",
        file_total_lines, total_lines_in_chunks
    );
}
