# 🍺 Homebrew Tap for batless

This is the official Homebrew tap for [batless](https://github.com/docdyhr/batless) - a fast, non-blocking code and text viewer inspired by bat.

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

**batless** is a minimal, blazing-fast file viewer that **never blocks, never pages, never hangs**. Unlike `bat`, it's purpose-built for:

- 🔄 **CI/CD pipelines** where interactive pagers would hang forever
- 📜 **Automation scripts** that require guaranteed non-blocking behavior
- 🚀 **Scripting workflows** where JSON output and a symbol index matter more than syntax highlighting

## Usage Examples

```bash
# Plain text output (default — no pager, no highlighting, no colors unless piped to a terminal)
batless src/main.rs

# JSON output for parsing
batless --mode=json src/main.rs

# Machine-readable symbol table (functions, classes, structs)
batless --mode=index src/main.rs

# Limit output
batless --max-lines=50 large-file.py
```

## Features

- ✅ **NEVER blocks** - no pager, no interactive prompts
- 📊 **Three output modes**: plain (default), JSON, index
- 🗂️ **Symbol index** for a quick structural map of a file
- 💾 **Memory efficient** - bounded reads via `--max-lines`/`--max-bytes`
- 📦 **Single binary under 2MB** with minimal dependencies

## Links

- **Homepage**: <https://github.com/docdyhr/batless>
- **Documentation**: <https://github.com/docdyhr/batless/blob/main/README.md>
- **Releases**: <https://github.com/docdyhr/batless/releases>
- **Issues**: <https://github.com/docdyhr/batless/issues>

## Support

For issues with the Homebrew formula, please open an issue in this repository.
For issues with batless itself, please use the [main repository](https://github.com/docdyhr/batless/issues).
