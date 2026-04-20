#!/usr/bin/env bash
set -euo pipefail

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PORT="${PORT:-4205}"

find_listener_pid() {
  lsof -nP -t -iTCP:"$PORT" -sTCP:LISTEN 2>/dev/null | head -n 1 || true
}

is_site_process() {
  local pid="$1"
  local cmd
  cmd="$(ps -p "$pid" -o command= 2>/dev/null || true)"
  [[ "$cmd" == *"stampalbumgenerator2-site"* || "$cmd" == *"cargo run"* || "$cmd" == *"cargo-watch"* || "$cmd" == *"cargo watch"* ]]
}

if ! command -v cargo-watch >/dev/null 2>&1; then
  echo "cargo-watch is not installed. Installing now..."
  cargo install cargo-watch
fi

existing_pid="$(find_listener_pid)"
if [[ -n "$existing_pid" ]]; then
  if is_site_process "$existing_pid"; then
    echo "Stopping existing site process on port $PORT (PID $existing_pid)..."
    kill "$existing_pid"
  else
    echo "Port $PORT is already in use by a different process (PID $existing_pid)."
    echo "Set a different port, for example: PORT=4300 ./stampalbumgenerator2-site/scripts/dev.sh"
    exit 1
  fi
fi

cd "$PROJECT_DIR"
echo "Starting live development server on http://localhost:$PORT"
exec cargo watch -w src -w static -x "run"
