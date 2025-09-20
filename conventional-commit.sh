#!/bin/bash

# Conventional Commit Helper Script
# This script helps create conventional commits easily

echo "Conventional Commit Helper"
echo "=========================="
echo ""
echo "Available commit types:"
echo "  feat:     A new feature"
echo "  fix:      A bug fix"
echo "  docs:     Documentation only changes"
echo "  style:    Changes that do not affect the meaning of the code"
echo "  refactor: A code change that neither fixes a bug nor adds a feature"
echo "  perf:     A code change that improves performance"
echo "  test:     Adding missing tests or correcting existing tests"
echo "  chore:    Changes to the build process or auxiliary tools"
echo "  ci:       Changes to CI configuration files and scripts"
echo "  build:    Changes that affect the build system or external dependencies"
echo "  revert:   Reverts a previous commit"
echo ""

# Get commit type
read -p "Enter commit type: " COMMIT_TYPE

# Validate commit type
VALID_TYPES=("feat" "fix" "docs" "style" "refactor" "perf" "test" "chore" "ci" "build" "revert")
if [[ ! " ${VALID_TYPES[@]} " =~ " ${COMMIT_TYPE} " ]]; then
    echo "Error: Invalid commit type '$COMMIT_TYPE'"
    echo "Valid types: ${VALID_TYPES[*]}"
    exit 1
fi

# Get scope (optional)
read -p "Enter scope (optional, press Enter to skip): " SCOPE

# Get description
read -p "Enter description: " DESCRIPTION

# Build commit message
if [ -n "$SCOPE" ]; then
    COMMIT_MSG="$COMMIT_TYPE($SCOPE): $DESCRIPTION"
else
    COMMIT_MSG="$COMMIT_TYPE: $DESCRIPTION"
fi

# Show the commit message
echo ""
echo "Commit message: $COMMIT_MSG"
echo ""

# Confirm and commit
read -p "Proceed with this commit? (y/N): " CONFIRM
if [[ $CONFIRM =~ ^[Yy]$ ]]; then
    git add .
    git commit -m "$COMMIT_MSG"
    echo "Commit created successfully!"
else
    echo "Commit cancelled."
fi