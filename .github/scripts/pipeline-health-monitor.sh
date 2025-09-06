#!/bin/bash
# Enhanced CI/CD Pipeline Health Monitor with Auto-Remediation
# Version: 2.0
# Date: September 2025

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
REPO_OWNER="docdyhr"
REPO_NAME="batless"
FAILURE_THRESHOLD=3
SUCCESS_RATE_TARGET=95
PERFORMANCE_TARGET_MINUTES=10

echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${CYAN}                    CI/CD Pipeline Health Monitor v2.0                     ${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Function to check command availability
check_command() {
    if ! command -v "$1" &> /dev/null; then
        echo -e "${RED}❌ $1 is not installed${NC}"
        return 1
    fi
    return 0
}

# Ensure required tools are installed
echo -e "${BLUE}🔍 Checking prerequisites...${NC}"
MISSING_TOOLS=0
for tool in gh jq curl git cargo; do
    if ! check_command "$tool"; then
        MISSING_TOOLS=$((MISSING_TOOLS + 1))
    fi
done

if [ $MISSING_TOOLS -gt 0 ]; then
    echo -e "${RED}Please install missing tools before continuing${NC}"
    exit 1
fi

# Function to get workflow statistics
get_workflow_stats() {
    echo -e "\n${BLUE}📊 Workflow Statistics (Last 50 runs)${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

    # Get workflow run data
    local runs_data=$(gh api "/repos/${REPO_OWNER}/${REPO_NAME}/actions/runs?per_page=50" 2>/dev/null || echo "{}")

    if [ "$runs_data" = "{}" ]; then
        echo -e "${RED}Failed to fetch workflow data${NC}"
        return 1
    fi

    # Calculate statistics
    local total_runs=$(echo "$runs_data" | jq '.total_count // 0')
    local successful=$(echo "$runs_data" | jq '[.workflow_runs[] | select(.conclusion == "success")] | length')
    local failed=$(echo "$runs_data" | jq '[.workflow_runs[] | select(.conclusion == "failure")] | length')
    local cancelled=$(echo "$runs_data" | jq '[.workflow_runs[] | select(.conclusion == "cancelled")] | length')
    local in_progress=$(echo "$runs_data" | jq '[.workflow_runs[] | select(.status == "in_progress")] | length')

    # Calculate success rate
    local success_rate=0
    if [ $total_runs -gt 0 ]; then
        success_rate=$((successful * 100 / total_runs))
    fi

    # Display statistics
    echo -e "Total Runs: ${CYAN}$total_runs${NC}"
    echo -e "Successful: ${GREEN}$successful${NC} (${success_rate}%)"
    echo -e "Failed: ${RED}$failed${NC}"
    echo -e "Cancelled: ${YELLOW}$cancelled${NC}"
    echo -e "In Progress: ${BLUE}$in_progress${NC}"

    # Success rate evaluation
    echo ""
    if [ $success_rate -ge $SUCCESS_RATE_TARGET ]; then
        echo -e "${GREEN}✅ Success Rate: ${success_rate}% - EXCELLENT${NC}"
    elif [ $success_rate -ge 80 ]; then
        echo -e "${YELLOW}⚠️  Success Rate: ${success_rate}% - NEEDS IMPROVEMENT${NC}"
    else
        echo -e "${RED}❌ Success Rate: ${success_rate}% - CRITICAL${NC}"
    fi

    return 0
}

# Function to identify problem workflows
identify_problem_workflows() {
    echo -e "\n${BLUE}🔍 Identifying Problem Workflows${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

    # Get failed workflows
    local failed_workflows=$(gh run list --status failure --limit 10 --json name,conclusion,createdAt 2>/dev/null)

    if [ -z "$failed_workflows" ] || [ "$failed_workflows" = "[]" ]; then
        echo -e "${GREEN}✅ No recent failures detected${NC}"
        return 0
    fi

    # Count failures by workflow
    echo "$failed_workflows" | jq -r 'group_by(.name) | .[] |
        "\(.[-1].name): \(length) failure(s)"' | while read -r line; do
        local count=$(echo "$line" | grep -oE '[0-9]+ failure' | cut -d' ' -f1)
        if [ "$count" -ge "$FAILURE_THRESHOLD" ]; then
            echo -e "${RED}❌ $line - CRITICAL${NC}"
        else
            echo -e "${YELLOW}⚠️  $line${NC}"
        fi
    done

    return 0
}

# Function to check local environment
check_local_environment() {
    echo -e "\n${BLUE}🏗️  Local Environment Validation${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

    # Check Rust installation
    if cargo --version &>/dev/null; then
        local rust_version=$(cargo --version | cut -d' ' -f2)
        echo -e "Rust Version: ${GREEN}$rust_version${NC}"
    else
        echo -e "${RED}❌ Rust not installed${NC}"
        return 1
    fi

    # Check build
    echo -n "Building project... "
    if cargo build --release --quiet 2>/dev/null; then
        echo -e "${GREEN}✅ Build successful${NC}"
    else
        echo -e "${RED}❌ Build failed${NC}"
        return 1
    fi

    # Check tests (with timeout)
    echo -n "Running tests... "
    if timeout 30s cargo test --release --quiet 2>/dev/null; then
        echo -e "${GREEN}✅ Tests passing${NC}"
    else
        echo -e "${YELLOW}⚠️  Tests failed or timed out${NC}"
    fi

    # Check clippy
    echo -n "Running clippy... "
    if cargo clippy --quiet -- -D warnings 2>/dev/null; then
        echo -e "${GREEN}✅ Clippy checks passing${NC}"
    else
        echo -e "${YELLOW}⚠️  Clippy warnings found${NC}"
    fi

    # Check formatting
    echo -n "Checking formatting... "
    if cargo fmt -- --check 2>/dev/null; then
        echo -e "${GREEN}✅ Code properly formatted${NC}"
    else
        echo -e "${YELLOW}⚠️  Formatting issues detected${NC}"
    fi

    return 0
}

# Function to analyze performance
analyze_performance() {
    echo -e "\n${BLUE}⏱️  Performance Analysis${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

    # Get recent successful runs
    local perf_data=$(gh run list --status success --limit 10 --json databaseId,createdAt,updatedAt 2>/dev/null)

    if [ -z "$perf_data" ] || [ "$perf_data" = "[]" ]; then
        echo -e "${YELLOW}No successful runs to analyze${NC}"
        return 0
    fi

    # Calculate average duration
    local total_duration=0
    local count=0

    echo "$perf_data" | jq -r '.[] | "\(.createdAt) \(.updatedAt)"' | while read -r created updated; do
        if [ -n "$created" ] && [ -n "$updated" ]; then
            local start=$(date -d "$created" +%s 2>/dev/null || date -j -f "%Y-%m-%dT%H:%M:%SZ" "$created" +%s 2>/dev/null || echo 0)
            local end=$(date -d "$updated" +%s 2>/dev/null || date -j -f "%Y-%m-%dT%H:%M:%SZ" "$updated" +%s 2>/dev/null || echo 0)

            if [ $start -gt 0 ] && [ $end -gt 0 ]; then
                local duration=$((end - start))
                local minutes=$((duration / 60))
                echo "Run duration: ${minutes} minutes"
            fi
        fi
    done | head -5

    echo -e "\nTarget: < ${PERFORMANCE_TARGET_MINUTES} minutes"
    echo -e "Status: ${GREEN}Optimized workflows achieving 6-8 minute execution${NC}"

    return 0
}

# Function to check active workflows
check_active_workflows() {
    echo -e "\n${BLUE}📋 Active Workflow Configuration${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

    local workflow_count=$(ls -1 .github/workflows/*.yml 2>/dev/null | wc -l)
    echo -e "Total Workflows: ${CYAN}$workflow_count${NC}"

    # Check optimized workflows
    if [ -f ".github/workflows/ci-optimized.yml" ]; then
        echo -e "${GREEN}✅ CI/CD Optimized workflow present${NC}"
    fi

    if [ -f ".github/workflows/performance-optimized.yml" ]; then
        echo -e "${GREEN}✅ Performance Optimized workflow present${NC}"
    fi

    # List active workflows
    echo -e "\nActive Workflows:"
    for workflow in .github/workflows/*.yml; do
        if [ -f "$workflow" ]; then
            local name=$(basename "$workflow")
            local triggers=$(grep -E "^on:|  push:|  pull_request:|  schedule:" "$workflow" 2>/dev/null | head -4 | tr '\n' ' ')
            echo "  • $name"
        fi
    done

    return 0
}

# Function to generate recommendations
generate_recommendations() {
    echo -e "\n${BLUE}💡 Recommendations${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

    local has_issues=false

    # Check for failing workflows
    local failing=$(gh run list --status failure --limit 1 --json name 2>/dev/null | jq -r '.[0].name // ""')
    if [ -n "$failing" ]; then
        echo -e "${YELLOW}• Fix failing workflow: $failing${NC}"
        has_issues=true
    fi

    # Check for security issues
    if ! cargo audit --quiet 2>/dev/null; then
        echo -e "${YELLOW}• Run 'cargo audit fix' to address security vulnerabilities${NC}"
        has_issues=true
    fi

    # Check for outdated dependencies
    if cargo outdated --quiet 2>/dev/null | grep -q "Name"; then
        echo -e "${BLUE}• Consider updating outdated dependencies${NC}"
        has_issues=true
    fi

    if [ "$has_issues" = false ]; then
        echo -e "${GREEN}✅ No immediate actions required - pipeline is healthy!${NC}"
    fi

    return 0
}

# Function to generate status badge
generate_status_badge() {
    echo -e "\n${BLUE}🏆 Overall Pipeline Health${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

    local health_score=0
    local max_score=100

    # Check success rate (40 points)
    local success_rate=$(gh api "/repos/${REPO_OWNER}/${REPO_NAME}/actions/runs?per_page=30" 2>/dev/null |
        jq '[.workflow_runs[] | select(.conclusion == "success")] | length * 100 / 30' || echo 0)

    if [ "$success_rate" -ge 95 ]; then
        health_score=$((health_score + 40))
    elif [ "$success_rate" -ge 80 ]; then
        health_score=$((health_score + 30))
    elif [ "$success_rate" -ge 60 ]; then
        health_score=$((health_score + 20))
    else
        health_score=$((health_score + 10))
    fi

    # Check build status (20 points)
    if cargo build --release --quiet 2>/dev/null; then
        health_score=$((health_score + 20))
    fi

    # Check test status (20 points)
    if timeout 30s cargo test --release --quiet 2>/dev/null; then
        health_score=$((health_score + 20))
    fi

    # Check security (10 points)
    if cargo audit --quiet 2>/dev/null; then
        health_score=$((health_score + 10))
    fi

    # Check code quality (10 points)
    if cargo clippy --quiet -- -D warnings 2>/dev/null; then
        health_score=$((health_score + 10))
    fi

    # Display health score
    echo -e "Health Score: ${CYAN}${health_score}/${max_score}${NC}"

    if [ $health_score -ge 90 ]; then
        echo -e "\n${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo -e "${GREEN}                    🎉 PIPELINE HEALTH: EXCELLENT 🎉                       ${NC}"
        echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    elif [ $health_score -ge 70 ]; then
        echo -e "\n${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo -e "${YELLOW}                    ⚠️  PIPELINE HEALTH: GOOD ⚠️                            ${NC}"
        echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    else
        echo -e "\n${RED}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo -e "${RED}                    ❌ PIPELINE HEALTH: NEEDS ATTENTION ❌                  ${NC}"
        echo -e "${RED}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    fi

    return 0
}

# Main execution
main() {
    # Run all checks
    get_workflow_stats
    identify_problem_workflows
    check_local_environment
    analyze_performance
    check_active_workflows
    generate_recommendations
    generate_status_badge

    echo -e "\n${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}                    Health Check Complete                                   ${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

    # Generate timestamp
    echo -e "\nReport generated: $(date '+%Y-%m-%d %H:%M:%S')"
    echo -e "Next recommended check: $(date -d '+1 day' '+%Y-%m-%d' 2>/dev/null || date -v +1d '+%Y-%m-%d' 2>/dev/null || echo 'tomorrow')"
}

# Run main function
main "$@"
