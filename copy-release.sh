#!/bin/bash
# Build and copy release script

echo "🔧 Formatting code..."
cargo fmt --all

echo "🏗️  Building release..."
cargo build --release

echo "📁 Copying binary..."

cp ./target/release/stamp_album_gen2 "/media/ray/Lacie/Stamps/Stamps Albums/AlbumGen2/stamp_album_gen2"
# Then in that directory re-run:
cd "/media/ray/Lacie/Stamps/Stamps Albums/AlbumGen2"
./stamp_album_gen2 --version
# ./stamp_album_gen2 --site-name Switzerland --album-name Switzerland --start-year 1976 --end-year 1976
echo "stamp_album_gen2 copied"
