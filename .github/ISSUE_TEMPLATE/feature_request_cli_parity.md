---
name: CLI Command Parity with Common Tools
about: Request for additional CLI flags to match common Unix tool patterns
title: '[FEATURE] Add common CLI flags for better tool compatibility'
labels: 'enhancement, documentation, cli'
assignees: ''

---

## Summary

When using batless in automated workflows (especially with AI assistants following the CLAUDE.md protocol), several expected command patterns are not available, requiring fallback to standard Unix tools. This issue proposes adding commonly expected flags to improve tool adoption and workflow consistency.

## Current Behavior

Commands attempted that don't exist in current batless:

- `batless -n file.txt` → Expected line numbers (like `cat -n`)
- `batless --pattern "TODO" src/` → Expected pattern search (like `grep -r`)
- `batless -r 10:50 file.py` → Expected line range (like `sed -n '10,50p'`)
- `batless --list *.py` → Expected file listing

Current batless equivalents require different syntax:

- Line numbers: Not available (batless explicitly avoids decorations)
- Pattern search: Not implemented (out of scope?)
- Line range: `batless --max-lines=50` (only from start)
- File listing: Not a batless feature

## Proposed Enhancement

Add compatibility flags that map to existing or new functionality:

### Option 1: Alias Flags (Minimal Change)

```bash
# Map common flags to existing features
batless -n file.txt          # → batless --line-numbers file.txt (new feature)
batless -r START:END file.txt # → batless --lines=START:END file.txt (new feature)
```

### Option 2: Full Parity Mode (Larger Scope)

```bash
# Add a compatibility mode for common Unix patterns
batless --compat cat -n file.txt     # Enable line numbers
batless --compat grep pattern file   # Basic pattern matching
batless --compat head -n 50 file     # Same as --max-lines=50
```

## Use Cases

1. **AI Assistant Integration**: AI tools trained on Unix commands expect these patterns
2. **Migration Path**: Easier adoption for users coming from cat/bat
3. **Muscle Memory**: Developers expect `-n` for line numbers across tools
4. **Scripting**: Consistent flags across tool ecosystem

## Impact Analysis

### Pros

- Lower barrier to adoption
- Better integration with existing workflows
- Clearer migration path from cat/bat

### Cons

- Conflicts with batless philosophy (no decorations)
- Scope creep into grep/sed territory
- May confuse users about tool's primary purpose

## Alternatives Considered

1. **Documentation-only fix**: Update CLAUDE.md to reflect actual batless syntax
2. **Wrapper script**: Create a `batless-compat` wrapper that translates commands
3. **Keep current design**: Maintain focus on non-blocking, decoration-free output

## Fallback Workarounds (Current State)

Users currently need to use these alternatives:

```bash
# Instead of: batless -n file.py
cat -n file.py

# Instead of: batless --pattern "TODO" src/
grep -rn "TODO" src/

# Instead of: batless -r 10:50 file.py
sed -n '10,50p' file.py

# Instead of: batless --list *.py
ls -la *.py
```

## Recommendation

At minimum, update documentation to clarify:

1. What batless does NOT do (by design)
2. Recommended alternatives for common patterns
3. Clear examples in CLAUDE.md that match actual implementation

If adding features, prioritize:

1. Line range selection (`--lines=START:END`) - fits streaming model
2. Optional line numbers (`--line-numbers`) - commonly needed
3. Skip pattern matching - out of scope, use grep

## Related Issues

- Related to philosophy of "no decorations"
- May conflict with streaming architecture for some features
- Documentation issue in CLAUDE.md needs addressing regardless

## Environment

- batless version: Expected from CLAUDE.md documentation
- Use case: AI assistant code analysis workflows
- Impact: Workflow disruption, fallback to multiple tools required
