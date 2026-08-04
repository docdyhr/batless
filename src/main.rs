use batless::{config_manager::ConfigManager, BatlessError, BatlessResult, OutputMode};
use clap::CommandFactory;
use clap_complete::generate;
use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

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
    let output_mode = config_manager.output_mode();

    // Handle commands that don't require file processing
    if handle_special_commands(args)? {
        return Ok(());
    }

    let file_path = config_manager.file_path()?;

    // Directory input with index mode: walk and emit NDJSON
    if output_mode == OutputMode::Index && std::path::Path::new(&file_path).is_dir() {
        return handle_directory_index(&file_path, &config_manager);
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

    if args.list_languages {
        for language in batless::LanguageDetector::list_languages() {
            println!("{language}");
        }
        return Ok(true);
    }

    Ok(false)
}

/// Iteratively walks `root` collecting every regular file, using an explicit
/// heap-allocated stack rather than recursion so directory depth can never
/// overflow the call stack. Output is sorted at the end for a deterministic
/// order (the walk itself no longer proceeds directory-by-directory).
fn collect_files_recursive(root: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    let mut pending = vec![root.to_path_buf()];

    while let Some(dir) = pending.pop() {
        let entries = match std::fs::read_dir(&dir) {
            Ok(e) => e,
            Err(e) => {
                eprintln!("batless: cannot read directory {}: {}", dir.display(), e);
                continue;
            }
        };
        for entry in entries.flatten() {
            let path = entry.path();
            // Use symlink_metadata so symlinks are never followed — prevents cycles.
            let meta = match std::fs::symlink_metadata(&path) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("batless: cannot stat {}: {}", path.display(), e);
                    continue;
                }
            };
            if meta.is_symlink() {
                // Skip symlinks entirely to avoid directory cycles.
                continue;
            }
            if meta.is_dir() {
                // Skip hidden directories
                if path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.starts_with('.'))
                {
                    continue;
                }
                pending.push(path);
            } else if meta.is_file() {
                out.push(path);
            }
        }
    }

    out.sort();
}

fn handle_directory_index(dir_path: &str, manager: &ConfigManager) -> BatlessResult<()> {
    let config = manager.config();
    let mut files = Vec::new();
    collect_files_recursive(std::path::Path::new(dir_path), &mut files);

    let stdout = io::stdout();
    let mut out = stdout.lock();

    for file in &files {
        let path_str = file.to_string_lossy();
        let compact = match batless::process_file(&path_str, config) {
            Ok(file_info) => {
                match batless::format_output(&file_info, &path_str, config, OutputMode::Index) {
                    Ok(pretty) => {
                        // Compact the pretty JSON to a single line for NDJSON
                        serde_json::from_str::<serde_json::Value>(&pretty)
                            .and_then(|v| serde_json::to_string(&v))
                            .unwrap_or(pretty)
                    }
                    Err(e) => {
                        let err_obj =
                            serde_json::json!({"file": path_str.as_ref(), "error": e.to_string()});
                        serde_json::to_string(&err_obj).unwrap_or_default()
                    }
                }
            }
            Err(e) => {
                let err_obj =
                    serde_json::json!({"file": path_str.as_ref(), "error": e.to_string()});
                serde_json::to_string(&err_obj).unwrap_or_default()
            }
        };
        writeln!(out, "{compact}")?;
    }
    Ok(())
}

fn handle_normal_processing(file_path: &str, manager: &ConfigManager) -> BatlessResult<()> {
    let config = manager.config();
    let output_mode = manager.output_mode();

    let start_time = std::time::Instant::now();
    if config.debug {
        eprintln!("🔍 DEBUG: Starting file processing for {file_path}");
    }

    let final_file_info = batless::process_file(file_path, config)?;

    if config.debug {
        eprintln!(
            "🔍 DEBUG: Processing completed in {:?}",
            start_time.elapsed()
        );
    }

    let formatted_output =
        batless::format_output(&final_file_info, file_path, config, output_mode)?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_collect_files_recursive_sorted_and_nested() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        fs::write(root.join("b.txt"), "b").unwrap();
        fs::write(root.join("a.txt"), "a").unwrap();
        fs::create_dir(root.join("sub")).unwrap();
        fs::write(root.join("sub").join("c.txt"), "c").unwrap();

        let mut files = Vec::new();
        collect_files_recursive(root, &mut files);

        assert_eq!(
            files,
            vec![
                root.join("a.txt"),
                root.join("b.txt"),
                root.join("sub").join("c.txt"),
            ]
        );
    }

    #[test]
    fn test_collect_files_recursive_skips_hidden_dirs() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        fs::create_dir(root.join(".hidden")).unwrap();
        fs::write(root.join(".hidden").join("secret.txt"), "x").unwrap();
        fs::write(root.join("visible.txt"), "x").unwrap();

        let mut files = Vec::new();
        collect_files_recursive(root, &mut files);

        assert_eq!(files, vec![root.join("visible.txt")]);
    }

    #[cfg(unix)]
    #[test]
    fn test_collect_files_recursive_skips_symlinks() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        fs::write(root.join("real.txt"), "x").unwrap();
        std::os::unix::fs::symlink(root.join("real.txt"), root.join("link.txt")).unwrap();
        std::os::unix::fs::symlink(root, root.join("self_loop")).unwrap();

        let mut files = Vec::new();
        collect_files_recursive(root, &mut files);

        // Only the real file is collected; the file symlink and the
        // directory symlink (which would otherwise cause infinite
        // recursion) are both skipped.
        assert_eq!(files, vec![root.join("real.txt")]);
    }
}
