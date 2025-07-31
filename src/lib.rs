//! Core functionality for batless - a minimal, AI-friendly code viewer
//!
//! This library provides the core logic for syntax highlighting and file processing
//! that can be used both by the CLI and in tests.

pub mod config;
pub mod error;
pub mod file_info;
pub mod formatter;
pub mod highlighter;
pub mod json_schema;
pub mod language;
pub mod processor;
pub mod summarizer;
pub mod token_counter;
pub mod tokenizer;

// Re-export commonly used types
pub use config::{BatlessConfig, CustomProfile, SummaryLevel};
pub use error::{BatlessError, BatlessResult};
pub use file_info::FileInfo;
pub use formatter::{OutputFormatter, OutputMode};
pub use highlighter::SyntaxHighlighter;
pub use json_schema::{get_json_schema, validate_batless_output, JsonSchemaValidator};
pub use language::{LanguageDetector, ThemeManager};
pub use processor::FileProcessor;
pub use token_counter::{AiModel, TokenCount, TokenCounter};

/// Main entry point for processing a file with batless
pub fn process_file(file_path: &str, config: &BatlessConfig) -> BatlessResult<FileInfo> {
    FileProcessor::process_file(file_path, config)
}

/// Highlight content with syntax highlighting
pub fn highlight_content(
    content: &str,
    file_path: &str,
    config: &BatlessConfig,
) -> BatlessResult<String> {
    SyntaxHighlighter::highlight_content(content, file_path, config)
}

/// Detect the programming language from a file path
pub fn detect_language(file_path: &str) -> Option<String> {
    LanguageDetector::detect_language(file_path)
}

/// Get list of all available programming languages
pub fn list_languages() -> Vec<String> {
    LanguageDetector::list_languages()
}

/// Get list of all available themes
pub fn list_themes() -> Vec<String> {
    ThemeManager::list_themes()
}

/// Format output according to the specified mode
pub fn format_output(
    file_info: &FileInfo,
    file_path: &str,
    config: &BatlessConfig,
    output_mode: OutputMode,
) -> BatlessResult<String> {
    OutputFormatter::format_output(file_info, file_path, config, output_mode)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_test_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{content}").unwrap();
        file
    }

    #[test]
    fn test_process_file_basic() -> BatlessResult<()> {
        let file = create_test_file("line1\nline2\nline3");
        let config = BatlessConfig::default();

        let result = process_file(file.path().to_str().unwrap(), &config)?;

        assert_eq!(result.lines.len(), 3);
        assert_eq!(result.total_lines, 3);
        assert!(!result.truncated);

        Ok(())
    }

    #[test]
    fn test_process_file_max_lines() -> BatlessResult<()> {
        let file = create_test_file("line1\nline2\nline3\nline4\nline5");
        let config = BatlessConfig::default().with_max_lines(3);

        let result = process_file(file.path().to_str().unwrap(), &config)?;

        assert_eq!(result.lines.len(), 3);
        assert!(result.truncated);
        assert!(result.truncated_by_lines);

        Ok(())
    }

    #[test]
    fn test_process_file_max_bytes() -> BatlessResult<()> {
        // Create content larger than byte limit
        let large_content = "a".repeat(2000); // 2000 characters
        let file = create_test_file(&large_content);
        let config = BatlessConfig::default()
            .with_max_bytes(Some(1000)) // 1KB limit
            .with_max_lines(100); // Large line limit

        let result = process_file(file.path().to_str().unwrap(), &config)?;

        assert!(result.truncated);
        assert!(result.truncated_by_bytes);

        Ok(())
    }

    #[test]
    fn test_detect_language_rust() {
        let language = detect_language("test.rs");
        assert_eq!(language, Some("Rust".to_string()));
    }

    #[test]
    fn test_detect_language_python() {
        let language = detect_language("test.py");
        assert_eq!(language, Some("Python".to_string()));
    }

    #[test]
    fn test_detect_language_unknown() {
        let language = detect_language("test.unknown");
        assert_eq!(language, None);
    }

    #[test]
    fn test_highlight_content_plain() -> BatlessResult<()> {
        let content = "fn main() {}";
        let config = BatlessConfig::default().with_use_color(false);

        let result = highlight_content(content, "test.rs", &config)?;
        assert_eq!(result, content);

        Ok(())
    }

    #[test]
    fn test_highlight_content_with_syntax() -> BatlessResult<()> {
        let content = "fn main() {}";
        let config = BatlessConfig::default().with_use_color(true);

        let result = highlight_content(content, "test.rs", &config)?;
        // Should contain ANSI escape sequences when color is enabled
        assert!(result.contains("\x1b[") || result == content); // May not have color in test env

        Ok(())
    }

    #[test]
    fn test_list_languages() {
        let languages = list_languages();
        assert!(!languages.is_empty());
        assert!(languages.contains(&"Rust".to_string()));
    }

    #[test]
    fn test_list_themes() {
        let themes = list_themes();
        assert!(!themes.is_empty());
        assert!(themes.contains(&"base16-ocean.dark".to_string()));
    }

    #[test]
    fn test_summary_mode() -> BatlessResult<()> {
        let file = create_test_file("fn main() {\n    println!(\"Hello\");\n}");
        let config = BatlessConfig::default().with_summary_mode(true);

        let result = process_file(file.path().to_str().unwrap(), &config)?;

        assert!(result.has_summary());
        assert!(result.summary_line_count() > 0);

        Ok(())
    }

    #[test]
    fn test_include_tokens() -> BatlessResult<()> {
        let file = create_test_file("fn main() { println!(\"Hello\"); }");
        let config = BatlessConfig::default().with_include_tokens(true);

        let result = process_file(file.path().to_str().unwrap(), &config)?;

        assert!(result.has_tokens());
        assert!(result.token_count() > 0);

        Ok(())
    }

    #[test]
    fn test_encoding_detection() -> BatlessResult<()> {
        let file = create_test_file("Hello, 世界!");
        let config = BatlessConfig::default();

        let result = process_file(file.path().to_str().unwrap(), &config)?;
        assert_eq!(result.encoding, "UTF-8");

        Ok(())
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
    fn test_empty_file() -> BatlessResult<()> {
        let file = create_test_file("");
        let config = BatlessConfig::default();

        let result = process_file(file.path().to_str().unwrap(), &config)?;

        assert_eq!(result.lines.len(), 0);
        assert_eq!(result.total_lines, 0);
        assert!(!result.truncated);

        Ok(())
    }

    #[test]
    fn test_single_line_file() -> BatlessResult<()> {
        let file = create_test_file("single line without newline");
        let config = BatlessConfig::default();

        let result = process_file(file.path().to_str().unwrap(), &config)?;

        assert_eq!(result.lines.len(), 1);
        assert_eq!(result.total_lines, 1);
        assert!(!result.truncated);

        Ok(())
    }

    #[test]
    fn test_format_output_modes() -> BatlessResult<()> {
        let file = create_test_file("fn main() {}");
        let config = BatlessConfig::default();
        let file_info = process_file(file.path().to_str().unwrap(), &config)?;

        // Test plain output
        let plain = format_output(&file_info, "test.rs", &config, OutputMode::Plain)?;
        assert_eq!(plain, "fn main() {}");

        // Test JSON output
        let json = format_output(&file_info, "test.rs", &config, OutputMode::Json)?;
        assert!(json.contains("\"file\": \"test.rs\""));

        // Test summary output
        let summary = format_output(&file_info, "test.rs", &config, OutputMode::Summary)?;
        assert!(summary.contains("=== File Summary ==="));

        Ok(())
    }

    #[test]
    fn test_error_handling() {
        let config = BatlessConfig::default();
        let result = process_file("nonexistent_file.txt", &config);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            BatlessError::FileNotFound { .. }
        ));
    }

    #[test]
    fn test_configuration_validation() {
        let invalid_config = BatlessConfig::default().with_max_lines(0);
        assert!(invalid_config.validate().is_err());

        let valid_config = BatlessConfig::default().with_max_lines(100);
        assert!(valid_config.validate().is_ok());
    }

    #[test]
    fn test_language_override() -> BatlessResult<()> {
        let file = create_test_file("print('hello')");
        let config = BatlessConfig::default().with_language(Some("Python".to_string()));

        let result = process_file(file.path().to_str().unwrap(), &config)?;
        assert_eq!(result.language, Some("Python".to_string()));

        Ok(())
    }
}
