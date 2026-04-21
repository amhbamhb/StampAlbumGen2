#!/usr/bin/env bash

# Create a GitLab issue with glab, then create/push a linked branch.
# Default branch format: issue/<iid>-<slug>
#
# Examples:
#   scripts/new-issue-branch.sh
#   scripts/new-issue-branch.sh "Fix colnect inbox routing"
#   scripts/new-issue-branch.sh --title "Fix parser" --label bug --label cli
#   scripts/new-issue-branch.sh --title "Fix parser" --description "Details" --prefix feat

set -euo pipefail

usage() {
    cat <<'EOF'
Usage: scripts/new-issue-branch.sh [OPTIONS] [TITLE]

Create a GitLab issue (via glab), then create and push a branch named:
  issue/<iid>-<slug>

Options:
  --title <text>         Issue title. If omitted, you will be prompted once.
  --description <text>   Issue description/body (optional).
  --label <label>        Add a label (can be repeated).
  --assignee <user>      Assign issue to GitLab username (optional).
  --prefix <name>        Branch prefix (default: issue).
  --remote <name>        Git remote to push to (default: origin).
  -h, --help             Show this help.

Notes:
  - Requires authenticated glab session and a git repo with a valid remote.
  - No editor is opened; this is non-interactive except title prompt when missing.
EOF
}

require_cmd() {
    if ! command -v "$1" >/dev/null 2>&1; then
        echo "Error: required command not found: $1" >&2
        exit 1
    fi
}

slugify() {
    local raw="$1"
    local slug
    slug="$(printf '%s' "$raw" \
        | tr '[:upper:]' '[:lower:]' \
        | sed -E 's/[^a-z0-9]+/-/g; s/^-+//; s/-+$//; s/-+/-/g')"

    if [[ -z "$slug" ]]; then
        slug="issue"
    fi

    # Keep branch names readable.
    printf '%.64s' "$slug"
}

TITLE=""
DESCRIPTION=""
PREFIX="issue"
REMOTE_NAME="origin"
ASSIGNEE=""
declare -a LABELS=()

after_double_dash=false
while (($#)); do
    if [[ "$1" == "--" ]]; then
        after_double_dash=true
        shift
        continue
    fi

    if [[ "$after_double_dash" == true ]]; then
        # Treat trailing args as part of title text.
        if [[ -z "$TITLE" ]]; then
            TITLE="$1"
        else
            TITLE+=" $1"
        fi
        shift
        continue
    fi

    case "$1" in
        --title)
            TITLE="${2:-}"
            shift 2
            ;;
        --description)
            DESCRIPTION="${2:-}"
            shift 2
            ;;
        --label)
            LABELS+=("${2:-}")
            shift 2
            ;;
        --assignee)
            ASSIGNEE="${2:-}"
            shift 2
            ;;
        --prefix)
            PREFIX="${2:-}"
            shift 2
            ;;
        --remote)
            REMOTE_NAME="${2:-}"
            shift 2
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        -* )
            echo "Error: unknown option: $1" >&2
            usage
            exit 2
            ;;
        *)
            if [[ -z "$TITLE" ]]; then
                TITLE="$1"
            else
                TITLE+=" $1"
            fi
            shift
            ;;
    esac
done

if [[ -z "$TITLE" ]]; then
    read -r -p "Issue title: " TITLE
fi

if [[ -z "$TITLE" ]]; then
    echo "Error: title is required." >&2
    exit 2
fi

require_cmd git
require_cmd glab

if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    echo "Error: must be run from inside a git repository." >&2
    exit 1
fi

if ! git remote get-url "$REMOTE_NAME" >/dev/null 2>&1; then
    echo "Error: git remote '$REMOTE_NAME' not found." >&2
    exit 1
fi

if ! glab auth status >/dev/null 2>&1; then
    echo "Error: glab is not authenticated. Run: glab auth login" >&2
    exit 1
fi

create_args=(issue create --title "$TITLE")
create_args+=(--description "$DESCRIPTION")

if [[ -n "$ASSIGNEE" ]]; then
    create_args+=(--assignee "$ASSIGNEE")
fi

for label in "${LABELS[@]}"; do
    create_args+=(--label "$label")
done

echo "Creating GitLab issue..."
set +e
issue_output="$(glab "${create_args[@]}" 2>&1)"
status=$?
set -e
if [[ $status -ne 0 ]]; then
    echo "Error: failed to create issue with glab." >&2
    printf '%s\n' "$issue_output" >&2
    exit $status
fi

# glab typically prints issue URL; parse IID from trailing /issues/<iid>.
iid="$(printf '%s\n' "$issue_output" | sed -nE 's#.*?/issues/([0-9]+).*#\1#p' | tail -n 1)"
if [[ -z "$iid" ]]; then
    echo "Error: could not parse issue IID from glab output:" >&2
    printf '%s\n' "$issue_output" >&2
    exit 1
fi

slug="$(slugify "$TITLE")"
branch_name="${PREFIX}/${iid}-${slug}"

# If branch already exists locally or remotely, fail early with a clear message.
if git show-ref --verify --quiet "refs/heads/$branch_name"; then
    echo "Error: local branch already exists: $branch_name" >&2
    exit 1
fi
if git ls-remote --exit-code --heads "$REMOTE_NAME" "$branch_name" >/dev/null 2>&1; then
    echo "Error: remote branch already exists on $REMOTE_NAME: $branch_name" >&2
    exit 1
fi

echo "Creating branch: $branch_name"
git checkout -b "$branch_name"

echo "Pushing branch to $REMOTE_NAME"
git push -u "$REMOTE_NAME" "$branch_name"

echo
echo "Done."
echo "Issue IID: $iid"
echo "Branch:    $branch_name"
echo "glab output:"
printf '%s\n' "$issue_output"
