# PRD.md

## Product Requirements Document

**Project Name:** `batless` – A Minimal, AI-Safe Syntax Code Viewer
**Status:** Draft
**Author:** Thomas
**Target Users:** LLM-based code agents (e.g., Claude Code, Gemini), developers using automation, non-interactive CI pipelines.

---

## 1. Purpose

To build a fast, syntax-aware, non-interactive code viewer CLI for AI coding agents. Unlike traditional pagers (`less`, `more`) or developer-centric tools (`bat`), `batless` provides clean, syntax-highlighted output with no blocking, no decoration, and perfect predictability.

---

## 2. Problem Statement

Tools like `bat` often block on `less`, break in CI, or output too much formatting. Even with `--paging=never`, output is inconsistent across platforms. Claude and other code assistants fail or hang when pagers expect interaction.

---

## 3. Goals

- ✅ Always non-blocking and streaming
- ✅ Tree-sitter or Syntect-based syntax highlighting
- ✅ Clean, machine-readable output
- ✅ Optional JSON/token/tokenized output
- ✅ Extremely fast and small footprint

---

## 4. Features

### Required (MVP)

- `--plain`: Raw content, no decorations
- `--language=auto|<lang>`: Auto-detect or force syntax
- `--mode=highlight|plain|json`: Output format modes
- `--max-lines`, `--max-bytes`: Output limiting
- Streamed output (never loads entire file)

### Highlighting and Color Output

- ANSI syntax highlighting
- `--color=never` or `--strip-ansi` support
- Configurable theme (basic light/dark)

### Optional (Post-MVP)

- `--ast`: Emit syntax structure
- `--summary`: Show top-level code only
- `--llm-mode`: Safe defaults for agents

---

## 5. Non-Goals

- No interactive paging
- No Git integration or headers
- No line numbers unless explicitly requested

---

## 6. Users / Use Cases

| User          | Use Case                                         |
|---------------|--------------------------------------------------|
| Claude Code   | Show code in non-blocking view                   |
| CI systems    | Print source during failure/debugging            |
| Devs w/scripts| A safe, pretty `cat` for code output             |

---

## 7. Technical Architecture

- **Language:** Rust
- **Highlighting:** Tree-sitter or Syntect
- **Output:** Terminal ANSI, plain text, or JSON
- **CLI Parsing:** `clap`

---

## 8. Risks and Mitigations

| Risk                  | Mitigation                  |
|------------------------|-----------------------------|
| Blocked I/O            | Force streaming + buffering |
| Memory overload        | Enforce limits              |

---

## 9. Milestones

| Milestone           | Date      |
|---------------------|-----------|
| MVP core viewer     | Week 1    |
| Syntax highlighting | Week 2    |
| JSON support        | Week 3    |
| Claude test         | Week 4    |
| v0.1 release        | Week 5    |

---

## 10. Inspiration

- [`sharkdp/bat`](https://github.com/sharkdp/bat) — inspiration but heavier
- Claude & Gemini — agents that need clean, controlled output
- Unix tools like `cat`, `less`, and `ripgrep`

---

## 11. License

MIT or Apache 2.0

---

## 12. Future Ideas

- `--format=json+tokens`
- WASM/HTTP mode for web use
- Claude plugin wrapper
