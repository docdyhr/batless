//! AST-based code summarization using tree-sitter
//!
//! This module provides robust, syntax-aware code summarization by parsing
//! the source code into an Abstract Syntax Tree (AST) and extracting
//! relevant nodes based on the summary level.

use crate::summary::SummaryLevel;
// use streaming_iterator::StreamingIterator; // Removed
use tree_sitter::{Parser, Query, QueryCursor, StreamingIterator}; // Added StreamingIterator

/// AST-based summary extractor
pub struct AstSummarizer;

impl AstSummarizer {
    /// Extract a summary of important code structures using AST parsing
    pub fn extract_summary(
        content: &str,
        language: Option<&str>,
        level: SummaryLevel,
    ) -> Vec<String> {
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

    fn summarize_rust(content: &str, level: SummaryLevel) -> Vec<String> {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .expect("Error loading Rust grammar");

        let tree = parser.parse(content, None).unwrap();
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

        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&query, root_node, content.as_bytes());

        let lines: Vec<&str> = content.lines().collect();
        // Use BTreeSet to automatically keep line numbers sorted and unique
        let mut line_indices = std::collections::BTreeSet::new();

        while let Some(m) = matches.next() {
            for capture in m.captures {
                let start_line = capture.node.start_position().row;
                line_indices.insert(start_line);
            }
        }

        let mut summary_lines = Vec::new();
        for idx in line_indices {
            if let Some(&line) = lines.get(idx) {
                summary_lines.push(line.to_owned());
            }
        }

        summary_lines
    }

    fn summarize_python(content: &str, level: SummaryLevel) -> Vec<String> {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_python::LANGUAGE.into())
            .expect("Error loading Python grammar");

        let tree = parser.parse(content, None).unwrap();
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

        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&query, root_node, content.as_bytes());

        let lines: Vec<&str> = content.lines().collect();
        let mut line_indices = std::collections::BTreeSet::new();

        while let Some(m) = matches.next() {
            for capture in m.captures {
                let start_line = capture.node.start_position().row;
                line_indices.insert(start_line);
            }
        }

        let mut summary_lines = Vec::new();
        for idx in line_indices {
            if let Some(&line) = lines.get(idx) {
                summary_lines.push(line.to_owned());
            }
        }

        summary_lines
    }

    fn summarize_javascript(content: &str, level: SummaryLevel) -> Vec<String> {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_javascript::LANGUAGE.into())
            .expect("Error loading JavaScript grammar");

        let tree = parser.parse(content, None).unwrap();
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

        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&query, root_node, content.as_bytes());

        let lines: Vec<&str> = content.lines().collect();
        let mut line_indices = std::collections::BTreeSet::new();

        while let Some(m) = matches.next() {
            for capture in m.captures {
                let start_line = capture.node.start_position().row;
                line_indices.insert(start_line);
            }
        }

        let mut summary_lines = Vec::new();
        for idx in line_indices {
            if let Some(&line) = lines.get(idx) {
                summary_lines.push(line.to_owned());
            }
        }

        summary_lines
    }

    fn summarize_typescript(content: &str, level: SummaryLevel) -> Vec<String> {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .expect("Error loading TypeScript grammar");

        let tree = parser.parse(content, None).unwrap();
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

        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&query, root_node, content.as_bytes());

        let lines: Vec<&str> = content.lines().collect();
        let mut line_indices = std::collections::BTreeSet::new();

        while let Some(m) = matches.next() {
            for capture in m.captures {
                let start_line = capture.node.start_position().row;
                line_indices.insert(start_line);
            }
        }

        let mut summary_lines = Vec::new();
        for idx in line_indices {
            if let Some(&line) = lines.get(idx) {
                summary_lines.push(line.to_owned());
            }
        }

        summary_lines
    }
}
