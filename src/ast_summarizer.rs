//! AST-based code summarization using tree-sitter
//!
//! This module provides robust, syntax-aware code summarization by parsing
//! the source code into an Abstract Syntax Tree (AST) and extracting
//! relevant nodes based on the summary level.

use crate::summary::SummaryLevel;
use crate::summary_item::SummaryItem;
use std::ops::ControlFlow;
use std::time::{Duration, Instant};
use tree_sitter::{ParseOptions, Parser, Query, QueryCursor, StreamingIterator};

/// Maximum time allowed for tree-sitter parsing before aborting.
const PARSE_TIMEOUT: Duration = Duration::from_millis(500);

/// AST-based summary extractor
pub struct AstSummarizer;

impl AstSummarizer {
    /// Parse content with a timeout to prevent hangs on pathological inputs.
    /// Returns `None` if parsing fails or times out.
    fn parse_with_timeout(parser: &mut Parser, content: &str) -> Option<tree_sitter::Tree> {
        Self::parse_with_deadline(parser, content, PARSE_TIMEOUT)
    }

    /// Parse content with a configurable timeout duration.
    /// Returns `None` if parsing fails or the deadline is exceeded.
    fn parse_with_deadline(
        parser: &mut Parser,
        content: &str,
        timeout: Duration,
    ) -> Option<tree_sitter::Tree> {
        let deadline = Instant::now() + timeout;
        let bytes = content.as_bytes();
        let len = bytes.len();
        let mut progress = |_: &_| {
            if Instant::now() >= deadline {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        };
        let mut options = ParseOptions::new().progress_callback(&mut progress);
        parser.parse_with_options(
            &mut |i, _| {
                if i < len {
                    &bytes[i..]
                } else {
                    &[]
                }
            },
            None,
            Some(options.reborrow()),
        )
    }

    /// Extract a summary of important code structures using AST parsing
    pub fn extract_summary(
        content: &str,
        language: Option<&str>,
        level: SummaryLevel,
    ) -> Vec<SummaryItem> {
        if !level.is_enabled() {
            return Vec::new();
        }

        match language {
            Some("Rust") => Self::summarize_rust(content, level),
            Some("Python") => Self::summarize_python(content, level),
            Some("JavaScript" | "JSX") => Self::summarize_javascript(content, level),
            Some("TypeScript" | "TSX") => Self::summarize_typescript(content, level),
            // Fallback to empty for unsupported languages (caller should handle fallback to regex)
            _ => Vec::new(),
        }
    }

    fn summarize_rust(content: &str, level: SummaryLevel) -> Vec<SummaryItem> {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .expect("Error loading Rust grammar");

        let Some(tree) = Self::parse_with_timeout(&mut parser, content) else {
            return Vec::new();
        };
        let root_node = tree.root_node();

        let query_string = match level {
            SummaryLevel::Minimal => {
                "(function_item name: (identifier) @name) @function
                 (struct_item name: (type_identifier) @name) @struct
                 (enum_item name: (type_identifier) @name) @enum
                 (impl_item) @impl"
            }
            SummaryLevel::Standard => {
                "(function_item name: (identifier) @name) @function
                 (struct_item name: (type_identifier) @name) @struct
                 (enum_item name: (type_identifier) @name) @enum
                 (impl_item) @impl
                 (trait_item name: (type_identifier) @name) @trait
                 (mod_item name: (identifier) @name) @mod
                 (use_declaration) @use"
            }
            SummaryLevel::Detailed => {
                "(function_item name: (identifier) @name) @function
                 (struct_item name: (type_identifier) @name) @struct
                 (enum_item name: (type_identifier) @name) @enum
                 (impl_item) @impl
                 (trait_item name: (type_identifier) @name) @trait
                 (mod_item name: (identifier) @name) @mod
                 (macro_definition name: (identifier) @name) @macro
                 (use_declaration) @use
                 (let_declaration) @let
                 (const_item) @const
                 (static_item) @static"
            }
            SummaryLevel::None => return Vec::new(),
        };

        let query = Query::new(&tree_sitter_rust::LANGUAGE.into(), query_string)
            .expect("Error compiling query");

        let capture_names = query.capture_names().to_vec();
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&query, root_node, content.as_bytes());

        let lines: Vec<&str> = content.lines().collect();
        // BTreeMap<start_line, (kind, end_line)> — first write wins per line
        let mut line_items: std::collections::BTreeMap<usize, (String, usize)> =
            std::collections::BTreeMap::new();

        while let Some(m) = matches.next() {
            // Use @name capture's row when present — it lands on the declaration line,
            // not on any preceding decorator whose span is included in the outer node
            let name_row = m
                .captures
                .iter()
                .find(|c| capture_names[c.index as usize] == "name")
                .map(|c| c.node.start_position().row);
            for capture in m.captures {
                let kind = &capture_names[capture.index as usize];
                if *kind == "name" {
                    continue;
                }
                let start_line = name_row.unwrap_or_else(|| capture.node.start_position().row);
                let end_line = capture.node.end_position().row;
                line_items
                    .entry(start_line)
                    .or_insert_with(|| (kind.to_string(), end_line));
            }
        }

        line_items
            .into_iter()
            .filter_map(|(idx, (kind, end_row))| {
                lines
                    .get(idx)
                    .map(|&line| SummaryItem::new(line, idx + 1, Some(end_row + 1), kind))
            })
            .collect()
    }

    fn summarize_python(content: &str, level: SummaryLevel) -> Vec<SummaryItem> {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_python::LANGUAGE.into())
            .expect("Error loading Python grammar");

        let Some(tree) = Self::parse_with_timeout(&mut parser, content) else {
            return Vec::new();
        };
        let root_node = tree.root_node();

        let query_string = match level {
            SummaryLevel::Minimal => {
                "(function_definition name: (identifier) @name) @function
                 (class_definition name: (identifier) @name) @class"
            }
            SummaryLevel::Standard => {
                "(function_definition name: (identifier) @name) @function
                 (class_definition name: (identifier) @name) @class
                 (import_statement) @import
                 (import_from_statement) @import_from
                 (decorated_definition) @decorator"
            }
            SummaryLevel::Detailed => {
                "(function_definition name: (identifier) @name) @function
                 (class_definition name: (identifier) @name) @class
                 (import_statement) @import
                 (import_from_statement) @import_from
                 (decorated_definition) @decorator
                 (assignment left: (identifier) @name) @assignment
                 (global_statement) @global
                 (nonlocal_statement) @nonlocal"
            }
            SummaryLevel::None => return Vec::new(),
        };

        let query = Query::new(&tree_sitter_python::LANGUAGE.into(), query_string)
            .expect("Error compiling query");

        let capture_names = query.capture_names().to_vec();
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&query, root_node, content.as_bytes());

        let lines: Vec<&str> = content.lines().collect();
        let mut line_items: std::collections::BTreeMap<usize, (String, usize)> =
            std::collections::BTreeMap::new();

        while let Some(m) = matches.next() {
            // Use @name capture's row when present — it lands on the declaration line,
            // not on any preceding decorator whose span is included in the outer node
            let name_row = m
                .captures
                .iter()
                .find(|c| capture_names[c.index as usize] == "name")
                .map(|c| c.node.start_position().row);
            for capture in m.captures {
                let kind = &capture_names[capture.index as usize];
                if *kind == "name" {
                    continue;
                }
                let start_line = name_row.unwrap_or_else(|| capture.node.start_position().row);
                let end_line = capture.node.end_position().row;
                line_items
                    .entry(start_line)
                    .or_insert_with(|| (kind.to_string(), end_line));
            }
        }

        line_items
            .into_iter()
            .filter_map(|(idx, (kind, end_row))| {
                lines
                    .get(idx)
                    .map(|&line| SummaryItem::new(line, idx + 1, Some(end_row + 1), kind))
            })
            .collect()
    }

    fn summarize_javascript(content: &str, level: SummaryLevel) -> Vec<SummaryItem> {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_javascript::LANGUAGE.into())
            .expect("Error loading JavaScript grammar");

        let Some(tree) = Self::parse_with_timeout(&mut parser, content) else {
            return Vec::new();
        };
        let root_node = tree.root_node();

        let query_string = match level {
            SummaryLevel::Minimal => {
                "(function_declaration name: (identifier) @name) @function
                 (class_declaration name: (identifier) @name) @class
                 (arrow_function) @arrow"
            }
            SummaryLevel::Standard => {
                "(function_declaration name: (identifier) @name) @function
                 (class_declaration name: (identifier) @name) @class
                 (method_definition name: (property_identifier) @name) @method
                 (arrow_function) @arrow
                 (export_statement) @export
                 (import_statement) @import"
            }
            SummaryLevel::Detailed => {
                "(function_declaration name: (identifier) @name) @function
                 (class_declaration name: (identifier) @name) @class
                 (method_definition name: (property_identifier) @name) @method
                 (arrow_function) @arrow
                 (export_statement) @export
                 (import_statement) @import
                 (variable_declarator name: (identifier) @name) @var
                 (lexical_declaration) @const"
            }
            SummaryLevel::None => return Vec::new(),
        };

        let query = Query::new(&tree_sitter_javascript::LANGUAGE.into(), query_string)
            .expect("Error compiling query");

        let capture_names = query.capture_names().to_vec();
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&query, root_node, content.as_bytes());

        let lines: Vec<&str> = content.lines().collect();
        let mut line_items: std::collections::BTreeMap<usize, (String, usize)> =
            std::collections::BTreeMap::new();

        while let Some(m) = matches.next() {
            // Use @name capture's row when present — it lands on the declaration line,
            // not on any preceding decorator whose span is included in the outer node
            let name_row = m
                .captures
                .iter()
                .find(|c| capture_names[c.index as usize] == "name")
                .map(|c| c.node.start_position().row);
            for capture in m.captures {
                let kind = &capture_names[capture.index as usize];
                if *kind == "name" {
                    continue;
                }
                let start_line = name_row.unwrap_or_else(|| capture.node.start_position().row);
                let end_line = capture.node.end_position().row;
                line_items
                    .entry(start_line)
                    .or_insert_with(|| (kind.to_string(), end_line));
            }
        }

        line_items
            .into_iter()
            .filter_map(|(idx, (kind, end_row))| {
                lines
                    .get(idx)
                    .map(|&line| SummaryItem::new(line, idx + 1, Some(end_row + 1), kind))
            })
            .collect()
    }

    fn summarize_typescript(content: &str, level: SummaryLevel) -> Vec<SummaryItem> {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .expect("Error loading TypeScript grammar");

        let Some(tree) = Self::parse_with_timeout(&mut parser, content) else {
            return Vec::new();
        };
        let root_node = tree.root_node();

        let query_string = match level {
            SummaryLevel::Minimal => {
                "(function_declaration name: (identifier) @name) @function
                 (class_declaration name: (type_identifier) @name) @class
                 (interface_declaration name: (type_identifier) @name) @interface
                 (arrow_function) @arrow"
            }
            SummaryLevel::Standard => {
                "(function_declaration name: (identifier) @name) @function
                 (class_declaration name: (type_identifier) @name) @class
                 (interface_declaration name: (type_identifier) @name) @interface
                 (type_alias_declaration name: (type_identifier) @name) @type
                 (method_definition name: (property_identifier) @name) @method
                 (arrow_function) @arrow
                 (export_statement) @export
                 (import_statement) @import"
            }
            SummaryLevel::Detailed => {
                "(function_declaration name: (identifier) @name) @function
                 (class_declaration name: (type_identifier) @name) @class
                 (interface_declaration name: (type_identifier) @name) @interface
                 (type_alias_declaration name: (type_identifier) @name) @type
                 (enum_declaration name: (identifier) @name) @enum
                 (method_definition name: (property_identifier) @name) @method
                 (arrow_function) @arrow
                 (export_statement) @export
                 (import_statement) @import
                 (variable_declarator name: (identifier) @name) @var
                 (lexical_declaration) @const"
            }
            SummaryLevel::None => return Vec::new(),
        };

        let query = Query::new(
            &tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            query_string,
        )
        .expect("Error compiling query");

        let capture_names = query.capture_names().to_vec();
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&query, root_node, content.as_bytes());

        let lines: Vec<&str> = content.lines().collect();
        let mut line_items: std::collections::BTreeMap<usize, (String, usize)> =
            std::collections::BTreeMap::new();

        while let Some(m) = matches.next() {
            // Use @name capture's row when present — it lands on the declaration line,
            // not on any preceding decorator whose span is included in the outer node
            let name_row = m
                .captures
                .iter()
                .find(|c| capture_names[c.index as usize] == "name")
                .map(|c| c.node.start_position().row);
            for capture in m.captures {
                let kind = &capture_names[capture.index as usize];
                if *kind == "name" {
                    continue;
                }
                let start_line = name_row.unwrap_or_else(|| capture.node.start_position().row);
                let end_line = capture.node.end_position().row;
                line_items
                    .entry(start_line)
                    .or_insert_with(|| (kind.to_string(), end_line));
            }
        }

        line_items
            .into_iter()
            .filter_map(|(idx, (kind, end_row))| {
                lines
                    .get(idx)
                    .map(|&line| SummaryItem::new(line, idx + 1, Some(end_row + 1), kind))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Write as _;

    use super::*;

    #[test]
    fn test_empty_input_all_languages() {
        for lang in &["Rust", "Python", "JavaScript", "TypeScript"] {
            let result = AstSummarizer::extract_summary("", Some(lang), SummaryLevel::Standard);
            assert!(
                result.is_empty(),
                "Empty input should produce empty summary for {lang}"
            );
        }
    }

    #[test]
    fn test_unsupported_language_returns_empty() {
        let result =
            AstSummarizer::extract_summary("some code", Some("Haskell"), SummaryLevel::Standard);
        assert!(result.is_empty());
    }

    #[test]
    fn test_none_language_returns_empty() {
        let result = AstSummarizer::extract_summary("fn main() {}", None, SummaryLevel::Standard);
        assert!(result.is_empty());
    }

    #[test]
    fn test_none_level_returns_empty() {
        let result =
            AstSummarizer::extract_summary("fn main() {}", Some("Rust"), SummaryLevel::None);
        assert!(result.is_empty());
    }

    #[test]
    fn test_binary_content_does_not_panic() {
        let binary = "\x00\x01\x02binary\x00data\x7f";
        for lang in &["Rust", "Python", "JavaScript", "TypeScript"] {
            // Should not panic, just return empty or partial results
            let _ = AstSummarizer::extract_summary(binary, Some(lang), SummaryLevel::Standard);
        }
    }

    #[test]
    fn test_malformed_rust_does_not_panic() {
        let bad = "fn {{ struct {{ impl {";
        let result = AstSummarizer::extract_summary(bad, Some("Rust"), SummaryLevel::Standard);
        // May return partial results or empty; should not panic
        let _ = result;
    }

    #[test]
    fn test_malformed_python_does_not_panic() {
        let bad = "def def class (((";
        let _ = AstSummarizer::extract_summary(bad, Some("Python"), SummaryLevel::Standard);
    }

    #[test]
    fn test_rust_minimal_level() {
        let code = "use std::io;\nfn main() {}\nstruct S {}\nenum E {}\ntrait T {}\nmod m {}";
        let result = AstSummarizer::extract_summary(code, Some("Rust"), SummaryLevel::Minimal);
        assert!(result.iter().any(|l| l.line.contains("fn main")));
        assert!(result.iter().any(|l| l.line.contains("struct S")));
        assert!(result.iter().any(|l| l.line.contains("enum E")));
        // Minimal should NOT include trait or mod
        assert!(!result.iter().any(|l| l.line.contains("trait T")));
        assert!(!result.iter().any(|l| l.line.contains("mod m")));
    }

    #[test]
    fn test_rust_detailed_includes_use_and_const() {
        let code = "use std::io;\nconst X: i32 = 1;\nstatic Y: i32 = 2;\nfn f() {}";
        let result = AstSummarizer::extract_summary(code, Some("Rust"), SummaryLevel::Detailed);
        assert!(result.iter().any(|l| l.line.contains("use std::io")));
        assert!(result.iter().any(|l| l.line.contains("const X")));
        assert!(result.iter().any(|l| l.line.contains("static Y")));
    }

    #[test]
    fn test_python_minimal_level() {
        let code = "import os\ndef foo():\n    pass\nclass Bar:\n    pass";
        let result = AstSummarizer::extract_summary(code, Some("Python"), SummaryLevel::Minimal);
        assert!(result.iter().any(|l| l.line.contains("def foo")));
        assert!(result.iter().any(|l| l.line.contains("class Bar")));
        // Minimal should NOT include imports
        assert!(!result.iter().any(|l| l.line.contains("import os")));
    }

    #[test]
    fn test_javascript_detects_classes_and_functions() {
        let code = "function hello() {}\nclass World {}\nconst x = () => {};";
        let result =
            AstSummarizer::extract_summary(code, Some("JavaScript"), SummaryLevel::Standard);
        assert!(result.iter().any(|l| l.line.contains("function hello")));
        assert!(result.iter().any(|l| l.line.contains("class World")));
    }

    #[test]
    fn test_typescript_detects_interfaces() {
        let code = "interface User { name: string; }\nfunction greet(u: User) {}";
        let result =
            AstSummarizer::extract_summary(code, Some("TypeScript"), SummaryLevel::Standard);
        assert!(result.iter().any(|l| l.line.contains("interface User")));
        assert!(result.iter().any(|l| l.line.contains("function greet")));
    }

    #[test]
    fn test_jsx_uses_javascript_parser() {
        let code = "function App() { return <div/>; }";
        let result = AstSummarizer::extract_summary(code, Some("JSX"), SummaryLevel::Standard);
        assert!(result.iter().any(|l| l.line.contains("function App")));
    }

    #[test]
    fn test_tsx_uses_typescript_parser() {
        // TSX is routed through the TypeScript parser, so pure TS syntax works
        let code = "function App(): string { return 'hello'; }";
        let result = AstSummarizer::extract_summary(code, Some("TSX"), SummaryLevel::Standard);
        assert!(result.iter().any(|l| l.line.contains("function App")));
    }

    #[test]
    fn test_very_long_single_line() {
        let code = format!("fn {}() {{}}", "a".repeat(10_000));
        let result = AstSummarizer::extract_summary(&code, Some("Rust"), SummaryLevel::Standard);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_whitespace_only_input() {
        let result =
            AstSummarizer::extract_summary("   \n\n\t\t\n  ", Some("Rust"), SummaryLevel::Standard);
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_with_timeout_returns_tree_for_valid_code() {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .unwrap();
        let tree = AstSummarizer::parse_with_timeout(&mut parser, "fn main() {}");
        assert!(tree.is_some(), "Valid code should parse successfully");
        let tree = tree.unwrap();
        assert_eq!(tree.root_node().kind(), "source_file");
    }

    #[test]
    fn test_parse_with_timeout_handles_empty_content() {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .unwrap();
        let tree = AstSummarizer::parse_with_timeout(&mut parser, "");
        assert!(tree.is_some(), "Empty content should still produce a tree");
    }

    #[test]
    fn test_parse_with_timeout_handles_invalid_syntax() {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .unwrap();
        // Tree-sitter produces partial trees for invalid syntax (doesn't return None)
        let tree = AstSummarizer::parse_with_timeout(&mut parser, "{{{{{{");
        assert!(tree.is_some());
        assert!(tree.unwrap().root_node().has_error());
    }

    #[test]
    fn test_parse_with_timeout_all_languages() {
        let languages: Vec<(&str, tree_sitter::Language)> = vec![
            ("Rust", tree_sitter_rust::LANGUAGE.into()),
            ("Python", tree_sitter_python::LANGUAGE.into()),
            ("JavaScript", tree_sitter_javascript::LANGUAGE.into()),
            (
                "TypeScript",
                tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            ),
        ];
        let snippets = [
            ("Rust", "fn hello() {} struct S {}"),
            ("Python", "def hello():\n    pass"),
            ("JavaScript", "function hello() {}"),
            ("TypeScript", "function hello(): void {}"),
        ];
        for ((name, lang), (_, snippet)) in languages.iter().zip(snippets.iter()) {
            let mut parser = Parser::new();
            parser.set_language(lang).unwrap();
            let tree = AstSummarizer::parse_with_timeout(&mut parser, snippet);
            assert!(
                tree.is_some(),
                "parse_with_timeout should succeed for {name}"
            );
        }
    }

    #[test]
    fn test_parse_with_zero_timeout_aborts() {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .unwrap();
        // Generate a large input so tree-sitter invokes the progress callback
        // before finishing. A zero-duration deadline is already in the past,
        // so the callback should return Break and cancel parsing.
        let mut large_input = String::new();
        for i in 0..1000 {
            writeln!(large_input, "fn func_{i}(x: i32) -> i32 {{ x + {i} }}").unwrap();
        }
        let tree = AstSummarizer::parse_with_deadline(&mut parser, &large_input, Duration::ZERO);
        assert!(
            tree.is_none(),
            "Zero timeout should abort parsing and return None"
        );
    }
}
