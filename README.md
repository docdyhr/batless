# 🦇 batless

<div align="center">

## The Ultimate Non-Blocking Code Viewer

Built for automation, AI assistants, and modern CLI workflows

[![Crates.io](https://img.shields.io/crates/v/batless?logo=rust&logoColor=white)](https://crates.io/crates/batless)
[![Crates.io Downloads](https://img.shields.io/crates/d/batless?logo=rust&logoColor=white)](https://crates.io/crates/batless)
[![GitHub Downloads](https://img.shields.io/github/downloads/docdyhr/batless/total?logo=github&logoColor=white)](https://github.com/docdyhr/batless/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-green?logo=opensource&logoColor=white)](https://opensource.org/licenses/MIT)
[![GitHub Release](https://img.shields.io/github/v/release/docdyhr/batless?include_prereleases&logo=github&logoColor=white)](https://github.com/docdyhr/batless/releases)

[![Health Check](https://github.com/docdyhr/batless/actions/workflows/health-check.yml/badge.svg?branch=main)](https://github.com/docdyhr/batless/actions/workflows/health-check.yml)
[![Security](https://github.com/docdyhr/batless/actions/workflows/security.yml/badge.svg?branch=main)](https://github.com/docdyhr/batless/actions/workflows/security.yml)
[![Fuzz Testing](https://github.com/docdyhr/batless/actions/workflows/fuzz.yml/badge.svg?branch=main)](https://github.com/docdyhr/batless/actions/workflows/fuzz.yml)
[![Codecov](https://codecov.io/gh/docdyhr/batless/branch/main/graph/badge.svg?logo=codecov&logoColor=white)](https://codecov.io/gh/docdyhr/batless)

[![Rust](https://img.shields.io/badge/Rust-100%25-orange?logo=rust&logoColor=white)](https://github.com/docdyhr/batless)
[![Security Tests](https://img.shields.io/badge/security%20tests-passing-brightgreen?logo=shield&logoColor=white)](https://github.com/docdyhr/batless)
[![Performance](https://img.shields.io/badge/startup-<5ms*-brightgreen?logo=speedtest&logoColor=white)](https://github.com/docdyhr/batless)
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
- ⚡ **Blazing Fast**: <5ms typical startup (modern hardware), streaming architecture, ~2MB binary
- 🔧 **Automation-First**: Clean defaults, predictable behavior, perfect for scripts
- 📊 **Smart Output**: Multiple modes including summary extraction and token analysis

**batless** is a minimal, blazing-fast syntax viewer that **never blocks, never pages, never hangs**. While [`bat`](https://github.com/sharkdp/bat) is a feature-rich "cat with wings" for human users, `batless` is purpose-built for:

- 🤖 **AI code assistants** that need predictable, streaming output
- 🔄 **CI/CD pipelines** where interactive pagers would hang forever
- 📜 **Automation scripts** that require guaranteed non-blocking behavior
- 🚀 **Modern workflows** where JSON output and code summaries matter more than line numbers

**Core guarantee**: `batless` will NEVER wait for user input or block your pipeline.

## 🚀 Quick Start

### Installation

#### Option A: Pre-built Binaries (Fastest)

```bash
# Linux (x86_64)
curl -L https://github.com/docdyhr/batless/releases/latest/download/batless-x86_64-unknown-linux-gnu.tar.gz | tar xz

# macOS (Intel)
curl -L https://github.com/docdyhr/batless/releases/latest/download/batless-x86_64-apple-darwin.tar.gz | tar xz

# macOS (Apple Silicon)
curl -L https://github.com/docdyhr/batless/releases/latest/download/batless-aarch64-apple-darwin.tar.gz | tar xz
```

#### Option B: Via Cargo

```bash
cargo install batless
```

#### Option C: Homebrew (macOS/Linux)

```bash
brew tap docdyhr/batless
brew install batless
```

### Basic Usage

```bash
# View a file with syntax highlighting
batless src/main.rs

# Plain text output (no colors)
batless --plain file.py

# With line numbers (cat -n compatibility)
batless -n file.py

# JSON output for structured processing
batless --mode=json --max-lines=100 src/lib.rs

# Extract code summary (functions, classes, imports)
batless --mode=summary src/main.rs

# Get version info as JSON
batless --version-json
```

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
| **Startup Time** | ✅ **<5ms (typical)** | ⚠️ ~180ms | ✅ <10ms |

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
- 📏 **Smart limiting** by lines (`--max-lines`) and/or bytes (`--max-bytes`)
- 💾 **Memory efficient** - true streaming, never loads full files
- 🎯 **Predictable behavior** - same output in terminal or pipe

#### Built for Automation

- 🤖 **AI-optimized JSON** output with metadata, tokens, and summaries
- 📋 **Summary mode** extracts functions, classes, imports only
- 🔤 **Token extraction** for LLM context processing
- 🚫 **Clean defaults** - no decorations unless requested
- 📦 **Single ~2MB binary** with minimal dependencies

## 🚫 What batless is NOT

**batless** has a focused design philosophy. It intentionally does NOT provide:

### Features We Don't Implement (By Design)

| Feature | Why Not? | Use Instead |
|---------|----------|-------------|
| **Pattern Search** | That's `grep`'s job | `grep -rn "pattern" path/` |
| **Arbitrary Line Ranges** | Beyond our scope | `sed -n '10,50p' file` |
| **File Globbing** | Shell handles this | `batless *.py` (shell expands) |
| **Interactive Paging** | We're non-blocking | Use `bat` or `less` |
| **Git Integration** | Keep it simple | Use `git diff` or `bat` |
| **File Management** | Not a file browser | `ls`, `find`, `fd` |
| **Text Editing** | Viewer only | Use your editor |

### Common Misconceptions

❌ **"batless is a drop-in replacement for bat"**
✅ **Reality**: batless is purpose-built for automation and AI, not interactive use

❌ **"batless should add grep-like search"**
✅ **Reality**: Unix philosophy - do one thing well. Use `grep` for searching

❌ **"batless needs more features like bat"**
✅ **Reality**: Less is more. Our constraints are features for automation

### When NOT to Use batless

- 👤 **Interactive code review**: Use `bat` - it has better human-focused features
- 🔍 **Searching code**: Use `grep`, `rg` (ripgrep), or `ag` (silver searcher)
- 📝 **Editing files**: Use your favorite editor
- 📊 **Complex analysis**: Use language-specific tools (pylint, rust-analyzer, etc.)
- 🎨 **Pretty printing**: Use `bat` with its full decoration suite

### Our Philosophy

```text
Do ONE thing well: Stream code with syntax highlighting, never block.
Everything else? There's already a better tool for that.
```

## 📖 Usage Examples

### Basic File Viewing

```bash
# Syntax highlighted output
batless main.rs

# Plain text (no colors)
batless --plain main.rs

# With line numbers
batless -n main.rs

# Limit output
batless --max-lines=50 large-file.py
batless --max-bytes=10000 huge-file.log
```

### AI & Automation Workflows

```bash
# JSON output for LLM processing
batless --mode=json --include-tokens --summary src/main.rs | jq

# Extract code structure only
batless --mode=summary src/*.rs

# CI/CD context generation
batless --mode=json --max-lines=100 failing-test.rs > context.json

# Machine-readable metadata
batless --version-json
```

### Pipeline Integration

```bash
# Use as PAGER replacement
PAGER="batless --plain" gh pr view 42

# Process multiple files
find src -name "*.rs" -exec batless --mode=summary {} \;

# Combine with grep
grep -l "TODO" src/*.py | xargs batless -n

# Stream stdin
cat file.rs | batless --language=rust
```

### Custom Profiles

```bash
# Use AI-optimized profile
batless --profile=claude main.rs

# Interactive configuration wizard
batless --configure

# List available profiles
batless --list-profiles
```

## 🎨 Configuration

### Themes

batless supports multiple color themes for syntax highlighting:

```bash
# List available themes
batless --list-themes

# Use specific theme
batless --theme="Solarized (dark)" file.py
```

#### Available Themes

batless currently includes 7 carefully curated themes:

- **InspiredGitHub** - Clean, GitHub-inspired light theme
- **Solarized (dark)** - Popular dark theme with excellent contrast
- **Solarized (light)** - Light variant of the Solarized theme
- **base16-eighties.dark** - Retro 80s-inspired dark theme
- **base16-mocha.dark** - Warm, chocolate-toned dark theme
- **base16-ocean.dark** - Cool, oceanic dark theme
- **base16-ocean.light** - Light variant of the ocean theme

Try different themes to find the one that works best for your workflow:

```bash
# Try each theme with your code
batless --theme="InspiredGitHub" examples/theme-showcase.rs
batless --theme="Solarized (dark)" examples/theme-showcase.rs
batless --theme="base16-mocha.dark" examples/theme-showcase.rs
```

> **Note**: Theme examples are available in [docs/themes/](docs/themes/) and can be regenerated with `./scripts/generate-theme-showcase.sh`

### Language Detection

```bash
# Auto-detect (default)
batless file.txt

# Force specific language
batless --language=python unknown.file

# List supported languages
batless --list-languages
```

### Custom Profiles

Create custom profiles in `~/.batless/profiles/`:

```toml
# ~/.batless/profiles/my-profile.toml
name = "my-profile"
max_lines = 1000
summary_level = "medium"
include_tokens = true
```

Use with:

```bash
batless --custom-profile ~/.batless/profiles/my-profile.toml file.rs
```

### Shell Completions

batless includes built-in shell completion support for bash, zsh, fish, and PowerShell.

#### Bash

```bash
# Generate and install completions
batless --generate-completions bash > ~/.local/share/bash-completion/completions/batless

# Or for system-wide installation
sudo batless --generate-completions bash > /usr/share/bash-completion/completions/batless

# Then reload your shell or source the completion file
source ~/.local/share/bash-completion/completions/batless
```

#### Zsh

```bash
# Generate and install completions
batless --generate-completions zsh > ~/.zsh/completions/_batless

# Add to your ~/.zshrc (if not already present)
fpath=(~/.zsh/completions $fpath)
autoload -Uz compinit && compinit

# Then reload your shell
exec zsh
```

#### Fish

```bash
# Generate and install completions
batless --generate-completions fish > ~/.config/fish/completions/batless.fish

# Completions are automatically loaded in new fish shells
```

#### PowerShell

```powershell
# Generate and add to your profile
batless --generate-completions powershell | Out-String | Invoke-Expression

# Or save to your profile for persistence
batless --generate-completions powershell >> $PROFILE
```

## 🔧 CLI Options

### Output Modes

- `--mode <MODE>` - Output mode: `plain`, `highlight`, `json`, `summary`
- `--plain` - Plain text output (equivalent to `--mode=plain`)
- `--mode=json` - Structured JSON output for automation
- `--mode=summary` - Extract only key code structures

### Limiting Output

- `--max-lines <N>` - Limit output to N lines
- `--max-bytes <N>` - Limit output to N bytes
- `--lines <START:END>` - Select specific line range (e.g., `10:50`, `:100`, `50:`)

### Display Options

- `-n, --number` - Show line numbers (cat -n compatibility)
- `-b, --number-nonblank` - Number non-blank lines only (cat -b compatibility)
- `--theme <THEME>` - Color scheme to use
- `--language <LANG>` - Force specific language syntax

### AI/Automation Features

- `--include-tokens` - Include token analysis in JSON output
- `--summary` - Add code summary to JSON output
- `--profile <PROFILE>` - Use AI-optimized profile (claude, copilot, chatgpt)
- `--custom-profile <PATH>` - Load custom profile from file

### Configuration

- `--configure` - Launch interactive configuration wizard
- `--list-profiles` - Show all available custom profiles
- `--list-themes` - Show all available color themes
- `--list-languages` - Show all supported languages

### Utility

- `--version` - Show version information
- `--version-json` - Machine-readable version metadata
- `--help` - Show detailed help information

## 🤖 AI Assistant Integration

batless is designed to work seamlessly with AI coding assistants:

### Claude Code

```bash
# Use batless in Claude Code workflows
batless --profile=claude --max-lines=500 src/main.rs
```

### GitHub Copilot CLI

```bash
# Generate context for Copilot
batless --mode=json --summary src/ | gh copilot suggest
```

### ChatGPT / Other LLMs

```bash
# Generate structured context
batless --mode=json --include-tokens --max-lines=1000 file.rs > context.json
```

See [docs/AI_INTEGRATION.md](docs/AI_INTEGRATION.md) for detailed integration guides.

## 🏗️ Architecture

batless is built with:

- **Rust** - Memory safety and performance
- **syntect** - Syntax highlighting engine
- **Streaming architecture** - Memory-efficient processing
- **Modular design** - Clean separation of concerns

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for technical details.

## 🤝 Contributing

We welcome contributions! Please see:

- [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) - Community standards
- [docs/PHILOSOPHY_AND_SCOPE.md](docs/PHILOSOPHY_AND_SCOPE.md) - Project philosophy

### Development Setup

```bash
# Clone repository
git clone https://github.com/docdyhr/batless.git
cd batless

# Build
cargo build

# Run tests
cargo test

# Run with example
cargo run -- src/main.rs
```

## 📊 Performance

- **Startup time**: <5ms typical on modern hardware
- **Binary size**: ~2MB (minimal dependencies)
- **Memory usage**: Constant (streaming architecture)
- **Throughput**: Limited only by syntax highlighting speed

*Note: Performance varies by hardware. Benchmarks on typical developer workstation.*

## 📜 License

MIT License - see [LICENSE](LICENSE) for details.

## 🔗 Links

- **Documentation**: [docs/](docs/)
- **Changelog**: [CHANGELOG.md](CHANGELOG.md)
- **Releases**: [GitHub Releases](https://github.com/docdyhr/batless/releases)
- **Issues**: [GitHub Issues](https://github.com/docdyhr/batless/issues)
- **Crates.io**: [crates.io/crates/batless](https://crates.io/crates/batless)

## 🙏 Acknowledgments

- Inspired by [`bat`](https://github.com/sharkdp/bat) by @sharkdp
- Built with [`syntect`](https://github.com/trishume/syntect) by @trishume
- Community feedback and contributions

---

<div align="center">

**Built with ❤️ for automation, AI assistants, and modern CLI workflows**

[⭐ Star on GitHub](https://github.com/docdyhr/batless) | [📦 Install Now](#-quick-start) | [📖 Read the Docs](docs/)

</div>
