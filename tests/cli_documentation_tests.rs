//! Integration tests that verify documented CLAUDE.md examples work correctly
//! These tests prevent documentation drift by testing actual documented commands

use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

fn create_test_file(content: &str, extension: &str) -> NamedTempFile {
    let mut file = tempfile::Builder::new()
        .suffix(extension)
        .tempfile()
        .unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file
}

fn run_batless(args: &[&str]) -> std::process::Output {
    Command::new("cargo")
        .arg("run")
        .arg("--")
        .args(args)
        .output()
        .expect("Failed to execute batless")
}

#[test]
fn test_basic_file_viewing() {
    let file = create_test_file("fn main() {}\n", ".rs");
    let output = run_batless(&[file.path().to_str().unwrap()]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("main"));
}

#[test]
fn test_plain_mode() {
    let file = create_test_file("test content\n", ".txt");

    // Test --mode=plain
    let output = run_batless(&["--mode=plain", file.path().to_str().unwrap()]);
    assert!(output.status.success());

    // Test --plain shorthand
    let output = run_batless(&["--plain", file.path().to_str().unwrap()]);
    assert!(output.status.success());
}

#[test]
fn test_json_mode() {
    let file = create_test_file("test\n", ".txt");
    let output = run_batless(&["--mode=json", file.path().to_str().unwrap()]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("valid json");

    assert!(json["file"].is_string());
    assert!(json["lines"].is_array());
}

#[test]
fn test_summary_mode() {
    let file = create_test_file("fn main() {}\nstruct User {}\n", ".rs");
    let output = run_batless(&["--mode=summary", file.path().to_str().unwrap()]);

    assert!(output.status.success());
}

#[test]
fn test_max_lines_limit() {
    let file = create_test_file("line1\nline2\nline3\nline4\nline5\n", ".txt");
    let output = run_batless(&["--max-lines=3", file.path().to_str().unwrap()]);

    assert!(output.status.success());
}

#[test]
fn test_max_bytes_limit() {
    let file = create_test_file("test content here\n", ".txt");
    // Need to also set max-lines to avoid conflict
    let output = run_batless(&[
        "--max-bytes=1024",
        "--max-lines=10",
        file.path().to_str().unwrap(),
    ]);

    assert!(output.status.success());
}

#[test]
fn test_combined_limits() {
    let file = create_test_file("line1\nline2\nline3\n", ".txt");
    let output = run_batless(&[
        "--max-lines=100",
        "--max-bytes=5000",
        file.path().to_str().unwrap(),
    ]);

    assert!(output.status.success());
}

#[test]
fn test_ai_profiles() {
    let file = create_test_file("test\n", ".py");

    // Test Claude profile
    let output = run_batless(&["--profile=claude", file.path().to_str().unwrap()]);
    assert!(output.status.success());

    // Test Copilot profile
    let output = run_batless(&["--profile=copilot", file.path().to_str().unwrap()]);
    assert!(output.status.success());

    // Test ChatGPT profile
    let output = run_batless(&["--profile=chatgpt", file.path().to_str().unwrap()]);
    assert!(output.status.success());
}

#[test]
fn test_json_with_tokens() {
    let file = create_test_file("test content\n", ".py");
    let output = run_batless(&[
        "--mode=json",
        "--include-tokens",
        file.path().to_str().unwrap(),
    ]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("valid json");

    assert!(json["tokens"].is_array());
    assert!(json["token_count"].is_number());
}

#[test]
fn test_json_with_summary() {
    let file = create_test_file("fn main() {}\n", ".rs");
    let output = run_batless(&["--mode=json", "--summary", file.path().to_str().unwrap()]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("valid json");

    assert!(json["summary_lines"].is_array());
}

#[test]
fn test_language_override() {
    let file = create_test_file("test\n", ".unknown");
    let output = run_batless(&["--language=python", file.path().to_str().unwrap()]);

    assert!(output.status.success());
}

#[test]
fn test_theme_selection() {
    let file = create_test_file("test\n", ".py");
    let output = run_batless(&["--theme=base16-ocean.dark", file.path().to_str().unwrap()]);

    assert!(output.status.success());
}

#[test]
fn test_list_languages() {
    let output = run_batless(&["--list-languages"]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Rust") || stdout.contains("Python"));
}

#[test]
fn test_list_themes() {
    let output = run_batless(&["--list-themes"]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(!stdout.is_empty());
}

#[test]
fn test_line_numbers_with_plain() {
    let file = create_test_file("line1\nline2\nline3\n", ".txt");

    // Test -n with --plain
    let output = run_batless(&["-n", "--plain", file.path().to_str().unwrap()]);
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("1\t") || stdout.contains("     1\t"));

    // Test --number with --mode=plain
    let output = run_batless(&["--number", "--mode=plain", file.path().to_str().unwrap()]);
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("1\t") || stdout.contains("     1\t"));
}

#[test]
fn test_number_nonblank_with_plain() {
    let file = create_test_file("line1\n\nline3\n", ".txt");

    // Test -b with --plain
    let output = run_batless(&["-b", "--plain", file.path().to_str().unwrap()]);
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should have line numbers for non-blank lines
    assert!(stdout.contains("1\t") || stdout.contains("     1\t"));
}

#[test]
fn test_line_numbers_without_plain_no_numbers() {
    let file = create_test_file("line1\nline2\n", ".txt");

    // Line numbers without --plain should not show line numbers
    let output = run_batless(&["-n", file.path().to_str().unwrap()]);
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should NOT contain numbered format
    // (with highlighting, no line numbers are shown)
    assert!(!stdout.contains("     1\tline1"));
}

#[test]
fn test_pattern_flag_rejected() {
    let file = create_test_file("test\n", ".txt");
    let output = run_batless(&["--pattern", "TODO", file.path().to_str().unwrap()]);

    // Should exit with error (exit code 1)
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("pattern searching") || stderr.contains("search tools"));
}

#[test]
fn test_list_flag_rejected() {
    let output = run_batless(&["--list"]);

    // Should exit with error (exit code 1)
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("list files") || stderr.contains("file listing"));
}

#[test]
fn test_range_flag_rejected() {
    let file = create_test_file("test\n", ".txt");
    let output = run_batless(&["-r", "10:50", file.path().to_str().unwrap()]);

    // Should error on unknown flag
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("unexpected argument") || stderr.contains("error"));
}

#[test]
fn test_unbuffered_flag_ignored() {
    let file = create_test_file("test\n", ".txt");
    let output = run_batless(&["--plain", "--unbuffered", file.path().to_str().unwrap()]);

    // Should succeed (flag ignored for compatibility)
    assert!(output.status.success());
}

#[test]
fn test_version_flag() {
    let output = run_batless(&["--version"]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("batless"));
}

#[test]
fn test_version_json_flag() {
    let output = run_batless(&["--version-json"]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("valid json");

    assert!(json["version"].is_string());
}
