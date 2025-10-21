#!/usr/bin/env sh
# Helper to run the application with debug logging without flooding VS Code terminal
# Usage: ./scripts/run_debug_no_vscode.sh -- --source stampworld --site-name Switzerland --album-name Switzerland --start-year 1976 --end-year 1976

LOGS_DIR="Logs"
mkdir -p "$LOGS_DIR"
TS=$(date +"%Y%m%d-%H%M%S")
LOGFILE="$LOGS_DIR/debug-run-$TS.log"

# The `--` after the script's flags allows passing args to cargo run
# We run cargo in a nohup background so it won't crash the VS Code terminal
# and we capture all output (stdout+stderr) into the logfile.

echo "Starting background run with debug logging. Log file: $LOGFILE"
nohup sh -c "RUST_LOG=debug cargo run -- $*" > "$LOGFILE" 2>&1 &
echo "Started (PID $!) — tail -f $LOGFILE to follow progress if you like." 
