#!/bin/bash
# Demo script to showcase batless functionality
# This demonstrates all the key features of batless

set -e

echo "🦇 Batless Demo Script"
echo "======================"
echo

# Build the project first
echo "📦 Building batless..."
cargo build --release
echo

# Set up the batless binary path
BATLESS="./target/release/batless"

echo "✨ Feature Demonstrations:"
echo

echo "1. Basic syntax highlighting (Rust code):"
echo "----------------------------------------"
$BATLESS src/main.rs --max-lines=10
echo

echo "2. Plain text mode (no colors):"
echo "--------------------------------"
$BATLESS src/main.rs --mode=plain --max-lines=5 --color=never
echo

echo "3. JSON output mode:"
echo "--------------------"
$BATLESS examples/demo.py --mode=json --max-lines=3
echo

echo "4. Language auto-detection (Python):"
echo "-------------------------------------"
$BATLESS examples/demo.py --max-lines=8
echo

echo "5. Explicit language specification:"
echo "------------------------------------"
$BATLESS Cargo.toml --language=toml --max-lines=6
echo

echo "6. Byte limiting demonstration:"
echo "--------------------------------"
echo "This is a test file with multiple lines
Each line has different content
Some lines are longer than others
Short line
This is another longer line with more content" >/tmp/batless_demo.txt

$BATLESS /tmp/batless_demo.txt --mode=plain --max-bytes=50
echo

echo "7. Different themes:"
echo "--------------------"
echo "Theme: base16-ocean.dark (default)"
$BATLESS src/main.rs --theme="base16-ocean.dark" --max-lines=5
echo

echo "Theme: InspiredGitHub"
$BATLESS src/main.rs --theme="InspiredGitHub" --max-lines=5
echo

echo "8. ANSI stripping:"
echo "------------------"
$BATLESS src/main.rs --strip-ansi --max-lines=3
echo

echo "9. Large file handling (streaming):"
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

echo "10. JSON metadata extraction:"
echo "------------------------------"
echo "File metadata as JSON:"
$BATLESS examples/demo.py --mode=json --max-lines=1 | jq '{file, language, total_lines, total_bytes, truncated}'
echo

echo "11. AI-friendly output (no blocking, clean format):"
echo "----------------------------------------------------"
echo "Perfect for AI assistants like Claude:"
$BATLESS src/lib.rs --max-lines=15 --color=auto
echo

echo "12. CI/CD safe usage:"
echo "---------------------"
echo "Non-blocking output for automated systems:"
$BATLESS README.md --mode=plain --max-lines=10 --color=never
echo

echo "🎉 Demo completed!"
echo
echo "Key benefits shown:"
echo "✅ Always non-blocking (never hangs)"
echo "✅ Syntax highlighting for 100+ languages"
echo "✅ Multiple output formats (plain, highlight, JSON)"
echo "✅ Smart limiting (lines and bytes)"
echo "✅ Memory efficient streaming"
echo "✅ AI assistant friendly"
echo "✅ CI/CD pipeline safe"
echo

# Cleanup
rm -f /tmp/batless_demo.txt /tmp/large_file.txt

echo "Run 'batless --help' for all available options!"
