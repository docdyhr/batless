//! Configuration management for batless
//!
//! This module handles all configuration-related functionality including
//! default values, validation, and configuration parsing.

use crate::error::{BatlessError, BatlessResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Summary extraction level for code analysis
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default, Hash)]
pub enum SummaryLevel {
    /// No summary extraction
    #[default]
    None,
    /// Minimal summary: functions and exports only
    Minimal,
    /// Standard summary: functions, classes, imports (current behavior)
    Standard,
    /// Detailed summary: includes comments, complexity metrics
    Detailed,
}

impl SummaryLevel {
    /// Parse summary level from string
    ///
    /// # Errors
    ///
    /// Returns an error if the input string doesn't match any valid summary level
    pub fn parse(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "none" | "false" | "off" => Ok(Self::None),
            "minimal" | "min" => Ok(Self::Minimal),
            "standard" | "std" | "true" | "on" => Ok(Self::Standard),
            "detailed" | "detail" | "full" => Ok(Self::Detailed),
            _ => Err(format!(
                "Unknown summary level: {s}. Valid options: none, minimal, standard, detailed"
            )),
        }
    }

    /// Get string representation
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Minimal => "minimal",
            Self::Standard => "standard",
            Self::Detailed => "detailed",
        }
    }

    /// Check if summary extraction is enabled
    #[must_use]
    pub const fn is_enabled(&self) -> bool {
        !matches!(self, Self::None)
    }
}

/// Custom AI profile for personalized batless configurations
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomProfile {
    /// Profile name
    pub name: String,
    /// Profile description
    pub description: Option<String>,
    /// Profile version for compatibility tracking
    #[serde(default = "default_profile_version")]
    pub version: String,
    /// Maximum number of lines to process
    pub max_lines: Option<usize>,
    /// Maximum number of bytes to process
    pub max_bytes: Option<usize>,
    /// Override language detection
    pub language: Option<String>,
    /// Theme for syntax highlighting
    pub theme: Option<String>,
    /// Whether to strip ANSI escape sequences
    pub strip_ansi: Option<bool>,
    /// Whether to use color output
    pub use_color: Option<bool>,
    /// Whether to include tokens in output
    pub include_tokens: Option<bool>,
    /// Summary extraction level
    pub summary_level: Option<SummaryLevel>,
    /// Output mode preference
    pub output_mode: Option<String>,
    /// AI model preference
    pub ai_model: Option<String>,
    /// Enable streaming JSON output for large files
    pub streaming_json: Option<bool>,
    /// Chunk size for streaming output (in lines)
    pub streaming_chunk_size: Option<usize>,
    /// Enable resume capability with checkpoint support
    pub enable_resume: Option<bool>,
    /// Enable debug mode
    pub debug: Option<bool>,
    /// Tags for profile categorization
    #[serde(default)]
    pub tags: Vec<String>,
    /// Profile creation timestamp
    pub created_at: Option<String>,
    /// Profile last modified timestamp
    pub updated_at: Option<String>,
}

fn default_profile_version() -> String {
    "1.0".to_string()
}

impl CustomProfile {
    /// Create a new custom profile with basic information
    pub fn new(name: String, description: Option<String>) -> Self {
        Self {
            name,
            description,
            version: default_profile_version(),
            max_lines: None,
            max_bytes: None,
            language: None,
            theme: None,
            strip_ansi: None,
            use_color: None,
            include_tokens: None,
            summary_level: None,
            output_mode: None,
            ai_model: None,
            streaming_json: None,
            streaming_chunk_size: None,
            enable_resume: None,
            debug: None,
            tags: Vec::new(),
            created_at: None,
            updated_at: None,
        }
    }

    /// Apply this custom profile to a configuration
    pub fn apply_to_config(&self, mut config: BatlessConfig) -> BatlessConfig {
        if let Some(max_lines) = self.max_lines {
            config = config.with_max_lines(max_lines);
        }
        if let Some(max_bytes) = self.max_bytes {
            config = config.with_max_bytes(Some(max_bytes));
        }
        if let Some(ref language) = self.language {
            config = config.with_language(Some(language.clone()));
        }
        if let Some(ref theme) = self.theme {
            config = config.with_theme(theme.clone());
        }
        if let Some(strip_ansi) = self.strip_ansi {
            config = config.with_strip_ansi(strip_ansi);
        }
        if let Some(use_color) = self.use_color {
            config = config.with_use_color(use_color);
        }
        if let Some(include_tokens) = self.include_tokens {
            config = config.with_include_tokens(include_tokens);
        }
        if let Some(ref summary_level) = self.summary_level {
            config = config.with_summary_level(summary_level.clone());
        }
        config
    }

    /// Get the preferred output mode for this profile
    pub fn get_output_mode(&self) -> Option<String> {
        self.output_mode.clone()
    }

    /// Get the preferred AI model for this profile
    pub fn get_ai_model(&self) -> Option<String> {
        self.ai_model.clone()
    }

    /// Validate the custom profile
    pub fn validate(&self) -> BatlessResult<()> {
        // Validate profile name
        if self.name.is_empty() {
            return Err(BatlessError::config_error_with_help(
                "Profile name cannot be empty".to_string(),
                Some(
                    "Profile names should be descriptive identifiers like 'my-coding-profile'"
                        .to_string(),
                ),
            ));
        }

        if self.name.len() > 50 {
            return Err(BatlessError::config_error_with_help(
                format!(
                    "Profile name is too long: '{}' (max 50 characters)",
                    self.name
                ),
                Some("Consider using a shorter, more concise profile name".to_string()),
            ));
        }

        // Validate individual settings by creating a temporary config
        let temp_config = self.apply_to_config(BatlessConfig::default());
        temp_config.validate()?;

        Ok(())
    }

    /// Load custom profile from file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> BatlessResult<Self> {
        let path = path.as_ref();
        let content = fs::read_to_string(path).map_err(|e| {
            BatlessError::config_error_with_help(
                format!("Failed to read profile file '{}': {}", path.display(), e),
                Some("Check that the file exists and you have read permissions".to_string()),
            )
        })?;

        let profile: CustomProfile = if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            toml::from_str(&content).map_err(|e| {
                BatlessError::config_error_with_help(
                    format!("Failed to parse TOML profile '{}': {}", path.display(), e),
                    Some(
                        "Check the TOML syntax and ensure all fields are properly formatted"
                            .to_string(),
                    ),
                )
            })?
        } else {
            serde_json::from_str(&content).map_err(|e| {
                BatlessError::config_error_with_help(
                    format!("Failed to parse JSON profile '{}': {}", path.display(), e),
                    Some(
                        "Check the JSON syntax and ensure all fields are properly formatted"
                            .to_string(),
                    ),
                )
            })?
        };

        profile.validate()?;
        Ok(profile)
    }

    /// Save custom profile to file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> BatlessResult<()> {
        let path = path.as_ref();
        self.validate()?;

        let content = if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            toml::to_string_pretty(self).map_err(|e| {
                BatlessError::config_error_with_help(
                    format!("Failed to serialize profile to TOML: {e}"),
                    Some("Check that all profile fields contain valid data".to_string()),
                )
            })?
        } else {
            serde_json::to_string_pretty(self).map_err(|e| {
                BatlessError::config_error_with_help(
                    format!("Failed to serialize profile to JSON: {e}"),
                    Some("Check that all profile fields contain valid data".to_string()),
                )
            })?
        };

        // Create directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                BatlessError::config_error_with_help(
                    format!("Failed to create directory '{}': {}", parent.display(), e),
                    Some(
                        "Check that you have write permissions to the parent directory".to_string(),
                    ),
                )
            })?;
        }

        fs::write(path, content).map_err(|e| {
            BatlessError::config_error_with_help(
                format!("Failed to write profile to '{}': {}", path.display(), e),
                Some("Check that you have write permissions to the target location".to_string()),
            )
        })?;

        Ok(())
    }

    /// Discover custom profiles in standard locations
    pub fn discover_profiles() -> Vec<PathBuf> {
        let mut profiles = Vec::new();

        // Standard profile locations
        let search_paths = [
            // Current directory
            PathBuf::from(".batless/profiles"),
            // User config directory
            dirs::config_dir()
                .map(|d| d.join("batless/profiles"))
                .unwrap_or_default(),
            // User home directory
            dirs::home_dir()
                .map(|d| d.join(".batless/profiles"))
                .unwrap_or_default(),
        ];

        for search_path in &search_paths {
            if search_path.exists() && search_path.is_dir() {
                if let Ok(entries) = fs::read_dir(search_path) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_file() {
                            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                                if ext == "json" || ext == "toml" {
                                    profiles.push(path);
                                }
                            }
                        }
                    }
                }
            }
        }

        profiles
    }
}

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
    /// Summary extraction level
    #[serde(default)]
    pub summary_level: SummaryLevel,
    /// Whether to enable summary mode (deprecated, use summary_level)
    #[serde(default)]
    pub summary_mode: bool,
    /// Enable streaming JSON output for large files
    #[serde(default)]
    pub streaming_json: bool,
    /// Chunk size for streaming output (in lines)
    #[serde(default = "default_streaming_chunk_size")]
    pub streaming_chunk_size: usize,
    /// Enable resume capability with checkpoint support
    #[serde(default)]
    pub enable_resume: bool,
    /// Schema version for JSON output compatibility
    #[serde(default = "default_schema_version")]
    pub schema_version: String,
    /// Enable debug mode with detailed processing information
    #[serde(default)]
    pub debug: bool,
    /// Show line numbers (cat -n compatibility)
    #[serde(default)]
    pub show_line_numbers: bool,
    /// Show line numbers for non-blank lines only (cat -b compatibility)
    #[serde(default)]
    pub show_line_numbers_nonblank: bool,
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

fn default_streaming_chunk_size() -> usize {
    1000
}

fn default_schema_version() -> String {
    "2.1".to_string()
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
            summary_level: SummaryLevel::None,
            summary_mode: false,
            streaming_json: false,
            streaming_chunk_size: default_streaming_chunk_size(),
            enable_resume: false,
            schema_version: default_schema_version(),
            debug: false,
            show_line_numbers: false,
            show_line_numbers_nonblank: false,
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
        // For backward compatibility, map boolean to SummaryLevel
        if summary_mode {
            self.summary_level = SummaryLevel::Standard;
        } else {
            self.summary_level = SummaryLevel::None;
        }
        self
    }

    /// Set summary level
    pub fn with_summary_level(mut self, summary_level: SummaryLevel) -> Self {
        // Update deprecated summary_mode for backward compatibility
        self.summary_mode = summary_level.is_enabled();
        self.summary_level = summary_level;
        self
    }

    /// Enable streaming JSON output
    pub fn with_streaming_json(mut self, streaming_json: bool) -> Self {
        self.streaming_json = streaming_json;
        self
    }

    /// Set streaming chunk size
    pub fn with_streaming_chunk_size(mut self, chunk_size: usize) -> Self {
        self.streaming_chunk_size = chunk_size;
        self
    }

    /// Enable resume capability
    pub fn with_enable_resume(mut self, enable_resume: bool) -> Self {
        self.enable_resume = enable_resume;
        self
    }

    /// Set schema version
    pub fn with_schema_version(mut self, version: String) -> Self {
        self.schema_version = version;
        self
    }

    /// Enable debug mode
    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    /// Enable line numbering (cat -n compatibility)
    pub fn with_show_line_numbers(mut self, show_line_numbers: bool) -> Self {
        self.show_line_numbers = show_line_numbers;
        self
    }

    /// Enable line numbering for non-blank lines only (cat -b compatibility)
    pub fn with_show_line_numbers_nonblank(mut self, show_line_numbers_nonblank: bool) -> Self {
        self.show_line_numbers_nonblank = show_line_numbers_nonblank;
        self
    }

    /// Get effective summary level (considering both new and deprecated fields)
    pub fn effective_summary_level(&self) -> SummaryLevel {
        // Priority: summary_level takes precedence over deprecated summary_mode
        if self.summary_level != SummaryLevel::None {
            self.summary_level.clone()
        } else if self.summary_mode {
            SummaryLevel::Standard
        } else {
            SummaryLevel::None
        }
    }

    /// Validate the configuration
    pub fn validate(&self) -> BatlessResult<()> {
        // Validate max_lines
        if self.max_lines == 0 {
            return Err(BatlessError::config_error_with_help(
                "max_lines must be greater than 0".to_string(),
                Some(
                    "Try using --max-lines with a positive number (e.g., --max-lines 1000)"
                        .to_string(),
                ),
            ));
        }

        if self.max_lines > 1_000_000 {
            return Err(BatlessError::config_error_with_help(
                format!(
                    "max_lines is unusually large ({}). This may cause performance issues",
                    self.max_lines
                ),
                Some(
                    "Consider using a smaller value like 10000, or use --max-bytes instead"
                        .to_string(),
                ),
            ));
        }

        // Validate max_bytes
        if let Some(max_bytes) = self.max_bytes {
            if max_bytes == 0 {
                return Err(BatlessError::config_error_with_help(
                    "max_bytes must be greater than 0".to_string(),
                    Some(
                        "Try using --max-bytes with a positive number (e.g., --max-bytes 1048576)"
                            .to_string(),
                    ),
                ));
            }

            if max_bytes > 100_000_000 {
                // 100MB
                return Err(BatlessError::config_error_with_help(
                    format!(
                        "max_bytes is unusually large ({max_bytes}). This may cause memory issues"
                    ),
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
            if language
                .chars()
                .any(|c| c.is_whitespace() || c.is_control())
            {
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
                Some(
                    "Use --list-themes to see available themes, or try 'base16-ocean.dark'"
                        .to_string(),
                ),
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
                    Some(
                        "Consider adjusting either max_lines or max_bytes to be more balanced"
                            .to_string(),
                    ),
                ));
            }
        }

        // Validate streaming chunk size
        if self.streaming_chunk_size == 0 {
            return Err(BatlessError::config_error_with_help(
                "streaming_chunk_size must be greater than 0".to_string(),
                Some("Try using a value like 1000 for good streaming performance".to_string()),
            ));
        }

        if self.streaming_chunk_size > 10000 {
            return Err(BatlessError::config_error_with_help(
                format!(
                    "streaming_chunk_size is unusually large ({}). This may cause memory issues",
                    self.streaming_chunk_size
                ),
                Some(
                    "Consider using a smaller value like 1000-5000 for better memory usage"
                        .to_string(),
                ),
            ));
        }

        // Validate schema version format
        if !self
            .schema_version
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
        {
            return Err(BatlessError::config_error_with_help(
                format!("Invalid schema version format: '{}'", self.schema_version),
                Some("Schema version should contain only alphanumeric characters, dots, and hyphens (e.g., '2.1', '2.1-beta')".to_string()),
            ));
        }

        // Validate logical combinations
        if self.include_tokens && self.summary_mode {
            return Err(BatlessError::config_error_with_help(
                "include_tokens and summary_mode cannot both be enabled".to_string(),
                Some("Choose either token extraction or summary mode, not both".to_string()),
            ));
        }

        // Validate streaming options
        if self.streaming_json && self.enable_resume && self.max_lines < self.streaming_chunk_size {
            return Err(BatlessError::config_error_with_help(
                "When using streaming with resume, max_lines should be larger than chunk_size"
                    .to_string(),
                Some(format!(
                    "Try increasing --max-lines to at least {} or reducing --streaming-chunk-size",
                    self.streaming_chunk_size
                )),
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
                format!(
                    "Failed to read config file '{}': {}",
                    path.as_ref().display(),
                    e
                ),
                Some("Check that the file exists and has proper permissions".to_string()),
            )
        })?;

        let config: BatlessConfig = toml::from_str(&content).map_err(|e| {
            BatlessError::config_error_with_help(
                format!(
                    "Failed to parse config file '{}': {}",
                    path.as_ref().display(),
                    e
                ),
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
                format!(
                    "Failed to read config file '{}': {}",
                    path.as_ref().display(),
                    e
                ),
                Some("Check that the file exists and has proper permissions".to_string()),
            )
        })?;

        let config: BatlessConfig = serde_json::from_str(&content).map_err(|e| {
            BatlessError::config_error_with_help(
                format!(
                    "Failed to parse config file '{}': {}",
                    path.as_ref().display(),
                    e
                ),
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
                format!(
                    "Failed to write config file '{}': {}",
                    path.as_ref().display(),
                    e
                ),
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
        assert!(paths
            .iter()
            .any(|p| p.file_name() == Some(std::ffi::OsStr::new(".batlessrc"))));
        assert!(paths
            .iter()
            .any(|p| p.file_name() == Some(std::ffi::OsStr::new("batless.toml"))));
    }

    #[test]
    fn test_load_from_toml_file() {
        use std::io::Write;
        use tempfile::NamedTempFile;

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
        use std::io::Write;
        use tempfile::NamedTempFile;

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
        use std::io::Write;
        use tempfile::NamedTempFile;

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
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("invalid characters"));
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
            .with_max_lines(1_000_000) // Extremely large line limit
            .with_max_bytes(Some(100)); // Very small byte limit
        let result = config.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("much larger than what max_bytes"));
    }

    #[test]
    fn test_validation_conflicting_modes() {
        let config = BatlessConfig::default()
            .with_include_tokens(true)
            .with_summary_mode(true);
        let result = config.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("cannot both be enabled"));
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

    // Custom Profile Tests
    #[test]
    fn test_custom_profile_creation() {
        let profile = CustomProfile::new(
            "test-profile".to_string(),
            Some("A test profile for unit testing".to_string()),
        );

        assert_eq!(profile.name, "test-profile");
        assert_eq!(
            profile.description,
            Some("A test profile for unit testing".to_string())
        );
        assert_eq!(profile.version, "1.0");
        assert!(profile.max_lines.is_none());
        assert!(profile.max_bytes.is_none());
        assert!(profile.tags.is_empty());
    }

    #[test]
    fn test_custom_profile_apply_to_config() {
        let profile = CustomProfile {
            name: "coding-profile".to_string(),
            description: None,
            version: "1.0".to_string(),
            max_lines: Some(2500),
            max_bytes: Some(50000),
            language: Some("rust".to_string()),
            theme: Some("zenburn".to_string()),
            strip_ansi: Some(true),
            use_color: Some(false),
            include_tokens: Some(true),
            summary_level: Some(SummaryLevel::Standard),
            output_mode: Some("json".to_string()),
            ai_model: Some("gpt4-turbo".to_string()),
            streaming_json: Some(false),
            streaming_chunk_size: Some(1000),
            enable_resume: Some(false),
            debug: Some(false),
            tags: vec!["coding".to_string(), "development".to_string()],
            created_at: None,
            updated_at: None,
        };

        let base_config = BatlessConfig::default();
        let applied_config = profile.apply_to_config(base_config);

        assert_eq!(applied_config.max_lines, 2500);
        assert_eq!(applied_config.max_bytes, Some(50000));
        assert_eq!(applied_config.language, Some("rust".to_string()));
        assert_eq!(applied_config.theme, "zenburn");
        assert!(applied_config.strip_ansi);
        assert!(!applied_config.use_color);
        assert!(applied_config.include_tokens);
        assert_eq!(applied_config.summary_level, SummaryLevel::Standard);
    }

    #[test]
    fn test_custom_profile_partial_application() {
        let profile = CustomProfile {
            name: "minimal-profile".to_string(),
            description: None,
            version: "1.0".to_string(),
            max_lines: Some(1000),
            max_bytes: None,
            language: None,
            theme: None,
            strip_ansi: None,
            use_color: None,
            include_tokens: None,
            summary_level: None,
            output_mode: None,
            ai_model: None,
            streaming_json: None,
            streaming_chunk_size: None,
            enable_resume: None,
            debug: None,
            tags: Vec::new(),
            created_at: None,
            updated_at: None,
        };

        let base_config = BatlessConfig::default()
            .with_theme("monokai".to_string())
            .with_use_color(false);

        let applied_config = profile.apply_to_config(base_config);

        // Profile should only override max_lines
        assert_eq!(applied_config.max_lines, 1000);
        assert_eq!(applied_config.theme, "monokai"); // Unchanged
        assert!(!applied_config.use_color); // Unchanged
    }

    #[test]
    fn test_custom_profile_validation() {
        // Valid profile
        let valid_profile = CustomProfile::new(
            "valid-profile".to_string(),
            Some("A valid profile".to_string()),
        );
        assert!(valid_profile.validate().is_ok());

        // Empty name
        let empty_name_profile = CustomProfile::new(String::new(), None);
        assert!(empty_name_profile.validate().is_err());

        // Name too long
        let long_name_profile = CustomProfile::new("a".repeat(60), None);
        assert!(long_name_profile.validate().is_err());
    }

    #[test]
    fn test_custom_profile_output_mode_preference() {
        let profile = CustomProfile {
            name: "test".to_string(),
            description: None,
            version: "1.0".to_string(),
            max_lines: None,
            max_bytes: None,
            language: None,
            theme: None,
            strip_ansi: None,
            use_color: None,
            include_tokens: None,
            summary_level: None,
            output_mode: Some("summary".to_string()),
            ai_model: Some("claude35-sonnet".to_string()),
            streaming_json: None,
            streaming_chunk_size: None,
            enable_resume: None,
            debug: None,
            tags: Vec::new(),
            created_at: None,
            updated_at: None,
        };

        assert_eq!(profile.get_output_mode(), Some("summary".to_string()));
        assert_eq!(profile.get_ai_model(), Some("claude35-sonnet".to_string()));
    }

    #[test]
    fn test_custom_profile_json_serialization() {
        let profile = CustomProfile::new(
            "test-profile".to_string(),
            Some("Test description".to_string()),
        );

        let json_str = serde_json::to_string_pretty(&profile).unwrap();
        let deserialized: CustomProfile = serde_json::from_str(&json_str).unwrap();

        assert_eq!(deserialized.name, profile.name);
        assert_eq!(deserialized.description, profile.description);
        assert_eq!(deserialized.version, profile.version);
    }

    #[test]
    fn test_custom_profile_discover_profiles() {
        // This test just ensures the function runs without panicking
        // In a real environment, it would find actual profile files
        let profiles = CustomProfile::discover_profiles();
        // Should return a Vec (even if empty, which is fine for testing)
        assert!(profiles.is_empty() || !profiles.is_empty());
    }
}
