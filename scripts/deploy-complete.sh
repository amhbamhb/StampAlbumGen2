#!/usr/bin/env bash
# Called from GitLab CI pipeline - code is already checked out by the runner.
# Builds 3 binaries and copies them to /opt.

set -euo pipefail

DEPLOY_DIR="${1:-/opt/stampalbumgenerator2-site}"
# CI_PROJECT_DIR is set by GitLab CI to the app repo checkout directory.
APP_REPO="${CI_PROJECT_DIR:-.}"
# Website repo must be checked out alongside (or passed as arg 2).
WEBSITE_REPO="${2:-/tmp/StampAlbumGen2-website}"

log_info() { echo "[INFO] $*"; }
log_success() { echo "[✓] $*"; }
log_error() { echo "[ERROR] $*" >&2; }

log_info "Building 3 binaries and deploying to $DEPLOY_DIR"

# Build app binary (Linux)
log_info "Building app binary (Linux)..."
cd "$APP_REPO"
cargo build --release --bin stamp_album_gen2 2>&1 | tail -5
APP_LINUX="$APP_REPO/target/release/stamp_album_gen2"
[[ ! -f "$APP_LINUX" ]] && { log_error "Failed to build Linux app"; exit 1; }
log_success "Linux app built: stamp_album_gen2"

# Build app binary (Windows) - skip if mingw unavailable
log_info "Building app binary (Windows)..."
APP_WINDOWS=""
if command -v x86_64-w64-mingw32-gcc >/dev/null 2>&1; then
  rustup target add x86_64-pc-windows-gnu 2>/dev/null || true
  cd "$APP_REPO"
  cargo build --release --target x86_64-pc-windows-gnu --bin stamp_album_gen2 2>&1 | tail -5
  APP_WINDOWS="$APP_REPO/target/x86_64-pc-windows-gnu/release/stamp_album_gen2.exe"
  [[ -f "$APP_WINDOWS" ]] && log_success "Windows app built: stamp_album_gen2.exe" || { log_error "Failed to build Windows app"; exit 1; }
else
  log_info "mingw not available, skipping Windows build"
fi

# Build website binary
log_info "Building website binary..."
cd "$WEBSITE_REPO"
cargo build --release --bin stampalbumgenerator2-site 2>&1 | tail -5
WEBSITE_BIN="$WEBSITE_REPO/target/release/stampalbumgenerator2-site"
[[ ! -f "$WEBSITE_BIN" ]] && { log_error "Failed to build website"; exit 1; }
log_success "Website binary built: stampalbumgenerator2-site"

# Create deploy directories
log_info "Creating deployment structure..."
mkdir -p "$DEPLOY_DIR/bin"
mkdir -p "$DEPLOY_DIR/static/downloads/current"
mkdir -p "$DEPLOY_DIR/config"

# Copy binaries
log_info "Copying binaries..."
cp "$APP_LINUX" "$DEPLOY_DIR/bin/stamp_album_gen2"
[[ -f "$APP_WINDOWS" ]] && cp "$APP_WINDOWS" "$DEPLOY_DIR/bin/stamp_album_gen2.exe" || true
cp "$WEBSITE_BIN" "$DEPLOY_DIR/bin/stampalbumgenerator2-site"
log_success "Binaries deployed to $DEPLOY_DIR/bin/"

# Generate checksums
log_info "Generating checksums..."
cd "$DEPLOY_DIR/bin"
sha256sum stamp_album_gen2 > "$DEPLOY_DIR/static/downloads/current/SHA256SUMS.txt"
[[ -f stamp_album_gen2.exe ]] && sha256sum stamp_album_gen2.exe >> "$DEPLOY_DIR/static/downloads/current/SHA256SUMS.txt" || true
sha256sum stampalbumgenerator2-site >> "$DEPLOY_DIR/static/downloads/current/SHA256SUMS.txt"
log_success "Checksums generated"

# Copy to downloads directory
cp stamp_album_gen2 "$DEPLOY_DIR/static/downloads/current/"
[[ -f stamp_album_gen2.exe ]] && cp stamp_album_gen2.exe "$DEPLOY_DIR/static/downloads/current/" || true
log_success "Binaries copied to downloads"

# Copy config and website files
log_info "Copying config and website files..."
cp "$APP_REPO/config.toml" "$DEPLOY_DIR/config/" 2>/dev/null || true
cp "$APP_REPO/config_full_example.toml" "$DEPLOY_DIR/config/" 2>/dev/null || true
[[ -d "$WEBSITE_REPO/static" ]] && rsync -av --delete "$WEBSITE_REPO/static/" "$DEPLOY_DIR/static/" 2>&1 | tail -3 || true
log_success "Config and website files copied"

# Set permissions
chmod 755 "$DEPLOY_DIR/bin/"*
chmod 644 "$DEPLOY_DIR/static/downloads/current/"*.txt

# Summary
echo ""
log_success "Deployment Complete!"
echo "Location: $DEPLOY_DIR"
echo "Binaries:"
ls -lh "$DEPLOY_DIR/bin/" | tail -4
echo ""
echo "Checksums:"
cat "$DEPLOY_DIR/static/downloads/current/SHA256SUMS.txt"
echo ""
log_info "To restart service: sudo systemctl restart stampalbumgenerator2-site"

