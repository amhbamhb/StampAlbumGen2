#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ARTIFACTS_DIR="${1:-$ROOT_DIR/artifacts}"
WEBSITE_REPO_DIR="${2:-$ROOT_DIR/../StampAlbumGen2-website}"
WEBSITE_DEPLOY_DIR="${WEBSITE_DEPLOY_DIR:-}"

# Parse app version from Cargo.toml
APP_VERSION="$({ awk '
	BEGIN { in_pkg=0 }
	/^\[package\]/ { in_pkg=1; next }
	/^\[/ && in_pkg { in_pkg=0 }
	in_pkg && $0 ~ /^[[:space:]]*version[[:space:]]*=/ {
		line=$0
		sub(/^[^"]*"/, "", line)
		sub(/".*/, "", line)
		print line
		exit
	}
' "$ROOT_DIR/Cargo.toml"; } | tr -d '[:space:]')"

if [[ -z "$APP_VERSION" ]]; then
	echo "Error: failed to parse app version from Cargo.toml"
	exit 2
fi

# Validate inputs
if [[ ! -d "$ARTIFACTS_DIR" ]]; then
	echo "Error: artifacts directory not found: $ARTIFACTS_DIR"
	exit 2
fi

if [[ ! -d "$WEBSITE_REPO_DIR" ]]; then
	echo "Error: website repo directory not found: $WEBSITE_REPO_DIR"
	exit 2
fi

LINUX_BINARY="$ARTIFACTS_DIR/stamp_album_gen2"
WINDOWS_BINARY="$ARTIFACTS_DIR/stamp_album_gen2.exe"

if [[ ! -f "$LINUX_BINARY" ]]; then
	echo "Error: required Linux binary missing: $LINUX_BINARY"
	exit 2
fi

echo "Publishing app version $APP_VERSION"
echo "  App artifacts: $ARTIFACTS_DIR"
echo "  Website repo: $WEBSITE_REPO_DIR"
echo "  Deploy target: ${WEBSITE_DEPLOY_DIR:-(not set, build-only mode)}"

# Step 1: Build the website
echo ""
echo "Building website..."
(
	cd "$WEBSITE_REPO_DIR"
	cargo build --release --bin stampalbumgenerator2-site
)

# Step 2: Prepare website download directory
WEBSITE_DOWNLOAD_DIR="$WEBSITE_REPO_DIR/static/downloads/current"
mkdir -p "$WEBSITE_DOWNLOAD_DIR"

echo "Copying app binaries to website..."
cp "$LINUX_BINARY" "$WEBSITE_DOWNLOAD_DIR/stamp_album_gen2"
if [[ -f "$WINDOWS_BINARY" ]]; then
	cp "$WINDOWS_BINARY" "$WEBSITE_DOWNLOAD_DIR/stamp_album_gen2.exe"
fi

# Step 3: Copy checksums
echo "Copying checksums..."
if [[ -f "$ARTIFACTS_DIR/SHA256SUMS.txt" ]]; then
	cp "$ARTIFACTS_DIR/SHA256SUMS.txt" "$WEBSITE_DOWNLOAD_DIR/SHA256SUMS.txt"
fi

if [[ -f "$ARTIFACTS_DIR/CRC32SUMS.txt" ]]; then
	cp "$ARTIFACTS_DIR/CRC32SUMS.txt" "$WEBSITE_DOWNLOAD_DIR/CRC32SUMS.txt"
fi

if [[ -f "$ARTIFACTS_DIR/stamp_album_gen2.sha256" ]]; then
	cp "$ARTIFACTS_DIR/stamp_album_gen2.sha256" "$WEBSITE_DOWNLOAD_DIR/stamp_album_gen2.sha256"
fi

if [[ -f "$ARTIFACTS_DIR/stamp_album_gen2.exe.sha256" ]]; then
	cp "$ARTIFACTS_DIR/stamp_album_gen2.exe.sha256" "$WEBSITE_DOWNLOAD_DIR/stamp_album_gen2.exe.sha256"
fi

# Step 4: Handle articles (if zipped)
if [[ -f "$ARTIFACTS_DIR/articles.zip" ]]; then
	echo "Extracting articles..."
	mkdir -p "$WEBSITE_REPO_DIR/static/articles"
	unzip -o "$ARTIFACTS_DIR/articles.zip" -d "$WEBSITE_REPO_DIR/static/articles"
fi

# Step 5: Update version
printf '%s\n' "$APP_VERSION" > "$WEBSITE_REPO_DIR/APP_VERSION"
echo "Updated APP_VERSION to $APP_VERSION"

# Step 6: Deploy to host directory
if [[ -n "$WEBSITE_DEPLOY_DIR" ]]; then
	echo ""
	echo "Deploying to $WEBSITE_DEPLOY_DIR..."
	
	# Create target directories
	mkdir -p "$WEBSITE_DEPLOY_DIR/bin"
	mkdir -p "$WEBSITE_DEPLOY_DIR/static/downloads/current"
	mkdir -p "$WEBSITE_DEPLOY_DIR/static/articles"
	
	# Copy website binary
	cp "$WEBSITE_REPO_DIR/target/release/stampalbumgenerator2-site" "$WEBSITE_DEPLOY_DIR/bin/"
	
	# Copy static files
	if command -v rsync >/dev/null 2>&1; then
		rsync -a --delete "$WEBSITE_REPO_DIR/static/" "$WEBSITE_DEPLOY_DIR/static/"
	else
		cp -a "$WEBSITE_REPO_DIR/static/." "$WEBSITE_DEPLOY_DIR/static/"
	fi
	
	# Copy version
	cp "$WEBSITE_REPO_DIR/APP_VERSION" "$WEBSITE_DEPLOY_DIR/APP_VERSION"
	
	echo "✓ Website binary deployed"
	echo "✓ Static files deployed"
	echo "✓ Downloads deployed"
	if [[ -d "$WEBSITE_DEPLOY_DIR/static/articles" ]] && [[ -n "$(find "$WEBSITE_DEPLOY_DIR/static/articles" -type f 2>/dev/null || true)" ]]; then
		echo "✓ Articles deployed"
	fi
else
	echo ""
	echo "Note: WEBSITE_DEPLOY_DIR not set; website files remain in repo only"
	echo "To deploy, set WEBSITE_DEPLOY_DIR environment variable"
fi

echo ""
echo "✓ Done! App v$APP_VERSION published"
