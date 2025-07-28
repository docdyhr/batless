#!/usr/bin/env bash
# Script to publish batless to crates.io for the first time

set -euo pipefail

echo "🦇 Preparing to publish batless to crates.io..."

# Check if logged in to crates.io
if ! cargo login --list &>/dev/null; then
    echo "❌ Not logged in to crates.io"
    echo "Please run: cargo login <your-api-token>"
    echo "Get your API token from: https://crates.io/me"
    exit 1
fi

# Ensure we're on main branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ]; then
    echo "❌ Not on main branch (current: $CURRENT_BRANCH)"
    echo "Please switch to main branch before publishing"
    exit 1
fi

# Ensure working directory is clean
if ! git diff --quiet || ! git diff --cached --quiet; then
    echo "❌ Working directory has uncommitted changes"
    echo "Please commit or stash changes before publishing"
    exit 1
fi

# Run tests
echo "🧪 Running tests..."
cargo test --all-features

# Run clippy
echo "📋 Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

# Build in release mode
echo "🔨 Building release..."
cargo build --release

# Dry run first
echo "🏃 Running publish dry-run..."
cargo publish --dry-run

echo ""
echo "✅ All checks passed!"
echo ""
echo "Ready to publish batless v$(grep '^version' Cargo.toml | cut -d'"' -f2) to crates.io"
echo ""
read -p "Do you want to proceed with publishing? (y/N) " -n 1 -r
echo ""

if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "📦 Publishing to crates.io..."
    cargo publish
    
    echo ""
    echo "🎉 Successfully published!"
    echo ""
    echo "Next steps:"
    echo "1. Create a GitHub release with tag v$(grep '^version' Cargo.toml | cut -d'"' -f2)"
    echo "2. Update README.md to use the real crates.io badge:"
    echo "   [![Crates.io](https://img.shields.io/crates/v/batless.svg)](https://crates.io/crates/batless)"
else
    echo "❌ Publishing cancelled"
    exit 1
fi