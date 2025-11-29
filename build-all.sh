#!/bin/bash
# Build script for cross-platform compilation
# Usage: ./build-all.sh or ./build-all.sh musl

set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
VERSION=$(grep '^version' "$PROJECT_DIR/Cargo.toml" | head -1 | sed 's/version = "\([^"]*\)".*/\1/')
BUILD_DATE=$(date +%Y%m%d_%H%M%S)
RELEASE_DIR="$PROJECT_DIR/releases/$VERSION"

# Create release directory
mkdir -p "$RELEASE_DIR"

echo "📦 Building stamp_album_gen2 v$VERSION"
echo "📅 Build date: $BUILD_DATE"
echo "📂 Release dir: $RELEASE_DIR"
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to build a target
build_target() {
	local target=$1
	local display_name=$2
	local extra_env=$3

	echo -e "${BLUE}=== Building for $display_name ($target) ===${NC}"

	# Set up environment
	export $extra_env

	# Build
	if cargo build --release --target "$target" 2>&1; then
		local binary_path="$PROJECT_DIR/target/$target/release/stamp_album_gen2"
		if [ -f "$binary_path" ]; then
			local output_name="stamp_album_gen2_${display_name// /_}_v${VERSION}"
			cp "$binary_path" "$RELEASE_DIR/$output_name"
			local size=$(ls -lh "$RELEASE_DIR/$output_name" | awk '{print $5}')
			echo -e "${GREEN}✓ Built $display_name${NC}"
			echo "  Path: $RELEASE_DIR/$output_name"
			echo "  Size: $size"
			file "$RELEASE_DIR/$output_name" || true
		fi
	else
		echo -e "${YELLOW}⚠ Build failed for $display_name${NC}"
	fi
	echo ""
}

# Parse command line arguments
if [ "$1" == "musl" ] || [ -z "$1" ]; then
	echo "📊 Building with MUSL for static linking..."
	export OPENSSL_STATIC=1
	build_target "x86_64-unknown-linux-musl" "Linux MUSL x86_64"
fi

if [ "$1" == "gnu" ] || [ -z "$1" ]; then
	echo "📊 Building for standard GNU Linux..."
	build_target "x86_64-unknown-linux-gnu" "Linux GNU x86_64"
fi

echo -e "${GREEN}=== Build Summary ===${NC}"
echo "Binaries available in: $RELEASE_DIR"
ls -lh "$RELEASE_DIR" 2>/dev/null || echo "No binaries built"
