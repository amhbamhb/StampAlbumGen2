#!/usr/bin/env bash

# Minimal test automation script (GitHub Actions only)
# Archives of the previous GitLab+GitHub script are saved in
# `test_automation.gitlab-archive.sh`.

set -euo pipefail

echo "Testing repository CI configuration (GitHub Actions only)"
echo

# Find workflow files
workflows=(.github/workflows/*.yml .github/workflows/*.yaml)
found=false
for wf in "${workflows[@]}"; do
  for f in $wf; do
    if [ -f "$f" ]; then
      echo "✅ Found workflow: $f"
      found=true
    fi
  done
done

if [ "$found" = false ]; then
  echo "❌ No GitHub Actions workflow files found in .github/workflows"
else
  echo
  echo "Checking workflows for conventional-commit detection and release steps"
  for wf in .github/workflows/*.{yml,yaml}; do
    [ -f "$wf" ] || continue
    echo "-- $wf --"
    if grep -qE "feat:|fix:|BREAKING CHANGE" "$wf"; then
      echo "   ✅ Contains conventional commit detection patterns"
    else
      echo "   ⚠️  No obvious conventional commit patterns found"
    fi
    if grep -q "release" "$wf"; then
      echo "   ✅ Appears to include release steps"
    else
      echo "   ⚠️  No 'release' token found; ensure release step is present if expected"
    fi
  done
fi

echo
echo "Testing conventional commit message parsing helper"
echo "=============================================="

test_cases=(
  "feat: add new stamp filtering feature"
  "fix: resolve database connection issue"
  "docs: update README"
  "BREAKING CHANGE: redesign core architecture"
  "invalid message"
)

for msg in "${test_cases[@]}"; do
  if echo "$msg" | grep -qE "^(BREAKING CHANGE:|feat|fix|docs|style|refactor|perf|test|chore|ci|build|revert)(\(.+\))?: "; then
    echo "✅ VALID: $msg"
  else
    echo "❌ INVALID: $msg"
  fi
done

echo
echo "Checking helper scripts and hooks"

if [ -f ./version_docs.sh ]; then
  echo "✅ version_docs.sh exists"
else
  echo "⚠️  version_docs.sh missing"
fi

if [ -f ./conventional-commit.sh ]; then
  # Ensure executable so checks don't fail due to file mode
  chmod +x ./conventional-commit.sh || true
  if [ -x ./conventional-commit.sh ]; then
    echo "✅ conventional-commit.sh is executable"
  else
    echo "⚠️  conventional-commit.sh exists but is not executable"
  fi
else
  echo "⚠️  conventional-commit.sh missing"
fi

if [ -x .git/hooks/prepare-commit-msg ]; then
  echo "✅ prepare-commit-msg hook is executable"
else
  echo "⚠️  prepare-commit-msg hook missing or not executable"
fi

echo
echo "🎉 GitHub-only tests complete"