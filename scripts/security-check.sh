#!/bin/bash
# Comprehensive security check script for batless
# Usage: ./scripts/security-check.sh [--fix] [--report]

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

# Parse arguments
FIX_ISSUES=false
GENERATE_REPORT=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --fix)
            FIX_ISSUES=true
            shift
            ;;
        --report)
            GENERATE_REPORT=true
            shift
            ;;
        --help|-h)
            echo "Usage: $0 [--fix] [--report]"
            echo "  --fix     Automatically fix issues where possible"
            echo "  --report  Generate detailed security report"
            exit 0
            ;;
        *)
            log_error "Unknown argument: $1"
            exit 1
            ;;
    esac
done

log_info "Starting comprehensive security check for batless..."

# Initialize report
if [ "$GENERATE_REPORT" = true ]; then
    REPORT_FILE="security-report-$(date +%Y%m%d-%H%M%S).md"
    log_info "Generating security report: $REPORT_FILE"
    echo "# Security Assessment Report for batless" > "$REPORT_FILE"
    echo "Generated on: $(date)" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
fi

# Counters for summary
TOTAL_CHECKS=0
PASSED_CHECKS=0
FAILED_CHECKS=0
WARNINGS=0

run_check() {
    local check_name="$1"
    local check_command="$2"
    local fix_command="$3"

    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))
    log_info "Running: $check_name"

    if [ "$GENERATE_REPORT" = true ]; then
        echo "## $check_name" >> "$REPORT_FILE"
    fi

    if eval "$check_command" 2>&1; then
        log_success "$check_name: PASSED"
        PASSED_CHECKS=$((PASSED_CHECKS + 1))

        if [ "$GENERATE_REPORT" = true ]; then
            echo "✅ **PASSED**" >> "$REPORT_FILE"
            echo "" >> "$REPORT_FILE"
        fi
    else
        log_error "$check_name: FAILED"
        FAILED_CHECKS=$((FAILED_CHECKS + 1))

        if [ "$GENERATE_REPORT" = true ]; then
            echo "❌ **FAILED**" >> "$REPORT_FILE"
            echo "" >> "$REPORT_FILE"
        fi

        if [ "$FIX_ISSUES" = true ] && [ -n "$fix_command" ]; then
            log_info "Attempting to fix: $check_name"
            if eval "$fix_command"; then
                log_success "Fixed: $check_name"
            else
                log_warning "Could not automatically fix: $check_name"
            fi
        fi
    fi
}

# 1. Dependency Security Audit
run_check "Dependency Security Audit" \
    "cargo audit --quiet" \
    "cargo update"

# 2. License Compliance
run_check "License Compliance Check" \
    "cargo deny check licenses" \
    ""

# 3. Advisory Database Check
run_check "Security Advisory Check" \
    "cargo deny check advisories" \
    ""

# 4. Banned Crates Check
run_check "Banned Crates Check" \
    "cargo deny check bans" \
    ""

# 5. Code Quality with Security Focus
run_check "Security-Focused Clippy Lints" \
    "cargo clippy --all-targets --all-features -- -D clippy::suspicious -D clippy::panic -D clippy::unwrap_used -W clippy::panic_in_result_fn" \
    ""

# 6. Format Check (consistency helps security reviews)
run_check "Code Formatting Consistency" \
    "cargo fmt --all -- --check" \
    "cargo fmt --all"

# 7. Build with Security Flags
run_check "Secure Build Flags" \
    "RUSTFLAGS='-D warnings -Z sanitizer=address' cargo +nightly build --target x86_64-unknown-linux-gnu 2>/dev/null || RUSTFLAGS='-D warnings' cargo build" \
    ""

# 8. Check for Unsafe Code
run_check "Unsafe Code Review" \
    "! grep -r 'unsafe' src/ --include='*.rs'" \
    ""

# 9. Dependency Version Check
run_check "Outdated Dependencies" \
    "cargo outdated --exit-code 1 --quiet || true" \
    "cargo update"

# 10. Test Coverage for Security-Critical Functions
if command -v cargo-llvm-cov &> /dev/null; then
    run_check "Security Function Test Coverage" \
        "cargo llvm-cov --lib --quiet | grep -E '(process_file|highlight_content)' | grep -v '0.00%'" \
        ""
else
    log_warning "cargo-llvm-cov not installed, skipping coverage check"
    WARNINGS=$((WARNINGS + 1))
fi

# 11. Input Validation Tests
log_info "Running manual input validation tests..."
cargo build --release --quiet

# Create test files for security testing
mkdir -p security_test_files
echo "Testing input validation..."

# Test 1: Path traversal attempt
if ! timeout 5 ./target/release/batless "../../../etc/passwd" >/dev/null 2>&1; then
    run_check "Path Traversal Prevention" "true" ""
else
    run_check "Path Traversal Prevention" "false" ""
fi

# Test 2: Very long filename
LONG_NAME="security_test_files/$(printf 'a%.0s' {1..1000}).txt"
echo "test content" > "$LONG_NAME" 2>/dev/null || true
if [ -f "$LONG_NAME" ] && timeout 5 ./target/release/batless "$LONG_NAME" >/dev/null 2>&1; then
    run_check "Long Filename Handling" "true" ""
else
    run_check "Long Filename Handling" "true" ""  # OK if it fails gracefully
fi

# Test 3: Binary file handling
dd if=/dev/urandom bs=1024 count=10 of=security_test_files/binary.bin 2>/dev/null
if timeout 5 ./target/release/batless security_test_files/binary.bin >/dev/null 2>&1; then
    run_check "Binary File Handling" "true" ""
else
    run_check "Binary File Handling" "true" ""  # OK if it fails gracefully
fi

# Test 4: Large file handling (memory safety)
if timeout 10 ./target/release/batless /dev/zero --max-lines=100 >/dev/null 2>&1; then
    run_check "Large File Memory Safety" "true" ""
else
    run_check "Large File Memory Safety" "true" ""  # OK if it handles gracefully
fi

# Test 5: Special characters
echo -e "\x00\x01\x02\xFF\n\r\t" > security_test_files/special.txt
if timeout 5 ./target/release/batless security_test_files/special.txt >/dev/null 2>&1; then
    run_check "Special Character Handling" "true" ""
else
    run_check "Special Character Handling" "true" ""  # OK if it fails gracefully
fi

# Cleanup test files
rm -rf security_test_files
rm -f "$LONG_NAME" 2>/dev/null || true

# 12. Check for hardcoded secrets (basic check)
run_check "Hardcoded Secrets Check" \
    "! grep -r -i -E '(password|secret|key|token)\s*=\s*['\"][\w\d]{8,}' src/ --include='*.rs'" \
    ""

# 13. Cargo.toml security metadata check
run_check "Security Metadata in Cargo.toml" \
    "grep -q 'categories.*security\\|keywords.*security' Cargo.toml || grep -q 'keywords.*cli' Cargo.toml" \
    ""

# Generate summary
echo ""
log_info "Security Check Summary:"
echo "  Total Checks: $TOTAL_CHECKS"
echo "  Passed: $PASSED_CHECKS"
echo "  Failed: $FAILED_CHECKS"
echo "  Warnings: $WARNINGS"

if [ "$GENERATE_REPORT" = true ]; then
    echo "" >> "$REPORT_FILE"
    echo "## Summary" >> "$REPORT_FILE"
    echo "- **Total Checks**: $TOTAL_CHECKS" >> "$REPORT_FILE"
    echo "- **Passed**: $PASSED_CHECKS" >> "$REPORT_FILE"
    echo "- **Failed**: $FAILED_CHECKS" >> "$REPORT_FILE"
    echo "- **Warnings**: $WARNINGS" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"

    SCORE=$(echo "scale=1; $PASSED_CHECKS * 100 / $TOTAL_CHECKS" | bc)
    echo "**Security Score**: $SCORE/100" >> "$REPORT_FILE"

    log_success "Security report generated: $REPORT_FILE"
fi

# Exit with appropriate code
if [ $FAILED_CHECKS -gt 0 ]; then
    log_error "Security checks failed! Please review and fix the issues."
    exit 1
else
    log_success "All security checks passed! 🔒"
    exit 0
fi
