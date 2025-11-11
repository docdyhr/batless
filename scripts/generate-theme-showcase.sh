#!/usr/bin/env bash
# Script to generate theme showcase examples for README
set -euo pipefail

# Get the directory of this script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
EXAMPLE_FILE="$PROJECT_ROOT/examples/theme-showcase.rs"
OUTPUT_DIR="$PROJECT_ROOT/docs/themes"

# Create output directory if it doesn't exist
mkdir -p "$OUTPUT_DIR"

# List of themes to showcase
themes=(
    "InspiredGitHub"
    "Solarized (dark)"
    "Solarized (light)"
    "base16-eighties.dark"
    "base16-mocha.dark"
    "base16-ocean.dark"
    "base16-ocean.light"
)

echo "Generating theme showcase examples..."
echo "Using example file: $EXAMPLE_FILE"
echo "Output directory: $OUTPUT_DIR"
echo ""

# Check if batless is available
if ! command -v batless &> /dev/null; then
    echo "Error: batless not found in PATH"
    echo "Please install batless or run from target/release directory"
    exit 1
fi

# Generate examples for each theme
for theme in "${themes[@]}"; do
    output_file="$OUTPUT_DIR/${theme// /_}.txt"
    echo "Generating: $theme → $(basename "$output_file")"

    # Generate output with limited lines for preview
    batless --theme="$theme" --max-lines=20 "$EXAMPLE_FILE" > "$output_file" 2>&1 || {
        echo "  Warning: Failed to generate theme '$theme'"
        continue
    }
done

echo ""
echo "✅ Theme showcase generation complete!"
echo ""
echo "To view themes:"
batless --list-themes
echo ""
echo "To use a specific theme:"
echo "  batless --theme=\"Solarized (dark)\" file.rs"
