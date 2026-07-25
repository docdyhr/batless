# batless Philosophy & Scope

## Core Philosophy: Do One Thing Well

**batless exists to VIEW code and text files in a non-blocking way — fast, predictable, and honest about what it does.**

Following the Unix philosophy, batless intentionally maintains a narrow, focused scope:

> "Write programs that do one thing and do it well. Write programs to work together with other programs."
> — Doug McIlroy, Unix Philosophy

## What batless IS

✅ **A fast, non-blocking cat/bat alternative**

- Views individual files via `--mode=plain` (the default), `--mode=json`, or `--mode=index`
- Never blocks or waits for user input — no pager, ever
- Predictable, scriptable output whether run in a terminal or a pipe
- Drop-in replacement for `cat`, with `cat -n`/`cat -b` compatibility (`-n`, `-b`) and `PAGER` compatibility (`--plain`, `-u`, `--no-title`)

✅ **Scoped by real usage data, not guesswork**

- Usage telemetry (batless-stats logs, ~475 calls over 3.5 months of real usage) showed plain-mode viewing at 84.4% of all invocations, with `--mode=index` — a machine-readable symbol table — a distant but real second at 10.5%
- Features that measured 0-1.5% usage or none at all (tree-sitter AST mode, streaming/checkpointing, JSON schema validation, AI model profiles, LLM token estimation, `--mode=summary`, `--hash`) were removed rather than kept "just in case"
- The result is a smaller, more honest tool built around what people actually run, not around what sounded good on paper

✅ **Still useful in scripts and pipelines**

- `--mode=json` for structured metadata; `--mode=index` for a symbol table (functions, classes, structs with line ranges) without loading full file content
- `--strip-comments` / `--strip-blank-lines` for general-purpose text filtering, in the spirit of `cat -s` — not AI-specific
- Full stdin support, encoding detection, and shell completions (bash/zsh/fish/PowerShell)

## What batless IS NOT

❌ **Not a search tool** (use `grep`, `rg`, `ag`)
❌ **Not a file browser** (use `ls`, `find`, `fd`)
❌ **Not a text editor** (use `vim`, `nano`, `code`)
❌ **Not an interactive pager** (use `less`, `more`, `bat`)
❌ **Not a Git tool** (use `git`, `tig`, `lazygit`)

## Design Decisions

### Why No `--pattern` / Search?

**Decision**: Keep searching separate

**Rationale**:

- `grep` and `rg` are mature, optimized search tools
- Search requires different performance characteristics (indexing, parallel scanning)
- Would duplicate existing excellent tools
- Violates "do one thing well" principle

**Better approach**: Excellent error messages with hints

### Why No `--list` / File Browsing?

**Decision**: Keep file discovery separate

**Rationale**:

- `ls`, `find`, `fd`, `tree` are purpose-built for this
- Directory traversal adds complexity
- Respecting `.gitignore` requires git integration
- Metadata display (size, dates, permissions) is `ls` territory

**Better approach**: Document the pipeline pattern

### Why No `--range` / Line Selection?

**Decision**: This one is debatable

**Arguments FOR adding `--range`**:

- Very common use case
- Simple to implement
- Doesn't violate core mission
- Would improve usability significantly

**Arguments AGAINST**:

- `sed`, `head`, `tail` already exist
- Adds API surface area
- Not strictly "viewing" (more like "extracting")

**Recommendation**: Consider for v0.4.0, but with caveats:

- Only if trivial to implement
- Only basic syntax: `-r START:END`
- No complex features (negative indices, multiple ranges)

## The Middle Ground: Enhanced UX Without Scope Creep

Instead of adding features, improve the **user experience when using the right tools**:

### 1. ✅ Helpful Error Messages

**Current**:

```
Error: unexpected argument '--pattern' found
```

**Improved**:

```
Error: unexpected argument '--pattern' found

💡 Tip: batless is a file viewer, not a search tool.
   To search for patterns, use:
     grep -rn "pattern" src/
     rg "pattern" src/          # even faster!

   Then view results with batless:
     batless $(grep -l "pattern" src/*)
```

### 2. ✅ Comprehensive Documentation

**Current state**: Documentation exists but could be better integrated

**Improvements**:

- Quick reference guide with common patterns
- Cheat sheet for "I want to X" → "Use Y"
- Examples of batless in pipelines
- Integration guides for AI workflows

### 3. ✅ Better Pipeline Support

Make batless work excellently with other tools:

```bash
# Find and view pattern matches
grep -l "TODO" src/*.rs | xargs batless -n

# View specific ranges from multiple files
find . -name "*.py" -exec sh -c 'echo "=== {} ==="; sed -n "1,50p" {} | batless --language=python' \;

# Automation workflow: search, then extract structure
rg -l "async fn" src/ | while read f; do
  batless --mode=index "$f"
done | jq -s '[.[] | {file: .file, async_functions: [.symbols[] | select(.signature | contains("async fn"))]}]'
```

### 4. ✅ Man Page / Help Examples

```bash
batless --help-examples

COMMON PATTERNS

  View a file with line numbers:
    batless -n src/main.rs

  View specific line range (use sed):
    sed -n '10,50p' file.py | batless --language=python

  Find and view files containing pattern:
    grep -l "pattern" src/* | xargs batless

  List files in directory (use ls/fd):
    fd -e py | xargs batless

  Extract code structure (symbol index):
    batless --mode=index src/*.rs | jq '.symbols'
```

## Implementation Recommendation

### Phase 1: Improve UX (v0.3.1) ✅ **Recommended**

1. **Enhanced error messages** with helpful hints
2. **Add `--help-examples`** subcommand
3. **Improve documentation** with pipeline patterns
4. **Add cookbook** of common workflows
5. **Better integration** with grep/find/fd in docs

**Impact**: 90% of user confusion solved
**Effort**: Low (mostly documentation)
**Maintains**: Focused scope, Unix philosophy

### Phase 2: Consider Minimal Range Support (v0.4.0) 🤔 **Debatable**

**Only if**:

- Community strongly requests it
- Implementation is trivial (<100 lines)
- Doesn't add dependencies
- Doesn't compromise performance

**Implementation**:

```rust
// Simple, no-frills range support
--range START:END   // e.g., --range 10:50
```

**No complex features**:

- ❌ Negative indices
- ❌ Multiple ranges
- ❌ Regex-based ranges
- ❌ Context lines

### Phase 3: NO Feature Creep ❌ **Not Recommended**

**Do NOT add**:

- Pattern searching (`--pattern`)
- File listing (`--list`)
- Interactive features
- Git integration
- Diff viewing
- Anything that violates core mission

## Decision Framework

When considering new features, ask:

1. **Does it help VIEW code better?**
   - ✅ Yes → Consider
   - ❌ No → Reject

2. **Can existing tools do it well?**
   - ✅ Yes, and they're standard → Reject
   - ❌ No good alternative → Consider

3. **Does it add complexity?**
   - ✅ Significant → Reject
   - ❌ Trivial → Consider

4. **Is it automatable?**
   - ✅ Yes → Prefer pipeline pattern
   - ❌ No → Consider built-in

## Real-World Example: The Error Reports

Users tried:

- `batless --list .vscode/`
- `batless --pattern "import.*@/" src/`

**Wrong response**: Add these features

**Right response**:

1. Fix documentation ✅ (Done)
2. Improve error messages with hints ✅ (Recommended)
3. Document the correct tools to use ✅ (Recommended)
4. Show pipeline patterns ✅ (Recommended)

## Conclusion

**Recommendation**: **Stick to the middle ground**

1. ✅ **Keep batless focused** on viewing files
2. ✅ **Enhance user experience** through better errors/docs
3. ✅ **Embrace the pipeline** model
4. ✅ **Document integrations** with other tools
5. 🤔 **Maybe** add `--range` if trivial to implement

**Success metric**: Users understand batless's role and naturally reach for the right tool for each task.

---

## Action Items

### Immediate (v0.3.1)

- [ ] Add helpful hints to error messages
- [ ] Create `--help-examples` command
- [ ] Write cookbook of common patterns
- [ ] Improve CLAUDE.md examples
- [ ] Add pipeline integration guide

### Future (v0.4.0+)

- [ ] Evaluate `--range` feature request
- [ ] Survey users on most common pain points
- [ ] Measure: Are users still trying wrong commands?

### Never

- [ ] ~~Add pattern searching~~
- [ ] ~~Add file listing~~
- [ ] ~~Add interactive features~~
- [ ] ~~Become bat++~~

---

*"Perfection is achieved, not when there is nothing more to add, but when there is nothing left to take away."*
— Antoine de Saint-Exupéry

This philosophy guide ensures batless remains:

- **Simple** to understand
- **Easy** to maintain
- **Reliable** in behavior
- **Composable** with other tools
- **Focused** on its core mission

---

*Last Updated: July 25, 2026*
