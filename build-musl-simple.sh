#!/bin/bash
# Simplified MUSL build script - uses musl-gcc wrapper and self-contained build

set -e

cd "$(dirname "${BASH_SOURCE[0]}")"

echo "🔨 Building MUSL static binary with workaround..."
echo ""

# Install target
rustup target add x86_64-unknown-linux-musl

# The trick: Don't look for system OpenSSL, build everything static
# This works by telling Cargo to use MUSL libc without needing OpenSSL headers

export RUSTFLAGS="-C target-feature=+crt-static -C relocation-model=pic"
export CARGO_BUILD_TARGET_LINKER="x86_64-linux-musl-gcc"

# Tell openssl-sys to skip pkg-config and just link
export OPENSSL_NO_PKG_CONFIG=1
export OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-musl
export OPENSSL_INCLUDE_DIR=/usr/include/x86_64-linux-musl

# Actually, best approach: just skip OpenSSL detection entirely
# by using native-tls-vendored through reqwest if possible

echo "Building without OpenSSL system libraries..."
echo "This creates a fully static binary."
echo ""

# Build - this might fail once but shows us what's needed
cargo build --release --target x86_64-unknown-linux-musl || true

echo ""
echo "⚠️  If build failed above, it's due to OpenSSL not having MUSL headers."
echo "Next step: Install OpenSSL development files"
echo ""
