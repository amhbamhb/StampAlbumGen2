# Development Guide

This guide is for contributors and maintainers working on the codebase.

## Prerequisites

- Rust toolchain (stable)
- `cargo` available in shell
- Linux/macOS/Windows environment supported by Rust

## Common Commands

```bash
# Format and lint checks
./format-and-check.sh

# Run all tests
cargo test --all

# Build release binary
cargo build --release

# Validate local setup
./verify_setup.sh

# Build with embedded full flag database (requires flags/flag_codes.json)
cargo build --features full-flag-db
```

## Optional Native Dialogs

Default builds omit native dialogs and their Wayland dependencies. First-run
installation uses a terminal prompt when stdin is interactive; otherwise it
continues in the current directory without prompting. The browser Review UI is
available. Its Browse buttons use built-in Windows PowerShell/Windows Forms on
Windows (independent of the `rfd` feature), and `zenity` or `kdialog` on Linux.
Cancellation leaves the current selection unchanged; launch/dialog failures are
returned to the browser as errors.

To retain native folder/confirmation dialogs on Windows or Linux, build with
`cargo build --release --features rfd`. For development checks, use
`USE_RFD=1 ./format-and-check.sh` (or `cargo clippy --features rfd` on Windows).
On Ubuntu, the optional Linux dialog build requires `pkg-config` and
`libwayland-dev` at build time, and the Wayland client library and a working
desktop portal/backend at runtime. An X11 desktop can provide these; a Wayland
session is not required. Native-dialog builds are intended for graphical sessions.

The release workflow ships the Linux artifact without dialogs and does not
install Wayland development libraries; the Windows artifact retains dialogs.
The headless CLI parity and CLI-docs workflows explicitly use
`--no-default-features` for Cargo builds and runs, even if native dialogs are
enabled in the package defaults. Check this invariant and the Linux dependency
graph with `node --test tests/cli_workflows.test.cjs`.
For Linux distribution, inspect shared libraries with `ldd` and test on X11,
Wayland, and headless systems. Build against the oldest supported glibc baseline
to support older distributions; removing Wayland does not remove that requirement.

## Main Development Files

- `src/main.rs` main executable entry point
- `src/lib.rs` shared library crate entry
- `src/cli.rs` CLI flags/options
- `src/modes.rs` mode routing and orchestration
- `src/config.rs` configuration loading and defaults
- `scripts/update_versions.sh` semantic version synchronization for docs and manifest

## Configuration Workflow

- Local runtime config: `config.toml`
- Baseline template: `config_template.toml`
- Exhaustive example keys: `config_full_example.toml`

Recommended flow:

1. Copy `config_template.toml` to `config.toml` for local work.
2. Add only keys you need while testing behavior.
3. Promote stable options to `config_full_example.toml` when documenting features.

## Testing Notes

- Unit/integration tests live in `tests/` and code-level test modules.
- Check that Cargo binary sources exist in clean checkouts with
  `node --test tests/cargo_targets.test.cjs` (Node.js 18+). Do not declare
  ignored local debugging scripts as Cargo binary targets: they may build
  locally but fail target resolution in CI on both Windows and Linux.
- StampWorld credential-recovery UI behavior can be checked without a browser
  or external services using `node --test tests/review_ui_stampworld_credentials.test.cjs`
  (Node.js 18+). Rendering and handler checks use `cargo test --lib review_ui::`.
- Historical/manual test outputs are stored in `docs/testing/`.
- Keep root clean by placing new test run reports under `docs/testing/`.

## Documentation Rules

- Place user/developer guidance in `docs/guides/`.
- Place audits and project reports in `docs/reports/`.
- Place test result artifacts in `docs/testing/`.
- Keep root-level docs minimal: `README.md`, `DEVELOPMENT.md`, `CONTRIBUTING.md`, and `RELEASE_NOTES.md`.

## Release Workflow

For setup, public-release verification, and recovery procedures, follow the
[Public Release Checklist](docs/guides/PUBLIC_RELEASE_CHECKLIST.md).

Normal releases require no manual version edits, scripts, or tags:

1. Merge the changes and release notes into the default branch (normally `main`).
2. Open **Release (combined)**, choose that branch in **Use workflow from**, and
   select **patch** (default), **minor**, or **major**.
3. Click **Run workflow**. The run summary displays the calculated release tag.

The existing `scripts/update_versions.sh` is the single version calculator and
file synchronizer. CI calls its non-building `--prepare-only` mode. It increments
the greater of `[package].version` and all existing plain `vX.Y.Z` tags.
For example, package `1.13.2` plus existing tag `v1.14.0` produces patch `1.14.1`,
minor `1.15.0`, or major `2.0.0`. Older broken tags are never reused or moved.

The workflow:

- Updates Cargo, README, release-note, and instruction version headers together.
- Updates the workspace entry in the committed `Cargo.lock` rather than
  regenerating dependency versions. Platform builds and tests use `--locked`.
- Shares the exact version patch between the Windows/native-dialog and
  Linux/headless test/build jobs, verifying the complete Git tree hash.
- Only after both platforms and optional signing pass, commits the tested
  changes and atomically pushes the version commit and annotated tag.
- Uploads both binaries, checksums, and archive to a draft before publishing.
  Windows artifacts are included even when signing is not configured.

Releases are serialized. Repository permissions must let `GITHUB_TOKEN` push
the version commit to the default branch and create tags/releases. A protected
branch requiring PRs needs an approved automation exception; the workflow does
not bypass protections. If the branch advances during the build, the atomic
push fails without publishing the tag. Use **Re-run all jobs** to prepare and
validate against the new branch tip.

For transient build/signing failures, use **Re-run failed jobs**. If source
publication already succeeded, reruns of the same workflow run identify its
release commit by `Release-Run` and reuse that version, even if the default
branch has advanced. Draft uploads can resume; published releases are left
unchanged. Clicking **Run workflow** again starts a *new* release and increments
again. Never manually move a released tag.

The manual version script remains available for exceptional local preparation;
it is not needed before clicking Release. Do not separately bump versions in
pre-commit hooks or other workflows. Use Conventional Commits for source
changes; this release model uses the selected increment, not commit-message
heuristics.

Test automation with
`node --test tests/release_publication_shell.cjs tests/cli_workflows.test.cjs tests/cargo_targets.test.cjs`
on Linux (Git, Bash, Cargo, Node.js 18+, and Python 3.11+). These tests use small
temporary repositories and local remotes; they do not publish real releases.

## License Policy

- Project license: `LICENSE` (Laser Spider Stamp Collector License (LSSCL) v1.0).
- Personal no-fee use is limited to individual stamp collectors.
- Any non-collector, business, institutional, or commercial use requires a separate written paid agreement with Laser Spider LLC.
- Forks/copies must retain the project license unless explicitly approved in writing by Laser Spider LLC.
