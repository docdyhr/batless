# 🦇 batless

<div align="center">

## The Ultimate Non-Blocking Code Viewer

Built for automation, AI assistants, and modern CLI workflows

[![Crates.io](https://img.shields.io/crates/v/batless?logo=rust&logoColor=white)](https://crates.io/crates/batless)
[![Crates.io Downloads](https://img.shields.io/crates/d/batless?logo=rust&logoColor=white)](https://crates.io/crates/batless)
[![GitHub Downloads](https://img.shields.io/github/downloads/docdyhr/batless/total?logo=github&logoColor=white)](https://github.com/docdyhr/batless/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-green?logo=opensource&logoColor=white)](https://opensource.org/licenses/MIT)
[![GitHub Release](https://img.shields.io/github/v/release/docdyhr/batless?include_prereleases&logo=github&logoColor=white)](https://github.com/docdyhr/batless/releases)

[![CI/CD](https://github.com/docdyhr/batless/actions/workflows/ci-optimized.yml/badge.svg?branch=main)](https://github.com/docdyhr/batless/actions/workflows/ci-optimized.yml)
[![Security](https://github.com/docdyhr/batless/actions/workflows/security.yml/badge.svg?branch=main)](https://github.com/docdyhr/batless/actions/workflows/security.yml)
[![Release](https://github.com/docdyhr/batless/actions/workflows/release-v2.yml/badge.svg?branch=main)](https://github.com/docdyhr/batless/actions/workflows/release-v2.yml)
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

Get up and running in **under 2 minutes**:

### Prerequisites

- **Rust Toolchain**: For building from source (or use pre-built binaries)
- **Terminal**: Any POSIX-compatible shell
- **Files to View**: Any text-based source code files

### 3-Step Setup

#### 1️⃣ Install batless (Choose One)

```bash
# Option A: Pre-built binaries (fastest)
curl -L https://github.com/docdyhr/batless/releases/latest/download/batless-x86_64-unknown-linux-gnu.tar.gz | tar xz

# Option B: Via Cargo
cargo install batless

# Option C: Homebrew (macOS/Linux)
brew tap docdyhr/batless && brew install batless
```

#### 2️⃣ Test Your Installation

```bash
# View a file with syntax highlighting
batless src/main.rs

# Test JSON output mode
batless --mode=json --max-lines=10 src/lib.rs
```

#### 3️⃣ Integrate with Your Workflow

```bash
# CI/CD pipeline usage
batless --mode=summary --max-lines=50 failing-test.rs

# AI assistant context
batless --mode=json --include-tokens --summary src/main.rs

# Machine-readable version metadata
batless --version-json
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

## 🚀 Quick Start

Get up and running in **under 2 minutes**:

### Prerequisites

- **Rust Toolchain**: For building from source (or use pre-built binaries)
- **Terminal**: Any POSIX-compatible shell
- **Files to View**: Any text-based source code files

### 3-Step Setup

#### 1️⃣ Install batless (Choose One)

```bash
# Option A: Pre-built binaries (fastest)
curl -L https://github.com/docdyhr/batless/releases/latest/download/batless-x86_64-unknown-linux-gnu.tar.gz | tar xz

# Option B: Via Cargo
cargo install batless

# Option C: Homebrew (macOS/Linux)
brew tap docdyhr/batless && brew install batless
```

#### 2️⃣ Test Your Installation

```bash
# View a file with syntax highlighting
batless src/main.rs

# Test JSON output mode
batless --mode=json --max-lines=10 src/lib.rs
```

#### 3️⃣ Integrate with Your Workflow

```bash
# CI/CD pipeline usage
batless --mode=summary --max-lines=50 failing-test.rs

# AI assistant context
batless --mode=json --include-tokens --summary src/main.rs

# Machine-readable version metadata
batless --version-json
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

## 🚀 Quick Start

Get up and running in **under 2 minutes**:

### Prerequisites

- **Rust Toolchain**: For building from source (or use pre-built binaries)
- **Terminal**: Any POSIX-compatible shell
- **Files to View**: Any text-based source code files

### 3-Step Setup

#### 1️⃣ Install batless (Choose One)

```bash
# Option A: Pre-built binaries (fastest)
curl -L https://github.com/docdyhr/batless/releases/latest/download/batless-x86_64-unknown-linux-gnu.tar.gz | tar xz

# Option B: Via Cargo
cargo install batless

# Option C: Homebrew (macOS/Linux)
brew tap docdyhr/batless && brew install batless
```

#### 2️⃣ Test Your Installation

```bash
# View a file with syntax highlighting
batless src/main.rs

# Test JSON output mode
batless --mode=json --max-lines=10 src/lib.rs
```

#### 3️⃣ Integrate with Your Workflow

```bash
# CI/CD pipeline usage
batless --mode=summary --max-lines=50 failing-test.rs

# AI assistant context
batless --mode=json --include-tokens --summary src/main.rs

# Machine-readable version metadata
batless --version-json
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

## 🚀 Quick Start

Get up and running in **under 2 minutes**:

### Prerequisites

- **Rust Toolchain**: For building from source (or use pre-built binaries)
- **Terminal**: Any POSIX-compatible shell
- **Files to View**: Any text-based source code files

### 3-Step Setup

#### 1️⃣ Install batless (Choose One)

```bash
# Option A: Pre-built binaries (fastest)
curl -L https://github.com/docdyhr/batless/releases/latest/download/batless-x86_64-unknown-linux-gnu.tar.gz | tar xz

# Option B: Via Cargo
cargo install batless

# Option C: Homebrew (macOS/Linux)
brew tap docdyhr/batless && brew install batless
```

#### 2️⃣ Test Your Installation

```bash
# View a file with syntax highlighting
batless src/main.rs

# Test JSON output mode
batless --mode=json --max-lines=10 src/lib.rs
```

#### 3️⃣ Integrate with Your Workflow

```bash
# CI/CD pipeline usage
batless --mode=summary --max-lines=50 failing-test.rs

# AI assistant context
batless --mode=json --include-tokens --summary src/main.rs

# Machine-readable version metadata
batless --version-json
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

## 🚀 Quick Start

Get up and running in **under 2 minutes**:

### Prerequisites

- **Rust Toolchain**: For building from source (or use pre-built binaries)
- **Terminal**: Any POSIX-compatible shell
- **Files to View**: Any text-based source code files

### 3-Step Setup

#### 1️⃣ Install batless (Choose One)

```bash
# Option A: Pre-built binaries (fastest)
curl -L https://github.com/docdyhr/batless/releases/latest/download/batless-x86_64-unknown-linux-gnu.tar.gz | tar xz

# Option B: Via Cargo
cargo install batless

# Option C: Homebrew (macOS/Linux)
brew tap docdyhr/batless && brew install batless
```

#### 2️⃣ Test Your Installation

```bash
# View a file with syntax highlighting
batless src/main.rs

# Test JSON output mode
batless --mode=json --max-lines=10 src/lib.rs
```

#### 3️⃣ Integrate with Your Workflow

```bash
# CI/CD pipeline usage
batless --mode=summary --max-lines=50 failing-test.rs

# AI assistant context
batless --mode=json --include-tokens --summary src/main.rs

# Machine-readable version metadata
batless --version-json
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

## 🚀 Quick Start

Get up and running in **under 2 minutes**:

### Prerequisites

- **Rust Toolchain**: For building from source (or use pre-built binaries)
- **Terminal**: Any POSIX-compatible shell
- **Files to View**: Any text-based source code files

### 3-Step Setup

#### 1️⃣ Install batless (Choose One)

```bash
# Option A: Pre-built binaries (fastest)
curl -L https://github.com/docdyhr/batless/releases/latest/download/batless-x86_64-unknown-linux-gnu.tar.gz | tar xz

# Option B: Via Cargo
cargo install batless

# Option C: Homebrew (macOS/Linux)
brew tap docdyhr/batless && brew install batless
```

#### 2️⃣ Test Your Installation

```bash
# View a file with syntax highlighting
batless src/main.rs

# Test JSON output mode
batless --mode=json --max-lines=10 src/lib.rs
```

#### 3️⃣ Integrate with Your Workflow

```bash
# CI/CD pipeline usage
batless --mode=summary --max-lines=50 failing-test.rs

# AI assistant context
batless --mode=json --include-tokens --summary src/main.rs

# Machine-readable version metadata
batless --version-json
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

## 🚀 Quick Start

Get up and running in **under 2 minutes**:

### Prerequisites

- **Rust Toolchain**: For building from source (or use pre-built binaries)
- **Terminal**: Any POSIX-compatible shell
- **Files to View**: Any text-based source code files

### 3-Step Setup

#### 1️⃣ Install batless (Choose One)

```bash
# Option A: Pre-built binaries (fastest)
curl -L https://github.com/docdyhr/batless/releases/latest/download/batless-x86_64-unknown-linux-gnu.tar.gz | tar xz

# Option B: Via Cargo
cargo install batless

# Option C: Homebrew (macOS/Linux)
brew tap docdyhr/batless && brew install batless
```

#### 2️⃣ Test Your Installation

```bash
# View a file with syntax highlighting
batless src/main.rs

# Test JSON output mode
batless --mode=json --max-lines=10 src/lib.rs
```

#### 3️⃣ Integrate with Your Workflow

```bash
# CI/CD pipeline usage
batless --mode=summary --max-lines=50 failing-test.rs

# AI assistant context
batless --mode=json --include-tokens --summary src/main.rs

# Machine-readable version metadata
batless --version-json
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

## 🚀 Quick Start

Get up and running in **under 2 minutes**:

### Prerequisites

- **Rust Toolchain**: For building from source (or use pre-built binaries)
- **Terminal**: Any POSIX-compatible shell
- **Files to View**: Any text-based source code files

### 3-Step Setup

#### 1️⃣ Install batless (Choose One)

```bash
# Option A: Pre-built binaries (fastest)
curl -L https://github.com/docdyhr/batless/releases/latest/download/batless-x86_64-unknown-linux-gnu.tar.gz | tar xz

# Option B: Via Cargo
cargo install batless

# Option C: Homebrew (macOS/Linux)
brew tap docdyhr/batless && brew install batless
```

#### 2️⃣ Test Your Installation

```bash
# View a file with syntax highlighting
batless src/main.rs

# Test JSON output mode
batless --mode=json --max-lines=10 src/lib.rs
```

#### 3️⃣ Integrate with Your Workflow

```bash
# CI/CD pipeline usage
batless --mode=summary --max-lines=50 failing-test.rs

# AI assistant context
batless --mode=json --include-tokens --summary src/main.rs

# Machine-readable version metadata
batless --version-json
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

## 🚀 Quick Start

Get up and running in **under 2 minutes**:

### Prerequisites

- **Rust Toolchain**: For building from source (or use pre-built binaries)
- **Terminal**: Any POSIX-compatible shell
- **Files to View**: Any text-based source code files

### 3-Step Setup

#### 1️⃣ Install batless (Choose One)

```bash
# Option A: Pre-built binaries (fastest)
curl -L https://github.com/docdyhr/batless/releases/latest/download/batless-x86_64-unknown-linux-gnu.tar.gz | tar xz

# Option B: Via Cargo
cargo install batless

# Option C: Homebrew (macOS/Linux)
brew tap docdyhr/batless && brew install batless
```

#### 2️⃣ Test Your Installation

```bash
# View a file with syntax highlighting
batless src/main.rs

# Test JSON output mode
batless --mode=json --max-lines=10 src/lib.rs
```

#### 3️⃣ Integrate with Your Workflow

```bash
# CI/CD pipeline usage
batless --mode=summary --max-lines=50 failing-test.rs

# AI assistant context
batless --mode=json --include-tokens --summary src/main.rs

# Machine-readable version metadata
batless --version-json
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

## 🚀 Quick Start

Get up and running in **under 2 minutes**:

### Prerequisites

- **Rust Toolchain**: For building from source (or use pre-built binaries)
- **Terminal**: Any POSIX-compatible shell
- **Files to View**: Any text-based source code files

### 3-Step Setup

#### 1️⃣ Install batless (Choose One)

```bash
# Option A: Pre-built binaries (fastest)
curl -L https://github.com/docdyhr/batless/releases/latest/download/batless-x86_64-unknown-linux-gnu.tar.gz | tar xz

# Option B: Via Cargo
cargo install batless

# Option C: Homebrew (macOS/Linux)
brew tap docdyhr/batless && brew install batless
```

#### 2️⃣ Test Your Installation

```bash
# View a file with syntax highlighting
batless src/main.rs

# Test JSON output mode
batless --mode=json --max-lines=10 src/lib.rs
```

#### 3️⃣ Integrate with Your Workflow

```bash
# CI/CD pipeline usage
batless --mode=summary --max-lines=50 failing-test.rs

# AI assistant context
batless --mode=json --include-tokens --summary src/main.rs

# Machine-readable version metadata
batless --version-json
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

## 🚀 Quick Start

Get up and running in **under 2 minutes**:

### Prerequisites

- **Rust Toolchain**: For building from source (or use pre-built binaries)
- **Terminal**: Any POSIX-compatible shell
- **Files to View**: Any text-based source code files

### 3-Step Setup

#### 1️⃣ Install batless (Choose One)

```bash
# Option A: Pre-built binaries (fastest)
curl -L https://github.com/docdyhr/batless/releases/latest/download/batless-x86_64-unknown-linux-gnu.tar.gz | tar xz

# Option B: Via Cargo
cargo install batless

# Option C: Homebrew (macOS/Linux)
brew tap docdyhr/batless && brew install batless
```

#### 2️⃣ Test Your Installation

```bash
# View a file with syntax highlighting
batless src/main.rs

# Test JSON output mode
batless --mode=json --max-lines=10 src/lib.rs
```

#### 3️⃣ Integrate with Your Workflow

```bash
# CI/CD pipeline usage
batless --mode=summary --max-lines=50 failing-test.rs

# AI assistant context
batless --mode=json --include-tokens --summary src/main.rs

# Machine-readable version metadata
batless --version-json
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

## 🚀 Quick Start

Get up and running in **under 2 minutes**:

### Prerequisites

- **Rust Toolchain**: For building from source (or use pre-built binaries)
- **Terminal**: Any POSIX-compatible shell
- **Files to View**: Any text-based source code files

### 3-Step Setup

#### 1️⃣ Install batless (Choose One)

```bash
# Option A: Pre-built binaries (fastest)
curl -L https://github.com/docdyhr/batless/releases/latest/download/batless-x86_64-unknown-linux-gnu.tar.gz | tar xz

# Option B: Via Cargo
cargo install batless

# Option C: Homebrew (macOS/Linux)
brew tap docdyhr/batless && brew install batless
```

#### 2️⃣ Test Your Installation

```bash
# View a file with syntax highlighting
batless src/main.rs

# Test JSON output mode
batless --mode=json --max-lines=10 src/lib.rs
```

#### 3️⃣ Integrate with Your Workflow

```bash
# CI/CD pipeline usage
batless --mode=summary --max-lines=50 failing-test.rs

# AI assistant context
batless --mode=json --include-tokens --summary src/main.rs

# Machine-readable version metadata
batless --version-json
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

## 🚀 Quick Start

Get up and running in **under 2 minutes**:

### Prerequisites

- **Rust Toolchain**: For building from source (or use pre-built binaries)
- **Terminal**: Any POSIX-compatible shell
- **Files to View**: Any text-based source code files

### 3-Step Setup

#### 1️⃣ Install batless (Choose One)

```bash
# Option A: Pre-built binaries (fastest)
curl -L https://github.com/docdyhr/batless/releases/latest/download/batless-x86_64-unknown-linux-gnu.tar.gz | tar xz

# Option B: Via Cargo
cargo install batless

# Option C: Homebrew (macOS/Linux)
brew tap docdyhr/batless && brew install batless
```

#### 2️⃣ Test Your Installation

```bash
# View a file with syntax highlighting
batless src/main.rs

# Test JSON output mode
batless --mode=json --max-lines=10 src/lib.rs
```

#### 3️⃣ Integrate with Your Workflow

```bash
# CI/CD pipeline usage
batless --mode=summary --max-lines=50 failing-test.rs

# AI assistant context
batless --mode=json --include-tokens --summary src/main.rs

# Machine-readable version metadata
batless --version-json
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

## 🚀 Quick Start

Get up and running in **under 2 minutes**:

### Prerequisites

- **Rust Toolchain**: For building from source (or use pre-built binaries)
- **Terminal**: Any POSIX-compatible shell
- **Files to View**: Any text-based source code files

### 3-Step Setup

#### 1️⃣ Install batless (Choose One)

```bash
# Option A: Pre-built binaries (fastest)
curl -L https://github.com/docdyhr/batless/releases/latest/download/batless-x86_64-unknown-linux-gnu.tar.gz | tar xz

# Option B: Via Cargo
cargo install batless

# Option C: Homebrew (macOS/Linux)
brew tap docdyhr/batless && brew install batless
```

#### 2️⃣ Test Your Installation

```bash
# View a file with syntax highlighting
batless src/main.rs

# Test JSON output mode
batless --mode=json --max-lines=10 src/lib.rs
```

#### 3️⃣ Integrate with Your Workflow

```bash
# CI/CD pipeline usage
batless --mode=summary --max-lines=50 failing-test.rs

# AI assistant context
batless --mode=json --include-tokens --summary src/main.rs

# Machine-readable version metadata
batless --version-json
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

## 🚀 Quick Start

Get up and running in **under 2 minutes**:

### Prerequisites

- **Rust Toolchain**: For building from source (or use pre-built binaries)
- **Terminal**: Any POSIX-compatible shell
- **Files to View**: Any text-based source code files

### 3-Step Setup

#### 1️⃣ Install batless (Choose One)

```bash
# Option A: Pre-built binaries (fastest)
curl -L https://github.com/docdyhr/batless/releases/latest/download/batless-x86_64-unknown-linux-gnu.tar.gz | tar xz

# Option B: Via Cargo
cargo install batless

# Option C: Homebrew (macOS/Linux)
brew tap docdyhr/batless && brew install batless
```

#### 2️⃣ Test Your Installation

```bash
# View a file with syntax highlighting
batless src/main.rs

# Test JSON output mode
batless --mode=json --max-lines=10 src/lib.rs
```

#### 3️⃣ Integrate with Your Workflow

```bash
# CI/CD pipeline usage
batless --mode=summary --max-lines=50 failing-test.rs

# AI assistant context
batless --mode=json --include-tokens --summary src/main.rs

# Machine-readable version metadata
batless --version-json
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

## 🚀 Quick Start

Get up and running in **under 2 minutes**:

### Prerequisites

- **Rust Toolchain**: For building from source (or use pre-built binaries)
- **Terminal**: Any POSIX-compatible shell
- **Files to View**: Any text-based source code files

### 3-Step Setup

#### 1️⃣ Install batless (Choose One)

```bash
# Option A: Pre-built binaries (fastest)
curl -L https://github.com/docdyhr/batless/releases/latest/download/batless-x86_64-unknown-linux-gnu.tar.gz | tar xz

# Option B: Via Cargo
cargo install batless

# Option C: Homebrew (macOS/Linux)
brew tap docdyhr/batless && brew install batless
```

#### 2️⃣ Test Your Installation

```bash
# View a file with syntax highlighting
batless src/main.rs

# Test JSON output mode
batless --mode=json --max-lines=10 src/lib.rs
```

#### 3️⃣ Integrate with Your Workflow

```bash
# CI/CD pipeline usage
batless --mode=summary --max-lines=50 failing-test.rs

# AI assistant context
batless --mode=json --include-tokens --summary src/main.rs

# Machine-readable version metadata
batless --version-json
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

## 🚀 Quick Start

Get up and running in **under 2 minutes**:

### Prerequisites

- **Rust Toolchain**: For building from source (or use pre-built binaries)
- **Terminal**: Any POSIX-compatible shell
- **Files to View**: Any text-based source code files

### 3-Step Setup

#### 1️⃣ Install batless (Choose One)

```bash
# Option A: Pre-built binaries (fastest)
curl -L https://github.com/docdyhr/batless/releases/latest/download/batless-x86_64-unknown-linux-gnu.tar.gz | tar xz

# Option B: Via Cargo
cargo install batless

# Option C: Homebrew (macOS/Linux)
brew tap docdyhr/batless && brew install batless
```

#### 2️⃣ Test Your Installation

```bash
# View a file with syntax highlighting
batless src/main.rs

# Test JSON output mode
batless --mode=json --max-lines=10 src/lib.rs
```

#### 3️⃣ Integrate with Your Workflow

```bash
# CI/CD pipeline usage
batless --mode=summary --max-lines=50 failing-test.rs

# AI assistant context
batless --mode=json --include-tokens --summary src/main.rs

# Machine-readable version metadata
batless --version-json
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

## 🚀 Quick Start

Get up and running in **under 2 minutes**:

### Prerequisites

- **Rust Toolchain**: For building from source (or use pre-built binaries)
- **Terminal**: Any POSIX-compatible shell
- **Files to View**: Any text-based source code files

### 3-Step Setup

#### 1️⃣ Install batless (Choose One)

```bash
# Option A: Pre-built binaries (fastest)
curl -L https://github.com/docdyhr/batless/releases/latest/download/batless-x86_64-unknown-linux-gnu.tar.gz | tar xz

# Option B: Via Cargo
cargo install batless

# Option C: Homebrew (macOS/Linux)
brew tap docdyhr/batless && brew install batless
```

#### 2️⃣ Test Your Installation

```bash
# View a file with syntax highlighting
batless src/main.rs

# Test JSON output mode
batless --mode=json --max-lines=10 src/lib.rs
```

#### 3️⃣ Integrate with Your Workflow

```bash
# CI/CD pipeline usage
batless --mode=summary --max-lines=50 failing-test.rs

# AI assistant context
batless --mode=json --include-tokens --summary src/main.rs

# Machine-readable version metadata
batless --version-json
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

## 🚀 Quick Start

Get up and running in **under 2 minutes**:

### Prerequisites

- **Rust Toolchain**: For building from source (or use pre-built binaries)
- **Terminal**: Any POSIX-compatible shell
- **Files to View**: Any text-based source code files

### 3-Step Setup

#### 1️⃣ Install batless (Choose One)

```bash
# Option A: Pre-built binaries (fastest)
curl -L https://github.com/docdyhr/batless/releases/latest/download/batless-x86_64-unknown-linux-gnu.tar.gz | tar xz

# Option B: Via Cargo
cargo install batless

# Option C: Homebrew (macOS/Linux)
brew tap docdyhr/batless && brew install batless
```

#### 2️⃣ Test Your Installation

```bash
# View a file with syntax highlighting
batless src/main.rs

# Test JSON output mode
batless --mode=json --max-lines=10 src/lib.rs
```

#### 3️⃣ Integrate with Your Workflow

```bash
# CI/CD pipeline usage
batless --mode=summary --max-lines=50 failing-test.rs

# AI assistant context
batless --mode=json --include-tokens --summary src/main.rs

# Machine-readable version metadata
batless --version-json
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

## 🚀 Quick Start

Get up and running in **under 2 minutes**:

### Prerequisites

- **Rust Toolchain**: For building from source (or use pre-built binaries)
- **Terminal**: Any POSIX-compatible shell
- **Files to View**: Any text-based source code files

### 3-Step Setup

#### 1️⃣ Install batless (Choose One)

```bash
# Option A: Pre-built binaries (fastest)
curl -L https://github.com/docdyhr/batless/releases/latest/download/batless-x86_64-unknown-linux-gnu.tar.gz | tar xz

# Option B: Via Cargo
cargo install batless

# Option C: Homebrew (macOS/Linux)
brew tap docdyhr/batless && brew install batless
```

#### 2️⃣ Test Your Installation

```bash
# View a file with syntax highlighting
batless src/main.rs

# Test JSON output mode
batless --mode=json --max-lines=10 src/lib.rs
```

#### 3️⃣ Integrate with Your Workflow

```bash
# CI/CD pipeline usage
batless --mode=summary --max-lines=50 failing-test.rs

# AI assistant context
batless --mode=json --include-tokens --summary src/main.rs

# Machine-readable version metadata
batless --version-json
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

## 🚀 Quick Start

Get up and running in **under 2 minutes**:

### Prerequisites

- **Rust Toolchain**: For building from source (or use pre-built binaries)
- **Terminal**: Any POSIX-compatible shell
- **Files to View**: Any text-based source code files

### 3-Step Setup

#### 1️⃣ Install batless (Choose One)

```bash
# Option A: Pre-built binaries (fastest)
curl -L https://github.com/docdyhr/batless/releases/latest/download/batless-x86_64-unknown-linux-gnu.tar.gz | tar xz

# Option B: Via Cargo
cargo install batless

# Option C: Homebrew (macOS/Linux)
brew tap docdyhr/batless && brew install batless
```

#### 2️⃣ Test Your Installation

```bash
# View a file with syntax highlighting
batless src/main.rs

# Test JSON output mode
batless --mode=json --max-lines=10 src/lib.rs
```

#### 3️⃣ Integrate with Your Workflow

```bash
# CI/CD pipeline usage
batless --mode=summary --max-lines=50 failing-test.rs

# AI assistant context
batless --mode=json --include-tokens --summary src/main.rs

# Machine-readable version metadata
batless --version-json
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
