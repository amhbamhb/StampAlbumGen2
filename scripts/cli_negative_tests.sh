#!/usr/bin/env bash
# CLI negative-path smoke tests for stamp_album_gen2
set -eu
BIN="$(pwd)/target/release/stamp_album_gen2"

echo "1) No args"
$BIN || true

echo "\n2) Unknown source"
$BIN --source unknownsource || true

echo "\n3) Nonsensical years (start > end)"
$BIN --source stampworld --site-name 'Great-Britain' --start-year 3000 --end-year 1000 || true

echo "\n4) Non-existent DB path (db-only mode)"
$BIN --db-path Albums/NonExistent/NotAFile.db --album-name NonExistent || true

echo "\n5) Conflicting usage: source != stampworld but db_path provided (should prefer db_path)"
$BIN --source touchstamps --db-path Albums/Canada/Canada.db --album-name Canada || true

echo "\n6) Invalid numeric value for start-year"
$BIN --start-year notanumber || true

echo "\n7) Unknown flag"
$BIN --unknownflag || true

echo "\n8) Low-confidence misspelling with --yes in non-interactive mode"
# Simulate non-interactive by redirecting stdin from /dev/null
( echo; ) | $BIN --source stmpwld --yes </dev/null || true

echo "\n9) Corrupt DB file (non-sqlite data)"
# Create a small corrupt file and point DB at it
tmpdb="$(pwd)/tmp_corrupt.db"
printf '%s' "not a sqlite file" > "$tmpdb"
$BIN --db-path "$tmpdb" --album-name Corrupt || true
rm -f "$tmpdb" || true

echo "\n10) Malformed base-url"
$BIN --source stampworld --site-name 'Great-Britain' --base-url 'http:::/bad_url' --album-name GB || true

echo "\nCLI negative-path smoke tests complete"
