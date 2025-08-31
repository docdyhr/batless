//! Plain text and highlighted output formatters

use crate::config::BatlessConfig;
use crate::error::BatlessResult;
use crate::file_info::FileInfo;
use crate::formatter::OutputMode;
use crate::formatters::Formatter;
use crate::highlighter::SyntaxHighlighter;

/// Plain text formatter (no syntax highlighting)
pub struct PlainFormatter;

impl Formatter for PlainFormatter {
    fn format(
        &self,
        file_info: &FileInfo,
        _file_path: &str,
        _config: &BatlessConfig,
    ) -> BatlessResult<String> {
        let mut output = Vec::new();

        // For now, just return the content without line numbers
        // Line numbers would need to be added to BatlessConfig if required
        output.extend(file_info.lines.iter().cloned());

        Ok(output.join("\n"))
    }

    fn output_mode(&self) -> OutputMode {
        OutputMode::Plain
    }
}

/// Syntax highlighting formatter
pub struct HighlightFormatter;

impl Formatter for HighlightFormatter {
    fn format(
        &self,
        file_info: &FileInfo,
        file_path: &str,
        config: &BatlessConfig,
    ) -> BatlessResult<String> {
        // Use the highlighter's existing method
        SyntaxHighlighter::highlight_content(&file_info.lines.join("\n"), file_path, config)
    }

    fn output_mode(&self) -> OutputMode {
        OutputMode::Highlight
    }
}

/// Add line numbers to text lines
#[allow(dead_code)]
fn add_line_numbers(lines: &[String], number_nonblank_only: bool) -> Vec<String> {
    let mut result = Vec::new();
    let mut line_number = 1;

    for line in lines {
        if number_nonblank_only {
            // Only number non-blank lines (cat -b behavior)
            if line.trim().is_empty() {
                result.push(line.clone());
            } else {
                result.push(format!("{line_number:6}\t{line}"));
                line_number += 1;
            }
        } else {
            // Number all lines (cat -n behavior)
            result.push(format!("{line_number:6}\t{line}"));
            line_number += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_numbering() {
        let lines = vec!["line 1".to_string(), "".to_string(), "line 3".to_string()];

        let numbered = add_line_numbers(&lines, false);
        assert_eq!(numbered.len(), 3);
        assert!(numbered[0].contains("1\t"));
        assert!(numbered[2].contains("3\t"));

        let numbered_nonblank = add_line_numbers(&lines, true);
        assert_eq!(numbered_nonblank.len(), 3);
        assert!(numbered_nonblank[1].is_empty());
    }

    #[test]
    fn test_plain_formatter_mode() {
        let formatter = PlainFormatter;
        assert!(matches!(formatter.output_mode(), OutputMode::Plain));
    }
}
