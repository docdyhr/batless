//! Plain text output formatter

use crate::config::BatlessConfig;
use crate::error::BatlessResult;
use crate::file_info::FileInfo;
use crate::formatter::OutputMode;
use crate::formatters::Formatter;

pub struct PlainFormatter;

impl Formatter for PlainFormatter {
    fn format(
        &self,
        file_info: &FileInfo,
        _file_path: &str,
        config: &BatlessConfig,
    ) -> BatlessResult<String> {
        if config.show_line_numbers || config.show_line_numbers_nonblank {
            let mut result = Vec::new();
            let mut line_number = 1usize;

            for line in &file_info.lines {
                if config.show_line_numbers_nonblank {
                    if line.trim().is_empty() {
                        result.push(line.clone());
                    } else {
                        result.push(format!("{line_number:6}\t{line}"));
                        line_number += 1;
                    }
                } else {
                    result.push(format!("{line_number:6}\t{line}"));
                    line_number += 1;
                }
            }
            Ok(result.join("\n"))
        } else {
            Ok(file_info.lines.join("\n"))
        }
    }

    fn output_mode(&self) -> OutputMode {
        OutputMode::Plain
    }
}
