#!/bin/bash
set -euo pipefail

# Pipeline Health Monitor - Comprehensive CI/CD Analysis

echo "================================================"
echo "    CI/CD Pipeline Health Monitor Report"
echo "================================================"
echo "Timestamp: $(date)"
echo "Repository: batless"
echo ""

# Check if GitHub CLI is available
if ! command -v gh &> /dev/null; then
    echo "❌ GitHub CLI not installed. Please install: brew install gh"
    exit 1
fi

# Check authentication
if ! gh auth status &> /dev/null; then
    echo "❌ GitHub CLI not authenticated. Run: gh auth login"
    exit 1
fi

echo "📊 WORKFLOW STATUS OVERVIEW"
echo "----------------------------"

# Get recent workflow runs
echo "Recent Workflow Runs (Last 10):"
gh run list --limit 10 --json status,conclusion,name,createdAt,headBranch | \
    jq -r '.[] | "  - \(.name): \(.conclusion // .status) (\(.headBranch))"' || echo "Failed to fetch runs"

echo ""
echo "📈 FAILURE ANALYSIS"
echo "-------------------"

# Count failures by workflow
echo "Failures by Workflow:"
gh run list --status failure --limit 20 --json name | \
    jq -r '.[] | .name' | sort | uniq -c | sort -rn | \
    while read count name; do
        echo "  $count failures: $name"
    done

echo ""
echo "⏱️ PERFORMANCE METRICS"
echo "----------------------"

# Calculate average duration for successful runs
echo "Average Duration (successful runs):"
gh run list --status success --limit 10 --json name,createdAt,updatedAt | \
    jq -r '.[] | "\(.name)|\(.createdAt)|\(.updatedAt)"' | \
    while IFS='|' read -r name created updated; do
        if [[ -n "$created" && -n "$updated" ]]; then
            duration=$(( ($(date -d "$updated" +%s) - $(date -d "$created" +%s)) / 60 ))
            echo "  - $name: ${duration} minutes"
        fi
    done 2>/dev/null || echo "  Unable to calculate durations"

echo ""
echo "🔍 WORKFLOW FILE ANALYSIS"
echo "-------------------------"

# Count and analyze workflow files
total_workflows=$(find .github/workflows -name "*.yml" -o -name "*.yaml" | wc -l)
echo "Total Workflow Files: $total_workflows"

# Check for duplicate or redundant workflows
echo ""
echo "Checking for potential redundant workflows..."
find .github/workflows -name "*.yml" -o -name "*.yaml" | while read workflow; do
    basename=$(basename "$workflow" .yml)
    basename=$(basename "$basename" .yaml)

    # Check if there are similar named files
    similar_count=$(find .github/workflows -name "*${basename}*" | wc -l)
    if [ "$similar_count" -gt 1 ]; then
        echo "  ⚠️ Potential redundancy: $basename (found $similar_count similar files)"
    fi
done

echo ""
echo "================================================"
echo "          Health Check Complete"
echo "================================================"
