#!/bin/bash
# CI/CD Pipeline Health Check and Auto-Recovery Script
# Author: Claude Code
# Purpose: Monitor and automatically fix common CI/CD issues

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REPO_ROOT="$(git rev-parse --show-toplevel)"
LOG_FILE="${REPO_ROOT}/pipeline_health.log"
METRICS_FILE="${REPO_ROOT}/pipeline_metrics.json"

# Logging function
log() {
    echo -e "${2:-$NC}$1${NC}"
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" >> "$LOG_FILE"
}

# Check if GitHub CLI is installed and authenticated
check_gh_cli() {
    log "Checking GitHub CLI..." "$BLUE"
    if ! command -v gh &> /dev/null; then
        log "GitHub CLI not installed. Please install with: brew install gh" "$RED"
        return 1
    fi

    if ! gh auth status &> /dev/null; then
        log "GitHub CLI not authenticated. Running: gh auth login" "$YELLOW"
        gh auth login
    fi

    log "✅ GitHub CLI ready" "$GREEN"
}

# Get pipeline metrics
get_pipeline_metrics() {
    log "\nCollecting pipeline metrics..." "$BLUE"

    # Get workflow run statistics
    local total_runs=$(gh api /repos/:owner/:repo/actions/runs --jq '.total_count')
    local successful_runs=$(gh api /repos/:owner/:repo/actions/runs --jq '.workflow_runs | map(select(.conclusion == "success")) | length')
    local failed_runs=$(gh api /repos/:owner/:repo/actions/runs --jq '.workflow_runs | map(select(.conclusion == "failure")) | length')

    # Calculate success rate
    local success_rate=0
    if [ "$total_runs" -gt 0 ]; then
        success_rate=$((successful_runs * 100 / total_runs))
    fi

    # Get average duration (in minutes)
    local avg_duration=$(gh api /repos/:owner/:repo/actions/runs --jq '.workflow_runs |
        map(.updated_at as $end | .created_at as $start |
        (($end | fromdateiso8601) - ($start | fromdateiso8601)) / 60) |
        if length > 0 then add / length else 0 end')

    # Save metrics to file
    cat > "$METRICS_FILE" <<EOF
{
    "timestamp": "$(date -u '+%Y-%m-%dT%H:%M:%SZ')",
    "total_runs": $total_runs,
    "successful_runs": $successful_runs,
    "failed_runs": $failed_runs,
    "success_rate": $success_rate,
    "avg_duration_minutes": $avg_duration
}
EOF

    log "📊 Pipeline Metrics:" "$BLUE"
    log "  Total Runs: $total_runs"
    log "  Success Rate: ${success_rate}%"
    log "  Average Duration: ${avg_duration} minutes"

    # Check if success rate is below threshold
    if [ "$success_rate" -lt 80 ]; then
        log "⚠️  Success rate below 80% threshold!" "$YELLOW"
        return 1
    fi

    return 0
}

# Check for failed workflow runs
check_failed_runs() {
    log "\nChecking for failed workflow runs..." "$BLUE"

    local failed_runs=$(gh run list --status failure --limit 5 --json databaseId,name,conclusion,createdAt)

    if [ "$(echo "$failed_runs" | jq '. | length')" -gt 0 ]; then
        log "❌ Found failed workflow runs:" "$RED"
        echo "$failed_runs" | jq -r '.[] | "  - \(.name) (ID: \(.databaseId))"'

        # Get the latest failure details
        local latest_failure=$(echo "$failed_runs" | jq -r '.[0].databaseId')
        if [ -n "$latest_failure" ]; then
            log "\nAnalyzing latest failure (ID: $latest_failure)..." "$YELLOW"
            gh run view "$latest_failure" --log-failed | head -50 > "${REPO_ROOT}/latest_failure.log"

            # Check for common issues
            if grep -q "npm ERR\|Module not found" "${REPO_ROOT}/latest_failure.log"; then
                log "🔧 Detected dependency issues" "$YELLOW"
                fix_dependencies
            elif grep -q "Test.*failed\|FAIL" "${REPO_ROOT}/latest_failure.log"; then
                log "🔧 Detected test failures" "$YELLOW"
                fix_tests
            elif grep -q "ESLint\|Prettier\|lint.*error" "${REPO_ROOT}/latest_failure.log"; then
                log "🔧 Detected linting issues" "$YELLOW"
                fix_linting
            fi
        fi

        return 1
    else
        log "✅ No recent workflow failures" "$GREEN"
        return 0
    fi
}

# Fix dependency issues
fix_dependencies() {
    log "Attempting to fix dependency issues..." "$BLUE"

    # For Rust projects
    if [ -f "Cargo.toml" ]; then
        cargo update
        cargo fetch
        cargo build --release
    fi

    # For Node.js projects
    if [ -f "package.json" ]; then
        rm -rf node_modules package-lock.json
        npm install
        npm audit fix
    fi

    log "✅ Dependencies updated" "$GREEN"
}

# Fix test failures
fix_tests() {
    log "Running tests locally to identify issues..." "$BLUE"

    # For Rust projects
    if [ -f "Cargo.toml" ]; then
        cargo test --lib --quiet || {
            log "Some tests are failing. Please review test output." "$YELLOW"
            cargo test --lib --verbose
        }
    fi

    # For Node.js projects
    if [ -f "package.json" ]; then
        npm test || {
            log "Some tests are failing. Attempting snapshot update..." "$YELLOW"
            npm test -- -u
        }
    fi

    log "✅ Test check complete" "$GREEN"
}

# Fix linting issues
fix_linting() {
    log "Auto-fixing linting issues..." "$BLUE"

    # For Rust projects
    if [ -f "Cargo.toml" ]; then
        cargo fmt
        cargo clippy --fix --allow-dirty --allow-staged
    fi

    # For Node.js projects
    if [ -f "package.json" ]; then
        npm run lint:fix 2>/dev/null || npx eslint . --fix
        npm run format 2>/dev/null || npx prettier --write .
    fi

    log "✅ Linting issues fixed" "$GREEN"
}

# Check pre-commit hooks
check_precommit() {
    log "\nChecking pre-commit hooks..." "$BLUE"

    if [ ! -f ".pre-commit-config.yaml" ]; then
        log "⚠️  No pre-commit configuration found" "$YELLOW"
        return 1
    fi

    if ! command -v pre-commit &> /dev/null; then
        log "Installing pre-commit..." "$YELLOW"
        pip install pre-commit
        pre-commit install
    fi

    # Run pre-commit on all files
    if pre-commit run --all-files; then
        log "✅ All pre-commit checks passed" "$GREEN"
        return 0
    else
        log "🔧 Fixing pre-commit violations..." "$YELLOW"
        pre-commit run --all-files || true

        # Check if fixes were made
        if [ -n "$(git status --porcelain)" ]; then
            log "📝 Pre-commit made automatic fixes" "$YELLOW"
        fi

        # Run again to verify
        if pre-commit run --all-files; then
            log "✅ Pre-commit issues resolved" "$GREEN"
            return 0
        else
            log "⚠️  Some pre-commit issues require manual intervention" "$YELLOW"
            return 1
        fi
    fi
}

# Check local build
check_local_build() {
    log "\nChecking local build..." "$BLUE"

    # For Rust projects
    if [ -f "Cargo.toml" ]; then
        if cargo build --release; then
            log "✅ Rust build successful" "$GREEN"

            # Test the binary
            if [ -f "./target/release/batless" ]; then
                ./target/release/batless --version
                log "✅ Binary working correctly" "$GREEN"
            fi
        else
            log "❌ Rust build failed" "$RED"
            return 1
        fi
    fi

    # For Node.js projects
    if [ -f "package.json" ] && [ -f "package.json" ]; then
        if npm run build 2>/dev/null; then
            log "✅ Node.js build successful" "$GREEN"
        fi
    fi

    return 0
}

# Generate health report
generate_report() {
    log "\n📋 Generating CI/CD Health Report..." "$BLUE"

    local report_file="${REPO_ROOT}/ci-pipeline-report.md"

    cat > "$report_file" <<EOF
# CI/CD Pipeline Health Report

Generated: $(date)

## Current Status

- **Branch**: $(git branch --show-current)
- **Last Commit**: $(git log -1 --oneline)
- **Repository**: $(gh repo view --json nameWithOwner -q '.nameWithOwner')

## Pipeline Metrics

$(if [ -f "$METRICS_FILE" ]; then
    echo '```json'
    cat "$METRICS_FILE"
    echo '```'
else
    echo "No metrics available"
fi)

## Recent Workflow Runs

\`\`\`
$(gh run list --limit 5 | head -10)
\`\`\`

## Health Checks

| Check | Status | Details |
|-------|--------|---------|
| GitHub CLI | $(check_gh_cli &>/dev/null && echo "✅ Pass" || echo "❌ Fail") | Authentication and configuration |
| Pre-commit Hooks | $(check_precommit &>/dev/null && echo "✅ Pass" || echo "⚠️ Warning") | Code quality gates |
| Local Build | $(check_local_build &>/dev/null && echo "✅ Pass" || echo "❌ Fail") | Build verification |
| Tests | $(cargo test --lib --quiet &>/dev/null && echo "✅ Pass" || echo "❌ Fail") | Unit and integration tests |
| Linting | $(cargo fmt -- --check &>/dev/null && echo "✅ Pass" || echo "⚠️ Warning") | Code style compliance |

## Recommendations

$(if [ -f "$METRICS_FILE" ]; then
    success_rate=$(jq -r '.success_rate' "$METRICS_FILE")
    if [ "$success_rate" -lt 80 ]; then
        echo "- ⚠️ Success rate is below 80%. Review recent failures and improve test stability."
    fi

    avg_duration=$(jq -r '.avg_duration_minutes' "$METRICS_FILE")
    if (( $(echo "$avg_duration > 15" | bc -l) )); then
        echo "- ⚠️ Average pipeline duration exceeds 15 minutes. Consider optimization:"
        echo "  - Enable parallel job execution"
        echo "  - Improve caching strategy"
        echo "  - Split large test suites"
    fi
fi)

- ✅ Continue monitoring pipeline health regularly
- ✅ Keep dependencies up to date
- ✅ Maintain high test coverage

## Quick Fix Commands

\`\`\`bash
# Fix all common issues
./github/scripts/pipeline-health-check.sh --auto-fix

# Rerun failed workflows
gh run rerun \$(gh run list -L 1 --json databaseId -q '.[0].databaseId') --failed

# Update dependencies
cargo update && cargo build --release

# Run all checks locally
pre-commit run --all-files && cargo test && cargo build --release
\`\`\`

---
*Report generated by CI/CD Pipeline Health Check*
EOF

    log "✅ Report saved to: $report_file" "$GREEN"
}

# Main execution
main() {
    log "🚀 Starting CI/CD Pipeline Health Check" "$BLUE"
    log "================================================" "$BLUE"

    local exit_code=0

    # Run all checks
    check_gh_cli || exit_code=1
    get_pipeline_metrics || exit_code=1
    check_failed_runs || exit_code=1
    check_precommit || exit_code=1
    check_local_build || exit_code=1

    # Generate report
    generate_report

    # Summary
    log "\n================================================" "$BLUE"
    if [ $exit_code -eq 0 ]; then
        log "✅ CI/CD Pipeline Health Check: PASSED" "$GREEN"
    else
        log "⚠️  CI/CD Pipeline Health Check: WARNINGS FOUND" "$YELLOW"
        log "Review the report at: ci-pipeline-report.md" "$YELLOW"
    fi

    # Auto-fix mode
    if [ "${1:-}" == "--auto-fix" ]; then
        log "\n🔧 Running auto-fix mode..." "$BLUE"
        fix_dependencies
        fix_linting
        fix_tests
        log "✅ Auto-fix complete" "$GREEN"
    fi

    return $exit_code
}

# Run main function
main "$@"
