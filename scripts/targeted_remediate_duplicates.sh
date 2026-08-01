#!/usr/bin/env bash
# Targeted remediation script
# For every original_image_url referenced by more than one row in the stamps table,
# download each row's remote_image_url and compare its SHA256 to the local referenced file.
# If the remote differs or the local is missing/a placeholder, copy remote to a per-row file
# and UPDATE the stamps.rowid to point to the new filename. Create a timestamped backup first.

set -euo pipefail

ROOT_DIR="/media/ray/program/Rust/StampAlbumGen2"
DB_PATH="$ROOT_DIR/Albums/Canada/Canada.db"
IMAGES_DIR="$ROOT_DIR/Albums/Canada/images"
BACKUP_DIR="$ROOT_DIR/Albums/Canada/backup_$(date +%Y%m%d_%H%M%S)"
TMP_DIR="/tmp/stamp_remediate_$$"

mkdir -p "$TMP_DIR"
mkdir -p "$BACKUP_DIR"

echo "Backing up DB and images to: $BACKUP_DIR"
cp "$DB_PATH" "$BACKUP_DIR/" || { echo "Failed to copy DB"; exit 1; }
rsync -a --delete "$IMAGES_DIR/" "$BACKUP_DIR/images/" || { echo "Failed to copy images"; exit 1; }

echo "Preparing list of duplicate original_image_url groups..."
mapfile -t groups < <(sqlite3 "$DB_PATH" "SELECT original_image_url FROM stamps WHERE original_image_url IS NOT NULL AND TRIM(original_image_url) != '' GROUP BY original_image_url HAVING COUNT(*)>1;")

SEP=$'\x1f'  # Use a safe separator (unit separator) to avoid clashes with URL characters
mapfile -t groups < <(sqlite3 -separator "$SEP" "$DB_PATH" "SELECT original_image_url FROM stamps WHERE original_image_url IS NOT NULL AND TRIM(original_image_url) != '' GROUP BY original_image_url HAVING COUNT(*)>1;")
echo "Found ${#groups[@]} filename groups with >1 rows"

groups_processed=0
rows_checked=0
rows_changed=0
new_files_created=0

for orig in "${groups[@]}"; do
    groups_processed=$((groups_processed+1))
    # Trim whitespace
    orig_trimmed=$(echo "$orig" | tr -d '\r' | sed 's/^\s*//;s/\s*$//')
    if [[ -z "$orig_trimmed" ]]; then
        continue
    fi

    local_file="$IMAGES_DIR/$orig_trimmed"

    # Get rows that reference this original_image_url
    # Get rows that reference this original_image_url using safe separator
    esc_orig=$(echo "$orig_trimmed" | sed "s/'/''/g")  # Escape single quotes in orig_trimmed for SQL literal
    mapfile -t rows < <(sqlite3 -separator "$SEP" "$DB_PATH" "SELECT rowid || '$SEP' || COALESCE(original_image_url,'') || '$SEP' || COALESCE(remote_image_url,'') || '$SEP' || COALESCE(sg_number,'') || '$SEP' || COALESCE(series,'') FROM stamps WHERE original_image_url = '$esc_orig';")
    if [[ ${#rows[@]} -eq 0 ]]; then
        continue  # Nothing for this group (shouldn't happen), continue
    fi
    for rowline in "${rows[@]}"; do
        # Parse rowline into fields using the safe separator
        IFS="$SEP" read -r rowid origname remote sg series <<< "$rowline"
        rows_checked=$((rows_checked+1))
        # Skip if remote_image_url empty
        if [[ -z "$remote" ]]; then
            # Nothing to compare; skip
            continue
        fi

        # Determine if local file exists and whether it's a placeholder
        is_placeholder=false
        if [[ ! -f "$local_file" ]]; then
            is_placeholder=true
        else
            base_name=$(basename "$local_file")
            if [[ "$base_name" == no_image* || "$base_name" == "no_image_placeholder.png" ]]; then
                is_placeholder=true
            fi
        fi

        # Download remote to tmp file
        tmp_remote="$TMP_DIR/remote_${rowid}"
        curl -sL --fail -o "$tmp_remote" "$remote" || { echo "[WARN] Failed to download remote for row $rowid: $remote"; rm -f "$tmp_remote"; continue; }

        # Compute sha sums
        remote_sum=$(sha256sum "$tmp_remote" | awk '{print $1}')
        local_sum=""
        if [[ -f "$local_file" ]]; then
            local_sum=$(sha256sum "$local_file" | awk '{print $1}')
        fi

        if [[ "$is_placeholder" == true || -z "$local_sum" || "$remote_sum" != "$local_sum" ]]; then
            # Need to create per-row file
            # Use orig filename stem + _row<rowid> + ext
            ext="${orig_trimmed##*.}"
            if [[ "$ext" == "$orig_trimmed" ]]; then
                ext="jpg"
            fi
            safe_newname="${orig_trimmed%.*}_row${rowid}.${ext}"
            dest="$IMAGES_DIR/$safe_newname"
            # if dest exists, find an available suffix
            if [[ -f "$dest" ]]; then
                suffix=1
                while [[ -f "$IMAGES_DIR/${orig_trimmed%.*}_row${rowid}_${suffix}.${ext}" ]]; do
                    suffix=$((suffix+1))
                done
                dest="$IMAGES_DIR/${orig_trimmed%.*}_row${rowid}_${suffix}.${ext}"
                safe_newname="${orig_trimmed%.*}_row${rowid}_${suffix}.${ext}"
            fi
            mv "$tmp_remote" "$dest"
            chmod 644 "$dest" || true
            sqlite3 "$DB_PATH" "UPDATE stamps SET original_image_url = '$(echo $safe_newname | sed "s/'/''/g")' WHERE rowid = $rowid;"
            echo "[INFO] Created $safe_newname for row $rowid (orig: $orig_trimmed)"
            rows_changed=$((rows_changed+1))
            new_files_created=$((new_files_created+1))
        else
            # No change needed
            rm -f "$tmp_remote"
        fi
    done  # End of processing for each row
done

echo "Migration done." 
echo "Groups processed: $groups_processed" 
echo "Rows checked: $rows_checked" 
echo "Rows changed: $rows_changed" 
echo "New files created: $new_files_created" 

echo "Post-migration DB summary:"
sqlite3 "$DB_PATH" "SELECT COUNT(*) FROM stamps;"
sqlite3 "$DB_PATH" "SELECT COUNT(DISTINCT original_image_url) FROM stamps;"
echo "Top duplicate groups (post):"
sqlite3 "$DB_PATH" "SELECT original_image_url, COUNT(*) c FROM stamps GROUP BY original_image_url HAVING c>1 ORDER BY c DESC LIMIT 20;"

echo "Cleaning up tmp dir"
rm -rf "$TMP_DIR"

echo "Done. Backup is at: $BACKUP_DIR"
