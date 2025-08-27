---
name: Documentation Mismatch - Resolved
about: Documentation issue that has been identified and fixed
title: '[RESOLVED] CLI Documentation Mismatch with CLAUDE.md'
labels: 'documentation, resolved'
assignees: 'docdyhr'

---

## Issue Summary

Documentation in CLAUDE.md described non-existent CLI commands, causing AI assistants and users to encounter failures when following the documented protocol.

## What Was Wrong

The following commands were documented but didn't exist:

- `batless --pattern "search"` - Pattern search functionality
- `batless -r START:END file` - Line range selection
- `batless --list *.py` - File globbing

## Resolution

✅ **Fixed on August 15, 2025**

1. Updated CLAUDE.md to reflect actual CLI interface
2. Added "Non-Goals" section to README clarifying what batless doesn't do
3. Created decision matrix for requested features
4. Added fallback command suggestions for unsupported features

## Prevention Measures

- Created `scripts/validate-docs.sh` to test all documented examples
- Added CI workflow to catch documentation drift automatically
- Will run on every PR and push affecting markdown files

## Lessons Learned

- Documentation drift is a real risk for solo developers
- Automated validation is essential for maintaining accuracy
- Being explicit about non-features is as important as documenting features

## Status

This issue is closed as the documentation has been corrected and preventive measures are in place.
