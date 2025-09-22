#!/usr/bin/env bash
# Dedupe script for Albums/Canada images and DB
# - For any original_image_url that appears more than once in the stamps table,
#   keep the first row as-is and for each additional row create a unique filename
#   by appending _1, _2, ... before the extension, copying the existing file if present,
#   and update the DB row to point to the new filename.
# Usage: ./scripts/dedupe_images.sh Albums/Canada/Canada.db Albums/Canada/images

set -u
DB_PATH=${1:-Albums/Canada/Canada.db}
IMG_DIR=${2:-Albums/Canada/images}

if [ ! -f "$DB_PATH" ]; then
  echo "DB not found: $DB_PATH"
  exit 1
fi

if [ ! -d "$IMG_DIR" ]; then
  echo "Images dir not found: $IMG_DIR"
  exit 1
fi

echo "Using DB: $DB_PATH"
echo "Images dir: $IMG_DIR"

echo "Finding duplicate filenames in DB..."

# Get list of filenames with count > 1
mapfile -t duplicates < <(sqlite3 "$DB_PATH" "SELECT original_image_url FROM stamps GROUP BY original_image_url HAVING COUNT(*)>1;")

if [ ${#duplicates[@]} -eq 0 ]; then
  echo "No duplicate original_image_url entries found."
  exit 0
fi

echo "Found ${#duplicates[@]} duplicate filename groups. Processing..."

for fname in "${duplicates[@]}"; do
  # Trim whitespace
  fname=$(echo "$fname" | sed -e 's/^\s*//;s/\s*$//')
  if [ -z "$fname" ]; then
    continue
  fi
  echo "Processing duplicate group: $fname"

  # Get all rowids for this filename ordered by rowid
  mapfile -t rows < <(sqlite3 "$DB_PATH" "SELECT rowid FROM stamps WHERE original_image_url = '$(sqlite3 "$DB_PATH" "SELECT quote('$fname')" | sed "s/^'//;s/'$//")' ORDER BY rowid;")

  if [ ${#rows[@]} -le 1 ]; then
    echo "  Only one entry, skipping: $fname"
    continue
  fi

  # Keep the first row, change the rest
  keep=${rows[0]}
  echo "  Keeping row: $keep"

  # Prepare stem/extension
  base="$fname"
  ext=""
  stem="$base"
  if [[ "$base" == *.* ]]; then
    ext="${base##*.}"
    stem="${base%.*}"
  fi

  counter=1
  for ((i=1;i<${#rows[@]};i++)); do
    rid=${rows[$i]}

    # find candidate name that doesn't exist on disk or in DB
    while true; do
      if [ -z "$ext" ]; then
        candidate="${stem}_${counter}"
      else
        candidate="${stem}_${counter}.${ext}"
      fi
      candidate_path="$IMG_DIR/$candidate"

      # Check file existence and DB usage
      file_exists=0
      db_used=0
      if [ -f "$candidate_path" ]; then file_exists=1; fi
      cnt=$(sqlite3 "$DB_PATH" "SELECT COUNT(*) FROM stamps WHERE original_image_url = '$(sqlite3 "$DB_PATH" "SELECT quote('$candidate')" | sed "s/^'//;s/'$//")';")
      if [ "$cnt" -gt 0 ]; then db_used=1; fi

      if [ "$file_exists" -eq 0 -a "$db_used" -eq 0 ]; then
        break
      fi
      counter=$((counter+1))
      if [ "$counter" -gt 1000 ]; then
        echo "  Unable to find candidate name for row $rid of group $fname, giving up.";
        candidate=""
        break
      fi
    done

    if [ -z "$candidate" ]; then
      echo "  Skipping row $rid (no candidate available)"
      continue
    fi

    # If original file exists, copy it to candidate; otherwise, if there's an existing file for this group, try to copy that
    src_path="$IMG_DIR/$base"
    if [ -f "$src_path" ]; then
      cp -a "$src_path" "$candidate_path" && echo "  Copied $src_path -> $candidate_path"
    else
      echo "  Original file for group not found on disk: $src_path. Attempting to find any existing file matching stem..."
      # Try to find any file that starts with stem
      found=""
      for f in "$IMG_DIR"/*; do
        fnameonly=$(basename "$f")
        if [[ "$fnameonly" == "$stem"* ]]; then
          cp -a "$f" "$candidate_path" && found="$f" && echo "  Copied $f -> $candidate_path" && break
        fi
      done
      if [ -z "$found" ]; then
        echo "  No source file available to copy for row $rid. Leaving DB entry unchanged."
        continue
      fi
    fi

    # Update DB row to point to candidate
    sqlite3 "$DB_PATH" "UPDATE stamps SET original_image_url = '$(sqlite3 "$DB_PATH" "SELECT quote('$candidate')" | sed "s/^'//;s/'$//")' WHERE rowid = $rid;"
    echo "  Updated row $rid -> $candidate"
    counter=$((counter+1))
  done

done

echo "Dedupe completed. You should now inspect the DB and images dir to confirm results."

exit 0
