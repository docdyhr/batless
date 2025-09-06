#!/bin/bash
# Workflow Performance Monitoring and Optimization Script
# Purpose: Analyze workflow performance and suggest optimizations

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${CYAN}         Workflow Performance Analysis Tool${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

# Function to format duration
format_duration() {
    local minutes=$1
    if (( $(echo "$minutes < 1" | bc -l) )); then
        echo "$(printf "%.0f" $(echo "$minutes * 60" | bc)) seconds"
    else
        echo "$(printf "%.1f" "$minutes") minutes"
    fi
}

# Function to get workflow performance data
analyze_workflows() {
    echo -e "\n${BLUE}📊 Workflow Performance Metrics${NC}"
    echo "================================"

    # Get all unique workflow names
    local workflows=$(gh api /repos/:owner/:repo/actions/workflows --jq '.workflows[].name')

    while IFS= read -r workflow_name; do
        if [ -z "$workflow_name" ]; then
            continue
        fi

        echo -e "\n${YELLOW}📋 $workflow_name${NC}"

        # Get last 10 runs for this workflow
        local runs=$(gh run list --workflow "$workflow_name" --limit 10 --json conclusion,createdAt,updatedAt,status 2>/dev/null || echo "[]")

        if [ "$runs" = "[]" ]; then
            echo "  No recent runs"
            continue
        fi

        # Calculate metrics
        local total_runs=$(echo "$runs" | jq '. | length')
        local successful=$(echo "$runs" | jq '[.[] | select(.conclusion == "success")] | length')
        local avg_duration=$(echo "$runs" | jq '[.[] | select(.conclusion == "success") |
            (((.updatedAt | fromdateiso8601) - (.createdAt | fromdateiso8601)) / 60)] |
            if length > 0 then add / length else 0 end')

        local success_rate=0
        if [ "$total_runs" -gt 0 ]; then
            success_rate=$((successful * 100 / total_runs))
        fi

        echo "  Success Rate: ${success_rate}%"
        echo "  Avg Duration: $(format_duration "$avg_duration")"

        # Performance rating
        if (( $(echo "$avg_duration > 20" | bc -l) )); then
            echo -e "  Performance: ${RED}⚠️  Needs Optimization${NC}"
        elif (( $(echo "$avg_duration > 10" | bc -l) )); then
            echo -e "  Performance: ${YELLOW}⚡ Could Be Faster${NC}"
        else
            echo -e "  Performance: ${GREEN}🚀 Excellent${NC}"
        fi
    done <<< "$workflows"
}

# Function to analyze job parallelization
analyze_parallelization() {
    echo -e "\n${BLUE}🔀 Parallelization Analysis${NC}"
    echo "============================"

    # Check for matrix strategies in workflows
    local matrix_count=$(grep -r "matrix:" .github/workflows/*.yml 2>/dev/null | wc -l || echo "0")
    local workflow_count=$(ls -1 .github/workflows/*.yml 2>/dev/null | wc -l || echo "0")

    echo "Workflows with matrix builds: $matrix_count/$workflow_count"

    if [ "$matrix_count" -lt "$workflow_count" ]; then
        echo -e "${YELLOW}💡 Tip: Consider adding matrix strategies to parallelize tests${NC}"
    fi

    # Check for job dependencies
    local needs_count=$(grep -r "needs:" .github/workflows/*.yml 2>/dev/null | wc -l || echo "0")
    echo "Jobs with dependencies: $needs_count"

    if [ "$needs_count" -gt 10 ]; then
        echo -e "${YELLOW}💡 Tip: Many job dependencies may create bottlenecks${NC}"
    fi
}

# Function to analyze caching
analyze_caching() {
    echo -e "\n${BLUE}💾 Cache Analysis${NC}"
    echo "=================="

    # Check for cache usage
    local cache_actions=$(grep -r "actions/cache\|Swatinem/rust-cache" .github/workflows/*.yml 2>/dev/null | wc -l || echo "0")

    if [ "$cache_actions" -gt 0 ]; then
        echo -e "${GREEN}✅ Caching is configured ($cache_actions instances)${NC}"

        # Check cache hit rates (if available through API)
        echo "Checking cache effectiveness..."

        # Look for restore-keys
        local restore_keys=$(grep -r "restore-keys:" .github/workflows/*.yml 2>/dev/null | wc -l || echo "0")
        if [ "$restore_keys" -gt 0 ]; then
            echo -e "${GREEN}✅ Cache restore keys configured${NC}"
        else
            echo -e "${YELLOW}💡 Tip: Add restore-keys for better cache hits${NC}"
        fi
    else
        echo -e "${RED}❌ No caching detected${NC}"
        echo -e "${YELLOW}💡 Tip: Add caching to speed up builds significantly${NC}"
    fi
}

# Function to analyze concurrency
analyze_concurrency() {
    echo -e "\n${BLUE}🔄 Concurrency Configuration${NC}"
    echo "============================="

    local concurrency_configs=$(grep -r "concurrency:" .github/workflows/*.yml 2>/dev/null | wc -l || echo "0")

    if [ "$concurrency_configs" -gt 0 ]; then
        echo -e "${GREEN}✅ Concurrency controls configured${NC}"

        # Check for cancel-in-progress
        local cancel_configs=$(grep -r "cancel-in-progress:" .github/workflows/*.yml 2>/dev/null | wc -l || echo "0")
        if [ "$cancel_configs" -gt 0 ]; then
            echo -e "${GREEN}✅ Auto-cancellation enabled${NC}"
        else
            echo -e "${YELLOW}💡 Tip: Add cancel-in-progress to stop outdated runs${NC}"
        fi
    else
        echo -e "${RED}❌ No concurrency controls${NC}"
        echo -e "${YELLOW}💡 Tip: Add concurrency groups to prevent redundant runs${NC}"
    fi
}

# Function to generate optimization recommendations
generate_recommendations() {
    echo -e "\n${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}                 Optimization Recommendations${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

    echo -e "\n${BLUE}🚀 Quick Wins:${NC}"
    echo "1. Enable workflow concurrency with cancel-in-progress"
    echo "2. Use Swatinem/rust-cache@v2 for Rust projects"
    echo "3. Split tests into parallel shards"
    echo "4. Use ubuntu-latest for fastest runners"
    echo "5. Set timeout-minutes on all jobs"

    echo -e "\n${BLUE}📈 Advanced Optimizations:${NC}"
    echo "1. Use matrix strategies for multi-platform testing"
    echo "2. Implement job outputs to share data between jobs"
    echo "3. Use composite actions for repeated steps"
    echo "4. Consider self-hosted runners for resource-intensive jobs"
    echo "5. Use workflow_dispatch for manual triggers"

    echo -e "\n${BLUE}⚡ Performance Targets:${NC}"
    echo "• PR validation: < 5 minutes"
    echo "• Full CI suite: < 15 minutes"
    echo "• Security scans: < 3 minutes"
    echo "• Documentation builds: < 2 minutes"
}

# Function to show example optimized workflow
show_optimization_example() {
    echo -e "\n${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}                 Example Optimized Configuration${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

    cat << 'EOF'

# Add to your workflow:

concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: ${{ github.ref != 'refs/heads/main' }}

jobs:
  quick-check:
    runs-on: ubuntu-latest
    timeout-minutes: 5
    steps:
      - uses: actions/checkout@v5
        with:
          fetch-depth: 1  # Shallow clone for speed

      - uses: Swatinem/rust-cache@v2
        with:
          cache-all-crates: true
          save-if: ${{ github.ref == 'refs/heads/main' }}

  test-matrix:
    strategy:
      fail-fast: false
      matrix:
        shard: [1, 2, 3]  # Split tests into shards
    runs-on: ubuntu-latest
    timeout-minutes: 10
    steps:
      - run: cargo test --shard ${{ matrix.shard }}/3
EOF
}

# Main execution
main() {
    # Check if gh is available
    if ! command -v gh &> /dev/null; then
        echo -e "${RED}Error: GitHub CLI (gh) is required${NC}"
        echo "Install with: brew install gh"
        exit 1
    fi

    # Run analyses
    analyze_workflows
    analyze_parallelization
    analyze_caching
    analyze_concurrency

    # Generate recommendations
    generate_recommendations

    # Show example if requested
    if [ "${1:-}" = "--show-example" ]; then
        show_optimization_example
    fi

    echo -e "\n${GREEN}✨ Analysis complete!${NC}"
    echo -e "${CYAN}Run with --show-example to see optimization examples${NC}"
}

# Run main function
main "$@"
