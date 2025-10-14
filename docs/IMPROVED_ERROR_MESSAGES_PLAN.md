# Improved Error Messages Implementation Plan

## Goal

Enhance user experience by providing helpful error messages with actionable suggestions when users attempt to use non-existent features.

## Current State

**Before**:
```
Error: unexpected argument '--pattern' found
```

Users get confused and have to:
1. Search documentation
2. Try different approaches
3. Eventually find grep/rg

## Proposed Enhancement

### Design Principles

1. **Be helpful, not condescending**
2. **Provide immediate alternatives**
3. **Keep messages concise**
4. **Show concrete examples**
5. **Explain *why* feature doesn't exist**

## Implementation Examples

### Error: `--pattern` Not Found

```rust
// In src/error.rs or CLI argument parser

if args.contains("--pattern") || args.contains("-p") {
    eprintln!("Error: batless doesn't support pattern searching");
    eprintln!();
    eprintln!("💡 Tip: Use dedicated search tools:");
    eprintln!("     grep -rn \"pattern\" src/");
    eprintln!("     rg \"pattern\" src/          # even faster!");
    eprintln!();
    eprintln!("   Then view results with batless:");
    eprintln!("     batless $(grep -l \"pattern\" src/*)");
    eprintln!();
    eprintln!("Why? batless focuses on viewing files. grep/rg are");
    eprintln!("optimized for searching. Use the best tool for each job!");
    std::process::exit(1);
}
```

**Output**:
```
Error: batless doesn't support pattern searching

💡 Tip: Use dedicated search tools:
     grep -rn "pattern" src/
     rg "pattern" src/          # even faster!

   Then view results with batless:
     batless $(grep -l "pattern" src/*)

Why? batless focuses on viewing files. grep/rg are
optimized for searching. Use the best tool for each job!
```

### Error: `--list` Not Found

```rust
if args.contains("--list") || args.contains("-l") {
    eprintln!("Error: batless doesn't list files");
    eprintln!();
    eprintln!("💡 Tip: Use file listing tools:");
    eprintln!("     ls -la src/");
    eprintln!("     find . -name \"*.py\"");
    eprintln!("     fd -e rs                  # modern alternative");
    eprintln!("     tree src/                 # tree view");
    eprintln!();
    eprintln!("   Then view files with batless:");
    eprintln!("     fd -e py | xargs batless");
    eprintln!();
    eprintln!("Why? batless views individual files. Use ls/find/fd/tree");
    eprintln!("for file discovery, then pipe to batless for viewing.");
    std::process::exit(1);
}
```

### Error: `--range` or `-r` Not Found

```rust
if args.contains("--range") || args.contains("-r") {
    eprintln!("Error: batless doesn't support line ranges");
    eprintln!();
    eprintln!("💡 Tip: Use these alternatives:");
    eprintln!("     sed -n '10,50p' file.py | batless --language=python");
    eprintln!("     head -50 file.py | tail -41 | batless");
    eprintln!();
    eprintln!("   Or use batless with limiting:");
    eprintln!("     batless --max-lines=100 file.py");
    eprintln!();
    eprintln!("Note: Line range support may be added in a future version.");
    eprintln!("See: https://github.com/docdyhr/batless/issues/57");
    std::process::exit(1);
}
```

## Technical Implementation

### Location: `src/main.rs`

Add early detection before clap parsing:

```rust
fn main() {
    // Check for common misconceptions before parsing
    let args: Vec<String> = std::env::args().collect();

    // Check for --pattern
    if args.iter().any(|a| a == "--pattern" || a == "-p") {
        print_pattern_not_supported();
        std::process::exit(1);
    }

    // Check for --list
    if args.iter().any(|a| a == "--list" || a == "-l") {
        print_list_not_supported();
        std::process::exit(1);
    }

    // Check for --range (but not -r from existing flags)
    if args.iter().any(|a| a == "--range" || a.starts_with("-r=") || a.starts_with("-r ")) {
        print_range_not_supported();
        std::process::exit(1);
    }

    // Continue with normal parsing
    if let Err(e) = run() {
        print_error(&e);
        std::process::exit(e.error_code() as i32);
    }
}

fn print_pattern_not_supported() {
    // Implementation from above
}

fn print_list_not_supported() {
    // Implementation from above
}

fn print_range_not_supported() {
    // Implementation from above
}
```

### Alternative: Extend Error Type

```rust
// In src/error.rs
pub enum BatlessError {
    // Existing variants...

    /// User tried to use unsupported feature
    UnsupportedFeature {
        feature: String,
        alternative: String,
        example: String,
        reasoning: String,
    },
}

impl BatlessError {
    pub fn pattern_not_supported() -> Self {
        BatlessError::UnsupportedFeature {
            feature: "Pattern searching (--pattern)".to_string(),
            alternative: "Use grep or ripgrep".to_string(),
            example: "grep -rn \"pattern\" src/".to_string(),
            reasoning: "batless focuses on viewing files. grep/rg are optimized for searching.".to_string(),
        }
    }
}
```

## Add `--help-examples` Command

```rust
// In Args struct
#[derive(Parser)]
pub struct Args {
    // Existing fields...

    /// Show common usage examples
    #[arg(long)]
    help_examples: bool,
}

fn handle_help_examples() {
    println!("BATLESS - Common Usage Examples\n");

    println!("📄 VIEWING FILES");
    println!("  batless file.py");
    println!("  batless -n file.py                # with line numbers");
    println!("  batless --theme=\"Monokai\" file.py");
    println!();

    println!("🔍 SEARCHING THEN VIEWING");
    println!("  grep -l \"TODO\" src/*.rs | xargs batless");
    println!("  rg -l \"async fn\" | xargs batless -n");
    println!("  fd -e py | xargs batless --mode=summary");
    println!();

    println!("📏 LINE RANGES");
    println!("  sed -n '10,50p' file.py | batless --language=python");
    println!("  head -100 file.py | batless");
    println!();

    println!("📂 FILE LISTING");
    println!("  ls src/*.rs | xargs batless --mode=summary");
    println!("  find . -name \"*.py\" -exec batless {{}} \\;");
    println!();

    println!("🤖 AI WORKFLOWS");
    println!("  batless --profile=claude src/main.rs");
    println!("  batless --mode=json --include-tokens file.py | jq");
    println!("  batless --mode=summary --max-lines=200 *.rs");
    println!();

    println!("⚡ PIPELINES");
    println!("  cat file.py | batless --language=python");
    println!("  git diff | batless --language=diff");
    println!("  curl -s https://example.com | batless --language=html");
    println!();

    println!("For more: batless --help");
}
```

## Testing

### Manual Testing

```bash
# Test error messages
batless --pattern "test" src/
batless --list src/
batless -r 10:50 file.py

# Test help examples
batless --help-examples
```

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_pattern_argument_shows_helpful_error() {
        // Capture stderr
        // Verify helpful message appears
    }
}
```

## Documentation Updates

### Update CLAUDE.md

Add section about error messages:

```markdown
### Helpful Error Messages

batless provides guidance when you try unsupported features:

```bash
$ batless --pattern "TODO" src/
Error: batless doesn't support pattern searching

💡 Tip: Use dedicated search tools:
     grep -rn "pattern" src/
```

This ensures you quickly learn the right tool for each task.
```

### Update README.md

Add "Common Mistakes" section:

```markdown
## Common Mistakes

**Looking for pattern search?**
```bash
# ❌ batless --pattern "TODO"  # Doesn't exist
# ✅ Use grep instead
grep -rn "TODO" src/ | head -20
```

**Want to list files?**
```bash
# ❌ batless --list src/  # Doesn't exist
# ✅ Use ls/find/fd
fd -e rs | xargs batless
```
```

## Rollout Plan

### Phase 1: Core Implementation (v0.3.1)
- [ ] Implement detection for --pattern, --list, --range
- [ ] Add helpful error messages
- [ ] Add --help-examples command
- [ ] Test on macOS, Linux, Windows

### Phase 2: Documentation (v0.3.1)
- [ ] Update CLAUDE.md with error message info
- [ ] Update README.md with common mistakes
- [ ] Add examples to --help output
- [ ] Create cookbook in docs/

### Phase 3: Metrics (v0.3.2+)
- [ ] Log (anonymously) which "wrong" flags users try
- [ ] Measure reduction in confusion
- [ ] Gather feedback on error messages

## Success Metrics

**Before**: Users see "argument not found" and search for solutions
**After**: Users see helpful message and immediately know what to use

**Target**: 90%+ of users find the right tool on first error

## Alternative Approaches

### Option 1: Clap Custom Error Messages
Use clap's error handling to customize messages.

**Pros**: Integrated with parser
**Cons**: Harder to customize per-flag

### Option 2: Pre-Flight Validation
Check args before clap parsing (recommended above).

**Pros**: Full control, clearer errors
**Cons**: Duplicate some validation logic

### Option 3: Wrapper Script
Create `batless-helper` script that detects and suggests.

**Pros**: No Rust changes needed
**Cons**: Adds dependency, users bypass it

**Recommendation**: Option 2 (Pre-Flight Validation)

## Code Size Estimate

- Error detection: ~50 lines
- Error messages: ~150 lines (50 per feature)
- Help examples: ~100 lines
- Tests: ~50 lines
- **Total**: ~350 lines

**Impact**: Massive UX improvement for minimal code

## Timeline

- Implementation: 2-3 hours
- Testing: 1 hour
- Documentation: 1 hour
- **Total**: 1 dev day

---

*Ready for implementation in v0.3.1*
