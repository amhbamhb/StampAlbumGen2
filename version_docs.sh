#!/usr/bin/env bash

# Backward-compatible entry point.
# Canonical implementation now lives in scripts/update_versions.sh.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec "$ROOT_DIR/scripts/update_versions.sh" "$@"