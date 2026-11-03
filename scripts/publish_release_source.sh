#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."
: "${RELEASE_TAG:?Missing prepared release tag}"
: "${BASE_SHA:?Missing prepared base commit}"
: "${TREE_SHA:?Missing prepared source tree}"
: "${RELEASE_BRANCH:?Missing release branch}"
: "${GITHUB_RUN_ID:?Missing workflow run ID}"
: "${GITHUB_OUTPUT:?Missing workflow output path}"
: "${RUNNER_TEMP:?Missing runner temporary directory}"

if git rev-parse --verify "refs/tags/$RELEASE_TAG" >/dev/null 2>&1; then
    SOURCE_SHA="$(git rev-parse "refs/tags/$RELEASE_TAG^{commit}")"
    if [[ "$(git rev-parse "$SOURCE_SHA^{tree}")" != "$TREE_SHA" ]] ||
        ! git log -1 --format=%B "$SOURCE_SHA" | grep -Fxq "Release-Run: $GITHUB_RUN_ID"; then
        echo "Existing tag $RELEASE_TAG does not belong to this prepared release; refusing to overwrite it." >&2
        exit 1
    fi
    REMOTE_TAGS="$(git ls-remote origin "refs/tags/$RELEASE_TAG" "refs/tags/$RELEASE_TAG^{}")"
    if ! awk -v sha="$SOURCE_SHA" '$1 == sha { found=1 } END { exit !found }' <<< "$REMOTE_TAGS"; then
        echo "Release tag $RELEASE_TAG is absent or different on origin; refusing an unverified retry." >&2
        exit 1
    fi
    echo "Reusing already published source $RELEASE_TAG at $SOURCE_SHA."
else
    if [[ -n "${REUSED_SHA:-}" ]]; then
        echo "Previously published release tag $RELEASE_TAG is missing; refusing to recreate it." >&2
        exit 1
    fi
    if [[ "$(git rev-parse HEAD)" != "$BASE_SHA" ]]; then
        echo "Release checkout does not match the prepared base commit." >&2
        exit 1
    fi
    git apply --index "$RUNNER_TEMP/release-source/release-source.patch"
    if [[ "$(git write-tree)" != "$TREE_SHA" ]]; then
        echo "Release source differs from the tree tested by the build jobs." >&2
        exit 1
    fi
    git config user.name "github-actions[bot]"
    git config user.email "github-actions[bot]@users.noreply.github.com"
    # Both platform test/build jobs have validated this exact staged tree.
    git -c core.hooksPath=/dev/null commit -m "chore(release): $RELEASE_TAG" \
        -m "Release-Run: $GITHUB_RUN_ID" \
        -m "Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"
    SOURCE_SHA="$(git rev-parse HEAD)"
    git tag -a "$RELEASE_TAG" -m "Release $RELEASE_TAG"
    if ! git push --atomic origin "HEAD:refs/heads/$RELEASE_BRANCH" "refs/tags/$RELEASE_TAG"; then
        echo "Atomic publication failed. No release output was recorded. Re-run all jobs to prepare against the current branch." >&2
        exit 1
    fi
fi
echo "source_sha=$SOURCE_SHA" >> "$GITHUB_OUTPUT"
