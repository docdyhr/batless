//! Custom profile configuration for batless
//!
//! This module provides custom AI profiles for personalized batless configurations,
//! allowing users to save and reuse their preferred settings.

use crate::config::BatlessConfig;
use crate::error::{BatlessError, BatlessResult};
use crate::summary::SummaryLevel;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

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
        if let Some(summary_level) = self.summary_level {
            config = config.with_summary_level(summary_level);
        }
        config
    }

    /// Get the preferred output mode for this profile
    pub fn get_output_mode(&self) -> Option<&str> {
        self.output_mode.as_deref()
    }

    /// Get the preferred AI model for this profile
    pub fn get_ai_model(&self) -> Option<&str> {
        self.ai_model.as_deref()
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_custom_profile_creation() {
        let profile = CustomProfile::new(
            "test-profile".to_string(),
            Some("Test description".to_string()),
        );
        assert_eq!(profile.name, "test-profile");
        assert_eq!(profile.description, Some("Test description".to_string()));
        assert_eq!(profile.version, "1.0");
    }

    #[test]
    fn test_custom_profile_validation() {
        let mut profile = CustomProfile::new("valid-name".to_string(), None);
        assert!(profile.validate().is_ok());

        profile.name = "".to_string();
        assert!(profile.validate().is_err());

        profile.name = "a".repeat(60);
        assert!(profile.validate().is_err());
    }

    #[test]
    fn test_custom_profile_serialization() -> BatlessResult<()> {
        let profile = CustomProfile::new(
            "test-profile".to_string(),
            Some("Test description".to_string()),
        );

        // Test TOML serialization
        let toml_str = toml::to_string_pretty(&profile).unwrap();
        let deserialized: CustomProfile = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized.name, profile.name);

        // Test JSON serialization
        let json_str = serde_json::to_string_pretty(&profile).unwrap();
        let deserialized: CustomProfile = serde_json::from_str(&json_str).unwrap();
        assert_eq!(deserialized.name, profile.name);

        Ok(())
    }

    #[test]
    fn test_custom_profile_file_operations() -> BatlessResult<()> {
        let profile = CustomProfile::new("file-test".to_string(), Some("File test".to_string()));

        // Test JSON file
        let temp_file = NamedTempFile::new().unwrap();
        let json_path = temp_file.path().with_extension("json");
        profile.save_to_file(&json_path)?;
        let loaded_profile = CustomProfile::load_from_file(&json_path)?;
        assert_eq!(loaded_profile.name, profile.name);

        // Test TOML file
        let toml_path = temp_file.path().with_extension("toml");
        profile.save_to_file(&toml_path)?;
        let loaded_profile = CustomProfile::load_from_file(&toml_path)?;
        assert_eq!(loaded_profile.name, profile.name);

        Ok(())
    }
}
