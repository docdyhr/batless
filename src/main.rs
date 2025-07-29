use batless::{BatlessConfig, BatlessResult, OutputMode};
use clap::{CommandFactory, Parser, ValueEnum};
use clap_complete::{generate, shells::*};
use is_terminal::IsTerminal;
use std::io;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File to view
    file: Option<String>,

    /// Language for syntax highlighting (auto-detect if not specified)
    #[arg(long)]
    language: Option<String>,

    /// Limit lines shown
    #[arg(long, default_value = "10000")]
    max_lines: usize,

    /// Limit bytes shown
    #[arg(long)]
    max_bytes: Option<usize>,

    /// Output mode
    #[arg(long, value_enum, default_value = "highlight")]
    mode: CliOutputMode,

    /// Color output control
    #[arg(long, value_enum, default_value = "auto")]
    color: ColorMode,

    /// Theme for syntax highlighting
    #[arg(long, default_value = "base16-ocean.dark")]
    theme: String,

    /// Strip ANSI escape codes from output
    #[arg(long)]
    strip_ansi: bool,

    /// List all supported languages
    #[arg(long)]
    list_languages: bool,

    /// List all available themes
    #[arg(long)]
    list_themes: bool,

    /// Include tokens in JSON output (AI-friendly)
    #[arg(long)]
    include_tokens: bool,

    /// Summary mode: show only important code structures
    #[arg(long)]
    summary: bool,

    /// Generate shell completions for the specified shell
    #[arg(long, value_enum)]
    generate_completions: Option<Shell>,

    /// Use predefined AI tool profile (overrides other settings)
    #[arg(long, value_enum)]
    profile: Option<AiProfile>,

    /// Configuration file path (defaults to auto-discovery)
    #[arg(long)]
    config: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum CliOutputMode {
    Plain,
    Highlight,
    Json,
    Summary,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Shell {
    Bash,
    Zsh,
    Fish,
    #[clap(name = "power-shell")]
    Power,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum AiProfile {
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
    fn apply_to_config(self, config: BatlessConfig) -> BatlessConfig {
        match self {
            AiProfile::Claude => config
                .with_max_lines(4000)
                .with_summary_mode(true)
                .with_include_tokens(false)
                .with_use_color(false),
            AiProfile::Copilot => config
                .with_max_lines(2000)
                .with_include_tokens(true)
                .with_summary_mode(false)
                .with_use_color(false),
            AiProfile::Chatgpt => config
                .with_max_lines(3000)
                .with_include_tokens(true)
                .with_summary_mode(false)
                .with_use_color(false),
            AiProfile::Assistant => config
                .with_max_lines(5000)
                .with_include_tokens(false)
                .with_summary_mode(true)
                .with_use_color(false),
        }
    }

    fn get_output_mode(self) -> OutputMode {
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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ColorMode {
    Auto,
    Always,
    Never,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run() -> BatlessResult<()> {
    let args = Args::parse();

    // Handle completion generation first
    if let Some(shell) = args.generate_completions {
        let mut cmd = Args::command();
        let name = cmd.get_name().to_string();

        match shell {
            Shell::Bash => generate(Bash, &mut cmd, name, &mut io::stdout()),
            Shell::Zsh => generate(Zsh, &mut cmd, name, &mut io::stdout()),
            Shell::Fish => generate(Fish, &mut cmd, name, &mut io::stdout()),
            Shell::Power => generate(PowerShell, &mut cmd, name, &mut io::stdout()),
        }
        return Ok(());
    }

    // Handle list commands
    if args.list_languages {
        for language in batless::LanguageDetector::list_languages() {
            println!("{language}");
        }
        return Ok(());
    }

    if args.list_themes {
        for theme in batless::ThemeManager::list_themes() {
            println!("{theme}");
        }
        return Ok(());
    }

    // Require file for other operations
    let file_path = args.file.as_ref().ok_or_else(|| {
        batless::BatlessError::config_error_with_help(
            "File path required (unless using --list-languages or --list-themes)".to_string(),
            Some("Specify a file to view, or use --help for more options".to_string()),
        )
    })?;

    // Determine if we should use color
    let use_color = match args.color {
        ColorMode::Always => true,
        ColorMode::Never => false,
        ColorMode::Auto => std::io::stdout().is_terminal(),
    };

    // Load base configuration from files first
    let mut config = if let Some(config_path) = &args.config {
        // Load from specific config file
        let path = std::path::Path::new(config_path);
        if path.extension() == Some(std::ffi::OsStr::new("toml")) {
            BatlessConfig::from_file(path)?
        } else {
            BatlessConfig::from_json_file(path)?
        }
    } else {
        // Load with precedence from standard locations
        BatlessConfig::load_with_precedence()?
    };

    // Apply CLI arguments (highest precedence)
    if args.max_lines != 10000 {
        // Only override if not default
        config = config.with_max_lines(args.max_lines);
    }
    if args.max_bytes.is_some() {
        config = config.with_max_bytes(args.max_bytes);
    }
    if args.language.is_some() {
        config = config.with_language(args.language);
    }
    if args.theme != "base16-ocean.dark" {
        // Only override if not default
        config = config.with_theme(args.theme);
    }
    if args.strip_ansi {
        config = config.with_strip_ansi(args.strip_ansi);
    }
    config = config.with_use_color(
        use_color && (args.mode == CliOutputMode::Highlight || args.mode == CliOutputMode::Summary),
    );
    if args.include_tokens {
        config = config.with_include_tokens(args.include_tokens);
    }
    if args.summary || args.mode == CliOutputMode::Summary {
        config = config.with_summary_mode(true);
    }

    // Apply AI profile if specified (overrides other settings)
    let output_mode = if let Some(profile) = args.profile {
        config = profile.apply_to_config(config);
        profile.get_output_mode()
    } else {
        OutputMode::from(args.mode)
    };

    // Validate configuration
    config.validate()?;

    // Validate language and theme just before processing (to avoid loading heavy syntax sets for fast operations)
    if let Some(ref lang) = config.language {
        batless::LanguageDetector::validate_language(lang)?;
    }
    batless::ThemeManager::validate_theme(&config.theme)?;

    // Process the file
    let file_info = batless::process_file(file_path, &config)?;

    // Format and output the result

    // Handle summary mode with no important lines
    if output_mode == OutputMode::Summary && file_info.summary_line_count() == 0 {
        println!("// No summary-worthy code structures found");
        return Ok(());
    }

    let formatted_output = batless::format_output(&file_info, file_path, &config, output_mode)?;

    // Print output with newline only for JSON mode to avoid shell prompt appearing
    if output_mode == OutputMode::Json {
        println!("{formatted_output}");
    } else {
        print!("{formatted_output}");
    }

    // Add truncation messages for non-JSON modes
    if output_mode != OutputMode::Json {
        if file_info.truncated_by_lines {
            println!("\n// Output truncated after {} lines", config.max_lines);
        }
        if file_info.truncated_by_bytes {
            if let Some(max_bytes) = config.max_bytes {
                println!("\n// Output truncated after {max_bytes} bytes");
            }
        }
    }

    Ok(())
}
