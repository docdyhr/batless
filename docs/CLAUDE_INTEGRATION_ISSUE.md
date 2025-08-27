# Issue: Discrepancy Between CLAUDE.md Protocol and Actual batless Implementation

## Problem Statement

During a real-world usage session with an AI assistant (Claude) following the batless protocol specified in CLAUDE.md, multiple expected commands failed because they don't exist in the actual batless implementation.

## Specific Examples of Failed Commands

### 1. Line Numbers Flag

**Expected (per CLAUDE.md):** `batless -n pyproject.toml`
**Actual:** Flag `-n` doesn't exist
**Current batless way:** Not supported (by design - no decorations)
**Fallback used:** `cat -n pyproject.toml`

### 2. Pattern Search

**Expected:** `batless --pattern "function_name" src/`
**Actual:** `--pattern` flag doesn't exist
**Current batless way:** Not implemented (out of scope)
**Fallback used:** `grep -rn "function_name" src/`

### 3. Line Range Selection

**Expected:** `batless -r 10:50 file.py`
**Actual:** `-r START:END` syntax doesn't exist
**Current batless way:** Only `--max-lines` from start
**Fallback used:** `sed -n '10,50p' file.py`

### 4. File Listing

**Expected:** `batless --list *.py`
**Actual:** `--list` doesn't exist for files
**Current batless way:** Only `--list-languages` and `--list-themes`
**Fallback used:** `ls -la *.py`

## Root Cause Analysis

The CLAUDE.md file appears to describe an idealized or planned interface that doesn't match the actual implementation. This creates confusion for:

1. **AI Assistants**: Following outdated/incorrect protocol
2. **New Users**: Expecting features that don't exist
3. **Contributors**: Unclear on intended vs actual functionality

## Proposed Solutions

### Solution 1: Fix Documentation (Immediate)

Update CLAUDE.md to reflect actual batless capabilities:

```markdown
# Correct usage examples
batless file.py                    # View with syntax highlighting
batless --max-lines=50 file.py     # Limit output
batless --mode=summary file.py     # AI-friendly summary
batless --mode=json file.py        # Structured output

# NOT supported (use alternatives)
# For line numbers: use cat -n
# For pattern search: use grep
# For line ranges: use sed or head/tail
# For file listing: use ls or find
```

### Solution 2: Add Missing Features (Long-term)

Implement commonly expected flags:

```rust
// Add to CLI args
#[clap(short = 'n', long = "line-numbers")]
show_line_numbers: bool,

#[clap(short = 'r', long = "range")]
line_range: Option<String>,  // "10:50" format
```

### Solution 3: Compatibility Layer (Medium-term)

Create a wrapper script or compatibility mode that translates common patterns:

```bash
#!/bin/bash
# batless-compat
case "$1" in
    -n) shift; batless --with-line-numbers "$@" ;;
    --pattern) shift; pattern=$1; shift; grep -r "$pattern" "$@" ;;
    -r) shift; range=$1; shift; # Parse and use --max-lines ;;
    *) batless "$@" ;;
esac
```

## Testing Evidence

Real-world session where this caused issues:

- **Context**: Technical debt assessment using Claude
- **Impact**: All commands failed, requiring manual fallback
- **Workaround**: Successfully used cat, grep, sed alternatives
- **User Experience**: Confusing, reduced efficiency

## Acceptance Criteria

1. [ ] CLAUDE.md accurately reflects actual batless capabilities
2. [ ] Clear documentation of what batless does NOT do
3. [ ] Either implement missing features OR document alternatives
4. [ ] AI assistants can successfully use batless following the protocol

## Priority Assessment

- **Severity**: Medium (workarounds exist but cause confusion)
- **Frequency**: High (affects all AI assistant integrations)
- **Fix Effort**: Low for docs, Medium for features

## Recommendations

1. **Immediate**: Update CLAUDE.md with correct usage examples
2. **Short-term**: Add FAQ section about what batless doesn't do
3. **Consider**: Whether line ranges fit the streaming philosophy
4. **Evaluate**: If optional line numbers violate "no decorations" principle
