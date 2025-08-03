#!/bin/bash
# Version Status Tracker
# Usage: ./scripts/version-status.sh

set -e

echo "📊 Version Status Report"
echo "======================="
echo "Generated: $(date)"
echo ""

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Local version
CARGO_VERSION=$(grep '^version = ' Cargo.toml | cut -d'"' -f2)
echo "🏠 Local Cargo.toml: v$CARGO_VERSION"

# Git tags
LATEST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "none")
echo "🏷️  Latest Git tag: $LATEST_TAG"

# GitHub releases
GITHUB_LATEST=$(curl -s https://api.github.com/repos/docdyhr/batless/releases/latest | jq -r '.tag_name // "none"' 2>/dev/null || echo "unknown")
echo "🐙 GitHub latest: $GITHUB_LATEST"

# Crates.io
CRATES_VERSION=$(cargo search batless --limit 1 2>/dev/null | head -1 | cut -d'"' -f2 || echo "unknown")
echo "📦 Crates.io: v$CRATES_VERSION"

# Docker Hub (if applicable)
echo "🐳 Docker: Not implemented yet"

echo ""
echo "🎯 Status Summary:"
echo "=================="

# Check synchronization
if [ "v$CARGO_VERSION" = "$LATEST_TAG" ] && [ "$LATEST_TAG" = "$GITHUB_LATEST" ] && [ "$CARGO_VERSION" = "$CRATES_VERSION" ]; then
    echo -e "${GREEN}✅ All versions are synchronized${NC}"
    echo "   Status: READY FOR DEVELOPMENT"
else
    echo -e "${YELLOW}⚠️  Version mismatch detected${NC}"
    echo "   Status: MANUAL INTERVENTION NEEDED"
    
    echo ""
    echo "🔍 Detailed Analysis:"
    if [ "v$CARGO_VERSION" != "$LATEST_TAG" ]; then
        echo -e "   ${RED}• Git tag mismatch${NC}: Local v$CARGO_VERSION ≠ Tag $LATEST_TAG"
    fi
    if [ "$LATEST_TAG" != "$GITHUB_LATEST" ]; then
        echo -e "   ${RED}• GitHub release mismatch${NC}: Tag $LATEST_TAG ≠ GitHub $GITHUB_LATEST"
    fi
    if [ "$CARGO_VERSION" != "$CRATES_VERSION" ]; then
        echo -e "   ${RED}• Crates.io mismatch${NC}: Local v$CARGO_VERSION ≠ Crates v$CRATES_VERSION"
    fi
    
    echo ""
    echo "🛠️  Suggested Actions:"
    echo "   1. Check if release is in progress"
    echo "   2. Run: ./scripts/release-status.sh"
    echo "   3. Consider manual intervention if stuck"
fi

echo ""
echo "📈 Release History (Last 5):"
git tag --sort=-version:refname | head -5 | while read tag; do
    if [ -n "$tag" ]; then
        echo "   $tag ($(git log -1 --format=%ad --date=short $tag))"
    fi
done

echo ""
echo "🔄 Active Workflows:"
echo "   Check: https://github.com/docdyhr/batless/actions"
