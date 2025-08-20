//! Manages application configuration by merging settings from files,
//! command-line arguments, and profiles.

use crate::config::{BatlessConfig, CustomProfile, SummaryLevel};
use crate::error::{BatlessError, BatlessResult};
use crate::formatter::OutputMode;
use crate::token_counter::AiModel;
use clap::{Parser, ValueEnum};
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
    #[arg(long, default_value = "10000")]
    pub max_lines: usize,

    /// Limit bytes shown
    #[arg(long)]
    pub max_bytes: Option<usize>,

    /// Output mode
    #[arg(long, value_enum, default_value = "highlight")]
    pub mode: CliOutputMode,

    /// Color output control
    #[arg(long, value_enum, default_value = "auto")]
    pub color: ColorMode,

    /// Theme for syntax highlighting
    #[arg(long, default_value = "base16-ocean.dark")]
    pub theme: String,

    /// Strip ANSI escape codes from output
    #[arg(long)]
    pub strip_ansi: bool,

    /// List all supported languages
    #[arg(long)]
    pub list_languages: bool,

    /// List all available themes
    #[arg(long)]
    pub list_themes: bool,

    /// Include tokens in JSON output (AI-friendly)
    #[arg(long)]
    pub include_tokens: bool,

    /// Summary mode: show only important code structures (deprecated, use --summary-level)
    #[arg(long)]
    pub summary: bool,

    /// Summary level: control detail level of summary output
    #[arg(long, value_enum)]
    pub summary_level: Option<CliSummaryLevel>,

    /// Count tokens for AI model context estimation
    #[arg(long)]
    pub count_tokens: bool,

    /// AI model for token counting
    #[arg(long, value_enum, default_value = "generic")]
    pub ai_model: CliAiModel,

    /// Fit content within AI model context window (truncate if needed)
    #[arg(long)]
    pub fit_context: bool,

    /// Estimate prompt token overhead when fitting context
    #[arg(long, default_value = "500")]
    pub prompt_tokens: usize,

    /// Validate JSON output against schema
    #[arg(long)]
    pub validate_json: bool,

    /// Get JSON schema for specified output format
    #[arg(long)]
    pub get_schema: Option<String>,

    /// Generate shell completions for the specified shell
    #[arg(long, value_enum)]
    pub generate_completions: Option<Shell>,

    /// Use predefined AI tool profile (overrides other settings)
    #[arg(long, value_enum)]
    pub profile: Option<AiProfile>,

    /// Load custom AI profile from file
    #[arg(long)]
    pub custom_profile: Option<String>,

    /// Configuration file path (defaults to auto-discovery)
    #[arg(long)]
    pub config: Option<String>,

    /// Enable streaming JSON output for large files
    #[arg(long)]
    pub streaming_json: bool,

    /// Chunk size for streaming output (in lines)
    #[arg(long, default_value = "1000")]
    pub streaming_chunk_size: usize,

    /// Enable resume capability with checkpoint support
    #[arg(long)]
    pub enable_resume: bool,

    /// Checkpoint file path for resuming
    #[arg(long)]
    pub checkpoint: Option<String>,

    /// Run interactive configuration wizard
    #[arg(long)]
    pub configure: bool,

    /// List available custom profiles
    #[arg(long)]
    pub list_profiles: bool,

    /// Edit existing custom profile
    #[arg(long)]
    pub edit_profile: Option<String>,

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
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CliOutputMode {
    Plain,
    Highlight,
    Json,
    Summary,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    #[clap(name = "power-shell")]
    Power,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum AiProfile {
    /// Optimized for Claude's context window (4K lines, summary mode)
    Claude,
    /// Focused on code suggestions for GitHub Copilot (2K lines, tokens included)
    Copilot,
    /// OpenAI ChatGPT optimizations (3K lines, JSON output)
    Chatgpt,
    /// General AI assistant profile (5K lines, balanced output)
    Assistant,
}

impl AiProfile {
    pub fn apply_to_config(self, config: BatlessConfig) -> BatlessConfig {
        match self {
            AiProfile::Claude => config
                .with_max_lines(4000)
                .with_summary_level(SummaryLevel::Standard)
                .with_include_tokens(false)
                .with_use_color(false),
            AiProfile::Copilot => config
                .with_max_lines(2000)
                .with_include_tokens(true)
                .with_summary_level(SummaryLevel::None)
                .with_use_color(false),
            AiProfile::Chatgpt => config
                .with_max_lines(3000)
                .with_include_tokens(true)
                .with_summary_level(SummaryLevel::None)
                .with_use_color(false),
            AiProfile::Assistant => config
                .with_max_lines(5000)
                .with_include_tokens(false)
                .with_summary_level(SummaryLevel::Detailed)
                .with_use_color(false),
        }
    }

    pub fn get_output_mode(self) -> OutputMode {
        match self {
            AiProfile::Claude => OutputMode::Summary,
            AiProfile::Copilot => OutputMode::Json,
            AiProfile::Chatgpt => OutputMode::Json,
            AiProfile::Assistant => OutputMode::Summary,
        }
    }
}

impl From<CliOutputMode> for OutputMode {
    fn from(mode: CliOutputMode) -> Self {
        match mode {
            CliOutputMode::Plain => OutputMode::Plain,
            CliOutputMode::Highlight => OutputMode::Highlight,
            CliOutputMode::Json => OutputMode::Json,
            CliOutputMode::Summary => OutputMode::Summary,
        }
    }
}

impl FromStr for OutputMode {
    type Err = BatlessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "plain" => Ok(OutputMode::Plain),
            "highlight" => Ok(OutputMode::Highlight),
            "json" => Ok(OutputMode::Json),
            "summary" => Ok(OutputMode::Summary),
            _ => Err(BatlessError::ConfigurationError {
                message: format!("Invalid output mode: {s}"),
                help: Some("Valid modes are: plain, highlight, json, summary".to_string()),
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
            CliSummaryLevel::None => SummaryLevel::None,
            CliSummaryLevel::Minimal => SummaryLevel::Minimal,
            CliSummaryLevel::Standard => SummaryLevel::Standard,
            CliSummaryLevel::Detailed => SummaryLevel::Detailed,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CliAiModel {
    /// OpenAI GPT-4 family
    Gpt4,
    /// OpenAI GPT-4 Turbo with enhanced capabilities
    Gpt4Turbo,
    /// OpenAI GPT-3.5 family
    Gpt35,
    /// Anthropic Claude family
    Claude,
    /// Anthropic Claude-3.5 Sonnet with enhanced capabilities
    Claude35Sonnet,
    /// Generic model estimation
    Generic,
}

impl From<CliAiModel> for AiModel {
    fn from(model: CliAiModel) -> Self {
        match model {
            CliAiModel::Gpt4 => AiModel::Gpt4,
            CliAiModel::Gpt4Turbo => AiModel::Gpt4Turbo,
            CliAiModel::Gpt35 => AiModel::Gpt35,
            CliAiModel::Claude => AiModel::Claude,
            CliAiModel::Claude35Sonnet => AiModel::Claude35Sonnet,
            CliAiModel::Generic => AiModel::Generic,
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
        let args = Args::parse();
        let mut manager = Self {
            args,
            config: BatlessConfig::default(),
            output_mode: OutputMode::Highlight,
        };
        manager.load_and_apply_config()?;
        Ok(manager)
    }

    /// Returns a reference to the parsed command-line arguments.
    pub fn args(&self) -> &Args {
        &self.args
    }

    /// Returns a reference to the final, merged `BatlessConfig`.
    pub fn config(&self) -> &BatlessConfig {
        &self.config
    }

    /// Returns the determined `OutputMode`.
    pub fn output_mode(&self) -> OutputMode {
        self.output_mode
    }

    /// Determines the file path to process, handling stdin as well.
    pub fn file_path(&self) -> BatlessResult<String> {
        if let Some(file) = self.args.file.as_ref() {
            Ok(file.clone())
        } else if !std::io::IsTerminal::is_terminal(&std::io::stdin()) {
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

        // 3. Apply AI profiles (which can override previous settings)
        self.apply_profiles()?;

        // 4. Handle compatibility flags
        self.apply_compatibility_flags();

        // 5. Final validation
        self.config.validate()?;
        self.validate_language_and_theme()?;

        Ok(())
    }

    /// Applies command-line arguments to the configuration.
    fn apply_cli_args(&mut self) {
        // Build a new config from the current one with all CLI args applied
        let mut new_config = std::mem::take(&mut self.config);
        
        if self.args.max_lines != 10000 {
            new_config = new_config.with_max_lines(self.args.max_lines);
        }
        if self.args.max_bytes.is_some() {
            new_config = new_config.with_max_bytes(self.args.max_bytes);
        }
        if let Some(ref language) = self.args.language {
            new_config = new_config.with_language(Some(language.clone()));
        }
        if self.args.theme != "base16-ocean.dark" {
            new_config = new_config.with_theme(self.args.theme.clone());
        }
        if self.args.strip_ansi {
            new_config = new_config.with_strip_ansi(self.args.strip_ansi);
        }
        
        let use_color = match self.args.color {
            ColorMode::Always => true,
            ColorMode::Never => false,
            ColorMode::Auto => std::io::stdout().is_terminal(),
        };
        new_config = new_config.with_use_color(use_color);

        if self.args.include_tokens {
            new_config = new_config.with_include_tokens(self.args.include_tokens);
        }
        if self.args.streaming_json {
            new_config = new_config.with_streaming_json(self.args.streaming_json);
        }
        if self.args.json_pretty {
            new_config = new_config.with_pretty_json(true);
        }
        if self.args.streaming_chunk_size != 1000 {
            new_config = new_config.with_streaming_chunk_size(self.args.streaming_chunk_size);
        }
        if self.args.enable_resume {
            new_config = new_config.with_enable_resume(self.args.enable_resume);
        }
        if self.args.debug {
            new_config = new_config.with_debug(self.args.debug);
        }
        if let Some(summary_level) = self.args.summary_level {
            new_config = new_config.with_summary_level(summary_level.into());
        } else if self.args.summary || self.args.mode == CliOutputMode::Summary {
            new_config = new_config.with_summary_mode(true);
        }
        
        self.config = new_config;
    }

    /// Applies AI profiles to the configuration.
    fn apply_profiles(&mut self) -> BatlessResult<()> {
        self.output_mode = if let Some(custom_profile_path) = &self.args.custom_profile {
            let custom_profile = CustomProfile::load_from_file(custom_profile_path)?;
            self.config = custom_profile.apply_to_config(std::mem::take(&mut self.config));
            custom_profile
                .get_output_mode()
                .and_then(|mode| mode.parse().ok())
                .unwrap_or_else(|| self.args.mode.into())
        } else if let Some(profile) = self.args.profile {
            self.config = profile.apply_to_config(std::mem::take(&mut self.config));
            profile.get_output_mode()
        } else {
            self.args.mode.into()
        };
        Ok(())
    }

    /// Applies compatibility flags like `--plain` and `--number`.
    fn apply_compatibility_flags(&mut self) {
        if self.args.plain {
            self.output_mode = OutputMode::Plain;
            self.config = std::mem::replace(&mut self.config, BatlessConfig::default())
                .with_use_color(false);
        }
        if self.args.number {
            self.config = std::mem::replace(&mut self.config, BatlessConfig::default())
                .with_show_line_numbers(true);
        }
        if self.args.number_nonblank {
            self.config = std::mem::replace(&mut self.config, BatlessConfig::default())
                .with_show_line_numbers_nonblank(true);
        }
    }

    /// Validates the language and theme settings.
    fn validate_language_and_theme(&self) -> BatlessResult<()> {
        if let Some(ref lang) = self.config.language {
            crate::LanguageDetector::validate_language(lang)?;
        }
        crate::ThemeManager::validate_theme(&self.config.theme)?;
        Ok(())
    }
}
