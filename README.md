<div align="right">

<a href="https://railway.com?referralCode=QhjuBc">

  <img width="160" src="https://raw.githubusercontent.com/docdyhr/.github/main/assets/railway-corner-v2@2x.png" alt="Deploy on Railway — $20 free credits">

</a>

</div>

# 🦇 batless

<div align="center">

## A fast, non-blocking `cat`/`bat` alternative

No pager, no highlighting, no waiting — just fast, predictable file viewing for scripts, pipelines, and terminals alike

[![Crates.io](https://img.shields.io/crates/v/batless?logo=rust&logoColor=white)](https://crates.io/crates/batless)
[![Crates.io Downloads](https://img.shields.io/crates/d/batless?logo=rust&logoColor=white)](https://crates.io/crates/batless)
[![GitHub Downloads](https://img.shields.io/github/downloads/docdyhr/batless/total?logo=github&logoColor=white)](https://github.com/docdyhr/batless/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-green?logo=opensource&logoColor=white)](https://opensource.org/licenses/MIT)
[![GitHub Release](https://img.shields.io/github/v/release/docdyhr/batless?include_prereleases&logo=github&logoColor=white)](https://github.com/docdyhr/batless/releases)

[![CI](https://github.com/docdyhr/batless/actions/workflows/ci-optimized.yml/badge.svg?branch=main)](https://github.com/docdyhr/batless/actions/workflows/ci-optimized.yml)
[![Security](https://github.com/docdyhr/batless/actions/workflows/security.yml/badge.svg?branch=main)](https://github.com/docdyhr/batless/actions/workflows/security.yml)
[![CodeQL](https://github.com/docdyhr/batless/actions/workflows/codeql.yml/badge.svg?branch=main)](https://github.com/docdyhr/batless/actions/workflows/codeql.yml)
[![Fuzz Testing](https://github.com/docdyhr/batless/actions/workflows/fuzz.yml/badge.svg?branch=main)](https://github.com/docdyhr/batless/actions/workflows/fuzz.yml)
[![Health Check](https://github.com/docdyhr/batless/actions/workflows/health-check.yml/badge.svg?branch=main)](https://github.com/docdyhr/batless/actions/workflows/health-check.yml)
[![Codecov](https://codecov.io/gh/docdyhr/batless/branch/main/graph/badge.svg?logo=codecov&logoColor=white)](https://codecov.io/gh/docdyhr/batless)
[![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/docdyhr/batless/badge)](https://securityscorecards.dev/viewer/?uri=github.com/docdyhr/batless)

[![Rust](https://img.shields.io/badge/Rust-100%25-orange?logo=rust&logoColor=white)](https://github.com/docdyhr/batless)
[![Binary Size](https://img.shields.io/badge/binary%20size-%3C2MB-blue?logo=filetype&logoColor=white)](https://github.com/docdyhr/batless)

</div>

## 🎯 Why batless?

`bat` is great, but it uses a pager and syntax highlighting — both of which are wrong for scripts, CI, and automated pipelines. `cat` never blocks, but it's plain-text only with no structure. batless sits between them: it **never blocks**, **never pages**, and adds exactly two things `cat` doesn't have — a `--mode=json` output for scripting and a `--mode=index` symbol table for a quick structural map of a file, without loading or parsing the whole thing yourself.

```bash
# Drop-in cat replacement, never blocks
batless large-file.log

# cat -n / cat -b compatible line numbering
batless -n --plain file.py

# Structured JSON for scripts
batless --mode=json file.py | jq '.total_lines'

# Symbol table — functions, structs, classes, with line ranges
batless --mode=index src/main.rs | jq '.symbols[] | "\(.line_start): \(.kind) \(.name)"'
```

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
# Plain text (default mode — no pager, no highlighting)
batless file.py

# Structured JSON output
batless --mode=json file.py

# Symbol index — functions, structs, classes with line ranges
batless --mode=index src/main.rs

# Multi-file symbol index — walk a directory, one NDJSON line per file
batless --mode=index src/ | jq -c 'select(.symbol_count > 0) | {file, symbol_count}'

# Line numbers (cat -n / cat -b compatible — requires --plain or --mode=plain)
batless -n --plain file.py
batless -b --plain file.py

# Limit output
batless --max-lines=50 large-file.py
batless --max-bytes=10000 huge-file.log

# Get version info as JSON
batless --version-json
```

## 🌟 What Makes batless Different

### 🏆 Feature Comparison

| Feature | `batless` | `bat` | `cat` |
|---------|-----------|-------|-------|
| **Never Blocks** | ✅ Guaranteed | ❌ Uses pager | ✅ |
| **Symbol Index (`--mode=index`)** | ✅ | ❌ | ❌ |
| **JSON Output** | ✅ First-class | ❌ | ❌ |
| **`cat -n`/`cat -b` compatible** | ✅ | ❌ | ✅ |
| **Predictable in scripts/CI** | ✅ Same output every time | ❌ Pager varies by terminal | ✅ |
| **Syntax Highlighting** | ❌ Use `bat` | ✅ Rich | ❌ |
| **Interactive Human Use** | ❌ Not the goal | ✅ | ✅ |

### 🚀 Core Capabilities

#### Non-Blocking Guarantees

- 🚫 **NEVER uses a pager** — no `less`, no `more`, no blocking
- ⚡ **NEVER waits for input** — always streams output immediately
- 🔄 **NEVER hangs in pipes** — safe for `|`, `>`, and subprocess calls
- 📊 **ALWAYS returns quickly** — bounded reads via `--max-lines`/`--max-bytes`

#### Language Support

- 🔍 **Language auto-detection** with manual override (`--language`)
- 🌐 **Universal plain output** — works with any text-based file format
- 🗂️ **Symbol extraction** for Rust, Python, JavaScript, TypeScript, and a broad set of others via a lightweight heuristic extractor (functions, classes, structs, imports)

#### Output Modes

- 📊 **Three output modes**: `plain` (default), `json`, `index`
- 📏 **Smart limiting** by lines (`--max-lines`) and/or bytes (`--max-bytes`)
- 🎯 **Predictable behavior** — identical output in terminal or pipe
- ✂️ **Content stripping** — `--strip-comments`/`--strip-blank-lines` for a denser view of a file
- 📦 **Single &lt;2MB binary** with minimal dependencies (no bundled parser toolchains)

## 🚫 What batless is NOT

**batless** has a focused design philosophy. It intentionally does NOT provide:

### Features We Don't Implement (By Design)

| Feature | Why Not? | Use Instead |
|---------|----------|-------------|
| **Pattern Search** | That's `grep`'s job | `grep -rn "pattern" path/` |
| **Arbitrary Line Ranges** | Beyond our scope | `sed -n '10,50p' file` |
| **File Globbing** | Shell handles this | `batless *.py` (shell expands) |
| **Interactive Paging** | We're non-blocking | Use `bat` or `less` |
| **Syntax Highlighting** | Adds weight for no automation benefit | Use `bat` |
| **Git Integration** | Keep it simple | Use `git diff` or `bat` |
| **File Management** | Not a file browser | `ls`, `find`, `fd` |
| **Text Editing** | Viewer only | Use your editor |

### Common Misconceptions

❌ **"batless is a drop-in replacement for bat"**
✅ **Reality**: batless is purpose-built for scripts, CI, and automation — for interactive human reading, `bat` is the better tool.

❌ **"batless should add grep-like search"**
✅ **Reality**: Unix philosophy — do one thing well. Use `grep` for searching.

❌ **"batless needs more output modes"**
✅ **Reality**: Less is more. `plain`, `json`, and `index` cover what people actually use — usage data backs this up (see below).

### When NOT to Use batless

- 👤 **Interactive code review**: Use `bat` — it has syntax highlighting and paging built for human reading
- 🔍 **Searching code**: Use `grep`, `rg` (ripgrep), or `ag` (silver searcher)
- 📝 **Editing files**: Use your favorite editor
- 📊 **Complex analysis**: Use language-specific tools (pylint, rust-analyzer, etc.)

### Our Philosophy

```text
Do ONE thing well: view files without ever blocking. For everything else —
highlighting, searching, editing, interactive paging — there's already a
better tool. Add features only when real usage justifies the weight.
```

batless's own scope decisions are usage-driven, not guesswork: `--mode=plain` accounts for 84% of real invocations, `--mode=index` for another 10% — everything else in the CLI is either free (line limits, color control) or has demonstrated real use. See [docs/PHILOSOPHY_AND_SCOPE.md](docs/PHILOSOPHY_AND_SCOPE.md) for the full reasoning.

## 📖 Usage Examples

### Basic File Viewing

```bash
# Plain text (default — no pager, no highlighting, no colors unless piped to a terminal)
batless main.rs

# Force no color even in a terminal
batless --color=never main.rs

# With line numbers
batless -n --plain main.rs

# Limit output
batless --max-lines=50 large-file.py
batless --max-bytes=10000 huge-file.log
```

### JSON & Scripting Workflows

```bash
# JSON output for downstream processing
batless --mode=json src/main.rs | jq '.total_lines'

# Pretty-printed JSON
batless --mode=json --json-pretty src/main.rs

# JSON lines as {"n": N, "text": "..."} objects instead of plain strings
batless --mode=json --with-line-numbers src/main.rs

# CI/CD context: capture a bounded slice of a failing test file
batless --mode=json --max-lines=100 failing-test.rs > context.json

# Machine-readable version metadata
batless --version-json
```

### Symbol Index

```bash
# Symbol table for one file
batless --mode=index src/main.rs

# Walk a directory — one compact NDJSON line per file
batless --mode=index src/ | jq -c 'select(.symbol_count > 0) | {file, symbol_count}'

# Find every public function across a Rust crate
find src -name "*.rs" -exec batless --mode=index {} \; \
  | jq -c '.symbols[] | select(.kind=="function" and .visibility=="pub")'
```

### Pipeline Integration

```bash
# Use as PAGER replacement
PAGER="batless --plain" gh pr view 42

# Process multiple files
find src -name "*.rs" -exec batless --mode=index {} \;

# Combine with grep
grep -l "TODO" src/*.py | xargs batless -n --plain

# Stream stdin
cat file.rs | batless --language=rust
```

## 🎨 Configuration

### Language Detection

```bash
# Auto-detect (default)
batless file.txt

# Force specific language
batless --language=python unknown.file

# List supported languages
batless --list-languages
```

### Content Stripping

```bash
# Strip comment-only lines
batless --strip-comments src/main.rs

# Strip blank lines
batless --strip-blank-lines src/main.rs

# Combine both — JSON output includes a compression_ratio field
batless --mode=json --strip-comments --strip-blank-lines src/main.rs | jq '.compression_ratio'
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
batless --generate-completions power-shell | Out-String | Invoke-Expression

# Or save to your profile for persistence
batless --generate-completions power-shell >> $PROFILE
```

## 🔧 CLI Options

### Output Modes

- `--mode <MODE>` — Output mode: `plain` (default), `json`, `index`
- `--plain` — Plain text output (equivalent to `--mode=plain`, also used for PAGER compatibility)
- `--mode=json` — Structured JSON output
- `--mode=index` — Machine-readable symbol table (kind, name, line range, visibility); pass a directory to walk it and emit one NDJSON line per file

### Limiting Output

- `--max-lines <N>` — Limit output to N lines
- `--max-bytes <N>` — Limit output to N bytes

### Display Options

- `-n, --number` — Show line numbers (`cat -n` compatibility; requires `--plain`/`--mode=plain`)
- `-b, --number-nonblank` — Number non-blank lines only (`cat -b` compatibility; requires `--plain`/`--mode=plain`)
- `--language <LANG>` — Force specific language detection
- `--color <MODE>` — Color control: `auto` (default), `always`, `never`
- `--strip-ansi` — Strip ANSI escape codes from output

### JSON Output Options

- `--json-pretty` — Pretty-print JSON output
- `--with-line-numbers` — JSON `lines` array uses `{"n": N, "text": "..."}` objects instead of plain strings
- `--strip-comments` — Strip comment-only lines from output (adds `compression_ratio` to JSON output)
- `--strip-blank-lines` — Strip blank lines from output (adds `compression_ratio` to JSON output)

### JSON Output Fields

When using `--mode=json`, the output includes:

| Field | Type | Description |
|-------|------|-------------|
| `file` | string | File path |
| `language` | string\|null | Detected language |
| `lines` | array | File lines (strings, or `{"n","text"}` objects with `--with-line-numbers`) |
| `mode` | string | `"json"` |
| `processed_lines` | integer | Number of lines actually in `lines` |
| `total_lines` | integer | Line count in original file |
| `total_lines_exact` | boolean | Whether `total_lines` covers the full file |
| `total_bytes` | integer | File size in bytes |
| `truncated` | boolean | Whether output was truncated |
| `truncated_by_lines` | boolean | Whether truncation was due to `--max-lines` |
| `truncated_by_bytes` | boolean | Whether truncation was due to `--max-bytes` |
| `encoding` | string | Detected encoding |
| `syntax_errors` | array | Encoding/processing errors encountered, if any |
| `compression_ratio` | number\|null | original/stripped line ratio (present only with `--strip-comments`/`--strip-blank-lines`) |

When using `--mode=index`, the output includes:

| Field | Type | Description |
|-------|------|-------------|
| `file` | string | File path |
| `language` | string\|null | Detected language |
| `mode` | string | `"index"` |
| `symbol_count` | integer | Number of symbols found |
| `symbols` | array | Symbol table entries |
| `symbols[].kind` | string | `function`, `struct`, `class`, `impl`, `trait`, `import`, etc. |
| `symbols[].name` | string | Symbol identifier name |
| `symbols[].line_start` | integer | 1-based start line |
| `symbols[].line_end` | integer\|null | 1-based end line (when detectable) |
| `symbols[].signature` | string | Declaration line, trimmed |
| `symbols[].visibility` | string\|null | `pub`, `private`, `export`, `local`, depending on language |
| `total_lines` | integer | Line count |
| `total_bytes` | integer | File size in bytes |

### Utility

- `--list-languages` — Show all supported languages
- `--config <PATH>` — Configuration file path (defaults to auto-discovery of `.batlessrc`/`batless.toml`)
- `--debug` — Enable debug mode with detailed processing information
- `--generate-completions <SHELL>` — Generate shell completions (`bash`, `zsh`, `fish`, `power-shell`)
- `--version` — Show version information
- `--version-json` — Machine-readable version metadata
- `--help` — Show detailed help information

## 🏗️ Architecture

batless is built with:

- **Rust** — memory safety and performance
- **A small, focused dependency set** — clap for argument parsing, serde/serde_json for JSON, encoding_rs for encoding detection; no bundled parser toolchains
- **Bounded reads** — `--max-lines`/`--max-bytes` cap memory use on large files
- **Modular design** — clean separation between config parsing, file processing, and output formatting

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
- **Binary size**: <2MB stripped (1.5MB measured on macOS arm64, down from 8.0MB before the v0.7.0 scope reduction)
- **Memory usage**: Bounded by `--max-lines`/`--max-bytes`
- **Throughput**: Limited only by disk I/O

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
- Community feedback and contributions

---

<div align="center">

**Built for scripts, CI, and pipelines that can't afford to block**

[⭐ Star on GitHub](https://github.com/docdyhr/batless) | [📦 Install Now](#-quick-start) | [📖 Read the Docs](docs/)

</div>
