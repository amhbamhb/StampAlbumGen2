#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."
: "${VERSION_TYPE:?Choose patch, minor, or major}"
: "${GITHUB_RUN_ID:?Missing workflow run ID}"
: "${GITHUB_OUTPUT:?Missing workflow output path}"
: "${RUNNER_TEMP:?Missing runner temporary directory}"
case "$VERSION_TYPE" in
    patch|minor|major) ;;
    *) echo "Invalid version increment: $VERSION_TYPE" >&2; exit 1 ;;
esac
if [[ -n "$(git status --porcelain --untracked-files=no)" ]]; then
    echo "Release preparation requires a clean tracked working tree." >&2
    exit 1
fi

REUSED_SHA=""
RELEASE_TAG=""
while IFS= read -r tag; do
    if git log -1 --format=%B "$tag" | grep -Fxq "Release-Run: $GITHUB_RUN_ID"; then
        if [[ -n "$REUSED_SHA" ]]; then
            echo "Multiple release tags belong to this workflow run; refusing ambiguous retry." >&2
            exit 1
        fi
        RELEASE_TAG="$tag"
        REUSED_SHA="$(git rev-parse "refs/tags/$tag^{commit}")"
    fi
done < <(git tag --list 'v[0-9]*')

mkdir -p "$RUNNER_TEMP/release-source"
PATCH="$RUNNER_TEMP/release-source/release-source.patch"
VERSION_FILES=(Cargo.toml Cargo.lock .github/instructions README.md RELEASE_NOTES.md)
if [[ -n "$REUSED_SHA" ]]; then
    git checkout --detach "$REUSED_SHA"
    BASE_SHA="$(git rev-parse HEAD^)"
    git diff --binary --full-index "$BASE_SHA" HEAD -- "${VERSION_FILES[@]}" > "$PATCH"
    TREE_SHA="$(git rev-parse 'HEAD^{tree}')"
else
    BASE_SHA="$(git rev-parse HEAD)"
    if ! git ls-files --error-unmatch Cargo.lock >/dev/null 2>&1; then
        echo "Cargo.lock must be committed before automated releases can pin dependencies." >&2
        exit 1
    fi
    # Validate the source manifest and existing lock before changing its version.
    cargo metadata --no-deps --format-version 1 --locked > /dev/null
    bash scripts/update_versions.sh "$VERSION_TYPE" --prepare-only
    # Cargo updates only the package entry in an existing compatible lockfile.
    cargo update --workspace
    VERSION="$(python3 -c 'import tomllib; print(tomllib.load(open("Cargo.toml", "rb"))["package"]["version"])')"
    RELEASE_TAG="v$VERSION"
    if git rev-parse --verify "refs/tags/$RELEASE_TAG" >/dev/null 2>&1; then
        echo "Calculated release tag $RELEASE_TAG already exists; refusing to overwrite it." >&2
        exit 1
    fi
    git add -- "${VERSION_FILES[@]}"
    TREE_SHA="$(git write-tree)"
    git diff --binary --full-index "$BASE_SHA" -- "${VERSION_FILES[@]}" > "$PATCH"
fi

VERSION="$(python3 -c 'import tomllib; print(tomllib.load(open("Cargo.toml", "rb"))["package"]["version"])')"
if [[ "$RELEASE_TAG" != "v$VERSION" ]]; then
    echo "Release tag $RELEASE_TAG does not match prepared package version $VERSION." >&2
    exit 1
fi
{
    echo "release_tag=$RELEASE_TAG"
    echo "base_sha=$BASE_SHA"
    echo "tree_sha=$TREE_SHA"
    echo "reused_sha=$REUSED_SHA"
} >> "$GITHUB_OUTPUT"
echo "Prepared $RELEASE_TAG (source tree $TREE_SHA); no source commits or tags were pushed."
