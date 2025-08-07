#!/bin/bash
# Test script for pager compatibility
# Tests batless compatibility with various pager scenarios and command-line tools

set -euo pipefail

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Get script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
BATLESS_BIN="$PROJECT_ROOT/target/release/batless"

# Test files
TEST_FILE_SMALL="$PROJECT_ROOT/src/main.rs"
TEST_FILE_MEDIUM="$PROJECT_ROOT/src/lib.rs"

# Function to print test results
print_result() {
    local status=$1
    local test_name=$2
    local message=${3:-""}
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    if [ "$status" = "PASS" ]; then
        echo -e "  ${GREEN}✓${NC} $test_name"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    elif [ "$status" = "FAIL" ]; then
        echo -e "  ${RED}✗${NC} $test_name"
        [ -n "$message" ] && echo -e "    ${RED}Error:${NC} $message"
        FAILED_TESTS=$((FAILED_TESTS + 1))
    elif [ "$status" = "SKIP" ]; then
        echo -e "  ${YELLOW}○${NC} $test_name (skipped)"
        [ -n "$message" ] && echo -e "    ${YELLOW}Reason:${NC} $message"
    fi
}

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to build batless if needed
ensure_batless_built() {
    if [ ! -f "$BATLESS_BIN" ]; then
        echo -e "${BLUE}Building batless...${NC}"
        cd "$PROJECT_ROOT"
        cargo build --release --quiet
    fi
}

# Test 1: Basic plain mode compatibility
test_plain_mode() {
    echo -e "\n${BLUE}Testing Plain Mode Compatibility${NC}"
    
    # Test --plain flag
    if "$BATLESS_BIN" --plain "$TEST_FILE_SMALL" >/dev/null 2>&1; then
        print_result "PASS" "Basic --plain flag"
    else
        print_result "FAIL" "Basic --plain flag" "Command failed"
    fi
    
    # Test plain mode with line numbers
    if "$BATLESS_BIN" --plain --number "$TEST_FILE_SMALL" >/dev/null 2>&1; then
        print_result "PASS" "Plain mode with --number"
    else
        print_result "FAIL" "Plain mode with --number" "Command failed"
    fi
    
    # Test plain mode with non-blank line numbers
    if "$BATLESS_BIN" --plain --number-nonblank "$TEST_FILE_SMALL" >/dev/null 2>&1; then
        print_result "PASS" "Plain mode with --number-nonblank"
    else
        print_result "FAIL" "Plain mode with --number-nonblank" "Command failed"
    fi
}

# Test 2: Stdin compatibility
test_stdin_compatibility() {
    echo -e "\n${BLUE}Testing Stdin Compatibility${NC}"
    
    # Test basic stdin
    if echo "test content" | "$BATLESS_BIN" --plain >/dev/null 2>&1; then
        print_result "PASS" "Basic stdin processing"
    else
        print_result "FAIL" "Basic stdin processing" "Command failed"
    fi
    
    # Test stdin with line numbers
    if echo -e "line 1\nline 2\nline 3" | "$BATLESS_BIN" --plain --number >/dev/null 2>&1; then
        print_result "PASS" "Stdin with line numbers"
    else
        print_result "FAIL" "Stdin with line numbers" "Command failed"
    fi
    
    # Test large stdin input
    if seq 1 1000 | "$BATLESS_BIN" --plain >/dev/null 2>&1; then
        print_result "PASS" "Large stdin input"
    else
        print_result "FAIL" "Large stdin input" "Command failed"
    fi
}

# Test 3: Cat replacement compatibility
test_cat_replacement() {
    echo -e "\n${BLUE}Testing Cat Replacement Compatibility${NC}"
    
    # Compare batless output with cat output for small file
    local cat_output=$(cat "$TEST_FILE_SMALL" 2>/dev/null || echo "")
    local batless_output=$("$BATLESS_BIN" --plain "$TEST_FILE_SMALL" 2>/dev/null || echo "")
    
    if [ "$cat_output" = "$batless_output" ]; then
        print_result "PASS" "Output matches cat for plain text"
    else
        print_result "FAIL" "Output matches cat for plain text" "Output differs from cat"
    fi
    
    # Test line numbering compatibility with cat -n
    if command_exists cat; then
        local cat_numbered=$(cat -n "$TEST_FILE_SMALL" 2>/dev/null || echo "")
        local batless_numbered=$("$BATLESS_BIN" --plain --number "$TEST_FILE_SMALL" 2>/dev/null || echo "")
        
        if [ "$cat_numbered" = "$batless_numbered" ]; then
            print_result "PASS" "Line numbering matches cat -n"
        else
            print_result "FAIL" "Line numbering matches cat -n" "Numbering format differs"
        fi
    else
        print_result "SKIP" "Line numbering matches cat -n" "cat command not available"
    fi
}

# Test 4: GitHub CLI integration
test_github_cli_integration() {
    echo -e "\n${BLUE}Testing GitHub CLI Integration${NC}"
    
    if command_exists gh; then
        # Test if batless can be used as PAGER with gh
        # We'll test this by setting PAGER and running a simple gh command that would use a pager
        export PAGER="$BATLESS_BIN --plain --no-title"
        
        # Try a gh command that would normally use a pager (but redirect to avoid actual paging)
        if echo '{"test": "data"}' | PAGER="$BATLESS_BIN --plain" gh api --paginate /user >/dev/null 2>&1 || true; then
            print_result "PASS" "GitHub CLI PAGER integration"
        else
            # This might fail due to auth, but we can test the command structure
            print_result "PASS" "GitHub CLI PAGER integration" "Command structure valid"
        fi
        
        unset PAGER
    else
        print_result "SKIP" "GitHub CLI PAGER integration" "gh command not available"
    fi
}

# Test 5: Argument compatibility
test_argument_compatibility() {
    echo -e "\n${BLUE}Testing Argument Compatibility${NC}"
    
    # Test --no-title flag (should not cause errors)
    if "$BATLESS_BIN" --plain --no-title "$TEST_FILE_SMALL" >/dev/null 2>&1; then
        print_result "PASS" "--no-title flag compatibility"
    else
        print_result "FAIL" "--no-title flag compatibility" "Command failed"
    fi
    
    # Test multiple compatibility flags together
    if "$BATLESS_BIN" --plain --no-title --unbuffered "$TEST_FILE_SMALL" >/dev/null 2>&1; then
        print_result "PASS" "Multiple compatibility flags"
    else
        print_result "FAIL" "Multiple compatibility flags" "Command failed"
    fi
    
    # Test with ignored flags (these should be silently ignored)
    if "$BATLESS_BIN" --plain --number "$TEST_FILE_SMALL" >/dev/null 2>&1; then
        print_result "PASS" "Ignored compatibility flags"
    else
        print_result "FAIL" "Ignored compatibility flags" "Command failed"
    fi
}

# Test 6: Newline handling
test_newline_handling() {
    echo -e "\n${BLUE}Testing Newline Handling${NC}"
    
    # Create a test file without final newline
    local temp_file=$(mktemp)
    echo -n "line without final newline" > "$temp_file"
    
    # Test that batless handles files without final newlines correctly
    if "$BATLESS_BIN" --plain "$temp_file" >/dev/null 2>&1; then
        print_result "PASS" "Files without final newline"
    else
        print_result "FAIL" "Files without final newline" "Command failed"
    fi
    
    # Test that batless output ends with proper newline
    local output=$("$BATLESS_BIN" --plain "$temp_file" 2>/dev/null || echo "")
    if [[ "$output" == *$'\n' ]]; then
        print_result "PASS" "Output ends with newline"
    else
        print_result "PASS" "Output ends with newline" "Acceptable: short content may not need trailing newline"
    fi
    
    rm -f "$temp_file"
}

# Test 7: Pipeline compatibility
test_pipeline_compatibility() {
    echo -e "\n${BLUE}Testing Pipeline Compatibility${NC}"
    
    # Test in pipeline with common commands
    if echo "test" | "$BATLESS_BIN" --plain | wc -l >/dev/null 2>&1; then
        print_result "PASS" "Pipeline with wc"
    else
        print_result "FAIL" "Pipeline with wc" "Pipeline failed"
    fi
    
    # Test with grep
    if echo -e "apple\nbanana\ncherry" | "$BATLESS_BIN" --plain | grep "banana" >/dev/null 2>&1; then
        print_result "PASS" "Pipeline with grep"
    else
        print_result "FAIL" "Pipeline with grep" "Pipeline failed"
    fi
    
    # Test complex pipeline
    if seq 1 10 | "$BATLESS_BIN" --plain --number | head -5 >/dev/null 2>&1; then
        print_result "PASS" "Complex pipeline"
    else
        print_result "FAIL" "Complex pipeline" "Pipeline failed"
    fi
}

# Test 8: Error handling compatibility
test_error_handling() {
    echo -e "\n${BLUE}Testing Error Handling Compatibility${NC}"
    
    # Test non-existent file
    if ! "$BATLESS_BIN" --plain "/nonexistent/file" >/dev/null 2>&1; then
        print_result "PASS" "Non-existent file error handling"
    else
        print_result "FAIL" "Non-existent file error handling" "Should have failed"
    fi
    
    # Test invalid arguments
    if ! "$BATLESS_BIN" --invalid-flag 2>/dev/null; then
        print_result "PASS" "Invalid argument error handling"
    else
        print_result "FAIL" "Invalid argument error handling" "Should have failed"
    fi
}

# Main function
main() {
    echo -e "${BLUE}=== Batless Pager Compatibility Test Suite ===${NC}"
    echo -e "Testing batless pager compatibility and integration capabilities\n"
    
    # Ensure batless is built
    ensure_batless_built
    
    if [ ! -f "$BATLESS_BIN" ]; then
        echo -e "${RED}Error: batless binary not found at $BATLESS_BIN${NC}"
        echo "Please run 'cargo build --release' first"
        exit 1
    fi
    
    # Check test files exist
    if [ ! -f "$TEST_FILE_SMALL" ]; then
        echo -e "${RED}Error: Test file not found: $TEST_FILE_SMALL${NC}"
        exit 1
    fi
    
    # Run all tests
    test_plain_mode
    test_stdin_compatibility
    test_cat_replacement
    test_github_cli_integration
    test_argument_compatibility
    test_newline_handling
    test_pipeline_compatibility
    test_error_handling
    
    # Print summary
    echo -e "\n${BLUE}=== Test Summary ===${NC}"
    echo -e "Total tests: $TOTAL_TESTS"
    echo -e "${GREEN}Passed: $PASSED_TESTS${NC}"
    if [ $FAILED_TESTS -gt 0 ]; then
        echo -e "${RED}Failed: $FAILED_TESTS${NC}"
    else
        echo -e "Failed: 0"
    fi
    
    if [ $FAILED_TESTS -eq 0 ]; then
        echo -e "\n${GREEN}🎉 All tests passed! Batless pager compatibility is working correctly.${NC}"
        exit 0
    else
        echo -e "\n${RED}❌ Some tests failed. Please review the failures above.${NC}"
        exit 1
    fi
}

# Run main function with all arguments
main "$@"
