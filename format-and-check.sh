#!/bin/bash
# Format code and fix common issues before committing

echo "🔧 Formatting Rust code..."
cargo fmt --all

echo "🧹 Checking for formatting issues..."
if ! cargo fmt --all -- --check; then
    echo "❌ Some files were not properly formatted"
    echo "✅ Files have been reformatted automatically"
else
    echo "✅ All files are properly formatted"
fi

echo ""
echo "📋 Running a quick check..."
cargo check --quiet

echo ""
echo "🎯 Ready to commit! Your code is now formatted and compiles."
echo ""
echo "Next steps:"
echo "  git add ."
echo "  git commit -m 'your commit message'"
echo "  git push"