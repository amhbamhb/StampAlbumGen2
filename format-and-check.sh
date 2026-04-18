#!/bin/bash
# Format code and fix common issues before committing

set -e  # Exit on any error

# Default timeout for running the test-suite (seconds). Can be overridden by
# setting TEST_TIMEOUT in the environment when invoking the script.
TEST_TIMEOUT=${TEST_TIMEOUT:-600}

echo "🔎 Verifying config baseline files..."
if [ ! -f "config_template.toml" ]; then
    echo "❌ Missing config_template.toml in repo root."
    echo "Please restore it before running checks."
    exit 1
fi

if [ ! -f "config.toml" ]; then
    echo "🛠️  config.toml missing; creating it from config_template.toml"
    cp "config_template.toml" "config.toml"
fi

echo "✅ Config baseline files present"
echo ""

echo "� Checking current formatting status..."

# First check if formatting is needed
if cargo fmt --all -- --check > /dev/null 2>&1; then
    echo "✅ Code is already properly formatted!"
else
    echo "🔧 Formatting Rust code..."
    
    # Show what files need formatting
    echo "Files that need formatting:"
    cargo fmt --all -- --check 2>&1 | grep "Diff in" | sed 's/Diff in /  • /' || true
    echo ""
    
    # Actually format the code
    cargo fmt --all
    echo "✅ Code has been formatted"
fi

echo ""
echo "🔍 Verifying formatting is now correct..."
if ! cargo fmt --all -- --check; then
    echo "❌ Formatting verification failed!"
    echo "There may be syntax errors preventing proper formatting."
    exit 1
fi

echo "✅ Formatting verification passed"
echo ""

echo "📋 Running a quick compilation check..."
if ! cargo check --quiet; then
    echo "❌ Code doesn't compile!"
    echo "Please fix compilation errors before committing."
    exit 1
fi

echo "✅ Code compiles successfully"
echo ""
echo "🧪 Running test-suite (this may take a while)..."

# Prefer GNU `timeout` to bound the total runtime so developers don't get
# stuck on a single hanging test during a pre-commit check. If `timeout` is
# not available, fall back to running tests directly.
if command -v timeout >/dev/null 2>&1; then
    echo "⏱️  Running tests with timeout=${TEST_TIMEOUT}s (set TEST_TIMEOUT to override)"
    if ! timeout --preserve-status ${TEST_TIMEOUT}s cargo test --all; then
        echo "❌ Tests failed or timed out. Inspect the output above."
        echo "If a specific test is hanging, run it individually to debug, e.g."
        echo "  cargo test test_cli_special_characters_and_edge_cases --test cli_option_validation -- --nocapture"
        exit 1
    fi
else
    echo "⚠️  'timeout' command not found; running tests without a hard timeout."
    if ! cargo test --all; then
        echo "❌ Some tests failed. Please fix failures before committing."
        exit 1
    fi
fi

echo "✅ All tests passed"
echo ""
echo "🎯 Ready to commit! Your code is properly formatted and compiles."
echo ""
echo "Next steps:"
echo "  git add ."
echo "  git commit -m 'your commit message'"
echo "  git push"