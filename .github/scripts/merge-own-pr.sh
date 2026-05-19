#!/usr/bin/env bash
# Usage: ./merge-own-pr.sh <PR_NUMBER>
# Merges a PR authored by the repo owner by temporarily relaxing branch
# protection (review count 0), squash-merging, then restoring (review count 1).
set -euo pipefail

PR="${1:?Usage: $0 <PR_NUMBER>}"
REPO="docdyhr/batless"

STRICT_BP='{
  "required_status_checks": {"strict": true, "contexts": ["CI Status"]},
  "enforce_admins": true,
  "required_pull_request_reviews": {
    "required_approving_review_count": 1,
    "dismiss_stale_reviews": true,
    "require_code_owner_reviews": true
  },
  "restrictions": null,
  "required_linear_history": true
}'

RELAXED_BP='{
  "required_status_checks": {"strict": true, "contexts": ["CI Status"]},
  "enforce_admins": true,
  "required_pull_request_reviews": {
    "required_approving_review_count": 0,
    "dismiss_stale_reviews": true,
    "require_code_owner_reviews": false
  },
  "restrictions": null,
  "required_linear_history": true
}'

restore() {
    echo "Restoring branch protection..."
    echo "$STRICT_BP" | gh api --method PUT "repos/$REPO/branches/main/protection" --input - > /dev/null
    echo "Branch protection restored (1 required review)."
}
trap restore EXIT

echo "Relaxing branch protection for merge..."
echo "$RELAXED_BP" | gh api --method PUT "repos/$REPO/branches/main/protection" --input - > /dev/null

echo "Merging PR #$PR..."
gh api --method PUT "repos/$REPO/pulls/$PR/merge" -f merge_method=squash --jq '.message'
