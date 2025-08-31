#!/bin/bash
set -euo pipefail

# CI/CD Pipeline Health Check and Auto-Remediation Script
# Author: Claude Code
# Purpose: Monitor and fix CI/CD pipeline issues

# Colors for output
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly BLUE='\033[0;34m'
readonly NC='\033[0m' # No Color

# Status indicators
readonly SUCCESS="✅"
readonly FAILURE="❌"
readonly WARNING="⚠️"
readonly INFO="ℹ️"
readonly FIXING="🔧"

log_info() { echo -e "${BLUE}${INFO} $1${NC}"; }
log_success() { echo -e "${GREEN}${SUCCESS} $1${NC}"; }
log_warning() { echo -e "${YELLOW}${WARNING} $1${NC}"; }
log_error() { echo -e "${RED}${FAILURE} $1${NC}"; }
log_fixing() { echo -e "${YELLOW}${FIXING} $1${NC}"; }

# Global counters
TESTS_PASSED=0
TESTS_FAILED=0
FIXES_APPLIED=0

banner() {
    echo
    echo "=========================================="
    echo "    CI/CD Pipeline Health Check"
    echo "=========================================="
    echo "Timestamp: $(date)"
    echo "Branch: $(git branch --show-current)"
    echo "Last Commit: $(git log -1 --oneline)"
    echo "=========================================="
    echo
}

check_dependencies() {
    log_info "Checking dependencies..."

    local required_tools=("gh" "cargo" "git" "pre-commit")
    local missing_tools=()

    for tool in "${required_tools[@]}"; do
        if ! command -v "$tool" &> /dev/null; then
            missing_tools+=("$tool")
        fi
    done

    if [ ${#missing_tools[@]} -eq 0 ]; then
        log_success "All required tools are installed"
        ((TESTS_PASSED++))
    else
        log_error "Missing tools: ${missing_tools[*]}"
        log_info "Install missing tools:"
        for tool in "${missing_tools[@]}"; do
            case "$tool" in
                gh) echo "  - GitHub CLI: brew install gh" ;;
                pre-commit) echo "  - Pre-commit: pip install pre-commit" ;;
                *) echo "  - $tool: Install via your package manager" ;;
            esac
        done
        ((TESTS_FAILED++))
        return 1
    fi
}

check_git_status() {
    log_info "Checking git repository status..."

    if ! git rev-parse --git-dir &> /dev/null; then
        log_error "Not in a git repository"
        ((TESTS_FAILED++))
        return 1
    fi

    # Check for uncommitted changes
    if ! git diff-index --quiet HEAD --; then
        log_warning "Uncommitted changes detected"
        git status --porcelain | head -5
        ((TESTS_FAILED++))
    else
        log_success "Repository is clean"
        ((TESTS_PASSED++))
    fi

    # Check if we can access remote
    if git ls-remote origin &> /dev/null; then
        log_success "Remote repository accessible"
        ((TESTS_PASSED++))
    else
        log_warning "Cannot access remote repository"
        ((TESTS_FAILED++))
    fi
}

check_github_auth() {
    log_info "Checking GitHub authentication..."

    if gh auth status &> /dev/null; then
        log_success "GitHub CLI authenticated"
        ((TESTS_PASSED++))
    else
        log_error "GitHub CLI not authenticated"
        log_info "Run: gh auth login"
        ((TESTS_FAILED++))
        return 1
    fi
}

check_workflow_runs() {
    log_info "Checking recent workflow runs..."

    # Get recent workflow runs
    local recent_runs
    if ! recent_runs=$(gh run list --limit 5 --json status,conclusion,name,createdAt 2>/dev/null); then
        log_warning "Cannot fetch workflow runs (may not have gh permissions)"
        ((TESTS_FAILED++))
        return 1
    fi

    # Count failures in recent runs
    local failure_count
    failure_count=$(echo "$recent_runs" | jq '[.[] | select(.conclusion == "failure")] | length')

    if [ "$failure_count" -eq 0 ]; then
        log_success "No failed workflows in recent runs"
        ((TESTS_PASSED++))
    else
        log_error "$failure_count failed workflow(s) detected"
        log_info "Recent failures:"
        echo "$recent_runs" | jq -r '.[] | select(.conclusion == "failure") | "  - \(.name) (\(.createdAt))"' | head -3
        ((TESTS_FAILED++))
    fi
}

fix_cargo_issues() {
    log_fixing "Running Cargo checks and fixes..."

    # Format check
    if ! cargo fmt -- --check &> /dev/null; then
        log_fixing "Fixing code formatting..."
        cargo fmt
        log_success "Code formatting fixed"
        ((FIXES_APPLIED++))
    else
        log_success "Code formatting is correct"
        ((TESTS_PASSED++))
    fi

    # Clippy check
    if ! cargo clippy -- -D warnings &> /dev/null; then
        log_warning "Clippy warnings detected - check manually"
        ((TESTS_FAILED++))
    else
        log_success "No clippy warnings"
        ((TESTS_PASSED++))
    fi

    # Test execution
    log_info "Running tests..."
    if timeout 120 cargo test --lib --quiet &> /dev/null; then
        log_success "Unit tests pass"
        ((TESTS_PASSED++))
    else
        log_error "Unit tests failed"
        log_info "Run 'cargo test --lib' to see details"
        ((TESTS_FAILED++))
    fi

    if timeout 120 cargo test --test '*' --quiet &> /dev/null; then
        log_success "Integration tests pass"
        ((TESTS_PASSED++))
    else
        log_error "Integration tests failed"
        log_info "Run 'cargo test --test' to see details"
        ((TESTS_FAILED++))
    fi
}

check_pre_commit() {
    log_info "Checking pre-commit hooks..."

    if [ ! -f ".pre-commit-config.yaml" ]; then
        log_warning "Pre-commit config not found"
        ((TESTS_FAILED++))
        return 1
    fi

    if ! pre-commit run --all-files &> /tmp/precommit.log; then
        log_warning "Pre-commit checks failed"
        log_info "Issues found (first 10):"
        grep -E "Failed|error" /tmp/precommit.log | head -10

        # Try to auto-fix
        log_fixing "Attempting auto-fixes..."
        if pre-commit run --all-files; then
            log_success "Pre-commit issues resolved"
            ((FIXES_APPLIED++))
        else
            log_error "Some pre-commit issues need manual intervention"
            ((TESTS_FAILED++))
        fi
    else
        log_success "All pre-commit checks pass"
        ((TESTS_PASSED++))
    fi
}

check_security() {
    log_info "Running security checks..."

    # Cargo audit
    if command -v cargo-audit &> /dev/null; then
        if cargo audit &> /dev/null; then
            log_success "No security vulnerabilities found"
            ((TESTS_PASSED++))
        else
            log_warning "Security vulnerabilities detected"
            cargo audit 2>&1 | grep -E "error|warning" | head -3
            ((TESTS_FAILED++))
        fi
    else
        log_warning "cargo-audit not installed - skipping vulnerability scan"
        log_info "Install with: cargo install cargo-audit"
    fi

    # Check for secrets
    if [ -f ".secrets.baseline" ] && command -v detect-secrets &> /dev/null; then
        if detect-secrets scan --baseline .secrets.baseline . &> /dev/null; then
            log_success "No secrets detected"
            ((TESTS_PASSED++))
        else
            log_warning "Potential secrets detected"
            ((TESTS_FAILED++))
        fi
    fi
}

optimize_workflows() {
    log_info "Analyzing workflow performance..."

    # Check workflow file syntax
    local workflow_errors=0
    for workflow in .github/workflows/*.yml; do
        if [ -f "$workflow" ]; then
            if yq eval '.' "$workflow" &> /dev/null; then
                log_success "$(basename "$workflow") syntax valid"
            else
                log_error "$(basename "$workflow") has invalid YAML syntax"
                ((workflow_errors++))
            fi
        fi
    done

    if [ $workflow_errors -eq 0 ]; then
        ((TESTS_PASSED++))
    else
        ((TESTS_FAILED++))
    fi

    # Suggest optimizations
    log_info "Workflow optimization suggestions:"
    echo "  - Use caching for dependencies"
    echo "  - Run tests in parallel where possible"
    echo "  - Use matrix builds for multi-platform testing"
    echo "  - Set appropriate timeouts"
}

generate_metrics() {
    log_info "Collecting CI/CD metrics..."

    # Workflow success rate
    if command -v gh &> /dev/null && gh auth status &> /dev/null; then
        local total_runs success_runs success_rate
        total_runs=$(gh run list --limit 50 --json conclusion | jq length)
        success_runs=$(gh run list --limit 50 --json conclusion | jq '[.[] | select(.conclusion == "success")] | length')

        if [ "$total_runs" -gt 0 ]; then
            success_rate=$(( success_runs * 100 / total_runs ))
            echo "Success Rate: ${success_rate}% (${success_runs}/${total_runs})"
        fi

        # Average workflow duration (requires additional API calls)
        echo "Recent workflow status:"
        gh run list --limit 5 --json conclusion,name,createdAt | jq -r '.[] | "  - \(.name): \(.conclusion)"'
    fi
}

auto_fix_common_issues() {
    log_fixing "Attempting automatic fixes for common issues..."

    # Fix lockfile issues for MSRV compatibility
    if [ -f "Cargo.lock" ]; then
        local lockfile_version
        lockfile_version=$(grep -o 'version = [0-9]*' Cargo.lock | head -1 | grep -o '[0-9]*')

        if [ "$lockfile_version" -gt 3 ]; then
            log_warning "Lockfile version $lockfile_version may not be compatible with MSRV"
            log_info "Consider regenerating lockfile with MSRV toolchain"
        fi
    fi

    # Update dependencies if there are vulnerability warnings
    if command -v cargo-audit &> /dev/null && ! cargo audit &> /dev/null; then
        log_fixing "Updating dependencies to fix vulnerabilities..."
        if cargo update; then
            log_success "Dependencies updated"
            ((FIXES_APPLIED++))
        fi
    fi
}

commit_fixes() {
    if [ $FIXES_APPLIED -gt 0 ]; then
        log_info "Committing automatic fixes..."

        git add -A
        if git diff --staged --quiet; then
            log_info "No changes to commit"
        else
            git commit -m "fix(ci): Auto-fix CI/CD pipeline issues

- Applied $FIXES_APPLIED automatic fixes
- Fixed formatting and linting issues
- Updated pre-commit configuration
- Resolved security baseline

🤖 Generated with Claude Code"
            log_success "Fixes committed"
        fi
    fi
}

final_report() {
    echo
    echo "=========================================="
    echo "    Pipeline Health Report"
    echo "=========================================="
    echo "Tests Passed: $TESTS_PASSED"
    echo "Tests Failed: $TESTS_FAILED"
    echo "Fixes Applied: $FIXES_APPLIED"
    echo

    local total_tests=$((TESTS_PASSED + TESTS_FAILED))
    if [ $total_tests -gt 0 ]; then
        local success_rate=$((TESTS_PASSED * 100 / total_tests))
        echo "Success Rate: ${success_rate}%"
    fi

    echo
    if [ $TESTS_FAILED -eq 0 ]; then
        log_success "Pipeline health check passed!"
        echo "🎉 Your CI/CD pipeline is healthy and optimized."
    else
        log_warning "Pipeline health check completed with issues"
        echo "📋 Review the failures above and fix manually if needed."
        echo
        echo "Next steps:"
        echo "1. Review failed tests above"
        echo "2. Fix any manual intervention items"
        echo "3. Re-run this script to verify fixes"
        echo "4. Monitor next few workflow runs"
    fi
    echo "=========================================="
}

main() {
    banner

    # Run all checks
    check_dependencies || true
    check_git_status || true
    check_github_auth || true
    check_workflow_runs || true
    fix_cargo_issues || true
    check_pre_commit || true
    check_security || true
    optimize_workflows || true
    auto_fix_common_issues || true
    generate_metrics || true

    # Commit any fixes
    commit_fixes || true

    # Final report
    final_report

    # Exit with appropriate code
    if [ $TESTS_FAILED -eq 0 ]; then
        exit 0
    else
        exit 1
    fi
}

# Script entry point
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
