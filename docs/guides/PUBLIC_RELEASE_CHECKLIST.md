# Public Release Checklist

Audience: maintainers preparing and verifying public Stamp Album Generator 2
releases. Contributors do not need release credentials to submit changes.

## 1. Enable the automated release process once

Complete these steps before the first release using the new workflow:

- Review and merge the automatic-release implementation into the default branch
  (normally `main`). Include the workflow, version scripts, regression tests,
  `.gitignore` changes, and `Cargo.lock`. The workflow requires a committed
  lockfile; it must not be excluded from Git.
- Confirm normal CI is green and the default branch contains all intended fixes.
- In repository Actions settings, allow the workflow to obtain write access for
  source/tag and release publication. The workflow requests `contents: write`
  only in its publication jobs.
- Confirm branch and tag protection rules permit the approved release
  automation to publish the version commit and `vX.Y.Z` tag. The workflow cannot
  bypass required PRs or other protections. If policy forbids direct automated
  commits, resolve that policy/design mismatch before attempting a release;
  do not disable protections casually or add a personal token as a workaround.
- Decide whether Windows signing is required. If it is, configure `SIGNING_PFX`
  and `SIGNING_PFX_PASSWORD` through repository secrets and verify the signing
  job. Never place signing material in source files or logs.
- If signing is not configured, the workflow deliberately publishes the unsigned
  Windows executable. Document this for users; Windows may display a warning.
- Disable any separate automatic version bumps in hooks, workflows, or release
  tooling. The shared version script is the only version calculator.
- Ensure maintainers understand that this workflow publishes publicly after its
  jobs succeed. There is no final human approval or draft-review pause.

The workflow and scripts must already be on the default branch before the
normal dispatch instructions below apply. Do not mix deployment of the new
workflow with attempts to release an old tag containing broken source.

## 2. Prepare each public release

- Merge the intended changes into the default branch. Avoid unrelated merges
  while the release builds; an advancing branch can reject source publication.
- Review `RELEASE_NOTES.md`: summarize changes, compatibility or migration
  requirements, known limitations, and any manual user actions.
- Confirm Windows and Linux behavior, particularly file paths, config saving,
  first-run setup, and AlbumEasy integration.
- Confirm source and release inputs contain no private config, login credentials,
  personal databases, or signing secrets.
- Review the applicable license and user-facing documentation. Public
  availability does not change the project's licensing terms.
- Review dependency/lockfile changes independently of the release bump. Release
  preparation is not intended to upgrade third-party dependencies.
- Choose the increment:

  | Selection | Use for | Example from `1.14.0` |
  |-----------|---------|-----------------------|
  | patch | Compatible bug fixes | `1.14.1` |
  | minor | Compatible new functionality | `1.15.0` |
  | major | Breaking changes | `2.0.0` |

The starting point is the greater of the Cargo package version and existing
plain `vX.Y.Z` tags, not necessarily the version of the latest published release.
An old reserved or broken tag still occupies its number and is never overwritten.

## 3. Start the release

1. Open repository **Actions**.
2. Select **Release (combined)**, then **Run workflow**.
3. Under **Use workflow from**, select the default branch, normally `main`.
4. Choose **patch**, **minor**, or **major**. Patch is the default.
5. Click **Run workflow** once.
6. Open the run summary and verify the automatically selected version.

Do **not** enter a version number, create a tag, edit version headers, or run
`scripts/update_versions.sh` manually before dispatching a normal release.
Commit-message wording does not select the increment in this model.

### What happens automatically

1. Prepare the version and synchronized headers through the shared script.
2. Update the workspace lockfile entry and share an exact version patch.
3. Apply and verify the same complete source tree on Windows and Linux.
4. Run library tests and build both releases with the committed lockfile.
   Windows retains native dialogs; Linux disables them.
5. Sign the Windows binary if signing secrets are configured.
6. Commit the tested version changes and atomically publish the commit and
   annotated tag. A rejected push stops publication.
7. Download both binaries, create checksums and an archive, upload to a draft,
   then publish the release.

No source/tag publication occurs before the test/build/signing gates succeed.
Publishing the Git tag and publishing release assets are separate operations:
an asset-upload failure can leave a valid source tag and an incomplete draft.
Use recovery below rather than starting another version.

## 4. Verify the public release

Do not announce the release until these checks pass:

- All workflow jobs succeeded and the public release has the intended tag.
- The tag identifies the generated version commit, and `[package].version`
  matches the tag without its leading `v`.
- All five assets are available:
  - `stamp_album_gen2.exe`
  - `stamp_album_gen2.exe.sha256`
  - `stamp_album_gen2-linux`
  - `stamp_album_gen2-linux.sha256`
  - `stamp_album_gen2-vX.Y.Z.tar.gz` with the actual version substituted.
- Download from the public release page, not from a developer's build directory.
  Verify each executable against its corresponding checksum.
- On Windows PowerShell:

  ```powershell
  Get-FileHash .\stamp_album_gen2.exe -Algorithm SHA256
  Get-Content .\stamp_album_gen2.exe.sha256
  .\stamp_album_gen2.exe --version
  .\stamp_album_gen2.exe --help
  Get-AuthenticodeSignature .\stamp_album_gen2.exe
  ```

  Compare the hash values, confirm the expected version, and check signature
  status if signing was required. Successful execution alone is not signature
  verification.

- On Linux:

  ```bash
  sha256sum -c stamp_album_gen2-linux.sha256
  chmod +x stamp_album_gen2-linux
  ./stamp_album_gen2-linux --version
  ./stamp_album_gen2-linux --help
  ```

- On both platforms, use a temporary working directory and disposable test
  album/config to check startup, Review UI, and AlbumEasy path saving. Do not
  test migration or import behavior against an irreplaceable user database.
- Review the generated public release notes for accuracy and completeness.
- Coordinate website/download-link updates with the separate website repository
  maintainers. Publishing an app release does not itself verify website links.
- Announce the version, important changes, known limitations, signing status,
  and download/checksum location only after verification.

Checksums detect file changes; they do not independently authenticate a release
if both executable and checksum come from a compromised source.

## 5. Recover from failures

| Failure | Maintainer action |
|---------|-------------------|
| Transient build or signing error | Fix the environment or secrets, then use **Re-run failed jobs** on the same run. |
| Source/test defect | Fix and merge it; use **Re-run all jobs** before source publication, or start a new release if the previous source/tag was already published. |
| Default branch advanced or atomic push rejected | Inspect the rejection. Resolve permissions/protection issues as needed, then use **Re-run all jobs** to prepare and test the current branch tip. |
| Upload failed after source/tag publication | Use **Re-run failed jobs**. The same run retains its prepared version and resumes draft uploads. |
| All jobs rerun after source/tag publication | The same run locates its `Release-Run` commit and reuses that version; it does not rewind the default branch. |
| Release is already public | A retry leaves published assets unchanged. Ship corrections as a new release. |
| Invalid old manifest or occupied tag | Do not move/delete the tag. The new automatic release increments past occupied versions using current source. |
| Missing committed lockfile | Merge `Cargo.lock` and the corrected ignore rules before starting the workflow. |

**Run workflow** starts a new release and increments again.
**Re-run failed jobs / Re-run all jobs** retries the existing workflow run.
Use the retry controls for recovery; do not repeatedly click **Run workflow**.

Never force-push a release tag, replace published binaries in place, or skip a
failing test to obtain a release. If a public release is defective, communicate
the problem and publish a corrected version through the normal process.

## 6. Local maintainer validation

On Linux with Git, Bash, Cargo, Node.js 18+, and Python 3.11+:

```bash
node --test tests/release_publication_shell.cjs tests/cli_workflows.test.cjs tests/cargo_targets.test.cjs
cargo test --release --workspace --lib --no-default-features --locked
cargo build --release --no-default-features --locked
```

The Node tests use disposable local repositories/remotes, exercise failed pushes
and retries, and do not publish to the real repository. Native Windows builds
and signing must still be verified on Windows.

For implementation details, see [Release Workflow](../../DEVELOPMENT.md#release-workflow)
and [the workflow definition](../../.github/workflows/release-combined.yml).
