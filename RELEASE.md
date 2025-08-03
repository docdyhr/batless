# 📦 Release & Distribution Guide for `batless`

> Complete guide for packaging, releasing, and distributing batless following Rust ecosystem best practices.

[![Crates.io](https://img.shields.io/crates/v/batless.svg)](https://crates.io/crates/batless)
[![Downloads](https://img.shields.io/crates/d/batless.svg)](https://crates.io/crates/batless)

## 🎯 Distribution Overview

**batless** supports multiple distribution channels:

- 📦 **crates.io** - Primary Rust package registry
- 🏷️ **GitHub Releases** - Cross-platform binaries with automated builds
- 🍺 **Homebrew** - macOS package manager integration
- 🐧 **Linux packages** - .deb, .rpm, and Arch AUR support
- 🪟 **Windows** - Chocolatey and Scoop package managers
- 🐳 **Docker** - Containerized distribution

---

## 🚀 Quick Start for Maintainers

### Prerequisites

```bash
# Install required tools
cargo install cargo-release cargo-audit

# Verify tools
cargo release --version
```

### One-Time Setup

```bash
# 1. Setup release automation
# This project uses GitHub Actions for releases, no additional setup needed

# 2. Configure release automation
cargo dist generate-ci

# 3. Commit configuration
git add .
git commit -m "Configure release automation"
```

---

## 🔄 Release Process

### Method 1: Automated Release (Recommended)

```bash
# Use our release script (recommended)
./scripts/release.sh 0.2.1

# Or use cargo-release for direct release
cargo release 0.2.1 --execute
```

### Method 2: Manual Release (Emergency/Backup)

```bash
# 1. Ensure you have crates.io token
export CARGO_REGISTRY_TOKEN="your-token-here"

# 2. Update version and changelog
./scripts/prep-release.sh 0.2.1

# 3. Review changes and commit
git add .
git commit -m "Release v0.2.1"

# 4. Create and push tag
git tag -a v0.2.1 -m "Release v0.2.1"
git push origin v0.2.1

# 5. Publish to crates.io manually if needed
cargo release publish --execute

# 6. Create GitHub release manually if needed
gh release create v0.2.1 --title "batless v0.2.1" --notes "Release notes here"
```

### Method 3: Recovery from Failed Release

If a release partially fails (tag exists but not published):

```bash
# Check what's missing
curl -s "https://crates.io/api/v1/crates/batless" | jq -r '.crate.max_version'
curl -s "https://api.github.com/repos/docdyhr/batless/releases/latest" | jq -r '.tag_name'

# Publish to crates.io if missing
cargo release publish --execute

# Create GitHub release if missing
gh release create v0.2.1 --title "Release title" --notes "Release notes"
```

### What Happens on Release

1. 🧪 **CI Pipeline** runs full test suite across platforms
2. 🔨 **Cross-compilation** builds binaries for all targets
3. 📦 **Package Creation** generates .deb, .rpm, .msi installers
4. 🏷️ **GitHub Release** created with binaries and checksums
5. 📚 **crates.io** package published automatically
6. 🍺 **Homebrew** formula updated in tap repository
7. 🐳 **Docker** images built and pushed to registries

---

## 🎯 Supported Platforms

### Primary Targets (Tier 1)

| Platform | Architecture | Package Format | Status |
|----------|-------------|----------------|---------|
| Linux | x86_64 | .deb, .rpm, .tar.gz | ✅ Full support |
| macOS | x86_64, ARM64 | Homebrew, .pkg | ✅ Full support |
| Windows | x86_64 | .msi, .zip | ✅ Full support |

### Additional Targets (Tier 2)

| Platform | Architecture | Package Format | Status |
|----------|-------------|----------------|---------|
| Linux | ARM64 | .tar.gz | ✅ Binary only |
| Linux | ARMv7 | .tar.gz | ✅ Binary only |
| FreeBSD | x86_64 | .tar.gz | ⚠️ Best effort |

### Container Support

| Registry | Image | Size | Status |
|----------|-------|------|---------|
| Docker Hub | `batless/batless:latest` | ~8MB | ✅ Available |
| GHCR | `ghcr.io/owner/batless:latest` | ~8MB | ✅ Available |

---

## 📋 Installation Methods for Users

### 🦀 Rust Users (Primary)

```bash
# Install from crates.io
cargo install batless

# Install specific version
cargo install batless --version 0.1.0

# Install from git (development)
cargo install --git https://github.com/username/batless
```

### 🍺 macOS (Homebrew)

```bash
# Add our tap (one-time setup)
brew tap docdyhr/batless

# Install batless
brew install batless

# Or install directly without adding tap
brew install docdyhr/batless
```

### 🐧 Linux Package Managers

```bash
# Ubuntu/Debian (.deb)
wget https://github.com/username/batless/releases/latest/download/batless_0.1.0_amd64.deb
sudo dpkg -i batless_0.1.0_amd64.deb

# RHEL/Fedora (.rpm)
wget https://github.com/username/batless/releases/latest/download/batless-0.1.0-1.x86_64.rpm
sudo rpm -i batless-0.1.0-1.x86_64.rpm

# Arch Linux (AUR)
yay -S batless
# or
paru -S batless
```

### 🪟 Windows Package Managers

```powershell
# Chocolatey
choco install batless

# Scoop
scoop bucket add extras
scoop install batless

# winget
winget install batless
```

### 🐳 Docker

```bash
# Run batless in container
docker run --rm -v $(pwd):/workspace batless/batless /workspace/src/main.rs

# Use as base image
FROM batless/batless:alpine
COPY . /app
WORKDIR /app
RUN batless --mode=summary src/main.rs
```

---

## 🔐 Security & Integrity

### Binary Signing

```bash
# Sign release binaries (maintainers)
cargo dist sign --all

# Users can verify signatures
cosign verify-blob \
  --certificate-identity-regexp ".*" \
  --certificate-oidc-issuer "https://token.actions.githubusercontent.com" \
  --bundle batless-0.1.0.cosign.bundle \
  batless-x86_64-unknown-linux-gnu.tar.gz
```

### Checksums & SBOM

```bash
# Verify download integrity
sha256sum -c batless-0.1.0-checksums.txt

# Check software bill of materials
cat batless-0.1.0.spdx.json | jq '.packages[] | select(.name=="batless")'
```

### Supply Chain Security

- 🔒 **Reproducible builds** - deterministic compilation
- 📝 **SBOM generation** - complete dependency tracking
- 🔑 **Keyless signing** - GitHub OIDC with Sigstore
- 🛡️ **Audit pipeline** - cargo-audit on every build
- 🏷️ **Provenance** - SLSA Level 3 attestations

---

## 🧪 Testing Releases

### Local Testing

```bash
# Test debug build
cargo run -- src/main.rs

# Test release build
cargo build --release
./target/release/batless --version
./target/release/batless --help

# Test all platforms (requires Docker)
cargo dist build --all-targets
```

### Pre-Release Testing

```bash
# Test release process without publishing
cargo dist plan
cargo dist build
cargo dist check

# Test package installation
./scripts/test-packages.sh
```

### Integration Testing

```bash
# Test CLI behavior across platforms
cargo test --test integration_tests

# Test package managers (requires containers)
docker run -it ubuntu:latest bash -c "
  wget -O batless.deb https://github.com/.../batless_0.1.0_amd64.deb &&
  dpkg -i batless.deb &&
  batless --version
"
```

---

## 📊 Distribution Matrix

| Method | Primary Use Case | Pros | Cons |
|--------|-----------------|------|------|
| **cargo install** | Rust developers | Latest features, easy updates | Requires Rust toolchain |
| **GitHub Releases** | CI/CD, automation | No dependencies, all platforms | Manual updates |
| **Homebrew** | macOS users | System integration, auto-updates | macOS only |
| **Linux packages** | System admins | Package manager integration | Platform-specific |
| **Docker** | Containerized workflows | Isolated, reproducible | Container overhead |
| **Windows packages** | Windows users | System integration | Platform-specific |

## 🎯 Release Strategy

### Version Numbering

- **Major** (1.0.0): Breaking API changes, major features
- **Minor** (0.1.0): New features, backwards compatible
- **Patch** (0.0.1): Bug fixes, documentation

### Release Cadence

- 🚀 **Major releases**: Every 6-12 months
- ✨ **Minor releases**: Monthly or when features are ready
- 🐛 **Patch releases**: As needed for critical bugs
- 🔧 **Pre-releases**: Alpha/beta for testing major changes

### LTS Strategy

- **Current**: Latest stable with active development
- **Maintenance**: Previous major with security fixes
- **EOL**: Clear deprecation timeline

---

## 🔧 Maintenance Tasks

### Regular Maintenance

```bash
# Update dependencies (monthly)
cargo update
cargo audit
cargo outdated

# Update CI/CD (quarterly)
act -j test  # Test GitHub Actions locally
dependabot alerts  # Review security updates

# Performance benchmarks (per release)
cargo bench
hyperfine 'batless large-file.py' 'bat large-file.py'
```

### Security Response

```bash
# Emergency security release
cargo audit
cargo release patch --execute  # Immediate patch
./scripts/notify-security.sh    # Alert users
```

## 📚 References & Tools

### Core Tools

- 🚀 GitHub Actions - Cross-platform distribution and releases
- 🚀 [cargo-release](https://github.com/crate-ci/cargo-release) - Release automation
- 🔍 [cargo-audit](https://github.com/RustSec/rustsec/tree/main/cargo-audit) - Security auditing

### Package Managers

- 🍺 [Homebrew Tap Guide](https://docs.brew.sh/Taps)
- 🐧 [Debian Packaging](https://www.debian.org/doc/debian-policy/)
- 🪟 [Chocolatey Packages](https://docs.chocolatey.org/en-us/create/create-packages)

### Security & Compliance

- 🔑 [Sigstore](https://docs.sigstore.dev/) - Keyless code signing
- 📋 [SLSA](https://slsa.dev/) - Supply chain security framework
- 🛡️ [OSSF Scorecard](https://github.com/ossf/scorecard) - Security best practices

### CI/CD

- ⚡ [GitHub Actions](https://docs.github.com/en/actions)
- 🧪 [act](https://github.com/nektos/act) - Local GitHub Actions testing
- 🎭 [release-please](https://github.com/googleapis/release-please) - Automated releases
