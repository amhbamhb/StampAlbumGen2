# AGENTS.md — AI Agent Instructions for StampAlbumGen2

This file provides guidelines for AI coding agents (GitHub Copilot, Claude, etc.) working in this repository.

---

## Repository Overview

StampAlbumGen2 is a Rust CLI application that generates AlbumEasy stamp-album pages from stamp data scraped from StampWorld, US1847, and Colnect CSV exports. It stores data in SQLite databases and includes a local browser-based Review UI for manually linking Colnect candidates to StampWorld entries.

**Language:** Rust (edition 2021)  
**Build tool:** Cargo  
**Main binary:** `stamp_album_gen2`  
**Workspace:** root crate + `crates/stamp_album_db` shared crate

---

## Repository Layout

```text
src/                   Application source code
  main.rs              CLI entry point
  cli.rs               CLI flags/options (clap)
  modes.rs             Mode routing and orchestration
  config.rs            Configuration loading and defaults
  lib.rs               Library crate root
  review_ui/           Review UI HTTP server
  colnect/             Colnect CSV ingestion logic
  stampworld/          StampWorld scraper
  us1847/              US1847 scraper
  data_sources/        Source dispatch
  db_utils/            Database helpers
  album_globals/       Album page generation
  image_utils.rs       Image processing
  types.rs             Shared types
crates/stamp_album_db/ Shared DB crate (schema, queries)
tests/                 Integration and unit tests
docs/                  All project documentation
  guides/              Developer and user guides
  reports/             Audit and status reports
  testing/             Test run artifacts
samples/               Reference input files (canonical HTML, CSVs)
### 11. Error Handling and Safety
- **No Unwraps:** Do not use `.unwrap()` in production code. Use `?` for propagation, `match` for explicit handling, or provide safe defaults via `unwrap_or_else`.
- **Structured Errors:** When a request fails, return structured errors instead of panicking or ignoring the failure.
- **Graceful Degradation:** When possible, provide reasonable defaults if data is missing, but log the discrepancy.

  reference/           Example-Stampworldwebpage.txt (canonical HTML)
scripts/               Utility scripts (versioning, debug runner)
flags/                 Country→flag mapping data
config_template.toml   Baseline config template
config_full_example.toml  Full config reference with all keys
```

---

## Build and Test Commands

```bash
# Format and lint (run before committing)
./format-and-check.sh
# or individually:
cargo fmt
cargo clippy

# Run all tests
cargo test --all

# Build debug binary
cargo build

# Build release binary
cargo build --release

# Build with full embedded flag database
cargo build --features full-flag-db

# Validate local setup
./verify_setup.sh
```

---

## Key Rules for Agents

### 1. Rust Safety

- Do **not** use unchecked `.unwrap()` in runtime code paths.
- Prefer `?`, `ok_or_else`, `unwrap_or_else`, or `match` with a meaningful error.
- In request handlers and background tasks, return structured errors instead of panicking.

### 2. Cross-Platform Compatibility

- All changes must remain compatible with both **Windows and Linux**.
- Use `std::path::Path`/`PathBuf` for path handling — never raw string concatenation.
- Avoid shell-specific assumptions in commands or scripts.

### 3. Canonical Sample File for DB Import

- Before creating or modifying any import/parser logic, **load and reference**:  
  `samples/reference/Example-Stampworldwebpage.txt`
- Use the HTML structure in that file as the authoritative field mapping for stamp record extraction.
- Key elements to map:
  - Outer container: `div.container-fluid.content_table` with `id="group_box_<group_id>"`
  - Header fields (`.table_header`): title link, date, designer, engraving, perforation
  - Images: `span.stamp_img` with `data-stamp-type` and `data-stamp-group-id`
  - Table rows: `tr.stamp_tr` with per-row stamp type and number
- Preserve group-level summary rows separately from individual stamp rows.
- Write unit tests for any new parser, using the canonical sample file for assertions.

### 4. Git / Version Control

- **Do not use GitHub APIs, GitHub PRs, or GitHub Issues** for this project.  
  See `docs/COPILOT_AGENT_INSTRUCTIONS.md` for details on the preferred GitLab remote.
- Preferred remote: self-hosted GitLab at `gitinfo.laserspider.info`.
- When in doubt, commit locally with a clear message and notify maintainers of the branch name.
- Do **not** push without explicit maintainer instruction. Never expose credentials in commits or logs.
- Branch naming: `feature/<short-name>`, `fix/<short-name>`, `refactor/<file-or-area>`.

### 5. Commit Messages

Follow Conventional Commits. Types in use:

- `Add:` / `feat:` — new features
- `Fix:` / `fix:` — bug fixes
- `Update:` / `refactor:` — improvements/restructuring
- `Docs:` / `docs:` — documentation only
- `Test:` / `test:` — test changes

### 6. Configuration

- Local runtime config: `config.toml` (not committed; create from template).
- Template: `config_template.toml`
- Full reference: `config_full_example.toml`
- When adding new config keys: update `config.rs`, the template, the full example, and document in `docs/guides/CONFIG_PARAMETERS_INVENTORY.md`.

### 7. Documentation Placement

| Content type          | Location                  |
|-----------------------|---------------------------|
| User/developer guides | `docs/guides/`            |
| Audit/status reports  | `docs/reports/`           |
| Test run artifacts    | `docs/testing/`           |
| Root-level docs       | `README.md`, `DEVELOPMENT.md`, `CONTRIBUTING.md`, `RELEASE_NOTES.md` |

Do **not** place new user-facing documentation in the repository root unless it is one of the four files above (or `AGENTS.md`).

### 8. Database Schema Changes

- Maintain backward compatibility with existing SQLite databases.
- Auto-migration is the pattern: add missing columns on startup, never modify existing rows.
- Document schema changes and add migration notes.

### 9. Testing

- All tests live in `tests/` and in inline `#[cfg(test)]` modules.
- Write unit tests for new parsers and functions.
- Do not remove or edit existing tests unless directly caused by your change.
- Historical/manual test outputs go in `docs/testing/`.

### 10. Temporary Files

- Create temporary/one-off scripts outside the repository tree (preferred location is documented in `.github/copilot-instructions.md`; otherwise use your OS temp dir such as `/tmp` on Linux/macOS or `%TEMP%` on Windows) so they are never committed.

---

## Debugging

```bash
# Run with debug logging
RUST_LOG=debug ./target/release/stamp_album_gen2 [options]

# Run debug build in background with timestamped log (avoids VS Code terminal issues)
./scripts/run_debug_no_vscode.sh -- --source stampworld --site-name Switzerland \
  --album-name Switzerland --start-year 1976 --end-year 1976
# Logs written to Logs/debug-run-<timestamp>.log
```

---

## License

**Laser Spider Stamp Collector License (LSSCL) v1.0**  
Individual stamp collectors may use the software for personal use at no fee.  
Non-collector, business, corporate, institutional, or commercial use requires a separate written paid agreement with Laser Spider LLC.

See `LICENSE` and `LASER_SPIDER_STAMP_COLLECTOR_LICENSE_v1.0.md`.

<!-- okf-rs:begin -->
## Knowledge base

This project's structural knowledge — modules, types, functions, and their call graph — is available as an [OKF](https://cloud.google.com/blog/products/data-analytics/how-the-open-knowledge-format-can-improve-data-sharing) bundle in `knowledge/`. It's plain markdown with YAML frontmatter; browse `knowledge/index.md` for an overview, or query it with the CLI:

- `okf-rs search <query>` — find a symbol, type, or module by name or tag
- `okf-rs graph callers <id>` / `okf-rs graph callees <id>` — trace the call graph from a concept id (ids are shown by `search`)
- `okf-rs graph api` — list the public API surface
- `okf-rs graph cycles` — list call-graph cycles

Regenerate the bundle after code changes with `okf-rs generate`.
<!-- okf-rs:end -->
