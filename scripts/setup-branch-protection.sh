#!/bin/bash

# Branch Protection Setup Script for Solo Developer
# Sets up comprehensive branch protection following best practices
# while maintaining flexibility for solo development

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
NC='\033[0m' # No Color

# Helper functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check requirements
check_requirements() {
    log_info "Checking requirements..."

    if ! command -v curl &>/dev/null; then
        log_error "curl is required but not installed"
        exit 1
    fi

    if ! command -v jq &>/dev/null; then
        log_error "jq is required but not installed"
        exit 1
    fi

    if [ -z "${GITHUB_TOKEN:-}" ]; then
        log_error "GITHUB_TOKEN environment variable is required"
        echo "Please set it with: export GITHUB_TOKEN=your_token_here"
        exit 1
    fi

    log_success "Requirements check passed"
}

# Apply branch protection rules
apply_protection() {
    log_info "Applying branch protection rules to '$BRANCH' branch..."

    # Branch protection configuration optimized for solo developer
    local protection_config='{
        "required_status_checks": {
            "strict": true,
            "contexts": [
                "Test (ubuntu-latest, stable)",
                "Test (windows-latest, stable)",
                "Test (macos-latest, stable)",
                "Security Audit",
                "Code Coverage"
            ]
        },
        "enforce_admins": true,
        "required_pull_request_reviews": {
            "required_approving_review_count": 1,
            "dismiss_stale_reviews": true,
            "require_code_owner_reviews": false,
            "require_last_push_approval": false,
            "bypass_pull_request_allowances": {
                "users": [],
                "teams": [],
                "apps": []
            }
        },
        "restrictions": null,
        "required_linear_history": false,
        "allow_force_pushes": false,
        "allow_deletions": false,
        "block_creations": false,
        "required_conversation_resolution": true,
        "lock_branch": false,
        "allow_fork_syncing": true
    }'

    # Apply the protection rules
    local response
    response=$(curl -s -w "%{http_code}" \
        -X PUT \
        -H "Authorization: token $GITHUB_TOKEN" \
        -H "Accept: application/vnd.github.v3+json" \
        -H "Content-Type: application/json" \
        -d "$protection_config" \
        "$GITHUB_API_URL")

    local http_code="${response: -3}"
    local body="${response%???}"

    if [ "$http_code" -eq 200 ]; then
        log_success "Branch protection rules applied successfully!"
        echo "$body" | jq -r '.message // "Protection rules configured"' 2>/dev/null || true
    else
        log_error "Failed to apply branch protection rules (HTTP $http_code)"
        echo "$body" | jq -r '.message // .error // "Unknown error"' 2>/dev/null || echo "$body"
        exit 1
    fi
}

# Setup commit signature requirement
setup_commit_signing() {
    log_info "Setting up required commit signing..."

    local signing_config='{
        "required_signatures": true
    }'

    local signing_url="https://api.github.com/repos/$REPO_OWNER/$REPO_NAME/protected_branches/$BRANCH/required_signatures"

    local response
    response=$(curl -s -w "%{http_code}" \
        -X POST \
        -H "Authorization: token $GITHUB_TOKEN" \
        -H "Accept: application/vnd.github.sigstore-protected-signature+json" \
        -H "Content-Type: application/json" \
        -d "$signing_config" \
        "$signing_url")

    local http_code="${response: -3}"
    local body="${response%???}"

    if [ "$http_code" -eq 200 ] || [ "$http_code" -eq 201 ]; then
        log_success "Required commit signing enabled!"
    else
        log_warning "Could not enable required commit signing (HTTP $http_code)"
        echo "$body" | jq -r '.message // .error // "Unknown error"' 2>/dev/null || echo "$body"
    fi
}

# Verify current protection status
verify_protection() {
    log_info "Verifying branch protection configuration..."

    local response
    response=$(curl -s -w "%{http_code}" \
        -H "Authorization: token $GITHUB_TOKEN" \
        -H "Accept: application/vnd.github.v3+json" \
        "$GITHUB_API_URL")

    local http_code="${response: -3}"
    local body="${response%???}"

    if [ "$http_code" -eq 200 ]; then
        log_success "Current branch protection configuration:"
        echo "$body" | jq '{
            required_status_checks: .required_status_checks,
            enforce_admins: .enforce_admins,
            required_pull_request_reviews: .required_pull_request_reviews,
            allow_force_pushes: .allow_force_pushes,
            allow_deletions: .allow_deletions,
            required_conversation_resolution: .required_conversation_resolution
        }' 2>/dev/null || echo "$body"
    else
        log_error "Failed to verify protection status (HTTP $http_code)"
        echo "$body" | jq -r '.message // .error // "Unknown error"' 2>/dev/null || echo "$body"
    fi
}

# Display best practices
show_best_practices() {
    cat <<'EOF'

🛡️  BRANCH PROTECTION BEST PRACTICES FOR SOLO DEVELOPERS

✅ What's Now Protected:
  • All commits must be signed with GPG
  • Pull requests required (even for solo work)
  • CI/CD checks must pass before merge
  • Branches must be up-to-date before merge
  • Conversation resolution required
  • No force pushes allowed
  • No branch deletions allowed
  • Admin rules apply to repository owner

📋 Your Workflow:
  1. Create feature branches: git checkout -b feature/new-feature
  2. Make changes and commit with GPG signing
  3. Push branch: git push origin feature/new-feature
  4. Create pull request on GitHub
  5. Wait for CI/CD checks to pass
  6. Merge pull request (or approve if self-reviewing)

🔧 GPG Signing Setup (if not already done):
  • git config --global user.signingkey YOUR_GPG_KEY_ID
  • git config --global commit.gpgsign true
  • git config --global tag.gpgsign true

💡 Solo Developer Tips:
  • Use draft PRs for work-in-progress
  • Self-review is encouraged for documentation
  • Emergency hotfixes can still be applied via GitHub web interface
  • Repository insights will track your development velocity

🚨 Override Options (use sparingly):
  • Repository admins can temporarily disable protection
  • Use GitHub web interface for critical hotfixes
  • Consider creating hotfix branches for urgent changes

EOF
}

# Main execution
main() {
    echo "🛡️  Branch Protection Setup for Solo Developer"
    echo "================================================"
    echo

    check_requirements
    echo

    log_info "Repository: $REPO_OWNER/$REPO_NAME"
    log_info "Branch: $BRANCH"
    echo

    # Confirm before applying
    echo -n "Apply branch protection rules? [y/N]: "
    read -r confirmation
    if [[ ! "$confirmation" =~ ^[Yy]$ ]]; then
        log_info "Cancelled by user"
        exit 0
    fi

    apply_protection
    echo

    setup_commit_signing
    echo

    verify_protection
    echo

    show_best_practices

    log_success "Branch protection setup completed!"
}

# Run main function
main "$@"
