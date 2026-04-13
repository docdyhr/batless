//! Summary output formatter

use crate::config::BatlessConfig;
use crate::error::BatlessResult;
use crate::file_info::FileInfo;
use crate::formatter::OutputMode;
use crate::formatters::Formatter;

pub struct SummaryFormatter;

impl Formatter for SummaryFormatter {
    fn format(
        &self,
        file_info: &FileInfo,
        _file_path: &str,
        _config: &BatlessConfig,
    ) -> BatlessResult<String> {
        let mut output = Vec::new();

        output.push("=== File Summary ===".to_string());
        output.push(format!(
            "Language: {}",
            file_info.language.as_deref().unwrap_or("Unknown")
        ));
        output.push(format!("Encoding: {}", file_info.encoding));
        let total_lines_display = if file_info.total_lines_exact {
            file_info.total_lines.to_string()
        } else {
            format!("{}+", file_info.total_lines)
        };
        output.push(format!("Total Lines: {total_lines_display}"));
        output.push(format!("Processed Lines: {}", file_info.processed_lines()));

        if file_info.truncated {
            if let Some(reason) = file_info.truncation_reason() {
                output.push(format!("Truncated: Yes ({reason})"));
            } else {
                output.push("Truncated: Yes".to_string());
            }
        }

        output.push(String::new());

        if let Some(ref summary_lines) = file_info.summary_lines {
            output.push("=== Code Structure ===".to_string());
            for item in summary_lines {
                output.push(format!("line {}: {}", item.line_number, item.line));
            }
        } else {
            output.push("=== Content ===".to_string());
            for line in &file_info.lines {
                output.push(line.clone());
            }
        }

        if let Some(ref tokens) = file_info.tokens {
            output.push(String::new());
            output.push("=== Tokens ===".to_string());
            output.push(format!("Token Count: {}", tokens.len()));

            if !tokens.is_empty() {
                output.push("Sample Tokens:".to_string());
                for token in tokens.iter().take(20) {
                    output.push(format!("  {token}"));
                }
                if tokens.len() > 20 {
                    output.push(format!("  ... and {} more", tokens.len() - 20));
                }
            }
        }

        if !file_info.syntax_errors.is_empty() {
            output.push(String::new());
            output.push("=== Syntax Errors ===".to_string());
            for error in &file_info.syntax_errors {
                output.push(format!("  {error}"));
            }
        }

        Ok(output.join("\n"))
    }

    fn output_mode(&self) -> OutputMode {
        OutputMode::Summary
    }
}
