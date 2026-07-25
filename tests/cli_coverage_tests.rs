//! Additional CLI integration tests to improve coverage for main.rs and `config_manager.rs`
//! Focuses on edge cases, error handling, and command combinations not covered elsewhere

use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

fn batless_command() -> Command {
    Command::new(env!("CARGO_BIN_EXE_batless"))
}

fn run_batless_args(args: &[&str]) -> std::process::Output {
    batless_command()
        .args(args)
        .output()
        .expect("Failed to execute batless")
}

#[test]
fn test_version_json_command() {
    let output = run_batless_args(&["--version-json"]);
    assert!(output.status.success(), "Command should succeed");

    let stdout_str = String::from_utf8(output.stdout).expect("Valid UTF-8 output");
    assert!(stdout_str.contains("\"name\""), "Should contain name field");
    assert!(
        stdout_str.contains("\"version\""),
        "Should contain version field"
    );
    assert!(
        stdout_str.contains("\"git_hash\""),
        "Should contain git_hash field"
    );
    assert!(
        stdout_str.contains("\"build_timestamp\""),
        "Should contain build_timestamp field"
    );
    assert!(
        stdout_str.contains("\"authors\""),
        "Should contain authors field"
    );

    // Verify it's valid JSON
    let parsed: serde_json::Value =
        serde_json::from_str(&stdout_str).expect("Output should be valid JSON");
    assert!(parsed.is_object(), "Should be a JSON object");
}

#[test]
fn test_cli_with_max_lines_zero() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    writeln!(temp_file, "line1\nline2\nline3").expect("Failed to write to temp file");
    let temp_path = temp_file.path().to_str().expect("Invalid temp path");

    let output = run_batless_args(&["--max-lines", "0", temp_path]);
    assert!(
        !output.status.success(),
        "Command should fail with max-lines=0"
    );

    let stderr_str = String::from_utf8(output.stderr).expect("Valid UTF-8 output");
    assert!(
        stderr_str.contains("validation"),
        "Should show validation error"
    );
}

#[test]
fn test_cli_with_max_bytes_zero() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    writeln!(temp_file, "content").expect("Failed to write to temp file");
    let temp_path = temp_file.path().to_str().expect("Invalid temp path");

    let output = run_batless_args(&["--max-bytes", "0", temp_path]);
    assert!(
        !output.status.success(),
        "Command should fail with max-bytes=0"
    );

    let stderr_str = String::from_utf8(output.stderr).expect("Valid UTF-8 output");
    assert!(
        stderr_str.contains("validation"),
        "Should show validation error"
    );
}

#[test]
fn test_cli_with_invalid_language() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    writeln!(temp_file, "content").expect("Failed to write to temp file");
    let temp_path = temp_file.path().to_str().expect("Invalid temp path");

    let output = run_batless_args(&["--language", "NonexistentLang", temp_path]);
    assert!(
        !output.status.success(),
        "Command should fail with invalid language"
    );

    let stderr_str = String::from_utf8(output.stderr).expect("Valid UTF-8 output");
    assert!(
        stderr_str.contains("Language not found") || stderr_str.contains("language"),
        "Should show language-related error"
    );
}

#[test]
fn test_error_message_formatting() {
    // Test that error messages are properly formatted
    let output = run_batless_args(&["nonexistent_file.txt"]);
    assert!(!output.status.success(), "Command should fail");

    let stderr_str = String::from_utf8(output.stderr).expect("Valid UTF-8 output");
    assert!(
        stderr_str.contains("Error"),
        "Should contain 'Error' prefix"
    );
    assert!(
        stderr_str.len() > 10,
        "Should contain meaningful error message"
    );
}

#[test]
fn test_completion_generation_all_shells() {
    // Test bash completion
    let output = run_batless_args(&["--generate-completions", "bash"]);
    assert!(
        output.status.success(),
        "Bash completion generation should succeed"
    );
    let stdout = String::from_utf8(output.stdout).expect("Valid UTF-8");
    assert!(!stdout.is_empty(), "Should generate bash completions");

    // Test zsh completion
    let output = run_batless_args(&["--generate-completions", "zsh"]);
    assert!(
        output.status.success(),
        "Zsh completion generation should succeed"
    );
    let stdout = String::from_utf8(output.stdout).expect("Valid UTF-8");
    assert!(!stdout.is_empty(), "Should generate zsh completions");

    // Test fish completion
    let output = run_batless_args(&["--generate-completions", "fish"]);
    assert!(
        output.status.success(),
        "Fish completion generation should succeed"
    );
    let stdout = String::from_utf8(output.stdout).expect("Valid UTF-8");
    assert!(!stdout.is_empty(), "Should generate fish completions");

    // Test PowerShell completion
    let output = run_batless_args(&["--generate-completions", "power-shell"]);
    assert!(
        output.status.success(),
        "PowerShell completion generation should succeed"
    );
    let stdout = String::from_utf8(output.stdout).expect("Valid UTF-8");
    assert!(!stdout.is_empty(), "Should generate PowerShell completions");
}
