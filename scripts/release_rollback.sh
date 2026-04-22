#!/usr/bin/env bash
# scripts/release_rollback.sh
# Usage: scripts/release_rollback.sh <tag> [--revert-commit]
# Example: scripts/release_rollback.sh v0.22.20

set -euo pipefail
TAG=${1:-}
REVERT_COMMIT=false
if [ "$TAG" = "" ]; then
  echo "Usage: $0 <tag> [--revert-commit]" >&2
  exit 2
fi
if [ "${2:-}" = "--revert-commit" ]; then
  REVERT_COMMIT=true
fi

echo "Preparing to rollback tag: $TAG"

# Delete local tag if it exists
if git rev-parse --verify "refs/tags/$TAG" >/dev/null 2>&1; then
  echo "Deleting local tag $TAG"
  git tag -d "$TAG"
else
  echo "Local tag $TAG not found"
fi

# Delete remote tag on origin
if git ls-remote --exit-code --tags origin "refs/tags/$TAG" >/dev/null 2>&1; then
  echo "Deleting remote tag origin/$TAG"
  git push origin :refs/tags/$TAG
else
  echo "Remote tag origin/$TAG not found"
fi

if [ "$REVERT_COMMIT" = true ]; then
  # Try to find the commit that created the bump (heuristic: recent commit message contains 'bump version' or 'chore(release)')
  COMMIT=$(git log --grep="bump version\|chore(release)\|chore: bump version" -n 1 --pretty=format:%H || true)
  if [ -n "$COMMIT" ]; then
    echo "Reverting commit $COMMIT"
    git revert --no-edit "$COMMIT"
    echo "Pushing revert to origin"
    git push origin HEAD
  else
    echo "No suitable bump commit found to revert"
  fi
fi

echo "Rollback complete. Please verify repository state and inform team if needed."
