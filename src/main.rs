use batless::{detect_language, highlight_content, process_file, BatlessConfig};
use clap::{Parser, ValueEnum};
use serde_json::json;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File to view
    file: String,

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
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputMode {
    Plain,
    Highlight,
    Json,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ColorMode {
    Auto,
    Always,
    Never,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Determine if we should use color
    let use_color = match args.color {
        ColorMode::Always => true,
        ColorMode::Never => false,
        ColorMode::Auto => atty::is(atty::Stream::Stdout),
    };

    // Create configuration
    let config = BatlessConfig {
        max_lines: args.max_lines,
        max_bytes: args.max_bytes,
        language: args.language.clone(),
        theme: args.theme.clone(),
        strip_ansi: args.strip_ansi,
        use_color: use_color && args.mode == OutputMode::Highlight,
    };

    match args.mode {
        OutputMode::Plain => handle_plain_output(&args.file, &config)?,
        OutputMode::Highlight => handle_highlight_output(&args.file, &config)?,
        OutputMode::Json => handle_json_output(&args.file, &config)?,
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

    let output = json!({
        "file": file_path,
        "lines": file_info.lines,
        "total_lines": file_info.total_lines,
        "total_bytes": file_info.total_bytes,
        "truncated": file_info.truncated,
        "language": detected_language,
        "mode": "json"
    });

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
