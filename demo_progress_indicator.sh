#!/bin/bash

# Demo script to showcase the progress indicator
echo "🎬 Progress Indicator Demo for StampAlbumGen2"
echo "============================================="
echo

echo "1. 📊 Testing with ENABLED progress indicator:"
echo "   (You should see spinning icons with status messages)"
echo "   ./target/release/stamp_album_gen2 --db-path Albums/Switzerland/Switzerland.db --album-name 'Demo Test'"
echo

# Run with progress enabled
./target/release/stamp_album_gen2 --db-path Albums/Switzerland/Switzerland.db --album-name "Demo Test"

echo
echo "2. 🔇 Testing with DISABLED progress indicator:"
echo "   (Setting show_progress_indicator = false in config.toml)"

# Disable progress indicator
sed -i 's/show_progress_indicator = true/show_progress_indicator = false/' config.toml

echo "   ./target/release/stamp_album_gen2 --db-path Albums/Switzerland/Switzerland.db --album-name 'Demo Test Silent'"
echo

# Run with progress disabled  
./target/release/stamp_album_gen2 --db-path Albums/Switzerland/Switzerland.db --album-name "Demo Test Silent"

echo
echo "3. ✅ Re-enabling progress indicator for normal use"

# Re-enable progress indicator
sed -i 's/show_progress_indicator = false/show_progress_indicator = true/' config.toml

echo "   Progress indicator restored to enabled state"

echo
echo "🎉 Demo complete!"
echo
echo "📝 Summary of what was implemented:"
echo "   • Spinning progress indicator with status messages"
echo "   • Visual feedback for major operations:"
echo "     - 🔍 Database initialization"
echo "     - 📁 Directory creation"
echo "     - 🔗 Database connections"
echo "     - 🌐 StampWorld scraping setup"
echo "     - 📖 Album generation"
echo "     - ✅ Success/❌ Error indicators"
echo "   • Configurable via config.toml (show_progress_indicator = true/false)"
echo "   • Unicode spinner animation with different stages"
echo
echo "🔧 Configuration:"
echo "   Add to config.toml:"
echo "   show_progress_indicator = true  # Enable spinning progress indicator"
echo "   show_progress_indicator = false # Disable for silent operation"