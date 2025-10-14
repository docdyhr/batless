use batless::{
    config_manager::ConfigManager, AiModel, BatlessError, BatlessResult, JsonSchemaValidator,
    OutputMode, TokenCounter,
};
use clap::CommandFactory;
use clap_complete::generate;
use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

// The Args struct and its enums are now defined and re-exported from the config_manager module.
use batless::config_manager::{Args, Shell};

fn print_error(error: &BatlessError) {
    let mut stderr = StandardStream::stderr(ColorChoice::Auto);
    let error_string = error.to_string();
    let mut parts = error_string.splitn(2, '\n');
    let first_line = parts.next().unwrap_or("");
    let rest = parts.next().unwrap_or("");

    let _ = stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true));
    let _ = write!(&mut stderr, "Error");

    if let Some(end_of_code) = first_line.find(']') {
        let code_part = &first_line[..=end_of_code];
        let message_part = &first_line[end_of_code + 1..];
        let _ = stderr.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
        let _ = write!(&mut stderr, " {code_part}");
        let _ = stderr.reset();
        let _ = writeln!(&mut stderr, "{message_part}");
    } else {
        let _ = stderr.reset();
        let _ = writeln!(&mut stderr, ": {first_line}");
    }

    if !rest.is_empty() {
        let _ = stderr.reset();
        let _ = writeln!(&mut stderr, "\n{rest}");
    }
}

fn main() {
    // Check for common unsupported features before parsing
    let args: Vec<String> = std::env::args().collect();

    // Check for --pattern / -p (but not -p from existing flags like --plain)
    if args.iter().any(|a| a == "--pattern" || a == "-p") {
        print_pattern_not_supported();
        std::process::exit(1);
    }

    // Check for --list / -l
    if args.iter().any(|a| a == "--list" || a == "-l") {
        print_list_not_supported();
        std::process::exit(1);
    }

    // Check for --range or -r with value (but not standalone -r which doesn't exist anyway)
    if args.iter().any(|a| {
        a == "--range"
            || a.starts_with("--range=")
            || a.starts_with("-r=")
            || (a.starts_with("-r") && a.len() > 2 && !a.starts_with("--"))
    }) {
        print_range_not_supported();
        std::process::exit(1);
    }

    if let Err(e) = run() {
        print_error(&e);
        std::process::exit(e.error_code() as i32);
    }
}

fn run() -> BatlessResult<()> {
    let config_manager = ConfigManager::new()?;
    let args = config_manager.args();
    let config = config_manager.config();
    let output_mode = config_manager.output_mode();

    // Handle commands that don't require file processing
    if handle_special_commands(args)? {
        return Ok(());
    }

    let file_path = config_manager.file_path()?;

    if config.streaming_json && output_mode == OutputMode::Json {
        return handle_streaming_json(&file_path, &config_manager);
    }

    handle_normal_processing(&file_path, &config_manager)
}

fn handle_special_commands(args: &Args) -> BatlessResult<bool> {
    if args.version_json {
        // Collect build-time metadata populated by build script (if any)
        // Fallbacks ensure robustness in absence of environment variables.
        let version = env!("CARGO_PKG_VERSION");
        let name = env!("CARGO_PKG_NAME");
        let build_git_hash = option_env!("BATLESS_GIT_HASH").unwrap_or("unknown");
        let build_timestamp = option_env!("BATLESS_BUILD_TIMESTAMP").unwrap_or("unknown");
        let pkg_authors = env!("CARGO_PKG_AUTHORS");
        let json = serde_json::json!({
            "name": name,
            "version": version,
            "git_hash": build_git_hash,
            "build_timestamp": build_timestamp,
            "authors": pkg_authors,
        });
        println!("{}", serde_json::to_string_pretty(&json)?);
        return Ok(true);
    }
    if let Some(shell) = args.generate_completions {
        let mut cmd = Args::command();
        let name = cmd.get_name().to_string();
        let mut buffer: Vec<u8> = Vec::new();
        match shell {
            Shell::Bash => generate(clap_complete::shells::Bash, &mut cmd, name, &mut buffer),
            Shell::Zsh => generate(clap_complete::shells::Zsh, &mut cmd, name, &mut buffer),
            Shell::Fish => generate(clap_complete::shells::Fish, &mut cmd, name, &mut buffer),
            Shell::Power => generate(
                clap_complete::shells::PowerShell,
                &mut cmd,
                name,
                &mut buffer,
            ),
        }
        io::stdout().write_all(&buffer)?;
        return Ok(true);
    }

    if let Some(format) = &args.get_schema {
        let validator = JsonSchemaValidator::new();
        let schema = validator.get_schema(format).ok_or_else(|| {
            BatlessError::config_error_with_help(
                format!("Unknown schema format '{format}'"),
                Some(
                    "Available schemas: file_info, json_output, token_count, processing_stats"
                        .to_string(),
                ),
            )
        })?;
        let pretty = serde_json::to_string_pretty(schema)?; // pretty JSON already created
        println!("{pretty}");
        return Ok(true);
    }

    if args.list_languages {
        for language in batless::LanguageDetector::list_languages() {
            println!("{language}");
        }
        return Ok(true);
    }

    if args.list_themes {
        for theme in batless::ThemeManager::list_themes() {
            println!("{theme}");
        }
        return Ok(true);
    }

    if args.configure {
        batless::ConfigurationWizard::run()?;
        return Ok(true);
    }

    if args.list_profiles {
        batless::ConfigurationWizard::list_profiles()?;
        return Ok(true);
    }

    if let Some(profile_path) = &args.edit_profile {
        batless::ConfigurationWizard::edit_profile_by_path(profile_path)?;
        return Ok(true);
    }

    Ok(false)
}

fn handle_streaming_json(file_path: &str, manager: &ConfigManager) -> BatlessResult<()> {
    use batless::StreamingProcessor;

    let config = manager.config();
    let args = manager.args();

    let checkpoint = if config.enable_resume {
        args.checkpoint
            .as_ref()
            .and_then(|path| {
                if std::path::Path::new(path).exists() {
                    Some(StreamingProcessor::load_checkpoint(std::path::Path::new(
                        path,
                    )))
                } else {
                    None
                }
            })
            .transpose()?
    } else {
        None
    };

    let chunks = StreamingProcessor::process_streaming(file_path, config, checkpoint)?;

    for chunk_result in chunks {
        let chunk = chunk_result?;
        let json_output = serde_json::to_string_pretty(&chunk)?;
        println!("{json_output}");

        if config.enable_resume && !chunk.is_final {
            if let Some(checkpoint_path) = &args.checkpoint {
                StreamingProcessor::save_checkpoint(
                    &chunk.checkpoint,
                    std::path::Path::new(checkpoint_path),
                )?;
            }
        }

        if !chunk.is_final {
            println!("---");
        }
    }
    Ok(())
}

fn handle_normal_processing(file_path: &str, manager: &ConfigManager) -> BatlessResult<()> {
    let config = manager.config();
    let args = manager.args();
    let output_mode = manager.output_mode();

    let start_time = std::time::Instant::now();
    if config.debug {
        eprintln!("🔍 DEBUG: Starting file processing for {file_path}");
    }

    let file_info = batless::process_file(file_path, config)?;

    if config.debug {
        eprintln!(
            "🔍 DEBUG: Processing completed in {:?}",
            start_time.elapsed()
        );
    }

    if args.count_tokens {
        print_token_analysis(&file_info, args.ai_model.into())?;
    }

    let final_file_info = if args.fit_context {
        let counter = TokenCounter::new(args.ai_model.into());
        let (truncated_content, was_truncated) =
            counter.truncate_to_fit(&file_info.lines.join("\n"), args.prompt_tokens);
        if was_truncated {
            println!("📐 Context Fitting Applied");
            file_info.with_lines(truncated_content.lines().map(String::from).collect())
        } else {
            file_info
        }
    } else {
        file_info
    };

    if output_mode == OutputMode::Summary && final_file_info.summary_line_count() == 0 {
        println!("// No summary-worthy code structures found");
        return Ok(());
    }

    let formatted_output =
        batless::format_output(&final_file_info, file_path, config, output_mode)?;

    if args.validate_json && output_mode == OutputMode::Json {
        validate_json_output(&formatted_output)?;
    }

    println!("{formatted_output}");

    if output_mode != OutputMode::Json {
        if final_file_info.truncated_by_lines {
            let max_lines = config.max_lines; // local to allow inline capture
            println!("// Output truncated after {max_lines} lines");
        }
        if final_file_info.truncated_by_bytes {
            if let Some(max_bytes) = config.max_bytes {
                println!("// Output truncated after {max_bytes} bytes");
            }
        }
    }

    Ok(())
}

fn print_token_analysis(file_info: &batless::FileInfo, model: AiModel) -> BatlessResult<()> {
    let content = file_info.lines.join("\n");
    let counter = TokenCounter::new(model);
    let token_count = counter.count_tokens(&content);

    println!("Token Count Analysis:");
    let model_str = token_count.model.as_str();
    println!("  Model: {model_str}");
    let tokens = token_count.tokens;
    println!("  Tokens: {tokens}");
    let context_window = token_count.model.context_window();
    println!("  Context window: {context_window}");
    let fits = if token_count.fits_in_context {
        "✓"
    } else {
        "✗"
    };
    println!("  Fits in context: {fits}");
    println!();
    Ok(())
}

fn validate_json_output(json_output: &str) -> BatlessResult<()> {
    let validator = JsonSchemaValidator::new();
    let json_value: serde_json::Value = serde_json::from_str(json_output)?;
    if let Err(e) = validator.validate("json_output", &json_value) {
        eprintln!("⚠️  JSON validation warning: {e}. Output may not be fully AI-compatible.");
    }
    Ok(())
}

// Helpful error messages for unsupported features

fn print_pattern_not_supported() {
    let mut stderr = StandardStream::stderr(ColorChoice::Auto);
    let _ = stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true));
    let _ = writeln!(
        &mut stderr,
        "Error: batless doesn't support pattern searching"
    );
    let _ = stderr.reset();
    let _ = writeln!(&mut stderr);
    let _ = stderr.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
    let _ = writeln!(&mut stderr, "💡 Tip: Use dedicated search tools:");
    let _ = stderr.reset();
    let _ = writeln!(&mut stderr, "     grep -rn \"pattern\" src/");
    let _ = writeln!(
        &mut stderr,
        "     rg \"pattern\" src/          # even faster!"
    );
    let _ = writeln!(&mut stderr);
    let _ = writeln!(&mut stderr, "   Then view results with batless:");
    let _ = writeln!(&mut stderr, "     batless $(grep -l \"pattern\" src/*)");
    let _ = writeln!(&mut stderr);
    let _ = writeln!(
        &mut stderr,
        "Why? batless focuses on viewing files. grep/rg are"
    );
    let _ = writeln!(
        &mut stderr,
        "optimized for searching. Use the best tool for each job!"
    );
}

fn print_list_not_supported() {
    let mut stderr = StandardStream::stderr(ColorChoice::Auto);
    let _ = stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true));
    let _ = writeln!(&mut stderr, "Error: batless doesn't list files");
    let _ = stderr.reset();
    let _ = writeln!(&mut stderr);
    let _ = stderr.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
    let _ = writeln!(&mut stderr, "💡 Tip: Use file listing tools:");
    let _ = stderr.reset();
    let _ = writeln!(&mut stderr, "     ls -la src/");
    let _ = writeln!(&mut stderr, "     find . -name \"*.py\"");
    let _ = writeln!(
        &mut stderr,
        "     fd -e rs                  # modern alternative"
    );
    let _ = writeln!(&mut stderr, "     tree src/                 # tree view");
    let _ = writeln!(&mut stderr);
    let _ = writeln!(&mut stderr, "   Then view files with batless:");
    let _ = writeln!(&mut stderr, "     fd -e py | xargs batless");
    let _ = writeln!(&mut stderr);
    let _ = writeln!(
        &mut stderr,
        "Why? batless views individual files. Use ls/find/fd/tree"
    );
    let _ = writeln!(
        &mut stderr,
        "for file discovery, then pipe to batless for viewing."
    );
}

fn print_range_not_supported() {
    let mut stderr = StandardStream::stderr(ColorChoice::Auto);
    let _ = stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true));
    let _ = writeln!(&mut stderr, "Error: batless doesn't support line ranges");
    let _ = stderr.reset();
    let _ = writeln!(&mut stderr);
    let _ = stderr.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
    let _ = writeln!(&mut stderr, "💡 Tip: Use these alternatives:");
    let _ = stderr.reset();
    let _ = writeln!(
        &mut stderr,
        "     sed -n '10,50p' file.py | batless --language=python"
    );
    let _ = writeln!(&mut stderr, "     head -50 file.py | tail -41 | batless");
    let _ = writeln!(&mut stderr);
    let _ = writeln!(&mut stderr, "   Or use batless with limiting:");
    let _ = writeln!(&mut stderr, "     batless --max-lines=100 file.py");
    let _ = writeln!(&mut stderr);
    let _ = writeln!(
        &mut stderr,
        "Note: Line range support may be added in a future version."
    );
    let _ = writeln!(
        &mut stderr,
        "See: https://github.com/docdyhr/batless/issues/57"
    );
}
