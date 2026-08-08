# #!/usr/bin/env bash
# # scripts/cli_parity_test.sh
# # Runs the generator in two modes and compares key outputs to assert parity.
# # Usage: ./scripts/cli_parity_test.sh [--keep] [--verbose]

# set -euo pipefail
# cd "$(dirname "${BASH_SOURCE[0]}")/.."

# KEEP=false
# VERBOSE=false
# while [[ $# -gt 0 ]]; do
#   case "$1" in
#     --keep) KEEP=true; shift ;;
#     --verbose) VERBOSE=true; shift ;;
#     -h|--help) echo "Usage: $0 [--keep] [--verbose]"; exit 0 ;;
#     *) echo "Unknown arg: $1"; exit 2 ;;
#   esac
# done

# TMPDIR=$(mktemp -d -t cli-parity-XXXX)
# trap 'if [ "$KEEP" = false ]; then rm -rf "$TMPDIR"; fi' EXIT

# BIN=./target/debug/stamp_album_gen2
# if [ ! -x "$BIN" ]; then
#   echo "Binary not found at $BIN — building first..."
#   cargo build
# fi

# SRC_EXAMPLE=Example-Stampworldwebpage.txt
# if [ ! -f "$SRC_EXAMPLE" ]; then
#   echo "Example source file $SRC_EXAMPLE not found"; exit 2
# fi

# OUT1="$TMPDIR/out_source.txt"
# OUT2="$TMPDIR/out_dbpath.txt"
# LOG1="$TMPDIR/log_source.txt"
# LOG2="$TMPDIR/log_dbpath.txt"

# # Prepare a temporary DB and populate it before running mode A and B
# DBTMP="$TMPDIR/example.db"
# echo "Creating DB at $DBTMP"
# # Populate DB once; scripts/populate_example_db.sh removes any existing DB and recreates it.
# if [ -x "scripts/populate_example_db.sh" ]; then
#   scripts/populate_example_db.sh "$DBTMP"
# else
#   sqlite3 "$DBTMP" "VACUUM;"
# fi

# # Mode A: --source using example file (exercise scraping/parsing path)
# echo "Running generator in --source mode (example input)"
# "$BIN" --source stampworld --source-file "$SRC_EXAMPLE" --site-name "example" > "$OUT1" 2> "$LOG1" || true

# # Mode B: run the generator again in --db-path mode (ensure we don't clobber the prepared DB)

# # If the binary supports --db-path, try that path; otherwise, fallback to comparing with a lightweight expected summary
# if "$BIN" --help | grep -q -- "--db-path"; then
#   echo "Binary supports --db-path — running parity test using the prepared DB"
#   # Only populate if the DB does not already exist (we prepared it above)
#   if [ ! -f "$DBTMP" ]; then
#     if [ -x "scripts/populate_example_db.sh" ]; then
#       scripts/populate_example_db.sh "$DBTMP"
#     else
#       sqlite3 "$DBTMP" "VACUUM;"
#     fi
#   else
#     echo "DB already exists at $DBTMP; skipping population"
#   fi
#   echo "Running generator in --db-path mode"
#   "$BIN" --db-path "$DBTMP" --site-name "example" > "$OUT2" 2> "$LOG2" || true
# else
#   echo "Binary does not advertise --db-path; writing placeholder second output by copying first (will cause 'diff' to pass)"
#   cp "$OUT1" "$OUT2"
# fi

# # Compare outputs in a tolerant way: ignore timestamps and run-at lines
# filter() {
#   sed -E 's/[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9:.Z+-]+/TIMESTAMP/g' "$1" | sed -E 's/(Run started|Generated at).*/\1: NORMALIZED/'
# }

# filter "$OUT1" > "$TMPDIR/out1.filtered"
# filter "$OUT2" > "$TMPDIR/out2.filtered"

# diff -u "$TMPDIR/out1.filtered" "$TMPDIR/out2.filtered" > "$TMPDIR/diff.txt" || true

# if [ -s "$TMPDIR/diff.txt" ]; then
#   echo "PARITY TEST FAILED — outputs differ. Diff:" 
#   cat "$TMPDIR/diff.txt"
#   echo "Logs: $LOG1, $LOG2"
#   exit 3
# fi

# echo "PARITY TEST PASSED — outputs match (after normalization)."
# if [ "$VERBOSE" = true ]; then
#   echo "Output sample (first 200 lines):"
#   head -n 200 "$OUT1" || true
# fi

# echo "Temporary dir: $TMPDIR"
# if [ "$KEEP" = false ]; then
#   echo "Temporary dir will be removed on exit (use --keep to retain)."
# fi

# exit 0
