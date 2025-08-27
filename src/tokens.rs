//! Token processing functionality for batless
//!
//! This module provides comprehensive token processing capabilities including:
//! - AI model token counting and context management
//! - Source code semantic token extraction
//! - Token analysis and statistics
//!
//! The module is organized into two main sub-modules:
//! - `counting`: AI model-specific token counting and context window management
//! - `extraction`: Semantic token extraction from source code for analysis

pub mod counting;
pub mod extraction;

// Re-export commonly used types for convenience
pub use counting::{AiModel, TokenCount, TokenCounter};
pub use extraction::{TokenExtractor, TokenStats};

use crate::traits::TokenExtraction;

/// Get a token counter configured for a specific AI profile
pub fn get_token_counter_for_profile(profile: &str) -> TokenCounter {
    counting::get_token_counter_for_profile(profile)
}

impl TokenExtraction for TokenExtractor {
    fn extract_tokens(&self, content: &str, file_path: &str) -> Vec<String> {
        TokenExtractor::extract_tokens(content, file_path)
    }
    
    fn count_tokens(&self, content: &str) -> usize {
        TokenExtractor::extract_tokens(content, "").len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_token_module_exports() {
        // Test that we can access both counting and extraction functionality
        let counter = TokenCounter::new(AiModel::Generic);
        let count = counter.count_tokens("hello world");
        assert!(count.tokens > 0);

        let tokens = TokenExtractor::extract_tokens("fn main() {}", "test.rs");
        assert!(!tokens.is_empty());
    }

    #[test]
    fn test_profile_integration() {
        let counter = get_token_counter_for_profile("claude");
        assert_eq!(counter.model(), AiModel::Claude);
    }
}