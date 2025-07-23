#!/bin/bash

# Branch Protection Verification Script (GitHub CLI Version)
# Verifies that branch protection rules are properly configured
# Uses GitHub CLI authentication instead of environment tokens

set -euo pipefail

# Configuration
REPO_OWNER="docdyhr"
REPO_NAME="batless"
BRANCH="main"

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

# Check requirements
check_requirements() {
    if ! command -v gh >/dev/null 2>&1; then
        log_error "GitHub CLI (gh) is required but not installed"
        echo "Install with: brew install gh (macOS) or sudo apt install gh (Ubuntu)"
        exit 1
    fi

    if ! command -v jq >/dev/null 2>&1; then
        log_error "jq is required but not installed"
        echo "Install with: brew install jq (macOS) or sudo apt install jq (Ubuntu)"
        exit 1
    fi

    if ! gh auth status >/dev/null 2>&1; then
        log_error "GitHub CLI is not authenticated"
        echo "Please run: gh auth login"
        exit 1
    fi
}

# Check if branch protection is enabled
check_protection_exists() {
    log_header "Checking Branch Protection Status"

    local protection_response
    protection_response=$(gh api "repos/$REPO_OWNER/$REPO_NAME/branches/$BRANCH/protection" 2>&1 || true)

    if echo "$protection_response" | grep -q "Branch not protected"; then
        log_error "Branch protection is NOT ENABLED for '$BRANCH' branch"
        echo "  Run: ./scripts/setup-branch-protection-gh.sh to enable protection"
        return 1
    elif echo "$protection_response" | grep -q '"required_status_checks"'; then
        log_success "Branch protection is ENABLED for '$BRANCH' branch"
        return 0
    else
        log_warning "Could not determine protection status"
        echo "  Response: $protection_response"
        return 2
    fi
}

# Verify specific protection rules
verify_protection_rules() {
    log_header "Verifying Protection Rules"

    local protection_data
    protection_data=$(gh api "repos/$REPO_OWNER/$REPO_NAME/branches/$BRANCH/protection" 2>&1)

    if echo "$protection_data" | grep -q "Branch not protected"; then
        log_error "Branch protection is not enabled - cannot verify rules"
        echo "  Run: ./scripts/setup-branch-protection-gh.sh to enable protection first"
        return 1
    fi

    # Check required status checks
    local status_checks_enabled
    status_checks_enabled=$(echo "$protection_data" | jq -r '.required_status_checks != null' 2>/dev/null || echo "false")

    if [ "$status_checks_enabled" = "true" ]; then
        log_success "Required status checks: ENABLED"

        local strict_mode
        strict_mode=$(echo "$protection_data" | jq -r '.required_status_checks.strict' 2>/dev/null || echo "false")

        if [ "$strict_mode" = "true" ]; then
            log_success "  Strict mode (up-to-date branches): ENABLED"
        else
            log_warning "  Strict mode (up-to-date branches): DISABLED"
        fi

        log_info "  Required status check contexts:"
        echo "$protection_data" | jq -r '.required_status_checks.contexts[]?' 2>/dev/null | while read -r context; do
            if [ -n "$context" ]; then
                echo "    ✓ $context"
            fi
        done

        # Count contexts
        local context_count
        context_count=$(echo "$protection_data" | jq -r '.required_status_checks.contexts | length' 2>/dev/null || echo "0")
        if [ "$context_count" -eq 0 ]; then
            log_warning "  No required status check contexts configured"
        else
            log_success "  $context_count status check contexts configured"
        fi
    else
        log_error "Required status checks: DISABLED"
    fi

    # Check pull request reviews
    local pr_reviews_enabled
    pr_reviews_enabled=$(echo "$protection_data" | jq -r '.required_pull_request_reviews != null' 2>/dev/null || echo "false")

    if [ "$pr_reviews_enabled" = "true" ]; then
        log_success "Required pull request reviews: ENABLED"

        local required_reviewers
        required_reviewers=$(echo "$protection_data" | jq -r '.required_pull_request_reviews.required_approving_review_count' 2>/dev/null || echo "0")
        log_success "  Required approving reviews: $required_reviewers"

        local dismiss_stale
        dismiss_stale=$(echo "$protection_data" | jq -r '.required_pull_request_reviews.dismiss_stale_reviews' 2>/dev/null || echo "false")

        if [ "$dismiss_stale" = "true" ]; then
            log_success "  Dismiss stale reviews: ENABLED"
        else
            log_warning "  Dismiss stale reviews: DISABLED"
        fi

        local require_code_owners
        require_code_owners=$(echo "$protection_data" | jq -r '.required_pull_request_reviews.require_code_owner_reviews' 2>/dev/null || echo "false")

        if [ "$require_code_owners" = "false" ]; then
            log_success "  Code owner reviews: DISABLED (appropriate for solo dev)"
        else
            log_info "  Code owner reviews: ENABLED"
        fi
    else
        log_error "Required pull request reviews: DISABLED"
    fi

    # Check admin enforcement
    local enforce_admins
    enforce_admins=$(echo "$protection_data" | jq -r '.enforce_admins.enabled' 2>/dev/null || echo "false")

    if [ "$enforce_admins" = "true" ]; then
        log_success "Enforce rules for administrators: ENABLED"
    else
        log_warning "Enforce rules for administrators: DISABLED"
    fi

    # Check force pushes
    local allow_force_pushes
    allow_force_pushes=$(echo "$protection_data" | jq -r '.allow_force_pushes.enabled' 2>/dev/null || echo "true")

    if [ "$allow_force_pushes" = "false" ]; then
        log_success "Allow force pushes: DISABLED (secure)"
    else
        log_warning "Allow force pushes: ENABLED (less secure)"
    fi

    # Check deletions
    local allow_deletions
    allow_deletions=$(echo "$protection_data" | jq -r '.allow_deletions.enabled' 2>/dev/null || echo "true")

    if [ "$allow_deletions" = "false" ]; then
        log_success "Allow deletions: DISABLED (secure)"
    else
        log_warning "Allow deletions: ENABLED (less secure)"
    fi

    # Check conversation resolution
    local require_conversation_resolution
    require_conversation_resolution=$(echo "$protection_data" | jq -r '.required_conversation_resolution.enabled' 2>/dev/null || echo "false")

    if [ "$require_conversation_resolution" = "true" ]; then
        log_success "Require conversation resolution: ENABLED"
    else
        log_warning "Require conversation resolution: DISABLED"
    fi

    # Check linear history
    local require_linear_history
    require_linear_history=$(echo "$protection_data" | jq -r '.required_linear_history.enabled' 2>/dev/null || echo "false")

    if [ "$require_linear_history" = "false" ]; then
        log_success "Require linear history: DISABLED (allows merge commits)"
    else
        log_info "Require linear history: ENABLED (forces rebasing)"
    fi
}

# Check commit signing requirement
check_commit_signing() {
    log_header "Checking Commit Signing Requirements"

    local signing_response
    signing_response=$(gh api "repos/$REPO_OWNER/$REPO_NAME/branches/$BRANCH/protection/required_signatures" 2>&1 || true)

    if echo "$signing_response" | grep -q '"enabled".*true'; then
        log_success "Required commit signing: ENABLED"
    elif echo "$signing_response" | grep -q "Branch not protected"; then
        log_warning "Required commit signing: NOT CONFIGURED (branch not protected)"
        echo "  Run: ./scripts/setup-branch-protection-gh.sh to enable protection and commit signing"
    elif echo "$signing_response" | grep -q "Not Found"; then
        log_warning "Required commit signing: NOT CONFIGURED"
        echo "  Run: ./scripts/setup-branch-protection-gh.sh to enable commit signing"
    else
        log_warning "Could not determine commit signing status"
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

                # Test signing capability
                if echo "test" | gpg --clearsign --default-key "$signing_key" >/dev/null 2>&1; then
                    log_success "  GPG key can sign (test successful)"
                else
                    log_warning "  GPG key found but signing test failed"
                fi
            else
                log_error "  GPG key NOT found in keyring"
                echo "    Run: gpg --list-secret-keys to see available keys"
            fi
        else
            log_warning "  No signing key configured"
            echo "    Set with: git config --global user.signingkey YOUR_KEY_ID"
        fi
    else
        log_warning "Local GPG commit signing: DISABLED"
        echo "  Enable with: git config --global commit.gpgsign true"
    fi

    # Check user configuration
    local user_name user_email
    user_name=$(git config --get user.name 2>/dev/null || echo "")
    user_email=$(git config --get user.email 2>/dev/null || echo "")

    if [ -n "$user_name" ] && [ -n "$user_email" ]; then
        log_success "Git user configuration: $user_name <$user_email>"
    else
        log_warning "Git user configuration incomplete"
        [ -z "$user_name" ] && echo "  Missing name: git config --global user.name 'Your Name'"
        [ -z "$user_email" ] && echo "  Missing email: git config --global user.email 'your@email.com'"
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
    local found_workflows=0

    for workflow in "${required_workflows[@]}"; do
        if [ -f "$workflows_dir/$workflow" ]; then
            log_success "Workflow found: $workflow"
            found_workflows=$((found_workflows + 1))
        else
            log_warning "Missing workflow: $workflow"
        fi
    done

    # Check CI workflow triggers
    if [ -f "$workflows_dir/ci.yml" ]; then
        if grep -q "pull_request:" "$workflows_dir/ci.yml"; then
            log_success "CI workflow triggers on pull requests"
        else
            log_warning "CI workflow may not trigger on pull requests"
        fi

        if grep -q "push:" "$workflows_dir/ci.yml"; then
            log_success "CI workflow triggers on push"
        else
            log_info "CI workflow doesn't trigger on direct push (expected with protection)"
        fi
    fi

    log_info "Found $found_workflows out of ${#required_workflows[@]} recommended workflows"
}

# Check recent workflow runs
check_recent_workflows() {
    log_header "Recent Workflow Activity"

    local recent_runs
    recent_runs=$(gh run list --limit 5 --json status,conclusion,name,createdAt 2>/dev/null || echo "[]")

    if [ "$recent_runs" = "[]" ]; then
        log_warning "No recent workflow runs found"
        return
    fi

    log_info "Recent workflow runs:"
    echo "$recent_runs" | jq -r '.[] | "  \(.name): \(.status) (\(.conclusion // "in-progress")) - \(.createdAt)"' | head -5
}

# Generate security score
calculate_security_score() {
    log_header "Security Score Assessment"

    local score=0
    local max_score=10
    local details=()

    # Get protection data once
    local protection_data
    protection_data=$(gh api "repos/$REPO_OWNER/$REPO_NAME/branches/$BRANCH/protection" 2>&1 || true)
    local protection_enabled=false

    # Branch protection exists (+2)
    if echo "$protection_data" | grep -q '"required_status_checks"'; then
        protection_enabled=true
        score=$((score + 2))
        details+=("✓ Branch protection enabled (+2)")
    else
        details+=("✗ Branch protection disabled (0)")
    fi

    if [ "$protection_enabled" = "true" ]; then
        # Required status checks (+2)
        if echo "$protection_data" | jq -e '.required_status_checks != null' >/dev/null 2>&1; then
            score=$((score + 2))
            details+=("✓ Status checks required (+2)")
        else
            details+=("✗ No status checks required (0)")
        fi

        # PR reviews required (+2)
        if echo "$protection_data" | jq -e '.required_pull_request_reviews != null' >/dev/null 2>&1; then
            score=$((score + 2))
            details+=("✓ PR reviews required (+2)")
        else
            details+=("✗ No PR reviews required (0)")
        fi

        # Admin enforcement (+1)
        if echo "$protection_data" | jq -e '.enforce_admins.enabled == true' >/dev/null 2>&1; then
            score=$((score + 1))
            details+=("✓ Admin enforcement enabled (+1)")
        else
            details+=("✗ Admin enforcement disabled (0)")
        fi

        # Force pushes disabled (+1)
        if echo "$protection_data" | jq -e '.allow_force_pushes.enabled == false' >/dev/null 2>&1; then
            score=$((score + 1))
            details+=("✓ Force pushes disabled (+1)")
        else
            details+=("✗ Force pushes allowed (0)")
        fi

        # Conversation resolution (+1)
        if echo "$protection_data" | jq -e '.required_conversation_resolution.enabled == true' >/dev/null 2>&1; then
            score=$((score + 1))
            details+=("✓ Conversation resolution required (+1)")
        else
            details+=("✗ Conversation resolution not required (0)")
        fi
    fi

    # GPG signing configured locally (+1)
    if [ "$(git config --get commit.gpgsign 2>/dev/null)" = "true" ]; then
        score=$((score + 1))
        details+=("✓ Local GPG signing enabled (+1)")
    else
        details+=("✗ Local GPG signing disabled (0)")
    fi

    # Workflows present (+1)
    if [ -f ".github/workflows/ci.yml" ]; then
        score=$((score + 1))
        details+=("✓ CI workflow present (+1)")
    else
        details+=("✗ No CI workflow found (0)")
    fi

    # Display score with color coding
    local percentage=$((score * 100 / max_score))

    echo "Score breakdown:"
    for detail in "${details[@]}"; do
        echo "  $detail"
    done
    echo

    if [ $percentage -ge 80 ]; then
        log_success "Security Score: $score/$max_score ($percentage%) - EXCELLENT"
    elif [ $percentage -ge 60 ]; then
        log_warning "Security Score: $score/$max_score ($percentage%) - GOOD"
    else
        log_error "Security Score: $score/$max_score ($percentage%) - NEEDS IMPROVEMENT"
    fi

    echo
    if [ $percentage -lt 80 ]; then
        echo "💡 To improve your score:"
        echo "  • Run ./scripts/setup-branch-protection-gh.sh for full protection"
        echo "  • Enable local GPG signing: git config --global commit.gpgsign true"
        echo "  • Set up required CI/CD workflows"
    fi
}

# Provide recommendations
show_recommendations() {
    log_header "Recommendations & Next Steps"

    echo "🛡️ Security Best Practices:"
    echo "  • All protection rules should be enabled for maximum security"
    echo "  • GPG signing proves commit authenticity - enable if not active"
    echo "  • Regular security audits catch vulnerabilities early"
    echo
    echo "🔧 GitHub CLI Workflow Tips:"
    echo "  • Use 'gh pr create' for quick PR creation"
    echo "  • Monitor with 'gh pr status' and 'gh run list'"
    echo "  • Check protection: 'gh api repos/$REPO_OWNER/$REPO_NAME/branches/$BRANCH/protection'"
    echo
    echo "📋 If Protection is Missing:"
    echo "  • Run: ./scripts/setup-branch-protection-gh.sh"
    echo "  • Or manually configure via GitHub Settings → Branches"
    echo
    echo "🚀 Testing Your Setup:"
    echo "  • Create a test feature branch: git checkout -b test/protection-check"
    echo "  • Make a small change and commit: git commit -S -m 'test: protection'"
    echo "  • Push and create PR: git push origin test/protection-check && gh pr create"
    echo "  • Verify CI runs and protection works before merging"
}

# Main execution
main() {
    echo -e "${BOLD}${CYAN}🛡️  Branch Protection Verification (GitHub CLI)${NC}"
    echo -e "${CYAN}===============================================${NC}\n"

    echo -e "${BLUE}Repository:${NC} $REPO_OWNER/$REPO_NAME"
    echo -e "${BLUE}Branch:${NC} $BRANCH"
    echo -e "${BLUE}Authentication:${NC} GitHub CLI"

    check_requirements

    local protection_status
    protection_status=0
    check_protection_exists || protection_status=$?

    if [ $protection_status -eq 1 ]; then
        echo
        log_error "Branch protection is not enabled!"
        echo "Run: ./scripts/setup-branch-protection-gh.sh to enable protection"
        echo
        log_info "Continuing with local configuration checks..."
        echo
    elif [ $protection_status -eq 2 ]; then
        echo
        log_warning "Could not verify protection status, continuing with available checks..."
        echo
    fi

    if [ $protection_status -eq 0 ]; then
        verify_protection_rules
        check_commit_signing
    else
        log_info "Skipping detailed protection rule verification (protection not enabled)"
        echo
    fi
    check_local_git_config
    check_workflow_status
    check_recent_workflows
    calculate_security_score
    show_recommendations

    echo -e "\n${BOLD}${GREEN}Verification completed!${NC}"
    echo -e "\n${CYAN}💡 Quick commands:${NC}"
    echo "  • gh pr create                     - Create new pull request"
    echo "  • gh pr status                     - Check PR status"
    echo "  • gh run list                      - Recent workflow runs"
    echo "  • ./scripts/setup-branch-protection-gh.sh  - Reconfigure protection"
}

# Run main function
main "$@"
