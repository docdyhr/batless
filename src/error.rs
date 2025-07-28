//! Custom error types for batless
//!
//! This module provides specific error types to improve error handling
//! and provide better error messages to users.

use std::fmt;
use std::path::Path;

/// Error codes for programmatic handling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    /// File system errors (100-199)
    FileNotFound = 101,
    FileReadError = 102,
    PermissionDenied = 103,
    EncodingError = 104,

    /// Syntax highlighting errors (200-299)
    HighlightError = 201,
    ThemeNotFound = 202,
    LanguageNotFound = 203,
    LanguageDetectionError = 204,

    /// Processing errors (300-399)
    ProcessingError = 301,
    ConfigurationError = 302,

    /// Output errors (400-499)
    JsonSerializationError = 401,
    OutputError = 402,

    /// Generic errors (500-599)
    IoError = 501,
}

impl ErrorCode {
    /// Get the error code as a string for display
    pub fn as_str(self) -> &'static str {
        match self {
            ErrorCode::FileNotFound => "E101",
            ErrorCode::FileReadError => "E102",
            ErrorCode::PermissionDenied => "E103",
            ErrorCode::EncodingError => "E104",
            ErrorCode::HighlightError => "E201",
            ErrorCode::ThemeNotFound => "E202",
            ErrorCode::LanguageNotFound => "E203",
            ErrorCode::LanguageDetectionError => "E204",
            ErrorCode::ProcessingError => "E301",
            ErrorCode::ConfigurationError => "E302",
            ErrorCode::JsonSerializationError => "E401",
            ErrorCode::OutputError => "E402",
            ErrorCode::IoError => "E501",
        }
    }
}

/// Main error type for batless operations
#[derive(Debug)]
pub enum BatlessError {
    /// File system related errors
    FileNotFound {
        path: String,
        suggestions: Vec<String>,
    },
    FileReadError {
        path: String,
        source: std::io::Error,
    },
    PermissionDenied {
        path: String,
        help: String,
    },

    /// Syntax highlighting errors
    HighlightError(String),
    ThemeNotFound {
        theme: String,
        suggestions: Vec<String>,
    },
    LanguageNotFound {
        language: String,
        suggestions: Vec<String>,
    },
    LanguageDetectionError(String),

    /// Processing errors
    EncodingError {
        path: String,
        details: String,
    },
    ProcessingError(String),
    ConfigurationError {
        message: String,
        help: Option<String>,
    },

    /// Output formatting errors
    JsonSerializationError(serde_json::Error),
    OutputError(String),

    /// Generic I/O errors
    IoError(std::io::Error),
}

impl fmt::Display for BatlessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_code = self.error_code();
        match self {
            BatlessError::FileNotFound { path, suggestions } => {
                write!(f, "[{}] File not found: {path}", error_code.as_str())?;
                if !suggestions.is_empty() {
                    write!(f, "\n\nDid you mean:")?;
                    for suggestion in suggestions.iter().take(3) {
                        write!(f, "\n  {suggestion}")?;
                    }
                }
                Ok(())
            }
            BatlessError::FileReadError { path, source } => {
                write!(
                    f,
                    "[{}] Failed to read file '{path}': {source}",
                    error_code.as_str()
                )
            }
            BatlessError::PermissionDenied { path, help } => {
                write!(
                    f,
                    "[{}] Permission denied: {path}\n\nHelp: {help}",
                    error_code.as_str()
                )
            }
            BatlessError::HighlightError(msg) => {
                write!(
                    f,
                    "[{}] Syntax highlighting failed: {msg}",
                    error_code.as_str()
                )
            }
            BatlessError::ThemeNotFound { theme, suggestions } => {
                write!(f, "[{}] Theme '{theme}' not found", error_code.as_str())?;
                if !suggestions.is_empty() {
                    write!(f, "\n\nDid you mean:")?;
                    for suggestion in suggestions.iter().take(3) {
                        write!(f, "\n  {suggestion}")?;
                    }
                }
                write!(f, "\n\nUse --list-themes to see all available themes")
            }
            BatlessError::LanguageNotFound {
                language,
                suggestions,
            } => {
                write!(
                    f,
                    "[{}] Language '{language}' not found",
                    error_code.as_str()
                )?;
                if !suggestions.is_empty() {
                    write!(f, "\n\nDid you mean:")?;
                    for suggestion in suggestions.iter().take(3) {
                        write!(f, "\n  {suggestion}")?;
                    }
                }
                write!(f, "\n\nUse --list-languages to see all available languages")
            }
            BatlessError::LanguageDetectionError(msg) => {
                write!(
                    f,
                    "[{}] Language detection failed: {msg}",
                    error_code.as_str()
                )
            }
            BatlessError::EncodingError { path, details } => {
                write!(
                    f,
                    "[{}] Encoding error in file '{path}': {details}",
                    error_code.as_str()
                )
            }
            BatlessError::ProcessingError(msg) => {
                write!(f, "[{}] Processing error: {msg}", error_code.as_str())
            }
            BatlessError::ConfigurationError { message, help } => {
                write!(
                    f,
                    "[{}] Configuration error: {message}",
                    error_code.as_str()
                )?;
                if let Some(help_text) = help {
                    write!(f, "\n\nHelp: {help_text}")?;
                }
                Ok(())
            }
            BatlessError::JsonSerializationError(err) => {
                write!(
                    f,
                    "[{}] JSON serialization failed: {err}",
                    error_code.as_str()
                )
            }
            BatlessError::OutputError(msg) => {
                write!(f, "[{}] Output error: {msg}", error_code.as_str())
            }
            BatlessError::IoError(err) => {
                write!(f, "[{}] I/O error: {err}", error_code.as_str())
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

impl BatlessError {
    /// Get the error code for this error
    pub fn error_code(&self) -> ErrorCode {
        match self {
            BatlessError::FileNotFound { .. } => ErrorCode::FileNotFound,
            BatlessError::FileReadError { .. } => ErrorCode::FileReadError,
            BatlessError::PermissionDenied { .. } => ErrorCode::PermissionDenied,
            BatlessError::HighlightError(_) => ErrorCode::HighlightError,
            BatlessError::ThemeNotFound { .. } => ErrorCode::ThemeNotFound,
            BatlessError::LanguageNotFound { .. } => ErrorCode::LanguageNotFound,
            BatlessError::LanguageDetectionError(_) => ErrorCode::LanguageDetectionError,
            BatlessError::EncodingError { .. } => ErrorCode::EncodingError,
            BatlessError::ProcessingError(_) => ErrorCode::ProcessingError,
            BatlessError::ConfigurationError { .. } => ErrorCode::ConfigurationError,
            BatlessError::JsonSerializationError(_) => ErrorCode::JsonSerializationError,
            BatlessError::OutputError(_) => ErrorCode::OutputError,
            BatlessError::IoError(_) => ErrorCode::IoError,
        }
    }

    /// Create a FileNotFound error with file suggestions
    pub fn file_not_found_with_suggestions(path: String) -> Self {
        let suggestions = Self::suggest_similar_files(&path);
        BatlessError::FileNotFound { path, suggestions }
    }

    /// Create a ThemeNotFound error with theme suggestions
    pub fn theme_not_found_with_suggestions(theme: String, available_themes: &[String]) -> Self {
        let suggestions = Self::suggest_similar_strings(&theme, available_themes);
        BatlessError::ThemeNotFound { theme, suggestions }
    }

    /// Create a LanguageNotFound error with language suggestions
    pub fn language_not_found_with_suggestions(
        language: String,
        available_languages: &[String],
    ) -> Self {
        let suggestions = Self::suggest_similar_strings(&language, available_languages);
        BatlessError::LanguageNotFound {
            language,
            suggestions,
        }
    }

    /// Create a PermissionDenied error with helpful suggestions
    pub fn permission_denied_with_help(path: String) -> Self {
        let help = if Path::new(&path).is_dir() {
            "The path points to a directory. Please specify a file instead.".to_string()
        } else {
            format!("Try running with appropriate permissions or check if the file exists:\n  ls -la {path}")
        };
        BatlessError::PermissionDenied { path, help }
    }

    /// Create a ConfigurationError with helpful suggestions
    pub fn config_error_with_help(message: String, help: Option<String>) -> Self {
        BatlessError::ConfigurationError { message, help }
    }

    /// Suggest similar files in the current directory
    fn suggest_similar_files(target: &str) -> Vec<String> {
        let target_path = Path::new(target);
        let dir = if target_path.parent().is_some()
            && !target_path.parent().unwrap().as_os_str().is_empty()
        {
            target_path.parent().unwrap()
        } else {
            Path::new(".")
        };
        let filename = target_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();

        if let Ok(entries) = std::fs::read_dir(dir) {
            let mut suggestions = Vec::new();
            for entry in entries.flatten() {
                if let Some(entry_name) = entry.file_name().to_str() {
                    if Self::is_similar(&filename, entry_name) {
                        let full_path = if dir == Path::new(".") {
                            entry_name.to_string()
                        } else {
                            dir.join(entry_name).to_string_lossy().to_string()
                        };
                        suggestions.push(full_path);
                    }
                }
            }
            suggestions.sort_by(|a, b| {
                Self::levenshtein_distance(
                    &filename,
                    Path::new(a).file_name().unwrap().to_str().unwrap(),
                )
                .cmp(&Self::levenshtein_distance(
                    &filename,
                    Path::new(b).file_name().unwrap().to_str().unwrap(),
                ))
            });
            suggestions.truncate(3);
            suggestions
        } else {
            Vec::new()
        }
    }

    /// Suggest similar strings from a list
    fn suggest_similar_strings(target: &str, candidates: &[String]) -> Vec<String> {
        let mut suggestions: Vec<_> = candidates
            .iter()
            .filter(|candidate| Self::is_similar(target, candidate))
            .cloned()
            .collect();

        suggestions.sort_by(|a, b| {
            Self::levenshtein_distance(target, a).cmp(&Self::levenshtein_distance(target, b))
        });
        suggestions.truncate(3);
        suggestions
    }

    /// Check if two strings are similar enough to suggest
    fn is_similar(a: &str, b: &str) -> bool {
        let distance = Self::levenshtein_distance(a, b);
        let max_len = a.len().max(b.len());
        if max_len == 0 {
            return false;
        }
        // Allow up to 2 character differences or 30% difference, whichever is larger
        let threshold = 2.max(max_len * 3 / 10);
        distance <= threshold
    }

    /// Calculate Levenshtein distance between two strings
    fn levenshtein_distance(a: &str, b: &str) -> usize {
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();
        let a_len = a_chars.len();
        let b_len = b_chars.len();

        let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];

        for (i, row) in matrix.iter_mut().enumerate().take(a_len + 1) {
            row[0] = i;
        }
        for j in 0..=b_len {
            matrix[0][j] = j;
        }

        for i in 1..=a_len {
            for j in 1..=b_len {
                let cost = if a_chars[i - 1] == b_chars[j - 1] {
                    0
                } else {
                    1
                };
                matrix[i][j] = (matrix[i - 1][j] + 1)
                    .min(matrix[i][j - 1] + 1)
                    .min(matrix[i - 1][j - 1] + cost);
            }
        }

        matrix[a_len][b_len]
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
        let error = BatlessError::FileNotFound {
            path: "test.rs".to_string(),
            suggestions: vec!["test.js".to_string(), "test.py".to_string()],
        };
        let display = error.to_string();
        assert!(display.contains("File not found: test.rs"));
        assert!(display.contains("Did you mean:"));
        assert!(display.contains("test.js"));

        let error = BatlessError::ThemeNotFound {
            theme: "invalid-theme".to_string(),
            suggestions: vec!["valid-theme".to_string()],
        };
        assert!(error
            .to_string()
            .contains("Theme 'invalid-theme' not found"));
    }

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(BatlessError::levenshtein_distance("test", "test"), 0);
        assert_eq!(BatlessError::levenshtein_distance("test", "rest"), 1);
        assert_eq!(BatlessError::levenshtein_distance("test", "best"), 1);
        assert_eq!(BatlessError::levenshtein_distance("test", ""), 4);
    }

    #[test]
    fn test_similarity_check() {
        assert!(BatlessError::is_similar("test.rs", "test.js"));
        assert!(BatlessError::is_similar("main", "mian"));
        assert!(!BatlessError::is_similar("test", "completely_different"));
    }

    #[test]
    fn test_file_suggestions() {
        use std::fs::File;
        use std::io::Write;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        // Create test files
        let mut file1 = File::create(temp_path.join("similar_file.rs")).unwrap();
        file1.write_all(b"fn main() {}").unwrap();

        let mut file2 = File::create(temp_path.join("similar_fiel.rs")).unwrap();
        file2.write_all(b"fn test() {}").unwrap();

        // Test suggestion for misspelled filename
        let target_path = temp_path.join("similar_fle.rs");
        let error = BatlessError::file_not_found_with_suggestions(
            target_path.to_string_lossy().to_string(),
        );

        if let BatlessError::FileNotFound { suggestions, .. } = error {
            assert!(
                !suggestions.is_empty(),
                "Should find suggestions for similar files"
            );
            assert!(suggestions
                .iter()
                .any(|s| s.contains("similar_file.rs") || s.contains("similar_fiel.rs")));
        } else {
            panic!("Expected FileNotFound error");
        }
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

    #[test]
    fn test_error_codes() {
        let error = BatlessError::FileNotFound {
            path: "test.rs".to_string(),
            suggestions: vec![],
        };
        assert_eq!(error.error_code(), ErrorCode::FileNotFound);
        assert_eq!(error.error_code().as_str(), "E101");

        let error = BatlessError::ThemeNotFound {
            theme: "invalid".to_string(),
            suggestions: vec![],
        };
        assert_eq!(error.error_code(), ErrorCode::ThemeNotFound);
        assert_eq!(error.error_code().as_str(), "E202");

        // Test that error codes are included in display output
        let display = error.to_string();
        assert!(display.contains("[E202]"));
        assert!(display.contains("Theme 'invalid' not found"));
    }
}
