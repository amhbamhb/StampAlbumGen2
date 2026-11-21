#!/usr/bin/env bash
# ==============================================================================
# Build script for StampAlbumGen2 Help Manual on Linux
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
SOURCE_TYP="$SCRIPT_DIR/src/manual.typ"
MANUAL_PDF="$SCRIPT_DIR/StampAlbumGenerator2HelpManual.pdf"
ROOT_PDF="$REPO_ROOT/StampAlbumGenerator2HelpManual.pdf"

# Locate typst command
TYPST_CMD="typst"
if ! command -v "$TYPST_CMD" >/dev/null 2>&1; then
    if [ -x "$HOME/.cargo/bin/typst" ]; then
        TYPST_CMD="$HOME/.cargo/bin/typst"
    else
        echo "Error: typst is not installed or not in PATH." >&2
        echo "To install on Linux:" >&2
        echo "  cargo install --locked typst-cli" >&2
        echo "  or: sudo apt install typst    (Ubuntu 24.04+ / Debian)" >&2
        echo "  or: sudo pacman -S typst       (Arch Linux)" >&2
        echo "  or: sudo snap install typst     (Snap)" >&2
        exit 1
    fi
fi

echo "Compiling StampAlbumGen2 Help Manual with Typst..."
"$TYPST_CMD" compile --root "$REPO_ROOT" "$SOURCE_TYP" "$MANUAL_PDF"

if [ -f "$MANUAL_PDF" ]; then
    echo "Manual compiled successfully: $MANUAL_PDF"
    cp -f "$MANUAL_PDF" "$ROOT_PDF"
    echo "Synchronized embedded executable copy: $ROOT_PDF"
    
    FILE_SIZE=$(stat -c%s "$ROOT_PDF" 2>/dev/null || stat -f%z "$ROOT_PDF" 2>/dev/null || echo "0")
    SIZE_KB=$(( FILE_SIZE / 1024 ))
    echo "File size: ${SIZE_KB} KB"

    # Optional: open with xdg-open if --open flag is passed
    if [ "${1:-}" = "--open" ] || [ "${1:-}" = "-o" ]; then
        if command -v xdg-open >/dev/null 2>&1; then
            xdg-open "$ROOT_PDF"
        fi
    fi
else
    echo "Error: Failed to produce $MANUAL_PDF" >&2
    exit 1
fi
