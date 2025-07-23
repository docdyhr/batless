use batless::{detect_language, highlight_content, process_file, BatlessConfig};
use clap::{Parser, ValueEnum};
use is_terminal::IsTerminal;
use serde_json::json;

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
    mode: OutputMode,

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
enum OutputMode {
    Plain,
    Highlight,
    Json,
    Summary,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ColorMode {
    Auto,
    Always,
    Never,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Handle list commands first
    if args.list_languages {
        for language in batless::list_languages() {
            println!("{language}");
        }
        return Ok(());
    }

    if args.list_themes {
        for theme in batless::list_themes() {
            println!("{theme}");
        }
        return Ok(());
    }

    // Require file for other operations
    let file_path = args
        .file
        .as_ref()
        .ok_or("File path required (unless using --list-languages or --list-themes)")?;

    // Determine if we should use color
    let use_color = match args.color {
        ColorMode::Always => true,
        ColorMode::Never => false,
        ColorMode::Auto => std::io::stdout().is_terminal(),
    };

    // Create configuration
    let config = BatlessConfig {
        max_lines: args.max_lines,
        max_bytes: args.max_bytes,
        language: args.language.clone(),
        theme: args.theme.clone(),
        strip_ansi: args.strip_ansi,
        use_color: use_color
            && (args.mode == OutputMode::Highlight || args.mode == OutputMode::Summary),
        include_tokens: args.include_tokens,
        summary_mode: args.summary || args.mode == OutputMode::Summary,
    };

    match args.mode {
        OutputMode::Plain => handle_plain_output(file_path, &config)?,
        OutputMode::Highlight => handle_highlight_output(file_path, &config)?,
        OutputMode::Json => handle_json_output(file_path, &config)?,
        OutputMode::Summary => handle_summary_output(file_path, &config)?,
    }

    Ok(())
}

fn handle_plain_output(
    file_path: &str,
    config: &BatlessConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_info = process_file(file_path, config)?;

    for line in &file_info.lines {
        if config.strip_ansi {
            println!("{}", strip_ansi_escapes::strip_str(line));
        } else {
            println!("{line}");
        }
    }

    if file_info.truncated_by_bytes {
        println!("// Output truncated after {} bytes", file_info.total_bytes);
    }
    if file_info.truncated_by_lines {
        println!("// Output truncated after {} lines", file_info.total_lines);
    }

    Ok(())
}

fn handle_highlight_output(
    file_path: &str,
    config: &BatlessConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    if !config.use_color || config.strip_ansi {
        // Fall back to plain output
        return handle_plain_output(file_path, config);
    }

    let file_info = process_file(file_path, config)?;
    let content = file_info.lines.join("\n");

    if !content.is_empty() {
        let highlighted = highlight_content(&content, file_path, config)?;
        print!("{highlighted}");

        // Add newline if content doesn't end with one
        if !content.ends_with('\n') {
            println!();
        }
    }

    if file_info.truncated_by_bytes {
        println!("// Output truncated after {} bytes", file_info.total_bytes);
    }
    if file_info.truncated_by_lines {
        println!("// Output truncated after {} lines", file_info.total_lines);
    }

    Ok(())
}

fn handle_json_output(
    file_path: &str,
    config: &BatlessConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_info = process_file(file_path, config)?;

    // Detect language if not specified
    let detected_language = if config.language.is_some() {
        config.language.clone()
    } else {
        detect_language(file_path)
    };

    let mut output = json!({
        "file": file_path,
        "lines": file_info.lines,
        "total_lines": file_info.total_lines,
        "total_bytes": file_info.total_bytes,
        "truncated": file_info.truncated,
        "truncated_by_lines": file_info.truncated_by_lines,
        "truncated_by_bytes": file_info.truncated_by_bytes,
        "language": detected_language,
        "encoding": file_info.encoding,
        "syntax_errors": file_info.syntax_errors,
        "mode": "json"
    });

    // Add optional fields
    if let Some(tokens) = file_info.tokens {
        output["tokens"] =
            serde_json::Value::Array(tokens.into_iter().map(serde_json::Value::String).collect());
    }

    if let Some(summary_lines) = file_info.summary_lines {
        output["summary_lines"] = serde_json::Value::Array(
            summary_lines
                .into_iter()
                .map(serde_json::Value::String)
                .collect(),
        );
    }

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

fn handle_summary_output(
    file_path: &str,
    config: &BatlessConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut summary_config = config.clone();
    summary_config.summary_mode = true;
    // Make sure we get all lines for proper summary extraction
    summary_config.max_lines = usize::MAX;

    let file_info = process_file(file_path, &summary_config)?;

    if let Some(summary_lines) = &file_info.summary_lines {
        if summary_lines.is_empty() {
            println!("// No summary-worthy code structures found");
            return Ok(());
        }

        // Display only the summary lines, not all content
        if config.use_color {
            let content = summary_lines.join("\n");
            let highlighted = highlight_content(&content, file_path, config)?;
            print!("{highlighted}");
            if !content.ends_with('\n') {
                println!();
            }
        } else {
            for line in summary_lines {
                if config.strip_ansi {
                    println!("{}", strip_ansi_escapes::strip_str(line));
                } else {
                    println!("{line}");
                }
            }
        }

        println!(
            "// Summary: {} important lines from {} total lines",
            summary_lines.len(),
            file_info.total_lines
        );
    } else {
        println!("// No summary available");
    }

    Ok(())
}
