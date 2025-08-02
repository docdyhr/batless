//! Streaming JSON output functionality for batless
//!
//! This module provides streaming JSON output for very large files,
//! allowing partial content processing with resume capability.

use crate::config::BatlessConfig;
use crate::error::{BatlessError, BatlessResult};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Checkpoint information for resuming streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingCheckpoint {
    /// File path being processed
    pub file_path: String,
    /// Current line number (0-based)
    pub line_number: usize,
    /// Total bytes processed so far
    pub bytes_processed: usize,
    /// Chunk number being processed
    pub chunk_number: usize,
    /// Total chunks expected (if known)
    pub total_chunks: Option<usize>,
    /// Schema version used
    pub schema_version: String,
    /// Timestamp when checkpoint was created
    pub timestamp: String,
    /// Configuration hash for validation
    pub config_hash: String,
}

impl StreamingCheckpoint {
    /// Create a new checkpoint
    pub fn new(
        file_path: String,
        line_number: usize,
        bytes_processed: usize,
        chunk_number: usize,
        config: &BatlessConfig,
    ) -> Self {
        Self {
            file_path,
            line_number,
            bytes_processed,
            chunk_number,
            total_chunks: None,
            schema_version: config.schema_version.clone(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            config_hash: Self::compute_config_hash(config),
        }
    }

    /// Compute a hash of the relevant configuration for validation
    fn compute_config_hash(config: &BatlessConfig) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        config.max_lines.hash(&mut hasher);
        config.max_bytes.hash(&mut hasher);
        config.language.hash(&mut hasher);
        config.include_tokens.hash(&mut hasher);
        config.summary_level.hash(&mut hasher);
        config.streaming_chunk_size.hash(&mut hasher);

        format!("{:x}", hasher.finish())
    }

    /// Validate that this checkpoint is compatible with the current config
    pub fn is_compatible(&self, config: &BatlessConfig) -> bool {
        self.config_hash == Self::compute_config_hash(config)
            && self.schema_version == config.schema_version
    }
}

/// Streaming JSON chunk with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingChunk {
    /// Schema version
    pub schema_version: String,
    /// Chunk metadata
    pub metadata: ChunkMetadata,
    /// File content lines for this chunk
    pub lines: Vec<String>,
    /// Checkpoint information
    pub checkpoint: StreamingCheckpoint,
    /// Whether this is the final chunk
    pub is_final: bool,
}

/// Metadata for a streaming chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkMetadata {
    /// File path
    pub file_path: String,
    /// Language detected for the file
    pub language: Option<String>,
    /// File encoding
    pub encoding: String,
    /// Total file size in bytes
    pub total_file_bytes: u64,
    /// Total lines in file
    pub total_file_lines: usize,
    /// Lines in this chunk
    pub chunk_lines: usize,
    /// Bytes in this chunk
    pub chunk_bytes: usize,
    /// Starting line number for this chunk (0-based)
    pub start_line: usize,
    /// Ending line number for this chunk (0-based)
    pub end_line: usize,
}

/// Streaming JSON processor
pub struct StreamingProcessor;

impl StreamingProcessor {
    /// Process a file with streaming JSON output
    pub fn process_streaming(
        file_path: &str,
        config: &BatlessConfig,
        checkpoint: Option<StreamingCheckpoint>,
    ) -> BatlessResult<impl Iterator<Item = BatlessResult<StreamingChunk>>> {
        // Check if this is stdin input
        if file_path == "-" {
            // Note: Streaming from stdin doesn't support checkpoints since stdin is not seekable
            if checkpoint.is_some() {
                return Err(BatlessError::config_error_with_help(
                    "Resume/checkpoint functionality is not supported with stdin input".to_string(),
                    Some("Stdin is not seekable. Use file input for checkpoint support.".to_string()),
                ));
            }
            
            let processor = StreamingProcessorIterator::new_from_stdin(config)?;
            return Ok(processor);
        }

        // Validate checkpoint if provided
        if let Some(ref cp) = checkpoint {
            if !cp.is_compatible(config) {
                return Err(BatlessError::config_error_with_help(
                    "Checkpoint is incompatible with current configuration".to_string(),
                    Some("Configuration or schema version has changed. Start fresh without checkpoint.".to_string()),
                ));
            }

            if cp.file_path != file_path {
                return Err(BatlessError::config_error_with_help(
                    "Checkpoint file path doesn't match current file".to_string(),
                    Some("Checkpoint was created for a different file".to_string()),
                ));
            }
        }

        let processor = StreamingProcessorIterator::new(file_path, config, checkpoint)?;
        Ok(processor)
    }

    /// Create a checkpoint file for resuming later
    pub fn save_checkpoint(
        checkpoint: &StreamingCheckpoint,
        checkpoint_path: &Path,
    ) -> BatlessResult<()> {
        let json_data = serde_json::to_string_pretty(checkpoint)
            .map_err(BatlessError::JsonSerializationError)?;

        std::fs::write(checkpoint_path, json_data).map_err(|e| BatlessError::FileReadError {
            path: checkpoint_path.to_string_lossy().to_string(),
            source: e,
        })?;

        Ok(())
    }

    /// Load a checkpoint from file
    pub fn load_checkpoint(checkpoint_path: &Path) -> BatlessResult<StreamingCheckpoint> {
        let data =
            std::fs::read_to_string(checkpoint_path).map_err(|e| BatlessError::FileReadError {
                path: checkpoint_path.to_string_lossy().to_string(),
                source: e,
            })?;

        let checkpoint: StreamingCheckpoint =
            serde_json::from_str(&data).map_err(BatlessError::JsonSerializationError)?;

        Ok(checkpoint)
    }

    /// Generate streaming JSON schema
    pub fn get_streaming_schema() -> serde_json::Value {
        json!({
            "$schema": "http://json-schema.org/draft-07/schema#",
            "title": "Batless Streaming JSON Output",
            "description": "Schema for streaming JSON chunks from batless",
            "type": "object",
            "required": ["schema_version", "metadata", "lines", "checkpoint", "is_final"],
            "properties": {
                "schema_version": {
                    "type": "string",
                    "description": "Version of the JSON schema used"
                },
                "metadata": {
                    "type": "object",
                    "required": ["file_path", "encoding", "total_file_bytes", "total_file_lines", "chunk_lines", "chunk_bytes", "start_line", "end_line"],
                    "properties": {
                        "file_path": { "type": "string" },
                        "language": { "type": ["string", "null"] },
                        "encoding": { "type": "string" },
                        "total_file_bytes": { "type": "integer", "minimum": 0 },
                        "total_file_lines": { "type": "integer", "minimum": 0 },
                        "chunk_lines": { "type": "integer", "minimum": 0 },
                        "chunk_bytes": { "type": "integer", "minimum": 0 },
                        "start_line": { "type": "integer", "minimum": 0 },
                        "end_line": { "type": "integer", "minimum": 0 }
                    }
                },
                "lines": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Content lines for this chunk"
                },
                "checkpoint": {
                    "type": "object",
                    "required": ["file_path", "line_number", "bytes_processed", "chunk_number", "schema_version", "timestamp", "config_hash"],
                    "properties": {
                        "file_path": { "type": "string" },
                        "line_number": { "type": "integer", "minimum": 0 },
                        "bytes_processed": { "type": "integer", "minimum": 0 },
                        "chunk_number": { "type": "integer", "minimum": 0 },
                        "total_chunks": { "type": ["integer", "null"], "minimum": 1 },
                        "schema_version": { "type": "string" },
                        "timestamp": { "type": "string", "format": "date-time" },
                        "config_hash": { "type": "string" }
                    }
                },
                "is_final": {
                    "type": "boolean",
                    "description": "Whether this is the last chunk in the stream"
                }
            }
        })
    }
}

/// Iterator for streaming file processing
enum StreamingProcessorIterator {
    File {
        reader: BufReader<File>,
        config: BatlessConfig,
        file_metadata: FileMetadata,
        current_line: usize,
        bytes_processed: usize,
        chunk_number: usize,
        finished: bool,
    },
    Stdin {
        reader: BufReader<std::io::Stdin>,
        config: BatlessConfig,
        stdin_metadata: FileMetadata,
        current_line: usize,
        bytes_processed: usize,
        chunk_number: usize,
        finished: bool,
    },
}

/// File metadata for streaming
#[derive(Debug, Clone)]
struct FileMetadata {
    path: String,
    language: Option<String>,
    encoding: String,
    total_bytes: u64,
    total_lines: usize,
}

impl StreamingProcessorIterator {
    fn new(
        file_path: &str,
        config: &BatlessConfig,
        checkpoint: Option<StreamingCheckpoint>,
    ) -> BatlessResult<Self> {
        let file = File::open(file_path).map_err(|e| BatlessError::FileReadError {
            path: file_path.to_string(),
            source: e,
        })?;

        let file_metadata = Self::gather_file_metadata(file_path)?;
        let mut reader = BufReader::new(file);

        // If resuming, skip to checkpoint position
        let (current_line, bytes_processed, chunk_number) = if let Some(cp) = checkpoint {
            // Skip lines to resume position
            for _ in 0..cp.line_number {
                let mut line = String::new();
                reader
                    .read_line(&mut line)
                    .map_err(|e| BatlessError::FileReadError {
                        path: file_path.to_string(),
                        source: e,
                    })?;
            }
            (cp.line_number, cp.bytes_processed, cp.chunk_number)
        } else {
            (0, 0, 0)
        };

        Ok(StreamingProcessorIterator::File {
            reader,
            config: config.clone(),
            file_metadata,
            current_line,
            bytes_processed,
            chunk_number,
            finished: false,
        })
    }

    fn new_from_stdin(config: &BatlessConfig) -> BatlessResult<Self> {
        use std::io::stdin;

        let reader = BufReader::new(stdin());
        
        // Create metadata for stdin
        let stdin_metadata = FileMetadata {
            path: "<stdin>".to_string(),
            language: None, // Cannot detect language without file extension
            encoding: "UTF-8".to_string(),
            total_bytes: 0, // Unknown for stdin
            total_lines: 0, // Unknown for stdin
        };

        Ok(StreamingProcessorIterator::Stdin {
            reader,
            config: config.clone(),
            stdin_metadata,
            current_line: 0,
            bytes_processed: 0,
            chunk_number: 0,
            finished: false,
        })
    }

    fn gather_file_metadata(file_path: &str) -> BatlessResult<FileMetadata> {
        use crate::language::LanguageDetector;
        use crate::processor::FileProcessor;

        let file = File::open(file_path).map_err(|e| BatlessError::FileReadError {
            path: file_path.to_string(),
            source: e,
        })?;

        let metadata = file.metadata().map_err(|e| BatlessError::FileReadError {
            path: file_path.to_string(),
            source: e,
        })?;

        // Detect encoding
        let encoding = FileProcessor::detect_encoding(file_path)?;

        // Count total lines (this is expensive but needed for metadata)
        let total_lines =
            BufReader::new(
                File::open(file_path).map_err(|e| BatlessError::FileReadError {
                    path: file_path.to_string(),
                    source: e,
                })?,
            )
            .lines()
            .count();

        // Detect language
        let language = LanguageDetector::detect_language_with_fallback(file_path);

        Ok(FileMetadata {
            path: file_path.to_string(),
            language,
            encoding,
            total_bytes: metadata.len(),
            total_lines,
        })
    }
}

impl Iterator for StreamingProcessorIterator {
    type Item = BatlessResult<StreamingChunk>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            StreamingProcessorIterator::File {
                reader,
                config,
                file_metadata,
                current_line,
                bytes_processed,
                chunk_number,
                finished,
            } => {
                if *finished {
                    return None;
                }

                // Read chunk_size lines
                let mut chunk_lines = Vec::new();
                let mut chunk_bytes = 0;
                let start_line = *current_line;

                for _ in 0..config.streaming_chunk_size {
                    let mut line = String::new();
                    match reader.read_line(&mut line) {
                        Ok(0) => break, // EOF
                        Ok(bytes_read) => {
                            chunk_bytes += bytes_read;
                            *bytes_processed += bytes_read;

                            // Remove trailing newline for consistency
                            if line.ends_with('\n') {
                                line.pop();
                                if line.ends_with('\r') {
                                    line.pop();
                                }
                            }

                            chunk_lines.push(line);
                            *current_line += 1;
                        }
                        Err(e) => {
                            return Some(Err(BatlessError::FileReadError {
                                path: file_metadata.path.clone(),
                                source: e,
                            }));
                        }
                    }
                }

                if chunk_lines.is_empty() {
                    *finished = true;
                    return None;
                }

                let end_line = *current_line - 1;
                let is_final = *current_line >= file_metadata.total_lines
                    || *bytes_processed >= file_metadata.total_bytes as usize;

                if is_final {
                    *finished = true;
                }

                let metadata = ChunkMetadata {
                    file_path: file_metadata.path.clone(),
                    language: file_metadata.language.clone(),
                    encoding: file_metadata.encoding.clone(),
                    total_file_bytes: file_metadata.total_bytes,
                    total_file_lines: file_metadata.total_lines,
                    chunk_lines: chunk_lines.len(),
                    chunk_bytes,
                    start_line,
                    end_line,
                };

                let checkpoint = StreamingCheckpoint::new(
                    file_metadata.path.clone(),
                    *current_line,
                    *bytes_processed,
                    *chunk_number,
                    config,
                );

                let chunk = StreamingChunk {
                    schema_version: config.schema_version.clone(),
                    metadata,
                    lines: chunk_lines,
                    checkpoint,
                    is_final,
                };

                *chunk_number += 1;
                Some(Ok(chunk))
            }
            StreamingProcessorIterator::Stdin {
                reader,
                config,
                stdin_metadata,
                current_line,
                bytes_processed,
                chunk_number,
                finished,
            } => {
                if *finished {
                    return None;
                }

                // Read chunk_size lines from stdin
                let mut chunk_lines = Vec::new();
                let mut chunk_bytes = 0;
                let start_line = *current_line;

                for _ in 0..config.streaming_chunk_size {
                    let mut line = String::new();
                    match reader.read_line(&mut line) {
                        Ok(0) => break, // EOF
                        Ok(bytes_read) => {
                            chunk_bytes += bytes_read;
                            *bytes_processed += bytes_read;

                            // Remove trailing newline for consistency
                            if line.ends_with('\n') {
                                line.pop();
                                if line.ends_with('\r') {
                                    line.pop();
                                }
                            }

                            chunk_lines.push(line);
                            *current_line += 1;
                        }
                        Err(e) => {
                            return Some(Err(BatlessError::FileReadError {
                                path: stdin_metadata.path.clone(),
                                source: e,
                            }));
                        }
                    }
                }

                if chunk_lines.is_empty() {
                    *finished = true;
                    return None;
                }

                let end_line = *current_line - 1;
                
                // Check if we've hit EOF by trying to peek at the buffer
                let is_final = match reader.fill_buf() {
                    Ok(buf) => buf.is_empty(), // EOF if buffer is empty
                    Err(_) => true, // Assume EOF on error
                };
                
                if is_final {
                    *finished = true;
                }

                let metadata = ChunkMetadata {
                    file_path: stdin_metadata.path.clone(),
                    language: stdin_metadata.language.clone(),
                    encoding: stdin_metadata.encoding.clone(),
                    total_file_bytes: *bytes_processed as u64, // Use current bytes as estimate
                    total_file_lines: *current_line, // Use current line count as estimate
                    chunk_lines: chunk_lines.len(),
                    chunk_bytes,
                    start_line,
                    end_line,
                };

                let checkpoint = StreamingCheckpoint::new(
                    stdin_metadata.path.clone(),
                    *current_line,
                    *bytes_processed,
                    *chunk_number,
                    config,
                );

                let chunk = StreamingChunk {
                    schema_version: config.schema_version.clone(),
                    metadata,
                    lines: chunk_lines,
                    checkpoint,
                    is_final,
                };

                *chunk_number += 1;
                Some(Ok(chunk))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::BatlessConfig;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_test_file() -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "line 1").unwrap();
        writeln!(file, "line 2").unwrap();
        writeln!(file, "line 3").unwrap();
        writeln!(file, "line 4").unwrap();
        writeln!(file, "line 5").unwrap();
        file
    }

    #[test]
    fn test_streaming_checkpoint_creation() {
        let config = BatlessConfig::default().with_streaming_chunk_size(2);
        let checkpoint = StreamingCheckpoint::new("test.txt".to_string(), 10, 500, 2, &config);

        assert_eq!(checkpoint.file_path, "test.txt");
        assert_eq!(checkpoint.line_number, 10);
        assert_eq!(checkpoint.bytes_processed, 500);
        assert_eq!(checkpoint.chunk_number, 2);
        assert_eq!(checkpoint.schema_version, config.schema_version);
        assert!(!checkpoint.timestamp.is_empty());
        assert!(!checkpoint.config_hash.is_empty());
    }

    #[test]
    fn test_checkpoint_compatibility() {
        let config1 = BatlessConfig::default().with_streaming_chunk_size(1000);
        let config2 = BatlessConfig::default().with_streaming_chunk_size(2000);

        let checkpoint = StreamingCheckpoint::new("test.txt".to_string(), 0, 0, 0, &config1);

        assert!(checkpoint.is_compatible(&config1));
        assert!(!checkpoint.is_compatible(&config2));
    }

    #[test]
    fn test_streaming_schema() {
        let schema = StreamingProcessor::get_streaming_schema();
        assert!(schema["properties"]["schema_version"].is_object());
        assert!(schema["properties"]["metadata"].is_object());
        assert!(schema["properties"]["lines"].is_object());
        assert!(schema["properties"]["checkpoint"].is_object());
        assert!(schema["properties"]["is_final"].is_object());
    }

    #[test]
    fn test_streaming_processor_basic() -> BatlessResult<()> {
        let file = create_test_file();
        let config = BatlessConfig::default()
            .with_streaming_json(true)
            .with_streaming_chunk_size(2);

        let chunks: Result<Vec<_>, _> =
            StreamingProcessor::process_streaming(file.path().to_str().unwrap(), &config, None)?
                .collect();

        let chunks = chunks?;
        assert_eq!(chunks.len(), 3); // 5 lines with chunk size 2: [2, 2, 1]

        // Check first chunk
        assert_eq!(chunks[0].lines.len(), 2);
        assert_eq!(chunks[0].lines[0], "line 1");
        assert_eq!(chunks[0].lines[1], "line 2");
        assert!(!chunks[0].is_final);

        // Check last chunk
        assert!(chunks[2].is_final);
        assert_eq!(chunks[2].lines.len(), 1);
        assert_eq!(chunks[2].lines[0], "line 5");

        Ok(())
    }
}
