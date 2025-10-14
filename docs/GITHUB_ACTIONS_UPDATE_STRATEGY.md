# GitHub Actions Update Strategy

This document outlines the strategy for updating GitHub Actions to avoid breaking changes while maintaining security and performance.

## 📋 Current State

As of October 2025, the project uses various GitHub Actions with pinned major versions. Dependabot PR #55 proposed major version updates that include breaking changes.

## 🎯 Strategy

### 1. Phased Update Approach

#### Phase 1: Non-Breaking Updates (Immediate)

Update actions with backward-compatible changes:

- ✅ `actions/checkout`: v4 → v5 (minor breaking changes, well-documented)
- ✅ `ossf/scorecard-action`: 2.4.2 → 2.4.3 (patch update)
- ⚠️ Skip for now: Actions with major API changes

#### Phase 2: Test Breaking Changes (Next Sprint)

- **`codecov/codecov-action`: v4 → v5**
  - ⚠️ Breaking changes:
    - `file` → `files` (parameter renamed)
    - `plugin` → `plugins` (parameter renamed)
  - 📝 Action required:
    - Review all workflows using codecov
    - Update parameter names
    - Test coverage upload functionality

- **`actions/download-artifact`: v4 → v5**
  - ⚠️ Breaking change: Path behavior for single artifact downloads by ID
  - 📝 Action required:
    - Review artifact download workflows
    - Test artifact extraction paths

- **`actions/github-script`: v7 → v8**
  - ⚠️ Requires runner v2.327.1+
  - 📝 Action required:
    - Verify runner version compatibility

#### Phase 3: Major Ecosystem Updates (Future)

- **`github/codeql-action`: v3 → v4**
- **`EmbarkStudios/cargo-deny-action`: v1 → v2**
- **Other major version bumps**

## 🔧 Implementation Plan

### Step 1: Create Feature Branch

```bash
git checkout -b ci/github-actions-updates-phase1
```

### Step 2: Update Non-Breaking Actions

Update the following in `.github/workflows/*.yml`:

```yaml
# Safe updates (Phase 1)
- uses: actions/checkout@v5  # was v4
- uses: ossf/scorecard-action@v2.4.3  # was v2.4.2
```

### Step 3: Test Breaking Changes Locally

For codecov v5:

```yaml
# Before (v4)
- uses: codecov/codecov-action@v4
  with:
    file: ./coverage.xml

# After (v5)
- uses: codecov/codecov-action@v5
  with:
    files: ./coverage.xml  # Note: plural
```

### Step 4: Validation Checklist

- [ ] All workflows parse correctly (`yamllint`)
- [ ] Local workflow validation passes
- [ ] Test on feature branch with full CI run
- [ ] Coverage upload works correctly
- [ ] Artifact downloads work correctly
- [ ] No unexpected behavior in CI

## 📊 Testing Procedure

### Pre-Deployment

1. Create test PR with actions updates
2. Run full CI/CD pipeline
3. Verify all checks pass
4. Test artifact uploads/downloads
5. Verify coverage reporting works
6. Check for any warnings or deprecations

### Post-Deployment

1. Monitor first 3-5 CI runs
2. Check for any errors or warnings
3. Verify performance hasn't degraded
4. Confirm all features working

## 🚨 Rollback Plan

If issues occur:

```bash
# Revert the PR
git revert <commit-sha>
git push

# Or force push to remove commits
git reset --hard HEAD~1
git push --force
```

## 📝 Breaking Changes Documentation

### Codecov v4 → v5

**Migration Guide:**

- Replace `file:` with `files:`
- Replace `plugin:` with `plugins:`
- Review token requirements (new opt-out feature for public repos)
- See: <https://github.com/codecov/codecov-action#migration-guide>

**New Features:**

- Wrapper architecture (faster updates)
- Optional tokens for public repositories
- New parameters: `binary`, `gcov_*`, `report_type`, etc.

### Actions/Download-Artifact v4 → v5

**Breaking Change:**

- Single artifact downloads by ID now extract directly to path (not nested)

**Migration:**

```yaml
# Before v5 (nested structure)
- uses: actions/download-artifact@v4
  with:
    artifact-ids: 12345
    path: dist
# Files were in: dist/my-artifact/

# After v5 (direct structure)
- uses: actions/download-artifact@v5
  with:
    artifact-ids: 12345
    path: dist
# Files are now in: dist/
```

## 🔗 References

- [Codecov Action v5 Release Notes](https://github.com/codecov/codecov-action/releases/tag/v5.0.0)
- [Download Artifact v5 Release Notes](https://github.com/actions/download-artifact/releases/tag/v5.0.0)
- [GitHub Script v8 Release Notes](https://github.com/actions/github-script/releases/tag/v8.0.0)
- [GitHub Actions Runner Releases](https://github.com/actions/runner/releases)

## ✅ Recommendations

1. **Incremental Updates**: Don't update all actions at once
2. **Test Thoroughly**: Use feature branches for testing
3. **Monitor Closely**: Watch first few CI runs after updates
4. **Document Changes**: Keep this file updated with decisions
5. **Version Pinning**: Pin to specific versions, not floating tags
6. **Security First**: Prioritize security updates over feature updates

## 📅 Timeline

- **Phase 1**: October 2025 (Non-breaking updates)
- **Phase 2**: November 2025 (Breaking changes with testing)
- **Phase 3**: Q1 2026 (Major ecosystem updates)

## 👥 Responsibility

- **Owner**: @docdyhr
- **Reviewers**: All contributors
- **Testing**: Automated CI + manual verification

---

*Last Updated: October 14, 2025*
