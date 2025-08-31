#!/bin/bash
# Automated release script for batless
# Usage: ./scripts/release.sh <version> [--dry-run]

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

# Default values
DRY_RUN=false
SKIP_CHECKS=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --skip-checks)
            SKIP_CHECKS=true
            shift
            ;;
        --help|-h)
            echo "Usage: $0 <version> [--dry-run] [--skip-checks]"
            echo "  <version>      The version to release (e.g., 0.2.0)"
            echo "  --dry-run      Show what would be done without making changes"
            echo "  --skip-checks  Skip pre-release checks (not recommended)"
            exit 0
            ;;
        *)
            if [ -z "$NEW_VERSION" ]; then
                NEW_VERSION="$1"
            else
                log_error "Unknown argument: $1"
                exit 1
            fi
            shift
            ;;
    esac
done

# Check if version argument is provided
if [ -z "$NEW_VERSION" ]; then
    log_error "Usage: $0 <version> [--dry-run] [--skip-checks]"
    log_error "Example: $0 0.2.0"
    exit 1
fi

# Validate version format (basic semver check)
if ! [[ "$NEW_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?(\+[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?$ ]]; then
    log_error "Invalid version format. Please use semantic versioning (e.g., 1.2.3)"
    exit 1
fi

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')

if [ "$DRY_RUN" = true ]; then
    log_info "DRY RUN MODE - No changes will be made"
fi

log_info "Planning release: ${CURRENT_VERSION} -> ${NEW_VERSION}"

# Check if we're on main branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ]; then
    if [ "$DRY_RUN" = false ]; then
        log_warning "You're not on the main branch (currently on: $CURRENT_BRANCH)"
        read -p "Do you want to continue? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            log_info "Release cancelled"
            exit 0
        fi
    else
        log_warning "Would warn about not being on main branch: $CURRENT_BRANCH"
    fi
fi

# Check for uncommitted changes
if ! git diff-index --quiet HEAD --; then
    if [ "$DRY_RUN" = false ]; then
        log_error "You have uncommitted changes. Please commit or stash them first."
        exit 1
    else
        log_warning "Would fail due to uncommitted changes"
    fi
fi

# Pre-release checks
if [ "$SKIP_CHECKS" = false ]; then
    log_info "Running pre-release checks..."

    if [ "$DRY_RUN" = false ]; then
        # Pull latest changes
        log_info "Pulling latest changes..."
        git pull origin main

        # Run tests
        log_info "Running tests..."
        cargo test

        # Format check
        log_info "Checking code formatting..."
        cargo fmt --all -- --check

        # Clippy check
        log_info "Running clippy..."
        cargo clippy --all-targets --all-features -- -D warnings

        # Security audit (if available)
        if command -v cargo-audit &> /dev/null; then
            log_info "Running security audit..."
            cargo audit
        else
            log_warning "cargo-audit not installed. Skipping security audit."
        fi

        # Build release version
        log_info "Building release version..."
        cargo build --release

        # Test release binary
        log_info "Testing release binary..."
        if ! ./target/release/batless --version | grep -q "$CURRENT_VERSION"; then
            log_error "Release binary version check failed"
            exit 1
        fi
    else
        log_info "Would run: tests, formatting, clippy, audit, build"
    fi
else
    log_warning "Skipping pre-release checks (--skip-checks was specified)"
fi

# Update version
log_info "Updating version in Cargo.toml..."
if [ "$DRY_RUN" = false ]; then
    if command -v gsed &> /dev/null; then
        # GNU sed (installed via Homebrew on macOS)
        gsed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS/BSD sed
        sed -i '' "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
    else
        # GNU sed (Linux)
        sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
    fi

    # Update Cargo.lock
    log_info "Updating Cargo.lock..."
    cargo update -p batless
else
    log_info "Would update version to: $NEW_VERSION"
fi

# Build and test with new version
if [ "$DRY_RUN" = false ]; then
    log_info "Building with new version..."
    cargo build --release

    # Verify new version
    if ! ./target/release/batless --version | grep -q "$NEW_VERSION"; then
        log_error "Version verification failed after update"
        exit 1
    fi
else
    log_info "Would build and test new version"
fi

# Generate changelog entry
log_info "Preparing changelog..."
if [ -f "CHANGELOG.md" ] && [ "$DRY_RUN" = false ]; then
    # Create a backup
    cp CHANGELOG.md CHANGELOG.md.backup

    # Get commits since last tag
    LAST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "")
    if [ -n "$LAST_TAG" ]; then
        log_info "Getting commits since $LAST_TAG..."
        COMMITS=$(git log --oneline --no-merges ${LAST_TAG}..HEAD | sed 's/^/- /')
    else
        log_info "No previous tags found, getting recent commits..."
        COMMITS=$(git log --oneline --no-merges -10 | sed 's/^/- /')
    fi

    # Create temporary changelog entry
    cat > temp_changelog.md << EOF
## [${NEW_VERSION}] - $(date +%Y-%m-%d)

### Added
<!-- New features -->

### Changed
<!-- Changes in existing functionality -->
$COMMITS

### Fixed
<!-- Bug fixes -->

### Removed
<!-- Features removed in this version -->

EOF

    # Prepend to existing changelog
    if grep -q "## \[" CHANGELOG.md; then
        # Insert after the header, before the first release entry
        awk '/^## \[/{print temp; system("cat temp_changelog.md"); temp=""; print; next} {if(temp==""){temp=temp $0 ORS} else {print}}' CHANGELOG.md > CHANGELOG.md.new && mv CHANGELOG.md.new CHANGELOG.md
    else
        # Prepend to the entire file
        cat temp_changelog.md CHANGELOG.md > CHANGELOG.md.new && mv CHANGELOG.md.new CHANGELOG.md
    fi

    rm temp_changelog.md

    log_info "Please review and edit the changelog entry:"
    if command -v code &> /dev/null; then
        code CHANGELOG.md
    else
        log_info "Changelog prepared. Please edit CHANGELOG.md manually."
    fi

    read -p "Press Enter when you've finished editing CHANGELOG.md (or Ctrl+C to cancel)..."
else
    log_info "Would update CHANGELOG.md with version $NEW_VERSION"
fi

# Commit changes
log_info "Committing release changes..."
if [ "$DRY_RUN" = false ]; then
    git add Cargo.toml Cargo.lock
    if [ -f "CHANGELOG.md" ]; then
        git add CHANGELOG.md
    fi
    git commit -m "Release v${NEW_VERSION}

- Update version to ${NEW_VERSION}
- Update changelog with release notes
- Prepare for automated release"
else
    log_info "Would commit: Cargo.toml, Cargo.lock, CHANGELOG.md"
fi

# Create and push tag
log_info "Creating release tag..."
if [ "$DRY_RUN" = false ]; then
    git tag -a "v${NEW_VERSION}" -m "Release v${NEW_VERSION}"
    log_info "Pushing to origin..."
    git push origin main
    git push origin "v${NEW_VERSION}"
else
    log_info "Would create tag: v${NEW_VERSION}"
    log_info "Would push to origin: main and v${NEW_VERSION}"
fi

# Show next steps
log_success "Release ${NEW_VERSION} initiated!"
echo
log_info "What happens next:"
echo "  1. ✅ Version updated and committed"
echo "  2. ✅ Release tag created and pushed"
echo "  3. 🔄 GitHub Actions will now:"
echo "     • Run full test suite"
echo "     • Build cross-platform binaries"
echo "     • Create GitHub release with assets"
echo "     • Publish to crates.io"
echo "     • Build and push Docker images"
echo "     • Update package managers"
echo
log_info "Monitor the release at:"
echo "  • GitHub Actions: https://github.com/$(git config --get remote.origin.url | sed 's/.*[:/]\([^/]*\/[^/]*\)\.git$/\1/')/actions"
echo "  • Releases: https://github.com/$(git config --get remote.origin.url | sed 's/.*[:/]\([^/]*\/[^/]*\)\.git$/\1/')/releases"
echo
if [ "$DRY_RUN" = false ]; then
    log_warning "If anything goes wrong:"
    echo "  • Delete tag: git tag -d v${NEW_VERSION} && git push origin :v${NEW_VERSION}"
    echo "  • Revert commit: git revert HEAD"
else
    log_info "This was a dry run. No actual changes were made."
fi
