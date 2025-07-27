//! Syntax highlighting functionality for batless
//!
//! This module handles syntax highlighting using syntect, with support for
//! various themes, languages, and output formats including terminal colors
//! and ANSI escape sequences.

use crate::config::BatlessConfig;
use crate::error::{BatlessError, BatlessResult};
use crate::language::{get_syntax_set, LanguageDetector, ThemeManager};
use std::path::Path;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, Theme};
use syntect::parsing::SyntaxReference;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

/// Syntax highlighter for source code
pub struct SyntaxHighlighter;

impl SyntaxHighlighter {
    /// Highlight content according to configuration
    pub fn highlight_content(
        content: &str,
        file_path: &str,
        config: &BatlessConfig,
    ) -> BatlessResult<String> {
        // Return plain text if color is disabled
        if !config.use_color {
            return Ok(content.to_string());
        }

        let theme = Self::get_theme(&config.theme)?;
        let syntax = Self::get_syntax(file_path, &config.language)?;

        Self::highlight_with_syntax_and_theme(content, syntax, theme)
    }

    /// Highlight content with specific syntax and theme
    pub fn highlight_with_syntax_and_theme(
        content: &str,
        syntax: &SyntaxReference,
        theme: &Theme,
    ) -> BatlessResult<String> {
        let mut highlighter = HighlightLines::new(syntax, theme);
        let mut result = String::new();

        for line in LinesWithEndings::from(content) {
            let ranges: Vec<(Style, &str)> = highlighter
                .highlight_line(line, get_syntax_set())
                .map_err(|e| BatlessError::HighlightError(e.to_string()))?;

            let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
            result.push_str(&escaped);
        }

        Ok(result)
    }

    /// Get theme reference from theme name
    fn get_theme(theme_name: &str) -> BatlessResult<&'static Theme> {
        // Validate theme exists
        ThemeManager::validate_theme(theme_name)?;

        ThemeManager::get_theme(theme_name).ok_or_else(|| {
            BatlessError::theme_not_found_with_suggestions(
                theme_name.to_string(),
                &ThemeManager::list_themes(),
            )
        })
    }

    /// Get syntax reference from file path and optional language override
    fn get_syntax(
        file_path: &str,
        language_override: &Option<String>,
    ) -> BatlessResult<&'static SyntaxReference> {
        let syntax_set = get_syntax_set();

        // Use language override if provided
        if let Some(lang) = language_override {
            // Validate language exists
            LanguageDetector::validate_language(lang)?;

            return syntax_set
                .find_syntax_by_name(lang)
                .or_else(|| syntax_set.find_syntax_by_extension(lang))
                .ok_or_else(|| {
                    BatlessError::language_not_found_with_suggestions(
                        lang.to_string(),
                        &LanguageDetector::list_languages(),
                    )
                });
        }

        // Auto-detect from file path
        let path = Path::new(file_path);
        Ok(syntax_set
            .find_syntax_for_file(path)
            .map_err(|e| BatlessError::LanguageDetectionError(e.to_string()))?
            .unwrap_or_else(|| syntax_set.find_syntax_plain_text()))
    }

    /// Highlight content and strip ANSI codes if requested
    pub fn highlight_and_process(
        content: &str,
        file_path: &str,
        config: &BatlessConfig,
    ) -> BatlessResult<String> {
        let highlighted = Self::highlight_content(content, file_path, config)?;

        if config.strip_ansi {
            Self::strip_ansi_codes(&highlighted)
        } else {
            Ok(highlighted)
        }
    }

    /// Strip ANSI escape sequences from highlighted content
    pub fn strip_ansi_codes(content: &str) -> BatlessResult<String> {
        let stripped = strip_ansi_escapes::strip(content);

        String::from_utf8(stripped).map_err(|e| {
            BatlessError::OutputError(format!("Invalid UTF-8 after stripping ANSI codes: {e}"))
        })
    }

    /// Get available color schemes/themes with descriptions
    pub fn get_theme_info() -> Vec<ThemeInfo> {
        let themes = ThemeManager::list_themes();
        let popular = ThemeManager::popular_themes();

        themes
            .into_iter()
            .map(|name| ThemeInfo {
                name: name.clone(),
                is_popular: popular.contains(&name),
                is_dark: Self::is_dark_theme(&name),
            })
            .collect()
    }

    /// Check if a theme is a dark theme (heuristic)
    fn is_dark_theme(theme_name: &str) -> bool {
        let name_lower = theme_name.to_lowercase();
        name_lower.contains("dark")
            || name_lower.contains("black")
            || name_lower.contains("night")
            || name_lower.contains("monokai")
            || name_lower.contains("ocean")
            || name_lower.contains("eighties")
            || name_lower.contains("mocha")
    }

    /// Preview a theme with sample code
    pub fn preview_theme(
        theme_name: &str,
        sample_code: &str,
        language: &str,
    ) -> BatlessResult<String> {
        ThemeManager::validate_theme(theme_name)?;
        LanguageDetector::validate_language(language)?;

        let theme = Self::get_theme(theme_name)?;
        let syntax = get_syntax_set()
            .find_syntax_by_name(language)
            .ok_or_else(|| {
                BatlessError::language_not_found_with_suggestions(
                    language.to_string(),
                    &LanguageDetector::list_languages(),
                )
            })?;

        Self::highlight_with_syntax_and_theme(sample_code, syntax, theme)
    }

    /// Check if terminal supports color output
    pub fn supports_color() -> bool {
        is_terminal::is_terminal(std::io::stderr())
            && std::env::var("NO_COLOR").is_err()
            && std::env::var("TERM").map_or(true, |term| term != "dumb")
    }

    /// Get the optimal color configuration for current environment
    pub fn get_optimal_color_config() -> bool {
        Self::supports_color()
    }

    /// Highlight a single line (useful for streaming)
    pub fn highlight_line(line: &str, highlighter: &mut HighlightLines) -> BatlessResult<String> {
        let ranges: Vec<(Style, &str)> = highlighter
            .highlight_line(line, get_syntax_set())
            .map_err(|e| BatlessError::HighlightError(e.to_string()))?;

        Ok(as_24_bit_terminal_escaped(&ranges[..], false))
    }

    /// Create a new highlighter for streaming usage
    pub fn create_highlighter(
        file_path: &str,
        config: &BatlessConfig,
    ) -> BatlessResult<HighlightLines<'static>> {
        let theme = Self::get_theme(&config.theme)?;
        let syntax = Self::get_syntax(file_path, &config.language)?;

        Ok(HighlightLines::new(syntax, theme))
    }
}

/// Information about a color theme
#[derive(Debug, Clone)]
pub struct ThemeInfo {
    pub name: String,
    pub is_popular: bool,
    pub is_dark: bool,
}

/// Highlight statistics
#[derive(Debug, Clone)]
pub struct HighlightStats {
    pub lines_processed: usize,
    pub total_chars: usize,
    pub theme_used: String,
    pub language_detected: Option<String>,
    pub processing_time_ms: u128,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::BatlessConfig;

    const SAMPLE_RUST_CODE: &str = r#"fn main() {
    println!("Hello, world!");
    let x = 42;
    let message = "test";
}"#;

    const SAMPLE_PYTHON_CODE: &str = r#"def main():
    print("Hello, world!")
    x = 42
    message = "test"
"#;

    #[test]
    fn test_highlight_content_plain() -> BatlessResult<()> {
        let config = BatlessConfig::default().with_use_color(false);
        let result = SyntaxHighlighter::highlight_content(SAMPLE_RUST_CODE, "test.rs", &config)?;
        assert_eq!(result, SAMPLE_RUST_CODE);
        Ok(())
    }

    #[test]
    fn test_highlight_content_with_color() -> BatlessResult<()> {
        let config = BatlessConfig::default().with_use_color(true);
        let result = SyntaxHighlighter::highlight_content(SAMPLE_RUST_CODE, "test.rs", &config)?;
        // Should contain ANSI escape sequences
        assert!(result.contains("\x1b["));
        assert!(result.len() > SAMPLE_RUST_CODE.len());
        Ok(())
    }

    #[test]
    fn test_invalid_theme() {
        let config = BatlessConfig::default().with_theme("nonexistent-theme".to_string());
        let result = SyntaxHighlighter::highlight_content(SAMPLE_RUST_CODE, "test.rs", &config);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            BatlessError::ThemeNotFound { .. }
        ));
    }

    #[test]
    fn test_language_override() -> BatlessResult<()> {
        let config = BatlessConfig::default()
            .with_language(Some("Python".to_string()))
            .with_use_color(true);

        let result =
            SyntaxHighlighter::highlight_content(SAMPLE_RUST_CODE, "test.unknown", &config)?;
        // Should be highlighted as Python despite the content being Rust
        assert!(result.contains("\x1b["));
        Ok(())
    }

    #[test]
    fn test_invalid_language_override() {
        let config =
            BatlessConfig::default().with_language(Some("NonExistentLanguage".to_string()));
        let result = SyntaxHighlighter::highlight_content(SAMPLE_RUST_CODE, "test.rs", &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_strip_ansi_codes() -> BatlessResult<()> {
        let ansi_text = "\x1b[31mRed text\x1b[0m";
        let stripped = SyntaxHighlighter::strip_ansi_codes(ansi_text)?;
        assert_eq!(stripped, "Red text");
        Ok(())
    }

    #[test]
    fn test_highlight_and_process_with_strip() -> BatlessResult<()> {
        let config = BatlessConfig::default()
            .with_use_color(true)
            .with_strip_ansi(true);

        let result =
            SyntaxHighlighter::highlight_and_process(SAMPLE_RUST_CODE, "test.rs", &config)?;
        // Should not contain ANSI escape sequences after stripping
        assert!(!result.contains("\x1b["));
        Ok(())
    }

    #[test]
    fn test_get_theme_info() {
        let themes = SyntaxHighlighter::get_theme_info();
        assert!(!themes.is_empty());

        // Check that we have some popular themes
        let popular_count = themes.iter().filter(|t| t.is_popular).count();
        assert!(popular_count > 0);

        // Check that we can classify dark themes
        let dark_count = themes.iter().filter(|t| t.is_dark).count();
        assert!(dark_count > 0);
    }

    #[test]
    fn test_is_dark_theme() {
        assert!(SyntaxHighlighter::is_dark_theme("base16-ocean.dark"));
        assert!(SyntaxHighlighter::is_dark_theme("Monokai"));
        assert!(!SyntaxHighlighter::is_dark_theme("InspiredGitHub"));
        assert!(!SyntaxHighlighter::is_dark_theme("Solarized (light)"));
    }

    #[test]
    fn test_preview_theme() -> BatlessResult<()> {
        let preview =
            SyntaxHighlighter::preview_theme("base16-ocean.dark", SAMPLE_RUST_CODE, "Rust")?;
        assert!(preview.contains("\x1b["));
        assert!(preview.len() > SAMPLE_RUST_CODE.len());
        Ok(())
    }

    #[test]
    fn test_preview_theme_invalid() {
        let result =
            SyntaxHighlighter::preview_theme("nonexistent-theme", SAMPLE_RUST_CODE, "Rust");
        assert!(result.is_err());
    }

    #[test]
    fn test_create_highlighter() -> BatlessResult<()> {
        let config = BatlessConfig::default();
        let highlighter = SyntaxHighlighter::create_highlighter("test.rs", &config)?;

        // Test that we can use the highlighter
        let mut hl = highlighter;
        let result = SyntaxHighlighter::highlight_line("fn main() {", &mut hl)?;
        assert!(result.contains("\x1b["));
        Ok(())
    }

    #[test]
    fn test_supports_color() {
        // This test will vary based on environment, but should not panic
        let _supports = SyntaxHighlighter::supports_color();
    }

    #[test]
    fn test_get_optimal_color_config() {
        // This test will vary based on environment, but should not panic
        let _optimal = SyntaxHighlighter::get_optimal_color_config();
    }

    #[test]
    fn test_auto_language_detection() -> BatlessResult<()> {
        let config = BatlessConfig::default().with_use_color(true);

        // Test Rust detection
        let result = SyntaxHighlighter::highlight_content(SAMPLE_RUST_CODE, "test.rs", &config)?;
        assert!(result.contains("\x1b["));

        // Test Python detection
        let result = SyntaxHighlighter::highlight_content(SAMPLE_PYTHON_CODE, "test.py", &config)?;
        assert!(result.contains("\x1b["));

        Ok(())
    }

    #[test]
    fn test_plain_text_fallback() -> BatlessResult<()> {
        let config = BatlessConfig::default().with_use_color(true);
        let plain_text = "This is just plain text without any syntax.";

        let result = SyntaxHighlighter::highlight_content(plain_text, "test.txt", &config)?;
        // Should work without errors even for plain text
        assert!(!result.is_empty());
        Ok(())
    }
}
