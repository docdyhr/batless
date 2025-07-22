//! Core functionality for batless - a minimal, AI-friendly code viewer
//!
//! This library provides the core logic for syntax highlighting and file processing
//! that can be used both by the CLI and in tests.

use lazy_static::lazy_static;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

// Cache syntax and theme sets for better performance
lazy_static! {
    static ref SYNTAX_SET: SyntaxSet = SyntaxSet::load_defaults_newlines();
    static ref THEME_SET: ThemeSet = ThemeSet::load_defaults();
}

#[derive(Clone)]
pub struct BatlessConfig {
    pub max_lines: usize,
    pub max_bytes: Option<usize>,
    pub language: Option<String>,
    pub theme: String,
    pub strip_ansi: bool,
    pub use_color: bool,
    pub include_tokens: bool,
    pub summary_mode: bool,
}

impl Default for BatlessConfig {
    fn default() -> Self {
        Self {
            max_lines: 10000,
            max_bytes: None,
            language: None,
            theme: "base16-ocean.dark".to_string(),
            strip_ansi: false,
            use_color: true,
            include_tokens: false,
            summary_mode: false,
        }
    }
}

pub struct FileInfo {
    pub lines: Vec<String>,
    pub total_lines: usize,
    pub total_bytes: usize,
    pub truncated: bool,
    pub truncated_by_lines: bool,
    pub truncated_by_bytes: bool,
    pub language: Option<String>,
    pub encoding: String,
    pub syntax_errors: Vec<String>,
    pub tokens: Option<Vec<String>>,
    pub summary_lines: Option<Vec<String>>,
}

/// Read and process a file according to the given configuration
pub fn process_file(
    file_path: &str,
    config: &BatlessConfig,
) -> Result<FileInfo, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;

    // Detect encoding
    let mut buffer = vec![0; 1024.min(file.metadata()?.len() as usize)];
    let bytes_read = file.read(&mut buffer)?;
    buffer.truncate(bytes_read);

    let (_encoding, _, had_errors) = encoding_rs::UTF_8.decode(&buffer);
    let encoding_name = if !had_errors {
        "UTF-8".to_string()
    } else {
        "Unknown".to_string()
    };

    // Reset file position
    file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    let mut all_lines = Vec::new(); // For summary mode processing
    let mut line_count = 0;
    let mut byte_count = 0;
    let mut truncated = false;
    let mut truncated_by_lines = false;
    let mut truncated_by_bytes = false;

    // First pass: collect all lines if summary mode or collect limited lines
    for line_result in reader.lines() {
        let line = line_result?;
        let line_bytes = line.len() + 1; // +1 for newline

        if config.summary_mode {
            all_lines.push(line.clone());
        } else {
            if line_count >= config.max_lines {
                truncated = true;
                truncated_by_lines = true;
                break;
            }

            if let Some(max_bytes) = config.max_bytes {
                if byte_count + line_bytes > max_bytes {
                    truncated = true;
                    truncated_by_bytes = true;
                    break;
                }
            }

            lines.push(line);
        }

        line_count += 1;
        byte_count += line_bytes;
    }

    // Process summary if requested
    let summary_lines = if config.summary_mode {
        let summary = extract_summary(&all_lines, detect_language(file_path));
        // If summary mode, replace lines with summary for output
        lines = summary.clone();
        Some(summary)
    } else {
        None
    };

    // Extract tokens if requested
    let tokens = if config.include_tokens {
        let content = if config.summary_mode && summary_lines.is_some() {
            summary_lines.as_ref().unwrap().join("\n")
        } else {
            lines.join("\n")
        };
        Some(extract_tokens(&content, file_path))
    } else {
        None
    };

    Ok(FileInfo {
        lines,
        total_lines: line_count,
        total_bytes: byte_count,
        truncated,
        truncated_by_lines,
        truncated_by_bytes,
        language: config.language.clone(),
        encoding: encoding_name,
        syntax_errors: Vec::new(), // TODO: Implement syntax error detection
        tokens,
        summary_lines,
    })
}

/// Apply syntax highlighting to content
pub fn highlight_content(
    content: &str,
    file_path: &str,
    config: &BatlessConfig,
) -> Result<String, Box<dyn std::error::Error>> {
    if !config.use_color {
        return Ok(content.to_string());
    }

    // Use cached syntax and theme sets
    let theme = THEME_SET
        .themes
        .get(&config.theme)
        .or_else(|| THEME_SET.themes.get("base16-ocean.dark"))
        .ok_or("Failed to load theme")?;

    // Determine syntax based on file extension or explicit language
    let syntax = if let Some(lang) = &config.language {
        SYNTAX_SET
            .find_syntax_by_name(lang)
            .or_else(|| SYNTAX_SET.find_syntax_by_extension(lang))
            .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text())
    } else {
        let path = Path::new(file_path);
        SYNTAX_SET
            .find_syntax_for_file(path)?
            .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text())
    };

    let mut highlighter = HighlightLines::new(syntax, theme);
    let mut result = String::new();

    for line in LinesWithEndings::from(content) {
        let ranges: Vec<(syntect::highlighting::Style, &str)> =
            highlighter.highlight_line(line, &SYNTAX_SET)?;
        let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
        result.push_str(&escaped);
    }

    Ok(result)
}

/// Detect the language of a file based on its extension
pub fn detect_language(file_path: &str) -> Option<String> {
    let path = Path::new(file_path);

    if let Ok(Some(syntax)) = SYNTAX_SET.find_syntax_for_file(path) {
        Some(syntax.name.clone())
    } else {
        None
    }
}

/// Get list of all supported languages
pub fn list_languages() -> Vec<String> {
    let mut languages: Vec<String> = SYNTAX_SET
        .syntaxes()
        .iter()
        .map(|syntax| syntax.name.clone())
        .collect();
    languages.sort();
    languages
}

/// Get list of all available themes
pub fn list_themes() -> Vec<String> {
    let mut themes: Vec<String> = THEME_SET.themes.keys().cloned().collect();
    themes.sort();
    themes
}

/// Extract summary information from code (function signatures, class definitions, imports)
fn extract_summary(lines: &[String], language: Option<String>) -> Vec<String> {
    let mut summary = Vec::new();

    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty()
            || trimmed.starts_with("//")
            || trimmed.starts_with("#") && !is_important_comment(trimmed)
        {
            continue;
        }

        // Language-agnostic patterns for important code structures
        if is_summary_worthy(trimmed, &language) {
            summary.push(line.clone());
        }
    }

    summary
}

/// Check if a line contains summary-worthy code
fn is_summary_worthy(line: &str, language: &Option<String>) -> bool {
    let trimmed = line.trim();

    // Skip empty lines and comments
    if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with("#") {
        return false;
    }

    // Language-specific important patterns
    match language.as_ref().map(|s| s.as_str()) {
        Some("Python") => {
            trimmed.starts_with("def ")
                || trimmed.starts_with("class ")
                || trimmed.starts_with("import ")
                || trimmed.starts_with("from ")
                || trimmed.starts_with("@")
        }
        Some("Rust") => {
            // Only match function/struct/enum declarations, not their contents
            (trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ")) && trimmed.contains("{")
                || (trimmed.starts_with("struct ") || trimmed.starts_with("pub struct "))
                || (trimmed.starts_with("enum ") || trimmed.starts_with("pub enum "))
                || (trimmed.starts_with("trait ") || trimmed.starts_with("pub trait "))
                || trimmed.starts_with("impl ")
                || trimmed.starts_with("use ")
                || (trimmed.starts_with("const ") || trimmed.starts_with("pub const "))
                || (trimmed.starts_with("static ") || trimmed.starts_with("pub static "))
        }
        Some("JavaScript") | Some("TypeScript") => {
            trimmed.starts_with("function ")
                || trimmed.starts_with("class ")
                || trimmed.starts_with("interface ")
                || trimmed.starts_with("export ")
                || trimmed.starts_with("import ")
                || (trimmed.starts_with("const ")
                    && (trimmed.contains("function") || trimmed.contains("=>")))
                || (trimmed.starts_with("let ")
                    && (trimmed.contains("function") || trimmed.contains("=>")))
        }
        _ => {
            // Generic patterns for unknown languages - only top-level declarations
            (trimmed.starts_with("def ") && trimmed.contains(":"))
                || (trimmed.starts_with("class ") && trimmed.contains(":"))
                || (trimmed.starts_with("function ") && trimmed.contains("{"))
                || ((trimmed.starts_with("fn ") || trimmed.starts_with("pub fn "))
                    && trimmed.contains("{"))
                || (trimmed.starts_with("struct ") || trimmed.starts_with("pub struct "))
                || (trimmed.starts_with("enum ") || trimmed.starts_with("pub enum "))
                || trimmed.starts_with("import ")
                || trimmed.starts_with("use ")
                || trimmed.starts_with("export ")
                || trimmed.starts_with("module ")
        }
    }
}

/// Check if a comment is important enough to include in summary
fn is_important_comment(line: &str) -> bool {
    let line_lower = line.to_lowercase();
    line_lower.contains("todo")
        || line_lower.contains("fixme")
        || line_lower.contains("hack")
        || line_lower.contains("note")
        || line_lower.starts_with("///")
        || line_lower.starts_with("/**")
        || line_lower.starts_with("#!")
}

/// Extract tokens from content for AI processing
fn extract_tokens(content: &str, _file_path: &str) -> Vec<String> {
    // Simple token extraction - can be enhanced with proper lexing
    let mut tokens = Vec::new();

    // Basic word-based tokenization
    for line in content.lines() {
        let line_tokens: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        tokens.extend(line_tokens);
    }

    // Remove duplicates and sort
    tokens.sort();
    tokens.dedup();

    // Limit token count to prevent overflow
    if tokens.len() > 1000 {
        tokens.truncate(1000);
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_test_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file
    }

    #[test]
    fn test_process_file_basic() {
        let content = "line 1\nline 2\nline 3\n";
        let file = create_test_file(content);
        let config = BatlessConfig::default();

        let result = process_file(file.path().to_str().unwrap(), &config).unwrap();

        assert_eq!(result.lines.len(), 3);
        assert_eq!(result.lines[0], "line 1");
        assert_eq!(result.lines[1], "line 2");
        assert_eq!(result.lines[2], "line 3");
        assert_eq!(result.total_lines, 3);
        assert!(!result.truncated);
        assert!(!result.truncated_by_lines);
        assert!(!result.truncated_by_bytes);
    }

    #[test]
    fn test_process_file_max_lines() {
        let content = "line 1\nline 2\nline 3\nline 4\nline 5\n";
        let file = create_test_file(content);
        let mut config = BatlessConfig::default();
        config.max_lines = 3;

        let result = process_file(file.path().to_str().unwrap(), &config).unwrap();

        assert_eq!(result.lines.len(), 3);
        assert_eq!(result.total_lines, 3);
        assert!(result.truncated);
        assert!(result.truncated_by_lines);
        assert!(!result.truncated_by_bytes);
    }

    #[test]
    fn test_process_file_max_bytes() {
        let content = "short\nlonger line\neven longer line\n";
        let file = create_test_file(content);
        let mut config = BatlessConfig::default();
        config.max_bytes = Some(15); // Should stop after first two lines

        let result = process_file(file.path().to_str().unwrap(), &config).unwrap();

        assert!(result.truncated);
        assert!(result.truncated_by_bytes);
        assert!(!result.truncated_by_lines);
        assert!(result.total_bytes <= 15);
    }

    #[test]
    fn test_detect_language_rust() {
        let detected = detect_language("test.rs");
        assert_eq!(detected, Some("Rust".to_string()));
    }

    #[test]
    fn test_detect_language_python() {
        let detected = detect_language("test.py");
        assert_eq!(detected, Some("Python".to_string()));
    }

    #[test]
    fn test_detect_language_unknown() {
        let detected = detect_language("test.unknown");
        assert!(detected.is_none() || detected == Some("Plain Text".to_string()));
    }

    #[test]
    fn test_highlight_content_plain() {
        let content = "fn main() {\n    println!(\"Hello\");\n}\n";
        let mut config = BatlessConfig::default();
        config.use_color = false;

        let result = highlight_content(content, "test.rs", &config).unwrap();
        assert_eq!(result, content);
    }

    #[test]
    fn test_highlight_content_with_syntax() {
        let content = "fn main() {\n    println!(\"Hello\");\n}\n";
        let config = BatlessConfig::default();

        let result = highlight_content(content, "test.rs", &config).unwrap();
        // Should contain ANSI escape codes
        assert!(result.contains("\x1b["));
        assert!(result.len() > content.len());
    }

    #[test]
    fn test_list_languages() {
        let languages = list_languages();
        assert!(!languages.is_empty());
        assert!(languages.contains(&"Rust".to_string()));
        assert!(languages.contains(&"Python".to_string()));
    }

    #[test]
    fn test_list_themes() {
        let themes = list_themes();
        assert!(!themes.is_empty());
        assert!(themes.contains(&"base16-ocean.dark".to_string()));
    }

    #[test]
    fn test_summary_mode() {
        let content =
            "import os\n\ndef main():\n    print('hello')\n    x = 1\n\nclass Test:\n    pass\n";
        let file = create_test_file(content);
        let mut config = BatlessConfig::default();
        config.summary_mode = true;

        let result = process_file(file.path().to_str().unwrap(), &config).unwrap();

        assert!(result.summary_lines.is_some());
        let summary = result.summary_lines.unwrap();
        assert!(summary.iter().any(|line| line.contains("import")));
        assert!(summary.iter().any(|line| line.contains("def main")));
        assert!(summary.iter().any(|line| line.contains("class Test")));
    }

    #[test]
    fn test_include_tokens() {
        let content = "fn main() {\n    let x = 42;\n}\n";
        let file = create_test_file(content);
        let mut config = BatlessConfig::default();
        config.include_tokens = true;

        let result = process_file(file.path().to_str().unwrap(), &config).unwrap();

        assert!(result.tokens.is_some());
        let tokens = result.tokens.unwrap();
        assert!(tokens.contains(&"fn".to_string()));
        assert!(tokens.contains(&"main()".to_string()));
        assert!(tokens.contains(&"let".to_string()));
    }

    #[test]
    fn test_encoding_detection() {
        let content = "Hello, 世界!";
        let file = create_test_file(content);
        let config = BatlessConfig::default();

        let result = process_file(file.path().to_str().unwrap(), &config).unwrap();
        assert_eq!(result.encoding, "UTF-8");
    }

    #[test]
    fn test_config_default() {
        let config = BatlessConfig::default();
        assert_eq!(config.max_lines, 10000);
        assert_eq!(config.max_bytes, None);
        assert_eq!(config.language, None);
        assert_eq!(config.theme, "base16-ocean.dark");
        assert!(!config.strip_ansi);
        assert!(config.use_color);
        assert!(!config.include_tokens);
        assert!(!config.summary_mode);
    }

    #[test]
    fn test_empty_file() {
        let content = "";
        let file = create_test_file(content);
        let config = BatlessConfig::default();

        let result = process_file(file.path().to_str().unwrap(), &config).unwrap();

        assert_eq!(result.lines.len(), 0);
        assert_eq!(result.total_lines, 0);
        assert_eq!(result.total_bytes, 0);
        assert!(!result.truncated);
        assert!(!result.truncated_by_lines);
        assert!(!result.truncated_by_bytes);
    }

    #[test]
    fn test_single_line_file() {
        let content = "single line without newline";
        let file = create_test_file(content);
        let config = BatlessConfig::default();

        let result = process_file(file.path().to_str().unwrap(), &config).unwrap();

        assert_eq!(result.lines.len(), 1);
        assert_eq!(result.lines[0], "single line without newline");
        assert_eq!(result.total_lines, 1);
        assert!(!result.truncated);
        assert!(!result.truncated_by_lines);
        assert!(!result.truncated_by_bytes);
    }
}
