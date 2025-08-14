All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

# Changelog

## [0.2.4] - UNRELEASED

### Planned

- Define explicit goals for 0.2.4 (performance polish, docs, minor UX?)

### Added (0.2.4)

- feat: Add --version-json flag for machine-readable version/build metadata (includes git hash & build timestamp)

### Notes

- Collecting early maintenance items; no scoped features committed yet.

### Maintenance

- deps: Bump clap from 4.5.42 to 4.5.43 and clap_complete from 4.5.55 to 4.5.56
- deps: Bump toml from 0.8.23 to 0.9.5 (major version update, fully backward compatible)
- chore: Update shell completions with latest CLI options
- fix: Resolve Docker and cross-platform validation issues
- fix: Add gcc to Dockerfile for musl cross-compilation

## [0.2.3] - 2025-08-07

### Added (0.2.3)

- Enhanced interactive configuration wizard with more comprehensive options
- Improved error handling & user experience (clearer messages, better suggestions)
- Centralized configuration logic & architectural refactor for maintainability
- Expanded integration & monitoring documentation (AI & Editor Integration Guide, release monitoring)
- Additional integration & property tests increasing overall coverage

### Changed (0.2.3)

- Major refactor: reduced duplication, cleaner separation of concerns
- Documentation & TODO cleanup after 0.2.2 release
- CI/CD stability improvements
- docs: Correct v0.2.2 status - features complete but not yet released (1df3e83)
- docs: Clean up TODO.md header formatting after v0.2.2 release (684d543)
- ci/docs: Fix pipeline issues & clippy warnings (22bca40)

### Fixed (0.2.3)

- Resolved clippy format string/interpolation warnings
- Addressed minor documentation inconsistencies

## [0.2.2] - 2025-08-03

### Added (0.2.2)

- Complete cat replacement functionality with -n/--number & -b/--number-nonblank flags
- Exact compatibility with system cat line numbering format (6-character right-aligned + tab)
- Perfect newline handling to match cat/less output exactly
- Enhanced PAGER / cat compatibility flags & behaviors (--plain, compatibility flags)
- Comprehensive AI & Editor Integration Guide (13+ CLI AI tools, 5+ web AI platforms)

### Changed (0.2.2)

- Standardized output using consistent println formatting
- Modernized format strings to resolve uninlined-format-args warnings
- Simplified conditional logic in main.rs (removed if-same-then-else patterns)
- Improved CI/CD pipeline reliability and stability

### Fixed (0.2.2)

- Critical newline bug causing shell "%" indicators
- Test failures in CI (e.g. test_max_lines_limit)
- Clippy warnings (format strings, conditional logic)
- Truncation message formatting for better test compatibility

## [0.2.1] - 2025-08-02

### Summary

Enhanced Features & milestone completion: PAGER compatibility and comprehensive streaming improvements.

This release completes the v0.2.1 milestone with PAGER compatibility and comprehensive streaming features for handling large files efficiently.

#### 🆕 New Features

- **🔧 PAGER Compatibility**: Use batless as a drop-in replacement for cat/less
  - `--plain`: Plain text output mode equivalent to `--mode plain`
  - `--unbuffered`, `--number`: Compatibility flags (ignored for compatibility)
  - Stdin support: Read from pipes and redirects (`echo "content" | batless --plain`)
  - GitHub CLI integration: `PAGER="batless --plain" gh pr view 46`
  - Automatic color disabling in plain mode for better PAGER compatibility

- **🌊 Enhanced JSON Output**: Complete streaming and resumable processing
  - `--streaming-json`: Enable streaming mode for large file processing
  - `--streaming-chunk-size <SIZE>`: Configure chunk size (default: 1000 lines)
  - `--enable-resume --checkpoint <FILE>`: Resume processing from saved checkpoint
  - Stdin support for streaming operations (pipelines and PAGER usage)
  - JSON schema versioning with backwards compatibility
  - Comprehensive streaming validation and error handling

- **🌊 Streaming JSON Output**: Process very large files efficiently with streaming
  - `--streaming-json`: Enable streaming mode for large file processing
  - `--streaming-chunk-size <SIZE>`: Configure chunk size (default: 1000 lines)
  - Schema versioning with JSON output versioned as "2.1"
  - Chunk-based processing with metadata and checksums
- **⏯️ Resume Capability**: Checkpoint and resume interrupted processing
  - `--enable-resume`: Enable checkpoint-based resuming
  - `--checkpoint <FILE>`: Specify checkpoint file location
  - Configuration compatibility validation for safe resuming
- **🧙 Interactive Configuration Wizard**: User-friendly profile setup
  - `--configure`: Launch interactive configuration wizard
  - `--list-profiles`: Display all available custom profiles
  - `--edit-profile <PATH>`: Edit existing profiles interactively
  - Comprehensive profile management with timestamps and validation
- **🔍 Debug Mode**: Detailed processing information for troubleshooting
  - `--debug`: Enable debug output with timing and processing details
  - File statistics, processing time, and configuration details
  - Helpful for performance tuning and issue diagnosis

#### Enhanced

- **📊 JSON Schema System**: Added streaming chunk schema support
  - New `streaming_chunk` schema for validating streaming output
  - Enhanced error messages with field-level validation
  - Schema version tracking for compatibility
- **⚙️ Configuration System**: Extended with new streaming and debug options
  - Support for streaming configuration in custom profiles
  - Debug mode configuration persistence
  - Enhanced validation for streaming parameters

#### Developer Experience

- **🎯 Comprehensive CLI**: All new features fully integrated into help system
- **📝 Enhanced Documentation**: Complete CLI help for all new options
- **🧪 Test Coverage**: 162 tests covering all new functionality
- **🔒 Security**: Full security audit passed with no vulnerabilities
- **⚡ Performance**: Optimized for large file processing with streaming

#### Technical Details

- Added `streaming.rs` module for streaming JSON functionality
- Added `wizard.rs` module for interactive configuration management
- Extended configuration system with streaming and debug support
- Enhanced JSON schema validation with streaming support
- Added chrono dependency for timestamp management
- Maintained full backward compatibility with existing features

<!-- (Removed duplicate placeholder subsections for 0.2.0) -->

<!-- (Trimmed duplicated historical sections below; canonical history retained above) -->

<!-- (Removed secondary duplicate placeholder group) -->

## [0.1.0] - 2025-07-22

### Added

- 🎯 **Core batless functionality** - non-blocking code viewer for AI and automation
- 🎨 **Syntax highlighting** for 100+ programming languages via syntect
- 📊 **Multiple output modes**: plain, highlight, JSON, and summary
- 🤖 **AI-optimized features**:
  - Summary mode extracts functions, classes, imports
  - Token extraction for LLM context processing
  - JSON output with metadata and structure
- ⚡ **Performance features**:
  - Smart file limiting by lines and bytes
  - Memory-efficient streaming architecture
  - Cached syntax definitions for fast startup
- 🔒 **Enterprise-grade security**:
  - Comprehensive test coverage with property-based testing
  - Daily vulnerability scanning and dependency auditing
  - Supply chain security with SBOM generation
  - Memory safety validation and fuzz testing
- 📦 **Professional distribution**:
  - Cross-platform binaries (Linux, macOS, Windows)
  - Homebrew tap integration
  - Docker containerization
  - GitHub Actions CI/CD pipeline

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

### Technical Details (0.1.0)

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

<!-- Link references intentionally removed since trimmed history above covers 0.1.0 only -->
