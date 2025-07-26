# Development Guide for batless

## Quick Start

1. **Clone and setup**
   ```bash
   git clone https://github.com/docdyhr/batless
   cd batless
   cargo build
   ```

2. **Run tests**
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

3. **Make changes**
   - Create a feature branch: `git checkout -b feature/your-feature`
   - Make your changes
   - Run tests and linting
   - Commit with clear messages

## Release Process

### Automated Release (Recommended)

We now use a PR-based release workflow that respects branch protection:

```bash
# Trigger a release PR
gh workflow run manual-release-pr.yml -f version=0.1.4 -f create_tag=true -f dry_run=false

# This will:
# 1. Create a release branch (release/0.1.4)
# 2. Update version in Cargo.toml and Cargo.lock
# 3. Update CHANGELOG.md
# 4. Create a PR to main
# 5. After PR merge, automatically create tag v0.1.4
# 6. Trigger the release workflow
```

### Manual Release (if needed)

1. Create a release branch
   ```bash
   git checkout -b release/0.1.4
   ```

2. Update version
   ```bash
   # Update version in Cargo.toml
   sed -i 's/^version = ".*"/version = "0.1.4"/' Cargo.toml
   cargo update --workspace
   ```

3. Update CHANGELOG.md with release notes

4. Create PR and merge

5. After merge, create tag
   ```bash
   git checkout main
   git pull
   git tag -a v0.1.4 -m "Release 0.1.4"
   git push origin v0.1.4
   ```

## CI/CD Workflows

### Available Workflows

1. **CI Pipeline** (`.github/workflows/ci.yml`)
   - Runs on every push and PR
   - Tests on Linux, macOS, and Windows
   - Runs security audit and code coverage

2. **Release** (`.github/workflows/release.yml`)
   - Triggered by version tags (v*)
   - Builds and publishes binaries
   - Publishes to crates.io

3. **Manual Release PR** (`.github/workflows/manual-release-pr.yml`)
   - Creates release PRs that respect branch protection
   - Use this for all releases

4. **Security Audit** (`.github/workflows/security-audit.yml`)
   - Daily security vulnerability checks

### Manual Workflow Triggers

```bash
# Full test suite
gh workflow run workflow-dispatch.yml -f workflow_type=full-test-suite

# Security audit
gh workflow run workflow-dispatch.yml -f workflow_type=security-audit

# Performance benchmark
gh workflow run workflow-dispatch.yml -f workflow_type=performance-benchmark

# Quality check
gh workflow run workflow-dispatch.yml -f workflow_type=quality-check

# Quick validation
gh workflow run workflow-dispatch.yml -f workflow_type=quick-validation
```

## Branch Protection

The `main` branch is protected and requires:
- 5 status checks to pass:
  - Test (ubuntu-latest, stable)
  - Test (windows-latest, stable)
  - Test (macos-latest, stable)
  - Security Audit
  - Code Coverage
- PR reviews before merging
- Conversation resolution

## Development Tips

### Running Locally

```bash
# Debug mode
cargo run -- src/main.rs

# Release mode (optimized)
cargo run --release -- src/main.rs

# With specific options
cargo run -- --mode=summary --max-lines=50 src/lib.rs
```

### Testing

```bash
# All tests
cargo test

# Specific test
cargo test test_name

# Integration tests only
cargo test --test integration_tests

# With output
cargo test -- --nocapture
```

### Performance

```bash
# Run benchmarks
cargo bench

# Profile with flamegraph (requires cargo-flamegraph)
cargo flamegraph -- benchmark_files/large.rs

# Check binary size
cargo build --release
ls -lh target/release/batless
```

### Common Issues

1. **Release workflow fails**: Use the PR-based release workflow instead of direct push
2. **Tests fail on Windows**: Check line ending handling (CRLF vs LF)
3. **Performance regression**: Run benchmarks before merging