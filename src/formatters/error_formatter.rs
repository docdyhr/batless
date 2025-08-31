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
    fn error_type_name(error: &BatlessError) -> &'static str {
        match error {
            BatlessError::FileNotFound { .. } => "file not found",
            BatlessError::PermissionDenied { .. } => "permission denied",
            BatlessError::ConfigurationError { .. } => "configuration error",
            BatlessError::ProcessingError(_) => "processing error",
            BatlessError::IoError(_) => "I/O error",
            BatlessError::EncodingError { .. } => "encoding error",
            BatlessError::HighlightError(_) => "syntax highlighting error",
            BatlessError::FileReadError { .. } => "file read error",
            BatlessError::ThemeNotFound { .. } => "theme not found",
            BatlessError::LanguageNotFound { .. } => "language not found",
            BatlessError::LanguageDetectionError(_) => "language detection error",
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
}
