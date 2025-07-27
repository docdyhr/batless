//! Custom error types for batless
//!
//! This module provides specific error types to improve error handling
//! and provide better error messages to users.

use std::fmt;

/// Main error type for batless operations
#[derive(Debug)]
pub enum BatlessError {
    /// File system related errors
    FileNotFound(String),
    FileReadError {
        path: String,
        source: std::io::Error,
    },

    /// Syntax highlighting errors
    HighlightError(String),
    ThemeNotFound(String),
    LanguageDetectionError(String),

    /// Processing errors
    EncodingError {
        path: String,
        details: String,
    },
    ProcessingError(String),
    ConfigurationError(String),

    /// Output formatting errors
    JsonSerializationError(serde_json::Error),
    OutputError(String),

    /// Generic I/O errors
    IoError(std::io::Error),
}

impl fmt::Display for BatlessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BatlessError::FileNotFound(path) => {
                write!(f, "File not found: {path}")
            }
            BatlessError::FileReadError { path, source } => {
                write!(f, "Failed to read file '{path}': {source}")
            }
            BatlessError::HighlightError(msg) => {
                write!(f, "Syntax highlighting failed: {msg}")
            }
            BatlessError::ThemeNotFound(theme) => {
                write!(
                    f,
                    "Theme '{theme}' not found. Use --list-themes to see available themes"
                )
            }
            BatlessError::LanguageDetectionError(msg) => {
                write!(f, "Language detection failed: {msg}")
            }
            BatlessError::EncodingError { path, details } => {
                write!(f, "Encoding error in file '{path}': {details}")
            }
            BatlessError::ProcessingError(msg) => {
                write!(f, "Processing error: {msg}")
            }
            BatlessError::ConfigurationError(msg) => {
                write!(f, "Configuration error: {msg}")
            }
            BatlessError::JsonSerializationError(err) => {
                write!(f, "JSON serialization failed: {err}")
            }
            BatlessError::OutputError(msg) => {
                write!(f, "Output error: {msg}")
            }
            BatlessError::IoError(err) => {
                write!(f, "I/O error: {err}")
            }
        }
    }
}

impl std::error::Error for BatlessError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BatlessError::FileReadError { source, .. } => Some(source),
            BatlessError::JsonSerializationError(err) => Some(err),
            BatlessError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for BatlessError {
    fn from(err: std::io::Error) -> Self {
        BatlessError::IoError(err)
    }
}

impl From<serde_json::Error> for BatlessError {
    fn from(err: serde_json::Error) -> Self {
        BatlessError::JsonSerializationError(err)
    }
}

/// Result type alias for batless operations
pub type BatlessResult<T> = Result<T, BatlessError>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_error_display() {
        let error = BatlessError::FileNotFound("test.rs".to_string());
        assert_eq!(error.to_string(), "File not found: test.rs");

        let error = BatlessError::ThemeNotFound("invalid-theme".to_string());
        assert!(error
            .to_string()
            .contains("Theme 'invalid-theme' not found"));
    }

    #[test]
    fn test_error_source() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let error = BatlessError::from(io_error);
        assert!(error.source().is_some());
    }

    #[test]
    fn test_json_error_conversion() {
        let json_error = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let error = BatlessError::from(json_error);
        assert!(matches!(error, BatlessError::JsonSerializationError(_)));
    }
}
