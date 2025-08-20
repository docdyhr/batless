# 📋 Release Checklist

Use this checklist to ensure consistent, high-quality releases for batless.

## Pre-Release Preparation

### Code Quality
- [ ] All tests are passing (`cargo test`)
- [ ] Code is properly formatted (`cargo fmt --all -- --check`)
- [ ] No clippy warnings (`cargo clippy --all-targets --all-features -- -D warnings`)
- [ ] Security audit is clean (`cargo audit`)
- [ ] Documentation is up to date
- [ ] Examples are working and current

### Dependencies
- [ ] Dependencies are up to date (`cargo update`)
- [ ] Security advisories have been reviewed
- [ ] No deprecated dependencies
- [ ] License compatibility checked for new dependencies

### Version and Documentation
- [ ] Version number follows semantic versioning
- [ ] CHANGELOG.md updated with all changes since last release
- [ ] Release notes drafted (focus on user-facing changes)
- [ ] README.md updated if needed (features, installation, examples)
- [ ] Breaking changes are clearly documented

## Release Process

### Automated Release (Recommended)
- [ ] Run release script: `./scripts/release.sh X.Y.Z`
- [ ] Review generated changelog entry
- [ ] Verify version update in Cargo.toml
- [ ] Check that tag was created: `git tag | grep vX.Y.Z`

### Manual Release (Fallback)
- [ ] Run prep script: `./scripts/prep-release.sh X.Y.Z`
- [ ] Review changes: `git diff`
- [ ] Commit changes: `git add . && git commit -m "Release vX.Y.Z"`
- [ ] Create tag: `git tag vX.Y.Z`
- [ ] Push: `git push origin main --tags`

## Post-Release Verification

### GitHub Actions
- [ ] Release workflow completed successfully
- [ ] All target platforms built without errors
- [ ] GitHub release created with all assets
- [ ] Checksums and signatures generated

### Package Registries
- [ ] Package published to crates.io successfully
- [ ] Docker images built and pushed to registries
- [ ] Linux packages (.deb, .rpm) generated correctly

### Installation Testing
Test installation on different platforms:
- [ ] **Rust**: `cargo install batless --version X.Y.Z`
- [ ] **GitHub Releases**: Download and test binary
- [ ] **Docker**: `docker run ghcr.io/thomas/batless:X.Y.Z --version`
- [ ] **Homebrew** (if available): `brew install thomas/batless`

### Functionality Testing
Quick smoke tests:
- [ ] `batless --version` shows correct version
- [ ] `batless --help` displays help text
- [ ] Basic syntax highlighting works: `batless src/main.rs`
- [ ] JSON mode works: `batless --mode=json src/main.rs`
- [ ] Summary mode works: `batless --mode=summary src/main.rs`

## Communication

### Internal
- [ ] Team notified of release
- [ ] Release notes reviewed by team
- [ ] Any known issues documented

### External
- [ ] GitHub release published (not draft)
- [ ] Social media announcement (if applicable)
- [ ] Documentation sites updated
- [ ] Community channels notified (Discord, Reddit, etc.)

## Post-Release Monitoring

### First 24 Hours
- [ ] Monitor GitHub issues for bug reports
- [ ] Check crates.io download stats
- [ ] Review GitHub Actions for any failures
- [ ] Monitor Docker Hub/GHCR for pull stats

### First Week
- [ ] User feedback collected and reviewed
- [ ] Performance metrics compared to previous version
- [ ] Security scan results reviewed
- [ ] Plan next release based on feedback

## Rollback Procedures

If critical issues are discovered:

### Emergency Patch Release
- [ ] Create hotfix branch from release tag
- [ ] Apply minimal fix
- [ ] Follow abbreviated release process
- [ ] Increment patch version (X.Y.Z+1)

### Version Retraction (Last Resort)
- [ ] Retract version from crates.io: `cargo yank --version X.Y.Z`
- [ ] Update GitHub release to mark as broken
- [ ] Communicate issue to users
- [ ] Plan replacement release

## Release Types

### Patch Release (X.Y.Z+1)
- [ ] Bug fixes only
- [ ] No new features
- [ ] No breaking changes
- [ ] Can be released immediately

### Minor Release (X.Y+1.0)
- [ ] New features
- [ ] Backwards compatible
- [ ] May include deprecations
- [ ] Requires full testing

### Major Release (X+1.0.0)
- [ ] Breaking changes
- [ ] Major new features
- [ ] Migration guide written
- [ ] Extended beta period recommended

## Template Messages

### Release Announcement Template

```markdown
🚀 batless vX.Y.Z released!

## What's New
- Feature 1: Description
- Feature 2: Description  
- Bug fix: Description

## Installation
cargo install batless --version X.Y.Z

## Full Changelog
https://github.com/thomas/batless/releases/tag/vX.Y.Z

#rust #cli #automation #ai
```

### Bug Report Response Template

```markdown
Thanks for the report! This appears to be related to the vX.Y.Z release.

We're investigating and will have a patch release (vX.Y.Z+1) out within [timeframe].

As a workaround, you can:
- [Workaround steps]
- Or downgrade: cargo install batless --version X.Y.Z-1
```

---

## Notes

- Always test on multiple platforms before release
- Keep this checklist updated as process evolves
- Document any deviations from standard process
- Review and improve process after each release