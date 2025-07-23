# Performance Baseline

This document establishes performance baselines for batless v0.1.0 to track performance regressions over time.

## Benchmark Environment

- **Date**: January 2025
- **System**: macOS
- **Rust Version**: (run `rustc --version`)
- **CPU**: (run `sysctl -n machdep.cpu.brand_string` on macOS)
- **Binary Size**: 2.5MB (release build)

## Performance Baselines

### File Processing Throughput

| File Size | Time (µs) | Throughput (MiB/s) |
|-----------|-----------|-------------------|
| 1KB       | 27.3      | 31.5              |
| 10KB      | 49.4      | 173.7             |
| 100KB     | 266.9     | 321.6             |

### Syntax Highlighting Performance

| Language | Time (µs) | Notes |
|----------|-----------|-------|
| Rust     | 30.7      | Complex syntax |
| Python   | 34.3      | Moderate complexity |
| JSON     | 13.0      | Simple syntax |
| Plain    | 2.7       | No highlighting |

### Feature Performance

| Feature | Time (µs) | Description |
|---------|-----------|-------------|
| Summary Mode (enabled) | 60.6 | Extract important lines |
| Summary Mode (disabled) | 34.8 | Standard processing |

### Line Limiting Performance

| Line Limit | Time (µs) | Processing Rate |
|------------|-----------|-----------------|
| 100        | 27.8      | Fast early termination |
| 1,000      | 60.3      | Moderate processing |
| 5,000      | 208.8     | Larger file handling |
| 10,000     | 397.1     | Maximum typical use |

## Key Performance Characteristics

1. **Streaming Architecture**: Memory usage remains constant regardless of file size
2. **Early Termination**: Line/byte limits provide fast processing for large files
3. **Cached Resources**: Syntax and theme sets are loaded once and reused
4. **Linear Scaling**: Performance scales linearly with file size

## Monitoring Guidelines

1. Run benchmarks before major releases:
   ```bash
   cargo bench
   ```

2. Compare results against these baselines
3. Investigate any regression > 10%
4. Update baselines when intentional performance changes are made

## Optimization Opportunities

Based on current benchmarks:
- Summary mode adds ~75% overhead (60.6µs vs 34.8µs)
- Language detection is fast and cached
- Plain mode is 10x faster than highlighted mode
- Throughput ranges from 31-321 MiB/s depending on file size

## Notes

- Benchmarks use Criterion.rs for statistical analysis
- Results include outlier detection and variance measurement
- All benchmarks run on release builds with optimizations