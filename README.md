# 🦇 batless

<div align="center">

## Machine-Readable Code Analysis for AI and Automation

Symbol indexes, token-estimated context, semantic chunks — structured output that AI assistants can't produce themselves

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

AI assistants like Claude Code have native tools for reading files, searching, and listing directories. What they **don't** have is structured analysis output:

```bash
# Symbol index — navigate code without loading full content
batless --mode=index src/main.rs | jq '.symbols[] | "\(.line_start): \(.kind) \(.name)"'

# Token estimation — gate context decisions before loading a file
batless --mode=json --profile=claude file.py | jq '.estimated_llm_tokens'

# Compressed context — language-aware comment and blank stripping
batless --mode=json --profile=claude --strip-comments --strip-blank-lines file.py

# Semantic chunks — split large files at declaration boundaries
batless --mode=json --streaming --chunk-strategy=semantic large_file.rs

# Content hash — detect changes without loading content
batless --mode=json --hash file.rs | jq '.file_hash'
```

These are the outputs batless is built for. For plain file viewing, use `cat`, `bat`, or your editor.

**Core guarantee**: batless will NEVER wait for user input or block your pipeline.

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
# Symbol index — structure without loading full content
batless --mode=index src/main.rs

# Token estimation — check size before loading into AI context
batless --mode=json --profile=claude file.py | jq '.estimated_llm_tokens'

# Compressed AI context
batless --mode=json --profile=claude --strip-comments --strip-blank-lines src/lib.rs

# Semantic streaming chunks for large files
batless --mode=json --streaming --chunk-strategy=semantic large_file.rs

# Plain text (for piping to other tools)
batless --plain file.py

# Get version info as JSON
batless --version-json
```

## 🌟 What Makes batless Special

### 🏆 Feature Comparison

| Feature | `batless` | `bat` | `cat` / built-in Read |
|---------|-----------|-------|-------|
| **Never Blocks** | ✅ Guaranteed | ❌ Uses pager | ✅ |
| **Symbol Index (`--mode=index`)** | ✅ AST-backed | ❌ | ❌ |
| **LLM Token Estimation** | ✅ Per-profile | ❌ | ❌ |
| **Semantic Chunking** | ✅ tree-sitter | ❌ | ❌ |
| **Comment/Blank Stripping** | ✅ Language-aware | ❌ | ❌ |
| **Content Hash** | ✅ SHA-256 | ❌ | ❌ |
| **JSON Output** | ✅ First-class | ❌ | ❌ |
| **Syntax Highlighting** | ✅ (deprecated in v0.6) | ✅ Rich | ❌ |
| **Interactive Human Use** | ❌ Not the goal | ✅ | ✅ |

### 🚀 Core Capabilities

#### Non-Blocking Guarantees

- 🚫 **NEVER uses a pager** - no `less`, no `more`, no blocking
- ⚡ **NEVER waits for input** - always streams output immediately
- 🔄 **NEVER hangs in pipes** - safe for `|`, `>`, and subprocess calls
- 📊 **ALWAYS returns quickly** - even on huge files (streaming architecture)

#### Language Support

- 🔍 **Language auto-detection** with manual override (`--language`)
- 🌳 **AST-backed analysis** for Rust, Python, JavaScript, TypeScript (regex fallback for others)
- 🌐 **Universal plain output** — works with any text-based file format

#### Smart Output Modes

- 📊 **Multiple output modes**: plain, highlighted, JSON, summary
- 📏 **Smart limiting** by lines (`--max-lines`) and/or bytes (`--max-bytes`)
- 💾 **Memory efficient** - true streaming, never loads full files
- 🎯 **Predictable behavior** - same output in terminal or pipe
- 🧠 **Dual-view summaries** - `lines` always retains the full file while `summary_lines` carries the condensed view
- 🔢 **Token-aware JSON** - `token_count` reflects the full file even when the sampled `tokens` array is capped (~2K entries) and `tokens_truncated` tells you when sampling kicked in

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
Do ONE thing well: produce structured, machine-readable code analysis that
AI assistants can't generate themselves. For everything else — plain viewing,
searching, interactive use — there's already a better tool.
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

> **JSON structure tips:** `lines` always contains the full file content (even when `--summary` is enabled), while `summary_lines` carries the condensed view. The payload now exposes `total_lines_exact`, `token_count`, and `tokens_truncated` so downstream tools can distinguish between fully processed files and sampled metadata.

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

- `--mode <MODE>` - Output mode: `plain`, `highlight`, `json`, `summary`, `index`
- `--plain` - Plain text output (equivalent to `--mode=plain`)
- `--mode=json` - Structured JSON output for automation
- `--mode=summary` - Extract only key code structures
- `--mode=index` - Machine-readable symbol table (kind, name, line ranges, visibility)

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

- `--include-identifiers` - Include extracted code identifiers in JSON output (`--include-tokens` still works as alias)
- `--with-line-numbers` - JSON `lines` array uses `{"n": N, "text": "..."}` objects instead of plain strings
- `--hash` - Include SHA-256 content hash in JSON output (for change detection)
- `--strip-comments` - Strip comment-only lines from output
- `--strip-blank-lines` - Strip blank lines from output
- `--chunk-strategy <STRATEGY>` - Streaming chunk strategy: `line` (default) or `semantic` (splits at top-level declaration boundaries for Rust/Python/JS/TS)
- `--summary` - Add code summary to JSON output
- `--profile <PROFILE>` - Use AI-optimized profile (`claude` 20K lines, `claude-max` 150K lines, `copilot`, `chatgpt`, `gemini`, `assistant`)
- `--custom-profile <PATH>` - Load custom profile from file

### JSON Output Fields

When using `--mode=json`, the output includes:

| Field | Type | Description |
|-------|------|-------------|
| `file` | string | File path |
| `language` | string\|null | Detected language |
| `lines` | array | File lines (strings, or `{"n","text"}` objects with `--with-line-numbers`) |
| `total_lines` | integer | Line count in original file |
| `total_lines_exact` | boolean | Whether `total_lines` covers the full file |
| `total_bytes` | integer | File size in bytes |
| `truncated` | boolean | Whether output was truncated |
| `encoding` | string | Detected encoding |
| `summary_lines` | array\|null | Summary items `{line, line_number, end_line, kind}` |
| `identifiers` | array\|null | Extracted code identifiers (with `--include-identifiers`) |
| `identifier_total` | integer\|null | Total identifier count |
| `file_hash` | string\|null | SHA-256 hex digest (with `--hash`) |
| `estimated_llm_tokens` | integer\|null | Heuristic LLM token estimate (when profile active) |
| `token_model` | string\|null | Model used for token estimation |
| `compression_ratio` | number\|null | original/stripped lines ratio (with `--strip-*` flags) |

When using `--mode=index`, the output includes:

| Field | Type | Description |
|-------|------|-------------|
| `file` | string | File path |
| `language` | string\|null | Detected language |
| `symbol_count` | integer | Number of symbols found |
| `symbols` | array | Symbol table entries |
| `symbols[].kind` | string | `function`, `struct`, `class`, `impl`, `trait`, etc. |
| `symbols[].name` | string | Symbol identifier name |
| `symbols[].line_start` | integer | 1-based start line |
| `symbols[].line_end` | integer\|null | 1-based end line |
| `symbols[].signature` | string | First declaration line |
| `symbols[].visibility` | string\|null | `pub`, `private`, `export`, `local` |

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
