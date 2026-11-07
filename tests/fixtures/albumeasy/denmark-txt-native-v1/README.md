# Denmark 1989: bounded native AlbumEasy-input proof

This fixture tests an existing collector album, not a newly authored command
reference. The original source is untouched. Eight occurrences of six
currently unsupported commands are explicitly commented as `# NATIVE-TODO:`
in the supported copy and recorded in the
[long-running plan](../../../../docs/reports/NATIVE_PDF_IMPLEMENTATION_PLAN.md#required-denmark-mappings-still-deferred).
The original TXT does **not** yet render unchanged.
Preparation also trims trailing whitespace on supported command lines in the
adapted copies; original source bytes are never normalized or overwritten.

| Input | Output | Result |
|---|---|---|
| [Supported AlbumEasy TXT](Denmark-1989-1989-supported.txt) | [Native PDF](Denmark-1989-1989-native.pdf) | 16 pages, 41 image placements |
| [Identical raw-command `.stampalbum`](Denmark-1989-1989-raw.stampalbum) | [Raw-input native PDF](Denmark-1989-1989-raw-native.pdf) | 16 pages, 41 image placements |
| Original historical AlbumEasy album | [Supplied reference PDF](Denmark-1989-1989-reference.pdf) | Copied without alteration; not newly rendered or certified against current TXT |

The raw `.stampalbum` contains only AlbumEasy command lines and comments:
there is no `STAMPALBUM 1` header or native command syntax. Both compatibility
inputs use the bounded interpreter and shared native PDF writer, without an
AlbumEasy subprocess. The separate native-language case is covered by the
[accepted image-controls fixture](../stamp-image-controls-v1/README.md).
This work does not integrate the renderer into production.

## Source and asset provenance

- Supplied source: `/media/ray/program/Rust/StampAlbumGen2/Albums/Denmark/Denmark-1989-1989.txt`.
  SHA-256: `a802543f810ec9595175c5ee5b5302b28edd1e33961e340107821b3dd4c4cf68`.
- Supplied matching historical PDF: the same directory's
  `Denmark-1989-1989.pdf`.
  SHA-256: `b30881a9fdf072d7e8689cf88ecbc067ed47acc7caeed9df4380fd0edc316530`.
- All 41 referenced images, including the 4-bit indexed PNG flag, are copied
  unchanged into [images/](images/). Relative paths are retained. Their
  [checksum manifest](source-images.sha256) can verify both the fixture copies
  and original images.
- Source `STAMP_DIAG` comments and collector captions are retained. Explicit
  frame dimensions come from the TXT, not from a new Colnect dimension query.
  This run does not independently certify those physical measurements.
- The source has 16 explicit `PAGE_START` commands. Its paper size is
  **216.0 x 279.4 mm**, approximately Letter, with 10 mm margins; it is not
  silently changed to exact 215.9 mm Letter width.

## Deferred source lines

| Original line(s) | Command | Visual/state consequence |
|---|---|---|
| 42 | `COLOR_ALBUM_FOOTER` | Footer colour not mapped yet. |
| 45 | `COLOR_ALBUM_MARGIN_TXT` | Margin-text colour not mapped yet. |
| 49, 72, 79 | `COLOR_STAMP_BACKGROUND` | Black stamp backgrounds are absent. |
| 52 | `ALBUM_PAGES_FOOTER_NUMBER` | Numbered footer is absent. |
| 68 | `ALBUM_STAMP_SETTINGS_SAVE` | No saved stamp-state snapshot. |
| 83 | `ALBUM_STAMP_SETTINGS_RESTORE` | No restoration; intervening implemented settings remain active. |

Comments exist only in the supported fixture, never as an automatic runtime
fallback. Normal rendering rejects unsupported input. Supported command names
with unsupported variants or malformed arguments still error, rather than
being silently commented or approximated.

## Reproduction

From the repository root, on Linux or Windows with Cargo:

```sh
cargo run --example render_stampalbum -- \
  tests/fixtures/albumeasy/denmark-txt-native-v1/Denmark-1989-1989-supported.txt \
  tests/fixtures/albumeasy/denmark-txt-native-v1/Denmark-1989-1989-native.pdf
cargo run --example render_stampalbum -- \
  tests/fixtures/albumeasy/denmark-txt-native-v1/Denmark-1989-1989-raw.stampalbum \
  tests/fixtures/albumeasy/denmark-txt-native-v1/Denmark-1989-1989-raw-native.pdf
```

For PowerShell, enter each command on one line rather than using the POSIX
`\` continuation. The built example is
`target/debug/examples/render_stampalbum` on Linux and
`target/debug/examples/render_stampalbum.exe` on Windows.
Rendering requires only the checked-in inputs/assets, not the original album
directory, a database or AlbumEasy. Windows execution is not yet verified.

The opt-in preparation test requires the original Linux source directory:

```sh
cargo test --test native_pdf_albumeasy \
  render_denmark_commented_test_copy -- --ignored --nocapture
```

It creates/replaces the supported input and raw copy, copies only referenced
images, renders the TXT PDF from the **fixture** asset root, checks 16 pages
and 41 image draw operations, and verifies that the original TXT bytes are
unchanged. It does not regenerate the raw PDF or historical reference copy.

On Linux, verify all copied images from this fixture directory:

```sh
sha256sum -c source-images.sha256
```

On Windows, use `Get-FileHash -Algorithm SHA256` on each referenced image and
compare with the manifest. Refresh the manifest only when intentionally
refreshing the source assets.

## Visual/print review

PDF structure and image counts passed. Native and supplied-reference page 6
were rasterized and inspected. The native output retains the large sheet and
two smaller stamps, text, three-line page border and header. Its white stamp
backgrounds and missing numbered footer differ intentionally because those
commands are deferred. The historical supplied PDF also lacks the border/
header visible in the current TXT/native output; it is not verified as a
render of this exact current TXT. Text baselines/spacing differ. Long captions
in both renditions can run into adjacent caption columns and require review.

No exact visual parity or collector acceptance is claimed. The native PDFs
have not been printed. Print on US Letter at **Actual size / 100%**, with Fit,
Shrink and automatic scaling disabled; preserve the source's 216 mm width.
Review cover and multiple stamp pages, mixed-size rows, background/numbering
omissions, mounting clearances, image fits, captions and page spacing before
approving this adapted compatibility proof.

## Execution report

**Outcome:** standalone input rendering passes; full untouched-source
compatibility and human visual/print acceptance remain incomplete.

### Commands and results

1. `cargo fmt && cargo test --test native_pdf_albumeasy render_denmark_commented_test_copy -- --ignored --nocapture`
   - Final preparation run: exit 0, one passed; 16 pages, 41 image draws,
     41 copied images, eight explicit deferred occurrences.
2. Focused regression command:

   ```sh
   cargo test --no-fail-fast \
     --test native_pdf_albumeasy --test native_pdf_stamp_image \
     --test native_pdf_album --test native_pdf_stamp_items \
     --test native_pdf_stamp_rows --test native_pdf_stamp_text \
     --test native_pdf_pagination --test native_pdf_pagination_proofs \
     --test native_pdf_row_alignment --test native_pdf_explicit_settings \
     --test external_album_backend
   ```

   Exit 0: **77 passed, zero failed, 11 ignored**. The external-album
   backend's 23 passing tests are mock-renderer coverage, not evidence that
   production imports are native-only.
3. `cargo build --example render_stampalbum`, followed by actual example
   binary executions:

   ```sh
   target/debug/examples/render_stampalbum tests/fixtures/albumeasy/stamp-image-controls-v1/album.txt tests/fixtures/albumeasy/stamp-image-controls-v1/album-txt-native.pdf
   target/debug/examples/render_stampalbum tests/fixtures/albumeasy/stamp-image-controls-v1/album.stampalbum tests/fixtures/albumeasy/stamp-image-controls-v1/album-native.pdf
   target/debug/examples/render_stampalbum tests/fixtures/albumeasy/stamp-image-controls-v1/album-raw.stampalbum tests/fixtures/albumeasy/stamp-image-controls-v1/album-raw-native.pdf
   target/debug/examples/render_stampalbum tests/fixtures/albumeasy/denmark-txt-native-v1/Denmark-1989-1989-supported.txt tests/fixtures/albumeasy/denmark-txt-native-v1/Denmark-1989-1989-native.pdf
   target/debug/examples/render_stampalbum tests/fixtures/albumeasy/denmark-txt-native-v1/Denmark-1989-1989-raw.stampalbum tests/fixtures/albumeasy/denmark-txt-native-v1/Denmark-1989-1989-raw-native.pdf
   ```

   All exited 0: three one-page Letter outputs and two 16-page Denmark
   outputs. No AlbumEasy executable is called by either rendering path.
4. `cargo clippy --example render_stampalbum --test native_pdf_albumeasy --test native_pdf_stamp_image`
   - Exit 0. Existing unrelated library warnings remain; the new test's
     unnecessary-vector warning was corrected.
5. `pdfinfo tests/fixtures/albumeasy/denmark-txt-native-v1/Denmark-1989-1989-native.pdf`
   - 16 pages, 612.283 x 792 points, unencrypted PDF 1.7, approximately
     44.7 MB. `pdfimages -list` independently counted 41 image placements
     in each Denmark output.
6. `cmp` checked each TXT/raw input pair. `pdftotext` and `pdftoppm -scale-to 1200 -png -singlefile`
   followed by `cmp` verified identical extracted text and raster for the
   image-controls TXT/raw native outputs. This is native route equivalence,
   not AlbumEasy parity.

### Failure inventory and fixes

| Affected input/test | Error | Resolution |
|---|---|---|
| Unchanged Denmark TXT | `Unsupported AlbumEasy command COLOR_ALBUM_FOOTER` | Record all six missing names/eight occurrences; prepare explicitly commented copy. Still unsupported unchanged. |
| Denmark mixed-height row | `Imported AlbumEasy row exceeds the page height; add PAGE_START before this row` | Reserve each item's own frame-plus-caption height. Added mixed-height regression. |
| Denmark indexed flag PNG | `Insufficient PNG image data` | Decode palette/low-bit PNGs before native embedding. Added indexed-PNG and RGBA soft-mask regression. |
| Denmark image fit | `native_pdf.stamp_image cannot fit the image into its box` | Permit only `1e-9` mm comparison roundoff. |
| `invalid_asset_preserves_existing_pdf` | `assertion failed: error.contains("Cannot load native stamp image")` | Restore the established error prefix in the new PNG path. Focused regression rerun passed. |
| Precommit `cargo test --all` | `missing field options in initializer of StampImageSettings` in `examples/stamp_frame_proofs.rs` | Initialize the added image-options field with the existing native default; preserve proof behavior. |
| Precommit `page_fit_regression_for_conservative_buffer` (#85) | `expected forced page 2 with PAGE_START + ROW_START_ES` | Correct the stale test expectation to the existing generator's `ROW_START_FS`, matching the companion overflow regression; preserve the required forced-page sequence. No production page-fit behavior changed. |

### Before/after filesystem and database impact

- Created this fixture directory, its `images/` directory, two prepared
  inputs, two native PDFs, the historical reference copy, image checksum
  manifest and this README. Native PDFs are approximately 44.7 MB each;
  copied images total approximately 28 MB.
- Added the image-controls raw `.stampalbum`/PDF and regenerated only its
  unaccepted native outputs. Accepted AlbumEasy TXT/PDF remain unchanged.
- Source album TXT/PDF/images were only read/copied: no source inputs were
  deleted, moved, relocated or overwritten. Temporary automated-test
  directories clean themselves up. Raster/text/lint scratch outputs stay
  outside the repository in the session workspace.
- **Database target:** none. These preparation/render tests do not open,
  import into, migrate or modify any SQLite database. Schema checks and
  inserted/updated/skipped rows/duplicate detection are **not applicable**.
- No ingest flow was run; no inbox files or country databases were affected.

### Logic risks and next mappings

The interpreter is a bounded slice, not full AlbumEasy conformance. Page
settings after `PAGE_START`, advanced image overloads, additional row
alignment modes, shapes, complete heading forms, footer/background state and
save/restore remain open. Caption widths and exact font/image DPI semantics
need reference testing. Explicit TXT frame sizes must not be relabeled as
verified Colnect physical stamp sizes. PNG/PDF size optimization is separate
from visual correctness. Swiss early/modern measured examples remain a
separate selection task; the earlier candidate identities need verification.

Runtime verdict for this standalone-file task: Cargo build/start and native
input execution **PASS** (exit 0); focused integration/regressions **PASS**
(exit 0, 77 passed/11 ignored); Denmark opt-in **PASS** (exit 0, one passed).
Docker/browser/server tiers do not apply to this offline Rust PDF example.
Windows runtime and physical print validation are **UNVERIFIED**.
Overall: **NEEDS SIGNOFF**, not full compatibility certification.

### Final commit gate

After fixing the proof-example initializer and the stale page-fit regression
expectation, `cargo fmt --check && cargo test --all` completed with exit 0:
**421 passed, zero failed, 32 ignored**, across 105 test/doc-test results.
`cargo test --test page_fit_regression --test page_break_overflow` also passed
both forced-page regression tests. The opt-in Denmark preparation test was
rerun separately: exit 0, one passed, 16 pages and 41 image draws, with
original source bytes unchanged. Formatting and `git diff --check` passed.

The full workspace PDF-import tests created extracted-image directories
under `download-pdf/` for Great Britain and United States 1970–1979. Those
new test-only directories were removed after validation; their existing
source PDFs were preserved and no generated download assets were staged.
Human visual/print signoff and Windows runtime validation remain pending.
