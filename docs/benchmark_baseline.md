# Performance Baseline Metrics

This document captures the baseline performance metrics for batless to detect performance regressions.

## Test Environment
- Date: 2025-07-29
- Version: 0.1.5
- Rust: 1.84.0
- Platform: macOS (Darwin 24.6.0)
- Build: Release optimized

## Startup Operations (µs/ns)

These operations should be fast and not load heavy syntax sets:

| Operation | Baseline Time | Notes |
|-----------|---------------|-------|
| list_languages | ~2.77 µs | Lists all available programming languages |
| list_themes | ~159 ns | Lists all available syntax themes |
| config_default | ~13.6 ns | Creates default configuration |
| config_load_with_precedence | ~2.94 µs | Loads config with file precedence |
| validate_theme | ~13.1 ns | Validates theme exists |
| validate_language | ~2.77 µs | Validates language exists |

## Configuration Operations (ns)

These test config validation performance:

| Operation | Baseline Time | Notes |
|-----------|---------------|-------|
| validate_default | ~2.44 ns | Validates default configuration |
| validate_with_limits | ~2.44 ns | Validates config with custom limits |
| validate_with_summary | ~2.42 ns | Validates config with summary mode |
| validate_with_tokens | ~2.42 ns | Validates config with token extraction |

## Performance Regression Thresholds

- **Critical regression**: >50% performance degradation
- **Major regression**: >25% performance degradation  
- **Minor regression**: >10% performance degradation

## Key Optimizations Implemented

1. **Lazy Loading**: Heavy syntax/theme sets only loaded when needed
2. **Cached Resources**: Using lazy_static for syntax/theme sets
3. **Delayed Validation**: Language/theme validation moved after config loading
4. **Streaming Architecture**: Line-by-line processing to avoid memory bloat

## Benchmark Commands

To reproduce these benchmarks:

```bash
# Run all startup operation benchmarks
cargo bench --bench performance startup_operations

# Run all config operation benchmarks  
cargo bench --bench performance config_operations

# Run specific benchmark
cargo bench --bench performance list_languages
```

## Regression Testing

Run benchmarks before and after changes:

```bash
# Capture baseline
cargo bench --bench performance > baseline.txt

# After changes, compare
cargo bench --bench performance > current.txt
# Manual comparison or use criterion's built-in comparison
```