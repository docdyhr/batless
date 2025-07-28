//! Token extraction functionality for batless
//!
//! This module provides token extraction from source code for AI processing
//! and analysis. It supports multiple tokenization strategies and can be
//! extended for language-specific tokenization.

use std::collections::HashSet;

/// Token extractor for source code
pub struct TokenExtractor;

impl TokenExtractor {
    /// Extract tokens from content for AI processing
    pub fn extract_tokens(content: &str, file_path: &str) -> Vec<String> {
        let mut tokens = Vec::new();

        // Determine tokenization strategy based on file type
        let strategy = Self::determine_tokenization_strategy(file_path);

        match strategy {
            TokenizationStrategy::Programming => {
                tokens.extend(Self::extract_programming_tokens(content));
            }
            TokenizationStrategy::Markup => {
                tokens.extend(Self::extract_markup_tokens(content));
            }
            TokenizationStrategy::Data => {
                tokens.extend(Self::extract_data_tokens(content));
            }
            TokenizationStrategy::Text => {
                tokens.extend(Self::extract_text_tokens(content));
            }
        }

        // Post-process tokens
        Self::post_process_tokens(tokens)
    }

    /// Extract tokens specifically for programming languages
    fn extract_programming_tokens(content: &str) -> Vec<String> {
        let mut tokens = Vec::new();

        for line in content.lines() {
            // Skip empty lines and comments
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with("#") {
                continue;
            }

            // Extract identifiers, keywords, and symbols
            tokens.extend(Self::tokenize_programming_line(line));
        }

        tokens
    }

    /// Extract tokens from markup languages (HTML, XML, Markdown)
    fn extract_markup_tokens(content: &str) -> Vec<String> {
        let mut tokens = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            // Extract tag names, attributes, and text content
            tokens.extend(Self::tokenize_markup_line(line));
        }

        tokens
    }

    /// Extract tokens from data files (JSON, YAML, TOML)
    fn extract_data_tokens(content: &str) -> Vec<String> {
        let mut tokens = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("#") {
                continue;
            }

            // Extract keys, values, and structure
            tokens.extend(Self::tokenize_data_line(line));
        }

        tokens
    }

    /// Extract tokens from plain text
    fn extract_text_tokens(content: &str) -> Vec<String> {
        content.split_whitespace().map(|s| s.to_string()).collect()
    }

    /// Tokenize a single programming language line
    fn tokenize_programming_line(line: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        let mut in_string = false;
        let mut string_char = '"';
        let mut escaped = false;

        for ch in line.chars() {
            if escaped {
                escaped = false;
                current_token.push(ch);
                continue;
            }

            if ch == '\\' && in_string {
                escaped = true;
                current_token.push(ch);
                continue;
            }

            if in_string {
                current_token.push(ch);
                if ch == string_char {
                    in_string = false;
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                }
                continue;
            }

            if ch == '"' || ch == '\'' {
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
                in_string = true;
                string_char = ch;
                current_token.push(ch);
                continue;
            }

            if ch.is_alphanumeric() || ch == '_' {
                current_token.push(ch);
            } else {
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }

                // Include significant punctuation as tokens
                if "(){}[]<>;,.:!?=+-*/&|^~%".contains(ch) {
                    tokens.push(ch.to_string());
                }
            }
        }

        if !current_token.is_empty() {
            tokens.push(current_token);
        }

        tokens
    }

    /// Tokenize a single markup language line
    fn tokenize_markup_line(line: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        let mut in_tag = false;

        for ch in line.chars() {
            if ch == '<' {
                if !current_token.is_empty() {
                    tokens.push(current_token.trim().to_string());
                    current_token.clear();
                }
                in_tag = true;
                current_token.push(ch);
            } else if ch == '>' && in_tag {
                current_token.push(ch);
                tokens.push(current_token.clone());
                current_token.clear();
                in_tag = false;
            } else if ch.is_whitespace() && !in_tag {
                if !current_token.is_empty() {
                    tokens.push(current_token.trim().to_string());
                    current_token.clear();
                }
            } else {
                current_token.push(ch);
            }
        }

        if !current_token.is_empty() {
            tokens.push(current_token.trim().to_string());
        }

        tokens.into_iter().filter(|t| !t.is_empty()).collect()
    }

    /// Tokenize a single data format line
    fn tokenize_data_line(line: &str) -> Vec<String> {
        let mut tokens = Vec::new();

        // Simple key-value extraction
        if line.contains(':') {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                let key = parts[0].trim().trim_matches('"');
                let value = parts[1].trim().trim_matches('"').trim_matches(',');

                if !key.is_empty() {
                    tokens.push(key.to_string());
                }
                if !value.is_empty() && value != "{" && value != "[" {
                    tokens.push(value.to_string());
                }
            }
        } else {
            // Fallback to word splitting
            tokens.extend(
                line.split_whitespace()
                    .map(|s| s.trim_matches('"').trim_matches(',').to_string())
                    .filter(|s| !s.is_empty()),
            );
        }

        tokens
    }

    /// Determine the appropriate tokenization strategy
    fn determine_tokenization_strategy(file_path: &str) -> TokenizationStrategy {
        let path_lower = file_path.to_lowercase();

        // Programming languages
        if path_lower.ends_with(".rs")
            || path_lower.ends_with(".py")
            || path_lower.ends_with(".js")
            || path_lower.ends_with(".ts")
            || path_lower.ends_with(".java")
            || path_lower.ends_with(".cpp")
            || path_lower.ends_with(".c")
            || path_lower.ends_with(".h")
            || path_lower.ends_with(".go")
            || path_lower.ends_with(".rb")
            || path_lower.ends_with(".php")
            || path_lower.ends_with(".swift")
            || path_lower.ends_with(".kt")
            || path_lower.ends_with(".scala")
            || path_lower.ends_with(".hs")
            || path_lower.ends_with(".ml")
            || path_lower.ends_with(".fs")
            || path_lower.ends_with(".clj")
            || path_lower.ends_with(".ex")
            || path_lower.ends_with(".erl")
        {
            return TokenizationStrategy::Programming;
        }

        // Markup languages
        if path_lower.ends_with(".html")
            || path_lower.ends_with(".xml")
            || path_lower.ends_with(".md")
            || path_lower.ends_with(".markdown")
            || path_lower.ends_with(".rst")
            || path_lower.ends_with(".tex")
        {
            return TokenizationStrategy::Markup;
        }

        // Data formats
        if path_lower.ends_with(".json")
            || path_lower.ends_with(".yaml")
            || path_lower.ends_with(".yml")
            || path_lower.ends_with(".toml")
            || path_lower.ends_with(".ini")
            || path_lower.ends_with(".cfg")
            || path_lower.ends_with(".conf")
            || path_lower.ends_with(".csv")
        {
            return TokenizationStrategy::Data;
        }

        // Default to text
        TokenizationStrategy::Text
    }

    /// Post-process tokens to clean and filter them
    fn post_process_tokens(mut tokens: Vec<String>) -> Vec<String> {
        // Remove empty tokens
        tokens.retain(|t| !t.trim().is_empty());

        // Filter out very short tokens (single characters) except important ones
        tokens.retain(|t| t.len() > 1 || "(){}[]<>;,.:!?=+-*/&|^~%".contains(t));

        // Remove duplicates while preserving order
        let mut seen = HashSet::new();
        tokens.retain(|t| seen.insert(t.clone()));

        // Sort for consistency
        tokens.sort();

        // Limit token count to prevent overflow
        if tokens.len() > 1000 {
            tokens.truncate(1000);
        }

        tokens
    }

    /// Extract keywords specific to programming languages
    pub fn extract_keywords(content: &str, language: Option<&str>) -> Vec<String> {
        let keywords: Vec<String> = match language {
            Some("Rust") => vec![
                "fn", "let", "mut", "const", "static", "struct", "enum", "impl", "trait", "pub",
                "use", "mod", "crate", "super", "self", "Self", "match", "if", "else", "while",
                "for", "loop", "break", "continue", "return", "async", "await",
            ],
            Some("Python") => vec![
                "def", "class", "import", "from", "as", "if", "elif", "else", "for", "while",
                "try", "except", "finally", "with", "lambda", "return", "yield", "async", "await",
            ],
            Some("JavaScript") | Some("TypeScript") => vec![
                "function", "class", "const", "let", "var", "if", "else", "for", "while", "try",
                "catch", "finally", "return", "async", "await", "import", "export",
            ],
            _ => vec![],
        }
        .into_iter()
        .map(|s| s.to_string())
        .collect();

        let mut found_keywords = Vec::new();
        for keyword in keywords {
            if content.contains(&keyword) {
                found_keywords.push(keyword);
            }
        }

        found_keywords
    }

    /// Get token statistics
    pub fn get_token_stats(tokens: &[String]) -> TokenStats {
        let unique_tokens = tokens.iter().collect::<HashSet<_>>().len();
        let avg_token_length = if tokens.is_empty() {
            0.0
        } else {
            tokens.iter().map(|t| t.len()).sum::<usize>() as f64 / tokens.len() as f64
        };

        TokenStats {
            total_tokens: tokens.len(),
            unique_tokens,
            avg_token_length,
            longest_token: tokens.iter().map(|t| t.len()).max().unwrap_or(0),
            shortest_token: tokens.iter().map(|t| t.len()).min().unwrap_or(0),
        }
    }
}

/// Tokenization strategy based on file type
#[derive(Debug, Clone, PartialEq)]
enum TokenizationStrategy {
    Programming,
    Markup,
    Data,
    Text,
}

/// Statistics about the tokenization process
#[derive(Debug, Clone)]
pub struct TokenStats {
    pub total_tokens: usize,
    pub unique_tokens: usize,
    pub avg_token_length: f64,
    pub longest_token: usize,
    pub shortest_token: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_programming_tokens() {
        let content = "fn main() {\n    println!(\"Hello, world!\");\n}";
        let tokens = TokenExtractor::extract_programming_tokens(content);

        assert!(tokens.contains(&"fn".to_string()));
        assert!(tokens.contains(&"main".to_string()));
        assert!(tokens.contains(&"println".to_string()));
        assert!(tokens.contains(&"(".to_string()));
        assert!(tokens.contains(&")".to_string()));
    }

    #[test]
    fn test_extract_markup_tokens() {
        let content = "<html><body><h1>Title</h1></body></html>";
        let tokens = TokenExtractor::extract_markup_tokens(content);

        assert!(tokens.contains(&"<html>".to_string()));
        assert!(tokens.contains(&"<body>".to_string()));
        assert!(tokens.contains(&"<h1>".to_string()));
        assert!(tokens.contains(&"Title".to_string()));
    }

    #[test]
    fn test_extract_data_tokens() {
        let content = "\"name\": \"test\"\n\"value\": 42";
        let tokens = TokenExtractor::extract_data_tokens(content);

        assert!(tokens.contains(&"name".to_string()));
        assert!(tokens.contains(&"test".to_string()));
        assert!(tokens.contains(&"value".to_string()));
        assert!(tokens.contains(&"42".to_string()));
    }

    #[test]
    fn test_determine_strategy() {
        assert_eq!(
            TokenExtractor::determine_tokenization_strategy("test.rs"),
            TokenizationStrategy::Programming
        );
        assert_eq!(
            TokenExtractor::determine_tokenization_strategy("test.html"),
            TokenizationStrategy::Markup
        );
        assert_eq!(
            TokenExtractor::determine_tokenization_strategy("test.json"),
            TokenizationStrategy::Data
        );
        assert_eq!(
            TokenExtractor::determine_tokenization_strategy("test.txt"),
            TokenizationStrategy::Text
        );
    }

    #[test]
    fn test_post_process_tokens() {
        let tokens = vec![
            "".to_string(),
            "a".to_string(),
            "test".to_string(),
            "test".to_string(), // duplicate
            "another".to_string(),
            "(".to_string(), // important single char
        ];

        let processed = TokenExtractor::post_process_tokens(tokens);

        assert!(!processed.contains(&String::new()));
        assert!(!processed.contains(&"a".to_string()));
        assert!(processed.contains(&"(".to_string()));
        assert_eq!(processed.iter().filter(|&t| t == "test").count(), 1);
    }

    #[test]
    fn test_extract_keywords() {
        let content = "fn main() { let x = 5; }";
        let keywords = TokenExtractor::extract_keywords(content, Some("Rust"));

        assert!(keywords.contains(&"fn".to_string()));
        assert!(keywords.contains(&"let".to_string()));
        assert!(!keywords.contains(&"main".to_string())); // not a keyword
    }

    #[test]
    fn test_token_stats() {
        let tokens = vec![
            "hello".to_string(),
            "world".to_string(),
            "test".to_string(),
            "a".to_string(),
        ];

        let stats = TokenExtractor::get_token_stats(&tokens);

        assert_eq!(stats.total_tokens, 4);
        assert_eq!(stats.unique_tokens, 4);
        assert_eq!(stats.longest_token, 5); // "hello" or "world"
        assert_eq!(stats.shortest_token, 1); // "a"
    }

    #[test]
    fn test_string_handling() {
        let content = r#"let message = "Hello, world!";"#;
        let tokens = TokenExtractor::extract_programming_tokens(content);

        assert!(tokens.contains(&"let".to_string()));
        assert!(tokens.contains(&"message".to_string()));
        assert!(tokens.contains(&"\"Hello, world!\"".to_string()));
    }

    #[test]
    fn test_comment_filtering() {
        let content = "// This is a comment\nfn main() {}";
        let tokens = TokenExtractor::extract_programming_tokens(content);

        assert!(tokens.contains(&"fn".to_string()));
        assert!(tokens.contains(&"main".to_string()));
        // Should not contain comment content
        assert!(!tokens.iter().any(|t| t.contains("comment")));
    }
}
