#!/bin/bash
# Demo script to showcase batless functionality
# This demonstrates all the key features of batless including new enhancements

set -e

echo "🦇 Batless Demo Script - Enhanced Edition"
echo "========================================="
echo

# Build the project first
echo "📦 Building batless..."
cargo build --release
echo

# Set up the batless binary path
BATLESS="./target/release/batless"

echo "✨ Feature Demonstrations:"
echo

echo "0. List supported languages and themes:"
echo "---------------------------------------"
echo "Available languages (first 10):"
$BATLESS --list-languages | head -10
echo
echo "Available themes (first 5):"
$BATLESS --list-themes | head -5
echo

echo "1. Basic syntax highlighting (Rust code):"
echo "----------------------------------------"
$BATLESS src/main.rs --max-lines=10
echo

echo "2. Plain text mode (no colors):"
echo "--------------------------------"
$BATLESS src/main.rs --mode=plain --max-lines=5 --color=never
echo

echo "3. Enhanced JSON output mode with new fields:"
echo "---------------------------------------------"
$BATLESS examples/demo.py --mode=json --max-lines=3
echo

echo "4. Summary mode - AI-friendly code structure extraction:"
echo "--------------------------------------------------------"
echo "Python summary (functions, classes, imports only):"
$BATLESS examples/demo.py --mode=summary --max-lines=20
echo

echo "5. Summary mode with Rust code:"
echo "-------------------------------"
$BATLESS src/lib.rs --mode=summary --max-lines=15
echo

echo "6. JSON output with tokens (AI processing):"
echo "--------------------------------------------"
$BATLESS examples/demo.py --mode=json --include-tokens --max-lines=5
echo

echo "7. Language auto-detection (Python):"
echo "-------------------------------------"
$BATLESS examples/demo.py --max-lines=8
echo

echo "8. Explicit language specification:"
echo "------------------------------------"
$BATLESS Cargo.toml --language=toml --max-lines=6
echo

echo "9. Byte limiting demonstration:"
echo "--------------------------------"
echo "This is a test file with multiple lines
Each line has different content
Some lines are longer than others
Short line
This is another longer line with more content" >/tmp/batless_demo.txt

$BATLESS /tmp/batless_demo.txt --mode=plain --max-bytes=50
echo

echo "10. Different themes:"
echo "--------------------"
echo "Theme: base16-ocean.dark (default)"
$BATLESS src/main.rs --theme="base16-ocean.dark" --max-lines=5
echo

echo "Theme: InspiredGitHub"
$BATLESS src/main.rs --theme="InspiredGitHub" --max-lines=5
echo

echo "11. ANSI stripping:"
echo "------------------"
$BATLESS src/main.rs --strip-ansi --max-lines=3
echo

echo "12. Large file handling (streaming):"
echo "------------------------------------"
# Create a larger test file
for i in {1..100}; do
    echo "Line $i: This is line number $i with some content to make it longer"
done >/tmp/large_file.txt

echo "First 5 lines of 100-line file:"
$BATLESS /tmp/large_file.txt --mode=plain --max-lines=5
echo

echo "Byte-limited output:"
$BATLESS /tmp/large_file.txt --mode=plain --max-bytes=200
echo

echo "13. Enhanced JSON metadata extraction:"
echo "--------------------------------------"
echo "Enhanced file metadata with encoding, tokens, and summary:"
$BATLESS examples/demo.py --mode=json --include-tokens --summary --max-lines=5 | jq '{file, language, encoding, total_lines, total_bytes, truncated, tokens: (.tokens | length), summary_lines: (.summary_lines | length)}'
echo

echo "14. Summary with different languages (JavaScript):"
echo "--------------------------------------------------"
# Create a test JS file
cat >/tmp/test.js <<'EOF'
import React from 'react';
import { useState } from 'react';

// Regular comment
export function MyComponent() {
    const [count, setCount] = useState(0);
    console.log('debug info');
    return <div>Count: {count}</div>;
}

class MyClass {
    constructor(name) {
        this.name = name;
    }

    greet() {
        return `Hello, ${this.name}!`;
    }
}

export default MyClass;
EOF

$BATLESS /tmp/test.js --mode=summary --language=javascript
echo

echo "15. AI-friendly output (no blocking, clean format):"
echo "----------------------------------------------------"
echo "Perfect for AI assistants like Claude:"
$BATLESS src/lib.rs --max-lines=15 --color=auto
echo

echo "16. CI/CD safe usage:"
echo "---------------------"
echo "Non-blocking output for automated systems:"
$BATLESS README.md --mode=plain --max-lines=10 --color=never
echo

echo "17. Combined features demonstration:"
echo "------------------------------------"
echo "Summary + JSON + Tokens for comprehensive AI analysis:"
$BATLESS src/main.rs --mode=json --summary --include-tokens --max-lines=10 | jq '{file, language, summary_count: (.summary_lines | length), token_count: (.tokens | length), truncated}'
echo

echo "🎉 Enhanced Demo completed!"
echo
echo "🚀 New Features Demonstrated:"
echo "✅ --list-languages: Discover all supported languages"
echo "✅ --list-themes: Browse available syntax themes"
echo "✅ --mode=summary: Extract only important code structures (perfect for AI)"
echo "✅ --include-tokens: Add token arrays for AI processing"
echo "✅ Enhanced JSON output with encoding, syntax errors, and metadata"
echo "✅ Summary mode works across languages (Python, Rust, JavaScript, etc.)"
echo "✅ Performance optimizations with cached syntax/theme sets"
echo
echo "💡 Original Core Benefits:"
echo "✅ Always non-blocking (never hangs)"
echo "✅ Syntax highlighting for 100+ languages"
echo "✅ Multiple output formats (plain, highlight, JSON, summary)"
echo "✅ Smart limiting (lines and bytes)"
echo "✅ Memory efficient streaming"
echo "✅ AI assistant friendly"
echo "✅ CI/CD pipeline safe"
echo

# Cleanup
rm -f /tmp/batless_demo.txt /tmp/large_file.txt /tmp/test.js

echo "🤖 Perfect for AI Assistants:"
echo "• batless --mode=summary file.py    # Get code structure only"
echo "• batless --mode=json --include-tokens file.rs    # Full analysis data"
echo "• batless --list-languages    # Check supported languages"
echo
echo "Run 'batless --help' for all available options!"
