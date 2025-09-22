#!/bin/bash

# Conventional Commit Helper Script
# This script helps create conventional commits easily

echo "Conventional Commit Helper"
echo "=========================="
echo

# Parse conventional commit types from docs/AUTOMATION_SETUP.md
DOC_FILE="docs/AUTOMATION_SETUP.md"
if [ ! -f "$DOC_FILE" ]; then
    echo "Warning: $DOC_FILE not found — falling back to built-in types"
    TYPES=("feat" "fix" "docs" "style" "refactor" "perf" "test" "chore" "ci" "build" "revert" "BREAKING CHANGE")
    BUMPS=("minor" "patch" "patch" "patch" "patch" "patch" "patch" "patch" "patch" "patch" "patch" "major")
    DESCS=("New features" "Bug fixes" "Documentation changes" "Code style changes" "Code refactoring" "Performance improvements" "Test additions/changes" "Maintenance tasks" "CI/CD changes" "Build system changes" "Commit reversions" "Breaking change")
else
    TYPES=()
    BUMPS=()
    DESCS=()
    while IFS= read -r line; do
        # Process Markdown table rows that start with '|'
        if [[ $line == \|* ]]; then
            # split columns using awk to avoid complex regex and backtick parsing
            col2=$(printf "%s" "$line" | awk -F'|' '{print $2}')
            col3=$(printf "%s" "$line" | awk -F'|' '{print $3}')
            col4=$(printf "%s" "$line" | awk -F'|' '{print $4}')
            # trim whitespace
            type_raw=$(echo "$col2" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')
            bump_raw=$(echo "$col3" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')
            desc_raw=$(echo "$col4" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')
            # remove surrounding backticks and trailing colon from type
            type=$(echo "$type_raw" | sed 's/^`//;s/`$//;s/:$//')
            # skip header or separator rows like 'Type' or '------'
            lower_type=$(echo "$type" | tr '[:upper:]' '[:lower:]')
            if [ -z "$type" ] || [ "$lower_type" = "type" ] || [[ "$type" =~ ^-+$ ]]; then
                continue
            fi
            bump=$(echo "$bump_raw" | xargs)
            desc=$(echo "$desc_raw" | xargs)
            if [ -n "$type" ]; then
                TYPES+=("$type")
                BUMPS+=("$bump")
                DESCS+=("$desc")
            fi
        fi
    done < "$DOC_FILE"

    # If parsing produced nothing, fall back to built-in list
    if [ ${#TYPES[@]} -eq 0 ]; then
        echo "Warning: No table entries parsed from $DOC_FILE — using defaults"
        TYPES=("feat" "fix" "docs" "style" "refactor" "perf" "test" "chore" "ci" "build" "revert" "BREAKING CHANGE")
        BUMPS=("minor" "patch" "patch" "patch" "patch" "patch" "patch" "patch" "patch" "patch" "patch" "major")
        DESCS=("New features" "Bug fixes" "Documentation changes" "Code style changes" "Code refactoring" "Performance improvements" "Test additions/changes" "Maintenance tasks" "CI/CD changes" "Build system changes" "Commit reversions" "Breaking change")
    fi
fi

echo "Select commit type (choose the number):"
for i in "${!TYPES[@]}"; do
    idx=$((i+1))
    printf "  %2d) %-18s %-8s %s\n" "$idx" "${TYPES[i]}" "${BUMPS[i]}" "${DESCS[i]}"
done

read -p "Choice: " CHOICE
if ! [[ "$CHOICE" =~ ^[0-9]+$ ]] || [ "$CHOICE" -lt 1 ] || [ "$CHOICE" -gt ${#TYPES[@]} ]; then
    echo "Invalid choice"; exit 1
fi
INDEX=$((CHOICE-1))
COMMIT_TYPE_FULL="${TYPES[INDEX]}"
COMMIT_BUMP="${BUMPS[INDEX]}"

# Normalize type for header (remove trailing colon if present)
COMMIT_TYPE_HEADER="${COMMIT_TYPE_FULL%%:*}"

# Get scope (optional)
read -p "Enter scope (optional, press Enter to skip): " SCOPE

# Get description
read -p "Enter short description: " DESCRIPTION

# Build commit message
if [[ "$COMMIT_TYPE_FULL" == "BREAKING CHANGE" ]]; then
    COMMIT_MSG="BREAKING CHANGE: $DESCRIPTION"
else
    if [ -n "$SCOPE" ]; then
        COMMIT_MSG="$COMMIT_TYPE_HEADER($SCOPE): $DESCRIPTION"
    else
        COMMIT_MSG="$COMMIT_TYPE_HEADER: $DESCRIPTION"
    fi
fi

echo
echo "Commit message: $COMMIT_MSG"
echo
read -p "Proceed with this commit? (y/N): " CONFIRM
if [[ $CONFIRM =~ ^[Yy]$ ]]; then
    git add .
    git commit -m "$COMMIT_MSG"
    echo "Commit created successfully!"
    # Offer to run version bump
    read -p "Run version bump now (type: $COMMIT_BUMP)? (y/N): " DO_BUMP
    if [[ $DO_BUMP =~ ^[Yy]$ ]]; then
        if [ -x "./version_docs.sh" ]; then
            echo "Running version_docs.sh $COMMIT_BUMP..."
            ./version_docs.sh "$COMMIT_BUMP"
        else
            echo "version_docs.sh not found or not executable. Please run ./version_docs.sh $COMMIT_BUMP manually."
        fi
    else
        echo "Version bump skipped." 
    fi
else
    echo "Commit cancelled."
fi