#!/bin/bash
# Build and copy release script

echo "🔧 Formatting code..."
cargo fmt --all

echo "🏗️  Building release..."
cargo build --release

echo "📁 Copying binary..."

INSTALL_DIR="/media/ray/Lacie/Stamps/Stamps Albums/AlbumGen2"
cp ./target/release/stamp_album_gen2 "${INSTALL_DIR}/stamp_album_gen2"

cd "${INSTALL_DIR}"
./stamp_album_gen2 --version

echo "🖥️  Updating desktop launcher..."
./stamp_album_gen2 --install-desktop-launcher

echo "stamp_album_gen2 copied"
