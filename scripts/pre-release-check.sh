#!/bin/bash
# Enhanced Pre-Release Quality Gate
# Usage: ./scripts/pre-release-check.sh

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

echo "🔍 Enhanced Pre-Release Quality Check"
echo "====================================="

# Test everything locally
log_info "Running comprehensive test suite..."
if cargo test --all; then
    log_success "All tests passed"
else
    log_error "Tests failed"
    exit 1
fi

# Check clippy with strict warnings
log_info "Running clippy with strict warnings..."
if cargo clippy --all-targets --all-features -- -D warnings; then
    log_success "Clippy checks passed"
else
    log_error "Clippy warnings found"
    exit 1
fi

# Check formatting
log_info "Checking code formatting..."
if cargo fmt -- --check; then
    log_success "Code formatting is correct"
else
    log_error "Code formatting issues found. Run 'cargo fmt' to fix."
    exit 1
fi

# Build release binary
log_info "Building release binary..."
if cargo build --release; then
    log_success "Release build successful"
else
    log_error "Release build failed"
    exit 1
fi

# Test the actual binary functionality
log_info "Testing release binary functionality..."
if ./target/release/batless --help > /dev/null; then
    log_success "Binary help command works"
else
    log_error "Binary help command failed"
    exit 1
fi

# Test basic file processing
if ./target/release/batless Cargo.toml --plain | head -5 > /dev/null; then
    log_success "Binary file processing works"
else
    log_error "Binary file processing failed"
    exit 1
fi

# Verify version consistency
log_info "Checking version consistency..."
CARGO_VERSION=$(grep '^version = ' Cargo.toml | cut -d'"' -f2)
CLI_VERSION=$(./target/release/batless --version | cut -d' ' -f2)
if [ "$CARGO_VERSION" = "$CLI_VERSION" ]; then
    log_success "Version consistency verified: v$CARGO_VERSION"
else
    log_error "Version mismatch: Cargo.toml=v$CARGO_VERSION, CLI=v$CLI_VERSION"
    exit 1
fi

# Test cat replacement functionality specifically
log_info "Testing cat replacement functionality..."
echo -e "line 1\nline 2\nline 3" > /tmp/test_batless.txt
if ./target/release/batless /tmp/test_batless.txt --plain -n | grep -q "     1	line 1"; then
    log_success "Cat replacement line numbering works"
else
    log_error "Cat replacement line numbering failed"
    rm -f /tmp/test_batless.txt
    exit 1
fi
rm -f /tmp/test_batless.txt

# Check git status
log_info "Checking git status..."
if [ -n "$(git status --porcelain)" ]; then
    log_warning "Working directory has uncommitted changes"
    git status --short
    echo "Consider committing changes before release"
else
    log_success "Working directory is clean"
fi

echo ""
log_success "✅ All pre-release checks passed!"
log_info "Ready for release process"
