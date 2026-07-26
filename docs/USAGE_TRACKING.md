# Usage Tracking for Developers

Two scripts in `scripts/` let you instrument `batless` invocations, capture structured logs, and analyse usage patterns — useful for understanding how AI assistants (Claude Code, Copilot, etc.) actually call `batless` in practice.

- **`scripts/batless-logger`** — a transparent shell wrapper that logs every call as NDJSON and delegates to the real binary
- **`scripts/batless-stats`** — a Python analyser that reads those logs and reports statistics

Nothing is sent anywhere. All data stays in `~/.batless/stats/`.

---

## How it works

```
Claude Code / shell
       │
       ▼
 batless-logger          ← intercepts the call
       │  └─ writes one JSON line to ~/.batless/stats/YYYY-MM-DD.ndjson
       │
       ▼
 real batless binary     ← stderr captured, exit code checked
       │
       ├─ on success → output passed through unchanged
       │
       └─ on failure → error entry appended to same log
                        structured report printed to stderr
                        (includes link to file a GitHub issue)
```

Each log line captures: timestamp, session ID, mode, profile (kept in the schema for backward compatibility with historical logs; always `null` since `--profile` was removed from batless), `--max-lines`, `--max-bytes`, flags, extra flags, file extensions, and filenames. On failure, a separate error entry is appended with the exit code and stderr message.

---

## Installation

**1. Make the scripts executable**

```bash
chmod +x scripts/batless-logger scripts/batless-stats
```

**2. Put the logger on your PATH before the real binary**

```bash
mkdir -p ~/bin
cp scripts/batless-logger ~/bin/batless
```

```bash
mkdir -p ~/bin
cp scripts/batless-stats ~/bin/batless-stats
```

**3. Point the logger at the real binary**

Add to your `~/.zshrc` or `~/.bashrc`:

```bash
export BATLESS_REAL="$HOME/.cargo/bin/batless"
export PATH="$HOME/bin:$PATH"
```

Reload your shell:

```bash
source ~/.zshrc
```

**4. Verify**

```bash
which batless          # should show ~/bin/batless
batless --version      # should work normally
ls ~/.batless/stats/   # log file appears after first call
```

---

## Log format

One NDJSON object per call, written to `~/.batless/stats/YYYY-MM-DD.ndjson`:

**Successful call:**

```json
{
  "ts": "2026-07-25T10:23:01Z",
  "session": "a3f1b2c4",
  "mode": "index",
  "profile": null,
  "max_lines": 100,
  "max_bytes": null,
  "flags": ["--with-line-numbers"],
  "extra_flags": ["--language"],
  "files": ["src/main.rs"],
  "file_count": 1,
  "file_exts": [".rs"]
}
```

**Failed call** (appended to the same file immediately after the invocation entry):

```json
{
  "ts": "2025-04-02T10:23:01Z",
  "session": "a3f1b2c4",
  "error": true,
  "exit_code": 1,
  "args": ["--mode=badmode", "src/main.rs"],
  "stderr": "error: invalid value 'badmode' for '--mode'"
}
```

Error entries are distinguished by `"error": true`. Normal stats analysis skips them; use `--errors` to query them separately.

### Session ID

The `session` field is taken from the `BATLESS_SESSION` environment variable if set, otherwise generated once per shell session from the current timestamp. To tag a Claude Code session distinctly, export it before launching:

```bash
export BATLESS_SESSION="claude-$(date +%s)"
```

---

## Analysing logs

Run `batless-stats` from anywhere (it reads `~/.batless/stats/` by default):

```bash
# Today's log
batless-stats

# Specific date
batless-stats --date 2025-04-01

# All logs combined
batless-stats --all

# Filter by session ID
batless-stats --session a3f1b2c4

# Machine-readable JSON (pipe to jq)
batless-stats --json | jq '.command_signatures'

# Error summary
batless-stats --errors

# Error summary as JSON
batless-stats --errors --json
```

### Unique command signatures

To see exactly which flag combinations were used — without filenames — sorted by frequency:

```bash
batless-stats --commands
```

Output:

```
  batless --mode=plain                                      401 84.4%
  batless --mode=index                                       50 10.5%
  batless --mode=json --with-line-numbers                     8  1.7%
```

(Real numbers from the telemetry that drove the "Option A" scope reduction — see `CHANGELOG.md` — `--mode=plain` and `--mode=index` account for essentially all real usage.)

Each line is a canonical signature with flags in stable order, a call count, and a percentage of total calls.

### Full stats output includes

| Section | What it shows |
|---|---|
| Output modes | Breakdown of `plain`, `json`, `index`, `default`, etc. |
| AI profiles | Vestigial — `--profile` was removed from batless; this section will always show `none: 100%` for new logs. Kept so historical pre-refactor logs still render |
| Output limiting | How often `--max-lines` / `--max-bytes` were set, and averages |
| Unique command signatures | Every distinct flag combination, count, percentage |
| Top flags | Individual flag frequency |
| File extensions | Which file types were viewed most |
| Most-viewed files | Filenames (basenames) by call count |
| Hourly distribution | UTC hour histogram |

`--errors` output includes:

| Section | What it shows |
|---|---|
| Exit codes | Breakdown of non-zero exit codes |
| Flag patterns at time of error | Which flag combinations triggered failures |
| Error messages | Most common stderr strings (truncated) |
| Hourly distribution | UTC hour histogram of errors |

---

## Environment variables

| Variable | Default | Purpose |
|---|---|---|
| `BATLESS_REAL` | auto-detected | Path to the real `batless` binary |
| `BATLESS_LOG_DIR` | `~/.batless/stats` | Directory where NDJSON logs are written |
| `BATLESS_SESSION` | generated | Session ID stamped on every log entry |

---

## Removing the wrapper

To stop logging, either remove `~/bin/batless` or reorder your `PATH` so the real binary comes first:

```bash
rm ~/bin/batless
```

The log files remain in `~/.batless/stats/` and can be deleted manually at any time.

---

## Tips for analysing Claude Code behaviour

- Set `BATLESS_SESSION` to a known value before each Claude Code session so you can isolate its calls from your own manual use.
- Use `--commands` to see which flag combinations Claude Code reaches for most — useful for validating that your `CLAUDE.md` instructions are being followed.
- Use `--json | jq` for scripted analysis across multiple sessions.
- Log files are plain NDJSON; you can query them directly with `jq` without using `batless-stats` at all:

```bash
# All modes used today
jq -r '.mode' ~/.batless/stats/"$(date +%Y-%m-%d)".ndjson | sort | uniq -c | sort -rn

# All unique signatures across all logs
cat ~/.batless/stats/*.ndjson | jq -r '[.profile // "", .mode, (.flags // []) | tostring] | join(" ")' | sort | uniq -c | sort -rn
```
