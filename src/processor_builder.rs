//! Builder pattern for configurable file processing
//!
//! This module provides a more flexible and testable approach to file processing
//! by allowing injection of different implementations for language detection,
//! summary extraction, and token extraction.

use crate::config::BatlessConfig;
use crate::error::BatlessResult;
use crate::file_info::FileInfo;
use crate::language::LanguageDetector;
use crate::processor::FileProcessor;
use crate::summarizer::SummaryExtractor;
use crate::tokens::TokenExtractor;
use crate::traits::{LanguageDetection, SummaryExtraction, TokenExtraction};

/// Configurable file processor that supports dependency injection
pub struct ConfigurableProcessor {
    language_detector: Box<dyn LanguageDetection>,
    summary_extractor: Box<dyn SummaryExtraction>,
    token_extractor: Box<dyn TokenExtraction>,
}

impl ConfigurableProcessor {
    /// Create a new processor with default implementations
    pub fn new() -> Self {
        Self {
            language_detector: Box::new(LanguageDetector),
            summary_extractor: Box::new(SummaryExtractor),
            token_extractor: Box::new(TokenExtractor),
        }
    }

    /// Builder method to set custom language detector
    pub fn with_language_detector(mut self, detector: Box<dyn LanguageDetection>) -> Self {
        self.language_detector = detector;
        self
    }

    /// Builder method to set custom summary extractor
    pub fn with_summary_extractor(mut self, extractor: Box<dyn SummaryExtraction>) -> Self {
        self.summary_extractor = extractor;
        self
    }

    /// Builder method to set custom token extractor
    pub fn with_token_extractor(mut self, extractor: Box<dyn TokenExtraction>) -> Self {
        self.token_extractor = extractor;
        self
    }

    /// Process a file using the configured components
    pub fn process_file(&self, file_path: &str, config: &BatlessConfig) -> BatlessResult<FileInfo> {
        // For now, delegate to the existing processor but with configuration flexibility
        // In a future iteration, this could be fully reimplemented using the injected dependencies
        FileProcessor::process_file(file_path, config)
    }

    /// Process stdin using the configured components
    pub fn process_stdin(&self, config: &BatlessConfig) -> BatlessResult<FileInfo> {
        FileProcessor::process_stdin(config)
    }

    /// Get the language detector for external use
    pub fn language_detector(&self) -> &dyn LanguageDetection {
        self.language_detector.as_ref()
    }

    /// Get the summary extractor for external use
    pub fn summary_extractor(&self) -> &dyn SummaryExtraction {
        self.summary_extractor.as_ref()
    }

    /// Get the token extractor for external use
    pub fn token_extractor(&self) -> &dyn TokenExtraction {
        self.token_extractor.as_ref()
    }
}

impl Default for ConfigurableProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to create a processor with default components
pub fn create_default_processor() -> ConfigurableProcessor {
    ConfigurableProcessor::new()
}

/// Factory function for creating processors with specific configurations
pub fn create_processor_for_mode(mode: ProcessorMode) -> ConfigurableProcessor {
    match mode {
        ProcessorMode::Fast => ConfigurableProcessor::new()
            .with_summary_extractor(Box::new(FastSummaryExtractor))
            .with_token_extractor(Box::new(FastTokenExtractor)),
        ProcessorMode::Detailed => ConfigurableProcessor::new(),
        ProcessorMode::Testing => ConfigurableProcessor::new()
            .with_language_detector(Box::new(MockLanguageDetector))
            .with_summary_extractor(Box::new(MockSummaryExtractor))
            .with_token_extractor(Box::new(MockTokenExtractor)),
    }
}

/// Processing modes for different use cases
#[derive(Clone, Debug)]
pub enum ProcessorMode {
    /// Fast processing with minimal analysis
    Fast,
    /// Detailed processing with full analysis
    Detailed,
    /// Mock implementations for testing
    Testing,
}

// Mock implementations for testing

/// Mock language detector for testing
struct MockLanguageDetector;

impl LanguageDetection for MockLanguageDetector {
    fn detect_language_with_fallback(&self, _file_path: &str) -> Option<String> {
        Some("MockLanguage".to_string())
    }

    fn detect_from_content(&self, _content: &str, _file_path: Option<&str>) -> Option<String> {
        Some("MockLanguage".to_string())
    }
}

/// Mock summary extractor for testing
struct MockSummaryExtractor;

impl SummaryExtraction for MockSummaryExtractor {
    fn extract_summary(&self, lines: &[String], _language: Option<&str>) -> Vec<String> {
        lines.iter().take(3).cloned().collect()
    }

    fn is_summary_worthy(&self, _line: &str, _language: Option<&str>) -> bool {
        true
    }
}

/// Mock token extractor for testing
struct MockTokenExtractor;

impl TokenExtraction for MockTokenExtractor {
    fn extract_tokens(&self, content: &str, _file_path: &str) -> Vec<String> {
        content.split_whitespace().map(String::from).collect()
    }

    fn count_tokens(&self, content: &str) -> usize {
        content.split_whitespace().count()
    }
}

// Fast implementations for performance-critical scenarios

/// Fast summary extractor that only looks for basic patterns
struct FastSummaryExtractor;

impl SummaryExtraction for FastSummaryExtractor {
    fn extract_summary(&self, lines: &[String], _language: Option<&str>) -> Vec<String> {
        lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim();
                trimmed.starts_with("fn ") || trimmed.starts_with("class ") || trimmed.starts_with("import ")
            })
            .cloned()
            .collect()
    }

    fn is_summary_worthy(&self, line: &str, _language: Option<&str>) -> bool {
        let trimmed = line.trim();
        trimmed.starts_with("fn ") || trimmed.starts_with("class ") || trimmed.starts_with("import ")
    }
}

/// Fast token extractor that uses simple whitespace splitting
struct FastTokenExtractor;

impl TokenExtraction for FastTokenExtractor {
    fn extract_tokens(&self, content: &str, _file_path: &str) -> Vec<String> {
        content.split_whitespace().map(String::from).collect()
    }

    fn count_tokens(&self, content: &str) -> usize {
        content.split_whitespace().count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configurable_processor_creation() {
        let processor = ConfigurableProcessor::new();
        // Just verify we can create the processor - pointer comparison is complex with trait objects
        let _detector = processor.language_detector();
        let _extractor = processor.summary_extractor();
        let _tokenizer = processor.token_extractor();
    }

    #[test]
    fn test_processor_modes() {
        let _fast_processor = create_processor_for_mode(ProcessorMode::Fast);
        let _detailed_processor = create_processor_for_mode(ProcessorMode::Detailed);
        let _testing_processor = create_processor_for_mode(ProcessorMode::Testing);
    }

    #[test]
    fn test_mock_implementations() {
        let mock_detector = MockLanguageDetector;
        assert_eq!(mock_detector.detect_language_with_fallback("test.rs"), Some("MockLanguage".to_string()));

        let mock_extractor = MockSummaryExtractor;
        let lines = vec!["line1".to_string(), "line2".to_string(), "line3".to_string(), "line4".to_string()];
        let summary = mock_extractor.extract_summary(&lines, None);
        assert_eq!(summary.len(), 3);

        let mock_tokenizer = MockTokenExtractor;
        let tokens = mock_tokenizer.extract_tokens("hello world test", "test.txt");
        assert_eq!(tokens.len(), 3);
    }
}