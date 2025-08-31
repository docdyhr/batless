//! Modular formatting functionality
//!
//! This module provides specialized formatters for different output types,
//! promoting separation of concerns and easier testing/maintenance.

pub mod error_formatter;
pub mod json_formatter;
pub mod plain_formatter;
pub mod summary_formatter;

use crate::config::BatlessConfig;
use crate::error::BatlessResult;
use crate::file_info::FileInfo;
use crate::formatter::OutputMode;

/// Trait for output formatters
pub trait Formatter {
    /// Format file info into string output
    fn format(
        &self,
        file_info: &FileInfo,
        file_path: &str,
        config: &BatlessConfig,
    ) -> BatlessResult<String>;

    /// Get the output mode this formatter handles
    fn output_mode(&self) -> OutputMode;
}

/// Factory for creating appropriate formatter for given mode
pub fn create_formatter(mode: OutputMode) -> Box<dyn Formatter> {
    match mode {
        OutputMode::Plain => Box::new(plain_formatter::PlainFormatter),
        OutputMode::Highlight => Box::new(plain_formatter::HighlightFormatter),
        OutputMode::Json => Box::new(json_formatter::JsonFormatter),
        OutputMode::Summary => Box::new(summary_formatter::SummaryFormatter),
    }
}

/// Convenience function to format using the appropriate formatter
pub fn format_with_mode(
    file_info: &FileInfo,
    file_path: &str,
    config: &BatlessConfig,
    mode: OutputMode,
) -> BatlessResult<String> {
    let formatter = create_formatter(mode);
    formatter.format(file_info, file_path, config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formatter_creation() {
        let _plain = create_formatter(OutputMode::Plain);
        let _highlight = create_formatter(OutputMode::Highlight);
        let _json = create_formatter(OutputMode::Json);
        let _summary = create_formatter(OutputMode::Summary);
    }

    #[test]
    fn test_formatter_modes() {
        let plain = create_formatter(OutputMode::Plain);
        assert!(matches!(plain.output_mode(), OutputMode::Plain));

        let json = create_formatter(OutputMode::Json);
        assert!(matches!(json.output_mode(), OutputMode::Json));
    }
}
