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
- Historical/manual test outputs are stored in `docs/testing/`.
- Keep root clean by placing new test run reports under `docs/testing/`.

## Documentation Rules

- Place user/developer guidance in `docs/guides/`.
- Place audits and project reports in `docs/reports/`.
- Place test result artifacts in `docs/testing/`.
- Keep root-level docs minimal: `README.md`, `DEVELOPMENT.md`, `CONTRIBUTING.md`, and `RELEASE_NOTES.md`.

## Release Workflow

- Update versions with `scripts/update_versions.sh`.
- Review `RELEASE_NOTES.md` before tagging.
- Keep Conventional Commit format for clear history (`feat:`, `fix:`, `docs:`, etc.).

## License Policy

- Project license: `LICENSE` (Laser Spider Stamp Collector License (LSSCL) v1.0).
- Personal no-fee use is limited to individual stamp collectors.
- Any non-collector, business, institutional, or commercial use requires a separate written paid agreement with Laser Spider LLC.
- Forks/copies must retain the project license unless explicitly approved in writing by Laser Spider LLC.
