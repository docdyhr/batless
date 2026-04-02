# batless Development Roadmap

> Strategic development plan for batless — the non-blocking code viewer built for AI and automation

## Vision

batless is the definitive code-viewing tool for AI workflows and automation pipelines. It never blocks, never pages, and always produces machine-readable output. Each release extends what AI agents can do with code: better context efficiency, richer navigation data, and broader language coverage — without adding interactive features or scope creep.

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

## v0.6.0: Internal Consolidation + stdin Parity

*Target: Q3 2026*

Focus: clean up the architectural debt identified post-v0.5.0 before adding new features. No breaking changes to the CLI surface.

### Code Health

- **Delete dead modules** — `src/debt_prevention.rs` and `src/performance.rs` are never called; remove them
- **Delete unused traits** — `FileProcessing`, `EncodingDetection`, `ProcessorFactory` in `traits.rs` have zero implementations; remove them
- **Consolidate formatter system** — `src/formatter.rs` has inline Plain/JSON/Summary implementations; `src/formatters/` has parallel dead copies; migrate to a single trait-based path

### Feature: `process_stdin` Parity

`process_stdin` is missing four features that `process_file` has:
- AST summarization (currently regex-only fallback)
- `--hash` support (silently ignored on stdin)
- `--strip-comments` / `--strip-blank-lines` (silently ignored on stdin)
- Language detection from provided `--language` flag (partially works)

Fix: extract a shared `post_process(lines, language, config)` pipeline that both paths call.

### Feature: `--ast` Raw Output

Expose the tree-sitter parse tree directly as JSON. The parsers are already in the dependency tree; this is a new output mode, not new infrastructure.

```bash
batless --mode=ast src/main.rs | jq '.tree.children[0]'
```

### Feature: Multi-file Index Mode

Allow `batless --mode=index src/` to process a directory and emit one JSON object per file to stdout (NDJSON), enabling a project-wide symbol table in a single invocation.

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

These were in earlier drafts and have been explicitly removed:

- **Plugin architecture** — dynamic plugin loading adds significant complexity (sandboxing, signing, registry) for unclear gain; the trait-based extension points in `traits.rs` are sufficient
- **Language Server Protocol client** — LSP is a separate tool category; batless is a viewer, not an IDE backend
- **Cross-reference / call graph analysis** — requires multi-file indexing infrastructure; too large for the current scope
- **WASM / browser build** — possible but not a priority; nothing in the current user base requires it
- **Enterprise features** (SSO, SAML, multi-tenant, audit logging, compliance certifications) — out of scope for a CLI tool

---

## Contributing

Feature requests and design input welcome via GitHub Issues. For roadmap-level discussion, open a Discussion thread — changes to this document go through the same review process as code.
