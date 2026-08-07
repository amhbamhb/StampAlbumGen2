#!/usr/bin/env bash
# Minimal conventional commit validator
set -euo pipefail

usage() {
  cat <<EOF
Usage: $0 "<commit-message>"
Returns 0 if the message matches Conventional Commits or BREAKING CHANGE, else non-zero.
EOF
}

if [ "$#" -lt 1 ]; then
  usage
  exit 2
fi

msg="$1"

if echo "$msg" | grep -qE "^(BREAKING CHANGE: |(feat|fix|docs|style|refactor|perf|test|chore|ci|build|revert)(\(.+\))?: )"; then
  exit 0
else
  exit 1
fi
