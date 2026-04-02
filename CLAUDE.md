# batless Protocol for AI Assistants

## Overview

batless is a non-blocking code viewer designed for automation and AI workflows. Unlike `bat`, it NEVER uses a pager and NEVER blocks waiting for user input.

## Core Commands

### Basic File Viewing

```bash
# View file with syntax highlighting
batless file.py

# Plain text output (no colors)
batless --mode=plain file.py
batless --plain file.py  # short form

# JSON output for structured processing
batless --mode=json file.py

# Summary mode (extracts key code structures)
batless --mode=summary file.py

# Symbol index mode (machine-readable symbol table) [v0.5.0]
batless --mode=index file.py
```

### Output Limiting

```bash
# Limit output by lines
batless --max-lines=50 file.py

# Limit output by bytes
batless --max-bytes=1024 file.py

# Combine limits (both apply)
batless --max-lines=100 --max-bytes=5000 file.py
```

### AI-Optimized Features

```bash
# Use built-in AI profiles
batless --profile=claude file.py      # Claude-optimized: 20K lines, JSON, standard summary
batless --profile=claude-max file.py  # Claude large context: 150K lines, JSON, no summary [v0.5.0]
batless --profile=copilot file.py     # GitHub Copilot
batless --profile=chatgpt file.py     # ChatGPT

# Include extracted identifiers in JSON [--include-tokens is a deprecated alias]
batless --mode=json --include-identifiers file.py

# LLM token estimate in JSON output (requires profile or --ai-model) [v0.5.0]
batless --mode=json --profile=claude file.py  # adds estimated_llm_tokens + token_model

# Get both summary and full content
batless --mode=json --summary file.py

# Line-numbered JSON lines: {"n": 1, "text": "..."} instead of plain strings [v0.5.0]
batless --mode=json --with-line-numbers file.py

# SHA-256 file hash in JSON output [v0.5.0]
batless --mode=json --hash file.py
```

### Content Stripping (v0.5.0)

```bash
# Remove comment-only lines (language-aware: //, #, --, %, ;, /* */)
batless --strip-comments file.py

# Remove blank/whitespace-only lines
batless --strip-blank-lines file.py

# Combine for maximum compression — adds compression_ratio to JSON output
batless --mode=json --strip-comments --strip-blank-lines file.py
```

### Streaming

```bash
# Stream file in chunks (output is pure NDJSON — one JSON object per line) [v0.5.0]
batless --mode=json --streaming file.py

# Semantic chunking: chunks extend to tree-sitter declaration boundaries [v0.5.0]
# Supported: Rust, Python, JavaScript, TypeScript (line-based fallback for others)
batless --mode=json --streaming --chunk-strategy=semantic file.py
```

### Language & Themes

```bash
# Force specific language
batless --language=python unknown.file

# Change color theme
batless --theme="Solarized (dark)" file.py

# List available options
batless --list-languages
batless --list-themes
```

### Line Numbers (Cat Compatibility)

**IMPORTANT**: Line numbers only work in plain mode (no syntax highlighting).

```bash
# Show line numbers (like cat -n) - REQUIRES --plain or --mode=plain
batless -n --plain file.py
batless --number --mode=plain file.py

# Number non-blank lines only (like cat -b) - REQUIRES --plain or --mode=plain
batless -b --plain file.py
batless --number-nonblank --mode=plain file.py

# WRONG: These will NOT show line numbers (syntax highlighting ignores line numbers)
# batless -n file.py              # No line numbers shown
# batless --number file.py         # No line numbers shown
```

### Pipeline & PAGER Usage

```bash
# Use as PAGER replacement
PAGER="batless --plain" gh pr view 42

# Pipeline input
echo "code" | batless --language=python
cat file.py | batless --mode=summary

# Compatible flags (ignored for compatibility)
batless --plain --unbuffered file.py
```

## What batless DOES NOT Do

batless intentionally does NOT provide these features. Use the suggested alternatives:

### Pattern Search

```bash
# WRONG: batless does not search
# batless --pattern "TODO" src/

# CORRECT: Use grep or ripgrep
grep -rn "TODO" src/
rg "TODO" src/
```

### Line Range Selection

```bash
# WRONG: batless does not support arbitrary ranges
# batless -r 10:50 file.py
# batless --range 10:50 file.py

# CORRECT: Use sed, head/tail, or combine with batless
sed -n '10,50p' file.py | batless --language=python
head -50 file.py | tail -41 | batless --mode=plain
```

### File Globbing/Listing

```bash
# WRONG: batless does not list files
# batless --list *.py

# CORRECT: Use shell expansion or find
batless *.py  # Shell expands the glob
find . -name "*.py" -exec batless {} \;
fd -e py -x batless {}
```

### Interactive Features

```bash
# batless NEVER provides:
# - Interactive paging (no less/more behavior)
# - User prompts or confirmations
# - Terminal UI elements
# - Git integration (diffs, blame, etc.)
```

## JSON Output Schema

Standard `--mode=json` output:

```json
{
  "file": "path/to/file.py",
  "language": "Python",
  "lines": ["line1", "line2"],
  "summary_lines": [
    {"line": "def main():", "line_number": 10, "end_line": 25, "kind": "function"}
  ],
  "identifiers": ["def", "main", "os"],
  "identifier_total": 42,
  "estimated_llm_tokens": 1200,
  "token_model": "claude",
  "file_hash": "a3f1...",
  "compression_ratio": 1.35,
  "total_lines": 150,
  "total_lines_exact": true,
  "total_bytes": 3420,
  "truncated": false,
  "truncated_by_lines": false,
  "truncated_by_bytes": false,
  "truncated_by_context": false,
  "tokens_truncated": false,
  "encoding": "UTF-8",
  "syntax_errors": [],
  "mode": "json"
}
```

**Field notes:**
- `lines` — plain strings by default; `{"n": N, "text": "..."}` objects when `--with-line-numbers` is used
- `summary_lines` — present when `--summary` is active; each entry has `line_number`, `end_line`, `kind`
- `identifiers` / `identifier_total` — present when `--include-identifiers` is used (`--include-tokens` is a deprecated alias)
- `estimated_llm_tokens` / `token_model` — present when a `--profile` or `--ai-model` is active
- `file_hash` — present when `--hash` is used
- `compression_ratio` — present when `--strip-comments` or `--strip-blank-lines` is used

## Index Mode Schema (v0.5.0)

`--mode=index` emits a symbol table instead of file content. Backed by tree-sitter AST for Rust, Python, JavaScript, TypeScript; regex fallback for all other languages.

```json
{
  "file": "src/main.rs",
  "language": "Rust",
  "symbol_count": 3,
  "symbols": [
    {
      "kind": "function",
      "name": "main",
      "line_start": 10,
      "line_end": 25,
      "signature": "fn main()",
      "visibility": "public"
    },
    {
      "kind": "struct",
      "name": "Config",
      "line_start": 30,
      "line_end": 40,
      "signature": "pub struct Config",
      "visibility": "public"
    }
  ]
}
```

Use index mode when you need to navigate code structure without loading full file content.

## Error Handling

When batless is not available, use these fallback commands:

```bash
# Fallback for syntax highlighting
cat file.py  # or less -R for colors if available

# Fallback for line numbers
cat -n file.py

# Fallback for limiting output
head -50 file.py

# Fallback for JSON structure (basic)
echo "{\"file\": \"$1\", \"content\": \"$(cat $1 | jq -Rs .)\"}"
```

## Integration Examples

### Viewing Large Log Files

```bash
# View the first 1000 lines of a large log file
batless --max-lines=1000 application.log

# View specific size limit (useful for very large files)
batless --max-bytes=1048576 huge-log.log  # 1MB limit

# Combine with grep for error analysis
grep -n "ERROR" application.log | head -100 | batless --language=log

# Preview log file with line numbers for debugging (requires --plain)
batless -n --plain --max-lines=500 server.log
```

### Extracting JSON for AI Processing

```bash
# Extract code structure as JSON for LLM analysis
batless --mode=json --include-identifiers --max-lines=500 src/main.py | \
  jq '{file, language, summary: .summary_lines, identifiers: .identifier_total}'

# Get LLM token estimate before sending to API
batless --mode=json --profile=claude src/main.py | jq '.estimated_llm_tokens'

# Get function signatures only
batless --mode=summary src/lib.rs | grep "^fn "

# Build context for AI code review
batless --mode=json --summary --max-lines=300 complex.py > ai-context.json

# Extract multiple files for AI analysis
find src -name "*.py" -exec sh -c \
  'batless --mode=json --summary "$1" | jq -c .' _ {} \; > codebase-context.jsonl
```

### Symbol Navigation with Index Mode (v0.5.0)

```bash
# Get all symbols in a file
batless --mode=index src/main.rs | jq '.symbols[] | "\(.line_start): \(.kind) \(.name)"'

# Find all public functions across a codebase
find src -name "*.rs" -exec sh -c \
  'batless --mode=index "$1" | jq -c ".symbols[] | select(.kind==\"function\" and .visibility==\"public\")"' \
  _ {} \; | jq -s '.'

# Jump to a specific symbol's line range
batless --mode=index src/lib.py | jq '.symbols[] | select(.name=="MyClass")'
```

### Compressed AI Context (v0.5.0)

```bash
# Strip comments and blanks for token-efficient context
batless --mode=json --strip-comments --strip-blank-lines src/main.rs | \
  jq '{file, compression_ratio, lines: (.lines | length)}'

# Compare compressed vs original size
batless --mode=json --strip-comments --strip-blank-lines --profile=claude file.py | \
  jq '{estimated_llm_tokens, compression_ratio}'
```

### CI/CD Pipeline Usage

```yaml
# GitHub Actions - Show test failure context
- name: Show failing test code
  if: failure()
  run: batless --mode=summary --max-lines=100 tests/test_auth.py

# GitLab CI - Extract code summary for artifacts
analyze-code:
  script:
    - batless --mode=json --summary src/main.rs > code-analysis.json
  artifacts:
    paths:
      - code-analysis.json

# Jenkins - Generate code context for debugging
stage('Debug Context') {
  steps {
    sh 'batless --mode=summary --max-lines=200 src/failing-module.py'
  }
}

# CircleCI - Non-blocking code preview
- run:
    name: Show changed files
    command: |
      for file in $(git diff --name-only HEAD~1); do
        echo "=== $file ==="
        batless --max-lines=50 "$file"
      done
```

### Summary Mode for Code Review

```bash
# Quick overview of changed files
for file in $(git diff --name-only main...HEAD); do
  echo "=== Changes in $file ==="
  batless --mode=summary "$file"
  echo ""
done

# Extract function signatures for review
batless --mode=summary src/api.py | grep -E "^(def|class) "

# Compare code structure before and after
git show HEAD~1:src/main.rs | batless --mode=summary --language=rust > before.txt
batless --mode=summary src/main.rs > after.txt
diff -u before.txt after.txt

# Generate review checklist from imports
batless --mode=summary src/*.py | grep "^import " | sort -u
```

### AI Context Building

```bash
# Get structured data for AI processing
batless --mode=json --include-identifiers --max-lines=500 complex.py | \
  jq '{file, language, summary_lines, total_lines}'

# Build multi-file context for LLM
echo '{"files": [' > context.json
find src -name "*.rs" | while read file; do
  batless --mode=json --summary --max-lines=200 "$file" | jq -c '.'
  echo ","
done >> context.json
echo ']}' >> context.json

# Extract API endpoints for documentation
batless --mode=summary src/routes/*.py | grep -E "@app\.(get|post|put|delete)"
```

### Automated Code Review

```bash
# Extract code structure for analysis
for file in $(git diff --name-only); do
  batless --mode=summary --max-lines=50 "$file"
done

# Analyze complexity by counting identifiers
for file in src/*.py; do
  count=$(batless --mode=json --include-identifiers "$file" | jq '.identifier_total')
  echo "$file: $count identifiers"
done | sort -t: -k2 -n

# Check for TODO comments across codebase (use grep, not batless)
find . -name "*.rs" -exec grep -Hn "TODO\|FIXME\|XXX" {} \;

# Alternative: combine with batless for context
find . -name "*.rs" | while read file; do
  if grep -q "TODO\|FIXME\|XXX" "$file"; then
    echo "=== $file ==="
    batless --plain "$file" | grep -n "TODO\|FIXME\|XXX"
  fi
done
```

## Version Information

```bash
# Human-readable version
batless --version

# Machine-readable version (JSON)
batless --version-json
```

## Philosophy

batless is designed to be:

- **Non-blocking**: Never waits for user input
- **Predictable**: Same output whether in terminal or pipe
- **Minimal**: No decorations by default (unless explicitly requested)
- **Streaming**: Memory-efficient for large files
- **Automation-first**: Built for scripts and AI, not interactive use
