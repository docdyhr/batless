//! Modular formatting functionality
//!
//! This module provides the `Formatter` trait and all output formatters.
//! `src/formatter.rs` is a thin dispatcher that routes through these.

pub mod ast_formatter;
pub mod error_formatter;
pub mod index_formatter;
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
