#!/bin/bash
# Clean up the changelog duplications

# Create a clean changelog keeping only the first v0.2.2 entry and everything from v0.2.1 onwards
echo "Cleaning up CHANGELOG.md duplications..."

# Extract just the first section (header + first v0.2.2 entry)
sed -n '1,/^## \[0\.2\.1\]/p' CHANGELOG.md | head -n -1 > CHANGELOG_clean.md

# Extract everything from v0.2.1 onwards (avoiding duplicates)
sed -n '/^## \[0\.2\.1\]/,$p' CHANGELOG.md | \
  awk '/^## \[0\.2\.2\]/ { if (seen_022) next; seen_022=1 } 1' | \
  grep -v '^## \[0\.2\.2\]' >> CHANGELOG_clean.md

# Replace the original
mv CHANGELOG_clean.md CHANGELOG.md

echo "✅ Changelog cleaned!"
