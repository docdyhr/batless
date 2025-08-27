# Documentation Mismatch: CLAUDE.md describes non-existent CLI commands

## Problem Summary

An AI assistant (Claude) attempted to use batless following the protocol described in CLAUDE.md but encountered multiple failures because the documented commands don't exist in the actual implementation.

## Impact

- **User Experience**: AI assistants and users following CLAUDE.md encounter immediate failures
- **Trust**: Documentation doesn't match reality, causing confusion
- **Adoption**: Users may abandon the tool when basic documented commands fail

## Commands That Failed

### 1. Line Numbers Flag

```bash
# Documented in CLAUDE.md (doesn't work)
batless -n pyproject.toml

# Error: unexpected argument '-n'
```

### 2. Pattern Search

```bash
# Documented (doesn't exist)
batless --pattern "function_name" src/

# Error: unrecognized option '--pattern'
```

### 3. Line Range Selection

```bash
# Documented (doesn't exist)
batless -r 10:50 file.py

# Error: unexpected argument '-r'
```

### 4. File Listing

```bash
# Documented (doesn't exist)
batless --list *.py

# Current --list only supports:
# --list-languages
# --list-themes
```

## Root Cause

CLAUDE.md appears to describe aspirational features that were never implemented, or possibly removed at some point.

## Proposed Solutions

### Option 1: Fix Documentation (Immediate) ✅ RECOMMENDED

Update CLAUDE.md to reflect actual batless capabilities:

```markdown
# CORRECT batless usage for AI assistants

## View files with syntax highlighting
batless file.py

## Limit output
batless --max-lines=50 file.py
batless --max-bytes=1024 file.py

## Output modes
batless --mode=plain file.py     # No colors
batless --mode=json file.py      # Structured output
batless --mode=summary file.py   # Code structure only

## What batless DOESN'T do
- No line numbers (use `cat -n` if needed)
- No pattern search (use `grep`)
- No line ranges (use `sed -n '10,50p'`)
- No file globbing (use shell expansion or `find`)
```

### Option 2: Implement Missing Features (Medium-term)

If these features align with batless philosophy:

```rust
// Add commonly expected flags
#[derive(Parser)]
struct Cli {
    /// Show line numbers (conflicts with "no decorations" philosophy?)
    #[arg(short = 'n', long = "number")]
    line_numbers: bool,

    /// Select line range (could work with streaming)
    #[arg(short = 'r', long = "range")]
    line_range: Option<String>,  // "10:50"
}
```

### Option 3: Create Compatibility Wrapper

Provide a separate script for users expecting common flags:

```bash
#!/bin/bash
# batless-compat - wrapper for common unix-style flags

case "$1" in
    -n)
        shift
        echo "Note: batless doesn't support line numbers. Use 'cat -n' instead."
        batless "$@"
        ;;
    --pattern)
        shift
        pattern=$1
        shift
        echo "Note: batless doesn't support pattern search. Use 'grep -r \"$pattern\"' instead."
        ;;
    *)
        batless "$@"
        ;;
esac
```

## Recommendation for Solo Dev

As a solo developer, I recommend:

1. **Immediate (Today)**: Update CLAUDE.md to match reality
2. **Short-term (This week)**: Add a FAQ section explaining philosophy
3. **Consider**: Whether any of these features actually fit batless's goals
4. **Document**: Clear statement about what batless intentionally DOESN'T do

## Action Items

- [ ] Update CLAUDE.md with correct usage examples
- [ ] Add "What batless doesn't do" section to README
- [ ] Consider adding `--lines=START:END` if it fits streaming model
- [ ] Close this issue with clear decision on each missing feature

## Lessons Learned

- Keep documentation in sync with implementation
- Be explicit about non-features and design decisions
- Consider adding integration tests that verify documented examples work
