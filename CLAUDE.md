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
batless --profile=claude file.py      # Claude-optimized
batless --profile=copilot file.py     # GitHub Copilot
batless --profile=chatgpt file.py     # ChatGPT

# Include token counts in JSON
batless --mode=json --include-tokens file.py

# Get both summary and full content
batless --mode=json --summary file.py
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

```bash
# Show line numbers (like cat -n)
batless -n file.py
batless --number file.py

# Number non-blank lines only (like cat -b)
batless -b file.py
batless --number-nonblank file.py
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

# CORRECT: Use grep
grep -rn "TODO" src/
```

### Line Range Selection

```bash
# WRONG: batless does not support arbitrary ranges
# batless -r 10:50 file.py

# CORRECT: Use sed or head/tail
sed -n '10,50p' file.py
head -50 file.py | tail -41
```

### File Globbing/Listing

```bash
# WRONG: batless does not list files
# batless --list *.py

# CORRECT: Use shell expansion or find
batless *.py  # Shell expands the glob
find . -name "*.py" -exec batless {} \;
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

```json
{
  "file": "path/to/file.py",
  "language": "Python",
  "lines": ["line1", "line2", ...],
  "summary_lines": ["import os", "def main():", ...],
  "tokens": ["import", "os", "def", "main", ...],
  "total_lines": 150,
  "total_lines_exact": true,
  "total_bytes": 3420,
  "truncated": false,
  "truncated_by_lines": false,
  "truncated_by_bytes": false,
  "token_count": 420,
  "tokens_truncated": false,
  "encoding": "UTF-8",
  "syntax_errors": [],
  "mode": "json"
}
```

> `lines` always contains the full file contents. When `--summary` or `--summary-level` is enabled, the condensed view is emitted separately via `summary_lines`, so AI tools can combine both views without losing context.

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

# Preview log file with line numbers for debugging
batless -n --max-lines=500 server.log
```

### Extracting JSON for AI Processing

```bash
# Extract code structure as JSON for LLM analysis
batless --mode=json --include-tokens --max-lines=500 src/main.py | \
  jq '{file, language, summary: .summary_lines, tokens: .tokens | length}'

# Get function signatures only
batless --mode=summary src/lib.rs | grep "^fn "

# Build context for AI code review
batless --mode=json --summary --max-lines=300 complex.py > ai-context.json

# Extract multiple files for AI analysis
find src -name "*.py" -exec sh -c \
  'batless --mode=json --summary "$1" | jq -c .' _ {} \; > codebase-context.jsonl
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
batless --mode=json --include-tokens --max-lines=500 complex.py | \
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

# Analyze complexity by counting tokens
for file in src/*.py; do
  tokens=$(batless --mode=json --include-tokens "$file" | jq '.tokens | length')
  echo "$file: $tokens tokens"
done | sort -t: -k2 -n

# Check for TODO comments across codebase
find . -name "*.rs" -exec sh -c \
  'batless -n "$1" | grep -i "TODO\|FIXME\|XXX"' _ {} \;
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
