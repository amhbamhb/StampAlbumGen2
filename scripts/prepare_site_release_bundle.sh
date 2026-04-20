#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

linux_bin="target/release/stamp_album_gen2"
windows_bin="target/x86_64-pc-windows-gnu/release/stamp_album_gen2.exe"
bundle_root="${1:-build/site-release-bundle}"

if [[ ! -f "$linux_bin" ]]; then
  echo "Missing Linux binary: $linux_bin" >&2
  exit 1
fi

if [[ ! -f "$windows_bin" ]]; then
  echo "Missing Windows binary: $windows_bin" >&2
  exit 1
fi

static_src="stampalbumgenerator2-site/static"
if [[ ! -d "$static_src" ]]; then
  echo "Missing static site directory: $static_src" >&2
  exit 1
fi

mkdir -p "$bundle_root"

if command -v rsync >/dev/null 2>&1; then
  rsync -a --delete "$static_src/" "$bundle_root/"
else
  rm -rf "$bundle_root"
  mkdir -p "$bundle_root"
  cp -a "$static_src/." "$bundle_root/"
fi

current_dir="$bundle_root/downloads/current"
mkdir -p "$current_dir"

cp -f "$linux_bin" "$current_dir/stamp_album_gen2"
cp -f "$windows_bin" "$current_dir/stamp_album_gen2.exe"

sha256sum "$current_dir/stamp_album_gen2" | awk '{print $1"  stamp_album_gen2"}' > "$current_dir/stamp_album_gen2.sha256"
sha256sum "$current_dir/stamp_album_gen2.exe" | awk '{print $1"  stamp_album_gen2.exe"}' > "$current_dir/stamp_album_gen2.exe.sha256"

cat "$current_dir/stamp_album_gen2.sha256" "$current_dir/stamp_album_gen2.exe.sha256" > "$current_dir/SHA256SUMS.txt"

cksum "$current_dir/stamp_album_gen2" | awk '{print $1" "$2" stamp_album_gen2"}' > "$current_dir/CRC32SUMS.txt"
cksum "$current_dir/stamp_album_gen2.exe" | awk '{print $1" "$2" stamp_album_gen2.exe"}' >> "$current_dir/CRC32SUMS.txt"

app_version="$(tr -d ' \n\r\t' < stampalbumgenerator2-site/APP_VERSION)"
release_version="${CI_COMMIT_TAG:-$app_version}"
release_version="${release_version#v}"

cat > "$current_dir/release-metadata.json" <<EOF
{
  "version": "$release_version",
  "linuxBinary": "stamp_album_gen2",
  "windowsBinary": "stamp_album_gen2.exe",
  "checksums": {
    "sha256Manifest": "SHA256SUMS.txt",
    "crc32Manifest": "CRC32SUMS.txt",
    "linuxSha256": "stamp_album_gen2.sha256",
    "windowsSha256": "stamp_album_gen2.exe.sha256"
  }
}
EOF

echo "Prepared site release bundle at: $bundle_root"
ls -lah "$current_dir"
