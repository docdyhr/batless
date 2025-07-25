#!/bin/bash
set -euo pipefail

# Release Diagnostic Script for batless
# This script helps diagnose and fix common release workflow issues

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly SCRIPT_DIR
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
readonly PROJECT_ROOT
readonly GITHUB_REPO="docdyhr/batless"

# Colors for output
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly BLUE='\033[0;34m'
readonly NC='\033[0m' # No Color

# Logging functions
info() {
    echo -e "${BLUE}[INFO]${NC} $*"
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $*"
}

warning() {
    echo -e "${YELLOW}[WARNING]${NC} $*"
}

error() {
    echo -e "${RED}[ERROR]${NC} $*"
}

# Check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check GitHub CLI authentication
check_gh_auth() {
    info "Checking GitHub CLI authentication..."

    if ! command_exists gh; then
        error "GitHub CLI (gh) is not installed"
        echo "Install with: brew install gh (macOS) or see https://cli.github.com/manual/installation"
        return 1
    fi

    if ! gh auth status >/dev/null 2>&1; then
        error "GitHub CLI is not authenticated"
        echo "Run: gh auth login"
        return 1
    fi

    success "GitHub CLI is authenticated"
    return 0
}

# Check repository status
check_repo_status() {
    info "Checking repository status..."

    cd "$PROJECT_ROOT"

    # Check if we're in a git repository
    if ! git rev-parse --git-dir >/dev/null 2>&1; then
        error "Not in a git repository"
        return 1
    fi

    # Check if we're on main branch
    local current_branch
    current_branch=$(git branch --show-current)
    if [[ "$current_branch" != "main" ]]; then
        warning "Not on main branch (current: $current_branch)"
    else
        success "On main branch"
    fi

    # Check for uncommitted changes
    if ! git diff --quiet || ! git diff --cached --quiet; then
        warning "Uncommitted changes detected"
        git status --short
    else
        success "Working directory is clean"
    fi

    # Check if local is behind remote
    git fetch origin main >/dev/null 2>&1
    local local_hash remote_hash
    local_hash=$(git rev-parse HEAD)
    remote_hash=$(git rev-parse origin/main)

    if [[ "$local_hash" != "$remote_hash" ]]; then
        warning "Local branch is not in sync with origin/main"
    else
        success "Local branch is in sync with origin/main"
    fi

    return 0
}

# Check workflow files
check_workflow_files() {
    info "Checking workflow files..."

    local workflows_dir="${PROJECT_ROOT}/.github/workflows"

    if [[ ! -d "$workflows_dir" ]]; then
        error "Workflows directory not found: $workflows_dir"
        return 1
    fi

    local required_workflows=("release.yml" "ci.yml")
    local missing_workflows=()

    for workflow in "${required_workflows[@]}"; do
        if [[ ! -f "${workflows_dir}/${workflow}" ]]; then
            missing_workflows+=("$workflow")
        fi
    done

    if [[ ${#missing_workflows[@]} -gt 0 ]]; then
        error "Missing workflow files: ${missing_workflows[*]}"
        return 1
    fi

    success "All required workflow files present"

    # Check release workflow syntax
    info "Validating release workflow syntax..."
    if command_exists yamllint; then
        if yamllint "${workflows_dir}/release.yml" >/dev/null 2>&1; then
            success "Release workflow YAML syntax is valid"
        else
            warning "Release workflow YAML syntax issues detected"
            yamllint "${workflows_dir}/release.yml" || true
        fi
    else
        warning "yamllint not available, skipping YAML validation"
    fi

    return 0
}

# Check repository secrets
check_secrets() {
    info "Checking repository secrets..."

    if ! check_gh_auth; then
        return 1
    fi

    local required_secrets=("CARGO_REGISTRY_TOKEN")
    local optional_secrets=("HOMEBREW_TAP_TOKEN")

    info "Required secrets:"
    for secret in "${required_secrets[@]}"; do
        if gh secret list --repo "$GITHUB_REPO" | grep -q "$secret"; then
            success "  ✓ $secret is set"
        else
            error "  ✗ $secret is missing"
            echo "    Set with: gh secret set $secret --repo $GITHUB_REPO"
        fi
    done

    info "Optional secrets:"
    for secret in "${optional_secrets[@]}"; do
        if gh secret list --repo "$GITHUB_REPO" | grep -q "$secret"; then
            success "  ✓ $secret is set"
        else
            warning "  ⚠ $secret is not set (Homebrew tap updates will be skipped)"
            echo "    Set with: gh secret set $secret --repo $GITHUB_REPO"
        fi
    done

    return 0
}

# Check latest workflow runs
check_workflow_runs() {
    info "Checking recent workflow runs..."

    if ! check_gh_auth; then
        return 1
    fi

    echo "Recent workflow runs:"
    gh run list --repo "$GITHUB_REPO" --limit 5 --json conclusion,displayTitle,status,workflowName,createdAt |
        jq -r '.[] | "\(.workflowName): \(.conclusion // .status) - \(.displayTitle) (\(.createdAt))"' || {
        warning "Failed to fetch workflow runs"
        return 1
    }

    return 0
}

# Check releases
check_releases() {
    info "Checking releases..."

    if ! check_gh_auth; then
        return 1
    fi

    local releases
    releases=$(gh release list --repo "$GITHUB_REPO" --limit 5 2>/dev/null || echo "")

    if [[ -z "$releases" ]]; then
        warning "No releases found"
        echo "This might indicate that:"
        echo "  - Release workflows have never run successfully"
        echo "  - Tags exist but releases weren't created"
        echo "  - Repository is new and no releases have been made"
    else
        success "Recent releases:"
        echo "$releases"
    fi

    # Check tags
    info "Checking tags..."
    local tags
    tags=$(git tag -l --sort=-version:refname | head -5 || echo "")

    if [[ -z "$tags" ]]; then
        warning "No tags found"
    else
        success "Recent tags:"
        echo "$tags"
    fi

    return 0
}

# Check project files
check_project_files() {
    info "Checking project files..."

    cd "$PROJECT_ROOT"

    local required_files=("Cargo.toml" "README.md" "LICENSE" "CHANGELOG.md")
    local missing_files=()

    for file in "${required_files[@]}"; do
        if [[ ! -f "$file" ]]; then
            missing_files+=("$file")
        fi
    done

    if [[ ${#missing_files[@]} -gt 0 ]]; then
        error "Missing required files: ${missing_files[*]}"
        return 1
    fi

    success "All required files present"

    # Check Cargo.toml version
    local version
    version=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
    if [[ -n "$version" ]]; then
        success "Current version in Cargo.toml: $version"
    else
        error "Could not extract version from Cargo.toml"
        return 1
    fi

    return 0
}

# Check Rust toolchain
check_rust_toolchain() {
    info "Checking Rust toolchain..."

    if ! command_exists cargo; then
        error "Cargo is not installed"
        echo "Install Rust from: https://rustup.rs/"
        return 1
    fi

    local rust_version
    rust_version=$(rustc --version)
    success "Rust version: $rust_version"

    # Check if project builds
    info "Testing project build..."
    cd "$PROJECT_ROOT"

    if cargo check --quiet; then
        success "Project builds successfully"
    else
        error "Project build failed"
        return 1
    fi

    return 0
}

# Fix common issues
fix_issues() {
    info "Attempting to fix common issues..."

    cd "$PROJECT_ROOT"

    # Fix Cargo.lock
    if [[ -f "Cargo.lock" ]]; then
        info "Updating Cargo.lock..."
        cargo update
    fi

    # Fix formatting
    if command_exists cargo && cargo fmt --version >/dev/null 2>&1; then
        info "Formatting code..."
        cargo fmt
    fi

    # Run clippy
    if command_exists cargo && cargo clippy --version >/dev/null 2>&1; then
        info "Running clippy..."
        cargo clippy --fix --allow-dirty --allow-staged || warning "Clippy fixes failed"
    fi

    success "Common fixes applied"
    return 0
}

# Create a test release
create_test_release() {
    local version="${1:-0.1.2-test}"

    info "Creating test release: $version"

    if ! check_gh_auth; then
        return 1
    fi

    cd "$PROJECT_ROOT"

    # Create a test tag
    local tag="v$version"

    if git tag -l | grep -q "^$tag$"; then
        warning "Test tag $tag already exists, deleting..."
        git tag -d "$tag" 2>/dev/null || true
        git push origin ":$tag" 2>/dev/null || true
    fi

    info "Creating test tag: $tag"
    git tag -a "$tag" -m "Test release $version"
    git push origin "$tag"

    success "Test tag created: $tag"
    info "Monitor the release workflow at: https://github.com/$GITHUB_REPO/actions"

    return 0
}

# Main diagnostic function
run_diagnostics() {
    echo "🔍 Batless Release Diagnostics"
    echo "=============================="
    echo

    local checks=(
        "check_repo_status"
        "check_rust_toolchain"
        "check_project_files"
        "check_workflow_files"
        "check_secrets"
        "check_releases"
        "check_workflow_runs"
    )

    local failed_checks=()

    for check in "${checks[@]}"; do
        echo
        if ! $check; then
            failed_checks+=("$check")
        fi
    done

    echo
    echo "=============================="

    if [[ ${#failed_checks[@]} -eq 0 ]]; then
        success "All diagnostics passed! 🎉"
        echo
        echo "Your release setup looks good. You can:"
        echo "  1. Create a release manually: ./scripts/diagnose-release.sh test-release"
        echo "  2. Use GitHub Actions: gh workflow run manual-release.yml"
        echo "  3. Push a tag: git tag v0.1.2 && git push origin v0.1.2"
    else
        error "Some diagnostics failed:"
        for check in "${failed_checks[@]}"; do
            echo "  - $check"
        done
        echo
        echo "Run './scripts/diagnose-release.sh fix' to attempt automatic fixes"
    fi

    return 0
}

# Usage information
usage() {
    cat <<EOF
Batless Release Diagnostics

USAGE:
    $0 [COMMAND]

COMMANDS:
    check          Run all diagnostic checks (default)
    fix            Attempt to fix common issues
    test-release   Create a test release
    secrets        Check repository secrets
    workflows      Check workflow runs
    help           Show this help message

EXAMPLES:
    $0                    # Run all diagnostics
    $0 fix                # Fix common issues
    $0 test-release       # Create a test release
    $0 secrets            # Check only secrets

EOF
}

# Main script logic
main() {
    local command="${1:-check}"

    case "$command" in
    check)
        run_diagnostics
        ;;
    fix)
        fix_issues
        ;;
    test-release)
        create_test_release "${2:-}"
        ;;
    secrets)
        check_secrets
        ;;
    workflows)
        check_workflow_runs
        ;;
    help | --help | -h)
        usage
        ;;
    *)
        error "Unknown command: $command"
        usage
        exit 1
        ;;
    esac
}

# Script entry point
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
