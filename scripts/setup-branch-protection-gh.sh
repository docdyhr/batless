#!/bin/bash

# Branch Protection Setup Script for Solo Developer (GitHub CLI Version)
# Sets up comprehensive branch protection using GitHub CLI authentication
# Optimized for developers already authenticated with 'gh login'

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
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_header() {
    echo -e "\n${BOLD}${CYAN}$1${NC}"
    echo -e "${CYAN}$(printf '=%.0s' $(seq 1 ${#1}))${NC}"
}

# Check requirements
check_requirements() {
    log_info "Checking requirements..."

    if ! command -v gh &>/dev/null; then
        log_error "GitHub CLI (gh) is required but not installed"
        echo "Install with:"
        echo "  macOS: brew install gh"
        echo "  Ubuntu: sudo apt install gh"
        echo "  Or visit: https://cli.github.com/"
        exit 1
    fi

    if ! command -v jq &>/dev/null; then
        log_error "jq is required but not installed"
        echo "Install with:"
        echo "  macOS: brew install jq"
        echo "  Ubuntu: sudo apt install jq"
        exit 1
    fi

    # Check GitHub CLI authentication
    if ! gh auth status &>/dev/null; then
        log_error "GitHub CLI is not authenticated"
        echo "Please run: gh auth login"
        exit 1
    fi

    # Verify repository access
    if ! gh repo view "$REPO_OWNER/$REPO_NAME" &>/dev/null; then
        log_error "Cannot access repository $REPO_OWNER/$REPO_NAME"
        echo "Please check repository name and permissions"
        exit 1
    fi

    log_success "Requirements check passed"
}

# Check current protection status
check_current_status() {
    log_header "Current Branch Protection Status"

    local protection_status
    protection_status=$(gh api "repos/$REPO_OWNER/$REPO_NAME/branches/$BRANCH/protection" 2>/dev/null || echo "null")

    if [ "$protection_status" = "null" ]; then
        log_warning "Branch protection is currently DISABLED"
        return 1
    else
        log_info "Branch protection is currently ENABLED"

        # Show current status checks
        echo "$protection_status" | jq -r '.required_status_checks.contexts[]?' 2>/dev/null | while read -r context; do
            [ -n "$context" ] && log_info "  Current status check: $context"
        done

        return 0
    fi
}

# Apply branch protection rules
apply_protection() {
    log_header "Applying Branch Protection Rules"

    log_info "Configuring protection for '$BRANCH' branch..."

    # Create protection configuration optimized for solo developer (personal repo)
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
            "require_last_push_approval": false
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

    # Apply the protection rules using GitHub CLI
    if echo "$protection_config" | gh api "repos/$REPO_OWNER/$REPO_NAME/branches/$BRANCH/protection" --method PUT --input -; then
        log_success "Branch protection rules applied successfully!"
    else
        log_error "Failed to apply branch protection rules"
        exit 1
    fi
}

# Setup commit signature requirement
setup_commit_signing() {
    log_header "Setting Up Required Commit Signing"

    log_info "Enabling required GPG commit signatures..."

    local signing_config='{"required_signatures": true}'

    if echo "$signing_config" | gh api "repos/$REPO_OWNER/$REPO_NAME/branches/$BRANCH/protection/required_signatures" --method POST --input - 2>/dev/null; then
        log_success "Required commit signing enabled!"
    else
        log_warning "Required commit signing may already be enabled or not supported"
        log_info "This is normal and doesn't affect other protections"
    fi
}

# Verify the applied configuration
verify_configuration() {
    log_header "Verifying Configuration"

    local protection_data
    protection_data=$(gh api "repos/$REPO_OWNER/$REPO_NAME/branches/$BRANCH/protection" 2>/dev/null)

    if [ $? -eq 0 ]; then
        log_success "Branch protection verification:"

        # Check status checks
        local status_checks
        status_checks=$(echo "$protection_data" | jq -r '.required_status_checks.strict')
        if [ "$status_checks" = "true" ]; then
            log_success "  ✓ Strict status checks enabled"
        else
            log_warning "  ⚠ Strict status checks disabled"
        fi

        # Check PR reviews
        local pr_reviews
        pr_reviews=$(echo "$protection_data" | jq -r '.required_pull_request_reviews.required_approving_review_count')
        if [ "$pr_reviews" -ge 1 ]; then
            log_success "  ✓ Pull request reviews required ($pr_reviews reviewer)"
        else
            log_warning "  ⚠ Pull request reviews not required"
        fi

        # Check admin enforcement
        local enforce_admins
        enforce_admins=$(echo "$protection_data" | jq -r '.enforce_admins.enabled')
        if [ "$enforce_admins" = "true" ]; then
            log_success "  ✓ Admin enforcement enabled"
        else
            log_warning "  ⚠ Admin enforcement disabled"
        fi

        # Check force pushes
        local force_pushes
        force_pushes=$(echo "$protection_data" | jq -r '.allow_force_pushes.enabled')
        if [ "$force_pushes" = "false" ]; then
            log_success "  ✓ Force pushes disabled"
        else
            log_warning "  ⚠ Force pushes allowed"
        fi

        # List required contexts
        log_info "  Required status check contexts:"
        echo "$protection_data" | jq -r '.required_status_checks.contexts[]?' | while read -r context; do
            [ -n "$context" ] && echo "    - $context"
        done

    else
        log_error "Failed to verify branch protection configuration"
        return 1
    fi
}

# Check local Git and GPG configuration
check_local_setup() {
    log_header "Checking Local Development Setup"

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
                echo "    Run: gpg --list-secret-keys to check available keys"
            fi
        else
            log_warning "  No signing key configured"
            echo "    Set with: git config --global user.signingkey YOUR_KEY_ID"
        fi
    else
        log_warning "Local GPG commit signing: DISABLED"
        echo "  Enable with: git config --global commit.gpgsign true"
        echo "  Set key with: git config --global user.signingkey YOUR_KEY_ID"
    fi

    # Check Git user configuration
    local user_name user_email
    user_name=$(git config --get user.name 2>/dev/null || echo "")
    user_email=$(git config --get user.email 2>/dev/null || echo "")

    if [ -n "$user_name" ] && [ -n "$user_email" ]; then
        log_success "Git user configuration: $user_name <$user_email>"
    else
        log_warning "Git user configuration incomplete"
        echo "  Set with: git config --global user.name 'Your Name'"
        echo "  Set with: git config --global user.email 'your.email@example.com'"
    fi
}

# Display workflow guidance
show_workflow_guide() {
    cat <<'EOF'

🚀 NEW DEVELOPMENT WORKFLOW

Your development workflow has changed! Here's how to work with protected branches:

📋 Step-by-Step Process:
  1. Create feature branch:    git checkout -b feature/my-feature
  2. Make changes and commit:  git add . && git commit -S -m "feat: description"
  3. Push feature branch:      git push origin feature/my-feature
  4. Create pull request:      gh pr create --title "feat: description" --body "Details"
  5. Wait for CI/CD checks:    gh pr status (monitor progress)
  6. Self-review if needed:    gh pr review --approve
  7. Merge when ready:         gh pr merge --squash
  8. Clean up:                 git checkout main && git pull && git branch -d feature/my-feature

💡 GitHub CLI Shortcuts:
  • gh pr create                    - Create PR interactively
  • gh pr list                      - List your PRs
  • gh pr status                    - Check PR status
  • gh pr view                      - View PR details
  • gh pr merge --squash            - Squash and merge
  • gh pr merge --merge             - Create merge commit
  • gh workflow list                - List workflows
  • gh run list                     - List workflow runs

🔧 GPG Signing Setup (if not configured):
  • gpg --full-generate-key         - Generate new GPG key
  • gpg --list-secret-keys --keyid-format=long  - Get key ID
  • git config --global user.signingkey KEY_ID  - Set signing key
  • git config --global commit.gpgsign true     - Enable signing
  • gpg --armor --export KEY_ID      - Export public key for GitHub

🚨 Emergency Procedures:
  • Hotfix: Use hotfix/issue-name branch pattern
  • Critical: Temporarily disable protection via web UI if absolutely necessary
  • Always re-enable protection immediately after emergency fixes

📊 Monitoring:
  • gh pr status      - Check your PR status
  • gh run list       - Recent workflow runs
  • gh run view ID    - Detailed run information
  • Repository Insights on GitHub for overall project health

EOF
}

# Main execution
main() {
    echo -e "${BOLD}${CYAN}🛡️  Branch Protection Setup (GitHub CLI)${NC}"
    echo -e "${CYAN}=========================================${NC}"
    echo
    echo -e "${BLUE}Repository:${NC} $REPO_OWNER/$REPO_NAME"
    echo -e "${BLUE}Branch:${NC} $BRANCH"
    echo -e "${BLUE}Authentication:${NC} GitHub CLI"
    echo

    check_requirements
    echo

    # Show current status
    check_current_status
    echo

    # Confirm before applying
    echo -n "Apply branch protection rules? [y/N]: "
    read -r confirmation
    if [[ ! "$confirmation" =~ ^[Yy]$ ]]; then
        log_info "Setup cancelled by user"
        exit 0
    fi
    echo

    apply_protection
    echo

    setup_commit_signing
    echo

    verify_configuration
    echo

    check_local_setup
    echo

    show_workflow_guide

    log_success "Branch protection setup completed using GitHub CLI!"
    echo
    log_info "Next steps:"
    echo "  1. Test the workflow: Create a sample feature branch and PR"
    echo "  2. Run ./scripts/verify-protection.sh to double-check configuration"
    echo "  3. Read docs/BRANCH_PROTECTION.md for detailed guidance"
}

# Run main function
main "$@"
