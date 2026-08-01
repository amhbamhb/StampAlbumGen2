#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="/media/ray/program/Rust/StampAlbumGen2"
DB_PATH="$ROOT_DIR/Albums/Canada/Canada.db"
IMAGES_DIR="$ROOT_DIR/Albums/Canada/images"
BACKUP_DIR="$ROOT_DIR/Albums/Canada/backup_force_$(date +%Y%m%d_%H%M%S)"
TMP_DIR="/tmp/stamp_force_$$"

mkdir -p "$TMP_DIR"
mkdir -p "$BACKUP_DIR"

echo "Backing up DB and images to: $BACKUP_DIR"
cp "$DB_PATH" "$BACKUP_DIR/" || { echo "Failed to copy DB"; exit 1; }
rsync -a --delete "$IMAGES_DIR/" "$BACKUP_DIR/images/" || { echo "Failed to copy images"; exit 1; }

SEP=$'\x1f'
mapfile -t rows < <(sqlite3 -separator "$SEP" "$DB_PATH" "SELECT rowid || '$SEP' || COALESCE(original_image_url,'') || '$SEP' || COALESCE(remote_image_url,'') FROM stamps ORDER BY rowid;")

total=0
changed=0
skipped=0

for r in "${rows[@]}"; do
    IFS="$SEP" read -r rowid orig remote <<< "$r"
    total=$((total+1))
    # Determine source file
    if [[ -n "$orig" && -f "$IMAGES_DIR/$orig" ]]; then
        src="$IMAGES_DIR/$orig"
    else
        # try remote
        if [[ -n "$remote" ]]; then
            tmpf="$TMP_DIR/row_${rowid}"
            if curl -sL --fail -o "$tmpf" "$remote"; then
                src="$tmpf"
            else
                echo "[WARN] row $rowid: failed to download remote ($remote)" >&2
                skipped=$((skipped+1))
                continue
            fi
        else
            echo "[WARN] row $rowid: no orig and no remote, skipping" >&2
            skipped=$((skipped+1))
            continue
        fi
    fi

    # Create unique filename: orig_stem_row<rowid>.<ext> or row<rowid>.jpg
    if [[ -n "$orig" && "$orig" == *.* ]]; then
        stem="${orig%.*}"
        ext="${orig##*.}"
    else
        stem="row"
        ext="jpg"
    fi
    newname="${stem}_row${rowid}.${ext}"
    dest="$IMAGES_DIR/$newname"
    if [[ -f "$dest" ]]; then
        suffix=1
        while [[ -f "$IMAGES_DIR/${stem}_row${rowid}_${suffix}.${ext}" ]]; do
            suffix=$((suffix+1))
        done
        newname="${stem}_row${rowid}_${suffix}.${ext}"
        dest="$IMAGES_DIR/$newname"
    fi

    cp "$src" "$dest"
    chmod 644 "$dest" || true
    sqlite3 "$DB_PATH" "UPDATE stamps SET original_image_url = '$(echo $newname | sed "s/'/''/g")' WHERE rowid = $rowid;"
    changed=$((changed+1))

    # clean tmp if used
    if [[ "$src" == "$TMP_DIR/row_${rowid}" ]]; then
        rm -f "$src"
    fi
done

echo "Force-uniqueness done. total rows: $total changed: $changed skipped: $skipped"
sqlite3 "$DB_PATH" "SELECT COUNT(*) FROM stamps;"
sqlite3 "$DB_PATH" "SELECT COUNT(DISTINCT original_image_url) FROM stamps;"

rm -rf "$TMP_DIR"
echo "Backup at $BACKUP_DIR"
