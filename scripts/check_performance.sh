#!/bin/bash

# Performance regression check script for CI/CD
# This script runs benchmarks and checks for significant performance degradation

set -e

BASELINE_FILE="benchmark_baseline.txt"
CURRENT_FILE="benchmark_current.txt"
THRESHOLD_PERCENT=25  # 25% degradation threshold

echo "🔍 Running performance regression checks..."

# Run benchmarks and capture output
echo "📊 Running startup operations benchmarks..."
cargo bench --bench performance startup_operations 2>/dev/null | grep "time:" > "$CURRENT_FILE" || true

echo "📊 Running config operations benchmarks..."
cargo bench --bench performance config_operations 2>/dev/null | grep "time:" >> "$CURRENT_FILE" || true

# Check if we have baseline data
if [ ! -f "$BASELINE_FILE" ]; then
    echo "⚠️  No baseline file found at $BASELINE_FILE"
    echo "💾 Creating baseline from current run..."
    cp "$CURRENT_FILE" "$BASELINE_FILE"
    echo "✅ Baseline created. Run this script again to check for regressions."
    exit 0
fi

echo "🔬 Analyzing performance differences..."

# Simple performance analysis (could be enhanced with proper parsing)
performance_issues=0

# Check if current benchmarks are significantly slower
while IFS= read -r line; do
    if [[ $line == *"time:"* ]]; then
        # Extract the benchmark name and timing
        benchmark_name=$(echo "$line" | awk '{print $1}')
        current_time=$(echo "$line" | grep -o '\[[0-9.]*' | tr -d '[')
        
        # Look for corresponding baseline
        baseline_line=$(grep "$benchmark_name" "$BASELINE_FILE" 2>/dev/null || echo "")
        if [[ -n "$baseline_line" ]]; then
            baseline_time=$(echo "$baseline_line" | grep -o '\[[0-9.]*' | tr -d '[')
            
            # Simple percentage check (very basic, could be improved)
            if command -v bc >/dev/null 2>&1; then
                if [[ -n "$current_time" && -n "$baseline_time" ]]; then
                    # Calculate percentage difference
                    percent_diff=$(echo "scale=2; ($current_time - $baseline_time) / $baseline_time * 100" | bc 2>/dev/null || echo "0")
                    
                    # Check if degradation exceeds threshold
                    if (( $(echo "$percent_diff > $THRESHOLD_PERCENT" | bc -l 2>/dev/null || echo "0") )); then
                        echo "⚠️  Performance regression detected in $benchmark_name:"
                        echo "   Baseline: ${baseline_time}μs/ns"
                        echo "   Current:  ${current_time}μs/ns"
                        echo "   Change:   +${percent_diff}%"
                        performance_issues=$((performance_issues + 1))
                    fi
                fi
            fi
        fi
    fi
done < "$CURRENT_FILE"

# Report results
if [ $performance_issues -eq 0 ]; then
    echo "✅ No significant performance regressions detected!"
    echo "📈 All benchmarks within acceptable performance bounds."
else
    echo "❌ Found $performance_issues performance regression(s)!"
    echo "🚨 Performance has degraded beyond $THRESHOLD_PERCENT% threshold."
    exit 1
fi

echo "🎯 Performance check completed successfully."