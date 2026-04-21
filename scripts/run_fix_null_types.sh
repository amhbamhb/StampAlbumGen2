#!/usr/bin/env bash
# Safe wrapper to run scripts/fix_null_types.sql across multiple album DBs.
# Supports --dry-run (default), --apply to actually run the SQL, --backup to make backups,
# --pattern to limit to a glob (e.g., "Switzerland"). Logs written to Logs/run_fix_null_types-<timestamp>.log

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="${SCRIPT_DIR}/.."
LOG_DIR="${ROOT_DIR}/Logs"
SQL_SCRIPT="${ROOT_DIR}/scripts/fix_null_types.sql"
DRY_RUN=true
DO_BACKUP=false
PATTERN="*"

usage(){
  cat <<EOF
Usage: $(basename "$0") [--dry-run|--apply] [--backup] [--pattern <glob>] [--log-dir <dir>]

Options:
  --dry-run    : Show what would change without applying (default)
  --apply      : Apply the migration SQL to matching DBs
  --backup     : Create a timestamped backup for each DB before applying
  --pattern    : Glob to limit which album names to process (default: *)
  --log-dir    : Directory to write logs (default: Logs/ in repo root)
  -h, --help   : Show this help

Example:
  $(basename "$0") --dry-run --backup --pattern "Switzerland"
EOF
}

# Parse args
while [[ $# -gt 0 ]]; do
  case "$1" in
    --dry-run)
      DRY_RUN=true; shift ;;
    --apply)
      DRY_RUN=false; shift ;;
    --backup)
      DO_BACKUP=true; shift ;;
    --pattern)
      PATTERN="$2"; shift 2 ;;
    --log-dir)
      LOG_DIR="$2"; shift 2 ;;
    -h|--help)
      usage; exit 0 ;;
    *)
      echo "Unknown arg: $1"; usage; exit 1 ;;
  esac
done

mkdir -p "$LOG_DIR"
TS=$(date +%Y%m%d%H%M%S)
RUN_LOG="$LOG_DIR/run_fix_null_types-$TS.log"
exec 3>&1 1>>"$RUN_LOG" 2>&1

echo "Run started: $(date)" >&3
echo "Script dir: $SCRIPT_DIR" >&3
echo "SQL: $SQL_SCRIPT" >&3
echo "Dry run: $DRY_RUN" >&3
echo "Backup: $DO_BACKUP" >&3
echo "Pattern: $PATTERN" >&3
echo "Logging to: $RUN_LOG" >&3

# Build list of DBs
shopt -s nullglob
DBS=("${ROOT_DIR}/Albums/${PATTERN}"/*.db)
shopt -u nullglob

if [ ${#DBS[@]} -eq 0 ]; then
  echo "No DB files found using pattern '${PATTERN}' under ${ROOT_DIR}/Albums" >&3
  exit 0
fi

summary_file="$LOG_DIR/summary-run_fix_null_types-$TS.txt"
echo "DB|NULL_type_count|month_null_count|day_null_count|duplicate_groups|action" > "$summary_file"

for db in "${DBS[@]}"; do
  echo "---" >&3
  echo "Processing DB: $db" >&3
  # Basic checks
  if [ ! -f "$db" ]; then
    echo "Skipping missing file: $db" >&3
    continue
  fi
  # Skip known non-stamp DBs by name
  base_name=$(basename "$db")
  if [ "$base_name" = "wikipedia_cache.db" ]; then
  echo "Skipping known non-stamp DB: $db" >&3
  echo "$db|SKIPPED|SKIPPED|SKIPPED|SKIPPED|skipped_nonstamp" >> "$summary_file"
    continue
  fi

  # Ensure DB has a stamps table before attempting SQL
  has_stamps=$(sqlite3 "$db" "SELECT name FROM sqlite_master WHERE type='table' AND name='stamps' LIMIT 1;" 2>/dev/null || echo "")
  if [ -z "$has_stamps" ]; then
  echo "Skipping DB (no stamps table): $db" >&3
  echo "$db|NO_STAMPS_TABLE|NO_STAMPS_TABLE|NO_STAMPS_TABLE|NO_STAMPS_TABLE|skipped_no_table" >> "$summary_file"
    continue
  fi

  # Count NULL type rows
  null_count=$(sqlite3 "$db" "SELECT COUNT(*) FROM stamps WHERE type IS NULL;" 2>/dev/null || echo "ERR")
  # Count NULL month/day rows (for informational update)
  month_null_count=$(sqlite3 "$db" "SELECT COUNT(*) FROM stamps WHERE month IS NULL OR month = '';" 2>/dev/null || echo "ERR")
  day_null_count=$(sqlite3 "$db" "SELECT COUNT(*) FROM stamps WHERE day IS NULL OR day = '';" 2>/dev/null || echo "ERR")
  # Count duplicate groups by (country,series,sg_number)
  dup_count=$(sqlite3 "$db" "SELECT COUNT(*) FROM (SELECT lower(coalesce(country,''))||'|'||coalesce(series,'')||'|'||coalesce(sg_number,'') AS k FROM stamps GROUP BY k HAVING COUNT(*)>1);" 2>/dev/null || echo "ERR")
  echo "DB: $db" >&3
  echo "  NULL-type rows: $null_count" >&3
  echo "  Duplicate groups: $dup_count" >&3

  action_taken="none"
  if [ "$DRY_RUN" = true ]; then
    echo "  Dry-run: would run migration script if requested." >&3
    action_taken="dry-run"
  else
    # Backup if requested
    if [ "$DO_BACKUP" = true ]; then
      bak="${db}.bak.$TS"
      echo "  Backing up to: $bak" >&3
      cp -v "$db" "$bak" >&3 || { echo "Backup failed for $db" >&3; continue; }
    fi
    # Apply SQL script
    if [ ! -f "$SQL_SCRIPT" ]; then
      echo "  SQL script not found: $SQL_SCRIPT" >&3
      action_taken="error_no_sql"
    else
      echo "  Pre-migration: populate month/day from description/context if missing" >&3
      # Run the Python helper in dry-run or apply mode to propose or perform month/day extraction
      PY_SCRIPT="${ROOT_DIR}/scripts/populate_month_day.py"
      if [ "$DRY_RUN" = true ]; then
        echo "  (dry-run) Running populate_month_day.py --db '$db' --dry-run" >&3
        if python3 "$PY_SCRIPT" --db "$db" --dry-run 2>&1 | sed 's/^/    /' >&3; then
          echo "  populate_month_day.py dry-run completed for $db" >&3
        else
          echo "  populate_month_day.py dry-run failed for $db" >&3
        fi
      else
        echo "  Running populate_month_day.py --db '$db' to populate missing month/day" >&3
        if python3 "$PY_SCRIPT" --db "$db" 2>&1 | sed 's/^/    /' >&3; then
          echo "  populate_month_day.py applied for $db" >&3
        else
          echo "  populate_month_day.py failed for $db" >&3
        fi
      fi
      # After populating month/day attempt to ensure month/day are at least non-NULL (coalesce)
      if sqlite3 "$db" "BEGIN; UPDATE stamps SET month = COALESCE(month, ''); UPDATE stamps SET day = COALESCE(day, ''); COMMIT;" 2>/dev/null; then
        echo "  month/day normalization step completed for $db" >&3
      else
        echo "  month/day normalization step failed for $db" >&3
      fi
      echo "  Applying SQL script..." >&3
      if sqlite3 "$db" ".read $SQL_SCRIPT" 2>&3; then
        echo "  Applied migration to $db" >&3
        action_taken="applied"
      else
        echo "  Migration failed for $db" >&3
        action_taken="error_apply"
      fi
    fi
  fi
  echo "$db|$null_count|$month_null_count|$day_null_count|$dup_count|$action_taken" >> "$summary_file"
done

# Print summary to user (to stdout)
exec 1>&3 3>&-
echo "Run completed: $(date)"
echo "Log file: $RUN_LOG"
echo "Summary file: $summary_file"
cat "$summary_file"

exit 0
