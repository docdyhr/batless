//! Code summarization functionality for batless
//!
//! This module extracts important code structures and patterns from source files
//! to provide concise summaries of the code content.

use crate::traits::SummaryExtraction;
use std::collections::HashSet;

/// Code summary extractor
pub struct SummaryExtractor;

impl SummaryExtractor {
    /// Extract a summary of important code structures from the given lines
    pub fn extract_summary(lines: &[String], language: Option<&String>) -> Vec<String> {
        let mut summary = Vec::new();
        let mut seen_patterns = HashSet::new();

        for line in lines {
            let trimmed = line.trim();

            // Skip empty lines but include important comments
            if trimmed.is_empty() {
                continue;
            }

            // Include important comments
            if Self::is_important_comment(trimmed) {
                summary.push(line.clone());
                continue;
            }

            // Skip other comments
            if trimmed.starts_with("//") || trimmed.starts_with("#") {
                continue;
            }

            // Check if this line contains summary-worthy content
            if Self::is_summary_worthy(trimmed, &language) {
                // Avoid duplicate patterns in summary
                let pattern_key = Self::extract_pattern_key(trimmed);
                if !seen_patterns.contains(&pattern_key) {
                    summary.push(line.clone());
                    seen_patterns.insert(pattern_key);
                }
            }
        }

        // Limit summary size to keep it concise
        if summary.len() > 50 {
            summary.truncate(50);
        }

        summary
    }

    /// Check if a line contains summary-worthy code structures
    fn is_summary_worthy(line: &str, language: &Option<&String>) -> bool {
        let trimmed = line.trim();

        // Skip empty lines and basic comments
        if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with("#") {
            return false;
        }

        match language.map(|s| s.as_str()) {
            Some("Python") => Self::is_python_summary_worthy(trimmed),
            Some("Rust") => Self::is_rust_summary_worthy(trimmed),
            Some("JavaScript" | "TypeScript") => Self::is_js_ts_summary_worthy(trimmed),
            Some("Java") => Self::is_java_summary_worthy(trimmed),
            Some("C" | "C++") => Self::is_c_cpp_summary_worthy(trimmed),
            Some("Go") => Self::is_go_summary_worthy(trimmed),
            Some("Ruby") => Self::is_ruby_summary_worthy(trimmed),
            Some("PHP") => Self::is_php_summary_worthy(trimmed),
            Some("Swift") => Self::is_swift_summary_worthy(trimmed),
            Some("Kotlin") => Self::is_kotlin_summary_worthy(trimmed),
            Some("Scala") => Self::is_scala_summary_worthy(trimmed),
            Some("Haskell") => Self::is_haskell_summary_worthy(trimmed),
            Some("Clojure") => Self::is_clojure_summary_worthy(trimmed),
            Some("Elixir") => Self::is_elixir_summary_worthy(trimmed),
            Some("Erlang") => Self::is_erlang_summary_worthy(trimmed),
            _ => Self::is_generic_summary_worthy(trimmed),
        }
    }

    /// Python-specific summary patterns
    fn is_python_summary_worthy(line: &str) -> bool {
        line.starts_with("def ")
            || line.starts_with("class ")
            || line.starts_with("import ")
            || line.starts_with("from ")
            || line.starts_with("@")  // decorators
            || line.starts_with("async def ")
            || line.starts_with("if __name__ == ")
    }

    /// Rust-specific summary patterns
    fn is_rust_summary_worthy(line: &str) -> bool {
        // Function definitions
        ((line.starts_with("fn ") || line.starts_with("pub fn ") || line.starts_with("async fn "))
            && line.contains("{"))
        // Struct/enum/trait definitions
        || (line.starts_with("struct ") || line.starts_with("pub struct "))
        || (line.starts_with("enum ") || line.starts_with("pub enum "))
        || (line.starts_with("trait ") || line.starts_with("pub trait "))
        || line.starts_with("impl ")
        // Imports and constants
        || line.starts_with("use ")
        || (line.starts_with("const ") || line.starts_with("pub const "))
        || (line.starts_with("static ") || line.starts_with("pub static "))
        // Macros
        || line.starts_with("macro_rules!")
    }

    /// JavaScript/TypeScript-specific summary patterns
    fn is_js_ts_summary_worthy(line: &str) -> bool {
        line.starts_with("function ")
            || line.starts_with("class ")
            || line.starts_with("interface ")
            || line.starts_with("type ")
            || line.starts_with("export ")
            || line.starts_with("import ")
            || line.starts_with("async function ")
            || (line.starts_with("const ") && (line.contains("function") || line.contains("=>")))
            || (line.starts_with("let ") && (line.contains("function") || line.contains("=>")))
            || (line.starts_with("var ") && (line.contains("function") || line.contains("=>")))
    }

    /// Java-specific summary patterns
    fn is_java_summary_worthy(line: &str) -> bool {
        line.starts_with("public class ")
            || line.starts_with("private class ")
            || line.starts_with("protected class ")
            || line.starts_with("class ")
            || line.starts_with("interface ")
            || line.starts_with("enum ")
            || line.starts_with("public ")
            || line.starts_with("private ")
            || line.starts_with("protected ")
            || line.starts_with("import ")
            || line.starts_with("package ")
            || line.contains("void main(")
    }

    /// C/C++-specific summary patterns
    fn is_c_cpp_summary_worthy(line: &str) -> bool {
        line.starts_with("#include ")
            || line.starts_with("#define ")
            || line.starts_with("typedef ")
            || line.starts_with("struct ")
            || line.starts_with("class ")
            || line.starts_with("namespace ")
            || line.starts_with("template ")
            || (line.contains("(") && line.contains(")") && line.contains("{"))  // function definitions
            || line.starts_with("extern ")
            || line.starts_with("static ")
    }

    /// Go-specific summary patterns
    fn is_go_summary_worthy(line: &str) -> bool {
        line.starts_with("func ")
            || line.starts_with("type ")
            || line.starts_with("var ")
            || line.starts_with("const ")
            || line.starts_with("package ")
            || line.starts_with("import ")
            || line.starts_with("interface ")
            || line.starts_with("struct ")
    }

    /// Ruby-specific summary patterns
    fn is_ruby_summary_worthy(line: &str) -> bool {
        line.starts_with("def ")
            || line.starts_with("class ")
            || line.starts_with("module ")
            || line.starts_with("require ")
            || line.starts_with("include ")
            || line.starts_with("extend ")
            || line.starts_with("attr_")
    }

    /// PHP-specific summary patterns
    fn is_php_summary_worthy(line: &str) -> bool {
        line.starts_with("function ")
            || line.starts_with("class ")
            || line.starts_with("interface ")
            || line.starts_with("trait ")
            || line.starts_with("namespace ")
            || line.starts_with("use ")
            || line.starts_with("require ")
            || line.starts_with("include ")
            || line.starts_with("public function ")
            || line.starts_with("private function ")
            || line.starts_with("protected function ")
    }

    /// Swift-specific summary patterns
    fn is_swift_summary_worthy(line: &str) -> bool {
        line.starts_with("func ")
            || line.starts_with("class ")
            || line.starts_with("struct ")
            || line.starts_with("enum ")
            || line.starts_with("protocol ")
            || line.starts_with("extension ")
            || line.starts_with("import ")
            || line.starts_with("var ")
            || line.starts_with("let ")
            || line.starts_with("typealias ")
    }

    /// Kotlin-specific summary patterns
    fn is_kotlin_summary_worthy(line: &str) -> bool {
        line.starts_with("fun ")
            || line.starts_with("class ")
            || line.starts_with("interface ")
            || line.starts_with("object ")
            || line.starts_with("enum class ")
            || line.starts_with("data class ")
            || line.starts_with("sealed class ")
            || line.starts_with("import ")
            || line.starts_with("package ")
            || line.starts_with("val ")
            || line.starts_with("var ")
    }

    /// Scala-specific summary patterns
    fn is_scala_summary_worthy(line: &str) -> bool {
        line.starts_with("def ")
            || line.starts_with("class ")
            || line.starts_with("object ")
            || line.starts_with("trait ")
            || line.starts_with("case class ")
            || line.starts_with("sealed trait ")
            || line.starts_with("import ")
            || line.starts_with("package ")
            || line.starts_with("val ")
            || line.starts_with("var ")
    }

    /// Haskell-specific summary patterns
    fn is_haskell_summary_worthy(line: &str) -> bool {
        line.contains(" :: ")  // type signatures
            || line.starts_with("data ")
            || line.starts_with("type ")
            || line.starts_with("newtype ")
            || line.starts_with("class ")
            || line.starts_with("instance ")
            || line.starts_with("import ")
            || line.starts_with("module ")
    }

    /// Clojure-specific summary patterns
    fn is_clojure_summary_worthy(line: &str) -> bool {
        line.starts_with("(defn ")
            || line.starts_with("(defn- ")
            || line.starts_with("(defmacro ")
            || line.starts_with("(def ")
            || line.starts_with("(defprotocol ")
            || line.starts_with("(defrecord ")
            || line.starts_with("(deftype ")
            || line.starts_with("(ns ")
            || line.starts_with("(:require ")
            || line.starts_with("(:use ")
    }

    /// Elixir-specific summary patterns
    fn is_elixir_summary_worthy(line: &str) -> bool {
        line.starts_with("def ")
            || line.starts_with("defp ")
            || line.starts_with("defmodule ")
            || line.starts_with("defprotocol ")
            || line.starts_with("defimpl ")
            || line.starts_with("defstruct ")
            || line.starts_with("defmacro ")
            || line.starts_with("import ")
            || line.starts_with("alias ")
            || line.starts_with("use ")
    }

    /// Erlang-specific summary patterns
    fn is_erlang_summary_worthy(line: &str) -> bool {
        line.starts_with("-module(")
            || line.starts_with("-export(")
            || line.starts_with("-import(")
            || line.starts_with("-include(")
            || line.starts_with("-record(")
            || line.starts_with("-type(")
            || line.starts_with("-spec(")
            || (line.contains("(") && line.contains("->")) // function definitions
    }

    /// Generic patterns for unknown languages
    fn is_generic_summary_worthy(line: &str) -> bool {
        // Function-like patterns
        (line.starts_with("def ") && line.contains(":"))
            || (line.starts_with("class ") && line.contains(":"))
            || (line.starts_with("function ") && line.contains("{"))
            || ((line.starts_with("fn ") || line.starts_with("pub fn ")) && line.contains("{"))
            || (line.starts_with("struct ") || line.starts_with("pub struct "))
            || (line.starts_with("enum ") || line.starts_with("pub enum "))
            // Import-like patterns
            || line.starts_with("import ")
            || line.starts_with("use ")
            || line.starts_with("export ")
            || line.starts_with("module ")
            || line.starts_with("package ")
            || line.starts_with("namespace ")
            // Declaration patterns
            || line.starts_with("typedef ")
            || line.starts_with("interface ")
            || line.starts_with("protocol ")
            || line.starts_with("trait ")
    }

    /// Check if a comment is important enough to include in summary
    fn is_important_comment(line: &str) -> bool {
        let line_lower = line.to_lowercase();
        // Check for specific important markers at word boundaries
        line_lower.contains("todo:")
            || line_lower.contains("fixme:")
            || line_lower.contains("hack:")
            || line_lower.contains("note:")
            || line_lower.contains("warning:")
            || line_lower.contains("important:")
            || line_lower.starts_with("///")  // Rust doc comments
            || line_lower.starts_with("/**")  // Multi-line doc comments
            || line_lower.starts_with("#!")   // Shebang or module-level comments
            || line_lower.starts_with("##") // Important markdown headers
    }

    /// Extract a pattern key for deduplication
    fn extract_pattern_key(line: &str) -> String {
        let trimmed = line.trim();

        // Extract the essential pattern (first few words)
        let words: Vec<&str> = trimmed.split_whitespace().take(3).collect();
        words.join(" ")
    }

    /// Get summary statistics
    pub fn get_summary_stats(original_lines: &[String], summary_lines: &[String]) -> SummaryStats {
        SummaryStats {
            original_line_count: original_lines.len(),
            summary_line_count: summary_lines.len(),
            compression_ratio: if original_lines.is_empty() {
                0.0
            } else {
                summary_lines.len() as f64 / original_lines.len() as f64
            },
            reduction_percentage: if original_lines.is_empty() {
                0.0
            } else {
                (1.0 - (summary_lines.len() as f64 / original_lines.len() as f64)) * 100.0
            },
        }
    }
}

impl SummaryExtraction for SummaryExtractor {
    fn extract_summary(&self, lines: &[String], language: Option<&str>) -> Vec<String> {
        Self::extract_summary(lines, language.map(|s| s.to_string()).as_ref())
    }

    fn is_summary_worthy(&self, line: &str, language: Option<&str>) -> bool {
        let lang_string = language.map(|s| s.to_string());
        Self::is_summary_worthy(line, &lang_string.as_ref())
    }
}

/// Statistics about the summarization process
#[derive(Debug, Clone)]
pub struct SummaryStats {
    pub original_line_count: usize,
    pub summary_line_count: usize,
    pub compression_ratio: f64,
    pub reduction_percentage: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_summary() {
        let lines = vec![
            "def main():".to_string(),
            "    print('hello')".to_string(),
            "class MyClass:".to_string(),
            "    pass".to_string(),
            "import os".to_string(),
        ];

        let summary = SummaryExtractor::extract_summary(&lines, Some(&"Python".to_string()));
        assert_eq!(summary.len(), 3); // def, class, import
        assert!(summary.contains(&"def main():".to_string()));
        assert!(summary.contains(&"class MyClass:".to_string()));
        assert!(summary.contains(&"import os".to_string()));
    }

    #[test]
    fn test_rust_summary() {
        let lines = vec![
            "fn main() {".to_string(),
            "    println!(\"Hello\");".to_string(),
            "}".to_string(),
            "struct Point {".to_string(),
            "    x: i32,".to_string(),
            "}".to_string(),
            "use std::collections::HashMap;".to_string(),
        ];

        let summary = SummaryExtractor::extract_summary(&lines, Some(&"Rust".to_string()));
        assert_eq!(summary.len(), 3); // fn, struct, use
        assert!(summary.contains(&"fn main() {".to_string()));
        assert!(summary.contains(&"struct Point {".to_string()));
        assert!(summary.contains(&"use std::collections::HashMap;".to_string()));
    }

    #[test]
    fn test_javascript_summary() {
        let lines = vec![
            "function hello() {".to_string(),
            "    console.log('hello');".to_string(),
            "}".to_string(),
            "class MyClass {".to_string(),
            "    constructor() {}".to_string(),
            "}".to_string(),
            "export default MyClass;".to_string(),
        ];

        let summary = SummaryExtractor::extract_summary(&lines, Some(&"JavaScript".to_string()));
        assert_eq!(summary.len(), 3); // function, class, export
        assert!(summary.contains(&"function hello() {".to_string()));
        assert!(summary.contains(&"class MyClass {".to_string()));
        assert!(summary.contains(&"export default MyClass;".to_string()));
    }

    #[test]
    fn test_empty_input() {
        let lines = vec![];
        let summary = SummaryExtractor::extract_summary(&lines, Some(&"Python".to_string()));
        assert!(summary.is_empty());
    }

    #[test]
    fn test_comments_filtering() {
        let lines = vec![
            "// This is a comment".to_string(),
            "fn main() {".to_string(),
            "    // Another comment".to_string(),
            "    println!(\"Hello\");".to_string(),
            "}".to_string(),
            "/// This is an important doc comment".to_string(),
        ];

        let summary = SummaryExtractor::extract_summary(&lines, Some(&"Rust".to_string()));
        assert_eq!(summary.len(), 2); // fn and doc comment
        assert!(summary.contains(&"fn main() {".to_string()));
        assert!(summary.contains(&"/// This is an important doc comment".to_string()));
    }

    #[test]
    fn test_deduplication() {
        let lines = vec![
            "fn test1() {".to_string(),
            "fn test2() {".to_string(),
            "fn test3() {".to_string(),
        ];

        let summary = SummaryExtractor::extract_summary(&lines, Some(&"Rust".to_string()));
        // Should include all since they have different names
        assert_eq!(summary.len(), 3);
    }

    #[test]
    fn test_summary_stats() {
        let original = vec!["line1".to_string(); 100];
        let summary = vec!["line1".to_string(); 20];

        let stats = SummaryExtractor::get_summary_stats(&original, &summary);
        assert_eq!(stats.original_line_count, 100);
        assert_eq!(stats.summary_line_count, 20);
        assert_eq!(stats.compression_ratio, 0.2);
        assert_eq!(stats.reduction_percentage, 80.0);
    }

    #[test]
    fn test_generic_language() {
        let lines = vec![
            "function test() {".to_string(),
            "    return true;".to_string(),
            "}".to_string(),
            "import something;".to_string(),
        ];

        let summary = SummaryExtractor::extract_summary(&lines, None);
        assert_eq!(summary.len(), 2); // function and import
        assert!(summary.contains(&"function test() {".to_string()));
        assert!(summary.contains(&"import something;".to_string()));
    }
}
