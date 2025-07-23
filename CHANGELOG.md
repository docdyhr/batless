All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

# Changelog



## [0.1.1] - 2025-07-23

### Added
- Comprehensive release infrastructure with GitHub Actions
- Cross-platform binary builds (Linux, macOS, Windows) 
- Automated Homebrew tap updates
- Enhanced CI/CD pipeline for releases

### Changed
- Replaced unmaintained `atty` dependency with `is-terminal` for better security
- Simplified release workflow for faster build times
- Improved release automation scripts

### Fixed
- Security vulnerability in dependency chain
- Release workflow compatibility issues

### Infrastructure
- Set up automated crates.io publishing
- Added cross-platform binary distribution
- Enhanced security scanning and dependency management

### Fixed
<!-- Bug fixes -->

### Removed
<!-- Features removed in this version -->

## [Unreleased]
### Security
- Replaced unmaintained `atty` dependency with `is-terminal` to fix RUSTSEC-2024-0375 and RUSTSEC-2021-0145
- Updated author information in Cargo.toml

### Added
- Performance baseline documentation in `docs/PERFORMANCE_BASELINE.md`
- Established benchmark metrics for tracking performance regressions

### Fixed
- Resolved security vulnerabilities in dependencies
- Updated author email from placeholder to actual email



## [0.1.1] - 2025-07-23

### Added
<!-- New features -->

### Changed
<!-- Changes in existing functionality -->
- 70cc589 chore: remove backup file
- bcab2e1 fix: replace unmaintained atty with is-terminal for security
- 1baba90 Simplify release workflow for faster builds
- 0cd0ea2 Fix release workflow - remove cargo-dist dependency

### Fixed
<!-- Bug fixes -->

### Removed
<!-- Features removed in this version -->

## [0.1.0] - 2025-01-23
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

### Changed
- Established security-first development practices
- Implemented comprehensive testing infrastructure
- Added strategic roadmap and development planning

### Fixed
- Resolved all clippy warnings and security audit issues  
- Proper error handling throughout codebase
- Memory safety improvements in all critical paths



## [0.1.1] - 2025-07-23

### Added
<!-- New features -->

### Changed
<!-- Changes in existing functionality -->
- 70cc589 chore: remove backup file
- bcab2e1 fix: replace unmaintained atty with is-terminal for security
- 1baba90 Simplify release workflow for faster builds
- 0cd0ea2 Fix release workflow - remove cargo-dist dependency

### Fixed
<!-- Bug fixes -->

### Removed
<!-- Features removed in this version -->

## [0.1.0] - 2024-12-28
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
