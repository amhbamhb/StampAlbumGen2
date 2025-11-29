#!/bin/bash
# Build MUSL binary with vendored OpenSSL
# This script builds a fully static Linux binary using MUSL

set -e

cd "$(dirname "${BASH_SOURCE[0]}")"

echo "🔨 Building MUSL static binary..."
echo ""

# Install target if not present
echo "📦 Ensuring x86_64-unknown-linux-musl target is installed..."
rustup target add x86_64-unknown-linux-musl

# For MUSL builds, we need to compile OpenSSL from source
# Set environment variables to tell openssl-sys to build vendored OpenSSL
export OPENSSL_STATIC=1
export OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-musl
export OPENSSL_INCLUDE_DIR=/usr/include/x86_64-linux-musl
export RUSTFLAGS="-C target-feature=+crt-static"

# Try to get/create musl openssl headers if they exist
if [ -d "/usr/include/x86_64-linux-musl/openssl" ]; then
    echo "✓ Found musl openssl headers"
elif [ -d "/usr/include/openssl" ]; then
    echo "ℹ Using system OpenSSL headers (may need musl-specific)"
    export OPENSSL_DIR=/usr
fi

echo ""
echo "🏗️  Compiling...  (this may take 2-5 minutes)"
echo ""

# Cargo should use vendored openssl via the sys crate
cargo build --release --target x86_64-unknown-linux-musl

echo ""
echo "✓ Build complete!"
echo ""

BINARY="target/x86_64-unknown-linux-musl/release/stamp_album_gen2"
if [ -f "$BINARY" ]; then
    SIZE=$(ls -lh "$BINARY" | awk '{print $5}')
    echo "📦 Binary: $BINARY"
    echo "📊 Size: $SIZE"
    echo ""
    echo "🔍 Verifying static linking:"
    file "$BINARY"
    ldd "$BINARY" 2>&1 | head -5 || true
    echo ""
    echo "✓ Ready to distribute!"
else
    echo "❌ Binary not found at expected location"
    exit 1
fi
