//! Configuration management for batless
//!
//! This module handles all configuration-related functionality including
//! default values, validation, and configuration parsing.

use crate::error::{BatlessError, BatlessResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Configuration structure for batless operations
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BatlessConfig {
    /// Maximum number of lines to process
    #[serde(default = "default_max_lines")]
    pub max_lines: usize,
    /// Maximum number of bytes to process (optional)
    #[serde(default)]
    pub max_bytes: Option<usize>,
    /// Override language detection with specific language
    #[serde(default)]
    pub language: Option<String>,
    /// Theme name for syntax highlighting
    #[serde(default = "default_theme")]
    pub theme: String,
    /// Whether to strip ANSI escape sequences
    #[serde(default)]
    pub strip_ansi: bool,
    /// Whether to use color output
    #[serde(default = "default_use_color")]
    pub use_color: bool,
    /// Whether to include tokens in JSON output
    #[serde(default)]
    pub include_tokens: bool,
    /// Whether to enable summary mode
    #[serde(default)]
    pub summary_mode: bool,
}

fn default_max_lines() -> usize {
    10000
}

fn default_theme() -> String {
    "base16-ocean.dark".to_string()
}

fn default_use_color() -> bool {
    true
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
        // Validate max_lines
        if self.max_lines == 0 {
            return Err(BatlessError::config_error_with_help(
                "max_lines must be greater than 0".to_string(),
                Some("Try using --max-lines with a positive number (e.g., --max-lines 1000)".to_string()),
            ));
        }

        if self.max_lines > 1_000_000 {
            return Err(BatlessError::config_error_with_help(
                format!("max_lines is unusually large ({}). This may cause performance issues", self.max_lines),
                Some("Consider using a smaller value like 10000, or use --max-bytes instead".to_string()),
            ));
        }

        // Validate max_bytes
        if let Some(max_bytes) = self.max_bytes {
            if max_bytes == 0 {
                return Err(BatlessError::config_error_with_help(
                    "max_bytes must be greater than 0".to_string(),
                    Some("Try using --max-bytes with a positive number (e.g., --max-bytes 1048576)".to_string()),
                ));
            }

            if max_bytes > 100_000_000 {  // 100MB
                return Err(BatlessError::config_error_with_help(
                    format!("max_bytes is unusually large ({max_bytes}). This may cause memory issues"),
                    Some("Consider using a smaller value like 10485760 (10MB)".to_string()),
                ));
            }
        }

        // Validate language
        if let Some(ref language) = self.language {
            if language.is_empty() {
                return Err(BatlessError::config_error_with_help(
                    "language cannot be empty when specified".to_string(),
                    Some("Either remove the language setting or specify a valid language. Use --list-languages to see options".to_string()),
                ));
            }

            if language.len() > 50 {
                return Err(BatlessError::config_error_with_help(
                    format!("language name is too long: '{language}'"),
                    Some("Language names should be short identifiers like 'rust', 'python', or 'javascript'".to_string()),
                ));
            }

            // Check for obviously invalid characters
            if language.chars().any(|c| c.is_whitespace() || c.is_control()) {
                return Err(BatlessError::config_error_with_help(
                    format!("language name contains invalid characters: '{language}'"),
                    Some("Language names should contain only alphanumeric characters, hyphens, and underscores".to_string()),
                ));
            }
        }

        // Validate theme
        if self.theme.is_empty() {
            return Err(BatlessError::config_error_with_help(
                "theme cannot be empty".to_string(),
                Some("Use --list-themes to see available themes, or try 'base16-ocean.dark'".to_string()),
            ));
        }

        if self.theme.len() > 100 {
            return Err(BatlessError::config_error_with_help(
                format!("theme name is too long: '{}'", self.theme),
                Some("Theme names should be reasonable identifiers. Use --list-themes to see valid options".to_string()),
            ));
        }

        // Check for reasonable limits combination
        if let Some(max_bytes) = self.max_bytes {
            // Rough estimate: average line length of 20 characters (more conservative)
            // Only warn if the mismatch is really extreme (more than 100x difference)
            let estimated_lines_from_bytes = (max_bytes / 20).max(1); // At least 1 line
            if self.max_lines > estimated_lines_from_bytes * 100 {
                return Err(BatlessError::config_error_with_help(
                    format!(
                        "max_lines ({}) is much larger than what max_bytes ({}) would allow",
                        self.max_lines, max_bytes
                    ),
                    Some("Consider adjusting either max_lines or max_bytes to be more balanced".to_string()),
                ));
            }
        }

        // Validate logical combinations
        if self.include_tokens && self.summary_mode {
            return Err(BatlessError::config_error_with_help(
                "include_tokens and summary_mode cannot both be enabled".to_string(),
                Some("Choose either token extraction or summary mode, not both".to_string()),
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

    /// Load configuration from a TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> BatlessResult<Self> {
        let content = fs::read_to_string(path.as_ref()).map_err(|e| {
            BatlessError::config_error_with_help(
                format!("Failed to read config file '{}': {}", path.as_ref().display(), e),
                Some("Check that the file exists and has proper permissions".to_string()),
            )
        })?;

        let config: BatlessConfig = toml::from_str(&content).map_err(|e| {
            BatlessError::config_error_with_help(
                format!("Failed to parse config file '{}': {}", path.as_ref().display(), e),
                Some("Check the TOML syntax - use 'batless --help' for valid options".to_string()),
            )
        })?;

        config.validate()?;
        Ok(config)
    }

    /// Load configuration from JSON file (.batlessrc format)
    pub fn from_json_file<P: AsRef<Path>>(path: P) -> BatlessResult<Self> {
        let content = fs::read_to_string(path.as_ref()).map_err(|e| {
            BatlessError::config_error_with_help(
                format!("Failed to read config file '{}': {}", path.as_ref().display(), e),
                Some("Check that the file exists and has proper permissions".to_string()),
            )
        })?;

        let config: BatlessConfig = serde_json::from_str(&content).map_err(|e| {
            BatlessError::config_error_with_help(
                format!("Failed to parse config file '{}': {}", path.as_ref().display(), e),
                Some("Check the JSON syntax - use 'batless --help' for valid options".to_string()),
            )
        })?;

        config.validate()?;
        Ok(config)
    }

    /// Save configuration to a TOML file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> BatlessResult<()> {
        let content = toml::to_string_pretty(self).map_err(|e| {
            BatlessError::config_error_with_help(
                format!("Failed to serialize config: {e}"),
                Some("This is likely a bug - please report it".to_string()),
            )
        })?;

        fs::write(path.as_ref(), content).map_err(|e| {
            BatlessError::config_error_with_help(
                format!("Failed to write config file '{}': {}", path.as_ref().display(), e),
                Some("Check that the directory exists and has write permissions".to_string()),
            )
        })
    }

    /// Find configuration files in standard locations
    /// Returns a list of config file paths in order of precedence (highest first)
    pub fn find_config_files() -> Vec<PathBuf> {
        let mut paths = Vec::new();

        // 1. Project-level config files (highest precedence)
        paths.push(PathBuf::from(".batlessrc"));
        paths.push(PathBuf::from("batless.toml"));

        // 2. User home directory config files
        if let Some(home_dir) = dirs::home_dir() {
            paths.push(home_dir.join(".batlessrc"));
            paths.push(home_dir.join(".config/batless/config.toml"));
            paths.push(home_dir.join(".config/batless.toml"));
        }

        // 3. System config directories (lowest precedence)
        if let Some(config_dir) = dirs::config_dir() {
            paths.push(config_dir.join("batless/config.toml"));
        }

        paths
    }

    /// Load configuration with precedence: CLI args > project config > user config > defaults
    pub fn load_with_precedence() -> BatlessResult<Self> {
        let mut config = Self::default();

        // Try to load from config files in reverse precedence order
        for config_path in Self::find_config_files().into_iter().rev() {
            if config_path.exists() {
                let file_config = if config_path.extension() == Some(std::ffi::OsStr::new("toml")) {
                    Self::from_file(&config_path)?
                } else {
                    Self::from_json_file(&config_path)?
                };
                config = config.merge_with(file_config);
            }
        }

        Ok(config)
    }

    /// Merge this configuration with another, taking non-default values from the other
    pub fn merge_with(mut self, other: Self) -> Self {
        let default = Self::default();

        // Only update if the other value is different from default
        if other.max_lines != default.max_lines {
            self.max_lines = other.max_lines;
        }
        if other.max_bytes != default.max_bytes {
            self.max_bytes = other.max_bytes;
        }
        if other.language != default.language {
            self.language = other.language;
        }
        if other.theme != default.theme {
            self.theme = other.theme;
        }
        if other.strip_ansi != default.strip_ansi {
            self.strip_ansi = other.strip_ansi;
        }
        if other.use_color != default.use_color {
            self.use_color = other.use_color;
        }
        if other.include_tokens != default.include_tokens {
            self.include_tokens = other.include_tokens;
        }
        if other.summary_mode != default.summary_mode {
            self.summary_mode = other.summary_mode;
        }

        self
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

    #[test]
    fn test_toml_serialization() {
        let config = BatlessConfig::default()
            .with_max_lines(5000)
            .with_theme("monokai".to_string());
        
        let toml_str = toml::to_string_pretty(&config).unwrap();
        assert!(toml_str.contains("max_lines = 5000"));
        assert!(toml_str.contains("theme = \"monokai\""));

        let deserialized: BatlessConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized.max_lines, 5000);
        assert_eq!(deserialized.theme, "monokai");
    }

    #[test]
    fn test_json_serialization() {
        let config = BatlessConfig::default()
            .with_max_lines(3000)
            .with_include_tokens(true);
        
        let json_str = serde_json::to_string_pretty(&config).unwrap();
        let deserialized: BatlessConfig = serde_json::from_str(&json_str).unwrap();
        assert_eq!(deserialized.max_lines, 3000);
        assert!(deserialized.include_tokens);
    }

    #[test]
    fn test_merge_with() {
        let base = BatlessConfig::default();
        let override_config = BatlessConfig::default()
            .with_max_lines(2000)
            .with_theme("solarized".to_string())
            .with_summary_mode(true);

        let merged = base.merge_with(override_config);
        assert_eq!(merged.max_lines, 2000);
        assert_eq!(merged.theme, "solarized");
        assert!(merged.summary_mode);
        // Other values should remain default
        assert!(!merged.strip_ansi);
        assert!(merged.use_color);
    }

    #[test]
    fn test_config_file_discovery() {
        let paths = BatlessConfig::find_config_files();
        assert!(!paths.is_empty());
        assert!(paths.iter().any(|p| p.file_name() == Some(std::ffi::OsStr::new(".batlessrc"))));
        assert!(paths.iter().any(|p| p.file_name() == Some(std::ffi::OsStr::new("batless.toml"))));
    }

    #[test]
    fn test_load_from_toml_file() {
        use tempfile::NamedTempFile;
        use std::io::Write;

        let toml_content = r#"
max_lines = 15000
theme = "zenburn"
use_color = false
summary_mode = true
"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(toml_content.as_bytes()).unwrap();

        let config = BatlessConfig::from_file(temp_file.path()).unwrap();
        assert_eq!(config.max_lines, 15000);
        assert_eq!(config.theme, "zenburn");
        assert!(!config.use_color);
        assert!(config.summary_mode);
    }

    #[test]
    fn test_load_from_json_file() {
        use tempfile::NamedTempFile;
        use std::io::Write;

        let json_content = r#"{
  "max_lines": 8000,
  "theme": "github",
  "include_tokens": true,
  "strip_ansi": true
}"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(json_content.as_bytes()).unwrap();

        let config = BatlessConfig::from_json_file(temp_file.path()).unwrap();
        assert_eq!(config.max_lines, 8000);
        assert_eq!(config.theme, "github");
        assert!(config.include_tokens);
        assert!(config.strip_ansi);
    }

    #[test]
    fn test_invalid_toml_config() {
        use tempfile::NamedTempFile;
        use std::io::Write;

        let invalid_toml = r#"
max_lines = "not_a_number"
"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(invalid_toml.as_bytes()).unwrap();

        let result = BatlessConfig::from_file(temp_file.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_save_to_file() {
        use tempfile::NamedTempFile;

        let config = BatlessConfig::default()
            .with_max_lines(7000)
            .with_theme("dracula".to_string());

        let temp_file = NamedTempFile::new().unwrap();
        config.save_to_file(temp_file.path()).unwrap();

        let loaded_config = BatlessConfig::from_file(temp_file.path()).unwrap();
        assert_eq!(loaded_config.max_lines, 7000);
        assert_eq!(loaded_config.theme, "dracula");
    }

    #[test]
    fn test_validation_large_max_lines() {
        let config = BatlessConfig::default().with_max_lines(2_000_000);
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("unusually large"));
    }

    #[test]
    fn test_validation_large_max_bytes() {
        let config = BatlessConfig::default().with_max_bytes(Some(200_000_000));
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("unusually large"));
    }

    #[test]
    fn test_validation_empty_language() {
        let config = BatlessConfig::default().with_language(Some("".to_string()));
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_validation_long_language() {
        let config = BatlessConfig::default().with_language(Some("a".repeat(60)));
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("too long"));
    }

    #[test]
    fn test_validation_invalid_language_chars() {
        let config = BatlessConfig::default().with_language(Some("rust lang".to_string()));
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("invalid characters"));
    }

    #[test]
    fn test_validation_long_theme() {
        let config = BatlessConfig::default().with_theme("a".repeat(150));
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("too long"));
    }

    #[test]
    fn test_validation_conflicting_limits() {
        let config = BatlessConfig::default()
            .with_max_lines(1_000_000)  // Extremely large line limit
            .with_max_bytes(Some(100));  // Very small byte limit
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("much larger than what max_bytes"));
    }

    #[test]
    fn test_validation_conflicting_modes() {
        let config = BatlessConfig::default()
            .with_include_tokens(true)
            .with_summary_mode(true);
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot both be enabled"));
    }

    #[test]
    fn test_validation_reasonable_config() {
        let config = BatlessConfig::default()
            .with_max_lines(5000)
            .with_max_bytes(Some(1_000_000))
            .with_language(Some("rust".to_string()))
            .with_theme("monokai".to_string());
        assert!(config.validate().is_ok());
    }
}
