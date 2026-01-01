#!/bin/bash

# Script to rename files with a prefix, skipping if a string already exists in filename
# Usage: ./rename_with_prefix.sh <directory> <prefix> <check_string>
# Example: ./rename_with_prefix.sh /path/to/files "my_prefix_" "enhanced"

if [ $# -lt 2 ]; then
    echo "Usage: $0 [--dry-run] [--remove-prefix PREFIX] <directory> <prefix> [check_string]"
    echo ""
    echo "Renames files in a directory by adding a prefix."
    echo "If check_string is provided, skips files that already contain it."
    echo ""
    echo "Options:"
    echo "  --dry-run              - Preview changes without renaming (what-if mode)"
    echo "  --remove-prefix PREFIX - Remove this prefix from filenames before adding new prefix"
    echo ""
    echo "Arguments:"
    echo "  directory    - Directory containing files to rename"
    echo "  prefix       - Prefix to add to filenames (e.g., 'new_')"
    echo "  check_string - Optional: String to check for. Skip if found in filename."
    echo ""
    echo "Examples:"
    echo "  $0 ./files 'test_'                                    # Rename all files with 'test_' prefix"
    echo "  $0 ./files 'enhanced_' 'old'                          # Add prefix only if 'old' not in filename"
    echo "  $0 --dry-run ./files 'test_'                          # Preview changes (what-if)"
    echo "  $0 --remove-prefix 'bad_' ./files 'good_'             # Remove 'bad_' and add 'good_'"
    echo "  $0 --dry-run --remove-prefix 'bad_' ./files 'good_'   # Preview the fix"
    exit 1
fi

# Check for --dry-run flag anywhere in arguments
DRY_RUN=false
REMOVE_PREFIX=""
remaining_args=()
i=0
for arg in "$@"; do
    if [ "$arg" = "--dry-run" ] || [ "$arg" = "-d" ]; then
        DRY_RUN=true
    elif [ "$arg" = "--remove-prefix" ]; then
        # Next argument is the prefix to remove
        true  # Will be handled in next iteration
    elif [ "${prev_arg:-}" = "--remove-prefix" ]; then
        REMOVE_PREFIX="$arg"
    else
        remaining_args+=("$arg")
    fi
    prev_arg="$arg"
done

# Assign remaining arguments
DIR="${remaining_args[0]}"
PREFIX="${remaining_args[1]}"
CHECK_STRING="${remaining_args[2]:-}"

# Validate directory
if [ ! -d "$DIR" ]; then
    echo "Error: Directory '$DIR' does not exist"
    exit 1
fi

# Count renamed files
count=0
skipped=0

# Process files in directory
for file in "$DIR"/*; do
    # Skip directories
    if [ -d "$file" ]; then
        continue
    fi
    
    filename=$(basename "$file")
    
    # Check if string should be checked and if it exists
    if [ -n "$CHECK_STRING" ]; then
        if [[ "$filename" == *"$CHECK_STRING"* ]]; then
            echo "SKIP: $filename (contains '$CHECK_STRING')"
            ((skipped++))
            continue
        fi
    fi
    
    # Skip if prefix already exists
    if [[ "$filename" == "$PREFIX"* ]]; then
        echo "SKIP: $filename (already has prefix)"
        ((skipped++))
        continue
    fi
    
    # Construct new filename
    clean_filename="$filename"
    
    # Remove bad prefix if specified
    if [ -n "$REMOVE_PREFIX" ] && [[ "$clean_filename" == "$REMOVE_PREFIX"* ]]; then
        clean_filename="${clean_filename#$REMOVE_PREFIX}"
        echo "   Removing: '$REMOVE_PREFIX' from '$filename'"
    fi
    
    newfile="$DIR/${PREFIX}${clean_filename}"
    
    # Rename file
    if [ "$DRY_RUN" = true ]; then
        # Dry-run mode: just show what would happen
        echo "→ [WOULD RENAME] $filename → $(basename "$newfile")"
        ((count++))
    else
        # Actually rename the file
        if mv "$file" "$newfile"; then
            echo "✓ $filename → ${PREFIX}${filename}"
            ((count++))
        else
            echo "✗ Failed to rename: $filename"
        fi
    fi
done

echo ""
echo "Summary:"
if [ "$DRY_RUN" = true ]; then
    echo "  MODE: DRY-RUN (Preview only - no files renamed)"
    echo "  Would rename: $count file(s)"
else
    echo "  Renamed: $count file(s)"
fi
echo "  Skipped: $skipped file(s)"
