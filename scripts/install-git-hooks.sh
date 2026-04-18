#!/usr/bin/env bash

# Configure repository to use tracked hooks in .githooks.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

if [[ ! -f ".githooks/pre-commit" ]]; then
    echo "Error: .githooks/pre-commit not found" >&2
    exit 1
fi

chmod +x .githooks/pre-commit

git config core.hooksPath .githooks

echo "Installed hooks: core.hooksPath -> .githooks"
echo "Pre-commit auto version update is now enabled."
echo "Use SKIP_VERSION_BUMP=1 git commit ... to bypass once if needed."
