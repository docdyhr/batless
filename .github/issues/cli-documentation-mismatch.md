# CLI Documentation Mismatch - Resolved

## Issue Summary

Documentation in CLAUDE.md described non-existent CLI commands, causing AI assistants to fail when using batless.

## What Happened

- AI assistant (Claude) attempted to use commands from CLAUDE.md that don't exist
- Commands like `--pattern`, `-r START:END`, `--list *.py` were documented but never implemented
- User had to fall back to standard Unix tools (cat, grep, sed)

## Resolution (Completed)

✅ Updated CLAUDE.md with correct usage patterns
✅ Added "Non-Goals" section to README
✅ Clarified what batless intentionally doesn't do
✅ Provided fallback command alternatives

## Lessons Learned

- Documentation drift is a real issue for solo developers
- Need automated documentation validation in CI
- Being explicit about non-features is as important as documenting features

## Follow-up Actions

- [ ] Add `validate-docs.sh` to CI pipeline
- [ ] Consider adding `--lines=START:END` for range selection (fits streaming model)
- [ ] Write brief philosophy doc explaining design decisions

Closing as resolved - documentation now matches implementation.
