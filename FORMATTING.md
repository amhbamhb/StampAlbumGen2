# Code Formatting Guide

This project includes automated code formatting to prevent CI failures with **strict enforcement**.

## How It Works

### 🛡️ Triple Protection System

1. **Pre-commit Hook**: Blocks commits with formatting issues
2. **Pre-push Hook**: Blocks pushes with formatting/compilation issues  
3. **CI Pipeline**: Final server-side formatting verification

### ✅ Before Every Commit (Automatic)

The pre-commit hook checks formatting **without modifying files**. If formatting is needed:

1. Commit is blocked with clear error messages
2. Run `./format-and-check.sh` to fix issues
3. Add and commit the formatted changes

### ✅ Before Every Push (Automatic)

The pre-push hook does final verification:
- Formatting check
- Compilation check
- Blocks push if any issues found

## Manual Commands

### Safe Formatting Script
```bash
# Recommended: Format and verify everything
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
