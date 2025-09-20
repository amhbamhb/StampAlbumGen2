#!/bin/bash
# Documentation Versioning Script
# Usage: ./version_docs.sh [major|minor]

VERSION_TYPE=${1:-minor}
DATE=$(date +%Y-%m-%d)

# Function to increment version
increment_version() {
    local current=$1
    local type=$2

    if [ "$type" = "major" ]; then
        echo "$((current + 1)).0"
    else
        local major=$(echo $current | cut -d. -f1)
        local minor=$(echo $current | cut -d. -f2)
        echo "$major.$((minor + 1))"
    fi
}

# Backup current versions
echo "Creating backups..."
cp .github/instructions docs/versions/instructions_${DATE}_backup.md
cp README.md docs/versions/README_${DATE}_backup.md

echo "Documentation backups created in docs/versions/"
echo "Next steps:"
echo "1. Make your documentation changes"
echo "2. Update version headers in both files"
echo "3. Update the versioning guide if needed"