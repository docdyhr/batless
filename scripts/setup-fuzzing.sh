#!/usr/bin/env bash
set -euo pipefail

# Bootstrap cargo-fuzz for the project if not already initialized.
# Adds initial fuzz targets for tokenizer and streaming processor.

if ! command -v cargo-fuzz >/dev/null 2>&1; then
  echo "Installing cargo-fuzz..."
  cargo install cargo-fuzz
fi

if [ ! -d fuzz ]; then
  echo "Initializing cargo-fuzz workspace..."
  cargo fuzz init
fi

# Create tokenizer fuzz target if missing
if [ ! -f fuzz/fuzz_targets/tokenizer.rs ]; then
  cat > fuzz/fuzz_targets/tokenizer.rs <<'EOF'
#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(text) = std::str::from_utf8(data) {
        let _ = batless::tokens::TokenExtractor::extract_tokens(text, "fuzz_input.rs");
    }
});
EOF
  echo "✅ Added tokenizer fuzz target"
fi

# Create streaming fuzz target (placeholder) if missing
if [ ! -f fuzz/fuzz_targets/streaming.rs ]; then
  cat > fuzz/fuzz_targets/streaming.rs <<'EOF'
#![no_main]
use libfuzzer_sys::fuzz_target;

// Placeholder until streaming API exposed more cleanly
fuzz_target!(|data: &[u8]| {
    // Intentionally lightweight: ensure no panics on arbitrary UTF-8
    let _ = std::str::from_utf8(data);
});
EOF
  echo "✅ Added streaming fuzz target"
fi

echo "🎯 Fuzzing setup complete. Run with: cargo fuzz run tokenizer"
