#!/bin/bash
# Release preparation script for batless
# Usage: ./scripts/prep-release.sh <version>

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if version argument is provided
if [ $# -ne 1 ]; then
    log_error "Usage: $0 <version>"
    log_error "Example: $0 0.2.0"
    exit 1
fi

NEW_VERSION="$1"

# Validate version format (basic semver check)
if ! [[ "$NEW_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?(\+[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?$ ]]; then
    log_error "Invalid version format. Please use semantic versioning (e.g., 1.2.3)"
    exit 1
fi

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')

log_info "Preparing release: ${CURRENT_VERSION} -> ${NEW_VERSION}"

# Check if we're on main branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ]; then
    log_warning "You're not on the main branch (currently on: $CURRENT_BRANCH)"
    read -p "Do you want to continue? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        log_info "Release preparation cancelled"
        exit 0
    fi
fi

# Check for uncommitted changes
if ! git diff-index --quiet HEAD --; then
    log_error "You have uncommitted changes. Please commit or stash them first."
    exit 1
fi

# Pull latest changes
log_info "Pulling latest changes..."
git pull origin main

# Run tests to ensure everything is working
log_info "Running tests..."
if ! cargo test; then
    log_error "Tests failed. Please fix them before releasing."
    exit 1
fi

# Run additional checks
log_info "Running cargo checks..."
cargo fmt --all -- --check || {
    log_error "Code formatting check failed. Run 'cargo fmt' first."
    exit 1
}

cargo clippy --all-targets --all-features -- -D warnings || {
    log_error "Clippy checks failed. Please fix the issues first."
    exit 1
}

# Security audit
log_info "Running security audit..."
if command -v cargo-audit &> /dev/null; then
    cargo audit || {
        log_warning "Security audit found issues. Please review them."
        read -p "Do you want to continue anyway? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 1
        fi
    }
else
    log_warning "cargo-audit not installed. Skipping security audit."
fi

# Update version in Cargo.toml
log_info "Updating version in Cargo.toml..."
if command -v sed -i &> /dev/null; then
    # macOS/BSD sed
    sed -i '' "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
else
    # GNU sed
    sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
fi

# Update Cargo.lock
log_info "Updating Cargo.lock..."
cargo update -p batless

# Check if CHANGELOG.md exists and update it
if [ -f "CHANGELOG.md" ]; then
    log_info "Please update CHANGELOG.md with the changes for version $NEW_VERSION"
    log_info "Opening CHANGELOG.md..."
    
    # Try to open with common editors
    if command -v code &> /dev/null; then
        code CHANGELOG.md
    elif command -v vim &> /dev/null; then
        vim CHANGELOG.md
    elif command -v nano &> /dev/null; then
        nano CHANGELOG.md
    else
        log_info "Please manually edit CHANGELOG.md"
    fi
    
    read -p "Press Enter when you've finished updating CHANGELOG.md..."
else
    log_warning "CHANGELOG.md not found. Consider creating one."
fi

# Build and test the release version
log_info "Building release version..."
cargo build --release

# Test the release binary
log_info "Testing release binary..."
./target/release/batless --version | grep -q "$NEW_VERSION" || {
    log_error "Version mismatch in built binary"
    exit 1
}

./target/release/batless --help > /dev/null || {
    log_error "Release binary help command failed"
    exit 1
}

# Show summary of changes
log_success "Release preparation complete!"
echo
log_info "Summary of changes:"
echo "  • Version updated: ${CURRENT_VERSION} -> ${NEW_VERSION}"
echo "  • Cargo.toml updated"
echo "  • Cargo.lock updated"
echo "  • Tests passed"
echo "  • Release binary built and tested"
echo
log_info "Next steps:"
echo "  1. Review the changes: git diff"
echo "  2. Commit: git add . && git commit -m 'Release v${NEW_VERSION}'"
echo "  3. Tag: git tag v${NEW_VERSION}"
echo "  4. Push: git push origin main --tags"
echo "  5. GitHub Actions will handle the rest!"
echo
log_warning "Remember to:"
echo "  • Update CHANGELOG.md if you haven't already"
echo "  • Verify the release notes are accurate"
echo "  • Monitor the GitHub Actions release workflow"