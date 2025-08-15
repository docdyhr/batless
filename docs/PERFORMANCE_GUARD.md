# Performance Guard

This document describes the performance regression guard integrated in v0.2.4.

## Overview

The guard enforces relative performance stability for critical micro-benchmarks using the Criterion-backed bench `performance` and a custom script `scripts/check_performance.sh`.

Features:

- Median calculation across multiple sample runs per benchmark.
- JSON baseline (`benchmark_baseline.json`) plus legacy text baseline for backward compatibility.
- Configurable warn / fail thresholds via env vars (default WARN 15%, FAIL 25%).
- Machine-readable summary (`performance_summary.json`) and Markdown summary.
- Distinct exit codes: 0 pass, 50 warn, 1 fail.
- PR comment with formatted table.

## Benchmarks Tracked

- `startup_operations`: Measures initial startup & minimal file processing.
- `config_operations`: Exercises configuration parsing and profile handling.

## Workflow Integration

File: `.github/workflows/performance-check.yml`

Triggers:

- Pull Requests touching core code or benches.
- Pushes to `main`.

Artifacts:

- `benchmark_baseline.json` (baseline medians)
- `performance_summary.json` / `.md`
- Criterion raw reports under `target/criterion/`

## Baseline Management

On first run without a JSON baseline the script bootstraps one and exits successfully (status `bootstrap`). Subsequent runs compare current medians to baseline.

To refresh baseline intentionally:

1. Merge stabilized performance improvements to `main`.
2. Manually delete baseline files in a PR OR run the separate baseline workflow (`performance-baseline.yml`) if still present.
3. Merge; new baseline bootstraps.

## Threshold Customization

Environment variables (set at workflow or step level):

```bash
SAMPLES=5          # Number of sample runs per benchmark
WARN_THRESHOLD=15  # % over baseline for warning
FAIL_THRESHOLD=25  # % over baseline for failure
```

Example override in a workflow step:

```bash
run: |
  SAMPLES=7 WARN_THRESHOLD=10 FAIL_THRESHOLD=20 ./scripts/check_performance.sh
```

## Adding a New Benchmark

1. Implement benchmark function in `benches/performance.rs` using Criterion.
2. Add the benchmark name to `BENCHES` array in `scripts/check_performance.sh`.
3. Run locally: `./scripts/check_performance.sh` (first run bootstraps baseline).
4. Commit updated baseline if meaningful.

## Interpreting Results

- `pass`: All benchmarks within warn threshold.
- `warn`: Some > WARN% but ≤ FAIL% (non-blocking; review optimization).
- `fail`: One or more > FAIL% (block merge unless justified).
- `bootstrap`: New baseline created; next run enforces.

## Local Usage

```bash
./scripts/check_performance.sh
cat performance_summary.md
```

If you see `⚠️  No JSON baseline ... bootstrapping`, run again for enforcement.

## Known Limitations

- Does not account for cross-platform variability (Linux-only CI host currently).
- Median only; p95/p99 variability not tracked (future enhancement).
- Baseline drift requires manual discipline (document justification in PR when resetting).

## Planned Enhancements

- Historical trend storage (e.g., commit-sharded JSON archive).
- p95 latency capture.
- Automatic baseline update on explicit PR label (e.g., `perf-baseline-ok`).
- Separate Windows/macOS baseline pipelines.

---
Maintainers: keep this file updated when adding benchmarks or changing thresholds.
