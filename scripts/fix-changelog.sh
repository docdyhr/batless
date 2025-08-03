#!/bin/bash
# Script to fix the duplicated changelog entries

# Create a clean changelog with only the v0.2.2 entry and existing v0.2.1+ entries
cat > CHANGELOG.md << 'EOF'
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

# Changelog

## [0.2.2] - 2025-08-03

### Added

- Complete cat replacement functionality with `-n/--number` and `-b/--number-nonblank` flags
- Exact compatibility with system cat line numbering format (6-character right-aligned + tab)
- Perfect newline handling to match cat/less output exactly

### Fixed

- Critical newline bug that was causing shell "%" indicators on incomplete lines
- Test failures in CI/CD pipeline (test_max_lines_limit)
- Clippy warnings for format strings and conditional logic
- Truncation message formatting for better test compatibility

### Changed

- Standardized all output to use `println!("{formatted_output}")` for consistent newlines
- Modernized format strings to resolve clippy uninlined-format-args warnings
- Simplified conditional logic in main.rs to eliminate clippy if-same-then-else warnings
- Enhanced CI/CD pipeline reliability with comprehensive bug fixes

EOF

# Extract everything from the original changelog starting from v0.2.1
sed -n '/^## \[0\.2\.1\]/,$p' CHANGELOG.md.bak >> CHANGELOG.md 2>/dev/null || \
  sed -n '/^## \[0\.2\.1\]/,$p' /Users/thomas/Programming/batless/CHANGELOG.md >> CHANGELOG.md

echo "Changelog fixed!"
