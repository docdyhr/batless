use batless::{
    format_output, process_file, BatlessConfig, BatlessResult, LanguageDetector, OutputMode,
    ThemeManager,
};
use clap::{Parser, ValueEnum};
use is_terminal::IsTerminal;

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
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum CliOutputMode {
    Plain,
    Highlight,
    Json,
    Summary,
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
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> BatlessResult<()> {
    let args = Args::parse();

    // Handle list commands first
    if args.list_languages {
        for language in LanguageDetector::list_languages() {
            println!("{language}");
        }
        return Ok(());
    }

    if args.list_themes {
        for theme in ThemeManager::list_themes() {
            println!("{theme}");
        }
        return Ok(());
    }

    // Require file for other operations
    let file_path = args.file.as_ref().ok_or_else(|| {
        batless::BatlessError::ConfigurationError(
            "File path required (unless using --list-languages or --list-themes)".to_string(),
        )
    })?;

    // Validate language if specified
    if let Some(ref lang) = args.language {
        LanguageDetector::validate_language(lang)?;
    }

    // Validate theme
    ThemeManager::validate_theme(&args.theme)?;

    // Determine if we should use color
    let use_color = match args.color {
        ColorMode::Always => true,
        ColorMode::Never => false,
        ColorMode::Auto => std::io::stdout().is_terminal(),
    };

    // Create configuration
    let config = BatlessConfig::new()
        .with_max_lines(args.max_lines)
        .with_max_bytes(args.max_bytes)
        .with_language(args.language)
        .with_theme(args.theme)
        .with_strip_ansi(args.strip_ansi)
        .with_use_color(
            use_color
                && (args.mode == CliOutputMode::Highlight || args.mode == CliOutputMode::Summary),
        )
        .with_include_tokens(args.include_tokens)
        .with_summary_mode(args.summary || args.mode == CliOutputMode::Summary);

    // Validate configuration
    config.validate()?;

    // Process the file
    let file_info = process_file(file_path, &config)?;

    // Format and output the result
    let output_mode = OutputMode::from(args.mode);

    // Handle summary mode with no important lines
    if output_mode == OutputMode::Summary && file_info.summary_line_count() == 0 {
        println!("// No summary-worthy code structures found");
        return Ok(());
    }

    let formatted_output = format_output(&file_info, file_path, &config, output_mode)?;

    print!("{}", formatted_output);

    // Add truncation messages for non-JSON modes
    if output_mode != OutputMode::Json {
        if file_info.truncated_by_lines {
            println!("\n// Output truncated after {} lines", config.max_lines);
        }
        if file_info.truncated_by_bytes {
            if let Some(max_bytes) = config.max_bytes {
                println!("\n// Output truncated after {} bytes", max_bytes);
            }
        }
    }

    Ok(())
}
