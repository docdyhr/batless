# batless Protocol for AI Assistants

## Overview

batless is a fast, non-blocking `cat`/`bat` alternative. Unlike `bat`, it never uses a pager and never applies syntax highlighting — both are wrong for scripts, CI, and automated pipelines. Unlike plain `cat`, it can emit structured JSON and a symbol index on request. It NEVER blocks waiting for user input and NEVER pages.

This project went through a deliberate simplification ("Option A"): usage telemetry showed plain-text output accounted for the overwhelming majority of real invocations, `--mode=index` was a distant second, and the rest of the old "AI-native" surface (AI profiles, token estimation, AST/tree-sitter mode, streaming/chunking, content hashing, JSON schema validation, summary mode) was barely or never used. All of that has been removed from the codebase. This document describes only what the CLI actually does today.

## AI Assistant Integration: When to Use batless

If you are an AI assistant (Claude Code, Copilot, etc.) with built-in file tools (`Read`, `Grep`, `Glob`), **use those for routine file operations** — they are faster and require no shell invocation. batless earns its keep for a few specific things those tools can't do directly:

| Use batless for | Command |
|---|---|
| Symbol/structure index without loading full file content | `batless --mode=index file` |
| Machine-readable JSON for scripting/piping into `jq` | `batless --mode=json file` |
| Byte/line-capped preview of a large file | `batless --max-lines=200 --max-bytes=8192 file` |
| Compressed context (strip comment-only and blank lines) | `batless --strip-comments --strip-blank-lines file` |
| Project-wide symbol index over a directory (NDJSON) | `batless --mode=index src/` |

**For everything else** — reading files, searching content, listing files — use your built-in tools.

---

## Core Commands

### Basic File Viewing

```bash
# View file (plain text, no color, no pager) — this is the default mode
batless file.py

# Explicit plain mode (identical to the default; useful when overriding --mode)
batless --mode=plain file.py
batless --plain file.py   # PAGER-compatibility alias for --mode=plain

# JSON output for structured processing
batless --mode=json file.py

# Symbol index mode (machine-readable symbol table)
batless --mode=index file.py
```

There is no highlighted/colorized terminal output mode — batless intentionally does not do syntax highlighting (see Overview). `--mode` accepts exactly three values: `plain`, `json`, `index`.

### Output Limiting

```bash
# Limit output by lines
batless --max-lines=50 file.py

# Limit output by bytes
batless --max-bytes=1024 file.py

# Combine limits (both apply; whichever is hit first truncates)
batless --max-lines=100 --max-bytes=5000 file.py
```

In plain mode, a truncated read prints a trailing `// Output truncated after N lines` / `// Output truncated after N bytes` marker. JSON mode instead sets `truncated`, `truncated_by_lines`, and `truncated_by_bytes` booleans (no marker text is printed).

### Line Numbers (Cat Compatibility)

Line numbers only render in plain mode. Since plain is already the **default** mode, `-n`/`-b` work with no other flags — you only need to add `--plain`/`--mode=plain` if you've also passed `--mode=json` or `--mode=index` and want to switch back. In json/index mode, `-n`/`-b` are silently ignored.

```bash
# Show line numbers (like cat -n)
batless -n file.py
batless --number file.py

# Number non-blank lines only (like cat -b)
batless -b file.py
batless --number-nonblank file.py

# Explicit form if you're overriding a different --mode back to plain
batless -n --plain file.py
```

### Language & Listing

```bash
# Force a specific language for detection/labeling purposes
# (populates the "language" field in JSON/index output and drives the
# per-language heuristics used by --mode=index and --strip-comments;
# it does not enable any colorized output)
batless --language=python unknown.file

# List all supported language names
batless --list-languages
```

### Content Stripping

```bash
# Remove comment-only lines (language-aware: //, #, --, %, ;, /* */)
batless --strip-comments file.py

# Remove blank/whitespace-only lines
batless --strip-blank-lines file.py

# Combine for maximum compression — adds compression_ratio to JSON output
batless --mode=json --strip-comments --strip-blank-lines file.py
```

`compression_ratio` = original line count / stripped line count (higher means more was removed). It only appears in JSON output when at least one strip flag is used and produced a non-empty result.

### Pipeline, PAGER & Misc Flags

```bash
# Use as PAGER replacement
PAGER="batless --plain" gh pr view 42

# Pipeline / stdin input (batless reads stdin when no file arg is given
# and stdin is not a terminal)
echo "code" | batless --language=python
cat file.py | batless --mode=json

# Shell completions
batless --generate-completions bash   # also: zsh, fish, power-shell

# Explicit config file (.toml or .json, detected by extension); otherwise
# batless auto-discovers config the normal way
batless --config path/to/batless.toml file.py

# Diagnostics to stderr
batless --debug file.py

# Color/ANSI compatibility flags — batless does not add color/highlighting
# to its own output; these exist for cat/bat command-line compatibility and
# for content that already contains ANSI codes
batless --color=never file.py    # auto (default) | always | never
batless --strip-ansi file.py

# Compatibility flags accepted but ignored (present for drop-in cat/pager use)
batless --unbuffered file.py     # -u
batless --no-title file.py
```

## What batless DOES NOT Do

batless intentionally does not provide these features. Use the suggested alternatives — running `batless --pattern`, `--list`, or `--range`/`-r` exits with an error and prints the same guidance shown here.

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

# CORRECT: Use sed, head/tail, or combine with batless's own limiting
sed -n '10,50p' file.py | batless
head -50 file.py | tail -41 | batless
batless --max-lines=100 file.py
```

### File Globbing/Listing

```bash
# WRONG: batless does not list files
# batless --list *.py

# CORRECT: Use shell expansion or find/fd
batless *.py  # shell expands the glob
find . -name "*.py" -exec batless {} \;
fd -e py -x batless {}
```

### Interactive Features

batless NEVER provides interactive paging (no less/more behavior), user prompts or confirmations, terminal UI elements, or git integration (diffs, blame, etc.).

## JSON Output Schema

`--mode=json` output (verified shape, add `--json-pretty` for pretty-printing):

```json
{
  "file": "path/to/file.py",
  "language": "Python",
  "lines": ["line1", "line2"],
  "processed_lines": 2,
  "total_lines": 2,
  "total_lines_exact": true,
  "total_bytes": 30,
  "truncated": false,
  "truncated_by_lines": false,
  "truncated_by_bytes": false,
  "encoding": "UTF-8",
  "syntax_errors": [],
  "mode": "json"
}
```

**Field notes:**
- `lines` — plain strings by default; `{"n": N, "text": "..."}` objects when `--with-line-numbers` is used
- `compression_ratio` — only present when `--strip-comments`/`--strip-blank-lines` produced output (see Content Stripping above)
- `syntax_errors` — currently always an empty array; reserved field, no syntax validation is performed
- There is no `identifiers`, `estimated_llm_tokens`, `token_model`, `file_hash`, or `summary_lines` field — all of that was removed in the Option A refactor

## Index Mode Schema

`--mode=index` emits a symbol table instead of file content, extracted via regex/heuristic pattern matching in `src/summarizer.rs` (NOT tree-sitter/AST-backed — that was removed). It works uniformly across all supported languages using the same heuristic extractor.

```json
{
  "file": "src/main.rs",
  "language": "Rust",
  "mode": "index",
  "total_lines": 34,
  "total_bytes": 512,
  "symbol_count": 1,
  "symbols": [
    {
      "kind": "function",
      "name": "main",
      "line_start": 10,
      "signature": "fn main() {",
      "visibility": "public"
    }
  ]
}
```

**Symbol field notes:**
- `kind`, `name`, `line_start`, `signature` are always present
- `line_end` exists in the schema but the current regex/heuristic extractor never populates it — it is always absent today, not just "when detectable". Don't rely on it to locate a symbol's closing boundary; use `line_start` plus your own bracket/indent matching if you need the full body
- `visibility` is present only for Rust (`pub` / `pub(crate)` / `pub(super)` / `private`) and JavaScript/TypeScript (`export` / `local`); it is omitted entirely for other languages

### Directory Input (NDJSON)

Passing a directory to `--mode=index` recursively walks it (skipping hidden directories and symlinks) and emits one compact JSON object per line — NDJSON, one file per line, each with the schema above (or `{"file": ..., "error": ...}` if a file failed to process):

```bash
batless --mode=index src/ | jq -c '.symbols[] | select(.kind=="function")'
```

## Error Handling

When batless is not available, use these fallback commands:

```bash
# Fallback for line numbers
cat -n file.py

# Fallback for limiting output
head -50 file.py

# Fallback for JSON structure (basic)
echo "{\"file\": \"$1\", \"content\": \"$(cat $1 | jq -Rs .)\"}"
```

## Common Recipes

```bash
# Symbol navigation: jump to a symbol's line range
batless --mode=index src/lib.rs | jq '.symbols[] | select(.name=="MyStruct")'

# Public API surface across a directory (NDJSON walk)
batless --mode=index src | jq -c '.symbols[] | select(.kind=="function" and .visibility=="pub")'

# Symbol-level diff of a file's structure before and after a change
git show HEAD~1:src/main.rs > /tmp/before.rs
diff <(batless --mode=index /tmp/before.rs | jq '.symbols') <(batless --mode=index src/main.rs | jq '.symbols')

# Build a JSONL context file from multiple files for downstream tooling
find src -name "*.rs" -exec sh -c 'batless --mode=json "$1"' _ {} \; > codebase-context.jsonl
```

### CI/CD Pipeline Usage

```yaml
# GitHub Actions - show test failure context
- name: Show failing test code
  if: failure()
  run: batless --max-lines=100 tests/test_auth.py

# GitLab CI - extract JSON as a build artifact
analyze-code:
  script:
    - batless --mode=json src/main.rs > code-analysis.json
  artifacts:
    paths:
      - code-analysis.json
```

## Version Information

```bash
# Human-readable version
batless --version

# Machine-readable version (JSON: name, version, git_hash, build_timestamp, authors)
batless --version-json
```

## Philosophy

batless is designed to be:

- **Non-blocking**: Never waits for user input, never pages
- **Predictable**: Same output whether in terminal or pipe
- **Minimal**: Plain text by default, no decorations, no highlighting
- **Honest about scope**: Views individual files; use `ls`/`find`/`fd`/`grep`/`rg` for discovery and search, and pipe their results into batless
