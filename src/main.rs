use batless::{
    config_manager::ConfigManager,
    AiModel,
    BatlessError,
    BatlessResult,
    JsonSchemaValidator,
    OutputMode,
    TokenCounter,
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
        let _ = write!(&mut stderr, " {}", code_part);
        let _ = stderr.reset();
        let _ = writeln!(&mut stderr, "{}", message_part);
    } else {
        let _ = stderr.reset();
        let _ = writeln!(&mut stderr, ": {}", first_line);
    }

    if !rest.is_empty() {
        let _ = stderr.reset();
        let _ = writeln!(&mut stderr, "\n{}", rest);
    }
}

fn main() {
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
    if let Some(shell) = args.generate_completions {
        let mut cmd = Args::command();
        let name = cmd.get_name().to_string();
        let mut buffer: Vec<u8> = Vec::new();
        match shell {
            Shell::Bash => generate(clap_complete::shells::Bash, &mut cmd, name, &mut buffer),
            Shell::Zsh => generate(clap_complete::shells::Zsh, &mut cmd, name, &mut buffer),
            Shell::Fish => generate(clap_complete::shells::Fish, &mut cmd, name, &mut buffer),
            Shell::Power => generate(clap_complete::shells::PowerShell, &mut cmd, name, &mut buffer),
        }
        io::stdout().write_all(&buffer)?;
        return Ok(true);
    }

    if let Some(format) = &args.get_schema {
        let validator = JsonSchemaValidator::new();
        let schema = validator.get_schema(format).ok_or_else(|| {
            BatlessError::config_error_with_help(
                format!("Unknown schema format '{}'", format),
                Some("Available schemas: file_info, json_output, token_count, processing_stats".to_string()),
            )
        })?;
        println!("{}", serde_json::to_string_pretty(schema)?);
        return Ok(true);
    }

    if args.list_languages {
        for language in batless::LanguageDetector::list_languages() {
            println!("{}", language);
        }
        return Ok(true);
    }

    if args.list_themes {
        for theme in batless::ThemeManager::list_themes() {
            println!("{}", theme);
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
        batless::ConfigurationWizard::edit_profile(profile_path)?;
        return Ok(true);
    }

    Ok(false)
}

fn handle_streaming_json(file_path: &str, manager: &ConfigManager) -> BatlessResult<()> {
    use batless::StreamingProcessor;

    let config = manager.config();
    let args = manager.args();

    let checkpoint = if config.enable_resume {
        args.checkpoint.as_ref().and_then(|path| {
            if std::path::Path::new(path).exists() {
                Some(StreamingProcessor::load_checkpoint(std::path::Path::new(path)))
            } else {
                None
            }
        }).transpose()? 
    } else {
        None
    };

    let chunks = StreamingProcessor::process_streaming(file_path, config, checkpoint)?;

    for chunk_result in chunks {
        let chunk = chunk_result?;
        let json_output = serde_json::to_string_pretty(&chunk)?;
        println!("{}", json_output);

        if config.enable_resume && !chunk.is_final {
            if let Some(checkpoint_path) = &args.checkpoint {
                StreamingProcessor::save_checkpoint(&chunk.checkpoint, std::path::Path::new(checkpoint_path))?;
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
        eprintln!("🔍 DEBUG: Starting file processing for {}", file_path);
    }

    let file_info = batless::process_file(file_path, config)?;

    if config.debug {
        eprintln!("🔍 DEBUG: Processing completed in {:?}", start_time.elapsed());
    }

    if args.count_tokens {
        print_token_analysis(&file_info, args.ai_model.into())?;
    }

    let final_file_info = if args.fit_context {
        let counter = TokenCounter::new(args.ai_model.into());
        let (truncated_content, was_truncated) = counter.truncate_to_fit(&file_info.lines.join("\n"), args.prompt_tokens);
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

    let formatted_output = batless::format_output(&final_file_info, file_path, config, output_mode)?;

    if args.validate_json && output_mode == OutputMode::Json {
        validate_json_output(&formatted_output)?;
    }

    println!("{}", formatted_output);

    if output_mode != OutputMode::Json {
        if final_file_info.truncated_by_lines {
            println!("// Output truncated after {} lines", config.max_lines);
        }
        if final_file_info.truncated_by_bytes {
            if let Some(max_bytes) = config.max_bytes {
                println!("// Output truncated after {} bytes", max_bytes);
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
    println!("  Model: {}", token_count.model.as_str());
    println!("  Tokens: {}", token_count.tokens);
    println!("  Context window: {}", token_count.model.context_window());
    println!("  Fits in context: {}", if token_count.fits_in_context { "✓" } else { "✗" });
    println!();
    Ok(())
}

fn validate_json_output(json_output: &str) -> BatlessResult<()> {
    let validator = JsonSchemaValidator::new();
    let json_value: serde_json::Value = serde_json::from_str(json_output)?;
    if let Err(e) = validator.validate("json_output", &json_value) {
        eprintln!("⚠️  JSON validation warning: {}. Output may not be fully AI-compatible.", e);
    }
    Ok(())
}

