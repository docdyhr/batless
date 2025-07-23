# Branch Protection Guide for Solo Developers

This guide explains how to set up secure branch protection rules for the `batless` repository while maintaining developer productivity for solo work.

## 🎯 Overview

Branch protection enforces code quality and security standards by requiring pull requests, status checks, and commit signing—even for solo developers. This creates good habits, maintains project history, and ensures consistency.

## 🛡️ Protection Rules Applied

### Required Status Checks
- ✅ **CI/CD Pipeline**: All tests must pass on Ubuntu, Windows, and macOS
- ✅ **Security Audit**: `cargo audit` must pass with no vulnerabilities
- ✅ **Code Coverage**: Minimum coverage requirements enforced
- ✅ **Strict Mode**: Branches must be up-to-date before merging

### Pull Request Requirements
- ✅ **Required Reviews**: At least 1 approval (can be self-approval)
- ✅ **Dismiss Stale Reviews**: Reviews dismissed when new commits pushed
- ✅ **Conversation Resolution**: All review comments must be resolved
- ❌ **Code Owner Reviews**: Not required (solo developer)

### Additional Protections
- ✅ **Enforce for Admins**: Rules apply to repository owner
- ✅ **Required Commit Signing**: All commits must be GPG signed
- ❌ **Linear History**: Merge commits allowed for clarity
- ❌ **Force Pushes**: Disabled to preserve history
- ❌ **Branch Deletion**: Main branch cannot be deleted

## 🚀 Quick Setup

### Option 1: Using the Setup Script

```bash
# Set your GitHub token
export GITHUB_TOKEN="ghp_your_token_here"

# Run the setup script
chmod +x scripts/setup-branch-protection.sh
./scripts/setup-branch-protection.sh
```

### Option 2: Using GitHub CLI

```bash
# Install GitHub CLI if not already installed
# brew install gh  # macOS
# sudo apt install gh  # Ubuntu

# Authenticate
gh auth login

# Apply protection rules
gh api repos/docdyhr/batless/branches/main/protection \
  --method PUT \
  --input scripts/github-protection-config.yml
```

### Option 3: Manual Setup via GitHub Web Interface

1. Go to **Settings** → **Branches** in your repository
2. Click **Add rule** for the `main` branch
3. Configure the following settings:

**Protect matching branches:**
- ✅ Require a pull request before merging
  - ✅ Require approvals: 1
  - ✅ Dismiss stale reviews when new commits are pushed
  - ❌ Require review from CODEOWNERS
  - ❌ Require approval of the most recent reviewable push
- ✅ Require status checks to pass before merging
  - ✅ Require branches to be up to date before merging
  - Select: `Test (ubuntu-latest, stable)`, `Test (windows-latest, stable)`, `Test (macos-latest, stable)`, `Security Audit`, `Code Coverage`
- ✅ Require conversation resolution before merging
- ✅ Require signed commits
- ❌ Require linear history
- ✅ Include administrators
- ❌ Allow force pushes
- ❌ Allow deletions

## 🔐 GPG Commit Signing Setup

### Generate GPG Key (if needed)

```bash
# Generate new GPG key
gpg --full-generate-key

# List GPG keys to get the key ID
gpg --list-secret-keys --keyid-format=long

# Export public key for GitHub
gpg --armor --export YOUR_KEY_ID
```

### Configure Git

```bash
# Set your GPG key
git config --global user.signingkey YOUR_GPG_KEY_ID

# Enable GPG signing by default
git config --global commit.gpgsign true
git config --global tag.gpgsign true

# Set GPG program if needed
git config --global gpg.program gpg
```

### Add GPG Key to GitHub

1. Copy your GPG public key: `gpg --armor --export YOUR_KEY_ID`
2. Go to **Settings** → **SSH and GPG keys** → **New GPG key**
3. Paste your public key and save

## 📋 Solo Developer Workflow

### 1. Feature Development

```bash
# Create and switch to feature branch
git checkout -b feature/new-awesome-feature

# Make your changes
echo "// New feature code" >> src/lib.rs

# Commit with GPG signing (automatic if configured)
git add .
git commit -m "feat: add new awesome feature"

# Push feature branch
git push origin feature/new-awesome-feature
```

### 2. Pull Request Process

```bash
# Create PR using GitHub CLI
gh pr create \
  --title "feat: add new awesome feature" \
  --body "Description of the feature and changes made"

# Or use GitHub web interface to create PR
```

### 3. Review and Merge

1. **Wait for CI/CD**: All status checks must pass
2. **Self-Review**: Review your own code for quality
3. **Approve PR**: Either through web interface or CLI
4. **Merge**: Use "Squash and merge" or "Create merge commit"

```bash
# Check PR status
gh pr status

# Merge when ready
gh pr merge --squash  # or --merge or --rebase
```

### 4. Cleanup

```bash
# Switch back to main
git checkout main

# Pull latest changes
git pull origin main

# Delete feature branch
git branch -d feature/new-awesome-feature
git push origin --delete feature/new-awesome-feature
```

## 🚨 Emergency Procedures

### Hotfix Process

For critical fixes that need immediate deployment:

```bash
# Create hotfix branch from main
git checkout main
git pull origin main
git checkout -b hotfix/critical-security-fix

# Make minimal fix
git add .
git commit -m "hotfix: resolve critical security issue"

# Push and create emergency PR
git push origin hotfix/critical-security-fix
gh pr create --title "HOTFIX: Critical security issue" --body "Emergency fix"
```

### Bypassing Protection (Last Resort)

If absolutely necessary, repository admins can:

1. **Temporarily disable protection**: Settings → Branches → Edit rule → Uncheck options
2. **Make direct commits**: Push directly to main
3. **Re-enable protection**: Restore all rules immediately after

⚠️ **Use sparingly** - defeats the purpose of protection rules.

## 📊 Monitoring and Insights

### GitHub Insights

- **Pulse**: Track repository activity
- **Contributors**: Monitor commit frequency
- **Code frequency**: See development patterns
- **Dependency graph**: Track security alerts

### Status Check Monitoring

```bash
# Check recent workflow runs
gh run list

# View specific workflow details
gh run view RUN_ID

# Watch workflow in real-time
gh run watch
```

## 🔧 Troubleshooting

### Common Issues

**❌ Status checks failing**
```bash
# Run tests locally first
cargo test --all-features
cargo clippy -- -D warnings
cargo fmt --all -- --check
```

**❌ GPG signing issues**
```bash
# Check GPG configuration
git config --list | grep gpg
gpg --list-secret-keys

# Test signing
echo "test" | gpg --clearsign
```

**❌ Branch not up to date**
```bash
# Update your feature branch
git checkout feature/your-feature
git rebase main  # or git merge main
git push --force-with-lease origin feature/your-feature
```

### Getting Help

- **GitHub CLI**: `gh --help`
- **Git GPG**: `git help config` (search for gpg)
- **Repository Issues**: Create an issue for project-specific questions

## 📈 Benefits of Branch Protection

### For Solo Developers

1. **Enforced Quality**: Automated testing prevents regressions
2. **Better Documentation**: PR descriptions document changes
3. **Security Assurance**: Signed commits prove authenticity
4. **Professional Habits**: Builds good practices for team environments
5. **Project History**: Clear record of all changes and rationale

### For Project Maintenance

1. **Audit Trail**: Complete history of who changed what and when
2. **Quality Gates**: Automated prevention of broken releases
3. **Security Compliance**: Meet enterprise security requirements
4. **Backup Safety**: Protection against accidental deletions
5. **Future Collaboration**: Ready for contributors when they arrive

## 📚 Additional Resources

- [GitHub Branch Protection Documentation](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/defining-the-mergeability-of-pull-requests/about-protected-branches)
- [GPG Signing Documentation](https://docs.github.com/en/authentication/managing-commit-signature-verification)
- [GitHub CLI Manual](https://cli.github.com/manual/)
- [Git Flow for Solo Developers](https://nvie.com/posts/a-successful-git-branching-model/)

---

**Remember**: Branch protection is about creating sustainable development practices, not hindering productivity. The short-term overhead pays dividends in code quality, security, and maintainability.