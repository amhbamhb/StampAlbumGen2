#!/usr/bin/env bash
set -euo pipefail

# Run four variants of the program and compare outputs/DBs.
# Usage: ./scripts/compare_runs.sh

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_ROOT"

OUT_BASE="outputs/compare_runs"
rm -rf "$OUT_BASE"
mkdir -p "$OUT_BASE"

ORIG_DB="Albums/Switzerland/Switzerland.db"
ORIG_DB_BACKUP="${ORIG_DB}.orig"

echo "Starting compare_runs.sh in $REPO_ROOT"

# Backup original DB if present (keep existing backup)
if [ -f "$ORIG_DB" ] && [ ! -f "$ORIG_DB_BACKUP" ]; then
  echo "Backing up existing DB to $ORIG_DB_BACKUP"
  cp -v "$ORIG_DB" "$ORIG_DB_BACKUP"
fi

clean_pages() {
  # remove generated files for the specific year so we force re-generation
  rm -f Albums/Switzerland/Switzerland-1976-1976.* || true
}

dump_db() {
  local dbpath="$1"
  local outdump="$2"
  if command -v sqlite3 >/dev/null 2>&1 && [ -f "$dbpath" ]; then
    sqlite3 "$dbpath" .dump > "$outdump"
  else
    echo "sqlite3 not found or DB missing; skipping DB dump for $dbpath" > "$outdump"
  fi
}

copy_album_output() {
  local dest="$1"
  mkdir -p "$dest"
  cp -a Albums/Switzerland "$dest/" || true
}

normalize_page() {
  local infile="$1" outfile="$2"
  # Remove generated timestamp and command-line/working-dir lines and normalize any date fragments
  sed -e '/^# Generated:/d' -e '/^# Command Line:/d' -e '/^# Working Directory:/d' \
      -e 's/^# \([A-Za-z0-9 _:-]*\) [0-9]\{4\}-[0-9]\{2\}-[0-9]\{2\}.*$/# \1<DATE>/g' \
      -e 's/UTC: [0-9TZ:+-]\+/UTC:<TIME>/g' \
      "$infile" > "$outfile" || true
}

db_normalize() {
  # Normalize DB dump by sorting INSERTs for deterministic comparisons
  local indir="$1"
  local infile="$indir/db_dump.sql"
  local outfile="$indir/db_dump_normalized.sql"
  if [ ! -f "$infile" ]; then
    echo "" > "$outfile"
    return
  fi

  # extract header (lines before first INSERT INTO stamps), inserts, and footer (after last INSERT INTO stamps)
  if grep -q "^INSERT INTO stamps" "$infile"; then
    awk 'BEGIN{ins_start=0;ins_end=0} /^INSERT INTO stamps/{if(!ins_start){ins_start=NR}; ins_end=NR} END{print ins_start,ins_end}' "$infile" | read ins_start ins_end || true
    if [ -z "${ins_start:-}" ] || [ -z "${ins_end:-}" ]; then
      # fallback: just copy
      cp -v "$infile" "$outfile"
      return
    fi
    # header
    head -n $((ins_start-1)) "$infile" > "$outfile"
    # sorted inserts
    sed -n "${ins_start},${ins_end}p" "$infile" | sort >> "$outfile"
    # footer
    tail -n +$((ins_end+1)) "$infile" >> "$outfile"
  else
    # nothing to sort, just copy
    cp -v "$infile" "$outfile"
  fi
}

run_and_capture() {
  # run_and_capture <name> <preserve_db_flag> <cmd...>
  local name="$1"
  local preserve_db_flag="$2"
  shift 2
  local cmd=("${@}")
  local dir="$OUT_BASE/$name"
  mkdir -p "$dir"
  echo "Running $name: ${cmd[*]}"
  # ensure clean pages for all runs
  clean_pages
  # Remove DB only when preserve_db_flag is empty/false (for scrape runs we want a fresh DB)
  if [ "$preserve_db_flag" != "preserve_db" ]; then
    rm -f "$ORIG_DB"
  fi
  # Run and capture
  "${cmd[@]}" 2>&1 | tee "$dir/run.log" || true
  # copy produced album
  copy_album_output "$dir"
  # copy DB if present
  if [ -f "$ORIG_DB" ]; then
    cp -v "$ORIG_DB" "$dir/Switzerland.db" || true
    dump_db "$ORIG_DB" "$dir/db_dump.sql"
    # produce normalized DB dump
    db_normalize "$dir"
  else
    echo "No DB present after run $name" > "$dir/db_dump.sql"
  fi
}

echo "Building release binary (cargo build --release)"
cargo build --release

### 1) cargo run --source stampworld (scrape)
run_and_capture run1 "" cargo run -- --source stampworld --site-name Switzerland --album-name Switzerland --start-year 1976 --end-year 1976

### 2) target/release/stamp_album_gen2 --source stampworld (scrape with release binary)
run_and_capture run2 "" target/release/stamp_album_gen2 --source stampworld --site-name Switzerland --album-name Switzerland --start-year 1976 --end-year 1976

# For the db-path runs we want to use the DB produced by run1 as the canonical DB
if [ -f "$OUT_BASE/run1/Switzerland.db" ]; then
  echo "Using run1 DB as canonical DB for --db-path runs"
  cp -v "$OUT_BASE/run1/Switzerland.db" "$ORIG_DB"
else
  echo "Warning: run1 DB not found; --db-path runs will create/use empty DBs"
fi

### 3) target/release/stamp_album_gen2 --db-path Albums/Switzerland/Switzerland.db
# preserve DB for --db-path runs
run_and_capture run3 preserve_db target/release/stamp_album_gen2 --db-path Albums/Switzerland/Switzerland.db --site-name Switzerland --album-name Switzerland --start-year 1976 --end-year 1976

### 4) cargo run -- --db-path Albums/Switzerland/Switzerland.db
run_and_capture run4 preserve_db cargo run -- --db-path Albums/Switzerland/Switzerland.db --site-name Switzerland --album-name Switzerland --start-year 1976 --end-year 1976

echo "Normalizing generated page files and producing diffs"
mkdir -p "$OUT_BASE/diffs"

PAGE_PATH="Albums/Switzerland/Switzerland-1976-1976.txt"
for i in 1 2 3 4; do
  if [ -f "$OUT_BASE/run$i/Albums/Switzerland/Switzerland-1976-1976.txt" ]; then
    normalize_page "$OUT_BASE/run$i/Albums/Switzerland/Switzerland-1976-1976.txt" "$OUT_BASE/run$i/normalized_page.txt"
  else
    echo "" > "$OUT_BASE/run$i/normalized_page.txt"
  fi
done

# Diff pages against run1
for i in 2 3 4; do
  diff -u "$OUT_BASE/run1/normalized_page.txt" "$OUT_BASE/run$i/normalized_page.txt" > "$OUT_BASE/diffs/run1_vs_run${i}_page.diff" || true
  # compare normalized DB dumps (if present)
  diff -u "$OUT_BASE/run1/db_dump_normalized.sql" "$OUT_BASE/run$i/db_dump_normalized.sql" > "$OUT_BASE/diffs/run1_vs_run${i}_db.diff" || true
done

REPORT="$OUT_BASE/report.txt"
echo "Compare Runs Report" > "$REPORT"
echo "Repository: $REPO_ROOT" >> "$REPORT"
echo "" >> "$REPORT"
for i in 1 2 3 4; do
  echo "Run $i log: $OUT_BASE/run$i/run.log" >> "$REPORT"
  echo "Run $i db dump: $OUT_BASE/run$i/db_dump.sql" >> "$REPORT"
  echo "Run $i page (normalized): $OUT_BASE/run$i/normalized_page.txt" >> "$REPORT"
  echo "Run $i db dump (normalized): $OUT_BASE/run$i/db_dump_normalized.sql" >> "$REPORT"
  echo "" >> "$REPORT"
done

echo "Diff summaries:" >> "$REPORT"
for i in 2 3 4; do
  echo "run1 vs run$i page diff: $(wc -l < "$OUT_BASE/diffs/run1_vs_run${i}_page.diff" 2>/dev/null || echo 0) lines" >> "$REPORT"
  echo "run1 vs run$i db diff:   $(wc -l < "$OUT_BASE/diffs/run1_vs_run${i}_db.diff" 2>/dev/null || echo 0) lines" >> "$REPORT"
  echo "" >> "$REPORT"
done

echo "Detailed diffs are under $OUT_BASE/diffs/"
echo "Report written to $REPORT"

# restore original DB if we backed it up
if [ -f "$ORIG_DB_BACKUP" ]; then
  echo "Restoring original DB from $ORIG_DB_BACKUP"
  cp -v "$ORIG_DB_BACKUP" "$ORIG_DB"
fi

echo "Done."
