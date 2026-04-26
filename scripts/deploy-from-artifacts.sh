#!/usr/bin/env bash
# deploy-from-artifacts.sh — install StampAlbumGen2 from two CI artifact ZIPs.
#
# Usage:
#   deploy-from-artifacts.sh --app <app-artifact.zip> --web <web-artifact.zip> [--deploy-dir <path>]
#
# The app artifact is produced by the stamp_album_gen2 CI pipeline.
# The web artifact is produced by the stampalbumgenerator2-site CI pipeline.
#
# Both ZIPs are downloaded from GitLab (Job Artifacts > Download).
# Typical flow:
#   1. Download the two artifact ZIPs from each project's latest pipeline.
#   2. Run this script on the deployment host.
#
# Prerequisites on the host: bash, unzip, rsync, sha256sum, systemctl (optional).

set -euo pipefail

DEPLOY_DIR="/opt/stampalbumgenerator2-site"
TMP_ROOT="/home/admin/stamp-files"
APP_ZIP=""
WEB_ZIP=""

usage() {
  echo "Usage: $0 --app <app-artifact.zip> --web <web-artifact.zip> [--deploy-dir <path>]"
  echo "  --app          Path to the stamp_album_gen2 CI artifact ZIP"
  echo "  --web          Path to the stampalbumgenerator2-site CI artifact ZIP"
  echo "  --deploy-dir   Deployment root (default: $DEPLOY_DIR)"
  exit 1
}

log_info()    { echo "[INFO]  $*"; }
log_ok()      { echo "[OK]    $*"; }
log_error()   { echo "[ERROR] $*" >&2; }
log_section() { echo; echo "=== $* ==="; }

# ── Argument parsing ──────────────────────────────────────────────────────────
while [[ $# -gt 0 ]]; do
  case "$1" in
    --app)        APP_ZIP="$2"; shift 2 ;;
    --web)        WEB_ZIP="$2"; shift 2 ;;
    --deploy-dir) DEPLOY_DIR="$2"; shift 2 ;;
    -h|--help)    usage ;;
    *) log_error "Unknown argument: $1"; usage ;;
  esac
done

[[ -z "$APP_ZIP" || -z "$WEB_ZIP" ]] && { log_error "Both --app and --web are required."; usage; }
[[ ! -f "$APP_ZIP" ]] && { log_error "App artifact not found: $APP_ZIP"; exit 1; }
[[ ! -f "$WEB_ZIP" ]] && { log_error "Web artifact not found: $WEB_ZIP"; exit 1; }

# ── Extract to temp dirs ──────────────────────────────────────────────────────
if [[ -d "$TMP_ROOT" ]] || mkdir -p "$TMP_ROOT"; then
  TMPDIR="$(mktemp -d "$TMP_ROOT/deploy-XXXXXX")"
else
  log_info "Could not create $TMP_ROOT; falling back to system tmp"
  TMPDIR="$(mktemp -d)"
fi
trap 'rm -rf "$TMPDIR"' EXIT

APP_EXTRACT="$TMPDIR/app"
WEB_EXTRACT="$TMPDIR/web"
mkdir -p "$APP_EXTRACT" "$WEB_EXTRACT"

log_section "Extracting artifacts"
log_info "App artifact:  $APP_ZIP"
unzip -q "$APP_ZIP" -d "$APP_EXTRACT"
log_ok "App artifact extracted"

log_info "Web artifact:  $WEB_ZIP"
unzip -q "$WEB_ZIP" -d "$WEB_EXTRACT"
log_ok "Web artifact extracted"

# Both ZIPs unpack to an artifacts/ subdirectory
APP_ARTIFACTS="$APP_EXTRACT/artifacts"
WEB_ARTIFACTS="$WEB_EXTRACT/artifacts"

[[ ! -d "$APP_ARTIFACTS" ]] && { log_error "Expected artifacts/ dir inside app ZIP — got: $(ls "$APP_EXTRACT")"; exit 1; }
[[ ! -d "$WEB_ARTIFACTS" ]] && { log_error "Expected artifacts/ dir inside web ZIP — got: $(ls "$WEB_EXTRACT")"; exit 1; }

# ── Verify binaries exist ─────────────────────────────────────────────────────
APP_BIN="$APP_ARTIFACTS/stamp_album_gen2"
WEB_BIN="$WEB_ARTIFACTS/bin/stampalbumgenerator2-site"

[[ ! -f "$APP_BIN" ]] && { log_error "App binary not found at $APP_BIN"; exit 1; }
[[ ! -f "$WEB_BIN" ]] && { log_error "Web binary not found at $WEB_BIN"; exit 1; }

APP_WIN="$APP_ARTIFACTS/stamp_album_gen2.exe"

# ── Validate checksums ────────────────────────────────────────────────────────
log_section "Verifying checksums"
if [[ -f "$APP_ARTIFACTS/stamp_album_gen2.sha256" ]]; then
  expected_sha="$(cat "$APP_ARTIFACTS/stamp_album_gen2.sha256" | awk '{print $1}')"
  actual_sha="$(sha256sum "$APP_BIN" | awk '{print $1}')"
  if [[ "$expected_sha" == "$actual_sha" ]]; then
    log_ok "App binary checksum OK ($actual_sha)"
  else
    log_error "App binary checksum mismatch!"
    log_error "  expected: $expected_sha"
    log_error "  actual:   $actual_sha"
    exit 1
  fi
else
  log_info "No app checksum file found, skipping verification"
fi

if [[ -f "$WEB_ARTIFACTS/stampalbumgenerator2-site.sha256" ]]; then
  expected_sha="$(cat "$WEB_ARTIFACTS/stampalbumgenerator2-site.sha256" | awk '{print $1}')"
  actual_sha="$(sha256sum "$WEB_BIN" | awk '{print $1}')"
  if [[ "$expected_sha" == "$actual_sha" ]]; then
    log_ok "Web binary checksum OK ($actual_sha)"
  else
    log_error "Web binary checksum mismatch!"
    log_error "  expected: $expected_sha"
    log_error "  actual:   $actual_sha"
    exit 1
  fi
else
  log_info "No web checksum file found, skipping verification"
fi

# ── Create deployment directories ─────────────────────────────────────────────
log_section "Deploying to $DEPLOY_DIR"
mkdir -p \
  "$DEPLOY_DIR/bin" \
  "$DEPLOY_DIR/static/downloads/current" \
  "$DEPLOY_DIR/config"

# ── Install app binary and Windows binary ─────────────────────────────────────
cp "$APP_BIN" "$DEPLOY_DIR/bin/stamp_album_gen2"
chmod 755 "$DEPLOY_DIR/bin/stamp_album_gen2"
log_ok "Installed stamp_album_gen2"

if [[ -f "$APP_WIN" ]]; then
  cp "$APP_WIN" "$DEPLOY_DIR/bin/stamp_album_gen2.exe"
  chmod 755 "$DEPLOY_DIR/bin/stamp_album_gen2.exe"
  log_ok "Installed stamp_album_gen2.exe (Windows)"
fi

# ── Install website binary ────────────────────────────────────────────────────
cp "$WEB_BIN" "$DEPLOY_DIR/bin/stampalbumgenerator2-site"
chmod 755 "$DEPLOY_DIR/bin/stampalbumgenerator2-site"
log_ok "Installed stampalbumgenerator2-site"

# ── Copy app version file ─────────────────────────────────────────────────────
if [[ -f "$WEB_ARTIFACTS/APP_VERSION" ]]; then
  cp "$WEB_ARTIFACTS/APP_VERSION" "$DEPLOY_DIR/"
  log_ok "Installed APP_VERSION: $(cat "$WEB_ARTIFACTS/APP_VERSION")"
fi

# ── Copy config files ─────────────────────────────────────────────────────────
for cfg in config.toml config_full_example.toml; do
  if [[ -f "$APP_ARTIFACTS/$cfg" ]]; then
    cp "$APP_ARTIFACTS/$cfg" "$DEPLOY_DIR/config/"
    log_ok "Installed $cfg"
  fi
done

# ── Sync static files ─────────────────────────────────────────────────────────
if [[ -d "$WEB_ARTIFACTS/static" ]]; then
  rsync -a --delete "$WEB_ARTIFACTS/static/" "$DEPLOY_DIR/static/"
  log_ok "Static files synced"
fi

# ── Publish downloadable binaries and checksums ───────────────────────────────
DOWNLOAD_DIR="$DEPLOY_DIR/static/downloads/current"
cp "$DEPLOY_DIR/bin/stamp_album_gen2" "$DOWNLOAD_DIR/"
[[ -f "$DEPLOY_DIR/bin/stamp_album_gen2.exe" ]] && cp "$DEPLOY_DIR/bin/stamp_album_gen2.exe" "$DOWNLOAD_DIR/" || true

{
  sha256sum "$DOWNLOAD_DIR/stamp_album_gen2" | awk '{printf "%s  stamp_album_gen2\n", $1}'
  [[ -f "$DOWNLOAD_DIR/stamp_album_gen2.exe" ]] && sha256sum "$DOWNLOAD_DIR/stamp_album_gen2.exe" | awk '{printf "%s  stamp_album_gen2.exe\n", $1}' || true
} > "$DOWNLOAD_DIR/SHA256SUMS.txt"
log_ok "SHA256SUMS.txt written to $DOWNLOAD_DIR"

# ── Restart service if systemd unit exists ────────────────────────────────────
log_section "Service"
UNIT="stampalbumgenerator2-site"
if systemctl is-active --quiet "$UNIT" 2>/dev/null || systemctl is-enabled --quiet "$UNIT" 2>/dev/null; then
  log_info "Restarting $UNIT..."
  systemctl restart "$UNIT"
  sleep 1
  status="$(systemctl is-active "$UNIT" 2>/dev/null || true)"
  if [[ "$status" == "active" ]]; then
    log_ok "Service restarted and active"
  else
    log_error "Service restart attempted but status is: $status"
    exit 1
  fi
else
  log_info "No systemd unit '$UNIT' found — skipping service restart"
fi

# ── Summary ───────────────────────────────────────────────────────────────────
log_section "Deploy complete"
echo "Deploy dir:  $DEPLOY_DIR"
echo "Binaries:"
ls -lh "$DEPLOY_DIR/bin/"
echo ""
echo "SHA256SUMS:"
cat "$DOWNLOAD_DIR/SHA256SUMS.txt"
