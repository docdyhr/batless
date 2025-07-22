//! Core functionality for batless - a minimal, AI-friendly code viewer
//!
//! This library provides the core logic for syntax highlighting and file processing
//! that can be used both by the CLI and in tests.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

pub struct BatlessConfig {
    pub max_lines: usize,
    pub max_bytes: Option<usize>,
    pub language: Option<String>,
    pub theme: String,
    pub strip_ansi: bool,
    pub use_color: bool,
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
}

/// Read and process a file according to the given configuration
pub fn process_file(
    file_path: &str,
    config: &BatlessConfig,
) -> Result<FileInfo, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    let mut line_count = 0;
    let mut byte_count = 0;
    let mut truncated = false;
    let mut truncated_by_lines = false;
    let mut truncated_by_bytes = false;

    for line_result in reader.lines() {
        if line_count >= config.max_lines {
            truncated = true;
            truncated_by_lines = true;
            break;
        }

        let line = line_result?;
        let line_bytes = line.len() + 1; // +1 for newline

        if let Some(max_bytes) = config.max_bytes {
            if byte_count + line_bytes > max_bytes {
                truncated = true;
                truncated_by_bytes = true;
                break;
            }
        }

        lines.push(line);
        line_count += 1;
        byte_count += line_bytes;
    }

    Ok(FileInfo {
        lines,
        total_lines: line_count,
        total_bytes: byte_count,
        truncated,
        truncated_by_lines,
        truncated_by_bytes,
        language: config.language.clone(),
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

    // Load syntax set and theme set
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    // Get the theme
    let theme = ts
        .themes
        .get(&config.theme)
        .or_else(|| ts.themes.get("base16-ocean.dark"))
        .ok_or("Failed to load theme")?;

    // Determine syntax based on file extension or explicit language
    let syntax = if let Some(lang) = &config.language {
        ps.find_syntax_by_name(lang)
            .or_else(|| ps.find_syntax_by_extension(lang))
            .unwrap_or_else(|| ps.find_syntax_plain_text())
    } else {
        let path = Path::new(file_path);
        ps.find_syntax_for_file(path)?
            .unwrap_or_else(|| ps.find_syntax_plain_text())
    };

    let mut highlighter = HighlightLines::new(syntax, theme);
    let mut result = String::new();

    for line in LinesWithEndings::from(content) {
        let ranges: Vec<(syntect::highlighting::Style, &str)> =
            highlighter.highlight_line(line, &ps)?;
        let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
        result.push_str(&escaped);
    }

    Ok(result)
}

/// Detect the language of a file based on its extension
pub fn detect_language(file_path: &str) -> Option<String> {
    let ps = SyntaxSet::load_defaults_newlines();
    let path = Path::new(file_path);

    if let Ok(Some(syntax)) = ps.find_syntax_for_file(path) {
        Some(syntax.name.clone())
    } else {
        None
    }
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
    fn test_config_default() {
        let config = BatlessConfig::default();
        assert_eq!(config.max_lines, 10000);
        assert_eq!(config.max_bytes, None);
        assert_eq!(config.language, None);
        assert_eq!(config.theme, "base16-ocean.dark");
        assert!(!config.strip_ansi);
        assert!(config.use_color);
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
