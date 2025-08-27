# Feature Proposal: Line Range Selection

## Proposal

Add `--lines=START:END` flag for selecting specific line ranges, compatible with streaming architecture.

## Rationale

- Frequently requested by users familiar with `sed -n '10,50p'`
- Fits batless streaming model (can skip/stop at boundaries)
- Useful for AI context windows (extract specific sections)

## Implementation Approach

```rust
// Add to CLI args
#[arg(long, value_name = "START:END", help = "Display lines from START to END")]
lines: Option<String>,

// Parse in main
if let Some(range) = args.lines {
    let (start, end) = parse_range(&range)?;
    // Apply during streaming
}
```

## Compatibility

- Works with all output modes (plain, json, summary)
- Combines with existing `--max-lines` (whichever limit hits first)
- Maintains streaming efficiency (no full file load)

## Decision: Consider for v0.3.0

- Not urgent (workarounds exist)
- Fits philosophy (streaming-friendly)
- Moderate implementation effort
