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

### 0. Mandatory Pre-Commit Quality Gate (RUN BEFORE ANY COMMIT)

Every agent must run and pass the following quality gates BEFORE staging or committing any changes:

1. **Precheck (`format-and-check.sh`)**:
   ```bash
   ./format-and-check.sh
   # Or on Windows PowerShell:
   cargo fmt --all -- --check; cargo check --all-targets --quiet
   ```
2. **Lint Validation (`lint`)**:
   ```bash
   cargo clippy --all-targets -- -D warnings
   ```
3. **Full Test Suite Across All Targets & Profiles**:
   ```bash
   cargo test --all
   ```
   All tests in lib, bins, and integration suites must pass cleanly without compilation errors or panics.
4. **CLI Documentation Sync**:
   If any CLI flag, argument, option, or help text in `src/cli.rs` changes:
   ```bash
   cargo run --no-default-features -- --help > docs/CLI_USAGE.md
   ```
5. **Commit Header Length Limit**:
   The commit header (`<type>(<scope>): <description>`) must **not exceed 100 characters** (enforced by `commitlint`).

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

- This repository uses **GitHub**: `amhbamhb/StampAlbumGen2`.
- **Per-Item GitHub Issue & Branch Workflow (MANDATORY)**:
  - For each new item, fixture family, command implementation, or bug fix:
    1. Create a dedicated GitHub issue first using `gh issue create --repo amhbamhb/StampAlbumGen2 --title "<title>" --body "<details>"`.
    2. Create and switch to a dedicated branch for that item: `feature/issue-<num>-<short-name>` (or `fix/issue-<num>-<short-name>`).
    3. Complete implementation, automated tests, fixture documentation, and website gallery synchronization:
       - **Mandatory Native PDF Generation**: Always generate and save the actual PDF version of each variation using the native-pdf engine (`*-native.pdf` and `*-raw-native.pdf`) so maintainers can visually inspect and review the visual rendering.
       - **Dual Documentation (User-Friendly & Detailed)**: Always author documentation in both easy, user-focused language for stamp collectors and comprehensive technical details (parameters, coordinates, command grammar, and reference provenance).
       - **Keep PDFs and Gallery Updated**: Whenever changes occur, regenerate the native PDFs and synchronize all source files, rendered PDFs, and gallery entries to the website repository (`stampalbumgenerator2-site`).
       - **Standard Page Size (US Letter Default)**: The standard default page size for documents, album generators, and examples is **US Letter** (`215.9 × 279.4 mm` / `8.5 × 11.0 in`), unless the specific fixture or test is explicitly testing A4 (`210.0 × 297.0 mm`) or other custom dimensions.
    4. Reference the issue in commits: `feat: ... (fixes #<num>)` or `fix: ... (fixes #<num>)`.
- When in doubt, commit locally with a clear message and notify maintainers of the branch name.
- Do **not** push without explicit maintainer instruction. Never expose credentials in commits or logs.

### 5. Commit Messages

Follow Conventional Commits based on the [Conventional Commit Messages Specification & Cheat Sheet](https://gist.github.com/qoomon/5dfcdf8eec66a051ecd85625518cfd13).

#### Structure
```text
<type>(<optional scope>): <description>

[optional body]

[optional footer(s)]
```

#### Types
- `feat`: Commits that add, adjust, or remove a feature (API, CLI, UI, or rendering capability)
- `fix`: Commits that fix a bug
- `refactor`: Commits that rewrite or restructure code without altering external behavior
- `perf`: Special refactor commits that specifically improve execution or rendering performance
- `style`: Commits that address code style (white-space, formatting, semicolons) without logic changes
- `test`: Commits that add missing tests or correct existing ones
- `docs`: Commits that exclusively affect documentation or guides
- `build`: Commits affecting build tools, cargo dependencies, project version, or packaging
- `ops`: Commits affecting operational tooling, deployment scripts, CI/CD, or automated workflows
- `chore`: Commits for miscellaneous maintenance tasks (e.g. updating `.gitignore`, license headers)

#### Scopes
- Optional contextual indicator in lowercase parentheses: e.g. `feat(native-pdf):`, `fix(renderer):`, `docs(agents):`, `test(marginse):`.
- **Do not** use issue identifiers as scopes (put issue references in footer or description instead).

#### Description
- Mandatory, concise summary of the change.
- Use imperative, present tense: `"add"`, `"fix"`, `"update"`, `"refactor"` (not `"added"`, `"fixed"`). Think: *"This commit will..."*.
- Do **not** capitalize the first letter (unless a proper noun/identifier).
- Do **not** end with a period (`.`).
 **Header Length Limit (MANDATORY)**: The commit header line (`<type>(<scope>): <description>`) **must NOT exceed 100 characters** (strictly enforced by GitHub Actions `commitlint` via `[header-max-length]`). Keep descriptions punchy, and place detailed notes or issue links in the body/footer if the header approaches 100 characters.

#### Breaking Changes
- Indicate with an `!` before the colon: e.g. `feat(parser)!: break backwards compatibility with legacy format`.
- In the commit footer, prefix the explanation with `BREAKING CHANGE: <description>`.

#### Body & Footer
- **Body** (optional): Explains the motivation behind the change, edge cases handled, and contrasts with previous behavior.
- **Footer** (optional / mandatory for issues): Reference GitHub issues and tasks (e.g. `Fixes #122`, `Closes #84`).

#### Examples
- `feat(native-pdf): add .stampalbum fixtures for page-size-v1 (fixes #122)`
- `fix(renderer): correct stamp heading padding calculation for multi-line local titles`
- `test(marginse): verify alternating margins across page break boundaries`
- `docs(agents): update commit message standards from conventional commits gist`
- `refactor(album): consolidate stamp inner border state parser`

### 6. Configuration & CLI Parameters

- Local runtime config: `config.toml` (not committed; create from template).
- Template: `config_template.toml`
- Full reference: `config_full_example.toml`
- When adding new config keys: update `config.rs`, the template, the full example, and document in `docs/guides/CONFIG_PARAMETERS_INVENTORY.md`.
- **CLI Options & Documentation Sync (MANDATORY)**: Whenever any CLI flag, argument, option, or help text in `src/cli.rs` changes:
  - You **must** regenerate `docs/CLI_USAGE.md` before committing or pushing:
    ```bash
    cargo run --no-default-features -- --help > docs/CLI_USAGE.md
    ```
    *(Note: On Windows PowerShell, ensure the output file is saved with UTF-8 encoding without BOM, e.g. using `cargo run --no-default-features -- --help | Out-File -Encoding utf8 docs/CLI_USAGE.md` or Unix redirection).*
  - The `.github/workflows/verify-cli-docs.yml` CI workflow strictly compares `docs/CLI_USAGE.md` with `cargo run --no-default-features -- --help` and fails if they drift.

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
