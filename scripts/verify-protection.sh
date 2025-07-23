#!/bin/bash

# Branch Protection Verification Script
# Verifies that branch protection rules are properly configured
# and provides recommendations for improvements

set -euo pipefail

# Configuration
REPO_OWNER="docdyhr"
REPO_NAME="batless"
BRANCH="main"
GITHUB_API_URL="https://api.github.com/repos/$REPO_OWNER/$REPO_NAME/branches/$BRANCH/protection"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# Helper functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[✓]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[⚠]${NC} $1"
}

log_error() {
    echo -e "${RED}[✗]${NC} $1"
}

log_header() {
    echo -e "\n${BOLD}${CYAN}$1${NC}"
    echo -e "${CYAN}$(printf '=%.0s' $(seq 1 ${#1}))${NC}"
}

# Check if branch protection is enabled
check_protection_exists() {
    log_header "Checking Branch Protection Status"

    local response
    response=$(curl -s -w "%{http_code}" \
        -H "Authorization: token ${GITHUB_TOKEN:-}" \
        -H "Accept: application/vnd.github.v3+json" \
        "$GITHUB_API_URL" 2>/dev/null || echo "000")

    local http_code="${response: -3}"

    if [ "$http_code" -eq 200 ]; then
        log_success "Branch protection is enabled for '$BRANCH' branch"
        return 0
    elif [ "$http_code" -eq 404 ]; then
        log_error "Branch protection is NOT enabled for '$BRANCH' branch"
        echo "  Run: ./scripts/setup-branch-protection.sh to enable protection"
        return 1
    else
        log_warning "Could not verify protection status (HTTP $http_code)"
        if [ -z "${GITHUB_TOKEN:-}" ]; then
            echo "  Set GITHUB_TOKEN environment variable for full verification"
        fi
        return 2
    fi
}

# Verify specific protection rules
verify_protection_rules() {
    log_header "Verifying Protection Rules"

    if [ -z "${GITHUB_TOKEN:-}" ]; then
        log_warning "GITHUB_TOKEN not set - skipping detailed rule verification"
        return
    fi

    local response
    response=$(curl -s \
        -H "Authorization: token $GITHUB_TOKEN" \
        -H "Accept: application/vnd.github.v3+json" \
        "$GITHUB_API_URL" 2>/dev/null)

    if [ $? -ne 0 ]; then
        log_error "Failed to fetch protection details"
        return
    fi

    # Check required status checks
    local status_checks_enabled
    status_checks_enabled=$(echo "$response" | jq -r '.required_status_checks != null' 2>/dev/null || echo "false")

    if [ "$status_checks_enabled" = "true" ]; then
        log_success "Required status checks: ENABLED"

        local strict_mode
        strict_mode=$(echo "$response" | jq -r '.required_status_checks.strict' 2>/dev/null || echo "false")

        if [ "$strict_mode" = "true" ]; then
            log_success "  Strict mode (up-to-date branches): ENABLED"
        else
            log_warning "  Strict mode (up-to-date branches): DISABLED"
        fi

        local contexts
        contexts=$(echo "$response" | jq -r '.required_status_checks.contexts[]?' 2>/dev/null || echo "")

        if [ -n "$contexts" ]; then
            log_success "  Required contexts configured:"
            echo "$contexts" | while read -r context; do
                [ -n "$context" ] && echo "    - $context"
            done
        else
            log_warning "  No required status check contexts configured"
        fi
    else
        log_error "Required status checks: DISABLED"
    fi

    # Check pull request reviews
    local pr_reviews_enabled
    pr_reviews_enabled=$(echo "$response" | jq -r '.required_pull_request_reviews != null' 2>/dev/null || echo "false")

    if [ "$pr_reviews_enabled" = "true" ]; then
        log_success "Required pull request reviews: ENABLED"

        local required_reviewers
        required_reviewers=$(echo "$response" | jq -r '.required_pull_request_reviews.required_approving_review_count' 2>/dev/null || echo "0")
        log_success "  Required approving reviews: $required_reviewers"

        local dismiss_stale
        dismiss_stale=$(echo "$response" | jq -r '.required_pull_request_reviews.dismiss_stale_reviews' 2>/dev/null || echo "false")

        if [ "$dismiss_stale" = "true" ]; then
            log_success "  Dismiss stale reviews: ENABLED"
        else
            log_warning "  Dismiss stale reviews: DISABLED"
        fi
    else
        log_error "Required pull request reviews: DISABLED"
    fi

    # Check admin enforcement
    local enforce_admins
    enforce_admins=$(echo "$response" | jq -r '.enforce_admins.enabled' 2>/dev/null || echo "false")

    if [ "$enforce_admins" = "true" ]; then
        log_success "Enforce rules for administrators: ENABLED"
    else
        log_warning "Enforce rules for administrators: DISABLED"
    fi

    # Check force pushes
    local allow_force_pushes
    allow_force_pushes=$(echo "$response" | jq -r '.allow_force_pushes.enabled' 2>/dev/null || echo "true")

    if [ "$allow_force_pushes" = "false" ]; then
        log_success "Allow force pushes: DISABLED (secure)"
    else
        log_warning "Allow force pushes: ENABLED (less secure)"
    fi

    # Check deletions
    local allow_deletions
    allow_deletions=$(echo "$response" | jq -r '.allow_deletions.enabled' 2>/dev/null || echo "true")

    if [ "$allow_deletions" = "false" ]; then
        log_success "Allow deletions: DISABLED (secure)"
    else
        log_warning "Allow deletions: ENABLED (less secure)"
    fi

    # Check conversation resolution
    local require_conversation_resolution
    require_conversation_resolution=$(echo "$response" | jq -r '.required_conversation_resolution.enabled' 2>/dev/null || echo "false")

    if [ "$require_conversation_resolution" = "true" ]; then
        log_success "Require conversation resolution: ENABLED"
    else
        log_warning "Require conversation resolution: DISABLED"
    fi
}

# Check commit signing requirement
check_commit_signing() {
    log_header "Checking Commit Signing Requirements"

    if [ -z "${GITHUB_TOKEN:-}" ]; then
        log_warning "GITHUB_TOKEN not set - skipping commit signing verification"
        return
    fi

    local signing_url="https://api.github.com/repos/$REPO_OWNER/$REPO_NAME/branches/$BRANCH/protection/required_signatures"

    local response
    response=$(curl -s -w "%{http_code}" \
        -H "Authorization: token $GITHUB_TOKEN" \
        -H "Accept: application/vnd.github.sigstore-protected-signature+json" \
        "$signing_url" 2>/dev/null || echo "000")

    local http_code="${response: -3}"

    if [ "$http_code" -eq 200 ]; then
        log_success "Required commit signing: ENABLED"
    elif [ "$http_code" -eq 404 ]; then
        log_warning "Required commit signing: NOT CONFIGURED"
        echo "  Run: ./scripts/setup-branch-protection.sh to enable commit signing"
    else
        log_warning "Could not verify commit signing status (HTTP $http_code)"
    fi
}

# Check local git configuration
check_local_git_config() {
    log_header "Checking Local Git Configuration"

    # Check GPG signing configuration
    local gpg_signing
    gpg_signing=$(git config --get commit.gpgsign 2>/dev/null || echo "false")

    if [ "$gpg_signing" = "true" ]; then
        log_success "Local GPG commit signing: ENABLED"

        local signing_key
        signing_key=$(git config --get user.signingkey 2>/dev/null || echo "")

        if [ -n "$signing_key" ]; then
            log_success "  Signing key configured: $signing_key"

            # Test if GPG key is available
            if gpg --list-secret-keys "$signing_key" >/dev/null 2>&1; then
                log_success "  GPG key available in keyring"
            else
                log_error "  GPG key NOT found in keyring"
            fi
        else
            log_warning "  No signing key configured"
        fi
    else
        log_warning "Local GPG commit signing: DISABLED"
        echo "  Enable with: git config --global commit.gpgsign true"
    fi

    # Check user configuration
    local user_name
    user_name=$(git config --get user.name 2>/dev/null || echo "")
    local user_email
    user_email=$(git config --get user.email 2>/dev/null || echo "")

    if [ -n "$user_name" ] && [ -n "$user_email" ]; then
        log_success "Git user configuration: $user_name <$user_email>"
    else
        log_warning "Git user configuration incomplete"
    fi
}

# Check workflow status
check_workflow_status() {
    log_header "Checking Required Workflows"

    local workflows_dir=".github/workflows"

    if [ ! -d "$workflows_dir" ]; then
        log_error "No .github/workflows directory found"
        return
    fi

    # Check for required workflow files
    local required_workflows=("ci.yml" "security.yml" "testing.yml")

    for workflow in "${required_workflows[@]}"; do
        if [ -f "$workflows_dir/$workflow" ]; then
            log_success "Workflow found: $workflow"
        else
            log_warning "Missing workflow: $workflow"
        fi
    done

    # Check if workflows have the right triggers
    if [ -f "$workflows_dir/ci.yml" ]; then
        if grep -q "pull_request:" "$workflows_dir/ci.yml"; then
            log_success "CI workflow triggers on pull requests"
        else
            log_warning "CI workflow may not trigger on pull requests"
        fi
    fi
}

# Generate security score
calculate_security_score() {
    log_header "Security Score Assessment"

    local score=0
    local max_score=10

    # Branch protection exists (+2)
    if check_protection_exists >/dev/null 2>&1; then
        score=$((score + 2))
    fi

    # Required status checks (+2)
    if [ -n "${GITHUB_TOKEN:-}" ]; then
        local response
        response=$(curl -s -H "Authorization: token $GITHUB_TOKEN" -H "Accept: application/vnd.github.v3+json" "$GITHUB_API_URL" 2>/dev/null || echo "{}")

        if echo "$response" | jq -e '.required_status_checks != null' >/dev/null 2>&1; then
            score=$((score + 2))
        fi

        # PR reviews required (+2)
        if echo "$response" | jq -e '.required_pull_request_reviews != null' >/dev/null 2>&1; then
            score=$((score + 2))
        fi

        # Admin enforcement (+1)
        if echo "$response" | jq -e '.enforce_admins.enabled == true' >/dev/null 2>&1; then
            score=$((score + 1))
        fi

        # Force pushes disabled (+1)
        if echo "$response" | jq -e '.allow_force_pushes.enabled == false' >/dev/null 2>&1; then
            score=$((score + 1))
        fi
    fi

    # GPG signing configured locally (+1)
    if [ "$(git config --get commit.gpgsign 2>/dev/null)" = "true" ]; then
        score=$((score + 1))
    fi

    # Workflows present (+1)
    if [ -f ".github/workflows/ci.yml" ]; then
        score=$((score + 1))
    fi

    # Display score with color coding
    local percentage=$((score * 100 / max_score))

    if [ $percentage -ge 80 ]; then
        log_success "Security Score: $score/$max_score ($percentage%) - EXCELLENT"
    elif [ $percentage -ge 60 ]; then
        log_warning "Security Score: $score/$max_score ($percentage%) - GOOD"
    else
        log_error "Security Score: $score/$max_score ($percentage%) - NEEDS IMPROVEMENT"
    fi
}

# Provide recommendations
show_recommendations() {
    log_header "Recommendations"

    echo "🔒 Security Best Practices:"
    echo "  • Enable all branch protection rules"
    echo "  • Require GPG signed commits"
    echo "  • Set up required status checks"
    echo "  • Enable administrator rule enforcement"
    echo
    echo "⚡ Performance Tips:"
    echo "  • Use GitHub CLI for faster workflow"
    echo "  • Set up shell aliases for common commands"
    echo "  • Configure local GPG agent for seamless signing"
    echo
    echo "📚 Next Steps:"
    echo "  • Run ./scripts/setup-branch-protection.sh if not configured"
    echo "  • Review docs/BRANCH_PROTECTION.md for detailed guidance"
    echo "  • Test the workflow with a sample pull request"
}

# Main execution
main() {
    echo -e "${BOLD}${CYAN}🛡️  Branch Protection Verification${NC}"
    echo -e "${CYAN}====================================${NC}\n"

    echo -e "${BLUE}Repository:${NC} $REPO_OWNER/$REPO_NAME"
    echo -e "${BLUE}Branch:${NC} $BRANCH"

    check_protection_exists
    verify_protection_rules
    check_commit_signing
    check_local_git_config
    check_workflow_status
    calculate_security_score
    show_recommendations

    echo -e "\n${BOLD}${GREEN}Verification completed!${NC}"

    if [ -z "${GITHUB_TOKEN:-}" ]; then
        echo -e "\n${YELLOW}💡 Tip:${NC} Set GITHUB_TOKEN environment variable for complete verification"
    fi
}

# Check requirements
if ! command -v jq >/dev/null 2>&1; then
    log_error "jq is required but not installed"
    echo "Install with: brew install jq (macOS) or sudo apt install jq (Ubuntu)"
    exit 1
fi

if ! command -v curl >/dev/null 2>&1; then
    log_error "curl is required but not installed"
    exit 1
fi

# Run main function
main "$@"
