#!/usr/bin/env bash

# Canonical version synchronization script.
# Keeps semantic version values aligned across Cargo.toml and docs.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

DATE="$(date +%Y-%m-%d)"
DRY_RUN=false
NON_INTERACTIVE=false
SKIP_BACKUP=false
FROM_HOOK=false
AUTO_PATCH=false
DO_GIT=false
DO_PUSH=false
SET_VERSION=""
VERSION_TYPE="minor"

usage() {
    cat <<'EOF'
Usage:
  ./scripts/update_versions.sh [major|minor|patch]
  ./scripts/update_versions.sh --set X.Y.Z
  ./scripts/update_versions.sh auto-patch

Options:
  --set <version>  Set explicit semantic version (X.Y.Z)
  --dry-run        Print planned changes without modifying files
  --yes, -y        Non-interactive mode
  --from-hook      Optimized mode for pre-commit hook (non-interactive, no backups)
  --no-backup      Skip creating docs/versions backups
  --git            Commit and tag after update
  --push           Push commit/tag (requires --git)
  --help, -h       Show this help

Notes:
  - If Cargo.toml version is not semantic (X.Y.Z), the script derives base version
    from the highest semantic version in Cargo.toml and tags, then bumps from that base.
EOF
}

is_semver() {
    [[ "$1" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]
}

normalize_semver() {
    local v="${1#v}"
    if is_semver "$v"; then
        echo "$v"
    else
        echo ""
    fi
}

semver_ge() {
    local a="$1"
    local b="$2"
    IFS='.' read -r a1 a2 a3 <<< "$a"
    IFS='.' read -r b1 b2 b3 <<< "$b"

    if (( a1 > b1 )); then return 0; fi
    if (( a1 < b1 )); then return 1; fi
    if (( a2 > b2 )); then return 0; fi
    if (( a2 < b2 )); then return 1; fi
    if (( a3 >= b3 )); then return 0; fi
    return 1
}

increment_version() {
    local current="$1"
    local type="$2"
    IFS='.' read -r major minor patch <<< "$current"

    case "$type" in
        major) echo "$((major + 1)).0.0" ;;
        minor) echo "$major.$((minor + 1)).0" ;;
        patch) echo "$major.$minor.$((patch + 1))" ;;
        *)
            echo "Error: invalid version type '$type' (expected major|minor|patch)" >&2
            exit 1
            ;;
    esac
}

require_file() {
    local p="$1"
    if [[ ! -f "$p" ]]; then
        echo "Error: required file missing: $p" >&2
        exit 1
    fi
}

parse_args() {
    while [[ $# -gt 0 ]]; do
        case "$1" in
            major|minor|patch)
                VERSION_TYPE="$1"
                shift
                ;;
            auto-patch)
                AUTO_PATCH=true
                VERSION_TYPE="patch"
                shift
                ;;
            --set)
                if [[ $# -lt 2 ]]; then
                    echo "Error: --set requires a version argument" >&2
                    exit 1
                fi
                SET_VERSION="$2"
                shift 2
                ;;
            --dry-run)
                DRY_RUN=true
                shift
                ;;
            --yes|-y)
                NON_INTERACTIVE=true
                shift
                ;;
            --from-hook)
                FROM_HOOK=true
                NON_INTERACTIVE=true
                SKIP_BACKUP=true
                shift
                ;;
            --no-backup)
                SKIP_BACKUP=true
                shift
                ;;
            --git)
                DO_GIT=true
                shift
                ;;
            --push)
                DO_PUSH=true
                shift
                ;;
            --help|-h)
                usage
                exit 0
                ;;
            *)
                # Backward-compatible convenience: allow direct version input.
                if is_semver "${1#v}"; then
                    SET_VERSION="${1#v}"
                    shift
                else
                    echo "Error: unknown argument '$1'" >&2
                    usage
                    exit 1
                fi
                ;;
        esac
    done

    if [[ "$DO_PUSH" == true && "$DO_GIT" != true ]]; then
        echo "Error: --push requires --git" >&2
        exit 1
    fi
}

extract_cargo_package_version() {
    awk '
        BEGIN { in_pkg=0 }
        /^\[package\]/ { in_pkg=1; next }
        /^\[/ && in_pkg { in_pkg=0 }
        in_pkg && $0 ~ /^[[:space:]]*version[[:space:]]*=/ {
            line=$0
            sub(/^[^"]*"/, "", line)
            sub(/".*/, "", line)
            print line
            exit
        }
    ' Cargo.toml
}

highest_semver_tag() {
    local highest=""
    local tag
    while IFS= read -r tag; do
        local s
        s="$(normalize_semver "$tag")"
        if [[ -z "$s" ]]; then
            continue
        fi
        if [[ -z "$highest" ]] || semver_ge "$s" "$highest"; then
            highest="$s"
        fi
    done < <(git tag --list 2>/dev/null || true)

    if [[ -n "$highest" ]]; then
        echo "$highest"
    else
        echo ""
    fi
}

update_cargo_toml() {
    local new_version="$1"
    local tmp
    tmp="$(mktemp)"

    awk -v nv="$new_version" '
        BEGIN { in_pkg=0; replaced=0 }
        /^\[package\]/ { in_pkg=1; print; next }
        /^\[/ && in_pkg { in_pkg=0 }
        in_pkg && $0 ~ /^[[:space:]]*version[[:space:]]*=/ && replaced==0 {
            print "version = \"" nv "\""
            replaced=1
            next
        }
        { print }
        END {
            if (replaced==0) {
                print "Error: [package] version line not found" > "/dev/stderr"
                exit 2
            }
        }
    ' Cargo.toml > "$tmp"

    if [[ "$DRY_RUN" == true ]]; then
        rm -f "$tmp"
        return
    fi

    mv "$tmp" Cargo.toml
}

update_line_patterns() {
    local file="$1"
    local new_version="$2"
    local date="$3"

    if [[ "$DRY_RUN" == true ]]; then
        return
    fi

    # Version and date lines that are intentionally repeated in this repo are all updated.
    sed -E -i "s|^# Version: .*|# Version: ${new_version}|" "$file" || true
    sed -E -i "s|^# Last Updated: .*|# Last Updated: ${date}|" "$file" || true
    sed -E -i "s|^# Stamp Album Generator v.* Release Notes$|# Stamp Album Generator v${new_version} Release Notes|" "$file" || true
    if [[ "$file" == "RELEASE_NOTES.md" ]]; then
        sed -E -i "s|^# Stamp Album Generator v.*$|# Stamp Album Generator v${new_version} Release Notes|" "$file" || true
    fi
    sed -E -i "s|^# Stamp Album Generator v[^ ]*$|# Stamp Album Generator v${new_version}|" "$file" || true
    sed -E -i "s|^\*\*Version\*\*: .*|**Version**: ${new_version}|" "$file" || true
    sed -E -i "s|^\*\*Release Date\*\*: .*|**Release Date**: ${date}|" "$file" || true
    sed -E -i "s@^Version: .* \| Last Updated: .*@Version: ${new_version} | Last Updated: ${date}@" "$file" || true
    sed -E -i "s@<!-- Version: .* \| Last Updated: .* \| Previous:@<!-- Version: ${new_version} | Last Updated: ${date} | Previous:@" "$file" || true
}

create_backups() {
    local date="$1"
    mkdir -p docs/versions
    cp .github/instructions "docs/versions/instructions_${date}_backup.md"
    cp README.md "docs/versions/README_${date}_backup.md"
    cp RELEASE_NOTES.md "docs/versions/RELEASE_NOTES_${date}_backup.md"
    cp Cargo.toml "docs/versions/Cargo_${date}_backup.toml"
}

consistency_check() {
    local expected="$1"

    local cargo_v
    cargo_v="$(extract_cargo_package_version)"
    if [[ "$cargo_v" != "$expected" ]]; then
        echo "Consistency check failed: Cargo.toml package version is '$cargo_v', expected '$expected'" >&2
        return 1
    fi

    local instr_v
    instr_v="$(grep -E '^# Version: ' .github/instructions | head -1 | sed -E 's/^# Version: //')"
    if [[ "$instr_v" != "$expected" ]]; then
        echo "Consistency check failed: .github/instructions version is '$instr_v', expected '$expected'" >&2
        return 1
    fi

    return 0
}

git_commit_and_tag() {
    local new_version="$1"
    local tag="v${new_version}"

    git add Cargo.toml .github/instructions README.md RELEASE_NOTES.md
    git commit -m "chore(release): bump version to ${new_version}" --no-verify || true

    if git rev-parse --verify "refs/tags/${tag}" >/dev/null 2>&1; then
        echo "Tag ${tag} already exists. Skipping tag creation."
    else
        git tag -a "$tag" -m "Release ${tag}"
        echo "Created tag ${tag}"
    fi

    if [[ "$DO_PUSH" == true ]]; then
        git push --follow-tags
        git push origin "refs/tags/${tag}" || true
    fi
}

run_release_build() {
    # Keep pre-commit hook runs quick; full release build is for normal invocations.
    if [[ "$FROM_HOOK" == true ]]; then
        return
    fi

    echo "Building release binary (cargo build --release)..."
    cargo build --release --quiet
    echo "Release build complete."
    echo "Next step: run ./copy-release.sh"
}

main() {
    parse_args "$@"

    require_file "Cargo.toml"
    require_file "README.md"
    require_file "RELEASE_NOTES.md"
    require_file ".github/instructions"

    if [[ "$AUTO_PATCH" == true ]]; then
        echo "Running cargo build before auto-patch bump..."
        cargo build --quiet
    fi

    local cargo_version raw_cargo_semver tag_version base_version new_version
    cargo_version="$(extract_cargo_package_version)"
    raw_cargo_semver="$(normalize_semver "$cargo_version")"
    tag_version="$(highest_semver_tag)"

    if [[ -n "$SET_VERSION" ]]; then
        if ! is_semver "$SET_VERSION"; then
            echo "Error: --set requires semantic version X.Y.Z, got '$SET_VERSION'" >&2
            exit 1
        fi
        new_version="$SET_VERSION"
        base_version="$SET_VERSION"
    else
        if [[ -n "$raw_cargo_semver" ]]; then
            base_version="$raw_cargo_semver"
        else
            base_version="0.0.0"
        fi

        if [[ -n "$tag_version" ]] && semver_ge "$tag_version" "$base_version"; then
            base_version="$tag_version"
        fi

        new_version="$(increment_version "$base_version" "$VERSION_TYPE")"
    fi

    echo "=== Version Update ==="
    echo "Current Cargo package version: $cargo_version"
    echo "Highest semantic tag version: ${tag_version:-<none>}"
    echo "Base version used: $base_version"
    echo "New version: $new_version"
    echo "Date: $DATE"

    if [[ "$DRY_RUN" == true ]]; then
        echo "Dry-run mode: no files were modified."
        exit 0
    fi

    if [[ "$NON_INTERACTIVE" != true ]]; then
        read -r -p "Proceed with version update? (y/N): " confirm
        if [[ ! "$confirm" =~ ^[Yy]$ ]]; then
            echo "Cancelled."
            exit 0
        fi
    fi

    if [[ "$SKIP_BACKUP" != true ]]; then
        create_backups "$DATE"
    fi

    update_cargo_toml "$new_version"
    update_line_patterns ".github/instructions" "$new_version" "$DATE"
    update_line_patterns "README.md" "$new_version" "$DATE"
    update_line_patterns "RELEASE_NOTES.md" "$new_version" "$DATE"

    consistency_check "$new_version"
    run_release_build

    if [[ "$DO_GIT" == true ]]; then
        git_commit_and_tag "$new_version"
    fi

    echo "Updated version to $new_version in:"
    echo "  - Cargo.toml ([package] version)"
    echo "  - .github/instructions"
    echo "  - README.md"
    echo "  - RELEASE_NOTES.md"
}

main "$@"
