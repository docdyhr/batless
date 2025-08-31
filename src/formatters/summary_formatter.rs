//! Summary output formatting

use crate::config::BatlessConfig;
use crate::error::BatlessResult;
use crate::file_info::FileInfo;
use crate::formatter::OutputMode;
use crate::formatters::Formatter;

/// Summary formatter for overview output
pub struct SummaryFormatter;

impl Formatter for SummaryFormatter {
    fn format(
        &self,
        file_info: &FileInfo,
        file_path: &str,
        _config: &BatlessConfig,
    ) -> BatlessResult<String> {
        let mut output = Vec::new();

        // File header with basic info
        let stats = file_info.get_stats_summary();

        output.push(format!(
            r"File Processing Statistics
==========================
File: {}
Language: {}
Encoding: {}
Total Lines: {}
Processed Lines: {}
Total Bytes: {}
Processing Time: {}ms
Truncated: {}
Syntax Errors: {}
Tokens: {}
Summary Lines: {}
Processing Ratio: {:.2}%",
            file_path,
            stats.language.as_deref().unwrap_or("Unknown"),
            stats.encoding,
            stats.total_lines,
            stats.processed_lines,
            stats.total_bytes,
            0, // Processing time would need to be tracked separately
            if stats.truncated { "Yes" } else { "No" },
            stats.error_count,
            stats.token_count,
            stats.summary_line_count,
            if stats.total_lines > 0 {
                (stats.processed_lines as f64 / stats.total_lines as f64) * 100.0
            } else {
                0.0
            }
        ));

        // Add code structure if available
        if let Some(ref summary_lines) = file_info.summary_lines {
            output.push(String::new()); // Empty line separator
            output.push("=== Code Structure ===".to_string());
            for line in summary_lines {
                output.push(line.clone());
            }
        } else {
            output.push(String::new());
            output.push("=== Content ===".to_string());
            for line in &file_info.lines {
                output.push(line.clone());
            }
        }

        // Token information if available
        if let Some(ref tokens) = file_info.tokens {
            output.push(String::new());
            output.push("=== Tokens ===".to_string());
            output.push(format!("Token count: {}", tokens.len()));
            if tokens.len() <= 20 {
                output.push(format!("Tokens: {}", tokens.join(", ")));
            } else {
                let preview = tokens.iter().take(20).cloned().collect::<Vec<_>>();
                output.push(format!("First 20 tokens: {}", preview.join(", ")));
                output.push(format!("... and {} more", tokens.len() - 20));
            }
        }

        Ok(output.join("\n"))
    }

    fn output_mode(&self) -> OutputMode {
        OutputMode::Summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summary_formatter_mode() {
        let formatter = SummaryFormatter;
        assert!(matches!(formatter.output_mode(), OutputMode::Summary));
    }
}
