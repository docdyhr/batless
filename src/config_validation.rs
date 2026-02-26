//! Configuration validation for batless
//!
//! This module contains all validation logic for `BatlessConfig`,
//! extracted from `config.rs` for better separation of concerns.

use crate::config::BatlessConfig;
use crate::error::{BatlessError, BatlessResult};

/// Validate a `BatlessConfig`, returning an error if any values are invalid.
pub fn validate_config(config: &BatlessConfig) -> BatlessResult<()> {
    validate_max_lines(config)?;
    validate_max_bytes(config)?;
    validate_language(config)?;
    validate_theme(config)?;
    validate_limits_combination(config)?;
    validate_streaming(config)?;
    validate_schema_version(config)?;
    validate_logical_combinations(config)?;
    Ok(())
}

fn validate_max_lines(config: &BatlessConfig) -> BatlessResult<()> {
    if config.max_lines == 0 {
        return Err(BatlessError::config_error_with_help(
            "validation failed: max_lines must be greater than 0".to_string(),
            Some(
                "Try using --max-lines with a positive number (e.g., --max-lines 1000)".to_string(),
            ),
        ));
    }

    if config.max_lines > 1_000_000 {
        return Err(BatlessError::config_error_with_help(
            format!(
                "max_lines is unusually large ({}). This may cause performance issues",
                config.max_lines
            ),
            Some(
                "Consider using a smaller value like 10000, or use --max-bytes instead".to_string(),
            ),
        ));
    }

    Ok(())
}

fn validate_max_bytes(config: &BatlessConfig) -> BatlessResult<()> {
    if let Some(max_bytes) = config.max_bytes {
        if max_bytes == 0 {
            return Err(BatlessError::config_error_with_help(
                "validation failed: max_bytes must be greater than 0".to_string(),
                Some(
                    "Try using --max-bytes with a positive number (e.g., --max-bytes 1048576)"
                        .to_string(),
                ),
            ));
        }

        if max_bytes > 100_000_000 {
            // 100MB
            return Err(BatlessError::config_error_with_help(
                format!("max_bytes is unusually large ({max_bytes}). This may cause memory issues"),
                Some("Consider using a smaller value like 10485760 (10MB)".to_string()),
            ));
        }
    }

    Ok(())
}

fn validate_language(config: &BatlessConfig) -> BatlessResult<()> {
    if let Some(ref language) = config.language {
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

    Ok(())
}

fn validate_theme(config: &BatlessConfig) -> BatlessResult<()> {
    if config.theme.is_empty() {
        return Err(BatlessError::config_error_with_help(
            "theme cannot be empty".to_string(),
            Some(
                "Use --list-themes to see available themes, or try 'base16-ocean.dark'".to_string(),
            ),
        ));
    }

    if config.theme.len() > 100 {
        return Err(BatlessError::config_error_with_help(
            format!("theme name is too long: '{}'", config.theme),
            Some("Theme names should be reasonable identifiers. Use --list-themes to see valid options".to_string()),
        ));
    }

    Ok(())
}

fn validate_limits_combination(config: &BatlessConfig) -> BatlessResult<()> {
    if let Some(max_bytes) = config.max_bytes {
        // Rough estimate: average line length of 20 characters (more conservative)
        // Only warn if the mismatch is really extreme (more than 100x difference)
        let estimated_lines_from_bytes = (max_bytes / 20).max(1); // At least 1 line
        if config.max_lines > estimated_lines_from_bytes * 100 {
            return Err(BatlessError::config_error_with_help(
                format!(
                    "max_lines ({}) is much larger than what max_bytes ({}) would allow",
                    config.max_lines, max_bytes
                ),
                Some(
                    "Consider adjusting either max_lines or max_bytes to be more balanced"
                        .to_string(),
                ),
            ));
        }
    }

    Ok(())
}

fn validate_streaming(config: &BatlessConfig) -> BatlessResult<()> {
    if config.streaming_chunk_size == 0 {
        return Err(BatlessError::config_error_with_help(
            "streaming_chunk_size must be greater than 0".to_string(),
            Some("Try using a value like 1000 for good streaming performance".to_string()),
        ));
    }

    if config.streaming_chunk_size > 10000 {
        return Err(BatlessError::config_error_with_help(
            format!(
                "streaming_chunk_size is unusually large ({}). This may cause memory issues",
                config.streaming_chunk_size
            ),
            Some(
                "Consider using a smaller value like 1000-5000 for better memory usage".to_string(),
            ),
        ));
    }

    // Validate streaming options combination
    if config.streaming_json
        && config.enable_resume
        && config.max_lines < config.streaming_chunk_size
    {
        return Err(BatlessError::config_error_with_help(
            "When using streaming with resume, max_lines should be larger than chunk_size"
                .to_string(),
            Some(format!(
                "Try increasing --max-lines to at least {} or reducing --streaming-chunk-size",
                config.streaming_chunk_size
            )),
        ));
    }

    Ok(())
}

fn validate_schema_version(config: &BatlessConfig) -> BatlessResult<()> {
    if !config
        .schema_version
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
    {
        return Err(BatlessError::config_error_with_help(
            format!("Invalid schema version format: '{}'", config.schema_version),
            Some("Schema version should contain only alphanumeric characters, dots, and hyphens (e.g., '2.1', '2.1-beta')".to_string()),
        ));
    }

    Ok(())
}

fn validate_logical_combinations(config: &BatlessConfig) -> BatlessResult<()> {
    if config.include_tokens && config.summary_mode {
        return Err(BatlessError::config_error_with_help(
            "include_tokens and summary_mode cannot both be enabled".to_string(),
            Some("Choose either token extraction or summary mode, not both".to_string()),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_success() {
        let config = BatlessConfig::default();
        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_validation_zero_max_lines() {
        let config = BatlessConfig::default().with_max_lines(0);
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_validation_zero_max_bytes() {
        let config = BatlessConfig::default().with_max_bytes(Some(0));
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_validation_empty_theme() {
        let config = BatlessConfig::default().with_theme(String::new());
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_validation_large_max_lines() {
        let config = BatlessConfig::default().with_max_lines(2_000_000);
        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("unusually large"));
    }

    #[test]
    fn test_validation_large_max_bytes() {
        let config = BatlessConfig::default().with_max_bytes(Some(200_000_000));
        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("unusually large"));
    }

    #[test]
    fn test_validation_empty_language() {
        let config = BatlessConfig::default().with_language(Some(String::new()));
        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_validation_long_language() {
        let config = BatlessConfig::default().with_language(Some("a".repeat(60)));
        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("too long"));
    }

    #[test]
    fn test_validation_invalid_language_chars() {
        let config = BatlessConfig::default().with_language(Some("rust lang".to_string()));
        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("invalid characters"));
    }

    #[test]
    fn test_validation_long_theme() {
        let config = BatlessConfig::default().with_theme("a".repeat(150));
        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("too long"));
    }

    #[test]
    fn test_validation_conflicting_limits() {
        let config = BatlessConfig::default()
            .with_max_lines(1_000_000) // Extremely large line limit
            .with_max_bytes(Some(100)); // Very small byte limit
        let result = validate_config(&config);
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
        let result = validate_config(&config);
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
        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_validation_zero_streaming_chunk_size() {
        let config = BatlessConfig {
            streaming_chunk_size: 0,
            ..BatlessConfig::default()
        };
        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("streaming_chunk_size must be greater than 0"));
    }

    #[test]
    fn test_validation_large_streaming_chunk_size() {
        let config = BatlessConfig {
            streaming_chunk_size: 20000,
            ..BatlessConfig::default()
        };
        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("unusually large"));
    }

    #[test]
    fn test_validation_invalid_schema_version() {
        let config = BatlessConfig {
            schema_version: "2.1@beta".to_string(),
            ..BatlessConfig::default()
        };
        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid schema version"));
    }

    #[test]
    fn test_validation_streaming_resume_mismatch() {
        let config = BatlessConfig::default()
            .with_streaming_json(true)
            .with_enable_resume(true)
            .with_max_lines(500)
            .with_streaming_chunk_size(1000);
        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("max_lines should be larger than chunk_size"));
    }
}
