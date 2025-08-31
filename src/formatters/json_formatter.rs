//! JSON output formatting

use crate::config::BatlessConfig;
use crate::error::BatlessResult;
use crate::file_info::FileInfo;
use crate::formatter::OutputMode;
use crate::formatters::Formatter;
use serde_json::json;

/// JSON formatter for structured output
pub struct JsonFormatter;

impl Formatter for JsonFormatter {
    fn format(
        &self,
        file_info: &FileInfo,
        file_path: &str,
        config: &BatlessConfig,
    ) -> BatlessResult<String> {
        let json_output = json!({
            "file": file_path,
            "language": file_info.language,
            "lines": file_info.lines,
            "total_lines": file_info.total_lines,
            "total_bytes": file_info.total_bytes,
            "encoding": file_info.encoding,
            "truncated": file_info.truncated,
            "truncated_by_lines": file_info.truncated_by_lines,
            "truncated_by_bytes": file_info.truncated_by_bytes,
            "summary_lines": file_info.summary_lines,
            "syntax_errors": file_info.syntax_errors,
            "tokens": if config.include_tokens {
                file_info.tokens.clone()
            } else {
                None
            },
            "mode": "json"
        });

        Ok(serde_json::to_string_pretty(&json_output)?)
    }

    fn output_mode(&self) -> OutputMode {
        OutputMode::Json
    }
}

/// Compact JSON formatter for minimal output
pub struct CompactJsonFormatter;

impl Formatter for CompactJsonFormatter {
    fn format(
        &self,
        file_info: &FileInfo,
        file_path: &str,
        _config: &BatlessConfig,
    ) -> BatlessResult<String> {
        let json_output = json!({
            "file": file_path,
            "language": file_info.language,
            "total_lines": file_info.total_lines,
            "total_bytes": file_info.total_bytes,
            "encoding": file_info.encoding,
            "truncated": file_info.truncated,
        });

        Ok(serde_json::to_string(&json_output)?)
    }

    fn output_mode(&self) -> OutputMode {
        OutputMode::Json
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_formatter_mode() {
        let formatter = JsonFormatter;
        assert!(matches!(formatter.output_mode(), OutputMode::Json));
    }

    #[test]
    fn test_compact_formatter() {
        let formatter = CompactJsonFormatter;
        assert!(matches!(formatter.output_mode(), OutputMode::Json));
    }
}
