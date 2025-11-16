# Release flow and CI token setup

This document describes how releases and automated tagging work for StampAlbumGen2, how to configure CI tokens, and how to recover from mistakes.

Tag format and canonical source

- Tags are always created with a leading `v`, e.g. `v0.22.20`.
- `Cargo.toml` holds the canonical version string without the `v` (e.g. `0.22.20`).
- The CI bump job and `version_docs.sh` are written to keep tags and `Cargo.toml` in sync.

CI tokens and recommended variables

- To allow GitLab CI to push commits and tags automatically, provide one of the following:
  - `GITLAB_ACCESS_TOKEN` (recommended): a personal access token or project access token with `write_repository` (or full `api` if you need release API access). Store it as a masked, protected CI variable.
  - `CI_JOB_TOKEN`: available in CI jobs but it may not have repository write permissions depending on your GitLab setup. The pipeline already attempts to use `CI_JOB_TOKEN` where available.

How to create a token (GitLab)

1. For a project deploy token: Project -> Settings -> Repository -> Deploy Tokens -> create a token with `write_repository` scope.
2. For a personal access token: Profile -> Access Tokens -> create a token with `write_repository` scope (and `api` if you want release creation via API).
3. In the project: Settings -> CI/CD -> Variables -> Add variable
   - Key: `GITLAB_ACCESS_TOKEN`
   - Value: (paste token)
   - Masked: true
   - Protected: true (if you only want it usable on protected branches like `main`)

How CI uses the token

- When pushing commits/tags the pipeline prefers `CI_JOB_TOKEN` first, then `GITLAB_ACCESS_TOKEN`.
- If no token is available, the pipeline will create commits/tags locally but skip pushing (safe default).

Local developer workflow

- If you run `./version_docs.sh` locally, prefer running it without `--push` to preview changes and tags, then push manually.
- Example (safe):

```bash
git checkout -b release-test
./version_docs.sh patch --git --yes   # creates local commit + tag
# inspect changes
git show --name-only HEAD
git tag -l | tail -n 5
# when satisfied, push manually
git push origin HEAD
git push origin --tags
```

Automated bump logic (what happens)

1. The script computes a candidate new version (major/minor/patch rules).
2. Before creating a tag, it checks whether `v<version>` already exists locally or on the remote.
3. If the tag exists, the script increments the patch and repeats (up to a safety limit).
4. For each bump it updates `Cargo.toml`, documentation, and commits the change so repository contents match the tag.
5. Finally the script creates an annotated tag `vX.Y.Z` and pushes it if tokens are available.

Rolling back a mistaken tag or bump

- To remove a bad tag locally:

```bash
git tag -d vX.Y.Z
```

- To remove a bad tag on remote `origin`:

```bash
git push origin :refs/tags/vX.Y.Z
```

- To revert a version bump commit (if the bump created a commit you want to undo):

```bash
# revert the commit (safer than reset if pushed)
git revert <commit-sha>
# or to fully remove it (local only):
git reset --hard HEAD~1
```

Notes and recommendations

- Prefer `GITLAB_ACCESS_TOKEN` stored as a masked & protected CI variable for automatic pushing.
- Avoid force-updating remote tags unless you have a strict policy and approvals; force-updates are dangerous for downstream consumers.
- Keep `version_docs.sh` as the canonical script for local/manual bumps; CI contains a small bump implementation too but both now follow the same normalization rules.

Questions or customizations

- If you want the pipeline to always pick the next free patch number automatically (even across race conditions), we can add an optimistic lock step or use the GitLab API to create release and handle conflicts explicitly.
- If you'd like, I can add a small `docs/RELEASING.md` with example commands tailored to your team's branching policy.
