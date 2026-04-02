//! Structured summary item with line number metadata

use serde::{Deserialize, Serialize};

/// A single extracted code structure with its location in the source file.
/// Used as the element type for `file_info.summary_lines` when summary mode
/// is active, giving AI consumers exact line references for every symbol.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SummaryItem {
    /// The source line text
    pub line: String,
    /// 1-based start line number in the original file
    pub line_number: usize,
    /// 1-based end line number of the enclosing block (None when unknown)
    pub end_line: Option<usize>,
    /// Kind of structure: "function", "struct", "class", "import", etc.
    pub kind: String,
}

impl SummaryItem {
    pub fn new(
        line: impl Into<String>,
        line_number: usize,
        end_line: Option<usize>,
        kind: impl Into<String>,
    ) -> Self {
        Self {
            line: line.into(),
            line_number,
            end_line,
            kind: kind.into(),
        }
    }
}
