//! Error output formatting

use crate::error::BatlessError;
use crate::formatter::OutputMode;

/// Error formatter for consistent error display
pub struct ErrorFormatter;

impl ErrorFormatter {
    /// Format error with context information
    pub fn format_error(error: &BatlessError, file_path: &str, output_mode: OutputMode) -> String {
        let error_type = Self::error_type_name(error);

        match output_mode {
            OutputMode::Json => serde_json::json!({
                "error": true,
                "error_type": error_type,
                "message": error.to_string(),
                "file": file_path,
                "mode": "json"
            })
            .to_string(),
            _ => {
                format!("batless: {error_type}: {error}")
            }
        }
    }

    /// Get human-readable error type name
    const fn error_type_name(error: &BatlessError) -> &'static str {
        match error {
            BatlessError::FileNotFound { .. } => "file not found",
            BatlessError::PermissionDenied { .. } => "permission denied",
            BatlessError::ConfigurationError { .. } => "configuration error",
            BatlessError::ProcessingError { .. } => "processing error",
            BatlessError::IoError(_) => "I/O error",
            BatlessError::EncodingError { .. } => "encoding error",
            BatlessError::FileReadError { .. } => "file read error",
            BatlessError::LanguageNotFound { .. } => "language not found",
            BatlessError::LanguageDetectionError { .. } => "language detection error",
            BatlessError::JsonSerializationError(_) => "JSON serialization error",
            BatlessError::OutputError(_) => "output error",
        }
    }

    /// Format a simple error message
    pub fn format_simple(message: &str, file_path: &str) -> String {
        format!("batless: error: {message} (file: {file_path})")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_type_names() {
        let error = BatlessError::FileNotFound {
            path: "test.txt".to_string(),
            suggestions: vec![],
        };
        assert_eq!(ErrorFormatter::error_type_name(&error), "file not found");

        let error = BatlessError::PermissionDenied {
            path: "test.txt".to_string(),
            help: "Check permissions".to_string(),
        };
        assert_eq!(ErrorFormatter::error_type_name(&error), "permission denied");
    }

    #[test]
    fn test_simple_error_format() {
        let formatted = ErrorFormatter::format_simple("test error", "test.txt");
        assert!(formatted.contains("test error"));
        assert!(formatted.contains("test.txt"));
    }

    #[test]
    fn test_format_error_json_mode() {
        let error = BatlessError::FileNotFound {
            path: "missing.rs".to_string(),
            suggestions: vec![],
        };
        let formatted = ErrorFormatter::format_error(&error, "missing.rs", OutputMode::Json);
        let parsed: serde_json::Value = serde_json::from_str(&formatted).unwrap();

        assert_eq!(parsed["error"], true);
        assert_eq!(parsed["error_type"], "file not found");
        assert_eq!(parsed["file"], "missing.rs");
        assert_eq!(parsed["mode"], "json");
        assert!(parsed["message"].as_str().unwrap().contains("missing.rs"));
    }

    #[test]
    fn test_format_error_plain_mode() {
        let error = BatlessError::ConfigurationError {
            message: "bad config".to_string(),
            help: None,
        };
        let formatted = ErrorFormatter::format_error(&error, "config.toml", OutputMode::Plain);

        assert!(formatted.starts_with("batless: configuration error:"));
        assert!(formatted.contains("bad config"));
    }

    #[test]
    fn test_error_type_name_covers_all_variants() {
        let cases: Vec<(BatlessError, &str)> = vec![
            (
                BatlessError::FileReadError {
                    path: "a".to_string(),
                    source: std::io::Error::other("x"),
                },
                "file read error",
            ),
            (
                BatlessError::LanguageNotFound {
                    language: "brainfuck".to_string(),
                    suggestions: vec![],
                },
                "language not found",
            ),
            (
                BatlessError::LanguageDetectionError {
                    path: "a".to_string(),
                    details: "x".to_string(),
                },
                "language detection error",
            ),
            (
                BatlessError::EncodingError {
                    path: "a".to_string(),
                    details: "x".to_string(),
                },
                "encoding error",
            ),
            (
                BatlessError::ProcessingError {
                    message: "x".to_string(),
                    path: None,
                    help: None,
                },
                "processing error",
            ),
            (BatlessError::OutputError("x".to_string()), "output error"),
            (
                BatlessError::IoError(std::io::Error::other("x")),
                "I/O error",
            ),
        ];

        for (error, expected) in cases {
            assert_eq!(ErrorFormatter::error_type_name(&error), expected);
        }
    }
}
