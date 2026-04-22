#!/usr/bin/env bash
# Complete deployment script: build all binaries, generate checksums, deploy to /opt

set -euo pipefail

# Configuration
DEPLOY_DIR="${1:-/opt/stampalbumgenerator2-site}"
APP_REPO="${2:-/tmp/StampAlbumGen2}"
WEBSITE_REPO="${3:-/tmp/StampAlbumGen2-website}"

# GitLab URLs
GITLAB_HOST="gitinfo.laserspider.info"
APP_REPO_URL="https://${GITLAB_HOST}/rust-apps/rust-stamp_alblum_generate.git"
WEBSITE_REPO_URL="https://${GITLAB_HOST}/rust-apps/stampalbumgenerator2-site.git"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() {
  echo -e "${BLUE}[INFO]${NC} $*"
}

log_success() {
  echo -e "${GREEN}[SUCCESS]${NC} $*"
}

log_error() {
  echo -e "${RED}[ERROR]${NC} $*" >&2
}

# =============================================================================
# Prepare repositories (clone if missing, pull if present)
# =============================================================================
prepare_repo() {
  local repo_path="$1"
  local repo_url="$2"
  local repo_name="$3"
  
  if [[ ! -d "$repo_path" ]]; then
    log_info "Cloning $repo_name to $repo_path..."
    git clone "$repo_url" "$repo_path" || {
      log_error "Failed to clone $repo_name"
      exit 1
    }
  else
    log_info "Updating $repo_name at $repo_path..."
    cd "$repo_path"
    git fetch origin master 2>/dev/null || git fetch origin main 2>/dev/null || true
    git checkout -f FETCH_HEAD 2>/dev/null || git pull origin 2>/dev/null || true
  fi
}

# Verify and prepare repos
if [[ ! -d "$APP_REPO" ]] || [[ ! -d "$WEBSITE_REPO" ]]; then
  prepare_repo "$APP_REPO" "$APP_REPO_URL" "app repo"
  prepare_repo "$WEBSITE_REPO" "$WEBSITE_REPO_URL" "website repo"
fi

log_info "Starting complete deployment to $DEPLOY_DIR"
log_info "App repo: $APP_REPO"
log_info "Website repo: $WEBSITE_REPO"

# Create temporary build directory
BUILD_DIR=$(mktemp -d)
trap "rm -rf $BUILD_DIR" EXIT
log_info "Using temp build directory: $BUILD_DIR"

# =============================================================================
# Step 1: Build app binary (Linux)
# =============================================================================
log_info "Building app binary (Linux)..."
cd "$APP_REPO"
cargo build --release --bin stamp_album_gen2 2>&1 | tail -20
APP_LINUX="$APP_REPO/target/release/stamp_album_gen2"
if [[ ! -f "$APP_LINUX" ]]; then
  log_error "Failed to build Linux app binary"
  exit 1
fi
cp "$APP_LINUX" "$BUILD_DIR/"
log_success "Linux app binary built: stamp_album_gen2"

# =============================================================================
# Step 2: Build app binary (Windows) - if mingw available
# =============================================================================
log_info "Building app binary (Windows)..."
if command -v x86_64-w64-mingw32-gcc >/dev/null 2>&1; then
  rustup target add x86_64-pc-windows-gnu 2>/dev/null || true
  cd "$APP_REPO"
  cargo build --release --target x86_64-pc-windows-gnu --bin stamp_album_gen2 2>&1 | tail -20
  APP_WINDOWS="$APP_REPO/target/x86_64-pc-windows-gnu/release/stamp_album_gen2.exe"
  if [[ -f "$APP_WINDOWS" ]]; then
    cp "$APP_WINDOWS" "$BUILD_DIR/"
    log_success "Windows app binary built: stamp_album_gen2.exe"
  else
    log_error "Failed to build Windows app binary"
    exit 1
  fi
else
  log_info "mingw toolchain not available, skipping Windows build"
fi

# =============================================================================
# Step 3: Build website binary
# =============================================================================
log_info "Building website binary..."
cd "$WEBSITE_REPO"
cargo build --release --bin stampalbumgenerator2-site 2>&1 | tail -20
WEBSITE_BIN="$WEBSITE_REPO/target/release/stampalbumgenerator2-site"
if [[ ! -f "$WEBSITE_BIN" ]]; then
  log_error "Failed to build website binary"
  exit 1
fi
cp "$WEBSITE_BIN" "$BUILD_DIR/"
log_success "Website binary built: stampalbumgenerator2-site"

# =============================================================================
# Step 4: Extract version from Cargo.toml
# =============================================================================
APP_VERSION=$(grep -m1 "^version" "$APP_REPO/Cargo.toml" | sed 's/version = "\(.*\)"/\1/')
log_info "Version: $APP_VERSION"

# =============================================================================
# Step 5: Generate checksums
# =============================================================================
log_info "Generating checksums..."
cd "$BUILD_DIR"

# SHA256 checksums
> SHA256SUMS.txt
for file in stamp_album_gen2 stamp_album_gen2.exe stampalbumgenerator2-site; do
  if [[ -f "$file" ]]; then
    sha256sum "$file" >> SHA256SUMS.txt
  fi
done
log_success "SHA256 checksums generated"

# CRC32 checksums
> CRC32SUMS.txt
for file in stamp_album_gen2 stamp_album_gen2.exe stampalbumgenerator2-site; do
  if [[ -f "$file" ]]; then
    cksum "$file" >> CRC32SUMS.txt
  fi
done
log_success "CRC32 checksums generated"

# =============================================================================
# Step 6: Prepare deployment directory structure
# =============================================================================
log_info "Preparing deployment directory: $DEPLOY_DIR"

# Create directory structure
mkdir -p "$DEPLOY_DIR/bin"
mkdir -p "$DEPLOY_DIR/static/downloads/current"
mkdir -p "$DEPLOY_DIR/config"

# =============================================================================
# Step 7: Copy binaries
# =============================================================================
log_info "Copying binaries..."
cp "$BUILD_DIR/stamp_album_gen2" "$DEPLOY_DIR/bin/" 2>/dev/null || log_error "Failed to copy Linux app binary"
[[ -f "$BUILD_DIR/stamp_album_gen2.exe" ]] && cp "$BUILD_DIR/stamp_album_gen2.exe" "$DEPLOY_DIR/bin/" || log_info "Windows app binary not built"
cp "$BUILD_DIR/stampalbumgenerator2-site" "$DEPLOY_DIR/bin/" || log_error "Failed to copy website binary"
log_success "Binaries copied to $DEPLOY_DIR/bin/"

# =============================================================================
# Step 8: Copy checksums to downloads directory
# =============================================================================
log_info "Copying checksums..."
cp "$BUILD_DIR/SHA256SUMS.txt" "$DEPLOY_DIR/static/downloads/current/" || log_error "Failed to copy SHA256SUMS"
cp "$BUILD_DIR/CRC32SUMS.txt" "$DEPLOY_DIR/static/downloads/current/" || log_error "Failed to copy CRC32SUMS"
log_success "Checksums copied"

# =============================================================================
# Step 9: Copy app binaries to downloads (for end-users)
# =============================================================================
log_info "Copying app binaries to downloads..."
cp "$BUILD_DIR/stamp_album_gen2" "$DEPLOY_DIR/static/downloads/current/" || log_error "Failed to copy Linux app"
[[ -f "$BUILD_DIR/stamp_album_gen2.exe" ]] && cp "$BUILD_DIR/stamp_album_gen2.exe" "$DEPLOY_DIR/static/downloads/current/" || true
log_success "App binaries copied to downloads directory"

# =============================================================================
# Step 10: Copy config files
# =============================================================================
log_info "Copying config files..."
[[ -f "$APP_REPO/config.toml" ]] && cp "$APP_REPO/config.toml" "$DEPLOY_DIR/config/" || true
[[ -f "$APP_REPO/config_full_example.toml" ]] && cp "$APP_REPO/config_full_example.toml" "$DEPLOY_DIR/config/" || true
log_success "Config files copied"

# =============================================================================
# Step 11: Copy website static files
# =============================================================================
log_info "Copying website static files..."
if [[ -d "$WEBSITE_REPO/static" ]]; then
  rsync -av --delete "$WEBSITE_REPO/static/" "$DEPLOY_DIR/static/" 2>&1 | tail -10
  log_success "Website static files synced"
else
  log_info "Website static directory not found (will be created by website on first run)"
fi

# =============================================================================
# Step 12: Write version file
# =============================================================================
log_info "Writing version file..."
echo "$APP_VERSION" > "$DEPLOY_DIR/APP_VERSION"
log_success "Version file written: $APP_VERSION"

# =============================================================================
# Step 13: Set permissions
# =============================================================================
log_info "Setting permissions..."
chmod 755 "$DEPLOY_DIR/bin/"*
chmod 644 "$DEPLOY_DIR/static/downloads/current/"*.txt
log_success "Permissions set"

# =============================================================================
# Step 14: Summary and restart instruction
# =============================================================================
echo ""
log_success "==============================================="
log_success "Deployment Complete!"
log_success "==============================================="
echo ""
echo "Deployment directory: $DEPLOY_DIR"
echo ""
echo "Directory structure:"
tree "$DEPLOY_DIR" 2>/dev/null || find "$DEPLOY_DIR" -type f | head -20
echo ""
echo "Binaries:"
ls -lh "$DEPLOY_DIR/bin/"
echo ""
echo "Version: $APP_VERSION"
echo "Checksums:"
cat "$DEPLOY_DIR/static/downloads/current/SHA256SUMS.txt"
echo ""
log_info "To restart the website service (if running), run:"
echo "  sudo systemctl restart stampalbumgenerator2-site"
echo ""
log_info "Or if using docker/manual:"
echo "  pkill stampalbumgenerator2-site"
echo "  $DEPLOY_DIR/bin/stampalbumgenerator2-site"
echo ""
log_success "All done!"
echo ""
log_info "Script can be run on remote server:"
echo "  ssh user@laserspider.info 'bash /path/to/deploy-complete.sh'"
echo ""
log_info "Or with custom repo locations:"
echo "  bash deploy-complete.sh /opt/stampalbumgenerator2-site /tmp/StampAlbumGen2 /tmp/StampAlbumGen2-website"
