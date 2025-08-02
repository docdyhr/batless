
# 🥊 `batless` vs `bat`: Feature Comparison

This section compares [`batless`](https://github.com/docdyhr/batless) with [`bat`](https://github.com/sharkdp/bat), the popular Rust-based `cat` alternative. While both tools offer syntax highlighting via `syntect`, they serve different goals.

## 🧠 Philosophy

- **`bat`** is an opinionated, feature-rich `cat` replacement with decorations, Git integration, and paging.
- **`batless`** is minimal by design — a fast, zero-blocking syntax highlighter for modern developer workflows and AI assistants.

## ⚙️ Feature Matrix

| Feature / Aspect           | `bat`                                                                 | `batless`                                                                                 |
|----------------------------|------------------------------------------------------------------------|--------------------------------------------------------------------------------------------|
| **Syntax Highlighting**    | ✅ Yes — via `syntect`                                                 | ✅ Yes — via `syntect`                                                                     |
| **Stdin Support**          | ✅ Yes — via `bat -`                                                   | ✅ Yes — automatic and seamless                                                            |
| **Git Integration**        | ✅ Yes — Git diff indicators                                           | ❌ No                                                                                       |
| **Paging**                 | ✅ Yes — uses `less` (may block)                                       | ❌ Never — **always streams**                                                              |
| **Line Numbers**           | ✅ Optional                                                            | ❌ None                                                                                     |
| **File Headers / Decorations** | ✅ Filename, grid lines                                          | ❌ No distractions                                                                          |
| **Themes**                 | ✅ Configurable                                                        | ❌ Monokai Extended only                                                                   |
| **Binary File Detection**  | ✅ Blocks binary output                                                | ✅ Streams everything — including binary                                                   |
| **Blocking Behavior**      | ❌ May block on large input or pager                                   | ✅ Never blocks — ideal for scripts, TUI, AI tools                                         |
| **Best Use Case**          | Pretty CLI file viewer                                                | Blazing-fast CLI tool for automation, AI dev tools, logs, CI                               |
| **Dependencies**           | `less`, optionally `git`                                               | ❌ None                                                                                     |
| **Binary Size**            | 🐘 Large                                                              | 🪶 Tiny                                                                                     |
| **Custom Configs**         | ✅ `bat --config`, style, themes                                       | ❌ Zero config — it just works                                                             |

## 🧪 Example Usage

**Using `bat`:**
```bash
echo "fn main() {}" | bat -l rs --paging=never
```

**Using `batless`:**
```bash
echo "fn main() {}" | batless
```

## 🧭 When to Use `batless`

- ✅ You want **speed** and **streaming output**.
- ✅ You need a tool that **never blocks**, even on large files or in scripts.
- ✅ You build tools for **AI assistants**, **logs**, **CI**, **TTY UIs**, or **scripting pipelines**.
- ✅ You prefer **minimal output** without headers or noise.

## ✅ TL;DR

| Situation                                  | Tool       |
|-------------------------------------------|------------|
| Viewing a file with Git info              | `bat`      |
| Interactive file browsing with metadata   | `bat`      |
| High-speed piping, scripting, and tooling | **`batless`** |
| CI logs, AI tools, non-blocking viewers   | **`batless`** |

---

> `batless` is purpose-built for **non-blocking, lightning-fast syntax highlighting** in modern dev environments. No paging. No config. No delays. Just raw speed and clean output.
