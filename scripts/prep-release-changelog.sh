#!/usr/bin/env bash
set -euo pipefail

# Generate updated CHANGELOG.md using git-cliff and prepend Unreleased section.
# Usage: scripts/prep-release-changelog.sh <version> [--dry-run]

VERSION="${1:-}"
DRY_RUN=false
if [[ "$*" == *"--dry-run"* ]]; then
  DRY_RUN=true
fi

if [[ -z "$VERSION" ]]; then
  echo "Usage: $0 <version> [--dry-run]" >&2
  exit 1
fi

if ! command -v git-cliff >/dev/null 2>&1; then
  echo "git-cliff not found. Install with: cargo install git-cliff" >&2
  exit 1
fi

echo "🔄 Generating changelog for version $VERSION"

TMP_FILE=$(mktemp)

git-cliff --tag "$VERSION" > "$TMP_FILE"

if $DRY_RUN; then
  echo "--- BEGIN GENERATED CHANGELOG (dry run) ---"
  cat "$TMP_FILE"
  echo "--- END GENERATED CHANGELOG ---"
  rm "$TMP_FILE"
  exit 0
fi

mv "$TMP_FILE" CHANGELOG.md

echo "✅ CHANGELOG.md updated for $VERSION"

echo "Next steps:"
cat <<EOF
1. Review CHANGELOG.md
2. Commit: git add CHANGELOG.md && git commit -m "chore(release): update changelog for $VERSION"
3. Tag:    git tag $VERSION && git push origin $VERSION
4. Push:   git push origin HEAD
EOF
