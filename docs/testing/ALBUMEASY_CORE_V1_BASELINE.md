# AlbumEasy core-v1 baseline and execution report

**Date:** 2026-11-03
**Status:** Frozen reference inputs/output and measured expectations; maintainer
confirmed visual and 100%-scale physical print checks passed on 2026-11-03.
Windows execution remains unverified. No native renderer exists yet.

## Files

- [album.txt](../../tests/fixtures/albumeasy/core-v1/album.txt): self-contained
  editable AlbumEasy input; no external assets, dates or custom fonts.
- [reference.pdf](../../tests/fixtures/albumeasy/core-v1/reference.pdf): actual
  two-page AlbumEasy **4.5.0** output, 6,701 bytes.
- [expected.json](../../tests/fixtures/albumeasy/core-v1/expected.json): input
  checksum, page count/size, 11 measured frame positions/dimensions, required
  text and future native-comparison tolerances.

Do not regenerate the reference in place. Use a separate output directory.
The source is frozen for drift comparisons; use `core-v2` for approved changes
and additional named fixtures for new command families.

### Frozen identities

| Item | SHA-256 |
|------|---------|
| TXT input | `acc5a4bb5c643aa0db97f2f9c6e4ba7f44bbacedebbaf8ed913b7007a7bf73e7` |
| Reference PDF | `b186888681074cfa9b2c6be8ad39a17a6198c8824403a57f4d9f7a9a13a71081` |
| Reference AlbumEasy executable | `cc3badc44e4a2539424148d1038a4a569fa7d63c174d1fca7c8a58bc6dca5ca7` |
| Installed Qt help archive | `8b81707d861aacceb798b9f38efbbfdc3ea86fa49a57b7e343c3a25d5964d5be` |

## What this baseline covers

**Page 1:** equal/justified/fixed rows, top/middle/bottom alignment, unequal stamp
box heights, inside text, left/center/right captions and explicit text alignment.

**Page 2:** 100 x 10 mm horizontal calibration box, 10 x 50 mm vertical box,
vertical spacing, horizontal rule and multiline text.

Both pages have an explicit border, header and resolved page-number footer.
Twenty-four distinct commands are exercised:

```text
ALBUM_AUTHOR ALBUM_TITLE ALBUM_PAGES_SIZE ALBUM_PAGES_MARGINS
ALBUM_PAGES_SPACING ALBUM_PAGES_BORDER ALBUM_PAGES_HEADER
ALBUM_PAGES_HEADER_PAD ALBUM_PAGES_FOOTER_NUMBER ALBUM_PAGES_FOOTER_PAD
STAMP_BOXES_SIZE_ADJUST PAGE_START PAGE_TEXT PAGE_TEXT_CENTRE PAGE_TEXT_RIGHT
PAGE_VSPACE PAGE_RULE_H ROW_ALIGN_TOP ROW_ALIGN_MIDDLE ROW_ALIGN_BOTTOM
ROW_START_ES ROW_START_JS ROW_START_FS STAMP_ADD
```

This is not all-command certification. It deliberately excludes images, external
fonts, shapes, borders with assets, preprocessing and failures; add independent
fixtures for those rather than destabilizing this reference.

## Configuration policy

The TXT fixes values explicitly; neither renderer may silently replace them with
the current global config. This makes comparisons reproducible as user settings
change. The installed AlbumEasy console still uses its own application settings;
the executable/help identities and observed result above identify this run.
This ASCII/built-in-font fixture avoids encoding and system-font differences.

| Fixture value | Current app config mapping |
|---------------|----------------------------|
| 216 x 279.4 mm | `[page].album_page_width_mm`, `album_page_height_mm` |
| Four 15 mm margins | `[page].page_*_margin_mm`; intentionally differs from current local 10 mm |
| Single 0.2 mm border | `[page].page_border_outer`; other widths/gap zero |
| Horizontal 3 / vertical 4 mm content spacing | `[page].album_pages_spacing_h`, `album_pages_spacing_v` |
| HN 8 centered header | Current `[header].album_pages_title_font`, `album_pages_title_size`, `album_pages_header_position` feed `HEADER` |
| HN 8 footer, start 1, total pages | `[footer].album_pages_footer_number_*` |
| Header/footer padding | Not fully represented in current config schema |
| Row HN 8, 0.2 mm border, fixed gap 6 mm | Current row helper uses `row_start_es_font`, `row_start_es_size`, `row_start_es_spacing` (border) and `row_start_es_vspace` (FS gap) |
| Zero box adjustment | `[stamps].stamp_box_adjust` |
| Explicit page/body text and alignment state | Document commands, not blanket global-config overrides |

Do not modify or commit the local [config.toml](../../config.toml) to reproduce
this fixture. Future configuration-generated fixtures should be separate and
include their own frozen non-sensitive config.

## Exact renderer and inspection commands executed

Linux render working directory:
`/home/ray/.copilot/session-state/80e32404-199a-4106-9649-659ef9662bab/files/albumeasy-core-v1-run`

The source was copied there from the fixture directory. Start/end:
**2026-11-03 08:59:22 -06:00**; exit **0**, success.

```bash
QT_QPA_PLATFORM=offscreen timeout 60s /home/ray/work/AlbumEasy/AlbumEasy.sh -c album.txt >renderer.stdout.log 2>renderer.stderr.log
pdfinfo album.pdf
pdftotext -layout album.pdf extracted.txt
mutool draw -q -F trace -o trace.xml album.pdf
pdftoppm -r 144 -png -singlefile -f 1 -l 1 album.pdf page-1 >raster.stdout.log 2>raster.stderr.log
pdftoppm -r 144 -png -singlefile -f 2 -l 2 album.pdf page-2 >>raster.stdout.log 2>>raster.stderr.log
```

A second copy of the input was rendered in the `repeat` subdirectory:

```bash
QT_QPA_PLATFORM=offscreen timeout 60s /home/ray/work/AlbumEasy/AlbumEasy.sh -c album.txt >renderer.stdout.log 2>renderer.stderr.log
mutool draw -q -F trace -o trace.xml album.pdf
```

Both AlbumEasy runs returned success with empty stderr. The normalized input
filename was the same in both trace documents. Their complete vector/text traces
were **byte-identical**, including geometry, glyph positions and styles.
The PDF checksums differ because generated timestamps differ; PDF byte identity
is therefore not a drift acceptance criterion.

### Validation results

| Check | Result |
|-------|--------|
| Fresh PDF, two explicit pages | Pass |
| MediaBox | 216 x 279.4 mm, within 0.01 mm |
| Page 1 | Nine frame positions/dimensions and 12 required-text assertions passed |
| Page 2 | Two frame positions/dimensions and eight required-text assertions passed |
| Calibration geometry | 100 x 10 mm and 10 x 50 mm, within 0.01 mm |
| Two independent renders | Identical vector/text trace; differing timestamp metadata excluded |
| Raster review | Both 144 DPI pages inspected; no apparent clipping/overlap |
| Physical print | Maintainer reports visual and 100%-scale print checks passed; not independently measured by the checker |
| Windows rendering | Not run; portable TXT and commands provided below |
| Internal renderer comparison | Pending implementation; not claimed to pass |

Geometry checks read MuPDF's vector trace: each frame has two vertical and two
horizontal line segments. Compare the vertical-line x coordinates and
horizontal-line y coordinates, not full stroke endpoint bounds (the latter
include 0.1 mm corner extensions). The expected frame tuple is
`[left, bottom, width, height]` in PDF bottom-left millimetres. Every tuple in
`expected.json` was checked within 0.01 mm. Text checks used per-page
`pdftotext -f N -l N album.pdf -` with whitespace normalized. Input SHA-256,
two-page count, renderer logs and complete repeat-trace equality were also checked.

## Run it yourself

Copy only `album.txt` to a new working directory and open it in AlbumEasy,
then select Generate. Or run from that directory:

Linux (substitute your installation path):

```bash
QT_QPA_PLATFORM=offscreen /path/to/AlbumEasy/AlbumEasy.sh -c album.txt
```

Windows PowerShell (substitute your installation path):

```powershell
& 'C:\path\to\AlbumEasy\AlbumEasy.exe' -c .\album.txt
```

Open the generated `album.pdf`, compare with `reference.pdf` at equal scale, and
print page 2 at 100% to check the calibration boxes. Windows console diagnostics
may differ; inspect the actual fresh PDF and AlbumEasy's messages.

## Automated checker: approved T003 baby step

[albumeasy_baseline.rs](../../tests/albumeasy_baseline.rs) uses the existing Cargo
test runner. `roxmltree` is a direct test-only dependency (already present in the
lockfile transitively); no runtime dependency/backend is introduced.

Normal tests check frozen TXT/PDF identities and negative controls. External-tool
tests are explicitly ignored in ordinary Cargo runs so Windows/Linux developers
without MuPDF/Poppler can still run the normal suite. They **must** be run
explicitly for baseline acceptance; missing tools or a missing candidate variable
fail, rather than silently skipping.

Prerequisites: `mutool` and `pdftotext` on PATH. No browser, Docker, network or DB
is needed for this file-based checker.

```bash
cargo test --test albumeasy_baseline --test external_album_backend -- --nocapture
cargo test --test albumeasy_baseline reference_geometry_text_and_negative_controls -- --ignored --nocapture
ALBUMEASY_BASELINE_CANDIDATE=/absolute/path/to/fresh/album.pdf cargo test --test albumeasy_baseline candidate_matches_reference_strictly -- --ignored --nocapture
```

Windows PowerShell uses the same Cargo commands; set the candidate separately:

```powershell
$env:ALBUMEASY_BASELINE_CANDIDATE = 'C:\isolated\fresh\album.pdf'
cargo test --test albumeasy_baseline candidate_matches_reference_strictly -- --ignored --nocapture
Remove-Item Env:ALBUMEASY_BASELINE_CANDIDATE
```

The checker never invokes AlbumEasy or writes/overwrites a PDF. Generate the
candidate separately from the identical frozen TXT in an isolated directory using
the platform commands above. Reference and candidate use the **same installed
MuPDF version** for inspection. Candidate page traces must match completely:
glyph positions/fonts, vector paths, strokes and colors included. Only the
document filename and PDF metadata are excluded.

This is intentionally a strict **AlbumEasy-to-AlbumEasy core-v1** regression
checker, not a general native-renderer equivalence test. Frame extraction accepts
the measured AlbumEasy four-segment geometry and transform; unsupported layouts
fail explicitly. A future native comparator needs semantic geometry/text
comparisons and visual evidence rather than byte-identical page traces. Required
text assertions normalize whitespace; they alone do not certify text order or
line breaks, which are covered by strict page-trace comparison here.

### Execution report: 2026-11-03

Start: frozen TXT/PDF hashes rechecked successfully before edits. End: normal
tests, measured reference and two fresh candidate comparisons passed.

| Command/check | Exit | Passed / failed / ignored |
|---|---|---|
| `cargo test --test albumeasy_baseline --test external_album_backend -- --nocapture` | 0 | Baseline 3 / 0 / 2; imported-album regression 23 / 0 / 3 |
| `cargo test --test albumeasy_baseline reference_geometry_text_and_negative_controls -- --ignored --nocapture` | 0 | 1 / 0 / 0 |
| Each `QT_QPA_PLATFORM=offscreen timeout 60s /home/ray/work/AlbumEasy/AlbumEasy.sh -c album.txt` | 0 | Fresh renders in `first` and `repeat` both succeeded |
| Each candidate command below | 0 | 1 / 0 / 0, twice; full page traces match the reference |
| Final combined checker command below | 0 | 5 / 0 / 0; includes both external-tool tests |
| `cargo clippy --test albumeasy_baseline` | 0 | No checker warnings; existing application warnings unchanged |
| `rustfmt --edition 2021 --check tests/albumeasy_baseline.rs` and `git diff --check` | 0 | Formatting/whitespace passed |

Isolated run root:
`/home/ray/.copilot/session-state/80e32404-199a-4106-9649-659ef9662bab/files/t003-core-verification-6RHMrG`

Exact candidate commands (run from the repository):

```bash
ALBUMEASY_BASELINE_CANDIDATE=/home/ray/.copilot/session-state/80e32404-199a-4106-9649-659ef9662bab/files/t003-core-verification-6RHMrG/first/album.pdf cargo test --test albumeasy_baseline candidate_matches_reference_strictly -- --ignored --nocapture
ALBUMEASY_BASELINE_CANDIDATE=/home/ray/.copilot/session-state/80e32404-199a-4106-9649-659ef9662bab/files/t003-core-verification-6RHMrG/repeat/album.pdf cargo test --test albumeasy_baseline candidate_matches_reference_strictly -- --ignored --nocapture
ALBUMEASY_BASELINE_CANDIDATE=/home/ray/.copilot/session-state/80e32404-199a-4106-9649-659ef9662bab/files/t003-core-verification-6RHMrG/first/album.pdf cargo test --test albumeasy_baseline -- --include-ignored --nocapture
```

Filesystem impact: created only isolated `first`/`repeat` directories, each with
a copied `album.txt`, new `album.pdf`, two renderer logs and a checker log; Cargo
updated build artifacts. Frozen fixture bytes remained unchanged. No input was
moved/deleted; no production album or local config was touched. Existing
imported-album regression tests use self-cleaning temporary directories/DBs;
there was no country ingest or production DB operation.

Negative controls reject modified input SHA-256, non-finite measurements,
dimension/placement errors beyond 0.01 mm, changed page count/size, missing text,
malformed XML and changed stroke styles. Filename-only changes are accepted.

Error inventory:
- Initial checker compilation failed with `error[E0277]` at hash formatting:
  SHA-256 0.11's digest array does not implement `LowerHex`. Fixed by using the
  repository's existing per-byte hexadecimal formatting pattern; subsequent
  builds/tests passed. No commit was made with that failure.
- MuPDF: `warning: ICC support is not available`. Visible in checker logs;
  color-profile certification is not claimed.
- Existing application unused-import/variable/dead-code warnings remain unchanged.

Logic risks/limits: full language coverage, native equivalence, image/font/color
certification and Windows execution remain open. The baseline approval is
maintainer-reported; no physical-print automation is claimed. Full T003 remains
unchecked. GitHub epic #55 / sub-issue #56; task branch
`issue/56-t003-baseline-drift-checker`, local integration `development/native-pdf`.

## Drift protocol for subsequent implementation

1. Confirm frozen TXT checksum; feed the identical input and resolved assets to
   AlbumEasy and the internal renderer. Do not compare two separately generated
   TXT files or let user config change the fixture.
2. Compare page count, text/line breaks/order, frame sizes/placements,
   alignment, margins/borders and numbering against the PDF and expectations.
3. Use maximum dimension error 0.1 mm and placement error 0.25 mm for native
   acceptance; exact row alignment/order and required text are mandatory.
   The tighter 0.01 mm measurement check above validates the reference itself.
4. Rasterize both at the same DPI; review clipping, glyphs and overlays.
   PDF timestamps/object IDs/compression are not visual drift.
5. Add one command-family fixture as behavior is implemented, render it with
   the pinned external version first, review and freeze it, then require native
   conformance. Add failure cases separately; do not mock reference rendering.
6. Preserve every prior baseline. Intentional changes require a reviewed new
   version and written justification, not replacing the golden with current output.

The current app importer rejects some commands in this fixture and the native
renderer is absent. Those are known implementation gaps, not reasons to weaken
the reference. No new application test runner is introduced in this batch.

## Filesystem/database impact and error inventory

- Created frozen TXT/expectations/reference PDF under
  `tests/fixtures/albumeasy/core-v1/`, this report and the version-reference guide.
- Created isolated render/repeat directories with input copies, PDFs, logs,
  vector traces, extracted text and two PNG previews in session storage.
- Updated only related planning/gap documentation. No source inputs were moved,
  deleted or overwritten; no production albums/configs were altered.
- No database connections/writes or schema changes; country-ingest counts are
  not applicable.
- Renderer errors: **none**. MuPDF printed
  `warning: ICC support is not available`; this black/white fixture's geometry
  and text inspection succeeded. Colour-profile conformance is not claimed.
- Logic risks: timestamp-varying PDFs; installed-version dependence; unverified
  Windows/physical-print behavior; current importer/native coverage gaps; text
  extraction alone is insufficient without geometry/visual comparison.
- No commits, merges or pushes performed.
