#!/bin/bash



# Set the working directory as specified by the user

TARGET_DIR="/media/ray/2026-01-11/Rust/Rust/StampAlbumGen2"

cd "$TARGET_DIR"



echo "--- Executing command from $TARGET_DIR ---"



# --- Test 1: Spain, Single Year (1850) ---

echo "Attempting Test 1: Spain for year 1850..."

echo "Command: ./target/release/stamp_album_gen2 --source stampworld --site-name \\"Spain\\" --album-name \\"Spain\\" --start-year 1850 --end-year 1850"

echo "---------------------------------------------"



# Execute Test 1

./target/release/stamp_album_gen2 \\

--source stampworld \\

--site-name "Spain" \\

--album-name "Spain" \\

--start-year 1850 \\

--end-year 1850



echo "--- Test 1 Finished ---"



# --- Test 2: Spain, Range (1850-1855) ---

echo "Attempting Test 2: Spain for years 1850-1855..."

echo "Command: ./target/release/stamp_album_gen2 --source stampworld --site-name \\"Spain\\" --album-name \\"Spain\\" --start-year 1850 --end-year 1855"

echo "---------------------------------------------"


# Execute Test 2

./target/release/stamp_album_gen2 \\

--source stampworld \\

--site-name "Spain" \\

--album-name "Spain" \\

--start-year 1850 \\

--end-year 1855



echo "--- Test 2 Finished ---"
