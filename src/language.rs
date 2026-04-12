//! Language detection for batless
//!
//! Extension-based language detection. The syntect dependency has been removed
//! in v0.6.0 as part of the AI-native pivot; language names are derived from
//! a static extension map.

use crate::error::{BatlessError, BatlessResult};
use crate::traits::LanguageDetection;
use std::path::Path;

/// Language detection utilities
pub struct LanguageDetector;

impl LanguageDetector {
    /// Detect the programming language from a file path (extension-based)
    pub fn detect_language(file_path: &str) -> Option<String> {
        let path = Path::new(file_path);
        path.extension()
            .and_then(|e| e.to_str())
            .and_then(Self::extension_to_language)
    }

    /// Detect language with fallback (alias for detect_language post-syntect removal)
    pub fn detect_language_with_fallback(file_path: &str) -> Option<String> {
        Self::detect_language(file_path)
    }

    /// Map file extensions to language names
    pub fn extension_to_language(extension: &str) -> Option<String> {
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

    /// Validate that a language name is in our known set
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

    /// Get sorted list of all known languages
    pub fn list_languages() -> Vec<String> {
        // Derive unique sorted list from all extension mappings
        let all_extensions = [
            "rs",
            "py",
            "js",
            "ts",
            "go",
            "java",
            "cpp",
            "c",
            "rb",
            "php",
            "swift",
            "kt",
            "scala",
            "hs",
            "ml",
            "fs",
            "clj",
            "ex",
            "erl",
            "dart",
            "lua",
            "pl",
            "r",
            "m",
            "sh",
            "ps1",
            "sql",
            "json",
            "xml",
            "html",
            "css",
            "scss",
            "md",
            "yml",
            "toml",
            "ini",
            "dockerfile",
            "makefile",
        ];
        let mut langs: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        for ext in &all_extensions {
            if let Some(lang) = Self::extension_to_language(ext) {
                langs.insert(lang);
            }
        }
        langs.into_iter().collect()
    }

    /// Find a language by name (case-insensitive)
    pub fn find_language(name: &str) -> Option<String> {
        Self::list_languages()
            .into_iter()
            .find(|lang| lang.eq_ignore_ascii_case(name))
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
        let language = LanguageDetector::detect_language_with_fallback("test.rs");
        assert!(language.is_some());

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

        // Check sorted
        let mut sorted = languages.clone();
        sorted.sort();
        assert_eq!(languages, sorted);
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
}
