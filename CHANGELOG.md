# Changelog

All notable changes to this project will be documented in this file.

## [0.4.1] - 2026-02-27

### Bug Fixes

- **Panic elimination**: Replaced 4 `unwrap()` calls in `ast_summarizer.rs` with proper error handling, preventing panics on malformed AST queries
- **Panic elimination**: Replaced 4 `unwrap()` calls in `wizard.rs` with safe alternatives
- **Cast safety**: Fixed `u64` to `usize` cast in `processor.rs` with saturating conversion to prevent truncation on 32-bit platforms
- **Test reliability**: Changed integration tests to use `env!("CARGO_BIN_EXE_batless")` instead of `cargo run`, fixing intermittent macOS CI failures

### Improvements

- **AST summarizer wired into pipeline**: The tree-sitter `AstSummarizer` is now fully integrated into the summary mode processing pipeline, replacing the regex-based fallback when tree-sitter grammars are available
- **Error handling**: Improved error types with more specific variants and contextual help messages
- **Levenshtein optimization**: Switched from full-matrix to two-row algorithm for fuzzy matching in error suggestions, reducing memory from O(n*m) to O(m)
- **Config refactor**: Extracted validation logic from `config.rs` (1,120 lines) into dedicated `config_validation.rs` (376 lines) for better separation of concerns

### CI/CD

- Removed 8 deprecated workflows, consolidating from 19 to 11 workflows
- Reduced scheduled workflow frequency (security: daily to weekly, health-check/fuzz: daily to weekly)
- Added concurrency controls to prevent redundant parallel runs
- Fixed cross-platform-validation workflow parse errors caused by `matrix.shell` in non-triggered workflow validation

### Dependencies

- Updated 66+ compatible dependencies including:
  - clap 4.5.58 → 4.5.60, tree-sitter 0.26.5 → 0.26.6
  - regex 1.12.2 → 1.12.3, libc 0.2.177 → 0.2.182
  - proptest 1.9.0 → 1.10.0, tempfile 3.24.0 → 3.26.0

### Testing

- **386 total tests** (up from 325 in v0.4.0, +61 tests)
  - 35 new tests for `config_manager.rs`
  - 16 new tests for `ast_summarizer.rs`
  - 18 validation tests in new `config_validation.rs`
- Zero clippy warnings, zero test failures

---

## [0.4.0] - 2026-01-08

### 🎯 Major Features

- **🌲 Multi-Language AST Support**: Added Abstract Syntax Tree summarization for 4 programming languages
  - **Rust**: Functions, structs, enums, traits, impls, constants (already supported, now comprehensive)
  - **Python**: Functions, classes, decorators, async/await, imports, type annotations
  - **JavaScript**: Functions, classes, arrow functions, exports, imports, async/await
  - **TypeScript**: Interfaces, type aliases, enums, generics, decorators (all JS features + TS-specific)
  - Three-level summary depth: Minimal, Standard (default), Detailed
  - Smart extraction preserves code structure and context
  - Zero performance overhead: 5ms execution time maintained across all modes

### 🚀 Performance & Quality

- **⚡ Performance Excellence**: Comprehensive benchmarking validates design goals
  - Consistent 5ms execution time across all modes (plain, highlight, JSON, summary)
  - AST parsing adds zero measurable overhead
  - Performance maintained on files from 200 bytes to 1KB+
  - 8-10x better than original <50ms target
- **✅ Test Coverage**: Expanded test suite ensures reliability
  - 325 total tests (98 new tests added in v0.4.0)
  - 28 comprehensive AST tests covering all 4 languages
  - 23 CLI documentation tests prevent documentation drift
  - 15 JavaScript/TypeScript tests
  - 12 Python tests
  - Zero test failures, 100% pass rate

### 🔧 Bug Fixes & Improvements

- **📚 Documentation Accuracy**: Fixed critical CLAUDE.md documentation mismatches
  - Line numbers now correctly documented as requiring --plain mode
  - Removed references to non-existent --pattern flag (exits with error)
  - Removed references to non-existent --list flag (exits with error)
  - Removed references to non-existent -r/--range flag (never existed)
  - Added 23 automated tests to prevent future documentation drift
- **🎯 Context Truncation Tracking**: Enhanced JSON output for AI workflows
  - Added `truncated_by_context` boolean field to JSON output
  - Distinguishes context-based truncation from line/byte limits
  - Improves AI assistant awareness of truncated content

### 📦 Dependencies

- **tree-sitter** (0.26.3) - AST parsing framework
- **tree-sitter-rust** (0.24.0) - Rust grammar support
- **tree-sitter-python** (0.23.6) - Python grammar support
- **tree-sitter-javascript** (0.25.0) - JavaScript grammar support
- **tree-sitter-typescript** (0.23.2) - TypeScript grammar support

### 🏗️ Technical Architecture

- **AST Module**: New `src/ast_summarizer.rs` with language-specific extractors
- **Consistent Pattern**: Parser → Query → Cursor → BTreeSet → sorted output
- **Memory Efficient**: Streaming architecture maintained with AST parsing
- **Language Detection**: Automatic language detection via syntect integration
- **Query-Based Extraction**: Tree-sitter queries for precise code structure matching

### 📖 Documentation

- **CLAUDE.md**: Fixed all documentation/CLI mismatches, comprehensive examples
- **RELEASE_CHECKLIST_v0.4.0.md**: Complete release preparation guide
- **Performance Data**: Documented benchmarking methodology and results
- **AST Usage**: Examples for all supported languages and summary levels

### 🔐 Security & Stability

- All 325 tests passing with zero failures
- No new security vulnerabilities introduced
- Pre-commit hooks enforce code quality (formatting, clippy, security)
- Comprehensive integration testing across all AST features

### 🎨 Developer Experience

- Language-specific AST extraction available via library API
- Three summary levels configurable: Minimal, Standard, Detailed
- Works seamlessly with existing --mode=summary and --mode=json flags
- JSON output includes `summary_lines` field when --summary flag used

### 📊 Metrics

- **Test Count**: 227 → 325 tests (+98 tests, +43%)
- **Language Support**: 1 → 4 AST languages (+300%)
- **Performance**: 5ms maintained (no regression)
- **Code Quality**: Zero clippy warnings, zero security issues

### 🙏 Contributors

Special thanks to the tree-sitter team for their excellent parsing libraries that made multi-language AST support possible.

---

## [0.3.2] - 2025-10-29

### Bug Fixes

- **🔧 CI/CD Pipeline Fixes**: Resolved critical workflow issues blocking automated builds
  - Fixed YAML syntax errors in quality.yml (heredoc delimiter issues)
  - Fixed performance benchmark workflow hyperfine installation
  - Resolved heredoc delimiter compatibility between bash and YAML
- **🛠️ Workflow Stability**: All CI/CD pipelines now passing consistently
  - Unified CI/CD Pipeline operational
  - Code Quality & Security checks passing
  - Performance benchmarks restored

### Maintenance

- **📦 Dependency Updates**: Updated to latest stable versions
  - clap: 4.5.49 → 4.5.50
  - is-terminal: 0.4.16 → 0.4.17 (cleaner dependency tree)
  - proptest: 1.8.0 → 1.9.0 (removed lazy_static dependency)
- **🔄 GitHub Actions Updates**: Updated across 12 workflow files
  - actions/upload-artifact: v4 → v5 (Node v24 support)
  - actions/download-artifact: v5 → v6
  - actions/setup-node: v4 → v6
  - codecov/codecov-action: v4 → v5 (standardized)
  - github/codeql-action: v3 → v4
  - EmbarkStudios/cargo-deny-action: v1 → v2
  - dawidd6/action-homebrew-bump-formula: v3 → v5
- **🧹 Repository Cleanup**: Pruned stale branches and references
  - Removed 4 obsolete Dependabot branches
  - Cleaned up local and remote branch references

### Documentation

- **📚 Roadmap Update**: Updated [ROADMAP.md](ROADMAP.md) with 2025-2026 timeline
  - Marked v0.3.1 achievements
  - Set realistic targets for v0.4.0 through v1.0.0
  - Updated success metrics to reflect current progress
- **🤝 Community Infrastructure**: Established contributor ecosystem
  - Enabled GitHub Discussions for community Q&A
  - Created 3 "good first issue" items for new contributors
  - Added labels: `good first issue`, `help wanted`, `documentation`

### Infrastructure

- **✅ Quality Verification**: Comprehensive testing and validation
  - Verified Rust beta channel compatibility (1.92.0-beta.1)
  - All 188 unit tests passing
  - Clippy clean on both stable and beta channels
  - Code formatting verified
- **🔐 Security**: All security audits passing
  - Dependabot security updates applied
  - CodeQL analysis passing
  - No known vulnerabilities

### Contributors

Special thanks to Dependabot for automated dependency updates and to the community for issue reports.

---

## [0.3.1] - 2025-10-17

### User Experience Improvements

- **✨ Helpful Error Messages**: Added friendly, actionable error messages when users try unsupported features
  - `--pattern` flag now suggests using `grep` or `rg` instead
  - `--list` flag now suggests using `ls`, `find`, or `fd` instead
  - `--range` flag now suggests using `sed` or `--lines` instead
  - Each error includes working examples and explains the design philosophy
  - Dramatically reduces user confusion and learning curve

### Maintenance

- **📦 Dependency Updates**: Updated 50+ dependencies to latest versions
  - clap 4.5.47 → 4.5.49
  - syntect 5.2.0 → 5.3.0
  - serde 1.0.225 → 1.0.228
  - proptest 1.7.0 → 1.8.0
  - Plus many transitive dependency updates with security patches

### Documentation

- **📚 Enhanced Documentation**: Major README cleanup and improvements
  - Reduced README from 2,949 to 439 lines (85% reduction)
  - Eliminated massive duplication (same sections repeated 21 times)
  - Added comprehensive CLI options reference
  - Added detailed usage examples for all use cases
  - Added AI assistant integration guides
  - Better structured with logical flow
- **📖 Philosophy Guide**: Added [docs/PHILOSOPHY_AND_SCOPE.md](docs/PHILOSOPHY_AND_SCOPE.md)
- **🔧 CI/CD Strategy**: Added [docs/GITHUB_ACTIONS_UPDATE_STRATEGY.md](docs/GITHUB_ACTIONS_UPDATE_STRATEGY.md)

### CI/CD Optimization

- **⚡ Pipeline Performance**: 40% speed improvement (26min → <15min average)
  - Enhanced concurrency controls with intelligent cancellation
  - Parallel test execution across 3-7 concurrent jobs
  - Advanced caching with Swatinem/rust-cache@v2
  - Build optimizations (CARGO_INCREMENTAL=0, target-cpu=native)
- **📊 Monitoring & Auto-Recovery**: Comprehensive health check system
  - `pipeline-health-check.sh` - automated health assessment
  - `ci-health-check.sh` - quick status checker
  - `workflow-performance-monitor.sh` - performance analysis
  - Real-time metrics tracking and reporting
- **🔄 New Workflows**: Added optimized CI/CD workflows
  - `ci-optimized.yml` - ultra-fast validation
  - `performance-optimized.yml` - aggressive parallelization

### Quality

- **✅ Tests**: 188+ tests passing with zero failures
- **🔒 Security**: All security audits passing, zero vulnerabilities
- **🧹 Code Quality**: Zero clippy warnings, clean builds
- **📈 CI/CD Health**: All workflows passing consistently

## [0.3.0] - 2024-08-27

### Major Architecture Improvements

- **🏗️ Technical Debt Remediation**: Comprehensive refactoring addressing critical stability and maintainability issues
- **📦 Module Extraction**: Split large config.rs (1,366→1,054 lines) into focused modules:
  - `src/summary.rs` - SummaryLevel enum with Copy trait optimization
  - `src/profile.rs` - CustomProfile with optimized getter methods
  - `src/traits.rs` - Dependency inversion interfaces
  - `src/processor_builder.rs` - Configurable processing with dependency injection
  - `src/formatters/` - Modular output formatting architecture
  - `src/performance.rs` - Caching, metrics, and optimization utilities
  - `src/debt_prevention.rs` - Quality gates and automated debt prevention

### Code Quality & Performance

- **🔧 Error Handling**: Eliminated 7 unwrap() calls in production code with proper error propagation
- **⚡ Performance**: Reduced clone() operations from 54→49, added Copy traits where applicable
- **🧹 Code Quality**: Zero clippy warnings, comprehensive validation with quality gates
- **🔒 Security**: All cargo audit vulnerabilities addressed

### CI/CD Optimization

- **🚀 Workflow Consolidation**: Streamlined from 12→3 focused workflows (62% reduction in YAML)
  - `test-consolidated.yml` - Comprehensive testing across platforms
  - `quality-consolidated.yml` - Linting, security, and quality checks
  - `release-consolidated.yml` - Automated releases with proper artifacts
- **♻️ Reusable Actions**: Created modular GitHub Actions in `.github/actions/`
- **📊 Enhanced Testing**: 247+ tests with improved coverage and reliability

### Developer Experience

- **🎯 Dependency Inversion**: Trait-based architecture enabling better testability and modularity
- **🛠️ Builder Pattern**: Configurable processors with clean dependency injection
- **📋 Quality Gates**: Automated checks preventing future technical debt accumulation
- **📝 ADR Templates**: Architecture Decision Record system for documenting design choices

### Bug Fixes

- **✅ Test Reliability**: Fixed 7 failing CLI integration tests
- **🔧 Schema Validation**: Added proper "$schema" fields to all JSON outputs
- **🐚 Shell Completion**: Fixed PowerShell completion generation
- **⚠️ Validation**: Enhanced error messages with "validation" keywords for better UX

## [0.2.5] - 2024-12-19

### Added

- `--lines=START:END` flag for selecting specific line ranges
  - Supports formats: `10:50` (lines 10-50), `100:` (from line 100), `:50` (up to line 50), `42` (from line 42)
  - Works with all output modes (plain, json, summary)
  - Combines with `--max-lines` for additional control
  - Maintains streaming architecture for memory efficiency

### Fixed

- Performance baseline stabilization (3 consecutive green CI runs achieved)
- CLI documentation mismatch with CLAUDE.md resolved

### Documentation

- Added line range examples to README
- Updated CLI help text with range format examples
- Fixed CLAUDE.md to reflect actual CLI capabilities

## [0.2.4] - 2025-08-15

### Added (0.2.4)

- feat: Add --version-json flag for machine-readable version/build metadata (includes git hash & build timestamp)
- feat: Add --json-pretty flag to toggle pretty vs compact JSON output
- docs: Add Architecture Overview (docs/ARCHITECTURE.md)
- ci: Enhanced performance regression workflow with JSON summary & PR comments
- ux: Wizard profile listing now shows total count and latest update timestamp
- docs: README performance claim refined (<50ms → <5ms typical) with methodology note

### Notes

- Performance guard integrated; baseline stabilization ongoing (target: 3 consecutive green main runs post-release)

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
