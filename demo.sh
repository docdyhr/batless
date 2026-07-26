#!/bin/bash
# Demo script to showcase batless functionality
# batless is a fast, non-blocking cat/bat alternative — never pages, never blocks.

set -e

echo "🦇 Batless Demo Script"
echo "======================="
echo

# Build the project first
echo "📦 Building batless..."
cargo build --release
echo

# Set up the batless binary path
BATLESS="./target/release/batless"

echo "✨ Feature Demonstrations:"
echo

echo "0. List supported languages:"
echo "-----------------------------"
echo "Available languages (first 10):"
$BATLESS --list-languages | head -10
echo

echo "1. Plain text mode (default, no colors):"
echo "------------------------------------------"
$BATLESS src/main.rs --max-lines=10
echo

echo "2. Explicit plain mode with color disabled:"
echo "-----------------------------------------------"
$BATLESS src/main.rs --mode=plain --max-lines=5 --color=never
echo

echo "3. JSON output mode:"
echo "----------------------"
$BATLESS examples/demo.py --mode=json --max-lines=3
echo

echo "4. Index mode - machine-readable symbol table (Python):"
echo "-----------------------------------------------------------"
$BATLESS examples/demo.py --mode=index
echo

echo "5. Index mode with Rust code:"
echo "---------------------------------"
$BATLESS src/lib.rs --mode=index
echo

echo "6. Line-numbered JSON output:"
echo "---------------------------------"
$BATLESS examples/demo.py --mode=json --with-line-numbers --max-lines=5
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

$BATLESS /tmp/batless_demo.txt --mode=plain --max-lines=5 --max-bytes=50
echo

echo "10. cat -n / cat -b compatibility (requires --plain):"
echo "-----------------------------------------------------"
echo "Line numbers (-n):"
$BATLESS -n --plain /tmp/batless_demo.txt
echo
echo "Non-blank line numbers (-b):"
$BATLESS -b --plain /tmp/batless_demo.txt
echo

echo "11. ANSI stripping:"
echo "------------------"
$BATLESS src/main.rs --strip-ansi --max-lines=3
echo

echo "12. Large file handling:"
echo "-------------------------"
# Create a larger test file
for i in {1..100}; do
    echo "Line $i: This is line number $i with some content to make it longer"
done >/tmp/large_file.txt

echo "First 5 lines of 100-line file:"
$BATLESS /tmp/large_file.txt --mode=plain --max-lines=5
echo

echo "Byte-limited output:"
$BATLESS /tmp/large_file.txt --mode=plain --max-lines=20 --max-bytes=200
echo

echo "13. Compressed context: strip comments and blank lines (JavaScript):"
echo "--------------------------------------------------------------------"
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

$BATLESS /tmp/test.js --mode=index --language=javascript
echo
$BATLESS /tmp/test.js --strip-comments --strip-blank-lines --language=javascript
echo

echo "14. AI/automation-friendly output (no blocking, clean format):"
echo "----------------------------------------------------------------"
echo "Perfect for AI assistants and scripts:"
$BATLESS src/lib.rs --max-lines=15 --color=auto
echo

echo "15. CI/CD safe usage:"
echo "---------------------"
echo "Non-blocking output for automated systems:"
$BATLESS README.md --mode=plain --max-lines=10 --color=never
echo

echo "16. Combined features: JSON + index-derived symbol count via jq:"
echo "----------------------------------------------------------------"
$BATLESS src/main.rs --mode=index | jq '{file, language, symbol_count}'
echo

echo "🎉 Demo completed!"
echo
echo "🚀 Features Demonstrated:"
echo "✅ --list-languages: Discover all supported languages"
echo "✅ --mode=index: Machine-readable symbol table (functions, classes, structs)"
echo "✅ --strip-comments / --strip-blank-lines: Compressed, token-efficient context"
echo "✅ --with-line-numbers: Line-numbered JSON output"
echo "✅ Non-blocking output with encoding detection and metadata"
echo
echo "💡 Core Benefits:"
echo "✅ Always non-blocking (never hangs, never pages)"
echo "✅ Plain-text viewing by default, JSON and index modes for scripts"
echo "✅ Smart limiting (lines and bytes)"
echo "✅ Memory efficient (bounded reads via --max-lines/--max-bytes)"
echo "✅ cat -n / cat -b compatible"
echo "✅ CI/CD pipeline safe"
echo

# Cleanup
rm -f /tmp/batless_demo.txt /tmp/large_file.txt /tmp/test.js

echo "🤖 Quick Reference:"
echo "• batless --mode=index file.py    # Get code structure only"
echo "• batless --mode=json file.rs    # Full structured metadata"
echo "• batless --list-languages    # Check supported languages"
echo
echo "Run 'batless --help' for all available options!"
