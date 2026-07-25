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
| **v0.6.0** | Apr 2026 | Removed syntax highlighting, themes, and the interactive wizard; `--mode=plain` became the default |
| **Unreleased (Option A)** | — | Major scope reduction: removed AST mode, AI profiles, hashing, summary mode, and streaming based on usage telemetry — see below |

---

## v0.5.0: AI Efficiency (Released April 2026)

All items shipped and tagged. **Note:** most of the AI-specific features below (`--hash`, `--include-identifiers`, AI profiles, `--chunk-strategy=semantic`, `estimated_llm_tokens`) were later removed in the Option A scope reduction (see [Option A: Scope Reduction](#option-a-scope-reduction-unreleased) below) after usage telemetry showed near-zero real-world adoption.

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

> **Superseded (Option A):** AST summarization and `--hash` were removed from the CLI entirely rather than fixed on stdin — see [Option A: Scope Reduction](#option-a-scope-reduction-unreleased) below. `--strip-comments` / `--strip-blank-lines` / `--language` parity on stdin remains a valid, smaller-scoped fix.

`process_stdin` was missing feature parity with `process_file`:
- ~~AST summarization (currently regex-only fallback)~~ — moot, AST mode removed
- ~~`--hash` support (silently ignored on stdin)~~ — moot, `--hash` removed
- `--strip-comments` / `--strip-blank-lines` (silently ignored on stdin)
- Language detection from `--language` flag (partially works)

Fix: extract a shared `post_process(lines, language, config)` pipeline that both paths call.

### ~~Feature: `--mode=ast` Raw Output~~ (Cancelled)

> **Cancelled (Option A):** Rather than exposing the raw tree-sitter parse tree, AST mode and the entire tree-sitter dependency were removed. Usage telemetry showed `--mode=ast` was effectively unused. See [Option A: Scope Reduction](#option-a-scope-reduction-unreleased) below.

### Feature: Multi-file Index Mode

Allow `batless --mode=index src/` to process a directory and emit one JSON object per file (NDJSON), enabling a project-wide symbol table in one invocation.

```bash
batless --mode=index src/ | jq -s 'map(.symbols) | flatten | group_by(.kind)'
```

---

## Option A: Scope Reduction (Unreleased)

After v0.6.0 shipped, usage telemetry (batless-stats logs, ~475 real invocations over 3.5 months) showed the roadmap's AI-native direction wasn't matching real usage: `--mode=plain` accounted for 84.4% of calls and `--mode=index` a distant but real 10.5%, while every AI/automation-oriented feature planned above — AST mode, AI profiles, LLM token estimation, content hashing, summary mode, NDJSON streaming, JSON schema validation — measured at roughly 0-1.5% or was never invoked at all.

Rather than continuing to build out the tree-sitter/AI-profile foundation (as the v0.6.0 and v0.7.0 plans above assumed), batless pivoted to a scope reduction:

- **Removed**: `--mode=ast`, the entire tree-sitter dependency (4 grammar crates + core), the AI profile system (`--profile`, `--custom-profile`, `--ai-model`, `--fit-context`, `--prompt-tokens`, `--count-tokens`), `--include-identifiers`/`--include-tokens`, `--hash` (and `sha2`), `--mode=summary`/`--summary`/`--summary-level`, streaming (`--streaming-json`, `--streaming-chunk-size`, `--chunk-strategy`, `--enable-resume`, `--checkpoint`), and JSON schema validation (`--get-schema`, `--validate-json`)
- **Kept and simplified**: `--mode=index` now uses a single regex/heuristic symbol extractor for every language (previously tree-sitter-backed for Rust/Python/JS/TS with regex fallback elsewhere) — same output shape, less precise boundaries for those four languages
- **Result**: stripped binary reduced from 8.0MB to 1.5MB (81% smaller); test suite reduced from 365 to 193 tests to match the smaller surface area

See `CHANGELOG.md` (Unreleased section) for the full breaking-changes list and `docs/PHILOSOPHY_AND_SCOPE.md` for the resulting positioning: a fast, honest, non-blocking `cat`/`bat` alternative rather than an "AI-native" tool. The v0.6.0 and v0.7.0 sections above are kept for historical record but should be read with this in mind.

---

## v0.7.0: Deeper Language Analysis

*Target: Q4 2026*

> **Superseded (Option A):** this entire milestone assumed a tree-sitter foundation that no longer exists — the tree-sitter dependency and AST/summary modes were removed rather than extended. `--mode=index` is now regex/heuristic-only for every language. Any future language-analysis work would need to start from that regex foundation, not this plan.

~~Build on the tree-sitter foundation to expose richer per-symbol data.~~

- ~~Signature extraction improvements — full parameter types and return types from AST nodes (not string stripping)~~
- ~~Import/dependency listing — `--mode=imports` emits a flat list of all imports/requires/use statements~~
- ~~Additional language coverage — Go, Ruby, C, C++ added to AST summarizer (tree-sitter grammars available)~~
- ~~`--summary-level=comments` — extract doc comments and attach to their symbol in index output~~ (moot — `--mode=summary` was removed entirely)

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
