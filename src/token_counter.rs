//! Token counting for AI models
//!
//! This module provides token estimation for different AI models like GPT-4, Claude, etc.
//! Token counting is approximate and based on common patterns used by these models.

//! Token counting functionality for AI model context estimation
//!
//! This module provides token counting capabilities for various AI models,
//! allowing users to estimate how much content will fit within model context windows.
use serde::{Deserialize, Serialize};

/// Supported AI models for token counting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AiModel {
    /// OpenAI GPT-4 family (including GPT-4 Turbo)
    Gpt4,
    /// OpenAI GPT-4 Turbo with enhanced capabilities
    Gpt4Turbo,
    /// OpenAI GPT-3.5 family
    Gpt35,
    /// Anthropic Claude-3 family
    Claude,
    /// Anthropic Claude-3.5 Sonnet with enhanced capabilities
    Claude35Sonnet,
    /// Generic model using simple word-based estimation
    Generic,
}

impl AiModel {
    /// Parse model from string
    pub fn parse(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "gpt4" | "gpt-4" => Ok(Self::Gpt4),
            "gpt4-turbo" | "gpt-4-turbo" | "gpt4turbo" => Ok(Self::Gpt4Turbo),
            "gpt35" | "gpt-3.5" | "gpt-3.5-turbo" => Ok(Self::Gpt35),
            "claude" | "claude-3" => Ok(Self::Claude),
            "claude-3.5" | "claude-3.5-sonnet" | "claude35" | "claude35sonnet" => {
                Ok(Self::Claude35Sonnet)
            }
            "generic" => Ok(Self::Generic),
            _ => Err(format!("Unknown AI model: {s}")),
        }
    }

    /// Get string representation
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Gpt4 => "gpt-4",
            Self::Gpt4Turbo => "gpt-4-turbo",
            Self::Gpt35 => "gpt-3.5",
            Self::Claude => "claude",
            Self::Claude35Sonnet => "claude-3.5-sonnet",
            Self::Generic => "generic",
        }
    }

    /// Get approximate context window size for this model
    pub const fn context_window(&self) -> usize {
        match self {
            Self::Gpt4 => 128_000,           // GPT-4 Turbo
            Self::Gpt4Turbo => 128_000,      // GPT-4 Turbo (same as GPT-4)
            Self::Gpt35 => 16_384,           // GPT-3.5 Turbo
            Self::Claude => 200_000,         // Claude-3
            Self::Claude35Sonnet => 200_000, // Claude-3.5 Sonnet (same as Claude-3)
            Self::Generic => 8_192,          // Conservative default
        }
    }

    /// Get tokens per word ratio (approximate)
    fn tokens_per_word(&self) -> f64 {
        match self {
            Self::Gpt4 | Self::Gpt4Turbo | Self::Gpt35 => 1.3, // GPT models: ~1.3 tokens per word
            Self::Claude | Self::Claude35Sonnet => 1.2,        // Claude: slightly more efficient
            Self::Generic => 1.5,                              // Conservative estimate
        }
    }
}

/// Token counting statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenCount {
    /// Estimated token count
    pub tokens: usize,
    /// Number of words
    pub words: usize,
    /// Number of characters
    pub characters: usize,
    /// AI model used for estimation
    pub model: AiModel,
    /// Whether the content fits in the model's context window
    pub fits_in_context: bool,
    /// Percentage of context window used
    pub context_usage_percent: f64,
}

/// Token counter for AI models
pub struct TokenCounter {
    model: AiModel,
}

impl TokenCounter {
    /// Create a new token counter for the specified model
    pub const fn new(model: AiModel) -> Self {
        Self { model }
    }

    /// Get the AI model used by this counter
    pub const fn model(&self) -> AiModel {
        self.model
    }

    /// Count tokens in the given text with optimizations for large files
    pub fn count_tokens(&self, text: &str) -> TokenCount {
        let characters = text.chars().count();

        // For very large files (>100KB), use sampling for faster estimation
        let (words, tokens) = if characters > 100_000 {
            self.estimate_large_file_tokens(text, characters)
        } else {
            let words = self.count_words(text);
            let tokens = self.estimate_tokens(text, words);
            (words, tokens)
        };

        let context_window = self.model.context_window();

        TokenCount {
            tokens,
            words,
            characters,
            model: self.model,
            fits_in_context: tokens <= context_window,
            context_usage_percent: (tokens as f64 / context_window as f64) * 100.0,
        }
    }

    /// Optimized token estimation for large files using sampling
    fn estimate_large_file_tokens(&self, text: &str, total_chars: usize) -> (usize, usize) {
        // Sample multiple chunks from the file for better accuracy
        let sample_size = 10_000; // 10KB samples
        let num_samples = 5;

        let mut total_words = 0;
        let mut total_tokens = 0;
        let mut samples_taken = 0;

        let chunk_size = total_chars / num_samples;

        for i in 0..num_samples {
            let start = i * chunk_size;
            let end = if i == num_samples - 1 {
                total_chars
            } else {
                (start + sample_size).min(total_chars)
            };

            if start >= total_chars {
                break;
            }

            // Extract sample preserving word boundaries
            let sample = text
                .chars()
                .skip(start)
                .take(end - start)
                .collect::<String>();
            if !sample.is_empty() {
                let words = self.count_words(&sample);
                let tokens = self.estimate_tokens(&sample, words);

                total_words += words;
                total_tokens += tokens;
                samples_taken += 1;
            }
        }

        if samples_taken > 0 {
            // Scale up based on the ratio of sampled content to total content
            let sample_chars: usize = samples_taken * sample_size;
            let scale_factor = total_chars as f64 / sample_chars as f64;

            let estimated_words = (total_words as f64 * scale_factor) as usize;
            let estimated_tokens = (total_tokens as f64 * scale_factor) as usize;

            (estimated_words, estimated_tokens)
        } else {
            // Fallback for edge cases
            let words = self.count_words(text);
            let tokens = self.estimate_tokens(text, words);
            (words, tokens)
        }
    }

    /// Estimate if content will fit in context window with additional prompt
    pub fn fits_with_prompt(&self, content: &str, prompt_tokens: usize) -> bool {
        let content_tokens = self.estimate_tokens(content, self.count_words(content));
        let total_tokens = content_tokens + prompt_tokens;
        total_tokens <= self.model.context_window()
    }

    /// Calculate how much content can fit in context window
    pub fn max_content_length(&self, prompt_tokens: usize) -> usize {
        let available_tokens = self.model.context_window().saturating_sub(prompt_tokens);
        // Convert back to approximate character count
        let chars_per_token = 1.0 / self.model.tokens_per_word() * 4.5; // ~4.5 chars per word
        (available_tokens as f64 * chars_per_token) as usize
    }

    /// Truncate content to fit within context window with prompt overhead
    pub fn truncate_to_fit(&self, content: &str, prompt_tokens: usize) -> (String, bool) {
        let content_tokens = self.estimate_tokens(content, self.count_words(content));
        let available_tokens = self.model.context_window().saturating_sub(prompt_tokens);

        if content_tokens <= available_tokens {
            return (content.to_string(), false);
        }

        // Calculate how much content we need to keep
        let ratio = available_tokens as f64 / content_tokens as f64;
        let target_chars = (content.chars().count() as f64 * ratio) as usize;

        // Truncate at word boundaries for better readability
        let truncated = self.smart_truncate(content, target_chars);
        (truncated, true)
    }

    /// Smart truncation that preserves word boundaries and structure
    fn smart_truncate(&self, content: &str, target_chars: usize) -> String {
        if content.chars().count() <= target_chars {
            return content.to_string();
        }

        let lines: Vec<&str> = content.lines().collect();
        let mut result = String::new();
        let mut char_count = 0;

        // Try to keep complete lines when possible
        for line in lines {
            let line_chars = line.chars().count() + 1; // +1 for newline
            if char_count + line_chars > target_chars {
                // If we can't fit the whole line, try to fit part of it at word boundary
                let remaining = target_chars.saturating_sub(char_count);
                if remaining > 20 {
                    // Only if we have reasonable space left
                    let truncated_line = self.truncate_at_word_boundary(line, remaining);
                    if !truncated_line.is_empty() {
                        result.push_str(&truncated_line);
                        result.push_str("...");
                    }
                }
                break;
            }
            result.push_str(line);
            result.push('\n');
            char_count += line_chars;
        }

        result.trim_end().to_string()
    }

    /// Truncate text at the nearest word boundary
    fn truncate_at_word_boundary(&self, text: &str, max_chars: usize) -> String {
        if text.chars().count() <= max_chars {
            return text.to_string();
        }

        let chars: Vec<char> = text.chars().collect();
        let mut end = max_chars.min(chars.len());

        // Look backwards for a word boundary
        while end > 0 && end < chars.len() {
            let ch = chars[end - 1];
            if ch.is_whitespace() || ch == ',' || ch == ';' || ch == '.' {
                // Skip the whitespace/punctuation character
                while end > 0 && chars[end - 1].is_whitespace() {
                    end -= 1;
                }
                break;
            }
            end -= 1;
        }

        // If we couldn't find a good boundary, just cut at max_chars
        if end == 0 {
            end = max_chars.min(chars.len());
        }

        chars[..end].iter().collect()
    }

    /// Count words using whitespace and punctuation splitting
    fn count_words(&self, text: &str) -> usize {
        text.split_whitespace()
            .flat_map(|word| {
                // Split on common punctuation to get more accurate word counts
                word.split(&['.', ',', ';', ':', '!', '?', '(', ')', '[', ']', '{', '}'])
            })
            .filter(|word| !word.is_empty())
            .count()
    }

    /// Estimate tokens based on model-specific patterns
    fn estimate_tokens(&self, text: &str, words: usize) -> usize {
        match self.model {
            AiModel::Gpt4 | AiModel::Gpt4Turbo | AiModel::Gpt35 => {
                self.estimate_gpt_tokens(text, words)
            }
            AiModel::Claude | AiModel::Claude35Sonnet => self.estimate_claude_tokens(text, words),
            AiModel::Generic => self.estimate_generic_tokens(words),
        }
    }

    /// GPT-style token estimation
    fn estimate_gpt_tokens(&self, text: &str, words: usize) -> usize {
        let base_tokens = (words as f64 * self.model.tokens_per_word()) as usize;

        // Adjust for code vs natural language
        let code_penalty = if self.looks_like_code(text) { 1.2 } else { 1.0 };

        // Account for special tokens (newlines, indentation, etc.)
        let newlines = text.matches('\n').count();
        let special_tokens = newlines / 2; // Rough estimate

        ((base_tokens as f64 * code_penalty) as usize) + special_tokens
    }

    /// Claude-style token estimation
    fn estimate_claude_tokens(&self, text: &str, words: usize) -> usize {
        let base_tokens = (words as f64 * self.model.tokens_per_word()) as usize;

        // Claude is generally more efficient with code
        let code_bonus = if self.looks_like_code(text) { 0.9 } else { 1.0 };

        (base_tokens as f64 * code_bonus) as usize
    }

    /// Generic token estimation
    fn estimate_generic_tokens(&self, words: usize) -> usize {
        (words as f64 * self.model.tokens_per_word()) as usize
    }

    /// Heuristic to detect if text looks like code
    fn looks_like_code(&self, text: &str) -> bool {
        let code_indicators = [
            "fn ",
            "function ",
            "class ",
            "import ",
            "def ",
            "#include",
            "pub ",
            "private ",
            "public ",
            "const ",
            "let ",
            "var ",
            "struct ",
            "impl ",
            "trait ",
            "interface ",
            "extends ",
            "{",
            "}",
            "(",
            ")",
            "[",
            "]",
            ";",
            "=>",
            "->",
            "::",
            "::",
        ];

        let total_chars = text.len();
        if total_chars == 0 {
            return false;
        }

        let code_char_count: usize = code_indicators
            .iter()
            .map(|indicator| text.matches(indicator).count() * indicator.len())
            .sum();

        // If more than 10% of characters are code indicators, consider it code
        (code_char_count as f64 / total_chars as f64) > 0.1
    }
}

/// Get default token counter based on AI profile
pub fn get_token_counter_for_profile(profile: &str) -> TokenCounter {
    let model = match profile.to_lowercase().as_str() {
        "claude" => AiModel::Claude,
        "copilot" | "chatgpt" => AiModel::Gpt4, // Most likely to use GPT-4
        "assistant" => AiModel::Generic,
        _ => AiModel::Generic,
    };
    TokenCounter::new(model)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_model_parsing() {
        assert_eq!(AiModel::parse("gpt-4").unwrap(), AiModel::Gpt4);
        assert_eq!(AiModel::parse("claude").unwrap(), AiModel::Claude);
        assert!(AiModel::parse("unknown").is_err());
    }

    #[test]
    fn test_context_windows() {
        assert_eq!(AiModel::Gpt4.context_window(), 128_000);
        assert_eq!(AiModel::Claude.context_window(), 200_000);
    }

    #[test]
    fn test_token_counting() {
        let counter = TokenCounter::new(AiModel::Gpt4);
        let text = "Hello world this is a test";
        let count = counter.count_tokens(text);

        assert!(count.tokens > 0);
        assert!(count.words > 0);
        assert!(count.characters > 0);
        assert_eq!(count.model, AiModel::Gpt4);
    }

    #[test]
    fn test_code_detection() {
        let counter = TokenCounter::new(AiModel::Gpt4);

        let code_text = "fn main() { println!('Hello, world!'); }";
        assert!(counter.looks_like_code(code_text));

        let natural_text = "This is just regular English text without any code.";
        assert!(!counter.looks_like_code(natural_text));
    }

    #[test]
    fn test_fits_with_prompt() {
        let counter = TokenCounter::new(AiModel::Generic);
        let small_text = "Hello world";

        assert!(counter.fits_with_prompt(small_text, 100));
        assert!(!counter.fits_with_prompt(&"word ".repeat(10000), 1000));
    }

    #[test]
    fn test_max_content_length() {
        let counter = TokenCounter::new(AiModel::Gpt35);
        let max_length = counter.max_content_length(1000);

        assert!(max_length > 0);
        assert!(max_length < counter.model.context_window() * 10); // Sanity check
    }

    #[test]
    fn test_truncate_to_fit() {
        let counter = TokenCounter::new(AiModel::Generic);
        let small_text = "Hello world";

        // Small text should not be truncated
        let (result, truncated) = counter.truncate_to_fit(small_text, 100);
        assert_eq!(result, small_text);
        assert!(!truncated);

        // Large text should be truncated
        let large_text = "word ".repeat(10000);
        let (result, truncated) = counter.truncate_to_fit(&large_text, 1000);
        assert!(truncated);
        assert!(result.len() < large_text.len());
    }

    #[test]
    fn test_smart_truncate() {
        let counter = TokenCounter::new(AiModel::Gpt4);
        let text = "Line one\nLine two\nLine three\nLine four";

        // Should preserve complete lines when possible
        let result = counter.smart_truncate(text, 20);
        assert!(result.lines().all(|line| text.contains(line)));
    }

    #[test]
    fn test_truncate_at_word_boundary() {
        let counter = TokenCounter::new(AiModel::Gpt4);
        let text = "This is a long sentence with many words";

        let result = counter.truncate_at_word_boundary(text, 15);
        assert!(result.len() <= 15);
        assert!(!result.ends_with(' ')); // Should not end with space

        // Should not break words in the middle
        if result.len() < text.len() {
            let last_char = result.chars().last().unwrap_or(' ');
            assert!(last_char.is_alphabetic() || last_char.is_numeric());
        }
    }

    #[test]
    fn test_large_file_optimization() {
        let counter = TokenCounter::new(AiModel::Gpt4);

        // Create a large text (>100KB) to trigger optimization
        let base_text = "This is a test sentence with several words. ";
        let large_text = base_text.repeat(3000); // ~135KB

        // Ensure it's large enough to trigger optimization
        assert!(large_text.len() > 100_000);

        let count = counter.count_tokens(&large_text);

        // Should produce reasonable estimates
        assert!(count.tokens > 0);
        assert!(count.words > 0);
        assert!(count.characters > 100_000);
        assert_eq!(count.model, AiModel::Gpt4);

        // Verify the estimation is in a reasonable range
        // (not exact since it's using sampling)
        let expected_words = large_text.split_whitespace().count();
        let word_ratio = count.words as f64 / expected_words as f64;

        // Debug output for troubleshooting
        println!(
            "Expected words: {}, Estimated words: {}, Ratio: {:.2}",
            expected_words, count.words, word_ratio
        );

        // More lenient range for sampling-based estimation
        assert!(word_ratio > 0.5 && word_ratio < 2.0,
               "Word count estimation should be within reasonable range. Expected: {}, Got: {}, Ratio: {:.2}",
               expected_words, count.words, word_ratio);
    }

    #[test]
    fn test_new_ai_models() {
        // Test GPT-4 Turbo
        let gpt4_turbo = TokenCounter::new(AiModel::Gpt4Turbo);
        assert_eq!(gpt4_turbo.model(), AiModel::Gpt4Turbo);
        assert_eq!(gpt4_turbo.model().context_window(), 128_000);
        assert_eq!(gpt4_turbo.model().as_str(), "gpt-4-turbo");

        // Test Claude-3.5 Sonnet
        let claude35 = TokenCounter::new(AiModel::Claude35Sonnet);
        assert_eq!(claude35.model(), AiModel::Claude35Sonnet);
        assert_eq!(claude35.model().context_window(), 200_000);
        assert_eq!(claude35.model().as_str(), "claude-3.5-sonnet");

        // Test parsing
        assert_eq!(AiModel::parse("gpt-4-turbo").unwrap(), AiModel::Gpt4Turbo);
        assert_eq!(
            AiModel::parse("claude-3.5-sonnet").unwrap(),
            AiModel::Claude35Sonnet
        );
    }

    #[test]
    fn test_new_models_token_estimation() {
        let test_text = "fn main() {\n    println!(\"Hello, world!\");\n}";

        // Test GPT-4 Turbo token counting
        let gpt4_turbo = TokenCounter::new(AiModel::Gpt4Turbo);
        let gpt4_turbo_count = gpt4_turbo.count_tokens(test_text);
        assert!(gpt4_turbo_count.tokens > 0);
        assert_eq!(gpt4_turbo_count.model, AiModel::Gpt4Turbo);

        // Test Claude-3.5 Sonnet token counting
        let claude35 = TokenCounter::new(AiModel::Claude35Sonnet);
        let claude35_count = claude35.count_tokens(test_text);
        assert!(claude35_count.tokens > 0);
        assert_eq!(claude35_count.model, AiModel::Claude35Sonnet);

        // Compare with original models - should use similar estimation patterns
        let gpt4 = TokenCounter::new(AiModel::Gpt4);
        let gpt4_count = gpt4.count_tokens(test_text);

        let claude = TokenCounter::new(AiModel::Claude);
        let claude_count = claude.count_tokens(test_text);

        // GPT-4 Turbo should estimate similarly to GPT-4
        assert_eq!(gpt4_turbo_count.tokens, gpt4_count.tokens);

        // Claude-3.5 Sonnet should estimate similarly to Claude
        assert_eq!(claude35_count.tokens, claude_count.tokens);
    }

    #[test]
    fn test_new_models_context_fitting() {
        let test_text = "word ".repeat(1000);

        // Test GPT-4 Turbo context fitting
        let gpt4_turbo = TokenCounter::new(AiModel::Gpt4Turbo);
        assert!(gpt4_turbo.fits_with_prompt(&test_text, 500));

        let max_length = gpt4_turbo.max_content_length(1000);
        assert!(max_length > 0);
        assert!(max_length < 128_000 * 10); // Sanity check

        // Test Claude-3.5 Sonnet context fitting
        let claude35 = TokenCounter::new(AiModel::Claude35Sonnet);
        assert!(claude35.fits_with_prompt(&test_text, 500));

        let max_length_claude = claude35.max_content_length(1000);
        assert!(max_length_claude > 0);
        assert!(max_length_claude < 200_000 * 10); // Sanity check

        // Claude should have larger max content length due to bigger context window
        assert!(max_length_claude > max_length);
    }
}
