//! Semantic boundary finding for streaming chunk splitting
//!
//! Uses tree-sitter to locate the start lines of top-level declarations
//! (functions, classes, structs, impls, etc.) so that streaming chunks
//! can end at natural code boundaries rather than mid-function.

use std::ops::ControlFlow;
use std::time::{Duration, Instant};
use tree_sitter::{ParseOptions, Parser};

/// Maximum time allowed for a tree-sitter parse used for boundary detection.
const BOUNDARY_PARSE_TIMEOUT: Duration = Duration::from_millis(1000);

/// Finds top-level declaration boundaries in source code using tree-sitter.
pub struct SemanticBoundaryFinder;

impl SemanticBoundaryFinder {
    /// Return a sorted, deduplicated list of **0-based** line numbers that
    /// start a top-level declaration in `content`.
    ///
    /// Supports Rust, Python, JavaScript/JSX, and TypeScript/TSX.
    /// Returns an empty `Vec` for unsupported languages or when parsing fails.
    pub fn find_boundaries(content: &str, language: Option<&str>) -> Vec<usize> {
        let ts_language: tree_sitter::Language = match language {
            Some("Rust") => tree_sitter_rust::LANGUAGE.into(),
            Some("Python") => tree_sitter_python::LANGUAGE.into(),
            Some("JavaScript" | "JSX") => tree_sitter_javascript::LANGUAGE.into(),
            Some("TypeScript" | "TSX") => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            _ => return Vec::new(),
        };

        let mut parser = Parser::new();
        if parser.set_language(&ts_language).is_err() {
            return Vec::new();
        }

        let deadline = Instant::now() + BOUNDARY_PARSE_TIMEOUT;
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
        let Some(tree) = parser.parse_with_options(
            &mut |i, _| {
                if i < len {
                    &bytes[i..]
                } else {
                    &[]
                }
            },
            None,
            Some(options.reborrow()),
        ) else {
            return Vec::new();
        };

        let root = tree.root_node();
        let mut cursor = root.walk();
        let mut boundaries: Vec<usize> = root
            .children(&mut cursor)
            .filter(tree_sitter::Node::is_named)
            .map(|n| n.start_position().row)
            .collect();

        boundaries.sort_unstable();
        boundaries.dedup();
        boundaries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_boundaries() {
        let content = "fn foo() {}\nfn bar() {}\nstruct Baz {}";
        let boundaries = SemanticBoundaryFinder::find_boundaries(content, Some("Rust"));
        assert_eq!(boundaries, vec![0, 1, 2]);
    }

    #[test]
    fn test_unsupported_language_returns_empty() {
        let boundaries = SemanticBoundaryFinder::find_boundaries("hello world", Some("PlainText"));
        assert!(boundaries.is_empty());
    }

    #[test]
    fn test_none_language_returns_empty() {
        let boundaries = SemanticBoundaryFinder::find_boundaries("fn foo() {}", None);
        assert!(boundaries.is_empty());
    }
}
