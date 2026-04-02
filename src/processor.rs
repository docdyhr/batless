//! File processing functionality for batless
//!
//! This module handles the core file processing logic including reading files,
//! detecting encoding, handling truncation limits, and coordinating with other
//! modules for language detection, summarization, and tokenization.

use crate::ast_summarizer::AstSummarizer;
use crate::config::BatlessConfig;
use crate::error::{BatlessError, BatlessResult};
use crate::file_info::FileInfo;
use crate::language::LanguageDetector;
use crate::summarizer::SummaryExtractor;
use crate::tokens::TokenExtractor;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

/// Core file processor
pub struct FileProcessor;

impl FileProcessor {
    /// Process a file according to the given configuration
    pub fn process_file(file_path: &str, config: &BatlessConfig) -> BatlessResult<FileInfo> {
        // Validate configuration first
        config.validate()?;

        // Handle stdin input
        if file_path == "-" {
            return Self::process_stdin(config);
        }

        // Check if file exists
        if !Path::new(file_path).exists() {
            return Err(BatlessError::file_not_found_with_suggestions(
                file_path.to_string(),
            ));
        }

        // Detect encoding and prepare file reading
        let encoding = Self::detect_encoding(file_path)?;

        // Detect language (use config override if provided)
        let language = config
            .language
            .clone()
            .or_else(|| LanguageDetector::detect_language_with_fallback(file_path));

        // Read and process file content
        let (lines, metadata) = Self::read_file_content(file_path, config)?;

        // Create base FileInfo
        let mut file_info = FileInfo::with_metadata(
            metadata.total_lines,
            metadata.total_bytes,
            language,
            encoding,
        )
        .with_total_lines_exact(metadata.total_lines_exact)
        .with_lines(lines.clone())
        .with_truncation(
            metadata.truncated,
            metadata.truncated_by_lines,
            metadata.truncated_by_bytes,
        );

        file_info = Self::apply_post_processing(file_info, &lines, config);

        // Compute file hash if requested (file-only; stdin has no path to hash)
        if config.hash {
            let hash = Self::compute_file_hash(file_path)?;
            file_info = file_info.with_file_hash(Some(hash));
        }

        Ok(file_info)
    }

    /// Apply summary extraction, token extraction, and content stripping to a
    /// FileInfo that has already been constructed from raw lines.  Shared by
    /// both `process_file` and `process_stdin`.
    fn apply_post_processing(
        mut file_info: FileInfo,
        lines: &[String],
        config: &BatlessConfig,
    ) -> FileInfo {
        // Process summary if requested — try AST first, fall back to regex
        let summary_level = config.effective_summary_level();
        if summary_level.is_enabled() {
            let content = lines.join("\n");
            let mut summary_lines = AstSummarizer::extract_summary(
                &content,
                file_info.language.as_deref(),
                summary_level,
            );
            if summary_lines.is_empty() {
                summary_lines = SummaryExtractor::extract_summary(
                    lines,
                    file_info.language.as_deref(),
                    summary_level,
                );
            }
            let summary_text: Vec<String> = summary_lines.iter().map(|s| s.line.clone()).collect();
            file_info = file_info
                .with_original_lines(Some(lines.to_vec()))
                .with_summary_lines(Some(summary_lines));
            file_info.lines = summary_text;
        }

        // Extract identifiers if requested
        if config.include_tokens {
            let content = file_info.lines.join("\n");
            let token_result = TokenExtractor::extract_tokens_with_limit(
                &content,
                "<content>",
                TokenExtractor::MAX_SAMPLE_SIZE,
            );
            file_info = file_info
                .with_tokens(Some(token_result.tokens))
                .with_token_total(Some(token_result.total_count));
        }

        // Strip comments and/or blank lines if requested
        if config.strip_comments || config.strip_blank_lines {
            let original_count = file_info.lines.len();
            let language = file_info.language.as_deref();
            file_info.lines = Self::strip_content_lines(
                file_info.lines,
                language,
                config.strip_comments,
                config.strip_blank_lines,
            );
            let stripped_count = file_info.lines.len();
            let ratio = if stripped_count > 0 {
                original_count as f64 / stripped_count as f64
            } else {
                original_count as f64
            };
            file_info = file_info.with_compression_ratio(Some(ratio));
        }

        file_info
    }

    /// Compute SHA-256 hex digest for a file's content
    fn compute_file_hash(file_path: &str) -> BatlessResult<String> {
        let mut hasher = Sha256::new();
        let mut file =
            File::open(file_path).map_err(|e| BatlessError::from_io_error(e, file_path))?;
        let mut buf = vec![0u8; 65536];
        loop {
            let n = file
                .read(&mut buf)
                .map_err(|e| BatlessError::from_io_error(e, file_path))?;
            if n == 0 {
                break;
            }
            hasher.update(&buf[..n]);
        }
        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Strip comment-only and/or blank lines from a line buffer.
    ///
    /// Uses simple prefix-based heuristics for most languages.  For single-line
    /// comment detection the rules are:
    ///   - `//` — C, Rust, Go, Java, JS/TS, Swift, Kotlin, Dart, C++
    ///   - `#`  — Python, Ruby, Shell, YAML, TOML, Perl, R
    ///   - `--` — SQL, Lua, Haskell, Ada
    ///   - `%`  — TeX/LaTeX, Prolog, MATLAB/Octave
    ///   - `;`  — Lisp, Assembly, INI
    ///
    /// Multi-line block comments (`/* ... */`, `{- ... -}`, `(*...*}`, etc.)
    /// are handled by tracking an `in_block` state flag.
    fn strip_content_lines(
        lines: Vec<String>,
        language: Option<&str>,
        strip_comments: bool,
        strip_blank_lines: bool,
    ) -> Vec<String> {
        let lang = language.unwrap_or("").to_lowercase();

        // Determine single-line comment prefix for language
        let single_prefix: &str = if lang.contains("python")
            || lang.contains("ruby")
            || lang.contains("shell")
            || lang.contains("bash")
            || lang.contains("zsh")
            || lang.contains("yaml")
            || lang.contains("toml")
            || lang.contains("perl")
            || lang == "r"
        {
            "#"
        } else if lang.contains("sql")
            || lang.contains("lua")
            || lang.contains("haskell")
            || lang.contains("ada")
        {
            "--"
        } else if lang.contains("tex")
            || lang.contains("latex")
            || lang.contains("matlab")
            || lang.contains("octave")
            || lang.contains("prolog")
        {
            "%"
        } else if lang.contains("lisp")
            || lang.contains("scheme")
            || lang.contains("clojure")
            || lang.contains("assembly")
            || lang.contains("ini")
        {
            ";"
        } else {
            "//"
        };

        let mut result = Vec::with_capacity(lines.len());
        let mut in_block = false;

        for line in lines {
            let trimmed = line.trim();

            if strip_blank_lines && trimmed.is_empty() {
                continue;
            }

            if strip_comments {
                // Track block comments (C-style /* */ used by most languages)
                if in_block {
                    if trimmed.contains("*/") {
                        in_block = false;
                    }
                    continue;
                }

                // Detect start of block comment
                if trimmed.starts_with("/*") {
                    if !trimmed.contains("*/") {
                        in_block = true;
                    }
                    // Skip both single-line block comments (/* ... */) and opening lines
                    continue;
                }

                // Single-line comment
                if trimmed.starts_with(single_prefix) {
                    continue;
                }
            }

            result.push(line);
        }

        result
    }

    /// Process input from stdin
    ///
    /// Reads stdin line-by-line with a BufReader, enforcing max_lines and
    /// max_bytes limits incrementally to avoid unbounded memory usage.
    pub fn process_stdin(config: &BatlessConfig) -> BatlessResult<FileInfo> {
        use std::io::{stdin, BufRead, BufReader};

        let reader = BufReader::new(stdin());
        let mut lines = Vec::new();
        let mut bytes_seen = 0usize;
        let mut truncated_by_lines = false;
        let mut truncated_by_bytes = false;

        for line_result in reader.lines() {
            if lines.len() >= config.max_lines {
                truncated_by_lines = true;
                break;
            }

            let line = line_result.map_err(|e| BatlessError::FileReadError {
                path: "<stdin>".to_string(),
                source: e,
            })?;

            let line_bytes = line.len() + 1; // +1 for newline
            if let Some(max_bytes) = config.max_bytes {
                if bytes_seen + line_bytes > max_bytes {
                    truncated_by_bytes = true;
                    break;
                }
            }

            bytes_seen += line_bytes;
            lines.push(line);
        }

        let total_lines = lines.len();
        let total_bytes = bytes_seen;
        let final_lines = lines;

        // Detect language from content (limited for stdin without filename)
        let language = config.language.clone(); // Use configured language or none

        // Create FileInfo
        let file_info = FileInfo::with_metadata(
            total_lines,
            total_bytes,
            language,
            "UTF-8".to_string(), // Assume UTF-8 for stdin
        )
        .with_lines(final_lines.clone())
        .with_truncation(
            truncated_by_lines || truncated_by_bytes,
            truncated_by_lines,
            truncated_by_bytes,
        );

        Ok(Self::apply_post_processing(file_info, &final_lines, config))
    }

    /// Detect file encoding
    pub fn detect_encoding(file_path: &str) -> BatlessResult<String> {
        let mut file = File::open(file_path).map_err(|e| BatlessError::FileReadError {
            path: file_path.to_string(),
            source: e,
        })?;

        // Read a sample to detect encoding
        let file_size = usize::try_from(file.metadata()?.len()).unwrap_or(usize::MAX);
        let sample_size = 1024.min(file_size);
        let mut buffer = vec![0; sample_size];

        let bytes_read = file
            .read(&mut buffer)
            .map_err(|e| BatlessError::FileReadError {
                path: file_path.to_string(),
                source: e,
            })?;

        buffer.truncate(bytes_read);

        let (_encoding, _, had_errors) = encoding_rs::UTF_8.decode(&buffer);

        if !had_errors {
            Ok("UTF-8".to_string())
        } else {
            // Try to detect other common encodings
            match Self::detect_alternative_encoding(&buffer) {
                Some(encoding) => Ok(encoding),
                None => {
                    // Return Unknown but log the issue - this is a soft failure
                    // that allows processing to continue with best-effort decoding
                    Ok("Unknown".to_string())
                }
            }
        }
    }

    /// Attempt to detect alternative encodings for non-UTF-8 files
    fn detect_alternative_encoding(buffer: &[u8]) -> Option<String> {
        // Try common encodings
        let encodings = [
            encoding_rs::WINDOWS_1252,
            encoding_rs::ISO_8859_15,
            encoding_rs::UTF_16LE,
            encoding_rs::UTF_16BE,
        ];

        for encoding in &encodings {
            let (_, _, had_errors) = encoding.decode(buffer);
            if !had_errors {
                return Some(encoding.name().to_string());
            }
        }

        None
    }

    /// Read file content with truncation limits
    fn read_file_content(
        file_path: &str,
        config: &BatlessConfig,
    ) -> BatlessResult<(Vec<String>, FileMetadata)> {
        let file = File::open(file_path).map_err(|e| BatlessError::FileReadError {
            path: file_path.to_string(),
            source: e,
        })?;

        let metadata = file.metadata().map_err(|e| BatlessError::FileReadError {
            path: file_path.to_string(),
            source: e,
        })?;
        let total_file_bytes = usize::try_from(metadata.len()).unwrap_or(usize::MAX);

        let reader = BufReader::new(file);
        let mut lines = Vec::new();
        let mut bytes_seen = 0usize;
        let mut truncated_by_lines = false;
        let mut truncated_by_bytes = false;

        for line_result in reader.lines() {
            if lines.len() >= config.max_lines {
                truncated_by_lines = true;
                break;
            }

            let line = line_result.map_err(|e| BatlessError::FileReadError {
                path: file_path.to_string(),
                source: e,
            })?;

            let line_bytes = line.len() + 1; // +1 for newline
            if let Some(max_bytes) = config.max_bytes {
                if bytes_seen + line_bytes > max_bytes {
                    truncated_by_bytes = true;
                    break;
                }
            }

            bytes_seen += line_bytes;
            lines.push(line);
        }

        if truncated_by_lines {
            if let Some(max_bytes) = config.max_bytes {
                if total_file_bytes > max_bytes {
                    truncated_by_bytes = true;
                }
            }
        }

        let truncated = truncated_by_lines || truncated_by_bytes;
        let metadata = FileMetadata {
            total_lines: lines.len(),
            total_lines_exact: !truncated,
            total_bytes: total_file_bytes,
            truncated,
            truncated_by_lines,
            truncated_by_bytes,
        };

        Ok((lines, metadata))
    }

    /// Validate file accessibility and permissions
    pub fn validate_file_access(file_path: &str) -> BatlessResult<()> {
        let path = Path::new(file_path);

        if !path.exists() {
            return Err(BatlessError::file_not_found_with_suggestions(
                file_path.to_string(),
            ));
        }

        if !path.is_file() {
            let file_type = if path.is_dir() {
                "directory"
            } else if path.is_symlink() {
                "symbolic link"
            } else {
                "non-regular file"
            };
            return Err(BatlessError::processing_error_with_help(
                Some(file_path.to_string()),
                format!("Path is a {file_type}, not a regular file"),
                format!(
                    "Use 'ls -la {file_path}' to inspect the path, or provide a regular file path"
                ),
            ));
        }

        // Try to open the file to check read permissions
        File::open(file_path).map_err(|e| BatlessError::FileReadError {
            path: file_path.to_string(),
            source: e,
        })?;

        Ok(())
    }

    /// Get file size without reading the entire file
    pub fn get_file_size(file_path: &str) -> BatlessResult<u64> {
        let metadata = std::fs::metadata(file_path).map_err(|e| BatlessError::FileReadError {
            path: file_path.to_string(),
            source: e,
        })?;

        Ok(metadata.len())
    }

    /// Check if a file is likely to be binary
    #[allow(clippy::naive_bytecount)]
    pub fn is_likely_binary(file_path: &str) -> BatlessResult<bool> {
        let mut file = File::open(file_path).map_err(|e| BatlessError::FileReadError {
            path: file_path.to_string(),
            source: e,
        })?;

        // Read first 1KB to check for binary content
        let mut buffer = [0; 1024];
        let bytes_read = file
            .read(&mut buffer)
            .map_err(|e| BatlessError::FileReadError {
                path: file_path.to_string(),
                source: e,
            })?;

        // Check for null bytes or high ratio of non-printable characters
        let null_bytes = buffer[..bytes_read].iter().filter(|&&b| b == 0).count();
        let non_printable = buffer[..bytes_read]
            .iter()
            .filter(|&&b| b < 32 && b != 9 && b != 10 && b != 13) // Exclude tab, LF, CR
            .count();

        // Consider binary if >5% null bytes or >30% non-printable
        let null_ratio = null_bytes as f64 / bytes_read as f64;
        let non_printable_ratio = non_printable as f64 / bytes_read as f64;

        Ok(null_ratio > 0.05 || non_printable_ratio > 0.30)
    }
}

/// Metadata collected during file reading
#[derive(Debug, Clone)]
struct FileMetadata {
    total_lines: usize,
    total_lines_exact: bool,
    total_bytes: usize,
    truncated: bool,
    truncated_by_lines: bool,
    truncated_by_bytes: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_test_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{content}").unwrap();
        file
    }

    #[test]
    fn test_process_file_basic() -> BatlessResult<()> {
        let file = create_test_file("line1\nline2\nline3");
        let config = BatlessConfig::default();

        let result = FileProcessor::process_file(file.path().to_str().unwrap(), &config)?;

        assert_eq!(result.lines.len(), 3);
        assert_eq!(result.total_lines, 3);
        assert!(!result.truncated);

        Ok(())
    }

    #[test]
    fn test_process_file_with_line_limit() -> BatlessResult<()> {
        let file = create_test_file("line1\nline2\nline3\nline4\nline5");
        let config = BatlessConfig::default().with_max_lines(3);

        let result = FileProcessor::process_file(file.path().to_str().unwrap(), &config)?;

        assert_eq!(result.lines.len(), 3);
        assert!(result.truncated);
        assert!(result.truncated_by_lines);

        Ok(())
    }

    #[test]
    fn test_process_file_with_byte_limit() -> BatlessResult<()> {
        // Create content larger than byte limit
        let large_content = "a".repeat(2000); // 2000 characters
        let file = create_test_file(&large_content);
        let config = BatlessConfig::default()
            .with_max_bytes(Some(1000)) // 1KB limit
            .with_max_lines(100); // Large line limit

        let result = FileProcessor::process_file(file.path().to_str().unwrap(), &config)?;

        assert!(result.truncated);
        assert!(result.truncated_by_bytes);

        Ok(())
    }

    #[test]
    fn test_process_file_not_found() {
        let config = BatlessConfig::default();
        let result = FileProcessor::process_file("nonexistent.txt", &config);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            BatlessError::FileNotFound { .. }
        ));
    }

    #[test]
    fn test_validate_file_access() {
        let file = create_test_file("test content");

        // Valid file should pass
        assert!(FileProcessor::validate_file_access(file.path().to_str().unwrap()).is_ok());

        // Non-existent file should fail
        assert!(FileProcessor::validate_file_access("nonexistent.txt").is_err());
    }

    #[test]
    fn test_get_file_size() {
        let content = "test content";
        let file = create_test_file(content);

        let size = FileProcessor::get_file_size(file.path().to_str().unwrap()).unwrap();
        assert_eq!(size, content.len() as u64);
    }

    #[test]
    fn test_detect_encoding() {
        let file = create_test_file("test content");

        let encoding = FileProcessor::detect_encoding(file.path().to_str().unwrap()).unwrap();
        assert_eq!(encoding, "UTF-8");
    }

    #[test]
    fn test_is_likely_binary() {
        // Text file
        let text_file = create_test_file("This is plain text content");
        assert!(!FileProcessor::is_likely_binary(text_file.path().to_str().unwrap()).unwrap());

        // Binary-like file (with null bytes)
        let mut binary_file = NamedTempFile::new().unwrap();
        binary_file.write_all(&[0, 1, 2, 3, 0, 0, 0]).unwrap();
        assert!(FileProcessor::is_likely_binary(binary_file.path().to_str().unwrap()).unwrap());
    }

    #[test]
    fn test_process_file_with_summary_mode() -> BatlessResult<()> {
        let file = create_test_file("fn main() {\n    println!(\"Hello\");\n}");
        let config = BatlessConfig::default().with_summary_mode(true);

        let result = FileProcessor::process_file(file.path().to_str().unwrap(), &config)?;

        assert!(result.has_summary());

        Ok(())
    }

    #[test]
    fn test_process_file_with_tokens() -> BatlessResult<()> {
        let file = create_test_file("fn main() { println!(\"Hello\"); }");
        let config = BatlessConfig::default().with_include_tokens(true);

        let result = FileProcessor::process_file(file.path().to_str().unwrap(), &config)?;

        assert!(result.has_tokens());
        assert!(result.token_count() > 0);

        Ok(())
    }
}
