# PRD: Git Integration for Batless

## Title

Git Integration for `batless`

## Author

docdyhr / Claude (2025-01-22)

## Status

Proposed

## Overview

Add Git integration to `batless` similar to [`bat`](https://github.com/sharkdp/bat), enabling syntax-highlighted code output enriched with Git diff and blame context. This feature will enhance developer workflows by visually distinguishing code changes, deletions, and insertions, while maintaining non-interactive, AI-friendly formatting.

---

## Goals

- ✅ Display Git diff information (added, removed, modified lines)
- ✅ Display Git blame information (optional flag)
- ✅ Colorize output to match Git status (green = added, red = removed, blue = modified, etc.)
- ✅ Remain non-interactive and pager-free
- ✅ Integrate seamlessly with `batless`'s Rust architecture
- ✅ Preserve output compatibility for AI assistants and scripts

---

## Non-Goals

- ❌ Full Git client or porcelain interface
- ❌ Git conflict resolution or merge tools
- ❌ Interactive staging / unstaging

---

## Inspiration

From [`bat` Git integration](https://github.com/sharkdp/bat#git-integration), which:

- Shows diff indicators in the gutter
- Highlights changed lines using colors
- Detects staged vs. unstaged content
- Uses `git diff --no-index` or `--unified=0`

---

## Requirements

### Functional

| Feature                        | Description                                                                 |
|-------------------------------|-----------------------------------------------------------------------------|
| `--diff`                      | Show Git diff inline for files in a Git repo                                |
| `--blame`                     | Show Git blame information per line (author, timestamp, hash)              |
| Gutter indicators             | `+`, `-`, and `|` shown in gutter for additions, deletions, unchanged lines|
| Color scheme                  | Git-like: green (add), red (remove), blue (modify)                         |
| Fallback for non-repo files   | Graceful fallback when file is not under Git control                       |
| `--no-git`                    | Disable Git integration explicitly                                         |

### Non-Functional

- Must run quickly on large files
- Must work across platforms (macOS, Linux, Windows)
- Should work in CI logs and AI outputs without control sequences

---

## User Stories

### Developer using `batless` in a terminal

> As a developer, I want to view Git diffs inline with syntax highlighting, so I can understand changes at a glance.

```bash
batless --diff main.rs
```

### Claude user consuming logs

> As an AI agent reading file output, I want to understand what changed in a file using colored but static markup, not ANSI prompts.

---

## Implementation Plan

### Git Diff

- Use `git diff --color=always --no-index <file>` for colorized diffs
- Parse output to associate diffs with file lines
- Highlight modified lines using `syntect`
- Add gutter symbols (`+`, `-`, etc.)

### Git Blame

- Use `git blame --line-porcelain <file>`
- Parse author, commit hash, and date
- Show blame info optionally alongside each line in a muted format

### Flags & CLI

```bash
batless --diff              # Show inline Git diff
batless --blame             # Show blame annotations
batless --diff --blame      # Combine both
batless --no-git            # Disable Git features
```

### Output Format

Use a static layout like:

```
│+ fn new_function() {
│+     println!("Hello");
│  }
```

Color and syntax highlighting are preserved using `syntect`.

---

## Dependencies

- [libgit2](https://libgit2.org/) or CLI subprocesses (`git diff`, `git blame`)
- [syntect](https://github.com/trishume/syntect) for syntax highlighting
- [ansi_term](https://github.com/ogham/rust-ansi-term) or `nu-ansi-term` for coloring output

---

## Future Enhancements

- Show staged vs. unstaged diffs with `--staged`
- Integrate with diffing of untracked files
- Inline blame annotations using hover or tooltips in supported terminals

---

## Out of Scope

- No support for Git worktrees, submodules, or detached HEAD parsing initially
- No support for remote diffs or GitHub/GitLab API interactions

---

## Alternatives Considered

- Reimplementing Git features via `git2` crate — higher complexity
- Using `delta` or `difflib` — too interactive or stylistically complex

---

## References

- [`bat` Git diff example](https://github.com/sharkdp/bat#git-integration)
- [`delta`](https://github.com/dandavison/delta)
- [`git2` Rust crate](https://github.com/rust-lang/git2-rs)

---

## Appendix

### Example Output

```text
 1 │ fn main() {
 2 │     println!("Hello");
 3 │ }
```

With `--diff`:

```text
 1 │ fn main() {
 2 │-    println!("Hello");
 2 │+    println!("Hello, world!");
 3 │ }
```

With `--blame`:

```text
 1 │ [a1b2c3] Alice 2024-06-01 │ fn main() {
 2 │ [d4e5f6] Bob   2024-06-02 │     println!("Hello");
 3 │ [d4e5f6] Bob   2024-06-02 │ }
```

---

## Maintainer Notes

This should be an opt-in feature with robust CLI flags and fallbacks. Use unit tests to validate gutter rendering and edge-case Git behavior.
