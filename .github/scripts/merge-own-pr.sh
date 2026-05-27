#!/usr/bin/env bash
# Usage: ./merge-own-pr.sh <PR_NUMBER>
# Squash-merges a PR authored by the repo owner by temporarily relaxing both
# the legacy branch-protection rules and the "Protect main" ruleset, then
# restoring strict settings (no admin bypass after restore).
set -euo pipefail

PR="${1:?Usage: $0 <PR_NUMBER>}"
REPO="docdyhr/batless"
RULESET_ID="15969824"

# ── Legacy branch-protection (belt-and-suspenders) ──────────────────────────
STRICT_BP='{
  "required_status_checks": {"strict": true, "contexts": ["CI Status"]},
  "enforce_admins": true,
  "required_pull_request_reviews": {
    "required_approving_review_count": 1,
    "dismiss_stale_reviews": true,
    "require_code_owner_reviews": true,
    "require_last_push_approval": true
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
    "require_code_owner_reviews": false,
    "require_last_push_approval": false
  },
  "restrictions": null,
  "required_linear_history": true
}'

# ── Repository ruleset (no bypass_actors in strict — admins are enforced) ────
STRICT_RULESET=$(cat <<'JSON'
{
  "name": "Protect main: require PR and CI",
  "target": "branch",
  "enforcement": "active",
  "conditions": {"ref_name": {"exclude": [], "include": ["~DEFAULT_BRANCH"]}},
  "rules": [
    {"type": "pull_request", "parameters": {
      "required_approving_review_count": 1,
      "dismiss_stale_reviews_on_push": true,
      "required_reviewers": [],
      "require_code_owner_review": true,
      "require_last_push_approval": true,
      "required_review_thread_resolution": false,
      "allowed_merge_methods": ["squash"]
    }},
    {"type": "required_status_checks", "parameters": {
      "strict_required_status_checks_policy": true,
      "do_not_enforce_on_create": false,
      "required_status_checks": [{"context": "CI Status"}]
    }},
    {"type": "deletion"},
    {"type": "non_fast_forward"}
  ],
  "bypass_actors": []
}
JSON
)

RELAXED_RULESET=$(cat <<'JSON'
{
  "name": "Protect main: require PR and CI",
  "target": "branch",
  "enforcement": "active",
  "conditions": {"ref_name": {"exclude": [], "include": ["~DEFAULT_BRANCH"]}},
  "rules": [
    {"type": "pull_request", "parameters": {
      "required_approving_review_count": 0,
      "dismiss_stale_reviews_on_push": true,
      "required_reviewers": [],
      "require_code_owner_review": false,
      "require_last_push_approval": false,
      "required_review_thread_resolution": false,
      "allowed_merge_methods": ["squash"]
    }},
    {"type": "required_status_checks", "parameters": {
      "strict_required_status_checks_policy": true,
      "do_not_enforce_on_create": false,
      "required_status_checks": [{"context": "CI Status"}]
    }},
    {"type": "deletion"},
    {"type": "non_fast_forward"}
  ],
  "bypass_actors": []
}
JSON
)

restore() {
    echo "Restoring branch protection..."
    echo "$STRICT_BP" | gh api --method PUT "repos/$REPO/branches/main/protection" --input - > /dev/null
    echo "$STRICT_RULESET" | gh api --method PUT "repos/$REPO/rulesets/$RULESET_ID" --input - > /dev/null
    echo "Branch protection restored (1 required review, no admin bypass)."
}
trap restore EXIT

echo "Relaxing branch protection for merge..."
echo "$RELAXED_BP" | gh api --method PUT "repos/$REPO/branches/main/protection" --input - > /dev/null
echo "$RELAXED_RULESET" | gh api --method PUT "repos/$REPO/rulesets/$RULESET_ID" --input - > /dev/null

echo "Merging PR #$PR..."
gh api --method PUT "repos/$REPO/pulls/$PR/merge" -f merge_method=squash --jq '.message'
