//! Manages application configuration by merging settings from files,
//! command-line arguments, and profiles.

use crate::config::BatlessConfig;
use crate::error::{BatlessError, BatlessResult};
use crate::formatter::OutputMode;
use crate::summary::SummaryLevel;
use clap::{CommandFactory, FromArgMatches, Parser, ValueEnum};
use is_terminal::IsTerminal;
use std::str::FromStr;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// File to view
    pub file: Option<String>,

    /// Language for syntax highlighting (auto-detect if not specified)
    #[arg(long)]
    pub language: Option<String>,

    /// Limit lines shown
    #[arg(long)]
    pub max_lines: Option<usize>,

    /// Limit bytes shown
    #[arg(long)]
    pub max_bytes: Option<usize>,

    /// Output mode
    #[arg(long, value_enum)]
    pub mode: Option<CliOutputMode>,

    /// Color output control
    #[arg(long, value_enum, default_value = "auto")]
    pub color: ColorMode,

    /// Tracks whether --color was explicitly provided
    #[arg(skip)]
    pub color_specified: bool,

    /// Strip ANSI escape codes from output
    #[arg(long)]
    pub strip_ansi: bool,

    /// List all supported languages
    #[arg(long)]
    pub list_languages: bool,

    /// Summary mode: show only important code structures (deprecated, use --summary-level)
    #[arg(long)]
    pub summary: bool,

    /// Summary level: control detail level of summary output
    #[arg(long, value_enum)]
    pub summary_level: Option<CliSummaryLevel>,

    /// Generate shell completions for the specified shell
    #[arg(long, value_enum)]
    pub generate_completions: Option<Shell>,

    /// Configuration file path (defaults to auto-discovery)
    #[arg(long)]
    pub config: Option<String>,

    /// Enable debug mode with detailed processing information
    #[arg(long)]
    pub debug: bool,

    /// PAGER compatibility: equivalent to --mode plain (for cat replacement)
    #[arg(long)]
    pub plain: bool,

    /// PAGER compatibility: ignored for compatibility with other pagers
    #[arg(short = 'u', long)]
    pub unbuffered: bool,

    /// CAT compatibility: show line numbers (like cat -n)
    #[arg(short = 'n', long)]
    pub number: bool,

    /// CAT compatibility: number non-blank output lines (like cat -b)
    #[arg(short = 'b', long)]
    pub number_nonblank: bool,

    /// PAGER compatibility: ignored for compatibility with less (no title bar)
    #[arg(long)]
    pub no_title: bool,

    /// Output version information as machine-readable JSON
    #[arg(long)]
    pub version_json: bool,

    /// Pretty-print JSON output (when --mode=json); does not affect streaming
    #[arg(long)]
    pub json_pretty: bool,

    /// Include 1-based line numbers in JSON output lines array (e.g. {"n":1,"text":"..."})
    #[arg(long)]
    pub with_line_numbers: bool,

    /// Compute and include SHA-256 content hash in JSON output (for change detection)
    #[arg(long)]
    pub hash: bool,

    /// Strip comment-only lines from output
    #[arg(long)]
    pub strip_comments: bool,

    /// Strip blank lines from output
    #[arg(long)]
    pub strip_blank_lines: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CliOutputMode {
    Plain,
    Json,
    Summary,
    /// Machine-readable symbol index with line ranges
    Index,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    #[clap(name = "power-shell")]
    Power,
}

impl From<CliOutputMode> for OutputMode {
    fn from(mode: CliOutputMode) -> Self {
        match mode {
            CliOutputMode::Plain => Self::Plain,
            CliOutputMode::Json => Self::Json,
            CliOutputMode::Summary => Self::Summary,
            CliOutputMode::Index => Self::Index,
        }
    }
}

impl FromStr for OutputMode {
    type Err = BatlessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "plain" => Ok(Self::Plain),
            "json" => Ok(Self::Json),
            "summary" => Ok(Self::Summary),
            "index" => Ok(Self::Index),
            _ => Err(BatlessError::ConfigurationError {
                message: format!("Invalid output mode: {s}"),
                help: Some("Valid modes are: plain, json, summary, index".to_string()),
            }),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ColorMode {
    Auto,
    Always,
    Never,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CliSummaryLevel {
    /// No summary, show full file
    None,
    /// Minimal summary with only critical structures
    Minimal,
    /// Standard summary with most important code
    Standard,
    /// Detailed summary with comprehensive information
    Detailed,
}

impl From<CliSummaryLevel> for SummaryLevel {
    fn from(level: CliSummaryLevel) -> Self {
        match level {
            CliSummaryLevel::None => Self::None,
            CliSummaryLevel::Minimal => Self::Minimal,
            CliSummaryLevel::Standard => Self::Standard,
            CliSummaryLevel::Detailed => Self::Detailed,
        }
    }
}

/// Manages the application's configuration, merging settings from various
/// sources like files, command-line arguments, and profiles.
pub struct ConfigManager {
    args: Args,
    config: BatlessConfig,
    output_mode: OutputMode,
}

impl ConfigManager {
    /// Creates a new `ConfigManager` by parsing command-line arguments
    /// and loading configuration from files.
    pub fn new() -> BatlessResult<Self> {
        let command = Args::command();
        let matches = command.get_matches();
        let mut args = Args::from_arg_matches(&matches).map_err(|e| {
            BatlessError::config_error_with_help(
                format!("Failed to parse CLI arguments: {e}"),
                Some("Run `batless --help` for valid options".to_string()),
            )
        })?;
        args.color_specified = matches.contains_id("color");

        let mut manager = Self {
            args,
            config: BatlessConfig::default(),
            output_mode: OutputMode::Plain,
        };
        manager.load_and_apply_config()?;
        Ok(manager)
    }

    /// Creates a `ConfigManager` from a vector of argument strings.
    /// Useful for testing and programmatic usage.
    pub fn from_args_vec<I, T>(args: I) -> BatlessResult<Self>
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        let command = Args::command();
        let matches = command.try_get_matches_from(args).map_err(|e| {
            BatlessError::config_error_with_help(
                format!("Failed to parse arguments: {e}"),
                Some("Run `batless --help` for valid options".to_string()),
            )
        })?;
        let mut parsed_args = Args::from_arg_matches(&matches).map_err(|e| {
            BatlessError::config_error_with_help(
                format!("Failed to parse arguments: {e}"),
                Some("Run `batless --help` for valid options".to_string()),
            )
        })?;
        parsed_args.color_specified = matches.contains_id("color");

        let mut manager = Self {
            args: parsed_args,
            config: BatlessConfig::default(),
            output_mode: OutputMode::Plain,
        };
        manager.load_and_apply_config()?;
        Ok(manager)
    }

    /// Returns a reference to the parsed command-line arguments.
    pub const fn args(&self) -> &Args {
        &self.args
    }

    /// Returns a reference to the final, merged `BatlessConfig`.
    pub const fn config(&self) -> &BatlessConfig {
        &self.config
    }

    /// Returns the determined `OutputMode`.
    pub const fn output_mode(&self) -> OutputMode {
        self.output_mode
    }

    /// Determines the file path to process, handling stdin as well.
    pub fn file_path(&self) -> BatlessResult<String> {
        self.args.file.as_ref().map_or_else(
            || {
                if !std::io::IsTerminal::is_terminal(&std::io::stdin()) {
                    Ok("-".to_string())
                } else {
                    Err(BatlessError::config_error_with_help(
                        "File path required".to_string(),
                        Some(
                            "Specify a file to view, pipe input via stdin, or use --help for more options."
                                .to_string(),
                        ),
                    ))
                }
            },
            |file| Ok(file.clone()),
        )
    }

    /// Loads configuration from files, applies command-line arguments,
    /// and resolves profiles to create the final configuration.
    fn load_and_apply_config(&mut self) -> BatlessResult<()> {
        // 1. Load base configuration from files
        self.config = if let Some(config_path) = &self.args.config {
            let path = std::path::Path::new(config_path);
            if path.extension() == Some(std::ffi::OsStr::new("toml")) {
                BatlessConfig::from_file(path)?
            } else {
                BatlessConfig::from_json_file(path)?
            }
        } else {
            BatlessConfig::load_with_precedence()?
        };

        // 2. Apply command-line arguments
        self.apply_cli_args();

        // 3. Resolve output mode
        self.resolve_output_mode()?;

        // 4. Handle compatibility flags
        self.apply_compatibility_flags();

        // 5. Final validation
        self.config.validate()?;
        self.validate_language()?;

        Ok(())
    }

    /// Applies command-line arguments to the configuration.
    fn apply_cli_args(&mut self) {
        // Build a new config from the current one with all CLI args applied
        let mut new_config = std::mem::take(&mut self.config);

        if let Some(max_lines) = self.args.max_lines {
            new_config = new_config.with_max_lines(max_lines);
        }
        if self.args.max_bytes.is_some() {
            new_config = new_config.with_max_bytes(self.args.max_bytes);
        }
        if let Some(ref language) = self.args.language {
            new_config = new_config.with_language(Some(language.clone()));
        }
        if self.args.strip_ansi {
            new_config = new_config.with_strip_ansi(self.args.strip_ansi);
        }

        let use_color = if self.args.color_specified {
            match self.args.color {
                ColorMode::Always => true,
                ColorMode::Never => false,
                ColorMode::Auto => std::io::stdout().is_terminal(),
            }
        } else if new_config.use_color {
            std::io::stdout().is_terminal()
        } else {
            false
        };
        new_config = new_config.with_use_color(use_color);

        if self.args.json_pretty {
            new_config = new_config.with_pretty_json(true);
        }
        if self.args.with_line_numbers {
            new_config = new_config.with_json_line_numbers(true);
        }
        if self.args.hash {
            new_config = new_config.with_hash(true);
        }
        if self.args.strip_comments {
            new_config = new_config.with_strip_comments(true);
        }
        if self.args.strip_blank_lines {
            new_config = new_config.with_strip_blank_lines(true);
        }
        if self.args.debug {
            new_config = new_config.with_debug(self.args.debug);
        }
        if let Some(summary_level) = self.args.summary_level {
            new_config = new_config.with_summary_level(summary_level.into());
        } else if self.args.summary || self.args.mode == Some(CliOutputMode::Summary) {
            new_config = new_config.with_summary_mode(true);
        }

        self.config = new_config;
    }

    /// Resolves the output mode from the `--mode` flag.
    fn resolve_output_mode(&mut self) -> BatlessResult<()> {
        self.output_mode = self.args.mode.map_or(OutputMode::Plain, Into::into);
        Ok(())
    }

    /// Applies compatibility flags like `--plain` and `--number`.
    fn apply_compatibility_flags(&mut self) {
        if self.args.plain {
            self.output_mode = OutputMode::Plain;
            self.config = std::mem::take(&mut self.config).with_use_color(false);
        }
        if self.args.number {
            self.config = std::mem::take(&mut self.config).with_show_line_numbers(true);
        }
        if self.args.number_nonblank {
            self.config = std::mem::take(&mut self.config).with_show_line_numbers_nonblank(true);
        }
    }

    /// Validates the language setting.
    fn validate_language(&self) -> BatlessResult<()> {
        if let Some(ref lang) = self.config.language {
            crate::LanguageDetector::validate_language(lang)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_manager(args: &[&str]) -> ConfigManager {
        let mut full_args = vec!["batless"];
        full_args.extend_from_slice(args);
        ConfigManager::from_args_vec(full_args).unwrap()
    }

    #[test]
    fn test_default_output_mode() {
        let mgr = make_manager(&["Cargo.toml"]);
        assert_eq!(mgr.output_mode(), OutputMode::Plain);
    }

    #[test]
    fn test_plain_mode_flag() {
        let mgr = make_manager(&["--plain", "Cargo.toml"]);
        assert_eq!(mgr.output_mode(), OutputMode::Plain);
        assert!(!mgr.config().use_color);
    }

    #[test]
    fn test_json_mode() {
        let mgr = make_manager(&["--mode=json", "Cargo.toml"]);
        assert_eq!(mgr.output_mode(), OutputMode::Json);
    }

    #[test]
    fn test_summary_mode() {
        let mgr = make_manager(&["--mode=summary", "Cargo.toml"]);
        assert_eq!(mgr.output_mode(), OutputMode::Summary);
    }

    #[test]
    fn test_max_lines_applied() {
        let mgr = make_manager(&["--max-lines=42", "Cargo.toml"]);
        assert_eq!(mgr.config().max_lines, 42);
    }

    #[test]
    fn test_max_bytes_applied() {
        let mgr = make_manager(&["--max-bytes=1024", "--max-lines=10", "Cargo.toml"]);
        assert_eq!(mgr.config().max_bytes, Some(1024));
    }

    #[test]
    fn test_language_override() {
        let mgr = make_manager(&["--language=python", "Cargo.toml"]);
        assert_eq!(mgr.config().language, Some("python".to_string()));
    }

    #[test]
    fn test_strip_ansi() {
        let mgr = make_manager(&["--strip-ansi", "Cargo.toml"]);
        assert!(mgr.config().strip_ansi);
    }

    #[test]
    fn test_line_numbers_flag() {
        let mgr = make_manager(&["-n", "--plain", "Cargo.toml"]);
        assert!(mgr.config().show_line_numbers);
    }

    #[test]
    fn test_number_nonblank_flag() {
        let mgr = make_manager(&["-b", "--plain", "Cargo.toml"]);
        assert!(mgr.config().show_line_numbers_nonblank);
    }

    #[test]
    fn test_debug_flag() {
        let mgr = make_manager(&["--debug", "Cargo.toml"]);
        assert!(mgr.config().debug);
    }

    #[test]
    fn test_summary_flag_enables_summary() {
        let mgr = make_manager(&["--summary", "Cargo.toml"]);
        assert!(mgr.config().summary_level.is_enabled());
    }

    #[test]
    fn test_summary_level_override() {
        let mgr = make_manager(&["--summary-level=minimal", "Cargo.toml"]);
        assert_eq!(mgr.config().summary_level, SummaryLevel::Minimal);
    }

    #[test]
    fn test_summary_level_detailed() {
        let mgr = make_manager(&["--summary-level=detailed", "Cargo.toml"]);
        assert_eq!(mgr.config().summary_level, SummaryLevel::Detailed);
    }

    #[test]
    fn test_color_never() {
        let mgr = make_manager(&["--color=never", "Cargo.toml"]);
        assert!(!mgr.config().use_color);
    }

    #[test]
    fn test_json_pretty() {
        let mgr = make_manager(&["--json-pretty", "--mode=json", "Cargo.toml"]);
        assert!(mgr.config().pretty_json);
    }

    #[test]
    fn test_file_path_from_arg() {
        let mgr = make_manager(&["Cargo.toml"]);
        assert_eq!(mgr.file_path().unwrap(), "Cargo.toml");
    }

    #[test]
    fn test_file_path_missing_errors() {
        let mgr = make_manager(&["--mode=plain"]);
        let result = mgr.file_path();
        if std::io::IsTerminal::is_terminal(&std::io::stdin()) {
            assert!(result.is_err());
        } else {
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_invalid_language_rejected() {
        let result = ConfigManager::from_args_vec([
            "batless",
            "--language=nonexistent_lang_xyz",
            "Cargo.toml",
        ]);
        assert!(result.is_err());
    }

    #[test]
    fn test_output_mode_from_str() {
        assert_eq!(OutputMode::from_str("plain").unwrap(), OutputMode::Plain);
        assert_eq!(OutputMode::from_str("json").unwrap(), OutputMode::Json);
        assert_eq!(
            OutputMode::from_str("summary").unwrap(),
            OutputMode::Summary
        );
        assert_eq!(OutputMode::from_str("index").unwrap(), OutputMode::Index);
        assert!(OutputMode::from_str("highlight").is_err());
        assert!(OutputMode::from_str("invalid").is_err());
    }

    #[test]
    fn test_cli_output_mode_conversion() {
        assert_eq!(OutputMode::from(CliOutputMode::Plain), OutputMode::Plain);
        assert_eq!(OutputMode::from(CliOutputMode::Json), OutputMode::Json);
        assert_eq!(
            OutputMode::from(CliOutputMode::Summary),
            OutputMode::Summary
        );
        assert_eq!(OutputMode::from(CliOutputMode::Index), OutputMode::Index);
    }

    #[test]
    fn test_cli_summary_level_conversion() {
        assert_eq!(
            SummaryLevel::from(CliSummaryLevel::None),
            SummaryLevel::None
        );
        assert_eq!(
            SummaryLevel::from(CliSummaryLevel::Minimal),
            SummaryLevel::Minimal
        );
        assert_eq!(
            SummaryLevel::from(CliSummaryLevel::Standard),
            SummaryLevel::Standard
        );
        assert_eq!(
            SummaryLevel::from(CliSummaryLevel::Detailed),
            SummaryLevel::Detailed
        );
    }

    #[test]
    fn test_plain_flag_overrides_mode() {
        // --plain should override --mode=json
        let mgr = make_manager(&["--mode=json", "--plain", "Cargo.toml"]);
        assert_eq!(mgr.output_mode(), OutputMode::Plain);
    }

    #[test]
    fn test_multiple_flags_combined() {
        let mgr = make_manager(&[
            "--max-lines=100",
            "--max-bytes=5000",
            "--mode=json",
            "Cargo.toml",
        ]);
        assert_eq!(mgr.config().max_lines, 100);
        assert_eq!(mgr.config().max_bytes, Some(5000));
        assert_eq!(mgr.output_mode(), OutputMode::Json);
    }

    #[test]
    fn test_from_args_vec_invalid_args() {
        let result = ConfigManager::from_args_vec(["batless", "--nonexistent-flag"]);
        assert!(result.is_err());
    }
}
