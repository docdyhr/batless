# GitHub Actions Incremental Update Plan

**Created:** October 20, 2025
**Status:** Ready for implementation
**Context:** Closed Dependabot PR #60 due to bulk breaking changes

## Overview

This plan outlines the safe, incremental approach to updating 8 GitHub Actions dependencies that were bundled in the now-closed PR #60.

---

## Phase 1: Non-Breaking Updates (This Week)

### Priority: LOW RISK - Safe to merge quickly

#### 1.1. ossf/scorecard-action: 2.4.2 → 2.4.3
- **Type:** Patch update
- **Risk:** Very Low
- **Testing Required:** Minimal
- **Action:**
  ```bash
  # Update in workflow files
  find .github/workflows -name "*.yml" -exec sed -i '' 's/ossf\/scorecard-action@2.4.2/ossf\/scorecard-action@2.4.3/g' {} \;
  git add .github/workflows
  git commit -m "ci(deps): bump ossf/scorecard-action from 2.4.2 to 2.4.3"
  gh pr create --title "ci(deps): bump ossf/scorecard-action to 2.4.3" --body "Safe patch update"
  ```

#### 1.2. actions/github-script: 7 → 8
- **Type:** Major version update
- **Risk:** Low (well-maintained action, good backward compatibility)
- **Testing Required:** Verify all scripts still work
- **Breaking Changes:** Review [release notes](https://github.com/actions/github-script/releases/tag/v8.0.0)
- **Affected Workflows:** Check which workflows use github-script
- **Action:**
  ```bash
  # Find usage first
  grep -r "actions/github-script" .github/workflows/
  # Update and test
  ```

#### 1.3. actions/download-artifact: 4 → 5
- **Type:** Major version update
- **Risk:** Low-Medium
- **Testing Required:** Test artifact download functionality
- **Breaking Changes:** Check [migration guide](https://github.com/actions/download-artifact#v4-to-v5-migration)
- **Affected Workflows:** Release workflows, artifact handling
- **Action:**
  ```bash
  # Find usage
  grep -r "actions/download-artifact" .github/workflows/
  # Review each usage context
  ```

---

## Phase 2: Breaking Changes - High Impact (Next Week)

### Priority: HIGH RISK - Requires workflow modifications

#### 2.1. codecov/codecov-action: 4 → 5
- **Type:** Major version update with BREAKING API changes
- **Risk:** **HIGH**
- **Known Breaking Changes:**
  - `file` parameter deprecated → use `files` instead
  - `plugin` parameter deprecated → use `plugins` instead
  - New parameters: `binary`, `gcov_args`, `gcov_executable`, `skip_validation`, etc.
- **Migration Required:** Yes
- **Documentation:** https://github.com/codecov/codecov-action/releases/tag/v5.0.0
- **Action:**
  ```bash
  # 1. Review current usage
  grep -A10 "codecov-action" .github/workflows/

  # 2. Update configuration according to v5 API
  # Example change:
  #   - file: coverage.lcov          # OLD
  #   + files: coverage.lcov         # NEW

  # 3. Test locally with act or in draft PR
  # 4. Monitor coverage reporting after merge
  ```

#### 2.2. github/codeql-action: 3 → 4
- **Type:** Major version update
- **Risk:** **HIGH**
- **Known Breaking Changes:**
  - Updated CodeQL CLI version
  - Changed default queries/packs
  - Modified configuration schema
- **Testing Required:**
  - Verify all security scans complete
  - Check for new findings
  - Validate SARIF upload
- **Action:**
  ```bash
  # Review current CodeQL workflows
  batless -n .github/workflows/code-quality.yml | grep -A20 "codeql"

  # Check for custom configurations
  ls -la .github/codeql/

  # Update and test in isolated PR
  ```

#### 2.3. EmbarkStudios/cargo-deny-action: 1 → 2
- **Type:** Major version update
- **Risk:** Medium
- **Breaking Changes:** Review [v2 changes](https://github.com/EmbarkStudios/cargo-deny-action/releases)
- **Testing Required:**
  - Verify `cargo deny check` still works
  - Check for new deny rules
  - Validate advisory database updates
- **Action:**
  ```bash
  # Current usage
  grep -A5 "cargo-deny-action" .github/workflows/

  # Test locally first
  cargo install cargo-deny@latest
  cargo deny check --all-features
  ```

---

## Phase 3: Remaining Updates (Week After Next)

### Priority: MEDIUM RISK - Larger version jumps

#### 3.1. actions/setup-node: 4 → 6
- **Type:** Major version jump (skipping v5)
- **Risk:** Medium
- **Why Deferred:** Only needed for markdown linting currently
- **Breaking Changes:** Check both v5 and v6 release notes
- **Testing Required:**
  - Verify Node.js version selection works
  - Check npm installation behavior
  - Validate cache behavior
- **Action:**
  ```bash
  # Current usage
  grep -A5 "setup-node" .github/workflows/

  # May need to update node-version syntax
  ```

#### 3.2. dawidd6/action-homebrew-bump-formula: 3 → 5
- **Type:** Two major versions jump
- **Risk:** Medium
- **Impact:** Homebrew tap auto-updates
- **Breaking Changes:** Review v4 and v5 changelogs
- **Testing Required:**
  - Verify formula update logic
  - Check token/authentication changes
  - Test with dry-run if possible
- **Action:**
  ```bash
  # Current usage
  grep -A10 "homebrew-bump-formula" .github/workflows/

  # Review formula update workflow
  batless -n .github/workflows/homebrew-tap.yml
  ```

---

## Testing Strategy

### For Each Update:

1. **Local Testing (where possible)**
   ```bash
   # Use act to test workflows locally
   brew install act
   act -l  # List available jobs
   act -j <job-name> --dryrun
   ```

2. **Draft PR Testing**
   - Create draft PR with single update
   - Monitor all CI workflows
   - Check for unexpected failures
   - Review any new warnings/errors

3. **Validation Checklist**
   - [ ] All CI workflows pass
   - [ ] No new security findings
   - [ ] Coverage reporting works (if applicable)
   - [ ] Artifacts upload/download correctly
   - [ ] No regression in build times

4. **Rollback Plan**
   - Keep previous working version documented
   - Be ready to revert quickly if issues found
   - Monitor first few runs after merge

---

## Implementation Timeline

| Week | Phase | Updates | Status |
|------|-------|---------|--------|
| Week 1 (Oct 21-27) | Phase 1 | ossf/scorecard, github-script, download-artifact | 🔜 Ready |
| Week 2 (Oct 28-Nov 3) | Phase 2 | codecov v5, codeql v4, cargo-deny v2 | ⏳ Planned |
| Week 3 (Nov 4-10) | Phase 3 | setup-node v6, homebrew-bump v5 | ⏳ Planned |

---

## Success Criteria

- ✅ All 8 dependency updates completed
- ✅ Zero CI workflow failures
- ✅ No regression in functionality
- ✅ Documentation updated where needed
- ✅ Team knowledge transfer complete

---

## Notes & Learnings

### Why This Approach?

1. **Safety First:** Incremental updates reduce blast radius
2. **Clear Attribution:** Easy to identify which update causes issues
3. **Learning Opportunity:** Understand each action's breaking changes
4. **CI Stability:** Maintain green builds throughout process

### Dependabot Configuration

Consider configuring Dependabot to:
- Group only patch updates
- Separate major version updates
- Schedule updates less frequently

Example `.github/dependabot.yml`:
```yaml
version: 2
updates:
  - package-ecosystem: "github-actions"
    directory: "/"
    schedule:
      interval: "monthly"
    groups:
      patch-updates:
        patterns:
          - "*"
        update-types:
          - "patch"
      # Major updates should be individual PRs
```

---

## Resources

- [GitHub Actions Changelog](https://github.blog/changelog/label/actions/)
- [Dependabot Documentation](https://docs.github.com/en/code-security/dependabot)
- [act - Local GitHub Actions Testing](https://github.com/nektos/act)

---

**Last Updated:** October 20, 2025
**Owner:** Thomas (batless maintainer)
**Status:** Active planning - ready for Phase 1 implementation
