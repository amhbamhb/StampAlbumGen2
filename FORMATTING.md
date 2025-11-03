# Code Formatting Guide

This project now includes automated code formatting to prevent CI failures.

## Quick Start

### Before Every Commit (Automatic)

The pre-commit hook will automatically run `cargo fmt` before each commit. If formatting changes are needed, the commit will be blocked and you'll need to:

1. Review the formatting changes
2. Run `git add .` to stage them
3. Commit again

### Manual Formatting

```bash
# Format all code
./format-and-check.sh

# Or just format without extra checks
cargo fmt --all
```

### Checking Format (CI-style)

```bash
# Check if code is properly formatted (without changing files)
cargo fmt --all -- --check
```

## Files Created

- **`.git/hooks/pre-commit`** - Automatic formatting before commits
- **`format-and-check.sh`** - Manual formatting script with helpful output
- **`.editorconfig`** - Editor configuration for consistent style
- **Updated `copy-release.sh`** - Now includes formatting step

## Troubleshooting

### Pre-commit Hook Not Working?

```bash
# Make sure it's executable
chmod +x .git/hooks/pre-commit
```

### Skip Pre-commit Hook (Emergency)

```bash
git commit --no-verify -m "your message"
```

### CI Still Failing on Format?

1. Run `cargo fmt --all`
2. Check `git status` for changes
3. Commit and push the formatting fixes

## What Gets Formatted

- All Rust source files (`.rs`)
- Ensures files end with newlines
- Removes trailing whitespace
- Consistent indentation (4 spaces for Rust)

This setup should prevent all formatting-related CI failures going forward!
