# 🍺 Homebrew Tap for batless

This is the official Homebrew tap for [batless](https://github.com/docdyhr/batless) - a non-blocking, AI-friendly code viewer inspired by bat.

## Installation

### Install batless

```bash
# Add the tap (one-time setup)
brew tap docdyhr/batless

# Install batless
brew install batless
```

### Direct install (without adding tap)

```bash
brew install docdyhr/batless
```

## What is batless?

**batless** is a minimal, blazing-fast syntax viewer that **never blocks, never pages, never hangs**. Unlike `bat`, it's purpose-built for:

- 🤖 **AI code assistants** that need predictable, streaming output
- 🔄 **CI/CD pipelines** where interactive pagers would hang forever  
- 📜 **Automation scripts** that require guaranteed non-blocking behavior
- 🚀 **Modern workflows** where JSON output and code summaries matter more than line numbers

## Usage Examples

```bash
# View a file with syntax highlighting
batless src/main.rs

# Plain text output (no colors)
batless --mode=plain src/main.rs

# JSON output for parsing
batless --mode=json src/main.rs

# Extract code structure summary
batless --mode=summary src/main.rs

# Limit output
batless --max-lines=50 large-file.py
```

## Features

- ✅ **NEVER blocks** - no pager, no interactive prompts
- 🎨 **Syntax highlighting** for 100+ languages
- 📊 **Multiple output modes**: plain, highlighted, JSON, summary  
- 🤖 **AI-optimized** with metadata and token extraction
- 💾 **Memory efficient** - true streaming architecture
- 📦 **Single binary** with minimal dependencies

## Links

- **Homepage**: https://github.com/docdyhr/batless
- **Documentation**: https://github.com/docdyhr/batless/blob/main/README.md
- **Releases**: https://github.com/docdyhr/batless/releases
- **Issues**: https://github.com/docdyhr/batless/issues

## Support

For issues with the Homebrew formula, please open an issue in this repository.
For issues with batless itself, please use the [main repository](https://github.com/docdyhr/batless/issues).