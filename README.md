# 🦇 batless

<div align="center">

**The Ultimate Non-Blocking Code Viewer**

Built for automation, AI assistants, and modern CLI workflows

[Quick Start](#-quick-start) • [Features](#-features) • [Installation](#-installation-options) •
[Documentation](#-documentation) • [Examples](#-examples)

[![Crates.io](https://img.shields.io/crates/v/batless?logo=rust&logoColor=white)](https://crates.io/crates/batless)
[![Crates.io Downloads](https://img.shields.io/crates/d/batless?logo=rust&logoColor=white)](https://crates.io/crates/batless)
[![GitHub Downloads](https://img.shields.io/github/downloads/docdyhr/batless/total?logo=github&logoColor=white)](https://github.com/docdyhr/batless/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-green?logo=opensource&logoColor=white)](https://opensource.org/licenses/MIT)
[![GitHub Release](https://img.shields.io/github/v/release/docdyhr/batless?include_prereleases&logo=github&logoColor=white)](https://github.com/docdyhr/batless/releases)

[![CI/CD Pipeline](https://github.com/docdyhr/batless/workflows/CI%2FCD/badge.svg)](https://github.com/docdyhr/batless/actions)
[![Security Review](https://github.com/docdyhr/batless/workflows/Security%20Review/badge.svg)](https://github.com/docdyhr/batless/actions)
[![Codecov](https://codecov.io/gh/docdyhr/batless/branch/main/graph/badge.svg?logo=codecov&logoColor=white)](https://codecov.io/gh/docdyhr/batless)
[![Test Coverage](https://img.shields.io/badge/test%20coverage-90%25%2B-brightgreen?logo=jest&logoColor=white)](https://github.com/docdyhr/batless)

[![Rust](https://img.shields.io/badge/Rust-100%25-orange?logo=rust&logoColor=white)](https://github.com/docdyhr/batless)
[![Security Tests](https://img.shields.io/badge/security%20tests-passing-brightgreen?logo=shield&logoColor=white)](https://github.com/docdyhr/batless)
[![Performance](https://img.shields.io/badge/startup-<50ms-brightgreen?logo=speedtest&logoColor=white)](https://github.com/docdyhr/batless)
[![Binary Size](https://img.shields.io/badge/binary%20size-~2MB-blue?logo=filetype&logoColor=white)](https://github.com/docdyhr/batless)

</div>

## 🎯 Why batless?

**Transform code viewing** from blocking interactive pagers to predictable streaming output:

```text
❌ Before: bat file.rs → hangs in CI/CD, requires terminal, blocks automation
✅ After:  batless file.rs → streams immediately, works everywhere, never blocks
```

**Key Advantages:**

- 🚀 **Never Blocks**: Guaranteed non-blocking operation for CI/CD and automation
- 🤖 **AI-Optimized**: JSON output, summaries, and tokens for LLM processing
- ⚡ **Blazing Fast**: <50ms startup, streaming architecture, ~2MB binary
- 🔧 **Automation-First**: Clean defaults, predictable behavior, perfect for scripts
- 📊 **Smart Output**: Multiple modes including summary extraction and token analysis

**batless** is a minimal, blazing-fast syntax viewer that **never blocks, never pages, never hangs**. While [`bat`](https://github.com/sharkdp/bat) is a feature-rich "cat with wings" for human users, `batless` is purpose-built for:

- 🤖 **AI code assistants** that need predictable, streaming output
- 🔄 **CI/CD pipelines** where interactive pagers would hang forever
- 📜 **Automation scripts** that require guaranteed non-blocking behavior
- 🚀 **Modern workflows** where JSON output and code summaries matter more than line numbers

**Core guarantee**: `batless` will NEVER wait for user input or block your pipeline.

## 🚀 Quick Start

Get up and running in **under 2 minutes**:

### Prerequisites

- **Rust Toolchain**: For building from source (or use pre-built binaries)
- **Terminal**: Any POSIX-compatible shell
- **Files to View**: Any text-based source code files

### 3-Step Setup

**1️⃣ Install batless (Choose One)**

```bash
# Option A: Pre-built binaries (fastest)
curl -L https://github.com/docdyhr/batless/releases/latest/download/batless-x86_64-unknown-linux-gnu.tar.gz | tar xz

# Option B: Via Cargo
cargo install batless

# Option C: Homebrew (macOS/Linux)
brew tap docdyhr/batless && brew install batless
```

**2️⃣ Test Your Installation**

```bash
# View a file with syntax highlighting
batless src/main.rs

# Test JSON output mode
batless --mode=json --max-lines=10 src/lib.rs
```

**3️⃣ Integrate with Your Workflow**

```bash
# CI/CD pipeline usage
batless --mode=summary --max-lines=50 failing-test.rs

# AI assistant context
batless --mode=json --include-tokens --summary src/main.rs
```

📺 **[Try the Demo](demo.sh)** | 📖 **[Complete Setup Guide](#-installation-options)**

## 🌟 What Makes batless Special

### 🏆 Feature Comparison

| Feature | `batless` | `bat` | `cat` |
|---------|-----------|-------|-------|
| **Never Blocks** | ✅ **Guaranteed** | ❌ Uses pager | ✅ Simple output |
| **Syntax Highlighting** | ✅ 100+ languages | ✅ Rich highlighting | ❌ None |
| **JSON Output** | ✅ **First-class** | ❌ Not supported | ❌ Not supported |
| **Summary Mode** | ✅ **AI-optimized** | ❌ Not supported | ❌ Not supported |
| **Memory Usage** | ✅ **Streaming** | ⚠️ Loads full file | ✅ Streaming |
| **Binary Size** | ✅ **~2MB** | ⚠️ ~10MB | ✅ System binary |
| **Startup Time** | ✅ **<50ms** | ⚠️ ~180ms | ✅ <10ms |

### 🚀 Core Capabilities

#### Non-Blocking Guarantees

- 🚫 **NEVER uses a pager** - no `less`, no `more`, no blocking
- ⚡ **NEVER waits for input** - always streams output immediately
- 🔄 **NEVER hangs in pipes** - safe for `|`, `>`, and subprocess calls
- 📊 **ALWAYS returns quickly** - even on huge files (streaming architecture)

#### Syntax & Language Support

- 🎨 **Syntax highlighting** for 100+ languages via syntect
- 🔍 **Language auto-detection** with manual override support
- 🎭 **Theme support** - Multiple color schemes available
- 🌐 **Universal support** - Works with any text-based file format

#### Smart Output Modes

- 📊 **Multiple output modes**: plain, highlighted, JSON, summary
- 📏 **Smart limiting** by lines AND/OR bytes
- 💾 **Memory efficient** - true streaming, never loads full files
- 🎯 **Predictable behavior** - same output in terminal or pipe

#### Built for Automation

- 🤖 **AI-optimized JSON** output with metadata, tokens, and summaries
- 📋 **Summary mode** extracts functions, classes, imports only
- 🔤 **Token extraction** for LLM context processing
- 🚫 **Clean defaults** - no line numbers, headers, or decorations
- 📦 **Single ~2MB binary** with minimal dependencies
- 🚀 **Sub-50ms startup** with cached syntax definitions

## ⚡ Installation Options

### GitHub Releases (Recommended)

Download pre-compiled binaries for your platform:

```bash
# macOS/Linux - download and extract latest release
curl -L https://github.com/docdyhr/batless/releases/latest/download/batless-0.1.1-x86_64-apple-darwin.tar.gz | tar xz

# Or use wget
wget https://github.com/docdyhr/batless/releases/latest/download/batless-0.1.1-x86_64-unknown-linux-gnu.tar.gz
```

Available builds:

- **Linux**: `x86_64-unknown-linux-gnu`, `x86_64-unknown-linux-musl`, `aarch64-unknown-linux-gnu`
- **macOS**: `x86_64-apple-darwin` (Intel), `aarch64-apple-darwin` (Apple Silicon)
- **Windows**: `x86_64-pc-windows-msvc`

### Homebrew (macOS/Linux)

```bash
# Add the tap (one-time setup)
brew tap docdyhr/batless

# Install batless
brew install batless

# Or install directly without adding tap
brew install docdyhr/batless/batless
```

**Homebrew Tap Repository**: [docdyhr/homebrew-batless](https://github.com/docdyhr/homebrew-batless)

### From Crates.io

```bash
# Install the latest version:
cargo install batless
```

### Docker (Containerized Environments)

```bash
# Quick syntax highlighting in any environment
docker run --rm -v $(pwd):/workspace ghcr.io/docdyhr/batless:latest /workspace/src/main.rs

# JSON output for CI/CD pipelines
docker run --rm -v $(pwd):/workspace ghcr.io/docdyhr/batless:latest --mode=json /workspace/src/main.rs

# Summary mode for AI code analysis
docker run --rm -v $(pwd):/workspace ghcr.io/docdyhr/batless:latest --mode=summary /workspace/src/
```

### From Source

```bash
git clone https://github.com/docdyhr/batless.git
cd batless
cargo build --release
```

## 🍺 Homebrew Tap

The [docdyhr/homebrew-batless](https://github.com/docdyhr/homebrew-batless) tap provides the official Homebrew formula for `batless`.

### Features

- ✅ **Automatically updated** with every release
- ✅ **Comprehensive testing** included in formula
- ✅ **Cross-platform** support (macOS & Linux)
- ✅ **Zero maintenance** - formula stays in sync with releases

### Installation Commands

```bash
# Method 1: Add tap first (recommended)
brew tap docdyhr/batless
brew install batless

# Method 2: Direct install
brew install docdyhr/batless/batless

# Upgrade to latest version
brew upgrade batless
```

The formula automatically compiles from source using Rust, ensuring optimal performance for your system.

## 🎯 Real-World Use Cases

### 🤖 AI Assistant Integration

> **📖 [Complete AI & Editor Integration Guide](docs/AI_EDITOR_INTEGRATION.md)** - Comprehensive setup for Zed, VS Code, and GitHub CLI

**Quick Examples:**

```bash
# Use built-in AI profiles for optimal results
batless --profile=claude file.rs        # Claude-optimized summary mode
batless --profile=copilot file.rs       # GitHub Copilot JSON + tokens
batless --profile=chatgpt file.rs       # ChatGPT optimized output

# Smart GitHub CLI integration
export GH_PAGER="batless --plain --max-lines=500 --summary-level=standard"
gh pr view 42  # Perfect for AI analysis
```

**Advanced AI Workflows:**

```bash
# Context-aware fitting for AI models
batless --fit-context --ai-model=claude35-sonnet large-file.py

# Token counting for context planning
batless --count-tokens --ai-model=gpt4 file.rs

# Multi-file AI context building
find src/ -name "*.rs" | head -5 | xargs batless --profile=claude
```

### 🔄 CI/CD Pipeline Integration

**GitHub Actions Example:**

```yaml
- name: Show failing test context
  run: |
    batless --mode=summary --max-lines=100 tests/failing_test.rs

- name: Extract code metrics
  run: |
    batless --mode=json src/main.rs | jq '.total_lines'
```

**Jenkins Pipeline:**

```groovy
stage('Code Analysis') {
    steps {
        sh 'batless --mode=json --summary src/ | jq ".summary_lines | length"'
    }
}
```

**GitLab CI:**

```yaml
code_review:
  script:
    - batless --color=never --max-lines=50 src/main.rs
    - batless --mode=summary --max-lines=100 tests/
```

### 🛠️ Development Workflows

**Code Review Automation:**

```bash
# Show changed files without paging
git diff --name-only | xargs batless --mode=summary

# Generate PR context for AI review
batless --mode=json --include-tokens changed-files.rs

# Quick file preview in terminal
batless --max-lines=30 --theme="InspiredGitHub" src/new-feature.rs
```

**Documentation Generation:**

```bash
# Extract code structure for docs
batless --mode=summary src/ > code-structure.md

# Generate API documentation context
batless --mode=json --summary src/api.rs | jq '.summary_lines[]'

# Create code snippets for tutorials
batless --max-lines=20 examples/hello-world.rs
```

### 📊 Performance Monitoring

**Build System Integration:**

```bash
# Show code during build failures (non-blocking)
batless --color=never --max-lines=30 failing-test.js

# Get code summary for automated analysis
batless --mode=summary --color=never failing-module.py

# Extract enhanced metadata for build systems
batless --mode=json src/main.rs | jq '{language, encoding, total_lines, truncated}'
```

**Large File Processing:**

```bash
# Process huge files without memory issues
batless --max-bytes=1048576 --mode=summary huge-log-file.txt

# Stream first 1000 lines of large dataset
batless --max-lines=1000 --mode=plain data/large-dataset.csv

# Extract key information from massive JSON
batless --max-bytes=500000 --mode=json config/large-config.json
```

## 📖 Usage

### Basic Usage

```bash
# View a file with syntax highlighting
batless src/main.rs

# Plain text output (no colors)
batless --mode=plain src/main.rs

# JSON output for parsing
batless --mode=json src/main.rs
```

### PAGER Compatibility

**🔧 Use as PAGER replacement** - Perfect for tools like GitHub CLI:

```bash
# GitHub CLI integration
PAGER="batless --plain" gh pr view 46

# General PAGER replacement
export PAGER="batless --plain"

# Pipeline input support
echo "Sample content" | batless --plain

# Compatible flags (ignored for compatibility)
batless --plain --unbuffered --number file.txt
```

**Key PAGER features:**

- ✅ `--plain` flag for plain text output (no colors/decorations)
- ✅ stdin support for pipeline input
- ✅ Compatible with existing PAGER workflows
- ✅ Gracefully ignores common PAGER flags (`--unbuffered`, `--number`)

### Limiting Output

```bash
# Limit to first 50 lines
batless --max-lines=50 large-file.py

# Limit to first 1KB
batless --max-bytes=1024 data.json

# Combine limits
batless --max-lines=100 --max-bytes=5000 file.txt
```

### Language and Syntax

```bash
# Auto-detect language (default)
batless script.py

# Force specific language
batless --language=python unknown-extension

# List supported languages
batless --language=help
```

### Color and Themes

```bash
# Control color output
batless --color=always file.rs    # Force colors
batless --color=never file.rs     # No colors
batless --color=auto file.rs      # Auto-detect terminal

# Choose syntax theme
batless --theme="Solarized (dark)" file.rs
batless --theme="InspiredGitHub" file.rs

# List all supported languages and themes
batless --list-languages
batless --list-themes

# Strip ANSI codes from output
batless --strip-ansi file.rs
```

### Enhanced JSON Mode Examples

```bash
# Get structured file info with enhanced metadata
batless --mode=json --max-lines=10 src/main.rs
```

Output:

```json
{
  "file": "src/main.rs",
  "language": "Rust",
  "lines": ["use std::io;", "// ..."],
  "total_lines": 10,
  "total_bytes": 245,
  "truncated": true,
  "truncated_by_lines": true,
  "truncated_by_bytes": false,
  "encoding": "UTF-8",
  "syntax_errors": [],
  "mode": "json"
}
```

### AI-Friendly Summary Mode

```bash
# Extract only important code structures (perfect for AI context)
batless --mode=summary src/main.rs

# Get function signatures, class definitions, imports only
batless --mode=summary --max-lines=50 complex-file.py
```

### Advanced JSON with Tokens and Summary

```bash
# Full AI analysis with tokens and code summary
batless --mode=json --include-tokens --summary src/main.rs
```

Enhanced output:

```json
{
  "file": "src/main.rs",
  "language": "Rust",
  "lines": ["use std::io;", "fn main() {", "..."],
  "summary_lines": ["use std::io;", "fn main() {", "pub struct Config {"],
  "tokens": ["use", "std", "io", "fn", "main", "pub", "struct", "Config"],
  "total_lines": 150,
  "total_bytes": 3420,
  "truncated": false,
  "encoding": "UTF-8",
  "mode": "json"
}
```

## 🐳 Docker Usage

### Container-Based Code Analysis

```bash
# Basic syntax highlighting
docker run --rm -v $(pwd):/workspace \
  ghcr.io/docdyhr/batless:latest /workspace/src/main.rs

# JSON output for CI/CD integration
docker run --rm -v $(pwd):/workspace \
  ghcr.io/docdyhr/batless:latest --mode=json /workspace/src/main.rs

# AI-friendly summary extraction
docker run --rm -v $(pwd):/workspace \
  ghcr.io/docdyhr/batless:latest --mode=summary /workspace/src/
```

### CI/CD Pipeline Integration

```yaml
# GitHub Actions example
- name: Analyze code structure
  run: |
    docker run --rm -v ${{ github.workspace }}:/workspace \
      ghcr.io/docdyhr/batless:latest \
      --mode=json --max-lines=100 /workspace/src/main.rs | \
      jq '.summary_lines | length'

# GitLab CI example
analyze_code:
  image: docker:latest
  script:
    - docker run --rm -v $PWD:/workspace
        ghcr.io/docdyhr/batless:latest
        --mode=summary /workspace/src/
```

### Kubernetes Jobs

```yaml
apiVersion: batch/v1
kind: Job
metadata:
  name: code-analysis
spec:
  template:
    spec:
      containers:
      - name: batless
        image: ghcr.io/docdyhr/batless:latest
        args: ["--mode=json", "/workspace/src/main.rs"]
        volumeMounts:
        - name: source-code
          mountPath: /workspace
      volumes:
      - name: source-code
        hostPath:
          path: /path/to/source
      restartPolicy: Never
```

## 🤖 AI Assistant Integration

### Claude Code Assistant

```bash
# Get code structure for AI analysis
batless --mode=summary --max-lines=50 complex-file.py

# Full AI context with summary and tokens
batless --mode=json --summary --include-tokens --max-lines=100 src/main.rs

# List supported languages for analysis
batless --list-languages | grep -i python
```

### CI/CD Pipelines

```bash
# Show code during build failures (non-blocking)
batless --color=never --max-lines=30 failing-test.js

# Get code summary for automated analysis
batless --mode=summary --color=never failing-module.py

# Extract enhanced metadata for build systems
batless --mode=json src/main.rs | jq '{language, encoding, total_lines, truncated}'
```

## 🎨 Available Themes

Popular themes include:

- `base16-ocean.dark` (default)
- `InspiredGitHub`
- `Solarized (dark)`
- `Solarized (light)`
- `Monokai`
- `1337`

View all available themes:

```bash
batless --list-themes
```

## 🗣️ Supported Languages

Support for 100+ languages including:

- Rust, Python, JavaScript, TypeScript
- C, C++, Java, Go, Swift
- HTML, CSS, JSON, YAML, TOML
- Shell, Bash, PowerShell
- And many more...

View all supported languages:

```bash
batless --list-languages
```

## ⚙️ Configuration

`batless` supports flexible configuration through files and command-line arguments, with a clear precedence hierarchy:

**Configuration Precedence** (highest to lowest):

1. Command-line arguments
2. Project-level config (`.batlessrc`, `batless.toml`)
3. User home config (`~/.batlessrc`, `~/.config/batless/config.toml`)
4. System defaults

### Configuration Files

#### TOML Format (Recommended)

Create `batless.toml` in your project root or `~/.config/batless/config.toml`:

```toml
# Maximum lines to display
max_lines = 15000

# Maximum bytes to process (optional)
max_bytes = 1048576  # 1MB

# Override language detection
language = "rust"

# Theme for syntax highlighting
theme = "monokai"

# Color output control
use_color = true

# Strip ANSI escape sequences
strip_ansi = false

# Include tokens in JSON output
include_tokens = false

# Enable summary mode by default
summary_mode = true
```

#### JSON Format (.batlessrc)

Create `.batlessrc` in your project root or home directory:

```json
{
  "max_lines": 8000,
  "theme": "github",
  "use_color": true,
  "summary_mode": false,
  "include_tokens": true
}
```

### Configuration Examples

#### Project-Specific Settings

For a Rust project, create `batless.toml`:

```toml
# Optimize for Rust development
max_lines = 20000
theme = "base16-ocean.dark"
language = "rust"
summary_mode = true
use_color = true
```

#### AI Assistant Profile

For AI code analysis, create `.batlessrc`:

```json
{
  "max_lines": 5000,
  "theme": "github",
  "use_color": false,
  "summary_mode": true,
  "include_tokens": false
}
```

#### CI/CD Pipeline Settings

For automation environments:

```toml
max_lines = 1000
use_color = false
strip_ansi = true
summary_mode = false
```

### Custom Config File

Use `--config` to specify a custom configuration file:

```bash
# Use specific config file
batless --config my-config.toml src/main.rs

# Override with command line args
batless --config team-settings.toml --max-lines 500 src/lib.rs
```

### Configuration Discovery

`batless` automatically searches for config files in this order:

1. **Project level**: `.batlessrc`, `batless.toml`
2. **User home**: `~/.batlessrc`, `~/.config/batless/config.toml`
3. **System level**: System config directories

### AI Tool Profiles

Instead of manual configuration, use built-in AI profiles:

```bash
# Claude-optimized (4K lines, summary mode)
batless --profile claude src/main.rs

# GitHub Copilot (2K lines, JSON + tokens)
batless --profile copilot src/main.rs

# ChatGPT-optimized (3K lines, JSON + tokens)
batless --profile chatgpt src/main.rs

# General AI assistant (5K lines, summary)
batless --profile assistant src/main.rs
```

### Validation and Help

`batless` validates all configuration and provides helpful error messages:

```bash
# Example validation error
$ batless --max-lines 0 src/main.rs
Error: max_lines must be greater than 0
Help: Try using --max-lines with a positive number (e.g., --max-lines 1000)
```

Common configuration patterns and their use cases are documented in the [project wiki](https://github.com/docdyhr/batless/wiki/Configuration-Examples).

## 🆚 Why batless instead of bat?

### When to use `batless`

- ✅ **CI/CD pipelines** - Guaranteed to never hang waiting for input
- ✅ **AI assistants** - Clean output with JSON mode and code summaries
- ✅ **Automation scripts** - Predictable, streaming behavior
- ✅ **Large file processing** - Memory-efficient streaming architecture
- ✅ **Headless environments** - No terminal detection or pager issues

### When to use `bat`

- ✅ **Interactive terminal use** - Rich features like paging and git integration
- ✅ **Human code review** - Line numbers, file headers, and decorations
- ✅ **Git workflows** - Shows inline diffs and modifications
- ✅ **Terminal multiplexing** - Full terminal UI features

### Feature Comparison

| Feature | `batless` | `bat` |
|---------|-----------|-------|
| **Core Philosophy** | Built for machines | Built for humans |
| **Blocking behavior** | ✅ **NEVER blocks** | ❌ Uses interactive pager |
| **Default output** | ✅ Clean, no decorations | ❌ Headers, grids, line numbers |
| **JSON output** | ✅ First-class with metadata | ❌ Not supported |
| **Summary mode** | ✅ Extract code structure | ❌ Not supported |
| **Token extraction** | ✅ For AI processing | ❌ Not supported |
| **Byte limiting** | ✅ Memory-safe streaming | ❌ Loads entire file |
| **Binary size** | ✅ ~2MB minimal | ❌ ~10MB with features |
| **Startup time** | ✅ <50ms cached | ⚠️ ~180ms full init |
| **Dependencies** | ✅ 9 crates | ❌ 20+ crates |
| **Git integration** | ❌ No (by design) | ✅ Full support |
| **Line numbers** | ❌ No (use `cat -n` if needed) | ✅ Configurable |
| **Interactive paging** | ❌ No (by design) | ✅ Smart pager integration |

## 🧪 Testing & Status

### Current Test Status ✅

- **Main Test Suite**: 100% passed
- **Integration Tests**: 100% passed
- **Property-Based Tests**: 100% passed
- **Security Audit**: Clean
- **CI/CD Pipeline**: Fully functional

### Test Your Installation

```bash
# Run the demo script
./demo.sh

# Test with a sample file
echo 'fn main() { println!("Hello, World!"); }' | batless --language=rust

# Verify JSON output
batless --mode=json src/main.rs | jq '.language'
```

## 🔒 Security Status

### Comprehensive Security Testing

Our security posture is continuously monitored through automated testing and vulnerability scanning:

| **Security Area** | **Status** | **Coverage** |
|------------------|------------|--------------|
| **Memory Safety** | ✅ Secure | Rust's memory safety guarantees |
| **Input Validation** | ✅ Secure | All inputs validated and sanitized |
| **Dependency Audit** | ✅ Secure | Regular `cargo audit` checks |
| **Binary Security** | ✅ Secure | Stripped, optimized releases |
| **Supply Chain** | ✅ Secure | Trusted dependencies only |

### Security Features

- **🛡️ Memory Safety**: Built with Rust for guaranteed memory safety
- **🔍 Input Validation**: All file paths and parameters validated
- **📊 Dependency Audit**: Automated vulnerability scanning
- **🚨 Safe Defaults**: No unsafe operations or external commands

### Security Testing Commands

```bash
# Security audit
cargo audit

# Dependency check
cargo deny check

# Format and lint checks
cargo fmt --all -- --check
cargo clippy -- -D warnings
```

## 🐛 Troubleshooting

### Quick Diagnostics

**Installation Issues**

```bash
# Verify Rust toolchain
rustc --version
cargo --version

# Check binary location
which batless
batless --version

# Test basic functionality
echo "test" | batless --mode=plain
```

**Performance Issues**

```bash
# Check syntax cache
ls ~/.cache/batless/ || ls ~/Library/Caches/batless/

# Benchmark performance
time batless --mode=summary large-file.rs

# Memory usage monitoring
/usr/bin/time -v batless large-file.rs
```

**Output Format Issues**

```bash
# Test color support
batless --color=always test-file.rs

# Verify JSON format
batless --mode=json test-file.rs | jq .

# Check theme availability
batless --list-themes
```

### Common Error Solutions

| Error | Cause | Solution |
|-------|-------|----------|
| `No such file or directory` | File path incorrect | Verify file path exists |
| `Permission denied` | File permissions | Check read permissions |
| `Language not found` | Unknown extension | Use `--language` flag |
| `JSON parse error` | Invalid JSON output | Check file encoding |
| `Binary not found` | Installation issue | Reinstall or check PATH |

### Getting Help

**Self-Diagnostics**

```bash
# Version and build info
batless --version

# List all supported languages
batless --list-languages

# List all themes
batless --list-themes

# Test configuration
batless --help
```

**Community Support**

- 🐛 [Report Issues](https://github.com/docdyhr/batless/issues)
- 💬 [Discussions](https://github.com/docdyhr/batless/discussions)
- 📚 [Documentation](https://github.com/docdyhr/batless/wiki)
- 📧 [Contact](mailto:support@docdyhr.com)

## 🛠️ Development

### Running Tests

```bash
# Run all tests
cargo test

# Run property-based tests
cargo test --test property_tests

# Run benchmarks
cargo bench

# Run security checks
./scripts/security-check.sh
```

### Building & Quality Checks

```bash
# Build release
cargo build --release

# Comprehensive linting
cargo clippy --all-targets --all-features -- -D warnings

# Code formatting
cargo fmt --all -- --check

# Security audit
cargo audit

# Generate coverage report
cargo install cargo-llvm-cov
cargo llvm-cov --html
```

### Security & Testing

This project maintains high security and quality standards:

- ✅ **90%+ test coverage** with unit, integration, and property-based tests
- ✅ **Daily security audits** with automated vulnerability scanning
- ✅ **Fuzz testing** for crash resistance and input validation
- ✅ **Memory safety** verification with Valgrind
- ✅ **Supply chain security** with OSSF Scorecard monitoring
- ✅ **Performance benchmarking** with regression detection

See [SECURITY_TESTING.md](SECURITY_TESTING.md) for detailed security measures.

## 📊 Performance

`batless` is designed for speed and low memory usage:

- **Streaming**: Never loads entire files into memory
- **Fast startup**: Cached syntax sets and optimized loading
- **Efficient highlighting**: Pre-loaded syntax and theme sets
- **Small binary**: ~2MB release build
- **Memory efficient**: Constant memory usage regardless of file size

Enhanced benchmarks on a 10MB Python file:

```
batless (optimized): 95ms (streaming + cached)
batless (summary): 45ms (structure only)
bat: 180ms (full load)
cat: 50ms (no highlighting)
```

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Branch Protection & Contributing

This repository uses branch protection rules to ensure code quality and security:

- **Pull requests required** - No direct commits to `main`
- **CI/CD checks required** - All tests must pass
- **GPG signed commits recommended** - For authenticity verification

#### Quick Setup

```bash
# Setup branch protection (one-time)
gh auth login
./scripts/setup-branch-protection-gh.sh

# Verify configuration
./scripts/verify-protection-gh.sh
```

#### Development Workflow

```bash
# 1. Create feature branch
git checkout -b feature/my-feature

# 2. Make changes and commit
git add .
git commit -m "feat: description"

# 3. Push and create PR
git push origin feature/my-feature
gh pr create --title "feat: description"

# 4. Wait for CI, then merge
gh pr merge --squash
```

See [`docs/BRANCH_PROTECTION.md`](docs/BRANCH_PROTECTION.md) for detailed guidance.

### Release Automation

This project features fully automated releases and Homebrew tap updates:

- **Automated Releases**: Every git tag triggers cross-platform binary builds, GitHub releases, and crates.io publishing
- **Homebrew Integration**: The [homebrew-batless](https://github.com/docdyhr/homebrew-batless) tap automatically updates with each release
- **Zero Maintenance**: Formula SHA256 hashes and versions are calculated and updated automatically

#### Release Process

```bash
# Create and push a new tag - everything else is automated
git tag v0.1.6
git push origin v0.1.6

# Automated workflows will:
# ✅ Build binaries for all platforms
# ✅ Create GitHub release with assets
# ✅ Publish to crates.io
# ✅ Update Homebrew tap with correct SHA256
# ✅ Users get latest version via all install methods
```

See [`docs/HOMEBREW_AUTOMATION.md`](docs/HOMEBREW_AUTOMATION.md) for technical details.

### Development Setup

```bash
git clone https://github.com/docdyhr/batless.git
cd batless
cargo test
cargo run -- --help
```

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by [`sharkdp/bat`](https://github.com/sharkdp/bat)
- Built with [`syntect`](https://github.com/trishume/syntect) for syntax highlighting
- Designed for AI assistants like Claude and Gemini

## 📚 Documentation

### Getting Started

- **[Quick Start Guide](#-quick-start)** - Get running in 2 minutes
- **[Installation Guide](#-installation-options)** - All installation methods
- **[Usage Examples](#-real-world-use-cases)** - Common workflows and patterns
- **[Troubleshooting](#-troubleshooting)** - Common issues and solutions

### Advanced Usage

- **[🤖 AI & Editor Integration Guide](docs/AI_EDITOR_INTEGRATION.md)** - Complete setup for Zed, VS Code, and GitHub CLI
- **[Configuration Guide](CLAUDE.md)** - All configuration options and profiles
- **[AI Integration Examples](#-ai-assistant-integration)** - Claude, ChatGPT, Copilot
- **[CI/CD Integration](#-cicd-pipeline-integration)** - GitHub Actions, Jenkins, GitLab
- **[Performance Tuning](#-performance)** - Optimization tips and benchmarks

### Development

- **[Contributing Guide](CONTRIBUTING.md)** - Development guidelines and setup
- **[Architecture Overview](DEVELOPMENT_GUIDE.md)** - System design and structure
- **[Security Guidelines](SECURITY.md)** - Security best practices
- **[Release Process](RELEASE.md)** - How releases are managed

## 🚀 Next Steps

**Ready to transform your code viewing experience?**

1. **⚡ [Install batless](#-installation-options)** - Choose your preferred method (2 minutes)
2. **🎯 [Try Real Examples](#-real-world-use-cases)** - See what's possible with your workflow
3. **🤖 [Integrate with AI](#-ai-assistant-integration)** - Enhance your AI assistant workflows
4. **💬 [Join Community](https://github.com/docdyhr/batless/discussions)** - Get help and share ideas

---

## 🔗 Links & Resources

### Distribution Channels

- **Main Repository**: [github.com/docdyhr/batless](https://github.com/docdyhr/batless)
- **Homebrew Tap**: [github.com/docdyhr/homebrew-batless](https://github.com/docdyhr/homebrew-batless)
- **Crates.io Package**: [crates.io/crates/batless](https://crates.io/crates/batless)
- **Documentation**: [docs.rs/batless](https://docs.rs/batless)

### Community & Support

- **Issues & Bug Reports**: [github.com/docdyhr/batless/issues](https://github.com/docdyhr/batless/issues)
- **Feature Discussions**: [github.com/docdyhr/batless/discussions](https://github.com/docdyhr/batless/discussions)
- **Security Reports**: [security@docdyhr.com](mailto:security@docdyhr.com)
- **General Support**: [support@docdyhr.com](mailto:support@docdyhr.com)

---

## 🙏 Acknowledgments

Special thanks to:

- **[sharkdp/bat](https://github.com/sharkdp/bat)** - Inspiration for syntax highlighting excellence
- **[syntect](https://github.com/trishume/syntect)** - Powerful syntax highlighting engine
- **Rust Community** - For building amazing tools and ecosystem
- **AI Assistant Communities** - For driving the need for automation-friendly tools

---

<div align="center">

**⭐ Found this helpful? [Give us a star on GitHub!](https://github.com/docdyhr/batless) ⭐**

**Made with ❤️ for AI assistants and modern CLI workflows**

</div>
