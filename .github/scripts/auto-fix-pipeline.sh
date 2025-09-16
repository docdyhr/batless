#!/bin/bash
set -euo pipefail

echo "🔧 Auto-Fix Pipeline Script for Batless"
echo "========================================="
echo ""

# Function to run cargo commands
run_cargo_fix() {
    echo "🦀 Running Rust-specific fixes..."

    # Fix formatting
    if command -v cargo &> /dev/null; then
        echo "  Formatting code..."
        cargo fmt --all 2>/dev/null || true

        echo "  Running clippy fixes..."
        cargo clippy --fix --allow-dirty --allow-staged 2>/dev/null || true

        echo "  Updating dependencies..."
        cargo update 2>/dev/null || true

        echo "  Building to verify..."
        cargo build --release 2>/dev/null || true
    fi
}

# Function to check and fix test issues
fix_tests() {
    echo "🧪 Checking and fixing tests..."

    if cargo test --no-run 2>&1 | grep -q "error"; then
        echo "  Test compilation errors detected, attempting fixes..."
        cargo fix --allow-dirty --allow-staged 2>/dev/null || true
    fi

    # Run tests to see if they pass
    if ! cargo test --quiet 2>/dev/null; then
        echo "  ⚠️ Some tests are failing. Manual intervention may be required."
    else
        echo "  ✅ All tests passing"
    fi
}

# Function to optimize workflow files
optimize_workflows() {
    echo "📝 Optimizing workflow files..."

    # Check for workflow issues
    for workflow in .github/workflows/*.yml; do
        if [ -f "$workflow" ]; then
            basename=$(basename "$workflow")

            # Check for timeout issues
            if ! grep -q "timeout-minutes:" "$workflow"; then
                echo "  ⚠️ $basename: Missing timeout-minutes configuration"
            fi

            # Check for proper caching
            if ! grep -q "actions/cache" "$workflow" && grep -q "cargo\|rust" "$workflow"; then
                echo "  ⚠️ $basename: Could benefit from caching"
            fi
        fi
    done
}

# Function to fix common CI issues
fix_ci_issues() {
    echo "🔍 Checking for common CI issues..."

    # Check Cargo.lock
    if [ -f "Cargo.toml" ] && [ ! -f "Cargo.lock" ]; then
        echo "  Generating Cargo.lock..."
        cargo generate-lockfile
    fi

    # Check for large files
    large_files=$(find . -type f -size +1M -not -path "./.git/*" -not -path "./target/*" 2>/dev/null | head -5)
    if [ -n "$large_files" ]; then
        echo "  ⚠️ Large files detected that might slow CI:"
        echo "$large_files" | while read file; do
            size=$(du -h "$file" | cut -f1)
            echo "    - $file ($size)"
        done
    fi
}

# Main execution
echo "Starting auto-fix process..."
echo ""

run_cargo_fix
fix_tests
optimize_workflows
fix_ci_issues

echo ""
echo "🏁 Auto-fix complete!"
echo ""
echo "Summary:"
echo "--------"

# Check current status
if cargo build --release >/dev/null 2>&1; then
    echo "✅ Build: SUCCESS"
else
    echo "❌ Build: FAILED"
fi

if cargo test --quiet >/dev/null 2>&1; then
    echo "✅ Tests: PASSING"
else
    echo "❌ Tests: FAILING"
fi

if cargo clippy -- -D warnings >/dev/null 2>&1; then
    echo "✅ Clippy: CLEAN"
else
    echo "⚠️ Clippy: WARNINGS"
fi

echo ""
echo "Next steps:"
echo "-----------"
echo "1. Review any changes made by the auto-fix script"
echo "2. Run: git diff to see modifications"
echo "3. If satisfied, commit changes"
echo "4. Push to trigger CI/CD pipeline"

exit 0
