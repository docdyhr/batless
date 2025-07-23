# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-12-28

### Added
- Initial release of batless - a minimal, AI-friendly code viewer
- Core syntax highlighting using syntect with 100+ language support
- Multiple output modes:
  - `plain` - raw text output with optional ANSI stripping
  - `highlight` - syntax highlighted output with color control
  - `json` - structured JSON output for programmatic consumption
- Smart file limiting:
  - `--max-lines` - limit output by number of lines
  - `--max-bytes` - limit output by total bytes
  - Clear truncation messages indicating why output was limited
- Language detection and specification:
  - Automatic language detection based on file extension
  - Explicit language override with `--language` option
  - Support for 100+ programming languages via Tree-sitter
- Color and theme control:
  - `--color` option with auto/always/never modes
  - `--theme` option for syntax highlighting themes
  - `--strip-ansi` flag to remove ANSI escape codes
- Memory efficient streaming:
  - Never loads entire files into memory
  - Processes files line by line for large file support
  - Non-blocking output guaranteed for AI assistants and CI/CD
- Comprehensive CLI interface:
  - Help and version commands
  - Clear error messages for missing files
  - Intuitive command-line argument structure
- Library support:
  - Core functionality available as a Rust library
  - Clean API for integration into other tools
  - Configurable processing with `BatlessConfig`
- Extensive testing:
  - 11 unit tests covering core functionality
  - 15 integration tests validating CLI behavior
  - Edge case handling for empty files, truncation, and errors
- Documentation:
  - Comprehensive README with usage examples
  - Demo Python script showcasing syntax highlighting
  - Shell demo script demonstrating all features
  - Product requirements document (PRD)

### Features for AI Assistants
- Always non-blocking output (never hangs or waits for input)
- Clean, predictable output format
- JSON mode for structured data extraction
- Safe defaults for automated workflows
- No decorations or interactive elements

### Features for CI/CD
- Guaranteed streaming output
- Configurable color control for different environments
- Error handling that doesn't break pipelines
- Memory efficient for large codebases
- Single binary with no external dependencies

### Technical Details
- Built with Rust for performance and safety
- Uses syntect for syntax highlighting
- Clap for CLI argument parsing
- Supports all major operating systems
- Release binary size: ~2MB
- Fast startup time with lazy loading

### Comparison with `bat`
- ✅ Never blocks (unlike `bat` which can use `less`)
- ✅ JSON output mode (not available in `bat`)
- ✅ Byte limiting support (not in `bat`)
- ✅ AI/automation focused design
- ❌ No Git integration (by design)
- ❌ No line numbers (by design for clean output)
- ❌ No file headers (by design for minimal output)

[0.1.0]: https://github.com/your-username/batless/releases/tag/v0.1.0