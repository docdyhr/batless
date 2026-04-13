//! Raw AST output formatter using tree-sitter
//!
//! Serializes the full tree-sitter parse tree as JSON for AI agents that
//! need deep structural analysis beyond what index mode provides.

use crate::config::BatlessConfig;
use crate::error::BatlessResult;
use crate::file_info::FileInfo;
use crate::formatter::OutputMode;
use crate::formatters::Formatter;
use serde_json::{json, Value};
use tree_sitter::{Node, Parser};

/// Maximum node depth to prevent extreme recursion on pathological inputs.
const MAX_DEPTH: usize = 64;

pub struct AstFormatter;

impl AstFormatter {
    fn node_to_json(node: Node, source: &[u8], depth: usize) -> Value {
        let start = node.start_position();
        let end = node.end_position();

        let mut obj = json!({
            "type": node.kind(),
            "start": [start.row, start.column],
            "end": [end.row, end.column],
        });

        if node.is_error() {
            obj["is_error"] = json!(true);
        }
        if node.is_missing() {
            obj["is_missing"] = json!(true);
        }

        let child_count = node.child_count();
        if child_count == 0 {
            // Leaf node: include text if reasonably sized
            if let Ok(text) = std::str::from_utf8(&source[node.start_byte()..node.end_byte()]) {
                if text.len() <= 256 {
                    obj["text"] = json!(text);
                }
            }
        } else if depth < MAX_DEPTH {
            let children: Vec<Value> = (0..child_count as u32)
                .filter_map(|i| node.child(i))
                .map(|child| Self::node_to_json(child, source, depth + 1))
                .collect();
            obj["children"] = json!(children);
        } else {
            obj["truncated_depth"] = json!(true);
        }

        obj
    }

    fn parse_to_tree(
        content: &str,
        language: Option<&str>,
    ) -> Option<(tree_sitter::Tree, &'static str)> {
        let mut parser = Parser::new();
        match language {
            Some("Rust") => {
                parser
                    .set_language(&tree_sitter_rust::LANGUAGE.into())
                    .ok()?;
                let tree = parser.parse(content, None)?;
                Some((tree, "tree-sitter-rust"))
            }
            Some("Python") => {
                parser
                    .set_language(&tree_sitter_python::LANGUAGE.into())
                    .ok()?;
                let tree = parser.parse(content, None)?;
                Some((tree, "tree-sitter-python"))
            }
            Some("JavaScript" | "JSX") => {
                parser
                    .set_language(&tree_sitter_javascript::LANGUAGE.into())
                    .ok()?;
                let tree = parser.parse(content, None)?;
                Some((tree, "tree-sitter-javascript"))
            }
            Some("TypeScript") => {
                parser
                    .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
                    .ok()?;
                let tree = parser.parse(content, None)?;
                Some((tree, "tree-sitter-typescript"))
            }
            Some("TSX") => {
                parser
                    .set_language(&tree_sitter_typescript::LANGUAGE_TSX.into())
                    .ok()?;
                let tree = parser.parse(content, None)?;
                Some((tree, "tree-sitter-tsx"))
            }
            _ => None,
        }
    }
}

impl Formatter for AstFormatter {
    fn format(
        &self,
        file_info: &FileInfo,
        file_path: &str,
        config: &BatlessConfig,
    ) -> BatlessResult<String> {
        let language = file_info.language.as_deref();
        // Use original_lines if available so comment/blank stripping doesn't
        // alter the byte offsets that tree-sitter reports in its AST nodes.
        let content = file_info
            .original_lines
            .as_deref()
            .unwrap_or(&file_info.lines)
            .join("\n");
        let source = content.as_bytes();

        let (root_value, parser_name) = match Self::parse_to_tree(&content, language) {
            Some((tree, name)) => {
                let root = tree.root_node();
                (Self::node_to_json(root, source, 0), name)
            }
            None => (Value::Null, "none"),
        };

        let output = json!({
            "file": file_path,
            "language": language,
            "mode": "ast",
            "parser": parser_name,
            "total_lines": file_info.total_lines,
            "total_bytes": file_info.total_bytes,
            "root": root_value,
        });

        if config.pretty_json {
            Ok(serde_json::to_string_pretty(&output)?)
        } else {
            Ok(serde_json::to_string(&output)?)
        }
    }

    fn output_mode(&self) -> OutputMode {
        OutputMode::Ast
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_info::FileInfo;

    fn make_file_info(lines: Vec<String>, language: Option<String>) -> FileInfo {
        let total = lines.len();
        let bytes: usize = lines.iter().map(|l| l.len() + 1).sum();
        FileInfo::with_metadata(total, bytes, language, "UTF-8".to_string()).with_lines(lines)
    }

    #[test]
    fn test_ast_rust() {
        let fi = make_file_info(vec!["fn main() {}".to_string()], Some("Rust".to_string()));
        let config = BatlessConfig::default();
        let out = AstFormatter.format(&fi, "test.rs", &config).unwrap();
        let json: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(json["mode"], "ast");
        assert_eq!(json["parser"], "tree-sitter-rust");
        assert!(json["root"].is_object());
        assert_eq!(json["root"]["type"], "source_file");
    }

    #[test]
    fn test_ast_unsupported_language() {
        let fi = make_file_info(vec!["echo hello".to_string()], Some("Shell".to_string()));
        let config = BatlessConfig::default();
        let out = AstFormatter.format(&fi, "test.sh", &config).unwrap();
        let json: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(json["parser"], "none");
        assert!(json["root"].is_null());
    }

    #[test]
    fn test_ast_python() {
        let fi = make_file_info(
            vec!["def hello(): pass".to_string()],
            Some("Python".to_string()),
        );
        let config = BatlessConfig::default();
        let out = AstFormatter.format(&fi, "test.py", &config).unwrap();
        let json: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(json["parser"], "tree-sitter-python");
        assert_eq!(json["root"]["type"], "module");
    }
}
