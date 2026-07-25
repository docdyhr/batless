//! JSON output formatter

use crate::config::BatlessConfig;
use crate::error::{BatlessError, BatlessResult};
use crate::file_info::FileInfo;
use crate::formatter::OutputMode;
use crate::formatters::Formatter;
use serde_json::json;

pub struct JsonFormatter;

impl Formatter for JsonFormatter {
    fn format(
        &self,
        file_info: &FileInfo,
        file_path: &str,
        config: &BatlessConfig,
    ) -> BatlessResult<String> {
        let lines_value: serde_json::Value = if config.json_line_numbers {
            file_info
                .lines
                .iter()
                .enumerate()
                .map(|(i, text)| json!({"n": i + 1, "text": text}))
                .collect()
        } else {
            json!(file_info.lines)
        };

        let mut json_data = json!({
            "file": file_path,
            "lines": lines_value,
            "processed_lines": file_info.processed_lines(),
            "total_lines": file_info.total_lines,
            "total_lines_exact": file_info.total_lines_exact,
            "total_bytes": file_info.total_bytes,
            "truncated": file_info.truncated,
            "truncated_by_lines": file_info.truncated_by_lines,
            "truncated_by_bytes": file_info.truncated_by_bytes,
            "truncated_by_context": file_info.truncated_by_context,
            "language": file_info.language,
            "encoding": file_info.encoding,
            "syntax_errors": file_info.syntax_errors,
            "mode": "json"
        });

        if let Some(ratio) = file_info.compression_ratio {
            json_data["compression_ratio"] = json!(ratio);
        }

        if config.pretty_json {
            serde_json::to_string_pretty(&json_data).map_err(BatlessError::from)
        } else {
            serde_json::to_string(&json_data).map_err(BatlessError::from)
        }
    }

    fn output_mode(&self) -> OutputMode {
        OutputMode::Json
    }
}
