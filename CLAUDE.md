# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

batless is a Rust-based CLI tool designed as a non-blocking, AI-friendly code viewer and cat replacement. It's inspired by `bat` but optimized for AI code assistants, CI/CD pipelines, and non-interactive workflows. The tool never hangs or blocks, making it ideal for automation.

**Latest Release**: v0.2.2 (August 3, 2025)
- ✅ Complete cat replacement functionality with `-n/--number` and `-b/--number-nonblank` flags
- ✅ Exact compatibility with system cat line numbering format (6-character right-aligned + tab)
- ✅ Perfect newline handling to match cat/less output exactly
- ✅ All 201 tests passing (162 unit + 33 integration + 6 property)

### Key Cat Replacement Features
- `batless file.txt -n`: Line numbering for all lines (exact `cat -n` compatibility)
- `batless file.txt -b`: Number only non-blank lines (exact `cat -b` compatibility)
- `batless file.txt --plain`: Plain text output for perfect cat replacement
- Perfect for AI workflows: `PAGER="batless --plain" gh pr view`

## Development Commands

### Build and Test

```bash
cargo build                          # Debug build
cargo build --release               # Release build
cargo test                          # Run all tests
cargo test --test integration_tests # Run integration tests only
cargo test test_name                # Run specific test
```

### Code Quality

```bash
cargo clippy -- -D warnings         # Linting with warnings as errors
cargo fmt --all -- --check          # Check formatting
cargo fmt                           # Format code
cargo audit                         # Security vulnerability audit
```

### Running the Tool

```bash
cargo run -- file.rs                # Run in debug mode
cargo run --release -- file.rs      # Run in release mode
./target/debug/batless file.rs      # Run debug binary directly
./demo.sh                           # Run demo script
```

## 🚀 Release Process

### Automated Release (Recommended)

```bash
# Install required tools
cargo install cargo-release

# Set crates.io token (one-time setup)
export CARGO_REGISTRY_TOKEN="your-token-here"

# Full automated release
./scripts/release.sh 0.2.2

# Or use cargo-release directly
cargo release 0.2.2 --execute
```

### Manual/Emergency Release

If the automated process fails:

```bash
# 1. Publish to crates.io
cargo release publish --execute

# 2. Create GitHub release
git tag v0.2.2 -m "Release message"
git push origin v0.2.2
gh release create v0.2.2 --title "Release Title" --notes "Release notes"

# 3. Verify publication
curl -s "https://crates.io/api/v1/crates/batless" | jq -r '.crate.max_version'
curl -s "https://api.github.com/repos/docdyhr/batless/releases/latest" | jq -r '.tag_name'
```

### Release Checklist

- [ ] All tests passing (201 tests: 162 unit + 33 integration + 6 property)
- [ ] Version updated in Cargo.toml
- [ ] CHANGELOG.md updated with release notes
- [ ] Run pre-release checks: `./scripts/pre-release-check.sh`
- [ ] Check version synchronization: `./scripts/version-status.sh`
- [ ] Published to crates.io
- [ ] GitHub release created with proper tag
- [ ] Homebrew formula updated (automatic via workflow)
- [ ] Monitor release workflow: `./scripts/release-monitor.sh <workflow_id>`

### Enhanced Release Monitoring (v0.2.2+)

```bash
# Pre-release quality gate
./scripts/pre-release-check.sh

# Version status tracking
./scripts/version-status.sh

# Monitor release workflow progress
./scripts/release-monitor.sh <workflow_run_id>

# Manual release fallback (if automated workflow fails)
cargo publish                                    # Publish to crates.io
gh release create v0.2.2 --title "v0.2.2" \    # Create GitHub release
  --notes "Release notes"
```

### CI/CD Workflows

```bash
# Manual workflow triggers available:
gh workflow run workflow-dispatch.yml -f workflow_type=full-test-suite
gh workflow run workflow-dispatch.yml -f workflow_type=security-audit
gh workflow run workflow-dispatch.yml -f workflow_type=performance-benchmark
gh workflow run workflow-dispatch.yml -f workflow_type=quality-check
gh workflow run workflow-dispatch.yml -f workflow_type=quick-validation

# Release workflow (triggered by tags):
gh workflow run release.yml --ref v0.2.2
```

## Architecture

### Core Components

- **main.rs**: CLI entry point using clap for argument parsing
- **lib.rs**: Core library with streaming file processing, syntax highlighting, and summary extraction
- **integration_tests.rs**: Comprehensive CLI behavior tests

### Key Design Principles

1. **Streaming Architecture**: Never loads entire files into memory - processes line by line
2. **Cached Resources**: Uses lazy_static for syntax/theme sets to optimize performance
3. **Result-Based Error Handling**: All operations return Result types for proper error propagation
4. **Modular Output Modes**: Cleanly separated plain, highlight, JSON, and summary modes

### Output Modes

- **plain**: Raw text without highlighting (perfect for cat replacement)
- **highlight**: Syntax-highlighted output (default)
- **json**: Structured JSON with metadata, optionally includes tokens and summary
- **summary**: Extracts important code structures (functions, classes, imports)

### Cat Replacement Mode (v0.2.2+)

- **Line numbering**: Use `-n/--number` for all lines or `-b/--number-nonblank` for non-blank lines
- **Exact compatibility**: 6-character right-aligned line numbers with tab separator
- **Perfect newlines**: Matches system cat output exactly for shell compatibility
- **AI integration**: `PAGER="batless --plain" gh api` for seamless tool integration

### Important Implementation Details

- Language detection uses file extensions via syntect's syntax set
- Smart truncation supports both `--max-lines` and `--max-bytes` limits
- Summary extraction identifies functions, classes, imports, and type definitions
- Token extraction uses basic word boundary splitting for AI processing
- ANSI stripping is automatic in non-terminal environments

## Testing Guidelines

When modifying the codebase:

1. Run the full test suite with `cargo test`
2. Add integration tests for new CLI features in `tests/integration_tests.rs`
3. Add unit tests for new library functions in `src/lib.rs`
4. Ensure all tests pass on CI before considering changes complete

## Development Workflow

### Feature Development

- Use feature branches for changes
- Implement new features with comprehensive test coverage
- Ensure clean, modular code following Rust best practices

## Common Tasks

### Adding a New Output Mode

1. Add the variant to `OutputMode` enum in `src/lib.rs`
2. Update the clap `ValueEnum` implementation
3. Implement the logic in `process_file()` function
4. Add integration tests for the new mode

### Updating Dependencies

1. Update version in `Cargo.toml`
2. Run `cargo update` to update lock file
3. Run `cargo test` to ensure compatibility
4. Run `cargo audit` to check for vulnerabilities

```
