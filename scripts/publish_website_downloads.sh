#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ARTIFACTS_DIR="${1:-$ROOT_DIR/artifacts}"

WEBSITE_REPO_PUSH_URL="${WEBSITE_REPO_PUSH_URL:-}"
WEBSITE_REPO_BRANCH="${WEBSITE_REPO_BRANCH:-main}"
WEBSITE_GIT_AUTHOR_NAME="${WEBSITE_GIT_AUTHOR_NAME:-StampAlbum CI Publisher}"
WEBSITE_GIT_AUTHOR_EMAIL="${WEBSITE_GIT_AUTHOR_EMAIL:-ci-publisher@laser-spider.invalid}"
WEBSITE_DEPLOY_DIR="${WEBSITE_DEPLOY_DIR:-}"
WEBSITE_DEPLOY_MODE="${WEBSITE_DEPLOY_MODE:-downloads-only}"

print_runner_context() {
	echo "Runner context:"
	echo "  CI_RUNNER_DESCRIPTION=${CI_RUNNER_DESCRIPTION:-unknown}"
	echo "  CI_RUNNER_TAGS=${CI_RUNNER_TAGS:-unknown}"
	echo "  CI_RUNNER_EXECUTABLE_ARCH=${CI_RUNNER_EXECUTABLE_ARCH:-unknown}"
	echo "  HOSTNAME=$(hostname 2>/dev/null || echo unknown)"
	echo "  USER=$(id -un 2>/dev/null || echo unknown)"
	echo "  PWD=$(pwd)"
}

check_deploy_capability() {
	if [[ -z "$WEBSITE_DEPLOY_DIR" ]]; then
		echo "Deploy capability: WEBSITE_DEPLOY_DIR not set; host deploy disabled."
		return
	fi

	local parent_dir
	parent_dir="$(dirname "$WEBSITE_DEPLOY_DIR")"
	echo "Deploy capability: target dir is $WEBSITE_DEPLOY_DIR"

	if [[ -d "$WEBSITE_DEPLOY_DIR" ]]; then
		if [[ ! -w "$WEBSITE_DEPLOY_DIR" ]]; then
			echo "Error: target dir exists but is not writable by current runner user."
			exit 3
		fi
	else
		if [[ ! -d "$parent_dir" ]]; then
			echo "Error: parent dir does not exist: $parent_dir"
			exit 3
		fi
		if [[ ! -w "$parent_dir" ]]; then
			echo "Error: parent dir is not writable by current runner user: $parent_dir"
			exit 3
		fi
	fi

	mkdir -p "$WEBSITE_DEPLOY_DIR"
	local probe_file="$WEBSITE_DEPLOY_DIR/.deploy-write-test.$$"
	if ! ( : > "$probe_file" ); then
		echo "Error: runner cannot write to $WEBSITE_DEPLOY_DIR"
		exit 3
	fi
	rm -f "$probe_file"
	echo "Deploy capability: write probe passed for $WEBSITE_DEPLOY_DIR"
}

print_runner_context
check_deploy_capability

if [[ -z "$WEBSITE_REPO_PUSH_URL" ]]; then
	echo "Error: WEBSITE_REPO_PUSH_URL is required."
	echo "Set it as a protected CI/CD variable with an authenticated HTTPS Git URL for the website repo."
	exit 2
fi

if [[ ! -d "$ARTIFACTS_DIR" ]]; then
	echo "Error: artifacts directory not found: $ARTIFACTS_DIR"
	exit 2
fi

LINUX_BINARY="$ARTIFACTS_DIR/stamp_album_gen2"
WINDOWS_BINARY="$ARTIFACTS_DIR/stamp_album_gen2.exe"

if [[ ! -f "$LINUX_BINARY" ]]; then
	echo "Error: required Linux binary missing: $LINUX_BINARY"
	exit 2
fi

APP_VERSION="$({ awk '
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
' "$ROOT_DIR/Cargo.toml"; } | tr -d '[:space:]')"

if [[ -z "$APP_VERSION" ]]; then
	echo "Error: failed to parse app version from Cargo.toml"
	exit 2
fi

WORK_DIR="$(mktemp -d)"
cleanup() {
	rm -rf "$WORK_DIR"
}
trap cleanup EXIT

WEBSITE_CLONE_DIR="$WORK_DIR/website"

git clone --depth 1 --branch "$WEBSITE_REPO_BRANCH" "$WEBSITE_REPO_PUSH_URL" "$WEBSITE_CLONE_DIR"

DOWNLOAD_DIR="$WEBSITE_CLONE_DIR/static/downloads/current"
mkdir -p "$DOWNLOAD_DIR"

cp "$LINUX_BINARY" "$DOWNLOAD_DIR/stamp_album_gen2"
if [[ -f "$WINDOWS_BINARY" ]]; then
	cp "$WINDOWS_BINARY" "$DOWNLOAD_DIR/stamp_album_gen2.exe"
else
	rm -f "$DOWNLOAD_DIR/stamp_album_gen2.exe"
fi

# Remove stale checksum artifacts before re-generating.
rm -f "$DOWNLOAD_DIR/stamp_album_gen2.sha256"
rm -f "$DOWNLOAD_DIR/stamp_album_gen2.exe.sha256"
rm -f "$DOWNLOAD_DIR/SHA256SUMS.txt"
rm -f "$DOWNLOAD_DIR/CRC32SUMS.txt"

sha256sum "$DOWNLOAD_DIR/stamp_album_gen2" | awk '{print $1 "  stamp_album_gen2"}' > "$DOWNLOAD_DIR/stamp_album_gen2.sha256"

if [[ -f "$DOWNLOAD_DIR/stamp_album_gen2.exe" ]]; then
	sha256sum "$DOWNLOAD_DIR/stamp_album_gen2.exe" | awk '{print $1 "  stamp_album_gen2.exe"}' > "$DOWNLOAD_DIR/stamp_album_gen2.exe.sha256"
fi

{
	cat "$DOWNLOAD_DIR/stamp_album_gen2.sha256"
	if [[ -f "$DOWNLOAD_DIR/stamp_album_gen2.exe.sha256" ]]; then
		cat "$DOWNLOAD_DIR/stamp_album_gen2.exe.sha256"
	fi
} > "$DOWNLOAD_DIR/SHA256SUMS.txt"

cksum "$DOWNLOAD_DIR/stamp_album_gen2" | awk '{print $1 " " $2 " stamp_album_gen2"}' > "$DOWNLOAD_DIR/CRC32SUMS.txt"
if [[ -f "$DOWNLOAD_DIR/stamp_album_gen2.exe" ]]; then
	cksum "$DOWNLOAD_DIR/stamp_album_gen2.exe" | awk '{print $1 " " $2 " stamp_album_gen2.exe"}' >> "$DOWNLOAD_DIR/CRC32SUMS.txt"
fi

cat > "$DOWNLOAD_DIR/release-metadata.json" <<EOF
{
  "version": "$APP_VERSION",
  "linuxBinary": "stamp_album_gen2",
  "windowsBinary": "$(if [[ -f "$DOWNLOAD_DIR/stamp_album_gen2.exe" ]]; then echo 'stamp_album_gen2.exe'; else echo ''; fi)",
  "checksums": {
	"sha256Manifest": "SHA256SUMS.txt",
	"crc32Manifest": "CRC32SUMS.txt",
	"linuxSha256": "stamp_album_gen2.sha256",
	"windowsSha256": "$(if [[ -f "$DOWNLOAD_DIR/stamp_album_gen2.exe.sha256" ]]; then echo 'stamp_album_gen2.exe.sha256'; else echo ''; fi)"
  }
}
EOF

printf '%s\n' "$APP_VERSION" > "$WEBSITE_CLONE_DIR/APP_VERSION"

while IFS= read -r -d '' html_file; do
	perl -0pi -e 's/(class="js-app-version">)[^<]+/$1'"$APP_VERSION"'/g' "$html_file"
done < <(find "$WEBSITE_CLONE_DIR/static" -type f -name '*.html' -print0)

build_website_runtime() {
	echo "Building website runtime binary"
	(
		cd "$WEBSITE_CLONE_DIR"
		cargo build --release --bin stampalbumgenerator2-site
	)
}

deploy_to_host_dir() {
	if [[ -z "$WEBSITE_DEPLOY_DIR" ]]; then
		echo "WEBSITE_DEPLOY_DIR is not set; skipping host deploy."
		return
	fi

	mkdir -p "$WEBSITE_DEPLOY_DIR"

	if [[ "$WEBSITE_DEPLOY_MODE" == "full-site" ]]; then
		build_website_runtime
		local release_id release_root current_link
		release_id="${APP_VERSION}-$(date +%Y%m%d%H%M%S)"
		release_root="$WEBSITE_DEPLOY_DIR/releases/$release_id"
		current_link="$WEBSITE_DEPLOY_DIR/current"
		mkdir -p "$release_root/bin" "$release_root/static"
		if command -v rsync >/dev/null 2>&1; then
			rsync -a --delete "$WEBSITE_CLONE_DIR/static/" "$release_root/static/"
		else
			cp -a "$WEBSITE_CLONE_DIR/static/." "$release_root/static/"
		fi
		cp "$WEBSITE_CLONE_DIR/APP_VERSION" "$release_root/APP_VERSION"
		cp "$WEBSITE_CLONE_DIR/target/release/stampalbumgenerator2-site" "$release_root/bin/"
		ln -sfn "$release_root" "$current_link"
		echo "Deployed full website release to $release_root"
		echo "Updated symlink $current_link -> $release_root"
		return
	fi

	mkdir -p "$WEBSITE_DEPLOY_DIR/static/downloads/current"
	if command -v rsync >/dev/null 2>&1; then
		rsync -a --delete "$WEBSITE_CLONE_DIR/static/downloads/current/" "$WEBSITE_DEPLOY_DIR/static/downloads/current/"
	else
		rm -rf "$WEBSITE_DEPLOY_DIR/static/downloads/current"
		mkdir -p "$WEBSITE_DEPLOY_DIR/static/downloads/current"
		cp -a "$WEBSITE_CLONE_DIR/static/downloads/current/." "$WEBSITE_DEPLOY_DIR/static/downloads/current/"
	fi
	cp "$WEBSITE_CLONE_DIR/APP_VERSION" "$WEBSITE_DEPLOY_DIR/APP_VERSION"
	echo "Deployed release downloads to $WEBSITE_DEPLOY_DIR"
}

deploy_to_host_dir

cd "$WEBSITE_CLONE_DIR"

git config user.name "$WEBSITE_GIT_AUTHOR_NAME"
git config user.email "$WEBSITE_GIT_AUTHOR_EMAIL"

if git diff --quiet; then
	echo "No website release changes detected."
	exit 0
fi

git add APP_VERSION static/downloads/current static/index.html static/pages
git commit -m "chore(release): publish app v$APP_VERSION artifacts"
git push origin "$WEBSITE_REPO_BRANCH"

echo "Published app v$APP_VERSION artifacts to website branch $WEBSITE_REPO_BRANCH"
