# batless Development Roadmap

> Strategic development plan for batless — machine-readable code analysis for AI and automation

## Vision

batless is the definitive AI-native code analysis tool. It produces structured, machine-readable output that AI assistants, CI/CD pipelines, and automation scripts can consume directly. Its unique value is not file viewing — AI assistants already have native read tools for that — but the outputs they cannot produce themselves: symbol indexes, token-estimated compressed context, semantic chunks, and content hashes.

Each release sharpens this focus: richer analysis data, broader language coverage, and a leaner binary. Interactive and cosmetic features are out of scope.

---

## Release History

| Version | Released | Focus |
|---------|----------|-------|
| **v0.3.1** | Oct 2025 | UX & CI/CD — AI profiles, cat compatibility, performance suite |
| **v0.4.0** | Dec 2025 | AST Summarization — tree-sitter for Rust/Python/JS/TS, `SummaryItem` with line numbers |
| **v0.5.0** | Apr 2026 | AI Efficiency — NDJSON streaming, index mode, semantic chunking, comment stripping |

---

## v0.5.0: AI Efficiency (Released April 2026)

All items shipped and tagged.

- **NDJSON streaming** — removed `---` separator; each chunk is compact JSON + newline
- **`--with-line-numbers`** — JSON `lines` array entries become `{"n": N, "text": "..."}` objects
- **`--mode=index`** — machine-readable symbol table: `{kind, name, line_start, line_end, signature, visibility}`
- **`--chunk-strategy=semantic`** — streaming extends chunks to tree-sitter top-level boundaries
- **`--strip-comments` / `--strip-blank-lines`** — language-aware content stripping; `compression_ratio` in JSON output
- **`--hash`** — SHA-256 file hash in JSON output for change detection
- **`--include-identifiers`** — renamed from `--include-tokens` (deprecated alias kept); `tokens` field renamed `identifiers`
- **`estimated_llm_tokens` / `token_model`** in JSON when a profile or `--ai-model` is active
- **`--profile=claude-max`** — new profile: 150K lines, JSON output, no summary
- **Claude profile raised** to 20K lines (was 4K)

---

## v0.6.0: Sharpen the Core

*Target: Q3 2026*

Focus: remove the fluff, fix architectural debt, and extend the AI-specific features. This release makes the strategic pivot concrete in code.

### Remove: Syntax Highlighting and Themes

Syntax highlighting (`--mode=highlight`, `syntect` crate, `--theme`, `ThemeManager`) serves human terminal users — a use case where `bat` is the better tool. AI assistants don't benefit from ANSI color codes and have native read tools for plain file content.

- **Deprecate** `--mode=highlight` (the default bare invocation) and `--theme` — emit a warning directing users to `bat` for human viewing
- **Remove** `src/highlighter.rs`, `ThemeManager` from `src/language.rs`, syntect integration from `src/formatter.rs`
- **Remove** dependencies: `syntect`, `is-terminal`, `termcolor`, `strip-ansi-escapes`
- **New default mode**: `--mode=plain` (no colors, no syntect dependency)
- **Binary size reduction**: ~1.5MB off the ~2MB binary

### Remove: Interactive Wizard

`src/wizard.rs` (799 lines) is an interactive TUI config setup — the opposite of automation-first. Remove it; config is documented in the README.

### Fix: Dead Code Cleanup

- Delete `src/debt_prevention.rs` and `src/performance.rs` (never called)
- Delete unused traits `FileProcessing`, `EncodingDetection`, `ProcessorFactory` from `src/traits.rs`
- Consolidate formatter system: migrate `src/formatters/` parallel dead copies into a single trait-based path in `src/formatter.rs`

### Fix: `process_stdin` Parity

`process_stdin` is missing four features that `process_file` has:
- AST summarization (currently regex-only fallback)
- `--hash` support (silently ignored on stdin)
- `--strip-comments` / `--strip-blank-lines` (silently ignored on stdin)
- Language detection from `--language` flag (partially works)

Fix: extract a shared `post_process(lines, language, config)` pipeline that both paths call.

### Feature: `--mode=ast` Raw Output

Expose the tree-sitter parse tree directly as JSON. Parsers are already in the dependency tree.

```bash
batless --mode=ast src/main.rs | jq '.tree.children[0]'
```

### Feature: Multi-file Index Mode

Allow `batless --mode=index src/` to process a directory and emit one JSON object per file (NDJSON), enabling a project-wide symbol table in one invocation.

```bash
batless --mode=index src/ | jq -s 'map(.symbols) | flatten | group_by(.kind)'
```

---

## v0.7.0: Deeper Language Analysis

*Target: Q4 2026*

Build on the tree-sitter foundation to expose richer per-symbol data.

- **Signature extraction improvements** — full parameter types and return types from AST nodes (not string stripping)
- **Import/dependency listing** — `--mode=imports` emits a flat list of all imports/requires/use statements
- **Additional language coverage** — Go, Ruby, C, C++ added to AST summarizer (tree-sitter grammars available)
- **`--summary-level=comments`** — extract doc comments and attach to their symbol in index output

---

## v1.0.0: Stability & Ecosystem

*Target: H1 2027*

The 1.0 milestone signals API and output schema stability. No major new features — focus on guarantees.

- **Stable JSON schema** — commit to backwards compatibility for all JSON output fields; add schema version field
- **Shell completions** — generated completions for bash, zsh, fish, PowerShell included in release artifacts
- **`--validate`** flag — validate a file's JSON output against the published schema
- **GitHub Action** — `batless-action` for using batless in CI workflows without installing manually
- **Homebrew formula** — automated tap update on release
- **MSRV policy** — explicit minimum supported Rust version with a documented update policy

---

## What is NOT on the Roadmap

- **Syntax highlighting improvements** — `bat` does this better; AI assistants don't need ANSI colors
- **Theme support** — cosmetic; no AI value
- **Interactive features of any kind** — anti-automation by definition
- **Plugin architecture** — dynamic loading adds complexity (sandboxing, signing, registry) for unclear AI gain
- **Language Server Protocol client** — LSP is a separate tool category; batless is analysis output, not an IDE backend
- **WASM / browser build** — not a priority; nothing in the current user base requires it
- **Enterprise features** (SSO, SAML, audit logging) — out of scope for a CLI tool

---

## Contributing

Feature requests and design input welcome via GitHub Issues. For roadmap-level discussion, open a Discussion thread — changes to this document go through the same review process as code.
