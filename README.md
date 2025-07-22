# 🦇 batless

> A minimal, blazing-fast syntax viewer for AI code assistants and modern CLI workflows.

[![Crates.io](https://img.shields.io/crates/v/batless.svg)](https://crates.io/crates/batless)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**batless** is a non-blocking, AI-friendly code viewer inspired by [`bat`](https://github.com/sharkdp/bat) but designed specifically for:
- 🤖 **AI code assistants** (Claude, Gemini, etc.)
- 🔄 **CI/CD pipelines** and automation
- 📜 **Non-interactive workflows**

Unlike traditional pagers that can block or hang, `batless` guarantees streaming, predictable output perfect for automated tools and AI agents.

## ✨ Features

### Core Features
- ⚡ **Always non-blocking** - never hangs or waits for user input
- 🎨 **Syntax highlighting** with 100+ languages via Tree-sitter
- 📊 **Multiple output modes**: plain text, highlighted, JSON
- 🔍 **Language auto-detection** or explicit specification
- 📏 **Smart limiting** by lines or bytes
- 🎯 **Memory efficient** streaming for large files

### AI & Automation Friendly
- 🤖 **LLM-safe defaults** - no decorations, clean output
- 📋 **Enhanced JSON mode** with encoding, tokens, and metadata
- 🎯 **Summary mode** - extract only important code structures
- 🔤 **Token extraction** for AI processing and analysis
- 🚫 **ANSI stripping** support
- 🎨 **Color control** (auto/always/never)
- 📦 **Single binary** with no external dependencies
- 🚀 **Performance optimized** with cached syntax sets

## 🚀 Installation

### From Crates.io
```bash
cargo install batless
```

### From Source
```bash
git clone https://github.com/your-username/batless.git
cd batless
cargo build --release
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

## 🆚 Comparison with `bat`

| Feature | `batless` | `bat` |
|---------|-----------|-------|
| **Blocking behavior** | ✅ Never blocks | ❌ Can block on `less` |
| **AI-friendly** | ✅ Designed for it | ⚠️ Manual config needed |
| **Enhanced JSON output** | ✅ Built-in with metadata | ❌ No |
| **Summary mode** | ✅ Code structure extraction | ❌ No |
| **Token extraction** | ✅ For AI processing | ❌ No |
| **Byte limiting** | ✅ Yes | ❌ No |
| **Performance** | ✅ Cached, optimized | ⚠️ Slower startup |
| **CI/CD safe** | ✅ Always | ⚠️ Needs `--paging=never` |
| **Git integration** | ❌ No | ✅ Yes |
| **Line numbers** | ❌ No (by design) | ✅ Yes |
| **File headers** | ❌ No (by design) | ✅ Yes |

## 🛠️ Development

### Running Tests
```bash
cargo test
```

### Building
```bash
cargo build --release
```

### Linting
```bash
cargo clippy -- -D warnings
cargo fmt --all -- --check
```

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

### Development Setup
```bash
git clone https://github.com/your-username/batless.git
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

## 🔗 Links

- [Crates.io](https://crates.io/crates/batless)
- [Documentation](https://docs.rs/batless)
- [Repository](https://github.com/your-username/batless)
- [Issues](https://github.com/your-username/batless/issues)

---

**Made with ❤️ for AI assistants and modern CLI workflows**