//! Modular formatting functionality
//!
//! This module provides the `Formatter` trait and specialized formatters.
//! Production output routing goes through `src/formatter.rs`; this module
//! provides the trait interface and the Index/Error formatters that use it.

pub mod error_formatter;
pub mod index_formatter;

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
