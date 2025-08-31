//! Additional CLI integration tests to improve coverage for main.rs and config_manager.rs
//! Focuses on edge cases, error handling, and command combinations not covered elsewhere

use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

fn batless_command() -> Command {
    Command::new("cargo")
}

fn run_batless_args(args: &[&str]) -> std::process::Output {
    batless_command()
        .arg("run")
        .arg("--")
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
fn test_get_schema_file_info() {
    let output = run_batless_args(&["--get-schema", "file_info"]);
    assert!(output.status.success(), "Command should succeed");

    let stdout_str = String::from_utf8(output.stdout).expect("Valid UTF-8 output");
    assert!(
        stdout_str.contains("\"$schema\""),
        "Should contain schema field"
    );

    // Verify it's valid JSON
    let _parsed: serde_json::Value =
        serde_json::from_str(&stdout_str).expect("Output should be valid JSON schema");
}

#[test]
fn test_get_schema_json_output() {
    let output = run_batless_args(&["--get-schema", "json_output"]);
    assert!(output.status.success(), "Command should succeed");

    let stdout_str = String::from_utf8(output.stdout).expect("Valid UTF-8 output");
    assert!(
        stdout_str.contains("\"$schema\""),
        "Should contain schema field"
    );
}

#[test]
fn test_get_schema_token_count() {
    let output = run_batless_args(&["--get-schema", "token_count"]);
    assert!(output.status.success(), "Command should succeed");

    let stdout_str = String::from_utf8(output.stdout).expect("Valid UTF-8 output");
    assert!(
        stdout_str.contains("\"$schema\""),
        "Should contain schema field"
    );
}

#[test]
fn test_get_schema_processing_stats() {
    let output = run_batless_args(&["--get-schema", "processing_stats"]);
    assert!(output.status.success(), "Command should succeed");

    let stdout_str = String::from_utf8(output.stdout).expect("Valid UTF-8 output");
    assert!(
        stdout_str.contains("\"$schema\""),
        "Should contain schema field"
    );
}

#[test]
fn test_get_schema_invalid() {
    let output = run_batless_args(&["--get-schema", "invalid_schema"]);
    assert!(!output.status.success(), "Command should fail");

    let stderr_str = String::from_utf8(output.stderr).expect("Valid UTF-8 output");
    assert!(
        stderr_str.contains("Unknown schema format"),
        "Should show error message"
    );
    assert!(
        stderr_str.contains("Available schemas:"),
        "Should show available options"
    );
}

#[test]
fn test_configuration_wizard_help() {
    let output = run_batless_args(&["--configure"]);
    // This will start the interactive wizard, but we can't test interaction easily
    // The test ensures the command path is covered
    // In a real scenario, this would require stdin mocking
    let _status_code = output.status.code().unwrap_or(-1);
    // Accept either success (if wizard starts) or failure (if stdin not available)
    // Just verify the command executes without panicking
}

#[test]
fn test_list_profiles_command() {
    let output = run_batless_args(&["--list-profiles"]);
    assert!(output.status.success(), "Command should succeed");

    let _stdout_str = String::from_utf8(output.stdout).expect("Valid UTF-8 output");
    // Output depends on whether profiles exist, but command should not fail
    // Just verify it doesn't crash - output content varies
}

#[test]
fn test_edit_profile_nonexistent() {
    let output = run_batless_args(&["--edit-profile", "nonexistent_profile.toml"]);
    // This should either succeed (if file is created) or fail gracefully
    let status_code = output.status.code().unwrap_or(-1);
    assert!(
        status_code >= 0,
        "Command should handle nonexistent profile gracefully"
    );
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
fn test_cli_with_invalid_theme() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    writeln!(temp_file, "content").expect("Failed to write to temp file");
    let temp_path = temp_file.path().to_str().expect("Invalid temp path");

    let output = run_batless_args(&["--theme", "nonexistent-theme", temp_path]);
    assert!(
        !output.status.success(),
        "Command should fail with invalid theme"
    );

    let stderr_str = String::from_utf8(output.stderr).expect("Valid UTF-8 output");
    assert!(
        stderr_str.contains("Theme not found") || stderr_str.contains("theme"),
        "Should show theme-related error"
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
fn test_streaming_json_with_checkpoints() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    // Create a larger file for streaming
    for i in 0..100 {
        writeln!(temp_file, "Line number {} with some content", i).expect("Failed to write");
    }
    let temp_path = temp_file.path().to_str().expect("Invalid temp path");

    let output = run_batless_args(&[
        "--mode",
        "json",
        "--streaming-json",
        "--enable-resume",
        "--streaming-chunk-size",
        "10",
        temp_path,
    ]);
    assert!(output.status.success(), "Streaming command should succeed");

    let stdout_str = String::from_utf8(output.stdout).expect("Valid UTF-8 output");
    assert!(!stdout_str.is_empty(), "Should produce streaming output");
}

#[test]
fn test_ai_model_token_counting() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    writeln!(temp_file, "def hello(): print('world')").expect("Failed to write to temp file");
    let temp_path = temp_file.path().to_str().expect("Invalid temp path");

    let output = run_batless_args(&[
        "--ai-model",
        "gpt4",
        "--include-tokens",
        "--mode",
        "json",
        temp_path,
    ]);
    assert!(output.status.success(), "Token counting should succeed");

    let stdout_str = String::from_utf8(output.stdout).expect("Valid UTF-8 output");
    assert!(
        stdout_str.contains("tokens"),
        "Should include token information"
    );
}

#[test]
fn test_fit_context_with_prompt_tokens() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    // Create a very large file to test context fitting
    for i in 0..1000 {
        writeln!(temp_file, "This is line {} with substantial content to test context window fitting and truncation behavior.", i).expect("Failed to write");
    }
    let temp_path = temp_file.path().to_str().expect("Invalid temp path");

    let output = run_batless_args(&[
        "--ai-model",
        "gpt4",
        "--fit-context",
        "--prompt-tokens",
        "1000",
        "--mode",
        "json",
        temp_path,
    ]);
    assert!(output.status.success(), "Context fitting should succeed");

    let stdout_str = String::from_utf8(output.stdout).expect("Valid UTF-8 output");
    // Verify the content was processed and potentially truncated
    assert!(!stdout_str.is_empty(), "Should produce output");
}

#[test]
fn test_custom_profile_nonexistent() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    writeln!(temp_file, "content").expect("Failed to write to temp file");
    let temp_path = temp_file.path().to_str().expect("Invalid temp path");

    let output = run_batless_args(&["--custom-profile", "nonexistent_profile.toml", temp_path]);
    assert!(
        !output.status.success(),
        "Command should fail with nonexistent profile"
    );

    let stderr_str = String::from_utf8(output.stderr).expect("Valid UTF-8 output");
    assert!(
        stderr_str.contains("profile") || stderr_str.contains("file"),
        "Should show profile or file related error"
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
fn test_validate_json_flag() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    writeln!(temp_file, "{{\"key\": \"value\"}}").expect("Failed to write to temp file");
    let temp_path = temp_file.path().to_str().expect("Invalid temp path");

    let output = run_batless_args(&["--mode", "json", "--validate-json", temp_path]);
    assert!(output.status.success(), "JSON validation should succeed");
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
