#!/usr/bin/env bash
# Pipeline health check - monitors CI/CD status and suggests fixes

set -euo pipefail

echo "🔍 Batless Pipeline Health Check"
echo "================================="

# Check recent workflow runs
echo "📊 Recent Workflow Status:"
gh run list --limit 10 --json name,status,conclusion,createdAt | \
  jq -r '.[] | "\(.name): \(.status) - \(.conclusion // "in_progress")"'

echo ""

# Count failures in last 24 hours
FAILURES=$(gh run list --limit 20 --json conclusion | \
  jq '[.[] | select(.conclusion == "failure")] | length')

if [ "$FAILURES" -gt 5 ]; then
  echo "⚠️  WARNING: $FAILURES failures in recent runs"
  echo "💡 Consider investigating common failure patterns"
else
  echo "✅ Pipeline health looks good ($FAILURES failures in recent runs)"
fi

echo ""

# Check for currently failing workflows
echo "🚨 Recently Failed Workflows:"
gh run list --status failure --limit 5 --json name,url,createdAt | \
  jq -r '.[] | "- \(.name) (\(.url))"' || echo "None"

echo ""

# Suggest maintenance actions
echo "🛠️  Maintenance Suggestions:"

# Check if any workflows haven't run recently
LAST_SUCCESS=$(gh run list --status success --limit 1 --json createdAt | jq -r '.[0].createdAt')
if [ -n "$LAST_SUCCESS" ] && [ "$LAST_SUCCESS" != "null" ]; then
  echo "✅ Recent successful run: $(echo "$LAST_SUCCESS" | cut -c1-19)"
else
  echo "⚠️  No recent successful runs found"
  echo "💡 Consider running manual workflow dispatch to verify pipeline health"
fi

# Check cache health
echo "📦 Cache Status:"
echo "💡 Clear caches if builds are consistently slow:"
echo "   - Go to Actions → Caches and delete old entries"

echo ""
echo "📋 Quick Fixes:"
echo "• For Docker failures: Check disk space and build timeouts"
echo "• For coverage failures: Restart the job (usually transient)"
echo "• For fuzz failures: Ensure fuzz targets match current API"
echo "• For cross-platform failures: Check Alpine dependencies"

echo ""
echo "🔄 To trigger manual pipeline validation:"
echo "gh workflow run workflow-dispatch.yml -f workflow_type=full-test-suite"
