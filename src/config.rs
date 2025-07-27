//! Configuration management for batless
//!
//! This module handles all configuration-related functionality including
//! default values, validation, and configuration parsing.

use crate::error::{BatlessError, BatlessResult};

/// Configuration structure for batless operations
#[derive(Clone, Debug)]
pub struct BatlessConfig {
    /// Maximum number of lines to process
    pub max_lines: usize,
    /// Maximum number of bytes to process (optional)
    pub max_bytes: Option<usize>,
    /// Override language detection with specific language
    pub language: Option<String>,
    /// Theme name for syntax highlighting
    pub theme: String,
    /// Whether to strip ANSI escape sequences
    pub strip_ansi: bool,
    /// Whether to use color output
    pub use_color: bool,
    /// Whether to include tokens in JSON output
    pub include_tokens: bool,
    /// Whether to enable summary mode
    pub summary_mode: bool,
}

impl Default for BatlessConfig {
    fn default() -> Self {
        Self {
            max_lines: 10000,
            max_bytes: None,
            language: None,
            theme: "base16-ocean.dark".to_string(),
            strip_ansi: false,
            use_color: true,
            include_tokens: false,
            summary_mode: false,
        }
    }
}

impl BatlessConfig {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set maximum lines
    pub fn with_max_lines(mut self, max_lines: usize) -> Self {
        self.max_lines = max_lines;
        self
    }

    /// Set maximum bytes
    pub fn with_max_bytes(mut self, max_bytes: Option<usize>) -> Self {
        self.max_bytes = max_bytes;
        self
    }

    /// Set language override
    pub fn with_language(mut self, language: Option<String>) -> Self {
        self.language = language;
        self
    }

    /// Set theme
    pub fn with_theme(mut self, theme: String) -> Self {
        self.theme = theme;
        self
    }

    /// Set ANSI stripping
    pub fn with_strip_ansi(mut self, strip_ansi: bool) -> Self {
        self.strip_ansi = strip_ansi;
        self
    }

    /// Set color usage
    pub fn with_use_color(mut self, use_color: bool) -> Self {
        self.use_color = use_color;
        self
    }

    /// Set token inclusion
    pub fn with_include_tokens(mut self, include_tokens: bool) -> Self {
        self.include_tokens = include_tokens;
        self
    }

    /// Set summary mode
    pub fn with_summary_mode(mut self, summary_mode: bool) -> Self {
        self.summary_mode = summary_mode;
        self
    }

    /// Validate the configuration
    pub fn validate(&self) -> BatlessResult<()> {
        if self.max_lines == 0 {
            return Err(BatlessError::ConfigurationError(
                "max_lines must be greater than 0".to_string(),
            ));
        }

        if let Some(max_bytes) = self.max_bytes {
            if max_bytes == 0 {
                return Err(BatlessError::ConfigurationError(
                    "max_bytes must be greater than 0".to_string(),
                ));
            }
        }

        if self.theme.is_empty() {
            return Err(BatlessError::ConfigurationError(
                "theme cannot be empty".to_string(),
            ));
        }

        Ok(())
    }

    /// Check if color output should be used based on configuration and environment
    pub fn should_use_color(&self, is_terminal: bool) -> bool {
        self.use_color && is_terminal
    }

    /// Get the effective maximum lines (considering both line and byte limits)
    pub fn effective_max_lines(&self) -> usize {
        self.max_lines
    }

    /// Check if byte limiting is enabled
    pub fn has_byte_limit(&self) -> bool {
        self.max_bytes.is_some()
    }

    /// Get byte limit if set
    pub fn get_byte_limit(&self) -> Option<usize> {
        self.max_bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = BatlessConfig::default();
        assert_eq!(config.max_lines, 10000);
        assert_eq!(config.max_bytes, None);
        assert_eq!(config.language, None);
        assert_eq!(config.theme, "base16-ocean.dark");
        assert!(!config.strip_ansi);
        assert!(config.use_color);
        assert!(!config.include_tokens);
        assert!(!config.summary_mode);
    }

    #[test]
    fn test_builder_pattern() {
        let config = BatlessConfig::new()
            .with_max_lines(5000)
            .with_max_bytes(Some(1024))
            .with_language(Some("rust".to_string()))
            .with_theme("monokai".to_string())
            .with_strip_ansi(true)
            .with_use_color(false)
            .with_include_tokens(true)
            .with_summary_mode(true);

        assert_eq!(config.max_lines, 5000);
        assert_eq!(config.max_bytes, Some(1024));
        assert_eq!(config.language, Some("rust".to_string()));
        assert_eq!(config.theme, "monokai");
        assert!(config.strip_ansi);
        assert!(!config.use_color);
        assert!(config.include_tokens);
        assert!(config.summary_mode);
    }

    #[test]
    fn test_validation_success() {
        let config = BatlessConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validation_zero_max_lines() {
        let config = BatlessConfig::default().with_max_lines(0);
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validation_zero_max_bytes() {
        let config = BatlessConfig::default().with_max_bytes(Some(0));
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validation_empty_theme() {
        let config = BatlessConfig::default().with_theme("".to_string());
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_should_use_color() {
        let config = BatlessConfig::default();
        assert!(config.should_use_color(true));
        assert!(!config.should_use_color(false));

        let config_no_color = config.with_use_color(false);
        assert!(!config_no_color.should_use_color(true));
        assert!(!config_no_color.should_use_color(false));
    }

    #[test]
    fn test_byte_limit_helpers() {
        let config = BatlessConfig::default();
        assert!(!config.has_byte_limit());
        assert_eq!(config.get_byte_limit(), None);

        let config_with_limit = config.with_max_bytes(Some(1024));
        assert!(config_with_limit.has_byte_limit());
        assert_eq!(config_with_limit.get_byte_limit(), Some(1024));
    }
}
