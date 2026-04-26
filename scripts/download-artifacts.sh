#!/usr/bin/env bash
# download-artifacts.sh — download latest app + website GitLab artifacts for deployment.

set -euo pipefail

GITLAB_HOST_DEFAULT="gitinfo.laserspider.info"
APP_PROJECT_ID_DEFAULT="54"
WEB_PROJECT_ID_DEFAULT="60"
APP_PROJECT_PATH="rust-apps/rust-stamp_alblum_generate"
WEB_PROJECT_PATH="rust-apps/stampalbumgenerator2-site"
OUT_DIR="/home/admin/stamp-site"
APP_REF="master"
WEB_REF="main"
APP_PIPELINE_ID=""
WEB_PIPELINE_ID=""

usage() {
  cat <<EOF
Usage: $0 [options]

Options:
  --out-dir <path>          Output directory for ZIP files (default: $OUT_DIR)
  --app-ref <branch>        App ref for latest pipeline lookup (default: $APP_REF)
  --web-ref <branch>        Website ref for latest pipeline lookup (default: $WEB_REF)
  --app-pipeline <id>       Specific app pipeline ID (optional)
  --web-pipeline <id>       Specific website pipeline ID (optional)
  -h, --help                Show this help

Notes:
  - Requires authenticated glab CLI.
  - Resolves project IDs dynamically by path to avoid URL encoding issues.
EOF
}

log_info() { echo "[INFO]  $*"; }
log_ok()   { echo "[OK]    $*"; }
log_err()  { echo "[ERROR] $*" >&2; }

api_get_json() {
  local endpoint="$1"
  local response
  if ! response="$(glab api "$endpoint" 2>&1)"; then
    log_err "GitLab API request failed: $endpoint"
    log_err "glab response: $response"
    if [[ "$response" == *"404 Project Not Found"* ]]; then
      log_err "Current glab token is authenticated, but it does not have access to the requested project or resource."
    fi
    return 1
  fi
  if ! printf '%s' "$response" | jq -e . >/dev/null 2>&1; then
    log_err "GitLab API returned non-JSON output for: $endpoint"
    log_err "glab response: $response"
    if [[ "$response" == *"404 Project Not Found"* ]]; then
      log_err "Current glab token is authenticated, but it does not have access to the requested project or resource."
    fi
    return 1
  fi
  printf '%s\n' "$response"
}

require_project_access() {
  local project_id="$1"
  local project_label="$2"
  local project_path="$3"

  if ! api_get_json "projects/$project_id" >/dev/null; then
    log_err "The current GitLab credentials cannot access the $project_label project: $project_path (project ID $project_id)"
    log_err "If 'glab auth status' shows a project bot such as project_54_bot_..., re-authenticate with a token that can read both projects 54 and 60."
    exit 1
  fi
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --out-dir)      OUT_DIR="$2"; shift 2 ;;
    --app-ref)      APP_REF="$2"; shift 2 ;;
    --web-ref)      WEB_REF="$2"; shift 2 ;;
    --app-pipeline) APP_PIPELINE_ID="$2"; shift 2 ;;
    --web-pipeline) WEB_PIPELINE_ID="$2"; shift 2 ;;
    -h|--help)      usage; exit 0 ;;
    *) log_err "Unknown argument: $1"; usage; exit 1 ;;
  esac
done

export GITLAB_HOST="${GITLAB_HOST:-$GITLAB_HOST_DEFAULT}"
export GLAB_PAGER="${GLAB_PAGER:-cat}"

command -v glab >/dev/null 2>&1 || { log_err "glab not found in PATH"; exit 1; }
command -v jq >/dev/null 2>&1 || { log_err "jq not found in PATH"; exit 1; }

mkdir -p "$OUT_DIR" || { log_err "Could not create output directory: $OUT_DIR"; exit 1; }

# Resolve numeric project IDs dynamically (avoids URL path encoding issues)
resolve_project_id() {
  local path="$1"
  local fallback_id="$2"
  local encoded
  encoded="$(printf '%s' "$path" | sed 's#/#%2F#g')"
  local id
  local response
  if ! response="$(api_get_json "projects/$encoded" 2>&1)"; then
    if [[ -n "$fallback_id" ]]; then
      log_info "Path lookup failed for $path; falling back to known project ID $fallback_id" >&2
      echo "$fallback_id"
      return 0
    fi
    log_err "Could not access project: $path"
    log_err "glab response: $response"
    log_err "Current GitLab credentials likely do not have access to this project."
    exit 1
  fi
  id="$(printf '%s' "$response" | jq -r '.id // empty')"
  if [[ -z "$id" ]]; then
    if [[ -n "$fallback_id" ]]; then
      log_info "Unexpected project payload for $path; falling back to known project ID $fallback_id" >&2
      echo "$fallback_id"
      return 0
    fi
    log_err "Could not resolve project ID for: $path"
    log_err "GitLab returned an unexpected response while resolving the project."
    exit 1
  fi
  echo "$id"
}

log_info "Resolving project IDs..."
APP_PROJECT_ID="$(resolve_project_id "${APP_PROJECT_PATH:-rust-apps/rust-stamp_alblum_generate}" "${APP_PROJECT_ID_DEFAULT:-}")"
WEB_PROJECT_ID="$(resolve_project_id "${WEB_PROJECT_PATH:-rust-apps/stampalbumgenerator2-site}" "${WEB_PROJECT_ID_DEFAULT:-}")"
[[ "$APP_PROJECT_ID" =~ ^[0-9]+$ ]] || { log_err "Resolved app project ID is not numeric: $APP_PROJECT_ID"; exit 1; }
[[ "$WEB_PROJECT_ID" =~ ^[0-9]+$ ]] || { log_err "Resolved web project ID is not numeric: $WEB_PROJECT_ID"; exit 1; }
log_ok "App project ID=$APP_PROJECT_ID  Web project ID=$WEB_PROJECT_ID"

log_info "Verifying project access with current GitLab credentials..."
require_project_access "$APP_PROJECT_ID" "app" "${APP_PROJECT_PATH:-rust-apps/rust-stamp_alblum_generate}"
require_project_access "$WEB_PROJECT_ID" "website" "${WEB_PROJECT_PATH:-rust-apps/stampalbumgenerator2-site}"
log_ok "Project access verified"

latest_pipeline_for_ref() {
  local project_id="$1"
  local ref="$2"
  api_get_json "projects/$project_id/pipelines?ref=$ref&per_page=1" | jq -r 'if type=="array" and length>0 then .[0].id else "" end'
}

build_job_for_pipeline() {
  local project_id="$1"
  local pipeline_id="$2"
  api_get_json "projects/$project_id/pipelines/$pipeline_id/jobs?per_page=100" \
    | jq -r 'if type=="array" then (.[] | select(.name=="build_release") | .id) else "" end' \
    | head -n1
}

if [[ -z "$APP_PIPELINE_ID" ]]; then
  APP_PIPELINE_ID="$(latest_pipeline_for_ref "$APP_PROJECT_ID" "$APP_REF")"
fi
if [[ -z "$WEB_PIPELINE_ID" ]]; then
  WEB_PIPELINE_ID="$(latest_pipeline_for_ref "$WEB_PROJECT_ID" "$WEB_REF")"
fi

[[ -z "$APP_PIPELINE_ID" ]] && { log_err "Could not resolve app pipeline ID for ref '$APP_REF'"; exit 1; }
[[ -z "$WEB_PIPELINE_ID" ]] && { log_err "Could not resolve web pipeline ID for ref '$WEB_REF'"; exit 1; }

APP_JOB_ID="$(build_job_for_pipeline "$APP_PROJECT_ID" "$APP_PIPELINE_ID")"
WEB_JOB_ID="$(build_job_for_pipeline "$WEB_PROJECT_ID" "$WEB_PIPELINE_ID")"

[[ -z "$APP_JOB_ID" ]] && { log_err "Could not find app build_release job in pipeline $APP_PIPELINE_ID"; exit 1; }
[[ -z "$WEB_JOB_ID" ]] && { log_err "Could not find web build_release job in pipeline $WEB_PIPELINE_ID"; exit 1; }

APP_SHA="$(api_get_json "projects/$APP_PROJECT_ID/pipelines/$APP_PIPELINE_ID" | jq -r '.sha // "unknown"' | cut -c1-8)"
WEB_SHA="$(api_get_json "projects/$WEB_PROJECT_ID/pipelines/$WEB_PIPELINE_ID" | jq -r '.sha // "unknown"' | cut -c1-8)"

APP_ZIP="$OUT_DIR/stamp_album_gen2-${APP_REF}-${APP_SHA}.zip"
WEB_ZIP="$OUT_DIR/stampalbumgenerator2-site-${WEB_REF}-${WEB_SHA}.zip"

log_info "Downloading app artifacts: project=$APP_PROJECT_ID pipeline=$APP_PIPELINE_ID job=$APP_JOB_ID"
glab api "projects/$APP_PROJECT_ID/jobs/$APP_JOB_ID/artifacts" > "$APP_ZIP" || { log_err "Failed to download app artifacts for job $APP_JOB_ID"; exit 1; }
log_ok "Saved $APP_ZIP"

log_info "Downloading web artifacts: project=$WEB_PROJECT_ID pipeline=$WEB_PIPELINE_ID job=$WEB_JOB_ID"
glab api "projects/$WEB_PROJECT_ID/jobs/$WEB_JOB_ID/artifacts" > "$WEB_ZIP" || { log_err "Failed to download web artifacts for job $WEB_JOB_ID"; exit 1; }
log_ok "Saved $WEB_ZIP"

echo
echo "App ZIP: $APP_ZIP"
echo "Web ZIP: $WEB_ZIP"
echo ""
echo "Run deploy command:"
echo "  ./scripts/deploy-from-artifacts.sh --app '$APP_ZIP' --web '$WEB_ZIP' --deploy-dir /opt/stampalbumgenerator2-site"
