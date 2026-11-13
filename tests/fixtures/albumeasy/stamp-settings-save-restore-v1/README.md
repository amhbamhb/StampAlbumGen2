# Stamp-settings save/restore: AlbumEasy-only first pass

Tracking [#89](https://github.com/amhbamhb/StampAlbumGen2/issues/89), branch
`issue/89-stamp-settings-save-restore`, based on `e090368e` (#87).
The maintainer accepted the reference: "Accept reference; proceed with native
mapping." Native implementation is now a bounded standalone slice; see below.
No production integration, PR or OKF
regeneration. The maintainer subsequently requested commit/push; see the
precommit record below.

## Reference and observable cases

[album.txt](album.txt) contains only AlbumEasy commands.
[album.pdf](album.pdf) was generated solely by AlbumEasy 4.5.0:
two exact US Letter pages (215.9 x 279.4 mm / 612 x 792 points),
12 stamp placements, eight visible images and four intentionally hidden
images. Both pages retain a 0.2 mm border, 15 mm margins, 4 mm horizontal/
14 mm vertical spacing, real stamp assets and a darkblue numbered footer.

Page 1 has matching baseline/restored rows around a temporary-settings row:

- Baseline/restored: darkblue frames/captions, lightblue fill, greyscale,
  aspect-preserving fit, 3 mm image padding, zero size adjustment.
- Temporary: red frames/captions, white fill, colour, non-aspect-preserving
  fit, 15% padding and +2 mm total frame size adjustment.
- Physical stamp dimensions are unchanged. Baseline boxes are 58 x 35 mm
  and 32 x 35 mm (3 mm clearance per edge around 52 x 29 / 26 x 29 mm
  stamps); temporary boxes are 60 x 37 / 34 x 37 mm. The deliberate
  temporary padding/scaling changes are not new physical measurements.

Page 2 verifies a second save/restore cycle after the first is consumed:
hidden images / BLANK outline / lightblue fill, temporarily shown colour
images with solid red frames, then restored hidden/BLANK/lightblue.
Its blank-looking boxes are intentional, not missing assets.

The [repeated-save.txt](repeated-save.txt) negative probe tries save A,
save B before restoration, then two restores. AlbumEasy reports:

```text
Error in line 16: ALBUM_STAMP_SETTINGS_SAVE command - Settings have been saved previously, please restore before attempting to save again.
Error in line 26: ALBUM_STAMP_SETTINGS_RESTORE command - Settings have not yet been saved, please save before attempting to restore them.
```

No repeated-save PDF was created. Despite these errors the launcher returned
exit 0: command exit status alone is not a valid reference-render success
test. These diagnostics establish a single outstanding snapshot, consumed
by restore, not a nested stack or reusable snapshot. The positive reference
also establishes that a fresh save after a completed restore is valid.

## Provenance and saved-state coverage boundary

Reuse [source-records.json](../stamp-pagination-v1/source-records.json) and
unchanged assets:

| Record | SG | Colnect ID | Physical size | Asset |
|---:|---|---:|---|---|
| 150 | 1125a | 700029 | 52 x 29 mm | [us-150.png](../stamp-pagination-v1/us-150.png) |
| 151 | 1137a | 700273 | 26 x 29 mm | [us-151.png](../stamp-pagination-v1/us-151.png) |
| 152 | 1137b | 700274 | 26 x 29 mm | [us-152.png](../stamp-pagination-v1/us-152.png) |
| 156 | 1148a | 700272 | 52 x 29 mm | [us-156.png](../stamp-pagination-v1/us-156.png) |

All four image SHA-256 hashes match that manifest. No DB query or source
image modification was needed.

Pinned AlbumEasy 4.5.0 help (`AlbumEasy45.0.qch`, SHA-256
`8b81707d861aacceb798b9f38efbbfdc3ea86fa49a57b7e343c3a25d5964d5be`)
lists the saved settings: both mm/percentage image-setting modes,
aspect ratio, visibility, grayscale, box adjustment, outer/inner border
styles and inner border geometry, heading padding, footer-text padding/
alignment, border/inner-border/heading/text/background colours.

This reference exercises image modes/aspect/grayscale/visibility, size
adjustment, solid/blank outer borders, border/text/background colours.
It does not certify inner borders/styles, heading/footer padding/alignment
or heading/inner-border colours. Those surfaces require additional command
mapping/reference proofs; do not silently substitute or omit their state.
Currently unsupported commands must continue to fail explicitly.
Do not snapshot unrelated page settings or row layout state by assumption.

## Exact execution and impact report

From this fixture directory on Linux:

```sh
timeout 60s xvfb-run -a -s '-screen 0 1280x1024x24' env QT_QPA_PLATFORM=xcb /home/ray/work/AlbumEasy/AlbumEasy.sh -c album.txt
timeout 60s xvfb-run -a -s '-screen 0 1280x1024x24' env QT_QPA_PLATFORM=xcb /home/ray/work/AlbumEasy/AlbumEasy.sh -c repeated-save.txt
pdfinfo album.pdf
pdfimages -list album.pdf
pdftotext -layout album.pdf -
pdftoppm -f 1 -l 2 -scale-to 1200 -png album.pdf <session-preview>
```

Windows: open each TXT in AlbumEasy 4.5.0 using the unchanged relative
assets; export the positive PDF and inspect the negative command errors.
Linux execution only is verified. Print the positive reference at Actual
size / 100% on US Letter, without Fit/Shrink.

Positive render exit 0 with success message and verified PDF. Eight visible
image placements: page 1 has six, grayscale/RGB/grayscale in pairs;
page 2 has two RGB images. The matching first/restored image pixel sizes
and reported DPI confirm restoration of image fitting on page 1.
Both page rasters were inspected: complete page borders, readable captions,
frames and deliberate hidden-image fills remain visible, with footers
`Page 1 of 2` and `Page 2 of 2`.

Negative probe: semantic failure despite exit 0, exact error inventory above,
no output PDF. No other rendering errors occurred.

Filesystem impact: created this directory, two TXT files, the positive PDF
and this README. No inputs/assets/older proofs were deleted, moved or
relocated. Preview PNGs remain in the session workspace. Database impact:
none; no target DB opened, schema changes, inserted/updated/skipped rows or
duplicate detection applies. No ingestion/Colnect inbox flow ran.

Logic risks: preserve per-item captured state rather than retroactively
recolouring existing items; restore both padding unit and numeric values;
consume a snapshot only on successful restore; distinguish explicitly
unsupported state controls from valid save/restore; reject repeated save
and missing-snapshot restore without replacing existing output. Future
native tests must assert actual PDF state/geometry, not just command
recognition or return codes.

## Native pass after reference acceptance

1. Implement the snapshot lifecycle for all currently mapped stamp state;
   keep unsupported controls explicit and the complete saved-state checklist
   open until their mappings exist.
2. Add native-language and byte-identical raw AlbumEasy `.stampalbum` inputs,
   preserving this AlbumEasy-only reference/PDF.
3. Render all three input routes natively and compare page/frame/image/text
   state against the reference. Retain existing footer/background/image
   regressions and realistic complete-page review.
4. Create Denmark v4 from v3, restoring only settings save/restore, leaving
   margin-text colour deferred. Never edit earlier bundles.
5. Record exact test commands/counts, output/error preservation, filesystem/
   database impact and Windows/human print gaps before commit approval.

The ordered checklist above remains the standard for future updates.

### Implemented snapshot contract

The compatibility interpreter now accepts `ALBUM_STAMP_SETTINGS_SAVE` and
`ALBUM_STAMP_SETTINGS_RESTORE`, with no arguments. A single saved snapshot
captures every currently mapped stamp drawing setting: image options
(padding unit/values, stretch, aspect ratio, grayscale, visibility),
size adjustment, solid/blank outline, background, border/text/heading colours.
Restoration consumes the snapshot. Repeated save, restore without save
and nonempty argument lists are explicit line-numbered errors.
Already placed items retain their own captured settings; page geometry,
row font/spacing and page backgrounds are not added to the stamp snapshot.

Native-language equivalents are:

```text
NATIVE_STAMP_SETTINGS_SAVE ()
NATIVE_STAMP_SETTINGS_RESTORE ()
```

These capture the native document's currently mutable per-item image policy
and background/outline style. Frame width/colour/additions and text policy
remain fixed document settings, not newly mutable properties; this slice
does not add speculative style commands. Unsupported AlbumEasy inner-border/
heading/footer controls still fail rather than silently losing their state.
An outstanding save at document end is not newly prohibited; unmatched-final
save conformance is not certified by these reference probes.

### Three inputs and direct native outputs

| Input | Direct native PDF |
|---|---|
| [Original accepted TXT](album.txt) | [TXT-native PDF](album-txt-native.pdf) |
| [Byte-identical raw AlbumEasy `.stampalbum`](album-raw.stampalbum) | [Raw-native PDF](album-raw-native.pdf) |
| [Native-language `.stampalbum`](album.stampalbum) | [Native-format PDF](album-native.pdf) |

All produce two exact Letter pages, 12 stamp placements and eight visible
images. The TXT preserves six stamp placements per page; its two 1200-pixel
rasters are identical to the raw route. Baseline/restored frame dimensions
are explicitly checked (58 x 35 / 32 x 35 mm), temporary frames 60 x 37 /
34 x 37 mm, and first/restored image transform dimensions agree within
0.01 points. Tests verify fill order, border/text/heading colour changes,
visibility, outline counts, native captured grayscale/padding/style and
strict lifecycle/unsupported-control errors.

Screen inspection of both TXT-native pages and native page 1 confirms
greyscale restoration, lightblue fills, frames, readable captions and footers.
The page-6 Denmark v4 preview retains its black mixed-height sheet frame
and numbered footer. No pixel-parity claim against AlbumEasy is made:
TXT-native rows start earlier, captions/heading spacing differ, its page
title repeats and page text currently inherits the stamp text colour.
That last page-text/stamp-colour coupling predates this slice and remains
an explicit general-page-text mapping obligation, not part of the saved
stamp-state contract.

Native-language output retains fixed darkblue frame colour, fixed 6 mm
total frame additions and black captions. It does not reproduce temporary
red borders/captions or +2 mm frame changes from the TXT reference. Native
15% padding is additional to its geometry's reserved clearance, whereas
AlbumEasy padding applies within the full box; temporary native image
dimensions therefore differ. Baseline/restored native padding is zero
additional mm, avoiding double-counting the existing 3 mm-per-edge clearance.
These are deliberately labeled native-format capability gaps; image/outline/
background snapshot semantics work, but this fixture does not certify full
native-format style parity. Human native visual/Actual-size print acceptance
remains pending.

### Native execution commands and impact

From the repository root (portable Cargo commands):

```sh
cargo build --example render_stampalbum
cargo run --example render_stampalbum -- tests/fixtures/albumeasy/stamp-settings-save-restore-v1/album.txt tests/fixtures/albumeasy/stamp-settings-save-restore-v1/album-txt-native.pdf
cargo run --example render_stampalbum -- tests/fixtures/albumeasy/stamp-settings-save-restore-v1/album-raw.stampalbum tests/fixtures/albumeasy/stamp-settings-save-restore-v1/album-raw-native.pdf
cargo run --example render_stampalbum -- tests/fixtures/albumeasy/stamp-settings-save-restore-v1/album.stampalbum tests/fixtures/albumeasy/stamp-settings-save-restore-v1/album-native.pdf
cargo test --lib native_snapshot_captures_options_and_style_per_item
cargo test --test native_pdf_stamp_settings
cargo test --test native_pdf_stamp_settings regenerate_denmark_settings_proofs -- --ignored --nocapture
cargo check --all-targets
cargo clippy --lib --example render_stampalbum --test native_pdf_stamp_settings
cargo fmt --check
git diff --check
```

The actual runtime calls used the built
`target/debug/examples/render_stampalbum` with the exact input/output pairs
above; Windows can use `.exe` or the equivalent Cargo commands.
All three native invocations exited 0. Native snapshot unit test passed;
Denmark regeneration passed, both outputs 16 pages/41 image draws/one
retained deferral. Full focused-regression counts are recorded in the plan.

Error inventory during implementation:

- Initial integration test attempted private `NativeAlbum.items` access:
  compiler E0616. Moved internal option assertions into an inline unit test;
  integration tests assert rendered PDFs without widening the public API.
- Comparing `NamedColour` directly failed E0369 (no `PartialEq`); compare
  its RGB values instead. Runtime colour API unchanged.
- Clippy reported one test regex constructed inside an input loop; hoisted
  construction outside the loop.
- The first full focused gate exited 101: the historical unknown-command
  test used `ALBUM_STAMP_SETTINGS_SAVE` as its unsupported sentinel.
  Replaced only that sentinel with still-unmapped `STAMP_INNER_BORDER`,
  retaining the original output-preservation and explicit-loss assertions.
  The final full gate exits 0, with no remaining failures.
- Expected invalid lifecycle/argument/unsupported-control tests fail
  explicitly and preserve pre-existing output; they are not successful
  fallback renders. No dependencies were added or installed.

Filesystem impact: added two `.stampalbum` inputs and three separate native
PDFs, integration tests and a versioned Denmark v4 bundle; updated native
source plus the plan/matrix/report. Accepted TXT/reference PDF, source images
and earlier Denmark versions remain unchanged. No moved, deleted or
relocated inputs. Tests use temporary directories; previews/logs remain in
the session workspace. DB impact: none, no DB opened, schema/row/duplicate
counts not applicable. No ingestion/inbox flow ran. Windows execution remains
unverified, using conservative portable Rust/file paths.

The [Denmark v4 report](../denmark-settings-native-v4/README.md) and
[long-running plan](../../../../docs/reports/NATIVE_PDF_IMPLEMENTATION_PLAN.md)
are the restart points. **NEEDS_SIGNOFF** for native visual/print acceptance;
full saved-state/conformance and native-format style gaps remain open.

### Final validation gate and verdict

Exact focused command:

```sh
cargo test --no-fail-fast --test native_pdf_stamp_settings --test native_pdf_footer_import --test native_pdf_stamp_background --test native_pdf_albumeasy --test native_pdf_album --test native_pdf_footer_number --test native_pdf_page_footer --test native_pdf_pagination_proofs --test native_pdf_stamp_image --test native_pdf_stamp_items --test native_pdf_pagination --test external_album_backend --test native_pdf_stamp_rows --test native_pdf_stamp_text --test native_pdf_row_alignment --test native_pdf_explicit_settings
```

| Tier | Outcome | Exit code | Evidence / gap |
|---|---|---:|---|
| Build / three standalone native renders | PASS | 0 | Two pages / 12 placements / eight visible images each |
| Focused integration/regressions | PASS | 0 | 103 passed, 0 failed, 16 ignored; new settings tests 4 passed |
| Private native captured-state unit | PASS | 0 | 1 passed; actual grayscale, visibility, padding and style assertions |
| Denmark v4 opt-in | PASS | 0 | 1 passed; both formats 16 pages / 41 images / one deferral |
| All-target check / focused Clippy / format / whitespace | PASS | 0 | Existing library warnings only |
| Raster comparison | PASS | 0 | Both TXT/raw pages identical at 1200 pixels |
| Browser / infrastructure | Not applicable | n/a | Standalone file renderer; no server/auth/DB tier selected |
| Native human visual/Actual-size print | UNVERIFIED | n/a | Maintainer review and Letter printing pending |

Overall: **NEEDS_SIGNOFF** for native human/print acceptance and documented
parity gaps; automated bounded mapping checks pass. Windows runtime remains
unverified. The intentionally failing AlbumEasy negative probe and corrected
development failures do not count as outstanding automated test failures.

### Commit/push approval and precommit record

The maintainer requested commit/push of this checkpoint on
`issue/89-stamp-settings-save-restore`. Exact precommit commands:

```sh
cargo fmt --check
git diff --check
cargo test --lib native_snapshot_captures_options_and_style_per_item
cargo test --test native_pdf_stamp_settings --test native_pdf_albumeasy --test native_pdf_album --test native_pdf_stamp_background --test native_pdf_footer_import
```

All exited 0: unit 1 passed; integration 26 passed, 4 ignored, 0 failed.
Tests used temporary outputs, with no moved/deleted/relocated inputs,
reference changes or database impact. No new errors or logic risks were
found during tests. The initial staged whitespace check misclassified
`album-native.pdf` as text and exited 2; added fixture-local `*.pdf binary`,
matching existing proof bundles, without changing the PDF bytes.
Human native visual/print acceptance and documented capability gaps
remain open. The request does not authorize a PR, OKF regeneration or
production integration.
