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
  "total_bytes": 3420,
  "truncated": false,
  "truncated_by_lines": false,
  "truncated_by_bytes": false,
  "encoding": "UTF-8",
  "syntax_errors": [],
  "mode": "json"
}
```

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

### CI/CD Pipeline

```yaml
- name: Show code context
  run: batless --mode=summary --max-lines=100 src/main.py
```

### AI Context Building

```bash
# Get structured data for AI processing
batless --mode=json --include-tokens --max-lines=500 complex.py | \
  jq '{file, language, summary_lines, total_lines}'
```

### Automated Code Review

```bash
# Extract code structure for analysis
for file in $(git diff --name-only); do
  batless --mode=summary --max-lines=50 "$file"
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
