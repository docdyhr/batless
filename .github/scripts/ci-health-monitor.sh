#!/bin/bash
# CI/CD Health Monitoring and Tuning Script
# Provides insights into CI performance and suggests optimizations

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

# Configuration
REPO="docdyhr/batless"
SAMPLE_SIZE=20
THRESHOLD_WARNING=25  # minutes
THRESHOLD_CRITICAL=35  # minutes

echo -e "${CYAN}===========================================================================${NC}"
echo -e "${CYAN}                    CI/CD Health Monitor${NC}"
echo -e "${CYAN}===========================================================================${NC}"
echo ""

# Function to calculate statistics
calculate_stats() {
    local values="$1"
    local count=$(echo "$values" | wc -l | xargs)

    if [ "$count" -eq 0 ]; then
        echo "0,0,0,0"
        return
    fi

    local sum=$(echo "$values" | awk '{s+=$1} END {print s}')
    local mean=$(echo "scale=2; $sum / $count" | bc)

    # Calculate median
    local median=$(echo "$values" | sort -n | awk '{a[NR]=$1} END {if(NR%2==1) print a[(NR+1)/2]; else print (a[NR/2]+a[NR/2+1])/2}')

    # Calculate min/max
    local min=$(echo "$values" | sort -n | head -1)
    local max=$(echo "$values" | sort -n | tail -1)

    echo "$mean,$median,$min,$max"
}

# Get recent workflow runs
echo -e "${BLUE}📊 Analyzing last $SAMPLE_SIZE workflow runs...${NC}"
echo ""

# Fetch workflow runs
runs_data=$(gh api "/repos/$REPO/actions/runs?per_page=$SAMPLE_SIZE" --jq '.workflow_runs[] | "\(.id),\(.name),\(.conclusion),\(.created_at),\(.updated_at)"')

# Initialize counters
total_runs=0
successful_runs=0
failed_runs=0
cancelled_runs=0

declare -A workflow_durations
declare -A workflow_success
declare -A workflow_failures

# Process each run
while IFS=',' read -r id name conclusion created updated; do
    total_runs=$((total_runs + 1))

    # Count by conclusion
    case "$conclusion" in
        success)
            successful_runs=$((successful_runs + 1))
            ;;
        failure)
            failed_runs=$((failed_runs + 1))
            ;;
        cancelled)
            cancelled_runs=$((cancelled_runs + 1))
            ;;
    esac

    # Calculate duration in minutes
    if [ -n "$created" ] && [ -n "$updated" ]; then
        created_ts=$(date -jf "%Y-%m-%dT%H:%M:%SZ" "$created" +%s 2>/dev/null || date -d "$created" +%s 2>/dev/null || echo 0)
        updated_ts=$(date -jf "%Y-%m-%dT%H:%M:%SZ" "$updated" +%s 2>/dev/null || date -d "$updated" +%s 2>/dev/null || echo 0)

        if [ "$created_ts" -gt 0 ] && [ "$updated_ts" -gt 0 ]; then
            duration=$(( (updated_ts - created_ts) / 60 ))

            # Store duration by workflow
            if [ -z "${workflow_durations[$name]}" ]; then
                workflow_durations[$name]="$duration"
            else
                workflow_durations[$name]="${workflow_durations[$name]}\n$duration"
            fi

            # Count successes/failures by workflow
            if [ "$conclusion" = "success" ]; then
                workflow_success[$name]=$((${workflow_success[$name]:-0} + 1))
            else
                workflow_failures[$name]=$((${workflow_failures[$name]:-0} + 1))
            fi
        fi
    fi
done <<< "$runs_data"

# Calculate overall success rate
if [ $total_runs -gt 0 ]; then
    success_rate=$(echo "scale=1; $successful_runs * 100 / $total_runs" | bc)
else
    success_rate=0
fi

# Display overall metrics
echo -e "${GREEN}✅ Overall Health Metrics${NC}"
echo "─────────────────────────────────────────────────────────────────────────"
printf "Total Runs:       %d\n" $total_runs
printf "Successful:       %d (%.1f%%)\n" $successful_runs $success_rate
printf "Failed:           %d\n" $failed_runs
printf "Cancelled:        %d\n" $cancelled_runs

# Health status
if (( $(echo "$success_rate >= 90" | bc -l) )); then
    echo -e "Health Status:    ${GREEN}EXCELLENT ✓${NC}"
elif (( $(echo "$success_rate >= 75" | bc -l) )); then
    echo -e "Health Status:    ${YELLOW}GOOD${NC}"
else
    echo -e "Health Status:    ${RED}NEEDS ATTENTION${NC}"
fi
echo ""

# Per-workflow analysis
echo -e "${BLUE}📈 Per-Workflow Performance${NC}"
echo "─────────────────────────────────────────────────────────────────────────"

for workflow in "${!workflow_durations[@]}"; do
    durations=$(echo -e "${workflow_durations[$workflow]}")
    stats=$(calculate_stats "$durations")

    IFS=',' read -r mean median min max <<< "$stats"

    success_count=${workflow_success[$workflow]:-0}
    failure_count=${workflow_failures[$workflow]:-0}
    total_count=$((success_count + failure_count))

    if [ $total_count -gt 0 ]; then
        wf_success_rate=$(echo "scale=0; $success_count * 100 / $total_count" | bc)
    else
        wf_success_rate=0
    fi

    printf "\n${CYAN}%s${NC}\n" "$workflow"
    printf "  Average Duration: %.1f min\n" $mean
    printf "  Median Duration:  %.1f min\n" $median
    printf "  Range:            %.1f - %.1f min\n" $min $max
    printf "  Success Rate:     %d%% (%d/%d)\n" $wf_success_rate $success_count $total_count

    # Performance warnings
    if (( $(echo "$mean > $THRESHOLD_CRITICAL" | bc -l) )); then
        echo -e "  ${RED}⚠️  CRITICAL: Average duration exceeds ${THRESHOLD_CRITICAL}min${NC}"
    elif (( $(echo "$mean > $THRESHOLD_WARNING" | bc -l) )); then
        echo -e "  ${YELLOW}⚠️  WARNING: Average duration exceeds ${THRESHOLD_WARNING}min${NC}"
    else
        echo -e "  ${GREEN}✓ Performance within acceptable range${NC}"
    fi
done

echo ""
echo -e "${GREEN}💡 Recommendations${NC}"
echo "─────────────────────────────────────────────────────────────────────────"

# Provide recommendations based on analysis
if [ $failed_runs -gt 3 ]; then
    echo -e "${YELLOW}•${NC} High failure rate detected. Review recent failures for common issues."
fi

if (( $(echo "$success_rate < 80" | bc -l) )); then
    echo -e "${YELLOW}•${NC} Success rate below 80%. Consider investigating flaky tests."
fi

# Check for slow workflows
slow_workflows=0
for workflow in "${!workflow_durations[@]}"; do
    durations=$(echo -e "${workflow_durations[$workflow]}")
    stats=$(calculate_stats "$durations")
    mean=$(echo "$stats" | cut -d',' -f1)

    if (( $(echo "$mean > $THRESHOLD_WARNING" | bc -l) )); then
        slow_workflows=$((slow_workflows + 1))
    fi
done

if [ $slow_workflows -gt 0 ]; then
    echo -e "${YELLOW}•${NC} $slow_workflows workflow(s) running slower than expected."
    echo "  Consider:"
    echo "    - Enabling more aggressive caching"
    echo "    - Splitting long-running jobs into parallel shards"
    echo "    - Using faster runners or spot instances"
fi

if [ $total_runs -lt 10 ]; then
    echo -e "${YELLOW}•${NC} Limited sample size. Run more workflows for better insights."
fi

# Specific optimizations
echo ""
echo -e "${CYAN}🔧 Optimization Suggestions${NC}"
echo "─────────────────────────────────────────────────────────────────────────"
echo "• Use 'Swatinem/rust-cache@v2' for efficient Cargo caching"
echo "• Set 'CARGO_INCREMENTAL=0' for CI builds (faster from scratch)"
echo "• Consider 'cargo-nextest' for faster parallel test execution"
echo "• Use workflow concurrency groups to cancel redundant runs"
echo "• Enable 'fetch-depth: 1' for shallow clones (faster checkout)"
echo ""

echo -e "${GREEN}✅ Health check complete!${NC}"
echo ""
