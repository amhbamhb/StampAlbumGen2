#!/bin/bash

# Script to display help for stamp_album_gen2 executable
EXECUTABLE="./target/release/stamp_album_gen2"

if [ -f "$EXECUTABLE" ]; then
    echo "--- Executing: $EXECUTABLE --help ---"
    "$EXECUTABLE" --help
else
    echo "Error: Executable not found at $EXECUTABLE" >&2
    exit 1
fi
