#!/bin/bash
# Release Workflow Monitor
# Usage: ./scripts/release-monitor.sh [workflow_run_id]

set -e

WORKFLOW_ID=$1
if [ -z "$WORKFLOW_ID" ]; then
    echo "Usage: $0 <workflow_run_id>"
    echo "Find workflow IDs at: https://github.com/docdyhr/batless/actions"
    exit 1
fi

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

echo "🔍 Release Workflow Monitor"
echo "=========================="
echo "Monitoring workflow: $WORKFLOW_ID"
echo "Started: $(date)"
echo ""

# Function to check workflow status
check_workflow() {
    local response=$(curl -s -H "Authorization: token $(gh auth token)" \
        "https://api.github.com/repos/docdyhr/batless/actions/runs/$WORKFLOW_ID")
    
    local status=$(echo "$response" | jq -r '.status')
    local conclusion=$(echo "$response" | jq -r '.conclusion')
    local created_at=$(echo "$response" | jq -r '.created_at')
    local updated_at=$(echo "$response" | jq -r '.updated_at')
    local name=$(echo "$response" | jq -r '.name')
    
    echo "Workflow: $name"
    echo "Status: $status"
    echo "Conclusion: $conclusion"
    echo "Created: $created_at"
    echo "Updated: $updated_at"
    echo ""
    
    # Calculate runtime
    if [ "$status" = "completed" ]; then
        local start_time=$(date -d "$created_at" +%s 2>/dev/null || date -j -f "%Y-%m-%dT%H:%M:%SZ" "$created_at" "+%s")
        local end_time=$(date -d "$updated_at" +%s 2>/dev/null || date -j -f "%Y-%m-%dT%H:%M:%SZ" "$updated_at" "+%s")
        local duration=$((end_time - start_time))
        echo "Runtime: ${duration} seconds"
    fi
    
    echo "$status:$conclusion"
}

# Monitor with timeout
TIMEOUT=3600  # 1 hour timeout
INTERVAL=30   # Check every 30 seconds
ELAPSED=0

while [ $ELAPSED -lt $TIMEOUT ]; do
    result=$(check_workflow)
    status=$(echo "$result" | tail -1 | cut -d: -f1)
    conclusion=$(echo "$result" | tail -1 | cut -d: -f2)
    
    echo "$(date): $status"
    
    if [ "$status" = "completed" ]; then
        if [ "$conclusion" = "success" ]; then
            log_success "Workflow completed successfully!"
            echo ""
            echo "🎉 Release workflow finished!"
            echo "   Check: https://github.com/docdyhr/batless/releases"
            echo "   Verify: cargo search batless"
            exit 0
        else
            log_error "Workflow failed with conclusion: $conclusion"
            echo ""
            echo "🔍 Check workflow logs:"
            echo "   https://github.com/docdyhr/batless/actions/runs/$WORKFLOW_ID"
            exit 1
        fi
    elif [ "$status" = "queued" ] && [ $ELAPSED -gt 1800 ]; then
        log_warning "Workflow stuck in queue for ${ELAPSED}s (30+ minutes)"
        echo "Consider manual intervention or retry"
    fi
    
    sleep $INTERVAL
    ELAPSED=$((ELAPSED + INTERVAL))
done

log_error "Workflow monitoring timeout after ${TIMEOUT}s"
echo "Workflow may be stuck - manual intervention required"
exit 1
