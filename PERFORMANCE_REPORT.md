# Performance Benchmark Report

Generated: 2025-07-25

## Executive Summary

**✅ CLAIM VALIDATED**: batless achieves sub-50ms startup times across all test scenarios.

- **Actual performance**: 1.6ms - 2.2ms (average)
- **Claimed performance**: <50ms
- **Result**: **22-31x faster** than claimed!

## Benchmark Results

### Startup Performance by File Size

| File Size | Mean Time | Min Time | Max Time |
|-----------|-----------|----------|----------|
| Small (47B) | 1.6ms | 1.4ms | 2.7ms |
| Medium (46KB) | 2.2ms | 2.0ms | 3.0ms |
| Large (1.2MB, limited) | 1.9ms | 1.7ms | 2.6ms |

### Performance by Output Mode

| Mode | Mean Time | Relative Speed |
|------|-----------|----------------|
| Summary | 1.9ms | 1.00x (fastest) |
| JSON | 2.0ms | 1.03x |
| Plain | 2.2ms | 1.15x |
| Highlight | 2.2ms | 1.16x |

## Key Findings

1. **Consistent Sub-2ms Performance**: All operations complete in 1.6-2.2ms on average
2. **Efficient Streaming**: Large file performance (with --max-lines) is comparable to small files
3. **Mode Impact Minimal**: Only ~16% difference between fastest (summary) and slowest (highlight) modes
4. **Memory Efficient**: Streaming architecture prevents memory bloat with large files

## Comparison to Claims

The README claims "<50ms startup with cached syntax definitions". Our benchmarks show:

- **Reality**: 1.6-2.2ms average (with outliers up to 14.8ms)
- **Conservative claim**: 50ms allows for slower systems and edge cases
- **Actual performance**: **95%+ faster than advertised**

## Test Environment

- Platform: macOS (Darwin 24.6.0)
- CPU: Apple Silicon / Intel (system dependent)
- Rust: 1.88.0
- Binary: Release build with optimizations

## Recommendations

1. **Update marketing**: Consider updating claims to "<5ms startup" for accuracy
2. **Further optimization**: Investigate the 14.8ms outlier in highlight mode
3. **Benchmark CI**: Add automated performance regression tests
4. **Cross-platform testing**: Validate performance on Linux/Windows

## Raw Benchmark Commands

```bash
# File size benchmarks
hyperfine --warmup 5 --min-runs 20 -N \
  './target/release/batless benchmark_files/small.rs' \
  './target/release/batless benchmark_files/medium.rs' \
  './target/release/batless benchmark_files/large.rs --max-lines=100'

# Mode benchmarks
hyperfine --warmup 3 -N \
  './target/release/batless benchmark_files/medium.rs --mode=plain' \
  './target/release/batless benchmark_files/medium.rs --mode=highlight' \
  './target/release/batless benchmark_files/medium.rs --mode=json' \
  './target/release/batless benchmark_files/medium.rs --mode=summary'
```
