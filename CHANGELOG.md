# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Custom error types for better error handling and user experience
- Modular architecture with separate modules for each concern:
  - `config` - Configuration management with validation
  - `error` - Custom error types and result handling
  - `file_info` - File metadata and processing results
  - `formatter` - Output formatting for different modes
  - `highlighter` - Syntax highlighting functionality
  - `language` - Language detection and theme management
  - `processor` - Core file processing logic
  - `summarizer` - Code summary extraction
  - `tokenizer` - Token extraction for AI processing
- Performance benchmark suite using Criterion
- Comprehensive unit tests for all modules (107 tests total)
- Enhanced language detection with fallback mechanisms
- Advanced tokenization strategies for different file types
- Improved summary extraction supporting 15+ programming languages

### Changed
- Refactored monolithic `lib.rs` (595 lines) into focused modules
- Updated dependency configuration to eliminate security vulnerabilities
- Improved error messages with specific error types and context
- Enhanced JSON output format for better backward compatibility
- Upgraded syntax highlighting to use minimal syntect features
- Streamlined CLI error handling with proper stderr output

### Fixed
- Security vulnerability: Eliminated unmaintained `yaml-rust` dependency (RUSTSEC-2024-0320)
- Memory efficiency improvements in large file processing
- Enhanced encoding detection for non-UTF-8 files
- Improved comment filtering in summary mode for more accurate results
- Fixed truncation handling to properly report line vs byte limits

### Security
- Removed indirect dependency on unmaintained `yaml-rust` crate
- Updated syntect dependency to use minimal feature set without YAML loading
- Enhanced input validation and error handling throughout the codebase
- Added comprehensive test coverage including property-based tests

### Performance
- Maintained streaming architecture while improving modularity
- Optimized syntax highlighting with selective feature loading
- Enhanced caching strategies for repeated file processing
- Reduced memory footprint through better resource management

### Technical Debt Resolution
- Code complexity reduced from single 595-line file to 9 focused modules
- Test coverage expanded from basic integration tests to 107 unit tests
- Error handling improved from generic errors to 11 specific error types
- Documentation enhanced with comprehensive inline comments
- Build warnings eliminated and code quality improved

## [0.1.4] - 2025-07-26

### Added
- PR-based release workflow that respects branch protection rules
- Comprehensive development guide with release process documentation
- Benchmark files and performance test results

### Changed
- Updated release process to use PRs instead of direct main branch pushes
- Enhanced CLAUDE.md with development workflow guidelines
- Improved CI/CD automation with branch protection compliance

### Fixed
- Release workflow failures due to branch protection rules
- Cargo.lock tracking for binary crate reproducible builds
- Missing GH_TOKEN in GitHub Actions workflows

## [0.1.3] - 2025-07-25

### Added
- Manual workflow dispatch system for granular CI/CD control
- Repository health check automation with issue creation
- Comprehensive performance benchmarking with regression detection
- Enhanced dependency management with grouped Dependabot updates

### Changed
- Optimized CI/CD workflows with consolidated caching strategy
- Enhanced error handling with graceful failures and proper timeouts
- Updated security tools with locked dependencies and latest versions
- Improved release process with retry logic and better artifact management

### Fixed
- Replaced deprecated `actions/upload-release-asset@v1` with modern alternatives
- Fixed caching inefficiencies with restore keys and matrix strategy support
- Enhanced workflow stability with better external dependency handling

### Security
- Updated OSSF Scorecard to latest version with proper token handling
- Enhanced security audit workflows with better reporting
- Replaced unmaintained `atty` dependency with `is-terminal` to fix RUSTSEC-2024-0375 and RUSTSEC-2021-0145
- Updated author information in Cargo.toml

### Performance
- Implemented comprehensive benchmarking suite with criterion
- Added performance regression detection in CI/CD
- Performance baseline documentation in `docs/PERFORMANCE_BASELINE.md`

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

[0.1.1]: https://github.com/docdyhr/batless/releases/tag/v0.1.1
[0.1.0]: https://github.com/docdyhr/batless/releases/tag/v0.1.0