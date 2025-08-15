# Release Notes Template: v0.2.4 (Released 2025-08-15)

## Summary

Concise overview of the release: focus on documentation improvements, performance tooling, and JSON output enhancements.

## Highlights

- Pretty JSON output flag (`--json-pretty`) for human-friendly formatting
- Architecture documentation (`docs/ARCHITECTURE.md`)
- Performance guard documentation (`docs/PERFORMANCE_GUARD.md`) and enhanced workflow
- Wizard UX improvement (profile count & last updated timestamp)
- Refined performance claim (<5ms typical startup render)

## Breaking Changes

- None

## New Features

- Add `--json-pretty` CLI flag (and config option) for formatted JSON output
- Extended wizard listing output with metadata

## Improvements

- Performance regression guard: JSON baseline & PR comment integration
- Architecture overview documentation
- README links to architecture and performance docs

## Bug Fixes

- (List any fixes or write `N/A` if none)

## Performance

- Updated benchmarks show median startup under 5ms in baseline environment
- Guard thresholds: warn ≥15%, fail ≥25%

## Documentation

- New `docs/ARCHITECTURE.md`
- New `docs/PERFORMANCE_GUARD.md`
- README performance claim updated
- Added JSON formatting examples

## Upgrade Notes

- No action required; optional use of `--json-pretty` for readability

## Contributors

- @docdyhr (and any others)

## Checks

- [ ] All tests passing
- [ ] Performance guard status: pass
- [ ] CHANGELOG updated
- [ ] Version bumped in `Cargo.toml` (when releasing)
- [ ] Tag annotated
- [ ] Crates.io publish dry run

---

Copy this template into the GitHub Release UI when tagging v0.2.4 and fill in any TBD sections.
