# batless Architecture Overview

Last updated: 2026-07-25

## Goals

Provide a high-level, accurate map of the modules under `src/`, how a file
turns into output, and where the CLI surface is defined. This is an internal
reference for contributors, not marketing copy — it should be kept in sync
with `src/` rather than describing aspirational or historical design.

## Core Principles

- Non-blocking output — never pages, never waits on user input
- Minimal state and memory footprint (bounded reads via `--max-lines` /
  `--max-bytes`)
- Deterministic, testable transformations
- Clear separation of concerns: argument parsing → config → file processing →
  formatting

## Module Map

| Module | Purpose | Key Types / Functions |
|--------|---------|------------------------|
| `lib.rs` | Public library API surface used by both `main.rs` and the test suite | Re-exports `BatlessConfig`, `BatlessError`/`BatlessResult`, `FileInfo`, `OutputFormatter`/`OutputMode`, `LanguageDetector`, `FileProcessor`, `SummaryLevel`; thin `process_file`/`detect_language`/`list_languages`/`format_output` wrappers |
| `main.rs` | CLI entry point | `main`, `run`, `handle_special_commands`, `handle_directory_index`, `handle_normal_processing` |
| `config_manager.rs` | Defines the entire CLI surface and merges it into a `BatlessConfig` | `Args` (clap `Parser`), `ConfigManager`, `CliOutputMode`, `ColorMode`, `Shell` |
| `config.rs` | Serializable runtime configuration | `BatlessConfig` (builder-style `with_*` methods, TOML/JSON load & save, config-file discovery/precedence) |
| `config_validation.rs` | Standalone validation rules for `BatlessConfig` | `validate_config`, `validate_max_lines`, `validate_max_bytes`, `validate_language`, `validate_limits_combination`, `validate_schema_version` |
| `error.rs` | Unified error type and process exit codes | `BatlessError`, `ErrorCode`; Levenshtein-based "did you mean" suggestions for file-not-found / language-not-found |
| `language.rs` | Extension/filename-based language detection | `LanguageDetector` — static extension map plus extensionless-filename cases (`Dockerfile`, `Makefile`, `Gemfile`, etc.). No syntect dependency; detection is metadata-only, not a highlighting engine |
| `file_info.rs` | Intermediate result of reading a file | `FileInfo` (lines, total_lines/bytes, exactness, language, encoding, truncation flags, optional `syntax_errors`/`compression_ratio`) |
| `processor.rs` | Core file-reading pipeline | `FileProcessor::process_file`, `process_stdin`, `detect_encoding`, `read_file_content`, `strip_content_lines` |
| `formatter.rs` | Output-mode dispatcher and `OutputMode` enum | `OutputFormatter::format_output` (dispatches to `formatters::*`), `OutputMode { Plain, Json, Index }` |
| `formatters/mod.rs` | `Formatter` trait shared by all formatters | `trait Formatter { fn format(...); fn output_mode(...); }` |
| `formatters/plain_formatter.rs` | `--mode=plain` (default) | `PlainFormatter` — joins lines with `\n`; adds `cat -n`/`cat -b`-style numbering when `-n`/`-b` is set |
| `formatters/json_formatter.rs` | `--mode=json` | `JsonFormatter` — serializes `FileInfo` to the documented JSON schema; `--with-line-numbers` and `--json-pretty` controlled here |
| `formatters/index_formatter.rs` | `--mode=index` | `IndexFormatter` — builds the symbol table by calling into `summarizer.rs`/`summary.rs` |
| `formatters/error_formatter.rs` | Alternate error-rendering helper used internally/by tests | `ErrorFormatter` |
| `summary.rs` | Extraction-detail knob used internally by `IndexFormatter` | `SummaryLevel { None, Minimal, Standard, Detailed }` |
| `summarizer.rs` | Regex/heuristic structure-extraction engine, zero external parsing dependencies (no tree-sitter) | `SummaryExtractor::extract_summary`, per-language `is_*_summary_worthy` predicates (Python, Rust, JS/TS, Java, C/C++, Go, Ruby, PHP, Swift, Kotlin, Scala, Haskell, Clojure, Elixir, Erlang, generic fallback) |
| `summary_item.rs` | Element type produced by `summarizer.rs` | `SummaryItem { line, line_number, end_line, kind }` |
| `traits.rs` | Interfaces for decoupling/testability | `trait LanguageDetection`, `trait SummaryExtraction` |

## Data Flow

```text
Args (config_manager.rs, clap)
  → ConfigManager: load config file (or precedence-based discovery) + apply CLI args
  → BatlessConfig (config.rs) + OutputMode
  → FileProcessor::process_file (processor.rs)
      - reads the file (or stdin via "-"), enforcing --max-lines/--max-bytes incrementally
      - detects encoding (UTF-8 sample, falls back to Windows-1252/ISO-8859-15/UTF-16)
      - detects language (config override, else LanguageDetector by extension/filename)
      - optionally strips comment-only and/or blank lines (--strip-comments/--strip-blank-lines)
  → FileInfo (file_info.rs)
  → OutputFormatter::format_output (formatter.rs) dispatches on OutputMode to:
      - PlainFormatter   (formatters/plain_formatter.rs)
      - JsonFormatter    (formatters/json_formatter.rs)
      - IndexFormatter   (formatters/index_formatter.rs)
  → stdout
```

`main.rs` prints the formatted result directly for a single file
(`handle_normal_processing`). When `--mode=index` is given a *directory*
instead of a file, `handle_directory_index` walks it recursively (skipping
symlinks and hidden directories), runs the same `process_file` →
`format_output` pipeline per file, and emits one compact JSON object per
line — NDJSON — instead of a single response.

Before any of this runs, `main.rs` pre-scans `argv` for known unsupported
flags (`--pattern`, `--list`, `--range`) and prints a "use this other tool
instead" message, exiting before clap parsing — this is the mechanism behind
the "What batless IS NOT" guidance in `docs/PHILOSOPHY_AND_SCOPE.md`.

## The Symbol-Extraction Engine (`summary.rs` + `summarizer.rs`)

`SummaryLevel` and `SummaryExtractor` still exist and are used internally by
`IndexFormatter` to build the `--mode=index` symbol table (functions,
classes, structs, with line ranges). This is **not** the old AI-oriented
`--mode=summary`/`--summary` CLI feature, which was removed — it is the
regex/heuristic extraction engine that `--mode=index`'s symbol table is built
from, called with a fixed `SummaryLevel::Detailed`. `IndexFormatter` then
post-processes each `SummaryItem` to derive a symbol `name` (stripping
visibility/keyword prefixes and taking the leading identifier) and a
`visibility` (Rust: `pub`/`pub(crate)`/`pub(super)`/private; JS/TS:
export/local).

## Error Handling

- Central error type `BatlessError` (`error.rs`) covers file I/O, language
  detection, encoding, configuration, and (de)serialization failures, each
  mapped to an `ErrorCode` used as the process exit code
- `FileNotFound`/`LanguageNotFound` errors carry Levenshtein-distance-based
  "did you mean" suggestions
- `ConfigManager` validates configuration (`config_validation.rs`) before any
  file I/O happens, so invalid input fails fast
- `--mode=json` errors are rendered as structured JSON via
  `OutputFormatter::format_error`; other modes get a plain-text message

## Performance Considerations

- Reading is bounded early by `--max-lines` and `--max-bytes`, enforced
  incrementally while reading (not after loading the whole file)
- No syntax-highlighting engine and no tree-sitter parsing in the binary —
  language detection is a static extension/filename lookup, and structure
  extraction (`summarizer.rs`) is regex/heuristic-based
- No streaming/checkpointing mode; `--mode=index` over a directory still
  processes and emits one file at a time (NDJSON), but each file is fully
  read before its line is written

## Security & Safety

- No network calls; pure local file processing
- `fuzz/` contains a `cargo-fuzz`/`libfuzzer-sys` harness scaffold
  (`fuzz/fuzz_targets/fuzz_target_1.rs`); currently a no-op stub, not wired
  to a specific parser
- Directory walking for `--mode=index` uses `symlink_metadata` and skips
  symlinks entirely to avoid directory-traversal cycles

---

*This document reflects `src/` as of the commit that removed tree-sitter/AST
mode, streaming, JSON schema validation, AI profiles/token estimation, and
`--mode=summary`/`--hash` (see `git log --grep='refactor!'`). Update it
alongside any future module additions or removals.*
