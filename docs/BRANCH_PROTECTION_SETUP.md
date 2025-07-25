# Branch Protection Setup - Quick Start Guide

## 🚀 TL;DR - Quick Setup (GitHub CLI)

```bash
# 1. Authenticate with GitHub CLI (if not already done)
gh auth login

# 2. Run the GitHub CLI setup script
./scripts/setup-branch-protection-gh.sh

# 3. Verify configuration
./scripts/verify-protection-gh.sh
```

## 📋 What This Sets Up

### ✅ Branch Protection Rules
- **Pull requests required** - No direct commits to main
- **Status checks required** - CI/CD must pass before merge
- **Up-to-date branches** - Must rebase/merge before PR merge
- **Conversation resolution** - All review comments must be resolved
- **GPG signed commits** - All commits must be cryptographically signed
- **Admin enforcement** - Rules apply to repository owner too

### 🔧 Required Status Checks
- `Test (ubuntu-latest, stable)` - Linux testing
- `Test (windows-latest, stable)` - Windows testing  
- `Test (macos-latest, stable)` - macOS testing
- `Security Audit` - Dependency vulnerability scanning
- `Code Coverage` - Minimum coverage requirements

## 🔐 Prerequisites

### 1. GitHub CLI Authentication
Authenticate with GitHub CLI (more secure than tokens):
```bash
gh auth login
# Follow prompts to authenticate via browser or token
# Select HTTPS or SSH protocol
# This is a one-time setup
```

### 2. GPG Key Setup
```bash
# Generate GPG key (if needed)
gpg --full-generate-key

# Get your key ID
gpg --list-secret-keys --keyid-format=long

# Configure git
git config --global user.signingkey YOUR_KEY_ID
git config --global commit.gpgsign true

# Add public key to GitHub
gpg --armor --export YOUR_KEY_ID
# Copy output to GitHub → Settings → SSH and GPG keys
```

## 📱 Three Setup Methods

### Method 1: GitHub CLI Script (Recommended)
```bash
gh auth login  # If not already authenticated
./scripts/setup-branch-protection-gh.sh
```

### Method 2: GitHub CLI Direct
```bash
gh auth login
gh api repos/docdyhr/batless/branches/main/protection \
  --method PUT --input scripts/github-protection-config.yml
```

### Method 3: Legacy Token Script
```bash
export GITHUB_TOKEN="ghp_your_token"
./scripts/setup-branch-protection.sh
```

### Method 4: Manual (GitHub Web UI)
1. Repository → Settings → Branches
2. Add rule for `main` branch
3. Configure all protection options as documented

## 🏗️ New Development Workflow

### Before (Direct Commits)
```bash
git add .
git commit -m "changes"
git push origin main  # ❌ This will now fail
```

### After (Pull Request Workflow)
```bash
# 1. Create feature branch
git checkout -b feature/my-feature

# 2. Make changes and commit
git add .
git commit -S -m "feat: add new feature"  # Note: -S for signing

# 3. Push feature branch
git push origin feature/my-feature

# 4. Create pull request
gh pr create --title "feat: add new feature" --body "Description"

# 5. Wait for CI/CD checks to pass

# 6. Merge when ready
gh pr merge --squash

# 7. Clean up
git checkout main
git pull origin main
git branch -d feature/my-feature
```

## 🚨 Emergency Procedures

### Hotfix Process
```bash
git checkout -b hotfix/critical-fix
# Make minimal changes
git commit -S -m "hotfix: critical issue"
git push origin hotfix/critical-fix
gh pr create --title "HOTFIX: Critical issue"
# Expedited review and merge
```

### Override Protection (Last Resort)
1. Settings → Branches → Edit rule
2. Temporarily disable required checks
3. Make direct commit
4. **Immediately re-enable protection**

## ✅ Verification Checklist

Run the verification script: `./scripts/verify-protection.sh`

Expected results:
- ✅ Branch protection enabled
- ✅ Required status checks configured
- ✅ Pull request reviews required
- ✅ Admin enforcement enabled
- ✅ Force pushes disabled
- ✅ Commit signing required
- ✅ Local GPG configuration valid

## 🎯 Benefits for Solo Developers

### Short-term
- **Quality gates** - Automated testing prevents regressions
- **Documentation** - PR descriptions document all changes
- **Security** - GPG signatures prove commit authenticity

### Long-term  
- **Professional habits** - Ready for team collaboration
- **Audit trail** - Complete history of changes and rationale
- **Compliance** - Meet enterprise security requirements
- **Future-proofing** - Infrastructure ready for contributors

## 📊 Security Score Target

Aim for **8+/10** security score:
- Branch protection: +2 points
- Status checks: +2 points  
- PR reviews: +2 points
- Admin enforcement: +1 point
- Force pushes disabled: +1 point
- GPG signing: +1 point
- CI workflows: +1 point

## 🔧 Troubleshooting

### Common Issues
**CI checks failing:**
```bash
cargo test --all-features
cargo clippy -- -D warnings  
cargo fmt --all -- --check
gh run list  # Check recent workflow runs
gh run view RUN_ID  # View specific run details
```

**GPG signing problems:**
```bash
git config --list | grep gpg
gpg --list-secret-keys
echo "test" | gpg --clearsign
gh auth status  # Check GitHub CLI auth
```

**Branch not up to date:**
```bash
git checkout feature-branch
git rebase main
git push --force-with-lease origin feature-branch
gh pr status  # Check PR status
```

## 📚 Next Steps

1. **Test the workflow** - Create a sample PR: `gh pr create --title "test: verify protection"`
2. **Read full documentation** - See `docs/BRANCH_PROTECTION.md` for details
3. **Set up GitHub CLI aliases** - Streamline common commands with `gh alias set`
4. **Configure IDE integration** - Set up commit signing and GitHub CLI in your editor

## 🆘 Getting Help

- **Script issues**: Check script output and ensure `gh auth login` worked
- **GitHub CLI errors**: Run `gh auth status` to verify authentication
- **GPG problems**: Check key configuration with `gpg --list-keys`
- **Workflow questions**: Run `./scripts/verify-protection-gh.sh` for diagnostics

---

**Remember**: Branch protection creates sustainable development practices. The initial setup time pays dividends in code quality, security, and maintainability.