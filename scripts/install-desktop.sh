#!/usr/bin/env bash
# install-desktop.sh — Install the StampAlbumGen2 desktop entry and icon for the current user.
#
# Usage:
#   install-desktop.sh [--binary <path>] [--uninstall]
#
# On Ubuntu/Wayland the icon MUST live in the hicolor XDG icon theme directory
# (not just an arbitrary path) for the launcher and taskbar to pick it up.
# This script installs icons at all standard sizes and refreshes the icon cache.
#
# Run without arguments to auto-detect the binary from $PATH.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"

APP_ID="stamp_album_gen2"
DESKTOP_NAME="Stamp Album Generator 2"
DESKTOP_FILE="$HOME/.local/share/applications/${APP_ID}.desktop"
ICON_DIR_BASE="$HOME/.local/share/icons/hicolor"
ASSET_ICONS="$REPO_ROOT/assets/icons/generated"

BINARY_PATH=""
UNINSTALL=false

log_info()  { echo "[INFO]  $*"; }
log_ok()    { echo "[OK]    $*"; }
log_error() { echo "[ERROR] $*" >&2; }

usage() {
  echo "Usage: $0 [--binary <path>] [--uninstall]"
  echo "  --binary    Absolute path to the stamp_album_gen2 binary (auto-detected if omitted)"
  echo "  --uninstall Remove the desktop entry and icons"
  exit 1
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --binary)    BINARY_PATH="$2"; shift 2 ;;
    --uninstall) UNINSTALL=true; shift ;;
    -h|--help)   usage ;;
    *) log_error "Unknown argument: $1"; usage ;;
  esac
done

# ── Uninstall ─────────────────────────────────────────────────────────────────
if [[ "$UNINSTALL" == true ]]; then
  log_info "Removing desktop entry..."
  rm -f "$DESKTOP_FILE"

  log_info "Removing icons..."
  for size in 16 32 48 64 96 128 192 256 512; do
    rm -f "$ICON_DIR_BASE/${size}x${size}/apps/${APP_ID}.png"
  done

  command -v update-desktop-database >/dev/null 2>&1 && \
    update-desktop-database "$HOME/.local/share/applications" 2>/dev/null || true
  command -v gtk-update-icon-cache >/dev/null 2>&1 && \
    gtk-update-icon-cache -f -t "$ICON_DIR_BASE" 2>/dev/null || true

  log_ok "Uninstall complete."
  exit 0
fi

# ── Resolve binary path ───────────────────────────────────────────────────────
if [[ -z "$BINARY_PATH" ]]; then
  if command -v "$APP_ID" >/dev/null 2>&1; then
    BINARY_PATH="$(command -v "$APP_ID")"
    log_info "Auto-detected binary: $BINARY_PATH"
  else
    log_error "Binary '$APP_ID' not found in PATH. Install it first, or pass --binary <path>."
    exit 1
  fi
fi

[[ ! -x "$BINARY_PATH" ]] && { log_error "Binary not executable: $BINARY_PATH"; exit 1; }

# ── Install icons at each size ────────────────────────────────────────────────
log_info "Installing icons..."
declare -A SIZE_MAP=(
  [16]="stamp-gen2-premium-icon-16.png"
  [32]="stamp-gen2-premium-icon-32.png"
  [48]="stamp-gen2-premium-icon-48.png"
  [64]="stamp-gen2-premium-icon-64.png"
  [96]="stamp-gen2-premium-icon-96.png"
  [128]="stamp-gen2-premium-icon-128.png"
  [192]="stamp-gen2-premium-icon-192.png"
  [256]="stamp-gen2-premium-icon-256.png"
  [512]="stamp-gen2-premium-icon-512.png"
)

for size in "${!SIZE_MAP[@]}"; do
  src="$ASSET_ICONS/${SIZE_MAP[$size]}"
  dest_dir="$ICON_DIR_BASE/${size}x${size}/apps"
  dest="$dest_dir/${APP_ID}.png"
  if [[ -f "$src" ]]; then
    mkdir -p "$dest_dir"
    cp "$src" "$dest"
    log_ok "Icon ${size}x${size} installed"
  else
    log_info "No source icon for ${size}x${size} (skipped)"
  fi
done

# ── Write .desktop file ───────────────────────────────────────────────────────
log_info "Writing desktop entry..."
mkdir -p "$(dirname "$DESKTOP_FILE")"

cat > "$DESKTOP_FILE" <<EOF
[Desktop Entry]
Version=1.0
Type=Application
Name=${DESKTOP_NAME}
GenericName=Stamp Album Generator
Comment=Generate printable stamp album pages from Colnect stamp data
Exec=${BINARY_PATH}
Icon=${APP_ID}
Terminal=true
Categories=Utility;Graphics;
Keywords=stamp;album;philately;collector;generator;
StartupNotify=false
EOF

chmod 644 "$DESKTOP_FILE"
log_ok "Desktop entry written: $DESKTOP_FILE"

# ── Refresh caches (Wayland requires this to pick up the icon) ────────────────
log_info "Refreshing icon and desktop caches..."
command -v gtk-update-icon-cache >/dev/null 2>&1 && \
  gtk-update-icon-cache -f -t "$ICON_DIR_BASE" 2>/dev/null && \
  log_ok "gtk-update-icon-cache done" || true

command -v update-icon-caches >/dev/null 2>&1 && \
  update-icon-caches "$ICON_DIR_BASE" 2>/dev/null && \
  log_ok "update-icon-caches done" || true

command -v update-desktop-database >/dev/null 2>&1 && \
  update-desktop-database "$HOME/.local/share/applications" 2>/dev/null && \
  log_ok "update-desktop-database done" || true

# ── Summary ───────────────────────────────────────────────────────────────────
echo ""
log_ok "Desktop integration installed for user: $USER"
echo "  Entry:  $DESKTOP_FILE"
echo "  Icons:  $ICON_DIR_BASE/*/apps/${APP_ID}.png"
echo ""
echo "Log out and back in (or run: killall nautilus) if the icon still doesn't appear."
