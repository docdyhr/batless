#!/usr/bin/env bash
# Generate shell completions for batless
set -euo pipefail

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
COMPLETIONS_DIR="$PROJECT_ROOT/completions"

# Ensure completions directory exists
mkdir -p "$COMPLETIONS_DIR"

# Build the project to ensure we have the latest binary
echo "Building batless..."
cd "$PROJECT_ROOT"
cargo build --release

BATLESS_BIN="$PROJECT_ROOT/target/release/batless"

# Check if binary exists
if [[ ! -f "$BATLESS_BIN" ]]; then
    echo "Error: batless binary not found at $BATLESS_BIN"
    echo "Make sure 'cargo build --release' completed successfully"
    exit 1
fi

echo "Generating shell completions..."

# Generate completions for each shell
echo "  → Generating bash completion..."
"$BATLESS_BIN" --generate-completions bash >"$COMPLETIONS_DIR/batless.bash"

echo "  → Generating zsh completion..."
"$BATLESS_BIN" --generate-completions zsh >"$COMPLETIONS_DIR/_batless"

echo "  → Generating fish completion..."
"$BATLESS_BIN" --generate-completions fish >"$COMPLETIONS_DIR/batless.fish"

echo "  → Generating PowerShell completion..."
"$BATLESS_BIN" --generate-completions power-shell >"$COMPLETIONS_DIR/batless.ps1"

echo ""
echo "✅ Completions generated successfully!"
echo ""
echo "Installation instructions:"
echo ""
echo "Bash:"
echo "  sudo cp $COMPLETIONS_DIR/batless.bash /etc/bash_completion.d/"
echo "  # Or for user-only:"
echo "  mkdir -p ~/.local/share/bash-completion/completions"
echo "  cp $COMPLETIONS_DIR/batless.bash ~/.local/share/bash-completion/completions/batless"
echo ""
echo "Zsh:"
echo "  # Add to your fpath before compinit in ~/.zshrc:"
echo "  fpath=($COMPLETIONS_DIR \$fpath)"
echo "  # Or copy to a directory already in fpath:"
echo "  cp $COMPLETIONS_DIR/_batless /usr/local/share/zsh/site-functions/"
echo ""
echo "Fish:"
echo "  cp $COMPLETIONS_DIR/batless.fish ~/.config/fish/completions/"
echo ""
echo "PowerShell:"
echo "  # Add to your PowerShell profile:"
echo "  echo '. $COMPLETIONS_DIR/batless.ps1' >> \$PROFILE"
echo ""
echo "Generated files:"
for file in "$COMPLETIONS_DIR"/*; do
    if [[ -f "$file" ]]; then
        echo "  $(basename "$file") ($(wc -l <"$file") lines)"
    fi
done
