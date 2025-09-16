#!/bin/bash
set -euo pipefail

echo "📊 Workflow Performance Analysis"
echo "================================="
echo ""

# Function to convert ISO date to seconds
date_to_seconds() {
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        date -j -f "%Y-%m-%dT%H:%M:%SZ" "$1" "+%s" 2>/dev/null || echo "0"
    else
        # Linux
        date -d "$1" "+%s" 2>/dev/null || echo "0"
    fi
}

# Analyze workflow execution times
echo "⏱️ Workflow Execution Times (Last 20 runs)"
echo "-------------------------------------------"

gh run list --limit 20 --json name,createdAt,updatedAt,conclusion | \
    jq -r '.[] | select(.conclusion == "success" or .conclusion == "failure") | "\(.name)|\(.createdAt)|\(.updatedAt)|\(.conclusion)"' | \
    while IFS='|' read -r name created updated conclusion; do
        start_sec=$(date_to_seconds "$created")
        end_sec=$(date_to_seconds "$updated")

        if [ "$start_sec" -ne 0 ] && [ "$end_sec" -ne 0 ]; then
            duration=$(( (end_sec - start_sec) / 60 ))

            # Color code based on duration
            if [ "$duration" -lt 5 ]; then
                status="✅"
            elif [ "$duration" -lt 10 ]; then
                status="🟡"
            else
                status="🔴"
            fi

            printf "  %s %-40s %3d min [%s]\n" "$status" "$name" "$duration" "$conclusion"
        fi
    done | sort -t'|' -k2 | head -20

echo ""
echo "📊 Performance Score"
echo "-------------------"

# Calculate a performance score
score=100

# Check for workflows without caching
no_cache_workflows=$(grep -L "actions/cache" .github/workflows/*.yml 2>/dev/null | wc -l)
if [ "$no_cache_workflows" -gt 3 ]; then
    score=$((score - 20))
fi

# Display score
if [ "$score" -ge 80 ]; then
    echo "  Overall Score: $score/100 ✅"
elif [ "$score" -ge 60 ]; then
    echo "  Overall Score: $score/100 🟡"
else
    echo "  Overall Score: $score/100 🔴"
fi

echo ""
echo "================================="
echo "Performance Analysis Complete"
echo "================================="
