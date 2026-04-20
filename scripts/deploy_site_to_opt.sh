#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

source_dir="${1:-build/site-release-bundle}"
deploy_path="${SITE_DEPLOY_PATH:-/opt/stampalbumgenerator2-site}"

if [[ ! -d "$source_dir" ]]; then
  echo "Source site bundle not found: $source_dir" >&2
  exit 1
fi

mkdir -p "$deploy_path"

if command -v rsync >/dev/null 2>&1; then
  rsync -a --delete "$source_dir/" "$deploy_path/"
else
  rm -rf "$deploy_path"
  mkdir -p "$deploy_path"
  cp -a "$source_dir/." "$deploy_path/"
fi

if [[ -n "${SITE_DEPLOY_OWNER:-}" ]]; then
  chown -R "$SITE_DEPLOY_OWNER" "$deploy_path"
fi

if [[ -n "${SITE_SYSTEMD_SERVICE:-}" ]] && command -v systemctl >/dev/null 2>&1; then
  systemctl restart "$SITE_SYSTEMD_SERVICE"
fi

echo "Site deployed to: $deploy_path"
