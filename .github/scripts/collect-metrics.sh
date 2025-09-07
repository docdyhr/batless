#!/bin/bash
# Advanced CI/CD Metrics Collection and Analysis

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}🔍 Collecting CI/CD Pipeline Metrics...${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Configuration
REPO_OWNER="docdyhr"
REPO_NAME="batless"
METRICS_FILE="pipeline_metrics.json"
REPORT_FILE="PIPELINE_METRICS_REPORT.md"

# Function to collect workflow statistics
collect_workflow_stats() {
    echo -e "\n${BLUE}📊 Collecting Workflow Statistics...${NC}"

    # Get workflow run statistics (last 100 runs)
    gh api "/repos/${REPO_OWNER}/${REPO_NAME}/actions/runs?per_page=100" \
      --jq '.workflow_runs |
        group_by(.conclusion) |
        map({conclusion: (.[0].conclusion // "unknown"), count: length})' \
      > workflow_stats.json

    echo "✅ Workflow statistics collected"
}

# Function to calculate success rates
calculate_success_rate() {
    echo -e "\n${BLUE}📈 Calculating Success Rates...${NC}"

    local total_runs=$(gh api "/repos/${REPO_OWNER}/${REPO_NAME}/actions/runs?per_page=100" \
      --jq '.workflow_runs | length')

    local successful_runs=$(gh api "/repos/${REPO_OWNER}/${REPO_NAME}/actions/runs?per_page=100" \
      --jq '.workflow_runs | map(select(.conclusion == "success")) | length')

    local failed_runs=$(gh api "/repos/${REPO_OWNER}/${REPO_NAME}/actions/runs?per_page=100" \
      --jq '.workflow_runs | map(select(.conclusion == "failure")) | length')

    local cancelled_runs=$(gh api "/repos/${REPO_OWNER}/${REPO_NAME}/actions/runs?per_page=100" \
      --jq '.workflow_runs | map(select(.conclusion == "cancelled")) | length')

    local success_rate=0
    if [ "$total_runs" -gt 0 ]; then
        success_rate=$(echo "scale=1; $successful_runs * 100 / $total_runs" | bc -l 2>/dev/null || echo "0")
    fi

    # Create summary JSON
    cat > "$METRICS_FILE" << EOF
{
  "timestamp": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "repository": "${REPO_OWNER}/${REPO_NAME}",
  "metrics": {
    "total_runs": $total_runs,
    "successful_runs": $successful_runs,
    "failed_runs": $failed_runs,
    "cancelled_runs": $cancelled_runs,
    "success_rate": $success_rate
  }
}
EOF

    echo "✅ Success rate calculated: ${success_rate}%"
}

# Function to analyze performance trends
analyze_performance() {
    echo -e "\n${BLUE}⏱️  Analyzing Performance Trends...${NC}"

    # Get recent successful runs with timing data
    gh api "/repos/${REPO_OWNER}/${REPO_NAME}/actions/runs?status=success&per_page=10" \
      --jq '.workflow_runs | map({
        name: .name,
        created_at: .created_at,
        updated_at: .updated_at,
        duration_minutes: (((.updated_at | fromdateiso8601) - (.created_at | fromdateiso8601)) / 60 | floor)
      })' > performance_data.json

    # Calculate average duration
    local avg_duration=$(jq '[.[].duration_minutes] | add / length | floor' performance_data.json 2>/dev/null || echo "0")

    echo "✅ Average workflow duration: ${avg_duration} minutes"

    # Add to metrics
    jq --argjson avg "$avg_duration" '.metrics.average_duration_minutes = $avg' "$METRICS_FILE" > tmp.json && mv tmp.json "$METRICS_FILE"
}

# Function to identify problem workflows
identify_problem_workflows() {
    echo -e "\n${BLUE}🔍 Identifying Problem Workflows...${NC}"

    # Get failed workflows grouped by name
    gh api "/repos/${REPO_OWNER}/${REPO_NAME}/actions/runs?status=failure&per_page=50" \
      --jq '.workflow_runs | group_by(.name) |
        map({workflow: .[0].name, failure_count: length}) |
        sort_by(.failure_count) | reverse' > problem_workflows.json

    local problem_count=$(jq 'length' problem_workflows.json)
    echo "✅ Identified ${problem_count} workflows with failures"

    # Add to metrics
    jq --slurpfile problems problem_workflows.json '.problem_workflows = $problems[0]' "$METRICS_FILE" > tmp.json && mv tmp.json "$METRICS_FILE"
}

# Function to analyze active workflows
analyze_active_workflows() {
    echo -e "\n${BLUE}📋 Analyzing Active Workflows...${NC}"

    local workflow_count=$(ls -1 .github/workflows/*.yml 2>/dev/null | wc -l)
    local optimized_workflows=0

    # Check for optimized workflows
    [ -f ".github/workflows/ci-optimized.yml" ] && optimized_workflows=$((optimized_workflows + 1))
    [ -f ".github/workflows/performance-optimized.yml" ] && optimized_workflows=$((optimized_workflows + 1))

    echo "✅ Total workflows: ${workflow_count}, Optimized: ${optimized_workflows}"

    # Add to metrics
    jq --argjson total "$workflow_count" --argjson optimized "$optimized_workflows" \
       '.metrics.total_workflows = $total | .metrics.optimized_workflows = $optimized' \
       "$METRICS_FILE" > tmp.json && mv tmp.json "$METRICS_FILE"
}

# Function to check local environment health
check_local_health() {
    echo -e "\n${BLUE}🏗️  Checking Local Environment Health...${NC}"

    local build_status="failure"
    local test_status="failure"
    local security_status="failure"
    local format_status="failure"

    # Check build
    if cargo build --release --quiet 2>/dev/null; then
        build_status="success"
    fi

    # Check tests (with timeout)
    if timeout 30s cargo test --release --quiet 2>/dev/null; then
        test_status="success"
    fi

    # Check security
    if cargo audit --quiet 2>/dev/null; then
        security_status="success"
    fi

    # Check formatting
    if cargo fmt -- --check 2>/dev/null; then
        format_status="success"
    fi

    echo "✅ Local health check completed"

    # Add to metrics
    jq --arg build "$build_status" --arg test "$test_status" --arg security "$security_status" --arg format "$format_status" \
       '.local_health = {
          build: $build,
          tests: $test,
          security: $security,
          formatting: $format
        }' "$METRICS_FILE" > tmp.json && mv tmp.json "$METRICS_FILE"
}

# Function to generate health score
calculate_health_score() {
    echo -e "\n${BLUE}🏆 Calculating Pipeline Health Score...${NC}"

    local health_score=0
    local max_score=100

    # Success rate (40 points)
    local success_rate=$(jq -r '.metrics.success_rate' "$METRICS_FILE")
    if (( $(echo "$success_rate >= 95" | bc -l 2>/dev/null || echo 0) )); then
        health_score=$((health_score + 40))
    elif (( $(echo "$success_rate >= 80" | bc -l 2>/dev/null || echo 0) )); then
        health_score=$((health_score + 30))
    elif (( $(echo "$success_rate >= 60" | bc -l 2>/dev/null || echo 0) )); then
        health_score=$((health_score + 20))
    else
        health_score=$((health_score + 10))
    fi

    # Local health checks (40 points total - 10 each)
    local build_status=$(jq -r '.local_health.build' "$METRICS_FILE")
    local test_status=$(jq -r '.local_health.tests' "$METRICS_FILE")
    local security_status=$(jq -r '.local_health.security' "$METRICS_FILE")
    local format_status=$(jq -r '.local_health.formatting' "$METRICS_FILE")

    [ "$build_status" = "success" ] && health_score=$((health_score + 10))
    [ "$test_status" = "success" ] && health_score=$((health_score + 10))
    [ "$security_status" = "success" ] && health_score=$((health_score + 10))
    [ "$format_status" = "success" ] && health_score=$((health_score + 10))

    # Performance score (20 points)
    local avg_duration=$(jq -r '.metrics.average_duration_minutes' "$METRICS_FILE")
    if [ "$avg_duration" -lt 10 ]; then
        health_score=$((health_score + 20))
    elif [ "$avg_duration" -lt 20 ]; then
        health_score=$((health_score + 15))
    elif [ "$avg_duration" -lt 30 ]; then
        health_score=$((health_score + 10))
    else
        health_score=$((health_score + 5))
    fi

    echo "✅ Health score calculated: ${health_score}/${max_score}"

    # Add to metrics
    jq --argjson score "$health_score" --argjson max "$max_score" \
       '.health_score = {score: $score, max_score: $max, percentage: ($score * 100 / $max)}' \
       "$METRICS_FILE" > tmp.json && mv tmp.json "$METRICS_FILE"
}

# Function to generate markdown report
generate_report() {
    echo -e "\n${BLUE}📝 Generating Comprehensive Report...${NC}"

    local timestamp=$(jq -r '.timestamp' "$METRICS_FILE")
    local success_rate=$(jq -r '.metrics.success_rate' "$METRICS_FILE")
    local health_score=$(jq -r '.health_score.score' "$METRICS_FILE")
    local health_percentage=$(jq -r '.health_score.percentage' "$METRICS_FILE")
    local avg_duration=$(jq -r '.metrics.average_duration_minutes' "$METRICS_FILE")

    cat > "$REPORT_FILE" << EOF
# CI/CD Pipeline Metrics Report

**Generated**: $timestamp
**Repository**: ${REPO_OWNER}/${REPO_NAME}
**Branch**: $(git branch --show-current)

## 🏆 Pipeline Health Score: ${health_score}/100 (${health_percentage}%)

$(if [ "$health_score" -ge 90 ]; then
    echo "### ✅ **EXCELLENT** - Pipeline is performing exceptionally well"
elif [ "$health_score" -ge 70 ]; then
    echo "### ⚠️ **GOOD** - Pipeline is healthy with minor improvement opportunities"
else
    echo "### ❌ **NEEDS ATTENTION** - Pipeline requires immediate attention"
fi)

## 📊 Key Metrics

| Metric | Value | Status |
|--------|-------|---------|
| Success Rate | ${success_rate}% | $(if (( $(echo "$success_rate >= 80" | bc -l 2>/dev/null || echo 0) )); then echo "✅ Good"; else echo "⚠️ Needs Improvement"; fi) |
| Average Duration | ${avg_duration} minutes | $(if [ "$avg_duration" -lt 15 ]; then echo "✅ Fast"; else echo "⚠️ Could be faster"; fi) |
| Total Workflows | $(jq -r '.metrics.total_workflows' "$METRICS_FILE") | Active |
| Optimized Workflows | $(jq -r '.metrics.optimized_workflows' "$METRICS_FILE") | Deployed |

## 🔍 Workflow Analysis

### Recent Performance
$(jq -r '.workflow_runs[] | "- \(.name): \(.duration_minutes) minutes"' performance_data.json 2>/dev/null | head -5 || echo "- No recent performance data available")

### Problem Workflows
$(jq -r '.problem_workflows[] | "- \(.workflow): \(.failure_count) failures"' "$METRICS_FILE" 2>/dev/null | head -5 || echo "- No recent failures detected")

## 🏗️ Local Environment Status

| Component | Status |
|-----------|---------|
| Build | $(jq -r '.local_health.build' "$METRICS_FILE" | sed 's/success/✅ Passing/g; s/failure/❌ Failing/g') |
| Tests | $(jq -r '.local_health.tests' "$METRICS_FILE" | sed 's/success/✅ Passing/g; s/failure/❌ Failing/g') |
| Security | $(jq -r '.local_health.security' "$METRICS_FILE" | sed 's/success/✅ Secure/g; s/failure/⚠️ Issues Found/g') |
| Formatting | $(jq -r '.local_health.formatting' "$METRICS_FILE" | sed 's/success/✅ Correct/g; s/failure/❌ Needs Fix/g') |

## 🚀 Recommendations

$(if [ "$health_score" -lt 70 ]; then
    echo "### Immediate Actions Required"
    echo "- Review failing workflows and fix critical issues"
    echo "- Run \`./.github/scripts/auto-fix-pipeline.sh\` to apply automated fixes"
    echo "- Monitor success rate improvements"
elif [ "$health_score" -lt 90 ]; then
    echo "### Suggested Improvements"
    echo "- Continue monitoring pipeline performance"
    echo "- Consider optimizing slower workflows"
    echo "- Review and address any remaining test issues"
else
    echo "### Maintenance"
    echo "- Continue regular monitoring"
    echo "- Pipeline is performing excellently"
    echo "- Consider documenting best practices for other projects"
fi)

## 🛠️ Available Tools

\`\`\`bash
# Run health monitor
./.github/scripts/pipeline-health-monitor.sh

# Collect fresh metrics
./.github/scripts/collect-metrics.sh

# Auto-fix common issues
./.github/scripts/auto-fix-pipeline.sh

# Monitor live workflows
gh run list --limit 10 && gh run watch
\`\`\`

## 📈 Trends

- **Success Rate**: Currently ${success_rate}% (Target: >95%)
- **Performance**: Average ${avg_duration}min (Target: <15min)
- **Optimization**: $(jq -r '.metrics.optimized_workflows' "$METRICS_FILE")/$(jq -r '.metrics.total_workflows' "$METRICS_FILE") workflows optimized

---

*Report generated by automated CI/CD metrics collection*
*Next collection: Run \`./github/scripts/collect-metrics.sh\` for updated metrics*
EOF

    echo "✅ Report generated: $REPORT_FILE"
}

# Main execution
main() {
    # Create output directory
    mkdir -p "$(dirname "$METRICS_FILE")"

    # Run all collection functions
    collect_workflow_stats
    calculate_success_rate
    analyze_performance
    identify_problem_workflows
    analyze_active_workflows
    check_local_health
    calculate_health_score
    generate_report

    echo -e "\n${GREEN}✅ Metrics collection complete!${NC}"
    echo -e "📊 Metrics file: ${CYAN}$METRICS_FILE${NC}"
    echo -e "📝 Report file: ${CYAN}$REPORT_FILE${NC}"

    # Display summary
    echo -e "\n${BLUE}📋 Summary:${NC}"
    echo -e "Health Score: $(jq -r '.health_score.score' "$METRICS_FILE")/100"
    echo -e "Success Rate: $(jq -r '.metrics.success_rate' "$METRICS_FILE")%"
    echo -e "Avg Duration: $(jq -r '.metrics.average_duration_minutes' "$METRICS_FILE") minutes"
}

# Run main function
main "$@"
