//! Summary level configuration for code analysis
//!
//! This module defines the different levels of summary extraction that batless can perform.

use serde::{Deserialize, Serialize};

/// Summary extraction level for code analysis
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default, Hash)]
pub enum SummaryLevel {
    /// No summary extraction
    #[default]
    None,
    /// Minimal summary: functions and exports only
    Minimal,
    /// Standard summary: functions, classes, imports (current behavior)
    Standard,
    /// Detailed summary: includes comments, complexity metrics
    Detailed,
}

impl SummaryLevel {
    /// Parse summary level from string
    ///
    /// # Errors
    ///
    /// Returns an error if the input string doesn't match any valid summary level
    pub fn parse(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "none" | "false" | "off" => Ok(Self::None),
            "minimal" | "min" => Ok(Self::Minimal),
            "standard" | "std" | "true" | "on" => Ok(Self::Standard),
            "detailed" | "detail" | "full" => Ok(Self::Detailed),
            _ => Err(format!(
                "Unknown summary level: {s}. Valid options: none, minimal, standard, detailed"
            )),
        }
    }

    /// Get string representation
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Minimal => "minimal",
            Self::Standard => "standard",
            Self::Detailed => "detailed",
        }
    }

    /// Check if summary extraction is enabled
    #[must_use]
    pub const fn is_enabled(&self) -> bool {
        !matches!(self, Self::None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summary_level_parse() {
        assert_eq!(SummaryLevel::parse("none").unwrap(), SummaryLevel::None);
        assert_eq!(SummaryLevel::parse("minimal").unwrap(), SummaryLevel::Minimal);
        assert_eq!(SummaryLevel::parse("standard").unwrap(), SummaryLevel::Standard);
        assert_eq!(SummaryLevel::parse("detailed").unwrap(), SummaryLevel::Detailed);
        assert!(SummaryLevel::parse("invalid").is_err());
    }

    #[test]
    fn test_summary_level_as_str() {
        assert_eq!(SummaryLevel::None.as_str(), "none");
        assert_eq!(SummaryLevel::Minimal.as_str(), "minimal");
        assert_eq!(SummaryLevel::Standard.as_str(), "standard");
        assert_eq!(SummaryLevel::Detailed.as_str(), "detailed");
    }

    #[test]
    fn test_summary_level_is_enabled() {
        assert!(!SummaryLevel::None.is_enabled());
        assert!(SummaryLevel::Minimal.is_enabled());
        assert!(SummaryLevel::Standard.is_enabled());
        assert!(SummaryLevel::Detailed.is_enabled());
    }
}