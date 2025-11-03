#!/bin/bash
# Format code and fix common issues before committing

set -e  # Exit on any error

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
echo "🎯 Ready to commit! Your code is properly formatted and compiles."
echo ""
echo "Next steps:"
echo "  git add ."
echo "  git commit -m 'your commit message'"
echo "  git push"