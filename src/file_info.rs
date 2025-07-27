//! File information structure for batless
//!
//! This module defines the FileInfo structure that holds all information
//! about a processed file, including content, metadata, and processing results.

use serde::{Deserialize, Serialize};

/// Information about a processed file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// The processed lines of the file
    pub lines: Vec<String>,
    /// Total number of lines in the original file
    pub total_lines: usize,
    /// Total number of bytes in the original file
    pub total_bytes: usize,
    /// Whether the file was truncated during processing
    pub truncated: bool,
    /// Whether truncation was due to line limit
    pub truncated_by_lines: bool,
    /// Whether truncation was due to byte limit
    pub truncated_by_bytes: bool,
    /// Detected or specified language
    pub language: Option<String>,
    /// Detected encoding of the file
    pub encoding: String,
    /// Syntax errors encountered during processing
    pub syntax_errors: Vec<String>,
    /// Extracted tokens (if requested)
    pub tokens: Option<Vec<String>>,
    /// Summary lines (if in summary mode)
    pub summary_lines: Option<Vec<String>>,
}

impl FileInfo {
    /// Create a new FileInfo instance
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            total_lines: 0,
            total_bytes: 0,
            truncated: false,
            truncated_by_lines: false,
            truncated_by_bytes: false,
            language: None,
            encoding: "UTF-8".to_string(),
            syntax_errors: Vec::new(),
            tokens: None,
            summary_lines: None,
        }
    }

    /// Create a FileInfo with basic metadata
    pub fn with_metadata(
        total_lines: usize,
        total_bytes: usize,
        language: Option<String>,
        encoding: String,
    ) -> Self {
        Self {
            lines: Vec::new(),
            total_lines,
            total_bytes,
            truncated: false,
            truncated_by_lines: false,
            truncated_by_bytes: false,
            language,
            encoding,
            syntax_errors: Vec::new(),
            tokens: None,
            summary_lines: None,
        }
    }

    /// Set the processed lines
    pub fn with_lines(mut self, lines: Vec<String>) -> Self {
        self.lines = lines;
        self
    }

    /// Set truncation information
    pub fn with_truncation(mut self, truncated: bool, by_lines: bool, by_bytes: bool) -> Self {
        self.truncated = truncated;
        self.truncated_by_lines = by_lines;
        self.truncated_by_bytes = by_bytes;
        self
    }

    /// Add a syntax error
    pub fn add_syntax_error(&mut self, error: String) {
        self.syntax_errors.push(error);
    }

    /// Set tokens
    pub fn with_tokens(mut self, tokens: Option<Vec<String>>) -> Self {
        self.tokens = tokens;
        self
    }

    /// Set summary lines
    pub fn with_summary_lines(mut self, summary_lines: Option<Vec<String>>) -> Self {
        self.summary_lines = summary_lines;
        self
    }

    /// Check if the file was processed successfully
    pub fn is_success(&self) -> bool {
        self.syntax_errors.is_empty()
    }

    /// Get the number of processed lines
    pub fn processed_lines(&self) -> usize {
        self.lines.len()
    }

    /// Get the processing ratio (processed lines / total lines)
    pub fn processing_ratio(&self) -> f64 {
        if self.total_lines == 0 {
            1.0
        } else {
            self.processed_lines() as f64 / self.total_lines as f64
        }
    }

    /// Check if any tokens were extracted
    pub fn has_tokens(&self) -> bool {
        self.tokens.as_ref().map_or(false, |t| !t.is_empty())
    }

    /// Check if summary was generated
    pub fn has_summary(&self) -> bool {
        self.summary_lines.as_ref().map_or(false, |s| !s.is_empty())
    }

    /// Get the number of tokens (if any)
    pub fn token_count(&self) -> usize {
        self.tokens.as_ref().map_or(0, |t| t.len())
    }

    /// Get the number of summary lines (if any)
    pub fn summary_line_count(&self) -> usize {
        self.summary_lines.as_ref().map_or(0, |s| s.len())
    }

    /// Get truncation reason as a human-readable string
    pub fn truncation_reason(&self) -> Option<String> {
        if !self.truncated {
            return None;
        }

        let mut reasons = Vec::new();
        if self.truncated_by_lines {
            reasons.push("line limit");
        }
        if self.truncated_by_bytes {
            reasons.push("byte limit");
        }

        if reasons.is_empty() {
            Some("unknown reason".to_string())
        } else {
            Some(reasons.join(" and "))
        }
    }

    /// Get a summary of processing statistics
    pub fn get_stats_summary(&self) -> ProcessingStats {
        ProcessingStats {
            total_lines: self.total_lines,
            processed_lines: self.processed_lines(),
            total_bytes: self.total_bytes,
            truncated: self.truncated,
            truncation_reason: self.truncation_reason(),
            has_syntax_errors: !self.syntax_errors.is_empty(),
            error_count: self.syntax_errors.len(),
            language: self.language.clone(),
            encoding: self.encoding.clone(),
            token_count: self.token_count(),
            summary_line_count: self.summary_line_count(),
        }
    }
}

impl Default for FileInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// Processing statistics summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingStats {
    pub total_lines: usize,
    pub processed_lines: usize,
    pub total_bytes: usize,
    pub truncated: bool,
    pub truncation_reason: Option<String>,
    pub has_syntax_errors: bool,
    pub error_count: usize,
    pub language: Option<String>,
    pub encoding: String,
    pub token_count: usize,
    pub summary_line_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_file_info() {
        let info = FileInfo::new();
        assert_eq!(info.lines.len(), 0);
        assert_eq!(info.total_lines, 0);
        assert_eq!(info.total_bytes, 0);
        assert!(!info.truncated);
        assert!(!info.truncated_by_lines);
        assert!(!info.truncated_by_bytes);
        assert_eq!(info.language, None);
        assert_eq!(info.encoding, "UTF-8");
        assert_eq!(info.syntax_errors.len(), 0);
        assert_eq!(info.tokens, None);
        assert_eq!(info.summary_lines, None);
    }

    #[test]
    fn test_with_metadata() {
        let info =
            FileInfo::with_metadata(100, 1024, Some("rust".to_string()), "UTF-8".to_string());
        assert_eq!(info.total_lines, 100);
        assert_eq!(info.total_bytes, 1024);
        assert_eq!(info.language, Some("rust".to_string()));
        assert_eq!(info.encoding, "UTF-8");
    }

    #[test]
    fn test_builder_pattern() {
        let lines = vec!["line1".to_string(), "line2".to_string()];
        let tokens = vec!["token1".to_string(), "token2".to_string()];
        let summary = vec!["fn main()".to_string()];

        let info = FileInfo::new()
            .with_lines(lines.clone())
            .with_truncation(true, true, false)
            .with_tokens(Some(tokens.clone()))
            .with_summary_lines(Some(summary.clone()));

        assert_eq!(info.lines, lines);
        assert!(info.truncated);
        assert!(info.truncated_by_lines);
        assert!(!info.truncated_by_bytes);
        assert_eq!(info.tokens, Some(tokens));
        assert_eq!(info.summary_lines, Some(summary));
    }

    #[test]
    fn test_processing_ratio() {
        let mut info = FileInfo::new();
        info.total_lines = 100;
        info.lines = vec!["line".to_string(); 50];

        assert_eq!(info.processing_ratio(), 0.5);

        // Test edge case with zero total lines
        info.total_lines = 0;
        assert_eq!(info.processing_ratio(), 1.0);
    }

    #[test]
    fn test_truncation_reason() {
        let mut info = FileInfo::new();
        assert_eq!(info.truncation_reason(), None);

        info.truncated = true;
        info.truncated_by_lines = true;
        assert_eq!(info.truncation_reason(), Some("line limit".to_string()));

        info.truncated_by_bytes = true;
        assert_eq!(
            info.truncation_reason(),
            Some("line limit and byte limit".to_string())
        );

        info.truncated_by_lines = false;
        assert_eq!(info.truncation_reason(), Some("byte limit".to_string()));
    }

    #[test]
    fn test_helper_methods() {
        let mut info = FileInfo::new();

        // Test success status
        assert!(info.is_success());
        info.add_syntax_error("test error".to_string());
        assert!(!info.is_success());

        // Test token and summary checks
        assert!(!info.has_tokens());
        assert!(!info.has_summary());
        assert_eq!(info.token_count(), 0);
        assert_eq!(info.summary_line_count(), 0);

        info.tokens = Some(vec!["token".to_string()]);
        info.summary_lines = Some(vec!["summary".to_string()]);

        assert!(info.has_tokens());
        assert!(info.has_summary());
        assert_eq!(info.token_count(), 1);
        assert_eq!(info.summary_line_count(), 1);
    }

    #[test]
    fn test_stats_summary() {
        let mut info =
            FileInfo::with_metadata(100, 2048, Some("rust".to_string()), "UTF-8".to_string());
        info.lines = vec!["line".to_string(); 50];
        info.truncated = true;
        info.truncated_by_lines = true;
        info.add_syntax_error("test error".to_string());
        info.tokens = Some(vec!["token1".to_string(), "token2".to_string()]);

        let stats = info.get_stats_summary();
        assert_eq!(stats.total_lines, 100);
        assert_eq!(stats.processed_lines, 50);
        assert_eq!(stats.total_bytes, 2048);
        assert!(stats.truncated);
        assert_eq!(stats.truncation_reason, Some("line limit".to_string()));
        assert!(stats.has_syntax_errors);
        assert_eq!(stats.error_count, 1);
        assert_eq!(stats.language, Some("rust".to_string()));
        assert_eq!(stats.encoding, "UTF-8");
        assert_eq!(stats.token_count, 2);
        assert_eq!(stats.summary_line_count, 0);
    }
}
