//! Trait definitions for decoupling modules
//!
//! This module defines the core traits that establish contracts between modules,
//! enabling dependency inversion and better testability.

use crate::error::BatlessResult;
use crate::file_info::FileInfo;
use crate::summary::SummaryLevel;

/// Trait for language detection functionality
pub trait LanguageDetection {
    /// Detect language from file path
    fn detect_language_with_fallback(&self, file_path: &str) -> Option<String>;

    /// Detect language from content
    fn detect_from_content(&self, content: &str, file_path: Option<&str>) -> Option<String>;
}

/// Trait for summary extraction functionality
pub trait SummaryExtraction {
    /// Extract summary lines from content
    fn extract_summary(
        &self,
        lines: &[String],
        language: Option<&str>,
        level: SummaryLevel,
    ) -> Vec<String>;

    /// Check if a line is summary-worthy
    fn is_summary_worthy(&self, line: &str, language: Option<&str>, level: SummaryLevel) -> bool;
}

/// Trait for token extraction functionality
pub trait TokenExtraction {
    /// Extract tokens from content
    fn extract_tokens(&self, content: &str, file_path: &str) -> Vec<String>;

    /// Count tokens in content
    fn count_tokens(&self, content: &str) -> usize;
}

/// Trait for file processing operations
pub trait FileProcessing {
    /// Process a file and return FileInfo
    fn process_file(file_path: &str, config: &dyn ProcessingConfig) -> BatlessResult<FileInfo>;

    /// Process stdin input
    fn process_stdin(config: &dyn ProcessingConfig) -> BatlessResult<FileInfo>;

    /// Validate file access
    fn validate_file_access(file_path: &str) -> BatlessResult<()>;
}

/// Trait for configuration access (avoiding direct BatlessConfig dependency)
pub trait ProcessingConfig {
    fn max_lines(&self) -> usize;
    fn max_bytes(&self) -> Option<usize>;
    fn language(&self) -> Option<&str>;
    fn summary_mode(&self) -> bool;
    fn include_tokens(&self) -> bool;
}

/// Trait for encoding detection
pub trait EncodingDetection {
    fn detect_encoding(file_path: &str) -> BatlessResult<String>;
    fn is_likely_binary(file_path: &str) -> BatlessResult<bool>;
}

/// Factory trait for creating processors with injected dependencies
pub trait ProcessorFactory {
    type Processor: FileProcessing;

    fn create_processor(
        language_detector: Box<dyn LanguageDetection>,
        summary_extractor: Box<dyn SummaryExtraction>,
        token_extractor: Box<dyn TokenExtraction>,
    ) -> Self::Processor;
}
