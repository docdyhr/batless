# Publishing Guide

How a batless release gets from `main` to crates.io, GitHub Releases, and Homebrew. Everything after the release PR is automated; this guide covers the one manual step and what to do when the automation needs a nudge.

## Distribution channels

| Channel | Install command | Updated by |
|---|---|---|
| crates.io | `cargo install batless` | `publish-crate` job (OIDC trusted publishing) |
| GitHub Releases | download from [Releases](https://github.com/docdyhr/batless/releases) | `create-release` job |
| Homebrew | `brew install docdyhr/tap/batless` | `update-homebrew` job → [`docdyhr/homebrew-tap`](https://github.com/docdyhr/homebrew-tap) |

GitHub Releases ship binaries for `x86_64-unknown-linux-gnu`, `x86_64-unknown-linux-musl`, `x86_64-apple-darwin`, `aarch64-apple-darwin`, and `x86_64-pc-windows-msvc`, each with a build provenance attestation.

## Cutting a release

1. **Open a release PR** from a `release/X.Y.Z` branch that:
   - bumps `version` in `Cargo.toml` and refreshes `Cargo.lock` (`cargo update --workspace`)
   - moves the `## [Unreleased]` content of `CHANGELOG.md` into a dated `## [X.Y.Z] - YYYY-MM-DD` section and leaves an empty `## [Unreleased]` above it — the GitHub release notes are extracted from this section
2. **Title the PR exactly `release X.Y.Z`** (e.g. `release 0.7.1`). The `tag-after-merge` job in `release-consolidated.yml` extracts the version with `release[[:space:]]*([0-9]+\.[0-9]+\.[0-9]+)`, so `chore(release): X.Y.Z` passes the job's trigger condition but then fails to find a version.
3. **Merge the PR** (squash). `tag-after-merge` pushes an annotated tag `vX.Y.Z`.
4. **Start the release pipeline.** The tag is pushed with `GITHUB_TOKEN`, and GitHub does not start other workflows from events created by `GITHUB_TOKEN`, so the `push: tags` trigger doesn't fire. Dispatch it against the **tag**:

   ```bash
   gh workflow run "Release Management" --repo docdyhr/batless --ref vX.Y.Z -f tag=vX.Y.Z
   ```

   Use `--ref vX.Y.Z`, not `--ref main`. `publish-crate` and `update-homebrew` only run when `github.ref` starts with `refs/tags/v`; dispatching against `main` gives a green run that silently skips both.

The pipeline (`release-consolidated.yml` calling the shared `docdyhr/.github` `rust-release.yml`) then runs: validate → test (Linux, macOS, Windows) → build and attest 5 targets → create GitHub release → publish to crates.io → update the Homebrew tap. It takes about 6 minutes.

## Verifying a release

```bash
# crates.io — the API returns an empty object without a User-Agent header
curl -s -A "batless-release-check/1.0" https://crates.io/api/v1/crates/batless/X.Y.Z \
  | jq '.version | {num, rust_version, yanked, run: .trustpub_data.run_id}'

# GitHub release and its 5 archives
gh release view vX.Y.Z --repo docdyhr/batless

# Homebrew formula version and checksums
gh api repos/docdyhr/homebrew-tap/contents/Formula/batless.rb --jq .content | base64 -d | grep -E 'version|sha256'
```

`trustpub_data.run_id` should match the release run's ID, and the formula's `sha256` values should match `shasum -a 256` of the downloaded archives.

## Credentials

- **crates.io** uses [trusted publishing](https://rust-lang.github.io/rfcs/3691-trusted-publishing-cratesio.html) via [`rust-lang/crates-io-auth-action`](https://github.com/rust-lang/crates-io-auth-action) — no API token is stored. The crate has two trusted publishers configured on crates.io: `release-consolidated.yml` and `publish-crate.yml`. A new publishing workflow needs its own entry there, or the publish fails with a 403.
- **Homebrew** uses the `HOMEBREW_TAP_TOKEN` repository secret. `.github/scripts/update_homebrew_formula.py` writes `Formula/batless.rb` through the GitHub Contents API, so the token only needs write access to `docdyhr/homebrew-tap`'s contents. A fine-grained token scoped to that one repository with **Contents: Read and write** is enough. When it expires, the "Update Homebrew Tap" job fails and the other channels are unaffected. Rotate the secret, then re-run `update-homebrew.yml` for the tag.

## Re-running a single step

If one channel failed and the rest succeeded, re-run only that step (both take a `tag` input):

```bash
gh workflow run publish-crate.yml   --repo docdyhr/batless -f tag=vX.Y.Z   # crates.io only
gh workflow run update-homebrew.yml --repo docdyhr/batless -f tag=vX.Y.Z   # Homebrew tap only
```

Re-dispatching the whole "Release Management" workflow against the tag is also safe: the GitHub release is updated in place rather than duplicated, and the publish step skips a version that's already on crates.io. A published version can't be replaced. If one is broken, yank it (`cargo yank --version X.Y.Z`) and release a new patch version.
