# Publishing Guide

This guide covers how to publish batless to various package registries.

## First-time Publication to crates.io

### Prerequisites

1. Create a crates.io account at https://crates.io
2. Get your API token from https://crates.io/me
3. Login locally: `cargo login <your-api-token>`

### Publishing Process

1. **Use the publish script for first-time publication:**
   ```bash
   ./scripts/publish-crates-io.sh
   ```

2. **After successful publication, update README.md:**
   - Replace the "coming soon" badge with the real crates.io badge:
   ```markdown
   [![Crates.io](https://img.shields.io/crates/v/batless.svg)](https://crates.io/crates/batless)
   ```
   - Update the installation section to remove "Coming Soon"

3. **Create a GitHub release:**
   ```bash
   git tag v0.1.5
   git push origin v0.1.5
   ```

## Subsequent Releases

After the initial publication, the automated release workflow will handle:
- Building binaries for all platforms
- Creating GitHub releases
- Publishing to crates.io
- Updating the Homebrew tap

Simply create and push a tag:
```bash
git tag v0.1.6
git push origin v0.1.6
```

## Manual Release Process

If needed, you can trigger a manual release:
```bash
gh workflow run manual-release.yml -f version=0.1.6 -f create_tag=true -f dry_run=false
```

## Troubleshooting

### crates.io Publication Fails
- Ensure you're logged in: `cargo login --list`
- Check that the package name is available
- Verify all required fields in Cargo.toml
- Run `cargo publish --dry-run` first

### Badge Not Updating
- It may take a few minutes for crates.io to update
- Try clearing your browser cache
- The badge URL should be: `https://img.shields.io/crates/v/batless.svg`