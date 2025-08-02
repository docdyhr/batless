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
fn test_help_command() {
    let output = run_batless(&["--help"]);
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("A non-blocking, LLM-friendly code viewer"));
    assert!(stdout.contains("--mode <MODE>"));
    assert!(stdout.contains("--max-lines"));
}

#[test]
fn test_version_command() {
    let output = run_batless(&["--version"]);
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("batless"));
}

#[test]
fn test_plain_mode() {
    let content = "fn main() {\n    println!(\"Hello, world!\");\n}\n";
    let file = create_test_file(content, ".rs");

    let output = run_batless(&[file.path().to_str().unwrap(), "--mode=plain"]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("fn main()"));
    assert!(stdout.contains("println!"));
}

#[test]
fn test_highlight_mode() {
    let content = "fn main() {\n    println!(\"Hello, world!\");\n}\n";
    let file = create_test_file(content, ".rs");

    let output = run_batless(&[
        file.path().to_str().unwrap(),
        "--mode=highlight",
        "--color=always",
    ]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Should contain the content, might have ANSI codes but that's ok
    assert!(stdout.contains("main"));
    assert!(stdout.contains("println"));
}

#[test]
fn test_json_mode() {
    let content = "fn main() {\n    println!(\"Hello, world!\");\n}\n";
    let file = create_test_file(content, ".rs");

    let output = run_batless(&[file.path().to_str().unwrap(), "--mode=json"]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Parse as JSON to verify structure
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["mode"], "json");
    assert_eq!(json["language"], "Rust");
    assert!(json["lines"].is_array());
    assert!(json["total_lines"].is_number());
    assert!(json["total_bytes"].is_number());
}

#[test]
fn test_max_lines_limit() {
    let content = "line 1\nline 2\nline 3\nline 4\nline 5\n";
    let file = create_test_file(content, ".txt");

    let output = run_batless(&[
        file.path().to_str().unwrap(),
        "--mode=plain",
        "--max-lines=3",
    ]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.trim().split('\n').collect();

    // Should have 3 content lines + 1 truncation message
    assert_eq!(lines.len(), 4);
    assert!(lines[3].contains("Output truncated after 3 lines"));
}

#[test]
fn test_max_bytes_limit() {
    let content = "Short line 1\nShort line 2\nShort line 3\nShort line 4\n";
    let file = create_test_file(content, ".txt");

    let output = run_batless(&[
        file.path().to_str().unwrap(),
        "--mode=plain",
        "--max-bytes=25",
        "--max-lines=100", // Large line limit so bytes limit takes effect
    ]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Should be truncated since the content is longer than 25 bytes
    assert!(stdout.contains("Output truncated after") && stdout.contains("bytes"));
}

#[test]
fn test_language_detection() {
    let python_content = "def hello():\n    print('Hello, world!')\n";
    let file = create_test_file(python_content, ".py");

    let output = run_batless(&[file.path().to_str().unwrap(), "--mode=json"]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["language"], "Python");
}

#[test]
fn test_explicit_language() {
    let content = "function hello() {\n    console.log('Hello');\n}\n";
    let file = create_test_file(content, ".unknown");

    let output = run_batless(&[
        file.path().to_str().unwrap(),
        "--language=javascript",
        "--mode=json",
    ]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["language"], "javascript");
}

#[test]
fn test_color_never() {
    let content = "fn main() {\n    println!(\"Hello\");\n}\n";
    let file = create_test_file(content, ".rs");

    let output = run_batless(&[
        file.path().to_str().unwrap(),
        "--mode=highlight",
        "--color=never",
    ]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // With color=never, should not contain ANSI escape sequences
    assert!(!stdout.contains("\x1b["));
}

#[test]
fn test_strip_ansi() {
    let content = "fn main() {\n    println!(\"Hello\");\n}\n";
    let file = create_test_file(content, ".rs");

    let output = run_batless(&[
        file.path().to_str().unwrap(),
        "--mode=plain",
        "--strip-ansi",
    ]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("fn main()"));
}

#[test]
fn test_custom_theme() {
    let content = "fn main() {\n    println!(\"Hello\");\n}\n";
    let file = create_test_file(content, ".rs");

    let output = run_batless(&[
        file.path().to_str().unwrap(),
        "--mode=highlight",
        "--theme=InspiredGitHub",
        "--color=always",
    ]);

    assert!(output.status.success());
    // Should not crash with custom theme
}

#[test]
fn test_empty_file() {
    let file = create_test_file("", ".txt");

    let output = run_batless(&[file.path().to_str().unwrap(), "--mode=json"]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["total_lines"], 0);
    assert_eq!(json["total_bytes"], 0);
    assert_eq!(json["truncated"], false);
}

#[test]
fn test_nonexistent_file() {
    let output = run_batless(&["/nonexistent/file.txt", "--mode=plain"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    // Windows may have different error messages
    assert!(
        stderr.contains("No such file")
            || stderr.contains("not found")
            || stderr.contains("cannot find")
            || stderr.contains("system cannot find")
    );
}

#[test]
fn test_multiple_options_combined() {
    let content = "def func1():\n    pass\n\ndef func2():\n    pass\n\ndef func3():\n    pass\n";
    let file = create_test_file(content, ".py");

    let output = run_batless(&[
        file.path().to_str().unwrap(),
        "--mode=json",
        "--max-lines=2",
        "--language=python",
    ]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();

    assert_eq!(json["language"], "python");
    assert_eq!(json["total_lines"], 2);
    assert_eq!(json["truncated"], true);
    assert_eq!(json["lines"].as_array().unwrap().len(), 2);
}

#[test]
fn test_list_languages() {
    let output = run_batless(&["--list-languages"]);
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Rust"));
    assert!(stdout.contains("Python"));
    assert!(stdout.contains("JavaScript"));
}

#[test]
fn test_list_themes() {
    let output = run_batless(&["--list-themes"]);
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("base16-ocean.dark"));
    assert!(stdout.contains("InspiredGitHub"));
}

#[test]
fn test_summary_mode() {
    let content = "import os\nimport sys\n\ndef main():\n    print('hello')\n    x = 1\n    y = 2\n\nclass Test:\n    def method(self):\n        pass\n";
    let file = create_test_file(content, ".py");

    let output = run_batless(&[file.path().to_str().unwrap(), "--mode=summary"]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("import os"));
    assert!(stdout.contains("def main():"));
    assert!(stdout.contains("class Test:"));
    assert!(!stdout.contains("x = 1")); // Should not include non-summary lines
}

#[test]
fn test_summary_flag() {
    let content = "fn main() {\n    println!(\"hello\");\n    let x = 1;\n}\n\nstruct Test {\n    name: String,\n}\n";
    let file = create_test_file(content, ".rs");

    let output = run_batless(&[file.path().to_str().unwrap(), "--summary", "--mode=plain"]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("fn main()"));
    assert!(stdout.contains("struct Test"));
    assert!(!stdout.contains("let x = 1")); // Should not include non-summary lines
}

#[test]
fn test_include_tokens() {
    let content = "fn main() {\n    println!(\"Hello\");\n}\n";
    let file = create_test_file(content, ".rs");

    let output = run_batless(&[
        file.path().to_str().unwrap(),
        "--mode=json",
        "--include-tokens",
    ]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();

    assert!(json["tokens"].is_array());
    let tokens = json["tokens"].as_array().unwrap();
    assert!(!tokens.is_empty());
}

#[test]
fn test_enhanced_json_output() {
    let content = "def hello():\n    print('world')\n";
    let file = create_test_file(content, ".py");

    let output = run_batless(&[
        file.path().to_str().unwrap(),
        "--mode=json",
        "--include-tokens",
    ]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();

    // Check new fields
    assert!(json["encoding"].is_string());
    assert!(json["syntax_errors"].is_array());
    assert!(json["truncated_by_lines"].is_boolean());
    assert!(json["truncated_by_bytes"].is_boolean());
    assert!(json["tokens"].is_array());
}

#[test]
fn test_summary_with_no_important_lines() {
    let content = "// Just comments\n// Nothing important\n// More comments\n";
    let file = create_test_file(content, ".rs");

    let output = run_batless(&[file.path().to_str().unwrap(), "--mode=summary"]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("// No summary-worthy code structures found"));
}

#[test]
fn test_ai_profile_claude() {
    let test_file = create_test_file("fn main() {\n    println!(\"hello\");\n}\n", ".rs");
    let output = run_batless(&["--profile", "claude", test_file.path().to_str().unwrap()]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("=== File Summary ==="));
    assert!(stdout.contains("Language: Rust"));
}

#[test]
fn test_ai_profile_copilot() {
    let test_file = create_test_file("fn main() {\n    println!(\"hello\");\n}\n", ".rs");
    let output = run_batless(&["--profile", "copilot", test_file.path().to_str().unwrap()]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Should be JSON output with tokens
    assert!(stdout.contains("\"language\": \"Rust\""));
    assert!(stdout.contains("\"tokens\":"));
}

#[test]
fn test_ai_profile_chatgpt() {
    let test_file = create_test_file("def hello():\n    print('world')\n", ".py");
    let output = run_batless(&["--profile", "chatgpt", test_file.path().to_str().unwrap()]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Should be JSON output with tokens
    assert!(stdout.contains("\"language\": \"Python\""));
    assert!(stdout.contains("\"tokens\":"));
}

#[test]
fn test_ai_profile_assistant() {
    let test_file = create_test_file("class Test {\n    public void run() {}\n}\n", ".java");
    let output = run_batless(&["--profile", "assistant", test_file.path().to_str().unwrap()]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Should be summary output
    assert!(stdout.contains("=== File Summary ==="));
}

#[test]
fn test_profile_overrides_other_flags() {
    let test_file = create_test_file("fn main() {}\n", ".rs");
    let output = run_batless(&[
        "--profile",
        "claude",
        "--mode",
        "json", // This should be overridden by claude profile
        "--max-lines",
        "1000", // This should be overridden by claude profile
        test_file.path().to_str().unwrap(),
    ]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Should still be summary output (claude profile), not JSON
    assert!(stdout.contains("=== File Summary ==="));
    assert!(!stdout.starts_with("{"));
}

#[test]
fn test_shell_completion_generation_bash() {
    let output = run_batless(&["--generate-completions", "bash"]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("_batless()"));
    assert!(stdout.contains("COMPREPLY=()"));
}

#[test]
fn test_shell_completion_generation_zsh() {
    let output = run_batless(&["--generate-completions", "zsh"]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("#compdef batless"));
}

#[test]
fn test_shell_completion_generation_fish() {
    let output = run_batless(&["--generate-completions", "fish"]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("complete -c batless"));
}

#[test]
fn test_shell_completion_generation_powershell() {
    let output = run_batless(&["--generate-completions", "power-shell"]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Register-ArgumentCompleter"));
}

#[test]
fn test_stdin_processing() {
    // Test that stdin input is properly processed by using echo and pipe
    // This is more reliable than trying to capture cargo run output
    let output = Command::new("sh")
        .arg("-c")
        .arg("echo 'test line 1\\ntest line 2' | cargo run -- --mode=json")
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    
    // The output goes to stdout when using shell pipe
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("test line 1"));
    assert!(stdout.contains("test line 2"));
    assert!(stdout.contains("file\": \"-"));
}

#[test]
fn test_summary_mode_different_languages() {
    // Test JavaScript
    let js_content = "import React from 'react';\n\nfunction Component() {\n    console.log('test');\n    return <div>Hello</div>;\n}\n\nexport default Component;\n";
    let js_file = create_test_file(js_content, ".js");

    let output = run_batless(&[
        js_file.path().to_str().unwrap(),
        "--mode=summary",
        "--language=javascript",
    ]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("import React"));
    assert!(stdout.contains("function Component"));
    assert!(stdout.contains("export default"));
}
