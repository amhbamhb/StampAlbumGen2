#!/bin/bash
# Documentation Versioning Script
# Usage: ./version_docs.sh [major|minor|patch]
#        ./version_docs.sh auto-patch    # run build and bump patch on success (CI-friendly)
# Options:
#   --yes | -y        : do not prompt, assume yes
#   --help | -h       : show help
# This script automatically updates version numbers across all project files

set -e  # Exit on any error

VERSION_TYPE=${1:-minor}
DATE=$(date +%Y-%m-%d)

# CI/auto mode: if first argument is 'auto-patch' then run build and only on
# successful build bump the patch number automatically (non-interactive).
AUTO_PATCH=false
NON_INTERACTIVE=false
for arg in "$@"; do
    case "$arg" in
        auto-patch)
            AUTO_PATCH=true
            ;;
        --git)
            DO_GIT=true
            ;;
        --push)
            DO_PUSH=true
            ;;
        --yes|-y)
            NON_INTERACTIVE=true
            ;;
        --help|-h)
            ;;
    esac
done

# Default values
DO_GIT=${DO_GIT:-false}
DO_PUSH=${DO_PUSH:-false}

# Function to increment version
increment_version() {
    local current=$1
    local type=$2

    # Extract major, minor, patch from version string (e.g., "0.5.0" -> 0 5 0)
    local major=$(echo $current | cut -d. -f1)
    local minor=$(echo $current | cut -d. -f2)
    local patch=$(echo $current | cut -d. -f3)

    case $type in
        "major")
            echo "$((major + 1)).0.0"
            ;;
        "minor")
            echo "$major.$((minor + 1)).0"
            ;;
        "patch")
            echo "$major.$minor.$((patch + 1))"
            ;;
        *)
            echo "Error: Invalid version type. Use 'major', 'minor', or 'patch'"
            exit 1
            ;;
    esac
}

# Function to get current version from Cargo.toml
get_current_version() {
    grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/'
}

# Function to update Cargo.toml version
update_cargo_toml() {
    local new_version=$1
    echo "Updating Cargo.toml version to $new_version..."
    sed -i "s/^version = \".*\"/version = \"$new_version\"/" Cargo.toml
}

# Function to update instructions file
update_instructions() {
    local new_version=$1
    local date=$2
    echo "Updating .github/instructions version to $new_version..."

    # Update version number
    sed -i "s/^# Version: .*/# Version: $new_version/" .github/instructions

    # Update last updated date
    sed -i "s/^# Last Updated: .*/# Last Updated: $date/" .github/instructions
}

# Function to update README.md
update_readme() {
    local new_version=$1
    local date=$2
    echo "Updating README.md version to $new_version..."

    # Update title version
    sed -i "s/^# Stamp Album Generator v.*/# Stamp Album Generator v$new_version/" README.md

    # Update HTML comment version and date
    sed -i "s/<!-- Version: .* | Last Updated: .* | Previous:/<!-- Version: $new_version | Last Updated: $date | Previous:/" README.md

    # Update footer version
    sed -i "s/^\*\*Version\*\*: .*/**Version**: $new_version/" README.md

    # Update footer release date
    sed -i "s/^\*\*Release Date\*\*: .*/**Release Date**: $date/" README.md
}

# Function to update RELEASE_NOTES.md
update_release_notes() {
    local new_version=$1
    local date=$2
    echo "Updating RELEASE_NOTES.md version to $new_version..."

    # Update title version
    sed -i "s/^# Stamp Album Generator v.*/# Stamp Album Generator v$new_version Release Notes/" RELEASE_NOTES.md

    # Update footer version
    sed -i "s/^\*\*Version\*\*: .*/**Version**: $new_version/" RELEASE_NOTES.md

    # Update footer release date
    sed -i "s/^\*\*Release Date\*\*: .*/**Release Date**: $date/" RELEASE_NOTES.md
}

# Main execution
echo "=== Stamp Album Generator Version Update Script ==="

# Handle help
if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    echo "Usage: $0 [major|minor|patch]"
    echo "       $0 auto-patch [--yes]   # run build and bump patch on success (CI-friendly)"
    echo ""
    echo "Arguments:"
    echo "  major    Increment major version (X.0.0)"
    echo "  minor    Increment minor version (x.X.0) [default]"
    echo "  patch    Increment patch version (x.x.X)"
    echo "  auto-patch Run 'cargo build' and on success increment patch (non-interactive with --yes)"
    echo ""
    echo "Options:" 
    echo "  --yes, -y   Run non-interactively (assume yes)"
    echo "  --git       Create a git commit and annotated tag for the new version (local)"
    echo "  --push      When used with --git, push commit and tag to origin"
    echo "  --help, -h  Show this help"
    echo ""
    echo "Examples:"
    echo "  $0                # minor version bump (default)"
    echo "  $0 minor          # minor version bump"
    echo "  $0 patch          # patch version bump"
    echo "  $0 auto-patch -y  # run build and bump patch on success without prompting"
    echo "  $0 patch --git --push -y  # bump patch, create commit+tag, and push (CI-friendly)"
    exit 0
fi

VERSION_TYPE=${1:-minor}
DATE=$(date +%Y-%m-%d)

echo "Version type: $VERSION_TYPE"
echo "Date: $DATE"
echo ""

# If auto-patch mode requested: run a build and only proceed if build passes
if [ "$AUTO_PATCH" = true ]; then
    echo "AUTO-PATCH mode: running 'cargo build' to verify build succeeds before bumping patch..."
    if ! cargo build --quiet; then
        echo "Build failed — aborting auto-patch. Fix build errors then retry."
        exit 2
    fi
    echo "Build successful. Auto-patch will increment patch version."
    VERSION_TYPE=patch
fi

# Get current version
CURRENT_VERSION=$(get_current_version)
echo "Current version: $CURRENT_VERSION"

# Calculate new version
NEW_VERSION=$(increment_version "$CURRENT_VERSION" "$VERSION_TYPE")
echo "New version: $NEW_VERSION"
echo ""

# Confirm with user unless non-interactive
if [ "$NON_INTERACTIVE" = false ]; then
    read -p "Proceed with version update? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Version update cancelled."
        exit 0
    fi
else
    echo "Non-interactive mode: proceeding without confirmation."
fi

# Create backups
echo ""
echo "Creating backups..."
mkdir -p docs/versions
cp .github/instructions "docs/versions/instructions_${DATE}_backup.md"
cp README.md "docs/versions/README_${DATE}_backup.md"
cp RELEASE_NOTES.md "docs/versions/RELEASE_NOTES_${DATE}_backup.md"
cp Cargo.toml "docs/versions/Cargo_${DATE}_backup.toml"
echo "Backups created in docs/versions/"
echo ""

# Update all files
echo "Updating version numbers..."
update_cargo_toml "$NEW_VERSION"
update_instructions "$NEW_VERSION" "$DATE"
update_readme "$NEW_VERSION" "$DATE"
update_release_notes "$NEW_VERSION" "$DATE"

# Regenerate CLI_USAGE.md from the built binary so docs reflect latest CLI
generate_cli_docs() {
    echo "Regenerating CLI_USAGE.md from binary help output..."
    if command -v cargo >/dev/null 2>&1; then
        # Prefer building quietly then running help to avoid noisy output in CI
        if cargo build --quiet; then
            cargo run -- --help > docs/CLI_USAGE.md || echo "Warning: failed to run binary to generate CLI docs"
        else
            echo "Warning: cargo build failed — skipping CLI docs regeneration"
        fi
    else
        echo "cargo not available in PATH — skipping CLI docs regeneration"
    fi
}

generate_cli_docs

echo ""
echo "=== Version Update Complete ==="
echo "Updated to version: $NEW_VERSION"
echo "Updated files:"
echo "  - Cargo.toml"
echo "  - .github/instructions"
echo "  - README.md"
echo "  - RELEASE_NOTES.md"
echo ""
echo "Next steps:"
echo "1. Review the changes and make any necessary documentation updates"
echo "2. Test the build: cargo build"
echo "3. Test the version: cargo run -- --version"
echo "4. Commit the changes: git add -A && git commit -m 'Bump version to $NEW_VERSION'"
echo "5. Create a git tag: git tag v$NEW_VERSION"
echo "6. Push changes: git push && git push --tags"
echo ""
echo "=== Version Consistency Check ==="
echo "Verifying all files have consistent version numbers..."

# Check versions in all files
CARGO_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
INSTRUCTIONS_VERSION=$(grep '^# Version: ' .github/instructions | head -1 | sed 's/# Version: \(.*\)/\1/')
README_TITLE_VERSION=$(grep '^# Stamp Album Generator v' README.md | head -1 | sed 's/# Stamp Album Generator v\(.*\)/\1/')
README_FOOTER_VERSION=$(grep '^\*\*Version\*\*: ' README.md | head -1 | sed 's/\*\*Version\*\*: \(.*\)/\1/')
RELEASE_NOTES_VERSION=$(grep '^\*\*Version\*\*: ' RELEASE_NOTES.md | head -1 | sed 's/\*\*Version\*\*: \(.*\)/\1/')

echo "Cargo.toml: $CARGO_VERSION"
echo "Instructions: $INSTRUCTIONS_VERSION"
echo "README title: $README_TITLE_VERSION"
echo "README footer: $README_FOOTER_VERSION"
echo "Release Notes: $RELEASE_NOTES_VERSION"

# Check if all versions match
if [ "$CARGO_VERSION" = "$INSTRUCTIONS_VERSION" ] && \
   [ "$CARGO_VERSION" = "$README_TITLE_VERSION" ] && \
   [ "$CARGO_VERSION" = "$README_FOOTER_VERSION" ] && \
   [ "$CARGO_VERSION" = "$RELEASE_NOTES_VERSION" ]; then
    echo "✅ All versions are consistent: $CARGO_VERSION"
else
    echo "❌ Version mismatch detected!"
    echo "Please check the files and ensure all versions match."
    exit 1
fi

# If requested, create a git commit and tag the new version
if [ "$DO_GIT" = true ]; then
    echo "Preparing git commit and tag for v$NEW_VERSION"

    # Ensure working tree is clean unless NON_INTERACTIVE forces it
    if [ -n "$(git status --porcelain)" ] && [ "$NON_INTERACTIVE" = false ]; then
        echo "Working tree is dirty. Commit or stash changes before running with --git, or run with --yes to proceed." 
        exit 2
    fi

    # Stage changed files
    git add Cargo.toml .github/instructions README.md RELEASE_NOTES.md || true

    # Commit
    git commit -m "chore(release): bump version to $NEW_VERSION" --no-verify || echo "No changes to commit"

    # Create tag if it doesn't exist
    TAG_NAME="v$NEW_VERSION"
    if git rev-parse "$TAG_NAME" >/dev/null 2>&1; then
        echo "Tag $TAG_NAME already exists — skipping tag creation"
    else
        git tag -a "$TAG_NAME" -m "Release $TAG_NAME"
        echo "Created tag $TAG_NAME"
    fi

    if [ "$DO_PUSH" = true ]; then
        echo "Pushing commit and tag to origin..."
        git push --follow-tags || git push && git push --tags
    fi
fi