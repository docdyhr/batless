//! Output formatting functionality for batless
//!
//! This module handles different output formats including plain text,
//! syntax-highlighted text, JSON, and structured data formatting.

use crate::config::BatlessConfig;
use crate::error::{BatlessError, BatlessResult};
use crate::file_info::FileInfo;
use crate::highlighter::SyntaxHighlighter;
use serde_json::json;

/// Output formatter for different display modes
pub struct OutputFormatter;

impl OutputFormatter {
    /// Format file info according to the specified output mode
    pub fn format_output(
        file_info: &FileInfo,
        file_path: &str,
        config: &BatlessConfig,
        output_mode: OutputMode,
    ) -> BatlessResult<String> {
        match output_mode {
            OutputMode::Plain => Self::format_plain(file_info, config),
            OutputMode::Highlight => Self::format_highlighted(file_info, file_path, config),
            OutputMode::Json => Self::format_json(file_info, file_path, config),
            OutputMode::Summary => Self::format_summary(file_info, config),
        }
    }

    /// Format as plain text without highlighting
    fn format_plain(file_info: &FileInfo, config: &BatlessConfig) -> BatlessResult<String> {
        if config.show_line_numbers || config.show_line_numbers_nonblank {
            let mut result = Vec::new();
            let mut line_number = 1;

            for line in &file_info.lines {
                if config.show_line_numbers_nonblank {
                    // Only number non-blank lines (cat -b behavior)
                    if line.trim().is_empty() {
                        result.push(line.clone());
                    } else {
                        result.push(format!("{line_number:6}\t{line}"));
                        line_number += 1;
                    }
                } else {
                    // Number all lines (cat -n behavior)
                    result.push(format!("{line_number:6}\t{line}"));
                    line_number += 1;
                }
            }
            Ok(result.join("\n"))
        } else {
            Ok(file_info.lines.join("\n"))
        }
    }

    /// Format with syntax highlighting
    fn format_highlighted(
        file_info: &FileInfo,
        file_path: &str,
        config: &BatlessConfig,
    ) -> BatlessResult<String> {
        let content = file_info.lines.join("\n");
        SyntaxHighlighter::highlight_and_process(&content, file_path, config)
    }

    /// Format as JSON with metadata
    fn format_json(
        file_info: &FileInfo,
        file_path: &str,
        config: &BatlessConfig,
    ) -> BatlessResult<String> {
        // Create backward-compatible JSON format
        let mut json_data = json!({
            "file": file_path,
            "lines": file_info.lines,
            "total_lines": file_info.processed_lines(),
            "total_bytes": file_info.total_bytes,
            "truncated": file_info.truncated,
            "truncated_by_lines": file_info.truncated_by_lines,
            "truncated_by_bytes": file_info.truncated_by_bytes,
            "language": file_info.language,
            "encoding": file_info.encoding,
            "syntax_errors": file_info.syntax_errors,
            "mode": "json"
        });

        // Add optional fields if they exist
        if let Some(ref tokens) = file_info.tokens {
            json_data["tokens"] = json!(tokens);
        }

        if let Some(ref summary_lines) = file_info.summary_lines {
            json_data["summary_lines"] = json!(summary_lines);
        }

        if config.pretty_json {
            serde_json::to_string_pretty(&json_data).map_err(BatlessError::from)
        } else {
            serde_json::to_string(&json_data).map_err(BatlessError::from)
        }
    }

    /// Format summary output
    fn format_summary(file_info: &FileInfo, _config: &BatlessConfig) -> BatlessResult<String> {
        let mut output = Vec::new();

        // File header
        output.push("=== File Summary ===".to_string());
        output.push(format!(
            "Language: {}",
            file_info.language.as_deref().unwrap_or("Unknown")
        ));
        output.push(format!("Encoding: {}", file_info.encoding));
        output.push(format!("Total Lines: {}", file_info.total_lines));
        output.push(format!("Processed Lines: {}", file_info.processed_lines()));

        if file_info.truncated {
            if let Some(reason) = file_info.truncation_reason() {
                output.push(format!("Truncated: Yes ({reason})"));
            } else {
                output.push("Truncated: Yes".to_string());
            }
        }

        output.push(String::new()); // Empty line

        // Summary content
        if let Some(ref summary_lines) = file_info.summary_lines {
            output.push("=== Code Structure ===".to_string());
            for line in summary_lines {
                output.push(line.clone());
            }
        } else {
            output.push("=== Content ===".to_string());
            for line in &file_info.lines {
                output.push(line.clone());
            }
        }

        // Token information if available
        if let Some(ref tokens) = file_info.tokens {
            output.push(String::new());
            output.push("=== Tokens ===".to_string());
            output.push(format!("Token Count: {}", tokens.len()));

            if !tokens.is_empty() {
                output.push("Sample Tokens:".to_string());
                for token in tokens.iter().take(20) {
                    output.push(format!("  {token}"));
                }

                if tokens.len() > 20 {
                    output.push(format!("  ... and {} more", tokens.len() - 20));
                }
            }
        }

        // Error information if any
        if !file_info.syntax_errors.is_empty() {
            output.push(String::new());
            output.push("=== Syntax Errors ===".to_string());
            for error in &file_info.syntax_errors {
                output.push(format!("  {error}"));
            }
        }

        Ok(output.join("\n"))
    }

    /// Format for streaming output (line by line)
    pub fn format_line(
        line: &str,
        line_number: usize,
        file_path: &str,
        config: &BatlessConfig,
        output_mode: OutputMode,
    ) -> BatlessResult<String> {
        match output_mode {
            OutputMode::Plain => Ok(line.to_string()),
            OutputMode::Highlight => SyntaxHighlighter::highlight_content(line, file_path, config),
            OutputMode::Json => {
                let json_line = json!({
                    "line_number": line_number,
                    "content": line
                });
                serde_json::to_string(&json_line).map_err(BatlessError::from)
            }
            OutputMode::Summary => Ok(line.to_string()), // Summary mode doesn't stream
        }
    }

    /// Create a compact JSON representation
    pub fn format_compact_json(file_info: &FileInfo, file_path: &str) -> BatlessResult<String> {
        let json_data = json!({
            "path": file_path,
            "lines": file_info.total_lines,
            "bytes": file_info.total_bytes,
            "language": file_info.language,
            "truncated": file_info.truncated,
            "content": file_info.lines
        });

        serde_json::to_string(&json_data).map_err(BatlessError::from)
    }

    /// Format error information
    pub fn format_error(error: &BatlessError, file_path: &str, output_mode: OutputMode) -> String {
        match output_mode {
            OutputMode::Json => {
                let error_json = json!({
                    "error": true,
                    "file_path": file_path,
                    "error_type": Self::error_type_name(error),
                    "message": error.to_string()
                });
                serde_json::to_string_pretty(&error_json)
                    .unwrap_or_else(|_| format!("{{\"error\": true, \"message\": \"{error}\"}}"))
            }
            _ => format!("Error processing {file_path}: {error}"),
        }
    }

    /// Get a human-readable error type name
    fn error_type_name(error: &BatlessError) -> &'static str {
        match error {
            BatlessError::FileNotFound { .. } => "file_not_found",
            BatlessError::FileReadError { .. } => "file_read_error",
            BatlessError::PermissionDenied { .. } => "permission_denied",
            BatlessError::HighlightError(_) => "highlight_error",
            BatlessError::ThemeNotFound { .. } => "theme_not_found",
            BatlessError::LanguageNotFound { .. } => "language_not_found",
            BatlessError::LanguageDetectionError(_) => "language_detection_error",
            BatlessError::EncodingError { .. } => "encoding_error",
            BatlessError::ProcessingError(_) => "processing_error",
            BatlessError::ConfigurationError { .. } => "configuration_error",
            BatlessError::JsonSerializationError(_) => "json_serialization_error",
            BatlessError::OutputError(_) => "output_error",
            BatlessError::IoError(_) => "io_error",
        }
    }

    /// Format metadata only (without content)
    pub fn format_metadata_only(file_info: &FileInfo, file_path: &str) -> BatlessResult<String> {
        let metadata = json!({
            "file_path": file_path,
            "total_lines": file_info.total_lines,
            "total_bytes": file_info.total_bytes,
            "language": file_info.language,
            "encoding": file_info.encoding,
            "truncated": file_info.truncated,
            "truncation_reason": file_info.truncation_reason(),
            "has_syntax_errors": !file_info.syntax_errors.is_empty(),
            "error_count": file_info.syntax_errors.len(),
            "token_count": file_info.token_count(),
            "summary_line_count": file_info.summary_line_count(),
            "processing_ratio": file_info.processing_ratio()
        });

        serde_json::to_string_pretty(&metadata).map_err(BatlessError::from)
    }

    /// Format statistics report
    pub fn format_stats_report(
        file_info: &FileInfo,
        file_path: &str,
        processing_time_ms: u128,
    ) -> String {
        let stats = file_info.get_stats_summary();

        format!(
            r#"File Processing Statistics
==========================
File: {}
Language: {}
Encoding: {}
Total Lines: {}
Processed Lines: {}
Total Bytes: {}
Processing Time: {}ms
Truncated: {}
Syntax Errors: {}
Tokens: {}
Summary Lines: {}
Processing Ratio: {:.2}%"#,
            file_path,
            stats.language.as_deref().unwrap_or("Unknown"),
            stats.encoding,
            stats.total_lines,
            stats.processed_lines,
            stats.total_bytes,
            processing_time_ms,
            if stats.truncated { "Yes" } else { "No" },
            stats.error_count,
            stats.token_count,
            stats.summary_line_count,
            file_info.processing_ratio() * 100.0
        )
    }

    /// Create a formatted table of multiple files
    pub fn format_file_table(file_results: &[(String, Result<FileInfo, BatlessError>)]) -> String {
        let mut table = Vec::new();

        // Header
        table.push(format!(
            "{:<30} {:<10} {:<8} {:<8} {:<12} {:<10}",
            "File", "Language", "Lines", "Bytes", "Status", "Truncated"
        ));
        table.push("-".repeat(80));

        // Rows
        for (file_path, result) in file_results {
            let row = match result {
                Ok(info) => format!(
                    "{:<30} {:<10} {:<8} {:<8} {:<12} {:<10}",
                    Self::truncate_path(file_path, 30),
                    info.language.as_deref().unwrap_or("Unknown"),
                    info.total_lines,
                    info.total_bytes,
                    if info.is_success() {
                        "Success"
                    } else {
                        "Errors"
                    },
                    if info.truncated { "Yes" } else { "No" }
                ),
                Err(_error) => format!(
                    "{:<30} {:<10} {:<8} {:<8} {:<12} {:<10}",
                    Self::truncate_path(file_path, 30),
                    "-",
                    "-",
                    "-",
                    "Error",
                    "-"
                ),
            };
            table.push(row);
        }

        table.join("\n")
    }

    /// Truncate file path for display
    fn truncate_path(path: &str, max_length: usize) -> String {
        if path.len() <= max_length {
            path.to_string()
        } else {
            format!("...{}", &path[path.len() - (max_length - 3)..])
        }
    }
}

/// Output formatting modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputMode {
    Plain,
    Highlight,
    Json,
    Summary,
}

impl OutputMode {
    /// Parse output mode from string
    pub fn parse_mode(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "plain" => Ok(OutputMode::Plain),
            "highlight" => Ok(OutputMode::Highlight),
            "json" => Ok(OutputMode::Json),
            "summary" => Ok(OutputMode::Summary),
            _ => Err(format!("Unknown output mode: {s}")),
        }
    }

    /// Get all available output modes
    pub fn all() -> Vec<Self> {
        vec![
            OutputMode::Plain,
            OutputMode::Highlight,
            OutputMode::Json,
            OutputMode::Summary,
        ]
    }

    /// Get string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            OutputMode::Plain => "plain",
            OutputMode::Highlight => "highlight",
            OutputMode::Json => "json",
            OutputMode::Summary => "summary",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::BatlessConfig;
    use serde_json::Value;

    fn create_test_file_info() -> FileInfo {
        FileInfo::with_metadata(10, 256, Some("Rust".to_string()), "UTF-8".to_string())
            .with_lines(vec![
                "fn main() {".to_string(),
                "    println!(\"Hello\");".to_string(),
                "}".to_string(),
            ])
            .with_tokens(Some(vec!["fn".to_string(), "main".to_string()]))
    }

    #[test]
    fn test_format_plain() -> BatlessResult<()> {
        let file_info = create_test_file_info();
        let config = crate::config::BatlessConfig::new();
        let result = OutputFormatter::format_plain(&file_info, &config)?;

        assert_eq!(result, "fn main() {\n    println!(\"Hello\");\n}");
        Ok(())
    }

    #[test]
    fn test_format_json() -> BatlessResult<()> {
        let file_info = create_test_file_info();
        let config = BatlessConfig::default();
        let result = OutputFormatter::format_json(&file_info, "test.rs", &config)?;

        // Should be valid JSON
        let parsed: Value = serde_json::from_str(&result)?;
        assert!(parsed["file"].as_str().unwrap() == "test.rs");
        assert!(parsed["total_lines"].as_u64().unwrap() == 3);
        assert!(parsed["lines"].is_array());

        Ok(())
    }

    #[test]
    fn test_format_summary() -> BatlessResult<()> {
        let file_info = create_test_file_info();
        let config = BatlessConfig::default();
        let result = OutputFormatter::format_summary(&file_info, &config)?;

        assert!(result.contains("=== File Summary ==="));
        assert!(result.contains("Language: Rust"));
        assert!(result.contains("Total Lines: 10"));

        Ok(())
    }

    #[test]
    fn test_format_compact_json() -> BatlessResult<()> {
        let file_info = create_test_file_info();
        let result = OutputFormatter::format_compact_json(&file_info, "test.rs")?;

        // Should be valid JSON but more compact
        let parsed: Value = serde_json::from_str(&result)?;
        assert!(parsed["path"].as_str().unwrap() == "test.rs");
        assert!(parsed["lines"].as_u64().unwrap() == 10);

        Ok(())
    }

    #[test]
    fn test_format_error() {
        let error = BatlessError::FileNotFound {
            path: "test.txt".to_string(),
            suggestions: vec![],
        };

        let json_result = OutputFormatter::format_error(&error, "test.txt", OutputMode::Json);
        assert!(json_result.contains("\"error\": true"));
        assert!(json_result.contains("\"error_type\": \"file_not_found\""));

        let plain_result = OutputFormatter::format_error(&error, "test.txt", OutputMode::Plain);
        assert!(plain_result.contains("Error processing test.txt"));
    }

    #[test]
    fn test_output_mode_parsing() {
        assert_eq!(OutputMode::parse_mode("plain").unwrap(), OutputMode::Plain);
        assert_eq!(OutputMode::parse_mode("json").unwrap(), OutputMode::Json);
        assert_eq!(
            OutputMode::parse_mode("HIGHLIGHT").unwrap(),
            OutputMode::Highlight
        );
        assert!(OutputMode::parse_mode("invalid").is_err());
    }

    #[test]
    fn test_output_mode_string_conversion() {
        assert_eq!(OutputMode::Plain.as_str(), "plain");
        assert_eq!(OutputMode::Json.as_str(), "json");
        assert_eq!(OutputMode::Highlight.as_str(), "highlight");
        assert_eq!(OutputMode::Summary.as_str(), "summary");
    }

    #[test]
    fn test_format_metadata_only() -> BatlessResult<()> {
        let file_info = create_test_file_info();
        let result = OutputFormatter::format_metadata_only(&file_info, "test.rs")?;

        let parsed: Value = serde_json::from_str(&result)?;
        assert!(parsed["file_path"].as_str().unwrap() == "test.rs");
        assert!(parsed["total_lines"].as_u64().unwrap() == 10);
        // Should not contain actual content
        assert!(parsed["content"].is_null());

        Ok(())
    }

    #[test]
    fn test_format_stats_report() {
        let file_info = create_test_file_info();
        let result = OutputFormatter::format_stats_report(&file_info, "test.rs", 42);

        assert!(result.contains("File Processing Statistics"));
        assert!(result.contains("File: test.rs"));
        assert!(result.contains("Processing Time: 42ms"));
        assert!(result.contains("Language: Rust"));
    }

    #[test]
    fn test_format_file_table() {
        let file_info = create_test_file_info();
        let results = vec![
            ("test.rs".to_string(), Ok(file_info)),
            (
                "error.txt".to_string(),
                Err(BatlessError::FileNotFound {
                    path: "error.txt".to_string(),
                    suggestions: vec![],
                }),
            ),
        ];

        let table = OutputFormatter::format_file_table(&results);
        assert!(table.contains("File"));
        assert!(table.contains("Language"));
        assert!(table.contains("test.rs"));
        assert!(table.contains("error.txt"));
        assert!(table.contains("Success"));
        assert!(table.contains("Error"));
    }

    #[test]
    fn test_truncate_path() {
        assert_eq!(OutputFormatter::truncate_path("short.txt", 20), "short.txt");
        assert_eq!(
            OutputFormatter::truncate_path("very/long/path/to/file.txt", 15),
            ".../to/file.txt"
        );
    }

    #[test]
    fn test_error_type_names() {
        assert_eq!(
            OutputFormatter::error_type_name(&BatlessError::FileNotFound {
                path: "test".to_string(),
                suggestions: vec![],
            }),
            "file_not_found"
        );
        assert_eq!(
            OutputFormatter::error_type_name(&BatlessError::ThemeNotFound {
                theme: "test".to_string(),
                suggestions: vec![],
            }),
            "theme_not_found"
        );
    }
}
