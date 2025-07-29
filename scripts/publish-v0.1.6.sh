#!/bin/bash

# Manual script to publish v0.1.6 to crates.io
# This handles the dirty Cargo.lock issue

set -e

echo "🚀 Publishing batless v0.1.6 to crates.io..."

# Check if we're on the right version
VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
if [ "$VERSION" != "0.1.6" ]; then
    echo "❌ Error: Expected version 0.1.6, found $VERSION"
    exit 1
fi

echo "✅ Confirmed version: $VERSION"

# Check if we have a token
if [ -z "$CARGO_REGISTRY_TOKEN" ]; then
    echo "❌ Error: CARGO_REGISTRY_TOKEN not set"
    echo "Please run: export CARGO_REGISTRY_TOKEN=<your-token>"
    exit 1
fi

echo "✅ Token found"

# Publish with allow-dirty to handle Cargo.lock changes
echo "📦 Publishing to crates.io..."
cargo publish --allow-dirty

echo "🎉 Successfully published batless v$VERSION to crates.io!"
echo "📊 Check status: https://crates.io/crates/batless"