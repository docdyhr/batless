//! File processing functionality for batless
//!
//! This module handles the core file processing logic including reading files,
//! detecting encoding, handling truncation limits, and coordinating with other
//! modules for language detection, summarization, and tokenization.

use crate::config::BatlessConfig;
use crate::error::{BatlessError, BatlessResult};
use crate::file_info::FileInfo;
use crate::language::LanguageDetector;
use crate::summarizer::SummaryExtractor;
use crate::tokens::TokenExtractor;
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
            .as_ref()
            .cloned()
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
        .with_lines(lines.clone())
        .with_truncation(
            metadata.truncated,
            metadata.truncated_by_lines,
            metadata.truncated_by_bytes,
        );

        // Process summary if requested
        let summary_level = config.effective_summary_level();
        if summary_level.is_enabled() {
            let summary_lines = SummaryExtractor::extract_summary(
                &lines,
                file_info.language.as_ref(),
                summary_level,
            );
            // In summary mode, replace the output lines with summary
            file_info = file_info.with_summary_lines(Some(summary_lines.clone()));
            file_info.lines = summary_lines;
        }

        // Extract tokens if requested
        if config.include_tokens {
            let content = file_info.lines.join("\n");
            let tokens = TokenExtractor::extract_tokens(&content, file_path);
            file_info = file_info.with_tokens(Some(tokens));
        }

        Ok(file_info)
    }

    /// Process input from stdin
    pub fn process_stdin(config: &BatlessConfig) -> BatlessResult<FileInfo> {
        use std::io::{stdin, Read};

        // Read all content from stdin
        let mut content = String::new();
        stdin()
            .read_to_string(&mut content)
            .map_err(|e| BatlessError::FileReadError {
                path: "<stdin>".to_string(),
                source: e,
            })?;

        // Split into lines
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let total_lines = lines.len();
        let total_bytes = content.len();

        // Apply line limits if configured
        let (final_lines, truncated_by_lines) = if total_lines > config.max_lines {
            (lines[..config.max_lines].to_vec(), true)
        } else {
            (lines, false)
        };

        // Apply byte limits if configured
        let (final_lines, total_bytes, truncated_by_bytes) =
            if let Some(max_bytes) = config.max_bytes {
                let mut byte_count = 0;
                let mut truncated_lines = Vec::new();
                let mut truncated = false;

                for line in final_lines {
                    let line_bytes = line.len() + 1; // +1 for newline
                    if byte_count + line_bytes > max_bytes {
                        truncated = true;
                        break;
                    }
                    byte_count += line_bytes;
                    truncated_lines.push(line);
                }
                (truncated_lines, byte_count, truncated)
            } else {
                (final_lines, total_bytes, false)
            };

        // Detect language from content (limited for stdin without filename)
        let language = config.language.as_ref().cloned(); // Use configured language or none

        // Create FileInfo
        let mut file_info = FileInfo::with_metadata(
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

        // Process summary if requested
        let summary_level = config.effective_summary_level();
        if summary_level.is_enabled() {
            let summary_lines = SummaryExtractor::extract_summary(
                &final_lines,
                file_info.language.as_ref(),
                summary_level,
            );
            // In summary mode, replace the output lines with summary
            file_info = file_info.with_summary_lines(Some(summary_lines.clone()));
            file_info.lines = summary_lines;
        }

        // Extract tokens if requested
        if config.include_tokens {
            let content = file_info.lines.join("\n");
            let tokens = TokenExtractor::extract_tokens(&content, "<stdin>");
            file_info = file_info.with_tokens(Some(tokens));
        }

        Ok(file_info)
    }

    /// Detect file encoding
    pub fn detect_encoding(file_path: &str) -> BatlessResult<String> {
        let mut file = File::open(file_path).map_err(|e| BatlessError::FileReadError {
            path: file_path.to_string(),
            source: e,
        })?;

        // Read a sample to detect encoding
        let file_size = file.metadata()?.len() as usize;
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
            Ok(Self::detect_alternative_encoding(&buffer).unwrap_or_else(|| "Unknown".to_string()))
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

        let reader = BufReader::new(file);
        let mut lines = Vec::new();
        let mut total_lines = 0;
        let mut total_bytes = 0;
        let mut truncated = false;
        let mut truncated_by_lines = false;
        let mut truncated_by_bytes = false;
        let mut capturing = true;

        for line_result in reader.lines() {
            let line = line_result.map_err(|e| BatlessError::FileReadError {
                path: file_path.to_string(),
                source: e,
            })?;

            let line_bytes = line.len() + 1; // +1 for newline
            total_lines += 1;
            total_bytes += line_bytes;

            if capturing && lines.len() >= config.max_lines {
                truncated = true;
                truncated_by_lines = true;
                capturing = false;
            }

            if capturing {
                if let Some(max_bytes) = config.max_bytes {
                    if total_bytes > max_bytes {
                        truncated = true;
                        truncated_by_bytes = true;
                        capturing = false;
                    }
                }
            }

            if capturing {
                lines.push(line);
            }
        }

        let metadata = FileMetadata {
            total_lines,
            total_bytes,
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
            return Err(BatlessError::ProcessingError(format!(
                "Path '{file_path}' is not a regular file"
            )));
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
