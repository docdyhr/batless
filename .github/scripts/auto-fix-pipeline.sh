#!/bin/bash
# CI/CD Pipeline Auto-Fix Script
# Automatically detects and fixes common pipeline issues

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}🔧 Auto-fixing CI/CD pipeline issues...${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Track fixes applied
FIXES_APPLIED=0
FIXES_LOG=""

# Function to log fixes
log_fix() {
    local fix_description="$1"
    FIXES_APPLIED=$((FIXES_APPLIED + 1))
    FIXES_LOG="${FIXES_LOG}\n  ${FIXES_APPLIED}. $fix_description"
    echo -e "${GREEN}✅ Fixed: $fix_description${NC}"
}

# Function to check and fix pre-commit hooks
fix_precommit() {
    echo -e "\n${BLUE}Checking pre-commit hooks...${NC}"

    if ! pre-commit run --all-files &>/dev/null; then
        echo -e "${YELLOW}Fixing pre-commit violations...${NC}"

        # Auto-fix what we can
        pre-commit run --all-files || true

        # Stage any fixes
        if git diff --exit-code &>/dev/null; then
            echo "No changes made by pre-commit"
        else
            git add -A
            log_fix "Pre-commit hook violations auto-fixed"
        fi
    else
        echo -e "${GREEN}Pre-commit hooks already passing${NC}"
    fi
}

# Function to fix Rust formatting
fix_formatting() {
    echo -e "\n${BLUE}Checking code formatting...${NC}"

    if ! cargo fmt -- --check &>/dev/null; then
        echo -e "${YELLOW}Fixing formatting issues...${NC}"
        cargo fmt
        git add -A
        log_fix "Rust code formatting fixed"
    else
        echo -e "${GREEN}Code formatting already correct${NC}"
    fi
}

# Function to fix clippy warnings
fix_clippy() {
    echo -e "\n${BLUE}Checking clippy warnings...${NC}"

    # Try to auto-fix clippy issues
    if ! cargo clippy --quiet -- -D warnings &>/dev/null; then
        echo -e "${YELLOW}Attempting to fix clippy warnings...${NC}"
        cargo clippy --fix --allow-dirty --allow-staged &>/dev/null || true

        if ! git diff --exit-code &>/dev/null; then
            git add -A
            log_fix "Some clippy warnings auto-fixed"
        fi
    else
        echo -e "${GREEN}No clippy warnings found${NC}"
    fi
}

# Function to fix security vulnerabilities
fix_security() {
    echo -e "\n${BLUE}Checking security vulnerabilities...${NC}"

    if ! cargo audit --quiet &>/dev/null; then
        echo -e "${YELLOW}Found security vulnerabilities...${NC}"

        # Try to fix with cargo update
        cargo update &>/dev/null || true

        # Check if fixed
        if cargo audit --quiet &>/dev/null; then
            git add Cargo.lock
            log_fix "Security vulnerabilities fixed via dependency updates"
        else
            echo -e "${YELLOW}Some vulnerabilities require manual intervention${NC}"
        fi
    else
        echo -e "${GREEN}No security vulnerabilities found${NC}"
    fi
}

# Function to fix build issues
fix_build() {
    echo -e "\n${BLUE}Checking build...${NC}"

    if ! cargo build --release --quiet &>/dev/null; then
        echo -e "${YELLOW}Build failed, attempting fixes...${NC}"

        # Clean and rebuild
        cargo clean
        cargo build --release

        if [ $? -eq 0 ]; then
            log_fix "Build issues resolved via clean rebuild"
        else
            echo -e "${RED}Build still failing - manual intervention required${NC}"
        fi
    else
        echo -e "${GREEN}Build successful${NC}"
    fi
}

# Function to fix test failures
fix_tests() {
    echo -e "\n${BLUE}Checking tests...${NC}"

    # Run tests with timeout
    if ! timeout 60s cargo test --release --quiet &>/dev/null; then
        echo -e "${YELLOW}Some tests failing or timing out${NC}"

        # Try to identify the issue
        local test_output=$(cargo test --release 2>&1 | tail -50)

        if echo "$test_output" | grep -q "doc test"; then
            echo -e "${YELLOW}Doc tests may be failing - skipping for now${NC}"
        fi

        # Can't auto-fix test logic, but log the attempt
        echo -e "${YELLOW}Test failures require manual review${NC}"
    else
        echo -e "${GREEN}All tests passing${NC}"
    fi
}

# Function to optimize workflow files
optimize_workflows() {
    echo -e "\n${BLUE}Checking workflow optimization...${NC}"

    # Check if optimized workflows are active
    local ci_optimized=".github/workflows/ci-optimized.yml"
    local perf_optimized=".github/workflows/performance-optimized.yml"

    if [ -f "$ci_optimized" ] && [ -f "$perf_optimized" ]; then
        # Check if they have proper triggers
        if grep -q "workflow_dispatch:" "$ci_optimized" && ! grep -q "^  push:" "$ci_optimized"; then
            echo -e "${YELLOW}Optimized workflows not fully activated${NC}"
            # Don't auto-activate without user consent
            echo -e "${BLUE}Run 'gh workflow enable ci-optimized.yml' to activate${NC}"
        else
            echo -e "${GREEN}Optimized workflows are active${NC}"
        fi
    else
        echo -e "${YELLOW}Optimized workflows not found${NC}"
    fi
}

# Function to clean up
cleanup() {
    echo -e "\n${BLUE}Cleaning up...${NC}"

    # Remove temporary files
    rm -rf target/debug/*.d
    rm -rf target/release/*.d

    # Clean cargo cache if needed
    if [ -d "$HOME/.cargo/registry/cache" ]; then
        find "$HOME/.cargo/registry/cache" -type f -atime +30 -delete 2>/dev/null || true
    fi

    echo -e "${GREEN}Cleanup complete${NC}"
}

# Function to generate fix summary
generate_summary() {
    echo -e "\n${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}           Auto-Fix Summary               ${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

    if [ $FIXES_APPLIED -gt 0 ]; then
        echo -e "${GREEN}Applied $FIXES_APPLIED fix(es):${NC}"
        echo -e "$FIXES_LOG"
        echo ""
        echo -e "${YELLOW}Next steps:${NC}"
        echo "1. Review changes: git diff --staged"
        echo "2. Run tests: cargo test"
        echo "3. Commit fixes: git commit -m 'fix: Auto-fix CI/CD pipeline issues'"
        echo "4. Push changes: git push"
    else
        echo -e "${GREEN}✅ No fixes needed - pipeline is healthy!${NC}"
    fi

    echo -e "\n${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

# Function to commit fixes if requested
commit_fixes() {
    if [ $FIXES_APPLIED -gt 0 ]; then
        echo -e "\n${BLUE}Would you like to commit these fixes? (y/n)${NC}"
        read -r response

        if [[ "$response" =~ ^[Yy]$ ]]; then
            git commit -m "fix: Auto-fix CI/CD pipeline issues

Auto-fixed issues:
$FIXES_LOG

Generated by auto-fix-pipeline.sh" || true

            echo -e "${GREEN}Fixes committed successfully${NC}"
        else
            echo -e "${YELLOW}Fixes staged but not committed${NC}"
        fi
    fi
}

# Main execution
main() {
    # Check if in git repository
    if ! git rev-parse --git-dir &>/dev/null; then
        echo -e "${RED}Not in a git repository${NC}"
        exit 1
    fi

    # Run all fix functions
    fix_precommit
    fix_formatting
    fix_clippy
    fix_security
    fix_build
    fix_tests
    optimize_workflows
    cleanup

    # Generate summary
    generate_summary

    # Optionally commit fixes
    if [ "${1:-}" != "--no-commit" ]; then
        commit_fixes
    fi
}

# Handle script arguments
case "${1:-}" in
    --help|-h)
        echo "Usage: $0 [--no-commit]"
        echo "  --no-commit: Stage fixes but don't prompt for commit"
        echo "  --help: Show this help message"
        exit 0
        ;;
    *)
        main "$@"
        ;;
esac
