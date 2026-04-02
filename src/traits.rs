//! Trait definitions for decoupling modules
//!
//! This module defines the core traits that establish contracts between modules,
//! enabling dependency inversion and better testability.

use crate::summary::SummaryLevel;
use crate::summary_item::SummaryItem;

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
    ) -> Vec<SummaryItem>;

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
