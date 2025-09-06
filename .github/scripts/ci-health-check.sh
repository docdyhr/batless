#!/bin/bash
# Enhanced CI/CD Health Check with Accurate Metrics
# Purpose: Monitor, analyze, and optimize CI/CD pipelines

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}🔍 CI/CD Pipeline Health Analysis${NC}"
echo "=================================="

# Get recent workflow statistics
echo -e "\n${BLUE}📊 Workflow Statistics (Last 30 runs)${NC}"
RECENT_RUNS=$(gh run list --limit 30 --json conclusion,status,name,createdAt,databaseId)

# Calculate metrics from recent runs
TOTAL=$(echo "$RECENT_RUNS" | jq '. | length')
SUCCESS=$(echo "$RECENT_RUNS" | jq '[.[] | select(.conclusion == "success")] | length')
FAILED=$(echo "$RECENT_RUNS" | jq '[.[] | select(.conclusion == "failure")] | length')
IN_PROGRESS=$(echo "$RECENT_RUNS" | jq '[.[] | select(.status == "in_progress")] | length')

if [ "$TOTAL" -gt 0 ]; then
    SUCCESS_RATE=$((SUCCESS * 100 / TOTAL))
else
    SUCCESS_RATE=0
fi

echo "Total Runs: $TOTAL"
echo "Successful: $SUCCESS (${SUCCESS_RATE}%)"
echo "Failed: $FAILED"
echo "In Progress: $IN_PROGRESS"

# Check for patterns in failures
if [ "$FAILED" -gt 0 ]; then
    echo -e "\n${YELLOW}⚠️  Failed Workflows:${NC}"
    echo "$RECENT_RUNS" | jq -r '.[] | select(.conclusion == "failure") | "  - \(.name) (\(.createdAt))"' | head -5
fi

# Performance analysis
echo -e "\n${BLUE}⏱️  Performance Metrics${NC}"
AVG_DURATION=$(gh api /repos/:owner/:repo/actions/runs \
    --paginate false \
    --jq '[.workflow_runs[:30] |
    .[] | select(.conclusion == "success") |
    (((.updated_at | fromdateiso8601) - (.created_at | fromdateiso8601)) / 60)] |
    if length > 0 then add / length else 0 end' 2>/dev/null || echo "0")

echo "Average Duration: $(printf "%.2f" "$AVG_DURATION") minutes"

if (( $(echo "$AVG_DURATION > 15" | bc -l 2>/dev/null || echo "0") )); then
    echo -e "${YELLOW}  ⚠️  Consider optimizing workflows (target: <15 min)${NC}"
fi

# Check workflow configurations
echo -e "\n${BLUE}📝 Workflow Configuration Analysis${NC}"
WORKFLOW_COUNT=$(ls -1 .github/workflows/*.yml 2>/dev/null | wc -l)
echo "Active Workflows: $WORKFLOW_COUNT"

# Check for optimization opportunities
echo -e "\n${BLUE}🚀 Optimization Opportunities${NC}"

# Check for caching
if grep -q "actions/cache" .github/workflows/*.yml 2>/dev/null; then
    echo "✅ Caching enabled"
else
    echo "⚠️  Consider adding caching to speed up builds"
fi

# Check for parallelization
if grep -q "matrix:" .github/workflows/*.yml 2>/dev/null; then
    echo "✅ Matrix builds configured"
else
    echo "⚠️  Consider using matrix builds for parallel testing"
fi

# Check for concurrency controls
if grep -q "concurrency:" .github/workflows/*.yml 2>/dev/null; then
    echo "✅ Concurrency controls configured"
else
    echo "⚠️  Consider adding concurrency controls"
fi

# Local validation
echo -e "\n${BLUE}🏗️  Local Build Validation${NC}"
if cargo build --release --quiet 2>/dev/null; then
    echo "✅ Release build successful"
else
    echo "❌ Release build failed"
fi

if cargo test --lib --quiet 2>/dev/null; then
    echo "✅ Tests passing"
else
    echo "❌ Tests failing"
fi

if cargo clippy --quiet 2>/dev/null; then
    echo "✅ Clippy checks passing"
else
    echo "⚠️  Clippy warnings present"
fi

# Summary
echo -e "\n${BLUE}📋 Summary${NC}"
echo "=================================="
if [ "$SUCCESS_RATE" -ge 95 ]; then
    echo -e "${GREEN}✅ Pipeline Health: EXCELLENT${NC}"
elif [ "$SUCCESS_RATE" -ge 80 ]; then
    echo -e "${GREEN}✅ Pipeline Health: GOOD${NC}"
elif [ "$SUCCESS_RATE" -ge 60 ]; then
    echo -e "${YELLOW}⚠️  Pipeline Health: NEEDS ATTENTION${NC}"
else
    echo -e "${RED}❌ Pipeline Health: CRITICAL${NC}"
fi

echo -e "\nSuccess Rate: ${SUCCESS_RATE}%"
echo "Average Duration: $(printf "%.2f" "$AVG_DURATION") minutes"

# Recommendations
if [ "$SUCCESS_RATE" -lt 95 ] || (( $(echo "$AVG_DURATION > 15" | bc -l 2>/dev/null || echo "0") )); then
    echo -e "\n${BLUE}💡 Recommendations${NC}"

    if [ "$SUCCESS_RATE" -lt 95 ]; then
        echo "- Investigate and fix failing tests"
        echo "- Add retry logic for flaky tests"
        echo "- Improve test stability"
    fi

    if (( $(echo "$AVG_DURATION > 15" | bc -l 2>/dev/null || echo "0") )); then
        echo "- Enable job parallelization"
        echo "- Optimize dependency caching"
        echo "- Consider splitting large test suites"
        echo "- Use faster runners for critical paths"
    fi
fi

echo -e "\n✨ Health check complete!"
