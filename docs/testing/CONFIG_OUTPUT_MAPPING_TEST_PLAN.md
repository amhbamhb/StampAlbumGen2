# Config → AlbumEasy Output Mapping Test Plan

Goal: for each `config.toml` value, change it, generate output, and assert the
emitted AlbumEasy command matches the syntax in `docs/AlbumEasy_4.2.1.pdf`.

## Current state

- `tests/config_parameter_validation.rs` only verifies configs *load*; no test
  asserts that a value appears in the generated output.
- `ALBUM_PAGES_SIZE  ({:.1} {:.1})` is emitted by `write_globals` in
  `src/album_utils/globals.rs`, matching the manual's
  `ALBUM_PAGES_SIZE (width height)`.
- `[page]` section keys are promoted to top level by `load_config`.

### Blockers / findings

- **Testability:** `write_globals` hard-codes
  `load_effective_config("config.toml")` (CWD-relative). Tests cannot inject a
  config without `set_current_dir`, which races under parallel tests.
- **Default mismatch:** template and code default US Letter width to `216.0`;
  the AlbumEasy manual specifies `215.9`.
- **Precision:** output uses `{:.1}`, so `215.94` emits `215.9`. Tests must
  assert the rounded form.

## Plan

### Step 1 — Extract a pure seam (no behavior change)

Move the page-setup line generation (`ALBUM_PAGES_SIZE`,
`ALBUM_PAGES_MARGINS`, `ALBUM_PAGES_BORDER`) from `write_globals` into
`page_setup_lines(cfg: &Config) -> Vec<String>`; `write_globals` calls it.

Verify: `cargo test --all` passes; a generated album is byte-identical apart
from timestamp lines.

### Step 2 — First mapping tests (`tests/config_output_mapping.rs`)

Table-driven: write TOML to a tempdir → `load_config(abs_path)` →
`page_setup_lines` → assert exact line present.

| Config change | Expected line |
|---|---|
| `album_page_width_mm = 210.0`, `album_page_height_mm = 297.0` | `ALBUM_PAGES_SIZE  (210.0 297.0)` |
| width `279.4`, height `215.9` (landscape) | `ALBUM_PAGES_SIZE  (279.4 215.9)` |
| same values under `[page]` | same line (section promotion) |
| keys absent | `(216.0 279.4)` (current default) |

### Step 3 — Expand one group at a time

1. Margins → `ALBUM_PAGES_MARGINS (left right top bottom)`
2. Borders → `ALBUM_PAGES_BORDER (outer inner1 inner2 spacing)`
3. Title font / size / position → `ALBUM_PAGES_HEADER`
4. `include_*` toggles → presence/absence of fields in emitted text

Track covered keys in a checklist in this folder.

### Step 4 (optional) — End-to-end

Run the built binary as a subprocess with `current_dir` set to a tempdir that
holds its own `config.toml` and a minimal DB (per-process CWD, no race), then
grep the generated `.txt` for the same lines.

## PDF verification (optional)

`pdfinfo` (poppler-utils) reports page size in points (1 mm = 72/25.4 pt):

- A4 → ≈ 595.28 × 841.89 pt
- US Letter → ≈ 612 × 792 pt

Producing the PDF requires AlbumEasy (GUI), so this would be an `#[ignore]`
test that runs against a manually exported PDF. Deeper PDF layout checks are
out of scope.

## Approaches considered

1. (45%) Pure-function seam + table-driven tests (Steps 1–3) — chosen.
2. (25%) Subprocess end-to-end per config (Step 4).
3. (15%) Change `write_globals` to take `&Config` (touches multiple callers).
4. (10%) `set_current_dir` with serialized tests (brittle).
5. (5%) Golden-file snapshots (noisy due to timestamps).
