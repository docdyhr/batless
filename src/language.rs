//! Language detection and management for batless
//!
//! This module handles language detection from file paths and extensions,
//! and provides utilities for listing available languages and themes.

use crate::error::{BatlessError, BatlessResult};
use crate::traits::LanguageDetection;
use std::path::Path;
use std::sync::OnceLock;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

// Cache syntax and theme sets for better performance - loaded only when needed
static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
static THEME_SET: OnceLock<ThemeSet> = OnceLock::new();

fn get_syntax_set_internal() -> &'static SyntaxSet {
    SYNTAX_SET.get_or_init(SyntaxSet::load_defaults_newlines)
}

fn get_theme_set_internal() -> &'static ThemeSet {
    THEME_SET.get_or_init(ThemeSet::load_defaults)
}

/// Language detection and theme management
pub struct LanguageDetector;

impl LanguageDetector {
    /// Detect the programming language from a file path
    pub fn detect_language(file_path: &str) -> Option<String> {
        let path = Path::new(file_path);

        get_syntax_set_internal()
            .find_syntax_for_file(path)
            .ok()
            .flatten()
            .map(|syntax| syntax.name.clone())
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
        let language_name = match extension.to_lowercase().as_str() {
            "rs" => "Rust",
            "py" => "Python",
            "js" => "JavaScript",
            "ts" => "TypeScript",
            "go" => "Go",
            "java" => "Java",
            "cpp" | "cc" | "cxx" => "C++",
            "c" => "C",
            "h" | "hpp" => "C",
            "rb" => "Ruby",
            "php" => "PHP",
            "swift" => "Swift",
            "kt" => "Kotlin",
            "scala" => "Scala",
            "hs" => "Haskell",
            "ml" => "OCaml",
            "fs" => "F#",
            "clj" => "Clojure",
            "ex" | "exs" => "Elixir",
            "erl" => "Erlang",
            "dart" => "Dart",
            "lua" => "Lua",
            "pl" => "Perl",
            "r" => "R",
            "m" => "Objective-C",
            "sh" | "bash" | "zsh" => "Bash",
            "ps1" => "PowerShell",
            "sql" => "SQL",
            "json" => "JSON",
            "xml" => "XML",
            "html" => "HTML",
            "css" => "CSS",
            "scss" | "sass" => "SCSS",
            "md" => "Markdown",
            "yml" | "yaml" => "YAML",
            "toml" => "TOML",
            "ini" => "INI",
            "dockerfile" => "Dockerfile",
            "makefile" => "Makefile",
            _ => return None,
        };
        Some(language_name.to_string())
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
        let mut languages: Vec<String> = get_syntax_set_internal()
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
        get_syntax_set_internal().find_syntax_by_name(language)
    }

    /// Get syntax reference for a file path
    pub fn get_syntax_for_file(
        file_path: &str,
    ) -> Option<&'static syntect::parsing::SyntaxReference> {
        let path = Path::new(file_path);
        get_syntax_set_internal()
            .find_syntax_for_file(path)
            .ok()
            .flatten()
    }
}

impl LanguageDetection for LanguageDetector {
    fn detect_language_with_fallback(&self, file_path: &str) -> Option<String> {
        Self::detect_language_with_fallback(file_path)
    }

    fn detect_from_content(&self, content: &str, file_path: Option<&str>) -> Option<String> {
        if let Some(path) = file_path {
            Self::detect_language(path)
        } else {
            // Simple content-based detection for common patterns
            if content.contains("fn main()") || content.contains("pub fn") {
                Some("Rust".to_string())
            } else if content.contains("function ") || content.contains("const ") {
                Some("JavaScript".to_string())
            } else if content.contains("def ") || content.contains("import ") {
                Some("Python".to_string())
            } else {
                None
            }
        }
    }
}

/// Theme management utilities
pub struct ThemeManager;

impl ThemeManager {
    /// Get list of all available themes
    pub fn list_themes() -> Vec<String> {
        let mut themes: Vec<String> = get_theme_set_internal().themes.keys().cloned().collect();
        themes.sort();
        themes
    }

    /// Validate that a theme exists
    pub fn validate_theme(theme_name: &str) -> BatlessResult<()> {
        if get_theme_set_internal().themes.contains_key(theme_name) {
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
        get_theme_set_internal().themes.get(theme_name)
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
    get_syntax_set_internal()
}

pub fn get_theme_set() -> &'static ThemeSet {
    get_theme_set_internal()
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
