//! Language detection and management for batless
//!
//! This module handles language detection from file paths and extensions,
//! and provides utilities for listing available languages and themes.

use crate::error::{BatlessError, BatlessResult};
use lazy_static::lazy_static;
use std::path::Path;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

// Cache syntax and theme sets for better performance - loaded only when needed
lazy_static! {
    static ref SYNTAX_SET: SyntaxSet = SyntaxSet::load_defaults_newlines();
    static ref THEME_SET: ThemeSet = ThemeSet::load_defaults();
}

/// Language detection and theme management
pub struct LanguageDetector;

impl LanguageDetector {
    /// Detect the programming language from a file path
    pub fn detect_language(file_path: &str) -> Option<String> {
        let path = Path::new(file_path);

        if let Ok(Some(syntax)) = SYNTAX_SET.find_syntax_for_file(path) {
            Some(syntax.name.clone())
        } else {
            None
        }
    }

    /// Detect language with fallback to extension-based detection
    pub fn detect_language_with_fallback(file_path: &str) -> Option<String> {
        // First try syntect's built-in detection
        if let Some(lang) = Self::detect_language(file_path) {
            return Some(lang);
        }

        // Fallback to manual extension mapping for common cases
        let path = Path::new(file_path);
        if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
            Self::extension_to_language(extension)
        } else {
            None
        }
    }

    /// Map file extensions to language names for common cases not covered by syntect
    fn extension_to_language(extension: &str) -> Option<String> {
        match extension.to_lowercase().as_str() {
            "rs" => Some("Rust".to_string()),
            "py" => Some("Python".to_string()),
            "js" => Some("JavaScript".to_string()),
            "ts" => Some("TypeScript".to_string()),
            "go" => Some("Go".to_string()),
            "java" => Some("Java".to_string()),
            "cpp" | "cc" | "cxx" => Some("C++".to_string()),
            "c" => Some("C".to_string()),
            "h" | "hpp" => Some("C".to_string()),
            "rb" => Some("Ruby".to_string()),
            "php" => Some("PHP".to_string()),
            "swift" => Some("Swift".to_string()),
            "kt" => Some("Kotlin".to_string()),
            "scala" => Some("Scala".to_string()),
            "hs" => Some("Haskell".to_string()),
            "ml" => Some("OCaml".to_string()),
            "fs" => Some("F#".to_string()),
            "clj" => Some("Clojure".to_string()),
            "ex" | "exs" => Some("Elixir".to_string()),
            "erl" => Some("Erlang".to_string()),
            "dart" => Some("Dart".to_string()),
            "lua" => Some("Lua".to_string()),
            "pl" => Some("Perl".to_string()),
            "r" => Some("R".to_string()),
            "m" => Some("Objective-C".to_string()),
            "sh" | "bash" | "zsh" => Some("Bash".to_string()),
            "ps1" => Some("PowerShell".to_string()),
            "sql" => Some("SQL".to_string()),
            "json" => Some("JSON".to_string()),
            "xml" => Some("XML".to_string()),
            "html" => Some("HTML".to_string()),
            "css" => Some("CSS".to_string()),
            "scss" | "sass" => Some("SCSS".to_string()),
            "md" => Some("Markdown".to_string()),
            "yml" | "yaml" => Some("YAML".to_string()),
            "toml" => Some("TOML".to_string()),
            "ini" => Some("INI".to_string()),
            "dockerfile" => Some("Dockerfile".to_string()),
            "makefile" => Some("Makefile".to_string()),
            _ => None,
        }
    }

    /// Validate that a language name exists in the syntax set
    pub fn validate_language(language: &str) -> BatlessResult<()> {
        let languages = Self::list_languages();
        if languages.iter().any(|l| l.eq_ignore_ascii_case(language)) {
            Ok(())
        } else {
            Err(BatlessError::language_not_found_with_suggestions(
                language.to_string(),
                &languages,
            ))
        }
    }

    /// Get list of all available languages
    pub fn list_languages() -> Vec<String> {
        let mut languages: Vec<String> = SYNTAX_SET
            .syntaxes()
            .iter()
            .map(|syntax| syntax.name.clone())
            .collect();
        languages.sort();
        languages.dedup(); // Remove duplicates
        languages
    }

    /// Find a language by name (case-insensitive)
    pub fn find_language(name: &str) -> Option<String> {
        Self::list_languages()
            .into_iter()
            .find(|lang| lang.eq_ignore_ascii_case(name))
    }

    /// Get syntax reference for a language
    pub fn get_syntax_for_language(
        language: &str,
    ) -> Option<&'static syntect::parsing::SyntaxReference> {
        SYNTAX_SET.find_syntax_by_name(language)
    }

    /// Get syntax reference for a file path
    pub fn get_syntax_for_file(
        file_path: &str,
    ) -> Option<&'static syntect::parsing::SyntaxReference> {
        let path = Path::new(file_path);
        SYNTAX_SET.find_syntax_for_file(path).ok().flatten()
    }
}

/// Theme management utilities
pub struct ThemeManager;

impl ThemeManager {
    /// Get list of all available themes
    pub fn list_themes() -> Vec<String> {
        let mut themes: Vec<String> = THEME_SET.themes.keys().cloned().collect();
        themes.sort();
        themes
    }

    /// Validate that a theme exists
    pub fn validate_theme(theme_name: &str) -> BatlessResult<()> {
        if THEME_SET.themes.contains_key(theme_name) {
            Ok(())
        } else {
            let available_themes = Self::list_themes();
            Err(BatlessError::theme_not_found_with_suggestions(
                theme_name.to_string(),
                &available_themes,
            ))
        }
    }

    /// Find a theme by name (case-insensitive)
    pub fn find_theme(name: &str) -> Option<String> {
        Self::list_themes()
            .into_iter()
            .find(|theme| theme.eq_ignore_ascii_case(name))
    }

    /// Get the default theme name
    pub fn default_theme() -> &'static str {
        "base16-ocean.dark"
    }

    /// Get theme reference
    pub fn get_theme(theme_name: &str) -> Option<&'static syntect::highlighting::Theme> {
        THEME_SET.themes.get(theme_name)
    }

    /// Get a list of popular/recommended themes
    pub fn popular_themes() -> Vec<String> {
        vec![
            "base16-ocean.dark".to_string(),
            "base16-eighties.dark".to_string(),
            "base16-mocha.dark".to_string(),
            "Solarized (dark)".to_string(),
            "Solarized (light)".to_string(),
            "InspiredGitHub".to_string(),
        ]
    }
}

/// Get both syntax and theme sets (for use in other modules)
pub fn get_syntax_set() -> &'static SyntaxSet {
    &SYNTAX_SET
}

pub fn get_theme_set() -> &'static ThemeSet {
    &THEME_SET
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_language_rust() {
        let language = LanguageDetector::detect_language("test.rs");
        assert_eq!(language, Some("Rust".to_string()));
    }

    #[test]
    fn test_detect_language_python() {
        let language = LanguageDetector::detect_language("test.py");
        assert_eq!(language, Some("Python".to_string()));
    }

    #[test]
    fn test_detect_language_unknown() {
        let language = LanguageDetector::detect_language("test.unknown");
        assert_eq!(language, None);
    }

    #[test]
    fn test_detect_language_with_fallback() {
        // Test fallback for extensions that might not be in syntect
        let language = LanguageDetector::detect_language_with_fallback("test.rs");
        assert!(language.is_some());

        // Test unknown extension
        let language = LanguageDetector::detect_language_with_fallback("test.xyz");
        assert_eq!(language, None);
    }

    #[test]
    fn test_extension_to_language() {
        assert_eq!(
            LanguageDetector::extension_to_language("rs"),
            Some("Rust".to_string())
        );
        assert_eq!(
            LanguageDetector::extension_to_language("py"),
            Some("Python".to_string())
        );
        assert_eq!(LanguageDetector::extension_to_language("xyz"), None);
    }

    #[test]
    fn test_list_languages() {
        let languages = LanguageDetector::list_languages();
        assert!(!languages.is_empty());
        assert!(languages.contains(&"Rust".to_string()));

        // Check that list is sorted
        let mut sorted_languages = languages.clone();
        sorted_languages.sort();
        assert_eq!(languages, sorted_languages);
    }

    #[test]
    fn test_find_language_case_insensitive() {
        assert_eq!(
            LanguageDetector::find_language("rust"),
            Some("Rust".to_string())
        );
        assert_eq!(
            LanguageDetector::find_language("RUST"),
            Some("Rust".to_string())
        );
        assert_eq!(LanguageDetector::find_language("nonexistent"), None);
    }

    #[test]
    fn test_validate_language() {
        assert!(LanguageDetector::validate_language("Rust").is_ok());
        assert!(LanguageDetector::validate_language("NonExistent").is_err());
    }

    #[test]
    fn test_list_themes() {
        let themes = ThemeManager::list_themes();
        assert!(!themes.is_empty());
        assert!(themes.contains(&"base16-ocean.dark".to_string()));

        // Check that list is sorted
        let mut sorted_themes = themes.clone();
        sorted_themes.sort();
        assert_eq!(themes, sorted_themes);
    }

    #[test]
    fn test_validate_theme() {
        assert!(ThemeManager::validate_theme("base16-ocean.dark").is_ok());
        assert!(ThemeManager::validate_theme("nonexistent-theme").is_err());
    }

    #[test]
    fn test_find_theme_case_insensitive() {
        let themes = ThemeManager::list_themes();
        if let Some(first_theme) = themes.first() {
            let lower_case = first_theme.to_lowercase();
            let found = ThemeManager::find_theme(&lower_case);
            assert_eq!(found, Some(first_theme.clone()));
        }
    }

    #[test]
    fn test_default_theme() {
        let default = ThemeManager::default_theme();
        assert!(!default.is_empty());
        assert!(ThemeManager::validate_theme(default).is_ok());
    }

    #[test]
    fn test_popular_themes() {
        let popular = ThemeManager::popular_themes();
        assert!(!popular.is_empty());

        // All popular themes should be valid
        for theme in popular {
            assert!(ThemeManager::validate_theme(&theme).is_ok());
        }
    }

    #[test]
    fn test_get_syntax_and_theme_sets() {
        let syntax_set = get_syntax_set();
        let theme_set = get_theme_set();

        assert!(!syntax_set.syntaxes().is_empty());
        assert!(!theme_set.themes.is_empty());
    }
}
