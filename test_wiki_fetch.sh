#!/bin/bash
set -e

# Test Wikipedia fetch in isolation
cd /media/ray/program/Rust/StampAlbumGen2

# Create a test config that enables Wikipedia
cat > /tmp/test_wiki_config.toml << 'TOML'
fetch_wikipedia = true
wikipedia_cache_db_path = "/tmp/wiki_test_cache.db"
TOML

# Start a simple test that only tries to fetch
# We'll use Rust directly to test the fetcher
cargo run --bin stampworld_debug --example test_wikipedia_fetch 2>&1 &
PID=$!

# Give it 120 seconds
timeout 120 wait $PID || EXIT_CODE=$?

if [ $EXIT_CODE -eq 124 ]; then
    echo "❌ Wikipedia fetch timed out after 120 seconds"
    exit 1
elif [ $EXIT_CODE -eq 0 ]; then
    echo "✅ Wikipedia fetch succeeded"
    exit 0
else
    echo "❌ Wikipedia fetch failed with exit code $EXIT_CODE"
    exit $EXIT_CODE
fi
