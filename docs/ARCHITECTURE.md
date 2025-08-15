# batless Architecture Overview

Last updated: 2025-08-15

## Goals

Provide a high-level map of core modules, data flow, and extension points to accelerate onboarding and prepare for plugin architecture (v0.3.0).

## Core Principles

- Non-blocking streaming output
- Minimal state & memory footprint
- Deterministic, testable transformations
- Clear separation of concerns (parsing, formatting, output)

## Module Map

| Module | Purpose | Key Types / Functions | Notes |
|--------|---------|-----------------------|-------|
| `main.rs` | CLI entry, special command routing | `run`, `handle_special_commands` | Version JSON, completions, schema printing |
| `config_manager.rs` | Argument parsing, config resolution | `ConfigManager`, `Args` | Central place to access effective configuration |
| `config.rs` | Serializable runtime configuration | `BatlessConfig` | Parsed from profiles / CLI merge |
| `language.rs` | Language detection & metadata | `LanguageDetector` | Uses syntect syntaxes |
| `highlighter.rs` | Syntax highlighting | `Highlighter` | Delegates to syntect; isolated for future replacement |
| `formatter.rs` | Line formatting / coloring | `Formatter` | Stateless formatting helpers |
| `file_info.rs` | File reading & metadata extraction | `FileInfo` | Provides lines, encoding, truncation flags |
| `processor.rs` | Core orchestration for non-streaming modes | `Processor` | Applies mode logic (plain, highlight, json, summary) |
| `streaming.rs` | Chunked streaming JSON output | `StreamingProcessor` | Supports resume & checkpoints |
| `summarizer.rs` | Summary mode extraction | `Summarizer` | Extracts structural lines for AI context |
| `tokenizer.rs` | Token estimation & counting | `TokenCounter` | Model-specific heuristics; sampling for large files |
| `token_counter.rs` | Legacy/aux token counting logic | `TokenEstimation` | (Consolidation target) |
| `json_schema.rs` | Schema validation & retrieval | `JsonSchemaValidator` | Supplies schemas via `--get-schema` |
| `wizard.rs` | Interactive profile configuration | `ConfigurationWizard` | Reads/writes profile files |
| `error.rs` | Unified error handling | `BatlessError` | Error codes & messages |
| `streaming.rs` | Streaming JSON pipeline |  | (Listed above; critical path) |

## Data Flow (Typical Non-Streaming Mode)

```text
Args → ConfigManager (merge CLI + profiles) → FileInfo (read & limit) → Processor →
  ├─ Plain/Highlight: Formatter + Highlighter
  ├─ JSON: FileInfo + Summarizer (+ TokenCounter optional)
  └─ Summary: Summarizer
→ stdout (immediate flush)
```

## Data Flow (Streaming JSON Mode)

```text
Args → ConfigManager → StreamingProcessor
  ├─ Iterative chunk read (line/window)
  ├─ Per-chunk summarization (optional)
  ├─ Checkpoint state (if enabled)
  └─ Emit chunk JSON objects
→ stdout (newline-delimited / array semantics)
```

## Error Handling Strategy

- Central error type `BatlessError` with categories (config, io, schema, internal)
- Early validation in `ConfigManager` to fail-fast before heavy work
- User-facing messages colored and structured for clarity

## Extension Points (Planned v0.3.0)

| Future Extension | Candidate Hook | Considerations |
|------------------|----------------|----------------|
| Plugin analyzers | Post-FileInfo pre-output | Need stable trait & sandbox |
| Output formats | Processor / Formatter abstraction | Possibly dynamic registry |
| Token models | TokenCounter strategy trait | Versioned model metadata |
| AST extraction | New module (tree-sitter) | Avoid inflating core binary |

## Performance Considerations

- File reading limited by `--max-lines` & `--max-bytes` early
- Streaming mode prevents loading entire file
- Syntax definitions cached by syntect; startup dominated by minimal setup (<5ms typical)
- Token counting uses sampling for large files to bound cost

## Security & Safety

- No network calls; pure local processing
- Fuzz target: tokenizer (expanding set planned)
- Schema validation ensures JSON contract stability

## Planned Refactors (Post 0.2.4)

- Consolidate `token_counter.rs` & `tokenizer.rs`
- Introduce trait for output modes to reduce branching
- Abstract highlighter behind interface for future tree-sitter integration

## Open Questions

- Plugin sandbox model (WASM vs process isolation)
- Baseline for structured logging (tracing) without overhead
- Standardized internal event bus for metrics?

---

_Feedback & contributions welcome. This document will expand as v0.3.0 design progresses._
