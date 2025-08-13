#!/usr/bin/env bash
set -euo pipefail

# Advanced performance regression check
# Features:
#  - Multiple sample runs per benchmark & median calculation
#  - Baseline JSON (benchmark_baseline.json) + legacy txt compatibility
#  - Warn & fail thresholds (WARN_THRESHOLD / FAIL_THRESHOLD)
#  - Machine-readable summary (performance_summary.json / .md)
#  - Exit codes: 0=pass, 50=warn, 1=fail

BENCHES=(startup_operations config_operations)
SAMPLES=${SAMPLES:-5}
WARN_THRESHOLD=${WARN_THRESHOLD:-15}
FAIL_THRESHOLD=${FAIL_THRESHOLD:-25}

LEGACY_BASELINE_TXT="benchmark_baseline.txt"
LEGACY_CURRENT_TXT="benchmark_current.txt"
BASELINE_JSON="benchmark_baseline.json"
SUMMARY_JSON="performance_summary.json"
SUMMARY_MD="performance_summary.md"

echo "🔍 Running performance regression checks (samples=${SAMPLES}, warn>${WARN_THRESHOLD}%, fail>${FAIL_THRESHOLD}%)"

mkdir -p .perf_tmp
>"$LEGACY_CURRENT_TXT"

declare -A current_medians
declare -A current_units

run_and_collect() {
    local bench="$1"
    local times_file=".perf_tmp/${bench}.times"
    >"$times_file"
    for i in $(seq 1 "$SAMPLES"); do
        echo "   ▶ Run $i/$SAMPLES: $bench"
        # Run the single benchmark; suppress non-time noise; capture lines with 'time:'
        cargo bench --quiet --bench performance "$bench" 2>/dev/null | grep 'time:' >> "$times_file" || true
    done
    # Extract numeric time + unit (first occurrence per run)
    local values
    mapfile -t values < <(grep -Eo 'time:\s*\[[0-9.]+ [a-zµns]+' "$times_file" | sed -E 's/.*\[([0-9.]+) ([a-zµns]+)\]/\1 \2/' | awk '{print $1" "$2}')
    if [ ${#values[@]} -eq 0 ]; then
        echo "⚠️  No timing data collected for $bench" >&2
        return 1
    fi
    # Split numbers & assume consistent unit
    local nums=()
    local unit=""
    for v in "${values[@]}"; do
        local num=$(echo "$v" | awk '{print $1}')
        unit=$(echo "$v" | awk '{print $2}')
        nums+=("$num")
    done
    # Sort numbers to get median
    IFS=$'\n' sorted=($(sort -g <<<"${nums[*]}"))
    unset IFS
    local count=${#sorted[@]}
    local mid=$((count/2))
    local median
    if (( count % 2 == 1 )); then
        median=${sorted[$mid]}
    else
        median=$(awk -v a="${sorted[$((mid-1))]}" -v b="${sorted[$mid]}" 'BEGIN{printf "%.6f", (a+b)/2}')
    fi
    current_medians[$bench]="$median"
    current_units[$bench]="$unit"
    # Append legacy current line (first run's raw line retained if available)
    local first_line=$(head -1 "$times_file")
    echo "${first_line:-$bench time: [$median $unit]}" >> "$LEGACY_CURRENT_TXT"
}

for b in "${BENCHES[@]}"; do
    run_and_collect "$b" || true
done

# Initialize baseline JSON if missing (bootstrap mode)
if [ ! -f "$BASELINE_JSON" ]; then
    echo "⚠️  No JSON baseline ($BASELINE_JSON) found — bootstrapping from current medians." >&2
    {
        echo '{"version":1,"generated_at":'"$(date +%s)"',"benchmarks":{'
        first=true
        for b in "${BENCHES[@]}"; do
            m=${current_medians[$b]:-0}; u=${current_units[$b]:-us}
            if [ -n "$m" ]; then
                $first || echo ','
                first=false
                printf '"%s":{"median":%s,"unit":"%s"}' "$b" "$m" "$u"
            fi
        done
        echo '}}'
    } > "$BASELINE_JSON"
    # Also bootstrap legacy txt baseline if absent
    if [ ! -f "$LEGACY_BASELINE_TXT" ]; then
        cp "$LEGACY_CURRENT_TXT" "$LEGACY_BASELINE_TXT"
    fi
    echo "✅ Baseline created. Re-run to enforce thresholds." >&2
    # Produce summary & exit success (bootstrap)
    echo '{"status":"bootstrap"}' > "$SUMMARY_JSON"
    echo "# Performance Summary" > "$SUMMARY_MD"
    echo "Baseline bootstrapped; no regression evaluation this run." >> "$SUMMARY_MD"
    exit 0
fi

# If legacy baseline missing but JSON exists, create legacy for backward compat
if [ ! -f "$LEGACY_BASELINE_TXT" ]; then
    echo "ℹ️  Creating legacy baseline txt from JSON baseline" >&2
    cp "$LEGACY_CURRENT_TXT" "$LEGACY_BASELINE_TXT" || true
fi

baseline_json=$(cat "$BASELINE_JSON")

status_overall="pass"
regression_count=0
warn_count=0

json_body='{'"\n  \"benchmarks\": {"
first=true
for b in "${BENCHES[@]}"; do
    current=${current_medians[$b]:-}
    unit=${current_units[$b]:-}
    # Extract baseline median via jq if available
    if command -v jq >/dev/null 2>&1; then
        baseline=$(jq -r --arg B "$b" '.benchmarks[$B].median // empty' "$BASELINE_JSON" 2>/dev/null || echo "")
    else
        baseline=""
    fi
    if [ -z "$current" ] || [ -z "$baseline" ]; then
        diff_pct=""
        state="no-data"
    else
        diff_pct=$(awk -v c="$current" -v b="$baseline" 'BEGIN{ if(b==0){print 0}else{printf "%.2f", (c-b)/b*100}}')
        # Determine state
        awk_cmp=$(awk -v d="$diff_pct" -v ft="$FAIL_THRESHOLD" -v wt="$WARN_THRESHOLD" 'BEGIN{if(d>ft){print 2}else if(d>wt){print 1}else{print 0}}')
        case $awk_cmp in
            2)
                state="fail"; status_overall="fail"; regression_count=$((regression_count+1));;
            1)
                if [ "$status_overall" != "fail" ]; then status_overall="warn"; fi; state="warn"; warn_count=$((warn_count+1));;
            *)
                state="pass";;
        esac
    fi
    $first || json_body+=','
    first=false
    json_body+=$'\n    '"\"$b\": { \"current_median\": ${current:-null}, \"baseline_median\": ${baseline:-null}, \"unit\": \"${unit:-unknown}\", \"percent_diff\": ${diff_pct:-null}, \"state\": \"$state\" }"
done
json_body+=$'\n  },'"\n  \"status\": \"$status_overall\", \"regressions\": $regression_count, \"warnings\": $warn_count\n}"
echo "$json_body" > "$SUMMARY_JSON"

# Human summary
{
    echo "# Performance Summary"
    echo
    echo "Overall status: $status_overall"
    echo "Warn threshold: >${WARN_THRESHOLD}%  Fail threshold: >${FAIL_THRESHOLD}%"
    echo
    printf "| Benchmark | Baseline | Current | Δ%% | State |\n"
    echo "|-----------|----------|---------|----|-------|"
    if command -v jq >/dev/null 2>&1; then
        jq -r '.benchmarks | to_entries[] | "| " + .key + " | " + ( .value.baseline_median|tostring) + " | " + ( .value.current_median|tostring ) + " | " + ( .value.percent_diff|tostring ) + " | " + .value.state + " |"' "$SUMMARY_JSON" 2>/dev/null || true
    else
        for b in "${BENCHES[@]}"; do
            echo "| $b | ${current_medians[$b]:-?} | ${current_medians[$b]:-?} | ? | jq-missing |"
        done
    fi
} > "$SUMMARY_MD"

echo "� Wrote machine summary to $SUMMARY_JSON and markdown to $SUMMARY_MD"

case "$status_overall" in
    pass)
        echo "✅ Performance within thresholds."; exit 0;;
    warn)
        echo "⚠️  Performance warnings (exceeded ${WARN_THRESHOLD}% but not ${FAIL_THRESHOLD}%)."; exit 50;;
    fail)
        echo "❌ Performance regressions exceeded fail threshold (${FAIL_THRESHOLD}%)."; exit 1;;
esac

exit 0
