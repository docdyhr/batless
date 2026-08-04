//! Symbol index output mode for batless
//!
//! Produces a machine-readable JSON symbol table instead of file lines.
//! Useful for AI agents that want to build a project-wide symbol index
//! without reading every line of every file.

use crate::config::BatlessConfig;
use crate::error::BatlessResult;
use crate::file_info::FileInfo;
use crate::formatter::OutputMode;
use crate::formatters::Formatter;
use crate::summarizer::SummaryExtractor;
use crate::summary::SummaryLevel;
use crate::summary_item::SummaryItem;
use serde_json::{json, Value};

/// Symbol index formatter — emits a compact JSON symbol table.
pub struct IndexFormatter;

impl IndexFormatter {
    /// Extract a name identifier from a declaration signature line.
    ///
    /// Handles the most common patterns across the supported languages:
    /// - Rust:  `pub fn foo(`, `struct Foo {`, `impl Foo {`, `trait Foo {`
    /// - Python: `def foo(`, `class Foo:`
    /// - JS/TS:  `function foo(`, `class Foo`, `const foo =`, `export function foo(`
    fn extract_name(line: &str) -> Option<String> {
        let t = line.trim();
        // Strip common visibility / keyword prefixes to get to the identifier
        let t = t
            .trim_start_matches("pub(crate)")
            .trim_start_matches("pub(super)")
            .trim_start_matches("pub ")
            .trim_start_matches("async ")
            .trim_start_matches("export default ")
            .trim_start_matches("export async ")
            .trim_start_matches("export ")
            .trim_start_matches("default ")
            .trim()
            .trim_start_matches("fn ")
            .trim_start_matches("def ")
            .trim_start_matches("function ")
            .trim_start_matches("class ")
            .trim_start_matches("struct ")
            .trim_start_matches("enum ")
            .trim_start_matches("trait ")
            .trim_start_matches("impl ")
            .trim_start_matches("mod ")
            .trim_start_matches("type ")
            .trim_start_matches("const ")
            .trim_start_matches("let ")
            .trim_start_matches("var ")
            .trim();

        // Take the leading identifier (alphanumeric + underscore, possibly with generic < or ()
        let name: String = t
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();

        if name.is_empty() {
            None
        } else {
            Some(name)
        }
    }

    /// Determine visibility from the signature line.
    fn extract_visibility(line: &str, language: Option<&str>) -> Option<String> {
        let t = line.trim();
        let lang = language.unwrap_or("").to_lowercase();
        if lang.contains("rust") {
            if t.starts_with("pub(crate)") {
                return Some("pub(crate)".to_string());
            }
            if t.starts_with("pub(super)") {
                return Some("pub(super)".to_string());
            }
            if t.starts_with("pub ") {
                return Some("pub".to_string());
            }
            return Some("private".to_string());
        }
        if lang.contains("javascript")
            || lang.contains("typescript")
            || lang.contains("jsx")
            || lang.contains("tsx")
        {
            if t.starts_with("export default ")
                || t.starts_with("export ")
                || t.starts_with("module.exports")
            {
                return Some("export".to_string());
            }
            return Some("local".to_string());
        }
        None
    }

    /// Convert a `SummaryItem` into a JSON symbol object.
    fn symbol_to_json(item: &SummaryItem, language: Option<&str>) -> Value {
        let name = Self::extract_name(&item.line).unwrap_or_else(|| "unknown".to_string());
        let visibility = Self::extract_visibility(&item.line, language);
        let mut obj = json!({
            "kind": item.kind,
            "name": name,
            "line_start": item.line_number,
            "signature": item.line.trim(),
        });
        if let Some(end) = item.end_line {
            obj["line_end"] = json!(end);
        }
        if let Some(vis) = visibility {
            obj["visibility"] = json!(vis);
        }
        obj
    }
}

impl Formatter for IndexFormatter {
    fn format(
        &self,
        file_info: &FileInfo,
        file_path: &str,
        _config: &BatlessConfig,
    ) -> BatlessResult<String> {
        let language = file_info.language.as_deref();

        // Use detailed summary level to capture the most symbols
        let items: Vec<SummaryItem> =
            SummaryExtractor::extract_summary(&file_info.lines, language, SummaryLevel::Detailed);

        let symbols: Vec<Value> = items
            .iter()
            .map(|item| Self::symbol_to_json(item, language))
            .collect();

        let output = json!({
            "file": file_path,
            "language": language,
            "total_lines": file_info.total_lines,
            "total_bytes": file_info.total_bytes,
            "symbol_count": symbols.len(),
            "symbols": symbols,
            "mode": "index",
        });

        Ok(serde_json::to_string_pretty(&output)?)
    }

    fn output_mode(&self) -> OutputMode {
        OutputMode::Index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_name_rust_fn() {
        assert_eq!(
            IndexFormatter::extract_name("pub fn process_file("),
            Some("process_file".to_string())
        );
    }

    #[test]
    fn test_extract_name_python_class() {
        assert_eq!(
            IndexFormatter::extract_name("class MyClass:"),
            Some("MyClass".to_string())
        );
    }

    #[test]
    fn test_extract_name_js_function() {
        assert_eq!(
            IndexFormatter::extract_name("export function handleRequest("),
            Some("handleRequest".to_string())
        );
    }

    #[test]
    fn test_extract_visibility_rust_pub() {
        assert_eq!(
            IndexFormatter::extract_visibility("pub fn foo()", Some("Rust")),
            Some("pub".to_string())
        );
    }

    #[test]
    fn test_extract_visibility_rust_private() {
        assert_eq!(
            IndexFormatter::extract_visibility("fn foo()", Some("Rust")),
            Some("private".to_string())
        );
    }

    #[test]
    fn test_extract_visibility_js_export() {
        assert_eq!(
            IndexFormatter::extract_visibility("export function foo() {}", Some("JavaScript")),
            Some("export".to_string())
        );
    }

    #[test]
    fn test_format_produces_index_json() {
        let lines = vec![
            "pub fn process_file(path: &str) {".to_string(),
            "    todo!()".to_string(),
            "}".to_string(),
            "struct Config {".to_string(),
            "}".to_string(),
        ];
        let file_info = FileInfo::with_metadata(
            lines.len(),
            100,
            Some("Rust".to_string()),
            "UTF-8".to_string(),
        )
        .with_lines(lines);
        let config = BatlessConfig::default();

        let formatted = IndexFormatter
            .format(&file_info, "src/lib.rs", &config)
            .unwrap();
        let parsed: Value = serde_json::from_str(&formatted).unwrap();

        assert_eq!(parsed["file"], "src/lib.rs");
        assert_eq!(parsed["language"], "Rust");
        assert_eq!(parsed["mode"], "index");
        assert_eq!(parsed["total_lines"], 5);
        assert_eq!(parsed["total_bytes"], 100);

        let symbols = parsed["symbols"].as_array().unwrap();
        assert_eq!(parsed["symbol_count"], symbols.len());
        assert!(symbols
            .iter()
            .any(|s| s["name"] == "process_file" && s["visibility"] == "pub"));
        assert!(symbols
            .iter()
            .any(|s| s["name"] == "Config" && s["visibility"] == "private"));
    }

    #[test]
    fn test_symbol_to_json_unknown_name_fallback() {
        let item = SummaryItem {
            kind: "function".to_string(),
            line: "???".to_string(),
            line_number: 1,
            end_line: None,
        };
        let json = IndexFormatter::symbol_to_json(&item, None);
        assert_eq!(json["name"], "unknown");
        assert!(json.get("visibility").is_none());
    }
}
