# Footer numbering and colour: accepted reference and native import

Tracking [#87](https://github.com/amhbamhb/StampAlbumGen2/issues/87);
branch `issue/87-footer-numbering`, based on `4c94e22e`.

This increment maps `ALBUM_PAGES_FOOTER_NUMBER` and footer colour into the
standalone native input paths, reusing existing typed native footer/numbering
primitives. The first pass used **only AlbumEasy 4.5.0**. The maintainer
accepted the normal reference with "Accept reference; proceed with native
mapping." The native pass is implemented separately; previous fixtures and
these three AlbumEasy reference PDFs remain unchanged.

## Reference inputs

- [normal.txt](normal.txt): the collector-facing visual example, with
  `Page 1 of 2` then `Page 2 of 2`, centered in darkblue below the border.
- [album.txt](album.txt): two complete Letter pages, centered darkblue footer,
  explicit start 7, prefix `Page ` and suffix ` of $PAGES$`.
  Expected footer text is `Page 7 of 2`, then `Page 8 of 2`; total document
  count is independent of the final printed page number.
  This deliberately unusual result is retained as an import edge case, not a
  recommendation for collector-facing numbering.
- [defaults.txt](defaults.txt): the same album with three-argument footer
  numbering and the `COLOUR_ALBUM_FOOTER` spelling. Expected numbers 1 and 2.
  In-page captions retain the explicit-number case description because this
  derivative tests only the footer-command defaults, not different stamps.

Pinned help: `AlbumEasy45.0.qch`, SHA-256
`8b81707d861aacceb798b9f38efbbfdc3ea86fa49a57b7e343c3a25d5964d5be`.
Its footer-number section documents 3-6 positional arguments, default start 1,
optional before/after strings and actual total-page replacement. Colour names
accept both COLOR/COLOUR spellings. LR/RL alternation and other optional
argument combinations need native/reference tests before broader certification.

## Page and source provenance

Each page has six real image placements in three rows, a 0.2 mm border,
15 mm margins, 4 mm horizontal/14 mm vertical row spacing, title and readable
captions. Exact US Letter: 215.9 x 279.4 mm. Frame dimensions add 6 mm total
to each physical dimension (3 mm per edge); explicit reference image padding
is 3 mm with preserved aspect ratio.

Reuse the [verified source manifest](../stamp-pagination-v1/source-records.json)
and unchanged images:

| DB record | SG | Colnect ID | Physical size | Frame |
|---:|---|---:|---|---|
| 150 | 1125a | 700029 | 52 x 29 mm | 58 x 35 mm |
| 151 | 1137a | 700273 | 26 x 29 mm | 32 x 35 mm |
| 152 | 1137b | 700274 | 26 x 29 mm | 32 x 35 mm |
| 156 | 1148a | 700272 | 52 x 29 mm | 58 x 35 mm |

No new database lookup or inferred dimensions are needed for these verified
records. No source database, image or previous reference is modified.

## Reproduction

From this fixture directory on Linux:

```sh
timeout 60s xvfb-run -a -s '-screen 0 1280x1024x24' \
  env QT_QPA_PLATFORM=xcb /home/ray/work/AlbumEasy/AlbumEasy.sh -c normal.txt
timeout 60s xvfb-run -a -s '-screen 0 1280x1024x24' \
  env QT_QPA_PLATFORM=xcb /home/ray/work/AlbumEasy/AlbumEasy.sh -c album.txt
timeout 60s xvfb-run -a -s '-screen 0 1280x1024x24' \
  env QT_QPA_PLATFORM=xcb /home/ray/work/AlbumEasy/AlbumEasy.sh -c defaults.txt
```

Windows: open each TXT in AlbumEasy 4.5.0, preserving the relative asset
paths, and export its PDF beside the input. No native renderer is used in
this first pass. Review the complete pages and footer position/colour,
sequence, prefix/suffix and default numbers. Print at Actual size / 100% on
US Letter without Fit/Shrink. Reference acceptance and actual-size print
review is pending; screen reference acceptance is recorded above.

All three native input routes now have separate PDFs and regression coverage.
The versioned Denmark derivative restores footer colour/numbering only;
margin-text colour and save/restore remain separate. Do not overwrite older
proofs or claim full command compatibility.
No production integration, OKF generation or PR is part of this increment.

## First-pass execution results

The initial album/default AlbumEasy commands above exited 0 and produced two-page exact
Letter PDFs. [album.pdf](album.pdf) has 12 visible image placements and
extracted footer text `Page 7 of 2` and `Page 8 of 2`.
[defaults.pdf](defaults.pdf) displays numbers 1 and 2 using the British
colour-command spelling. The first/final page rasters were inspected:
darkblue centered footers sit below the border, separated from the final
captions; measured real stamps, row spacing and complete page context are
retained. AlbumEasy places the title only on the first page in this reference.

The subsequently requested [normal.pdf](normal.pdf) also rendered with
AlbumEasy, exit 0, two exact Letter pages. Extracted footers read `Page 1 of 2`
and `Page 2 of 2`; the first-page raster retains the full page composition
and clear darkblue footer. Its new TXT/PDF do not replace the edge-case pair.

Verification commands: `pdfinfo album.pdf`, `pdfinfo defaults.pdf`,
`pdfimages -list album.pdf`, `pdftotext -layout album.pdf -`,
`pdftotext -layout defaults.pdf -`, and
`pdftoppm -f 1 -l 1 -scale-to 1200 -png -singlefile album.pdf <session-preview>`,
with a corresponding final-page raster. These inspect the AlbumEasy outputs,
not native rendering.

Filesystem impact: created this fixture directory, two TXT inputs, two
reference PDFs and README. Previews stay outside the repository in the
session workspace. No source files/assets were changed, deleted, moved or
relocated. Database impact: none; no target DB, schema changes, inserted/
updated/skipped rows or duplicate detection applies. No ingest flow ran.
No errors occurred during rendering. Logic risk: the actual page count (2)
must not be confused with the last displayed number (8). Optional defaults,
footer placement and global colour capture need native regression coverage.
The normal reference was subsequently accepted; actual-size print review
remains pending.

## Native pass and restart checkpoint

| Case | AlbumEasy reference | TXT native | Raw AlbumEasy native | Native-language |
|---|---|---|---|---|
| Normal start 1 | [normal.pdf](normal.pdf) | [normal-txt-native.pdf](normal-txt-native.pdf) | [normal-raw-native.pdf](normal-raw-native.pdf) | [normal-native.pdf](normal-native.pdf) |
| Explicit start 7 | [album.pdf](album.pdf) | [album-txt-native.pdf](album-txt-native.pdf) | [album-raw-native.pdf](album-raw-native.pdf) | [album-native.pdf](album-native.pdf) |
| Optional defaults | [defaults.pdf](defaults.pdf) | [defaults-txt-native.pdf](defaults-txt-native.pdf) | [defaults-raw-native.pdf](defaults-raw-native.pdf) | Explicit native settings tested above |

Raw inputs are byte copies of their corresponding TXT; they contain no native
commands. Native [normal.stampalbum](normal.stampalbum) and
[album.stampalbum](album.stampalbum) start with `STAMPALBUM 1` and use:

```text
NATIVE_PAGE_FOOTER_NUMBER (HN 9 C 1 "Page " " of $PAGES$" 0 0 darkblue)
```

The nine arguments are font, point size, position, signed 32-bit start,
quoted prefix/suffix, signed padding mm, nonnegative minimum height mm and
named colour. These native settings are explicit; no new TOML keys or
runtime configuration defaults are added. The zero padding/minimum height
in the compatibility route represent AlbumEasy's default footer settings,
not configurable native defaults.

Imported `ALBUM_PAGES_FOOTER_NUMBER` accepts 3-6 arguments: default start 1,
empty prefix/suffix. `COLOR_ALBUM_FOOTER` and `COLOUR_ALBUM_FOOTER` set the
pre-page global colour, even when written after the footer definition.
Changes after `PAGE_START` fail explicitly rather than being silently
ignored. L/C/R/LR/RL positioning has native tests; only C has the newly
accepted AlbumEasy visual proof. This is bounded coverage, not certification
of every optional form or state transition.

Every output is two exact Letter pages and 12 visible image placements.
Tests verify text/counts, darkblue, the footer below the border and above
the physical edge, stamp outlines plus page border, explicit/optional starts,
alternating positions, invalid arguments/colours, duplicate native commands,
overflow and preservation of existing output on failure. Native and TXT
image dimensions agree within 0.02 points after sorting placements. Their
layouts are not identical: TXT preserves the two explicit pages (6 images
per page); native sections auto-paginate (8 then 4). Native title repetition,
earlier row placement and omitted native section-heading text remain visible
differences, not hidden parity claims.

Both normal TXT/raw page rasters are identical at 1200 pixels. Extracted
`Page` bounds match the reference on both pages within 0.003 points:
AlbumEasy x=282.982510, y=753.435640; TXT native x=282.980000,
y=753.438000 (PDF text-bounding-box coordinates from top). Screen inspection
of reference page 1, TXT page 1 and native page 2 retained border, frame
clearance, real images and the visible numbered footer. Native human
visual/Actual-size Letter print acceptance is still pending.

### Native reproduction and execution report

From the repository root:

```sh
cargo build --example render_stampalbum
target/debug/examples/render_stampalbum tests/fixtures/albumeasy/footer-numbering-import-v1/normal.txt tests/fixtures/albumeasy/footer-numbering-import-v1/normal-txt-native.pdf
target/debug/examples/render_stampalbum tests/fixtures/albumeasy/footer-numbering-import-v1/normal-raw.stampalbum tests/fixtures/albumeasy/footer-numbering-import-v1/normal-raw-native.pdf
target/debug/examples/render_stampalbum tests/fixtures/albumeasy/footer-numbering-import-v1/normal.stampalbum tests/fixtures/albumeasy/footer-numbering-import-v1/normal-native.pdf
target/debug/examples/render_stampalbum tests/fixtures/albumeasy/footer-numbering-import-v1/album.txt tests/fixtures/albumeasy/footer-numbering-import-v1/album-txt-native.pdf
target/debug/examples/render_stampalbum tests/fixtures/albumeasy/footer-numbering-import-v1/album-raw.stampalbum tests/fixtures/albumeasy/footer-numbering-import-v1/album-raw-native.pdf
target/debug/examples/render_stampalbum tests/fixtures/albumeasy/footer-numbering-import-v1/album.stampalbum tests/fixtures/albumeasy/footer-numbering-import-v1/album-native.pdf
target/debug/examples/render_stampalbum tests/fixtures/albumeasy/footer-numbering-import-v1/defaults.txt tests/fixtures/albumeasy/footer-numbering-import-v1/defaults-txt-native.pdf
target/debug/examples/render_stampalbum tests/fixtures/albumeasy/footer-numbering-import-v1/defaults-raw.stampalbum tests/fixtures/albumeasy/footer-numbering-import-v1/defaults-raw-native.pdf
cargo test --no-fail-fast --test native_pdf_footer_import --test native_pdf_stamp_background --test native_pdf_albumeasy --test native_pdf_album --test native_pdf_footer_number --test native_pdf_page_footer --test native_pdf_pagination_proofs --test native_pdf_stamp_image --test native_pdf_stamp_items --test native_pdf_pagination --test external_album_backend --test native_pdf_stamp_rows --test native_pdf_stamp_text --test native_pdf_row_alignment --test native_pdf_explicit_settings
cargo test --test native_pdf_footer_import regenerate_denmark_footer_proofs -- --ignored --nocapture
cargo check --all-targets
cargo clippy --lib --example render_stampalbum --test native_pdf_footer_import
cargo fmt --check
git diff --check
```

Windows uses the same Cargo commands, or
`target/debug/examples/render_stampalbum.exe` for the built example; all
fixture asset paths are relative. Linux execution only has been verified.
No server, Docker, browser E2E, authentication or ingestion tier applies to
this standalone file-to-PDF slice.

Results: build and all eight standalone invocations exit 0; focused suite
99 passed, 0 failed, 15 ignored; Denmark regeneration 1 passed; all-target
checking passes. Focused Clippy passes with pre-existing library warnings;
new test regex-in-loop warnings were corrected. Tests use temporary output
directories and do not overwrite accepted references.

Error inventory and fixes during development:

- `native_pdf.footer font/size_pt/text: Text must be nonempty WinAnsi text; only newline controls are supported` during initial numbered-style
  creation: initialise the existing style with `"1"`; the numbering wrapper
  always replaces it with the resolved label before rendering.
- An added negative-padding rejection test failed because signed padding is
  deliberately supported by the shared footer primitive. Corrected the test
  to reject nonfinite padding; the runtime contract was not restricted.
- Invalid footer fonts now include the originating AlbumEasy line and
  command in diagnostics. Expected-invalid tests explicitly reject numeric/
  quoted starts, unknown positions/fonts/colours, overflow and mid-page
  footer changes. No silent skipping or successful fallback is used.

Filesystem impact: added three raw and two native-language `.stampalbum`
inputs (five total), eight native PDFs and the new
regression test; updated this README and the long-running plan/matrix.
No existing reference/source/image was deleted, moved or relocated.
Preview PNGs, text-bounds HTML and build/test logs remain in the session
workspace. DB impact: none; no DB opened, schema checks/row changes/duplicate
counts are not applicable. No Colnect inbox or other ingest ran.

The [Denmark v3 report](../denmark-footer-native-v3/README.md) records the
five-to-three deferral reduction. Resume future work from the current
[plan](../../../../docs/reports/NATIVE_PDF_IMPLEMENTATION_PLAN.md) and keep
the three-input regression gate. Do not integrate production routing,
regenerate OKF, create a PR, or implement #88 as part of this checkpoint.

### Validation verdict

| Tier | Outcome | Exit code | Evidence / gap |
|---|---|---:|---|
| Standalone startup/build | PASS | 0 | Built example; eight file-to-PDF invocations completed |
| Focused integration/regressions | PASS | 0 | 99 passed, 15 ignored; final footer-only rerun 5 passed, 1 ignored |
| Opt-in Denmark regeneration | PASS | 0 | 1 passed; both inputs 16 pages, 41 images |
| All-target check / focused Clippy / formatting | PASS | 0 | Existing library warnings only; no new footer-test warnings |
| Native output / raster probes | PASS | 0 | Two normal TXT/raw page rasters identical; reference footer bounds measured |
| Browser/infra tiers | Not applicable | n/a | Standalone renderer, no server/DB; no Docker/Node/Playwright tier selected |
| Human native visual/print review | UNVERIFIED | n/a | Letter Actual-size printing and maintainer native acceptance pending |

Overall: **NEEDS_SIGNOFF** for human native/print acceptance, not full-language
certification. Automated scoped validation passes; Windows runtime remains
unverified. Corrected development failures above are not unresolved test
failures.

The maintainer subsequently requested commit/push on
`issue/87-footer-numbering`. Precommit checks:
`cargo fmt --check`, `git diff --check`, and
`cargo test --test native_pdf_footer_import --test native_pdf_albumeasy --test native_pdf_album --test native_pdf_footer_number --test native_pdf_page_footer`
all exited 0 (33 tests passed, 4 ignored). Test outputs were temporary;
no references, inputs, images or databases were modified, moved or deleted.
No new errors or logic risks were found. No PR or OKF regeneration is
authorized by this commit/push request.

## Separate future native numbering idea (#88)

The maintainer requested recording year/section-local numbering separately
while this item finishes AlbumEasy-compatible footer import. See
[#88](https://github.com/amhbamhb/StampAlbumGen2/issues/88).
Native TOML settings could select document or year/section scope, restart at
1 by default, and allow a custom start such as 10. Three-page year sections
would each run `Page 1 of 3` through `Page 3 of 3`, independent of their
positions in the overall document. Ten pages starting at 10 display 10–19;
the number printed on the page must remain distinct from the actual page count.

Decide separate tokens/policies for displayed number, section ordinal,
section count and document count before implementation. Do not change
AlbumEasy's `$PAGES$` meaning to make this work. Imported/native files need
explicit section boundaries; do not infer them from arbitrary title text.
When implemented, comments immediately above the new template TOML keys must
explain these native-only meanings and examples, with matching full-example
help and configuration inventory documentation. No speculative TOML keys or
runtime defaults are added in the current compatibility-only item.
