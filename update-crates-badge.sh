#!/usr/bin/env bash
# Script to update README with real crates.io badge after publication

echo "🦇 Updating README with real crates.io badge..."

# Replace the "coming soon" badge with the real one
sed -i.bak 's|[![Crates.io](https://img.shields.io/badge/crates.io-coming%20soon-inactive)](https://crates.io/crates/batless)|[![Crates.io](https://img.shields.io/crates/v/batless.svg)](https://crates.io/crates/batless)|g' README.md

# Also update the installation section
sed -i.bak 's|### From Crates.io (Coming Soon)|### From Crates.io|g' README.md
sed -i.bak 's|# Once published, you'"'"'ll be able to install with:|# Install the latest version:|g' README.md

# Remove backup file
rm README.md.bak

echo "✅ README updated!"
echo "📦 Crates.io badge: https://img.shields.io/crates/v/batless.svg"
echo "🔗 Package page: https://crates.io/crates/batless"

# Commit the change
if git diff --quiet README.md; then
    echo "ℹ️ No changes to commit"
else
    git add README.md
    git commit -m "docs: update crates.io badge after successful publication

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>"
    git push origin main
    echo "✅ Changes committed and pushed"
fi