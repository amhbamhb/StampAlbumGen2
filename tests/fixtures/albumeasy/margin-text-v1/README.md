# Vertical margin text: accepted reference and bounded native mapping

Tracking [#90](https://github.com/amhbamhb/StampAlbumGen2/issues/90),
branch `issue/90-margin-text`, based on `773747f9` (#89).
The maintainer's "continue" after the first-pass presentation was interpreted
and announced as reference acceptance. The standalone native slice is now
implemented. Native visual/Actual-size print acceptance remains pending.
No commit/push, PR, production routing or OKF regeneration is included.

## Complete-page references

| Input | AlbumEasy 4.5.0 PDF | Cases |
|---|---|---|
| [album.txt](album.txt) | [album.pdf](album.pdf) | Darkblue left/right, default centred vertically and reading UP |
| [top-down.txt](top-down.txt) | [top-down.pdf](top-down.pdf) | Darkred multiline L/TOP/DOWN, R/BOTTOM/UP, +2 mm padding, COLOUR alias |
| [alternating.txt](alternating.txt) | [alternating.pdf](alternating.pdf) | Darkgreen LR/TOP/UP and RL/BOTTOM/DOWN, -1 mm padding, physical odd/even pages |

Every input contains **only AlbumEasy commands**, with no native extensions.
Every PDF has two exact US Letter pages (215.9 x 279.4 mm; 612 x 792 points)
and 12 visible real-stamp images, six per page. Page composition derives from
the accepted [footer reference](../footer-numbering-import-v1/README.md):
15 mm margins, 0.2 mm page border, 4 mm horizontal/14 mm vertical row spacing,
title, readable captions and darkblue `Page 1 of 2` / `Page 2 of 2` footers.
The new margin cases do not replace earlier references.

Observed screen behavior:

- Default text is vertically centred outside each page border, reads UP on
  both sides and persists on the second page.
- TOP/DOWN multiline text sits at the top of the left margin; BOTTOM/UP
  multiline text sits at the bottom of the right margin. +2 mm padding
  moves the text farther from the border. No clipping was visible.
- LR changes from left to right on physical page 2; RL changes from right
  to left. The -1 mm padding brings both closer to the border. TOP/BOTTOM
  alignment remains the same when the side changes.
- AlbumEasy repeats the page title here. Do not infer universal title
  semantics from a previously observed fixture.

## Source provenance / syntax boundary

Four unchanged assets and recorded Colnect measurements come from
[source-records.json](../stamp-pagination-v1/source-records.json):

| Record | SG | Colnect ID | Physical size | Frame |
|---:|---|---:|---|---|
| 150 | 1125a | 700029 | 52 x 29 mm | 58 x 35 mm |
| 151 | 1137a | 700273 | 26 x 29 mm | 32 x 35 mm |
| 152 | 1137b | 700274 | 26 x 29 mm | 32 x 35 mm |
| 156 | 1148a | 700272 | 52 x 29 mm | 58 x 35 mm |

Each frame adds 6 mm total to each physical dimension (3 mm per edge);
reference image padding is 3 mm, with preserved source aspect ratio.
No SQLite query, image resize/copy or source modification was needed.

Pinned AlbumEasy 4.5.0 help (`AlbumEasy45.0.qch`, SHA-256
`8b81707d861aacceb798b9f38efbbfdc3ea86fa49a57b7e343c3a25d5964d5be`)
documents:

```text
ALBUM_PAGES_MARGIN_TXT (Font Fontsize "Text" Position [Align] [Direction])
ALBUM_PAGES_MARGIN_TXT_PAD (padding)
COLOR_ALBUM_MARGIN_TXT (colour)
COLOUR_ALBUM_MARGIN_TXT (colour)
```

Position: L/R/LR/RL. Alignment: TOP/CENTRE/BOTTOM (default CENTRE).
Direction: UP/DOWN (default UP). Text can contain `\n`.
Two definitions may supply the two margins; settings/padding persist for the
whole document. Signed padding moves text toward/away from the border.
Do not assume CENTER or other aliases not verified in this help.

The bounded mapping measures rotated text with existing AFM font metrics,
preserves outward multiline order, and chooses LR/RL sides from physical
one-based page parity rather than printed footer numbers. Final colour/padding
before `PAGE_START` applies to all definitions, including earlier ones.
Colour-only input with no margin text is valid. Settings changed after page
creation, invalid arguments, physical-page overflow, vertical-margin overflow
and overlapping nonblank definitions fail explicitly, preserving old output.
This is not full language or whole-page pixel-parity certification.

## Native inputs and persistent outputs

| Case | TXT-native PDF | Raw AlbumEasy input / native PDF | Native-language input / PDF |
|---|---|---|---|
| Default | [album-txt-native.pdf](album-txt-native.pdf) | [album-raw.stampalbum](album-raw.stampalbum) / [album-raw-native.pdf](album-raw-native.pdf) | [album.stampalbum](album.stampalbum) / [album-native.pdf](album-native.pdf) |
| Multiline | [top-down-txt-native.pdf](top-down-txt-native.pdf) | [top-down-raw.stampalbum](top-down-raw.stampalbum) / [top-down-raw-native.pdf](top-down-raw-native.pdf) | [top-down.stampalbum](top-down.stampalbum) / [top-down-native.pdf](top-down-native.pdf) |
| Alternating | [alternating-txt-native.pdf](alternating-txt-native.pdf) | [alternating-raw.stampalbum](alternating-raw.stampalbum) / [alternating-raw-native.pdf](alternating-raw-native.pdf) | [alternating.stampalbum](alternating.stampalbum) / [alternating-native.pdf](alternating-native.pdf) |

Each raw input is byte-identical to its AlbumEasy TXT. Native-language files
start with `STAMPALBUM 1` and use exactly eight explicit arguments:

```text
NATIVE_PAGE_MARGIN_TEXT (HN 9 "Text" L CENTRE UP 2 darkblue)
```

Order: font, point size, quoted text, position, alignment, direction,
border-to-text baseline-model gap in mm, named colour. This gap is **not**
additional AlbumEasy padding: compatibility uses the observed 2 mm default
gap plus `ALBUM_PAGES_MARGIN_TXT_PAD`; native fixtures explicitly use 2, 4 and
1 mm for default, +2 and -1 mm padding. No new TOML defaults/keys are added.

Shared layout lives in
[album_pages_margin_txt.rs](../../../../src/native_pdf/commands/page/album_pages_margin_txt.rs).
The shared PDF writer uses saved graphics state with exact UP/DOWN quarter-turn
transforms, draws text at the transformed origin and restores state.
All six transform coefficients/baselines on both pages of all nine routes
agree with actual AlbumEasy reference PDF text matrices within **0.01 pt**.
All outputs retain two Letter pages, 12 images, 12 stamp frames, two page
borders and every footer. Physical image dimensions agree between native-
language and TXT routes within 0.02 pt. These checks do not prove identical
frame positioning, title/caption layout or whole-page appearance.

All six TXT/raw page PNG pairs at 1200 pixels are byte-identical; the tests
also compare decoded TXT/raw PDF content streams. Native-language page
composition is intentionally not claimed identical to the reference.

### Third-definition probe: deliberate strict deviation

[duplicate-position.txt](duplicate-position.txt) and
[duplicate-position.pdf](duplicate-position.pdf) record AlbumEasy's behavior
when a third definition repeats L. AlbumEasy creates the PDF and exits 0:

```text
Warning in line 12: ALBUM_PAGES_MARGIN_TXT command ignored as two margin text items have already been assigned.
```

The native renderer accepts at most two definitions and **rejects** a third,
rather than silently dropping content. It also rejects actual overlap
between the two definitions, instead of shifting/clipping text. This strict
boundary is not exact conformance to AlbumEasy's third-definition behavior.

## Exact first-pass execution report

From this fixture directory on Linux:

```sh
timeout 60s xvfb-run -a -s '-screen 0 1280x1024x24' env QT_QPA_PLATFORM=xcb /home/ray/work/AlbumEasy/AlbumEasy.sh -c album.txt
timeout 60s xvfb-run -a -s '-screen 0 1280x1024x24' env QT_QPA_PLATFORM=xcb /home/ray/work/AlbumEasy/AlbumEasy.sh -c top-down.txt
timeout 60s xvfb-run -a -s '-screen 0 1280x1024x24' env QT_QPA_PLATFORM=xcb /home/ray/work/AlbumEasy/AlbumEasy.sh -c alternating.txt
pdfinfo album.pdf
pdfinfo top-down.pdf
pdfinfo alternating.pdf
pdfimages -list album.pdf
pdfimages -list top-down.pdf
pdfimages -list alternating.pdf
pdftoppm -f 1 -l 1 -scale-to 1200 -singlefile -png album.pdf <session-default-preview>
pdftoppm -f 1 -l 1 -scale-to 1200 -singlefile -png top-down.pdf <session-explicit-preview>
pdftoppm -f 1 -l 2 -scale-to 1200 -png alternating.pdf <session-alternating-preview>
```

All render/inspection commands exited 0. Each renderer logged successful
PDF creation; metadata verifies two Letter pages and image inspection shows
12 RGB placements. The default/explicit first pages and both alternating
pages were visually inspected as complete pages, retaining borders, sizes,
row spacing, images, captions and footer. No render errors occurred.

Windows equivalent: open/export the three TXT files in AlbumEasy 4.5.0,
preserving relative assets. Windows execution remains unverified.
Native visual acceptance/Actual-size Letter printing remains pending; print at 100%
without Fit/Shrink and inspect vertical margin text against the border.

Filesystem impact: created this directory, three TXT inputs, three reference
PDFs, README and fixture-local PDF binary attributes (existing proof-bundle
pattern). Updated the plan/matrix. No earlier input/reference/asset was
modified, deleted, moved or relocated. Preview PNGs stay in the session
workspace. DB impact: none; no target DB opened, schema checks/inserted/
updated/skipped rows/duplicate detection applies. No ingestion/inbox flow
ran. Logic risks are the rotated bounds, side alternation, global state and
duplicate semantics listed above.

## Native validation execution report

From repository root (Cargo commands also apply on Windows):

```sh
cargo test --test external_album_backend --test native_pdf_album --test native_pdf_albumeasy --test native_pdf_explicit_settings --test native_pdf_footer_import --test native_pdf_footer_number --test native_pdf_margin_text --test native_pdf_page_footer --test native_pdf_pagination --test native_pdf_pagination_proofs --test native_pdf_row_alignment --test native_pdf_stamp_background --test native_pdf_stamp_image --test native_pdf_stamp_items --test native_pdf_stamp_rows --test native_pdf_stamp_settings --test native_pdf_stamp_text
cargo test --test native_pdf_margin_text
cargo test --test native_pdf_margin_text regenerate_denmark_margin_proofs -- --ignored --nocapture
cargo check --all-targets
cargo clippy --test native_pdf_margin_text
cargo fmt -- --check
git diff --check
```

Focused regression gate: exit 0, **108 passed, 17 ignored, zero failed**.
The final margin-only rerun after strengthening frame/image assertions:
exit 0, 5 passed/1 ignored. Denmark's explicit generator: exit 0, 1 passed.
All-target checking, focused Clippy, format and whitespace checks passed.
Pre-existing library warnings remain; no unrelated cleanup was performed.

For each of `album`, `top-down`, `alternating`, the exact raster commands
from repository root were:

```sh
pdftoppm -f 1 -l 2 -scale-to 1200 -png tests/fixtures/albumeasy/margin-text-v1/<case>-txt-native.pdf <session>/margin-<case>-txt
pdftoppm -f 1 -l 2 -scale-to 1200 -png tests/fixtures/albumeasy/margin-text-v1/<case>-raw-native.pdf <session>/margin-<case>-raw
cmp <session>/margin-<case>-txt-1.png <session>/margin-<case>-raw-1.png
cmp <session>/margin-<case>-txt-2.png <session>/margin-<case>-raw-2.png
```

All exited 0. `<session>` is the active session's `files` directory; no PNG
is required in the tracked proof bundle. Windows equivalent: render with the
same Poppler arguments and compare PNG bytes with `fc.exe /b`.
Default native page 1, multiline TXT-native page 1, alternating native page 2
and Denmark page 6 were inspected as complete pages.

Error inventory: two early test assertions failed because graphics matrices
serialize with two decimals, not the expected three. Corrected the test
format assumption; no runtime workaround was needed. Final runs have no
failures. Negative tests exercise unsupported position/alignment/direction,
wrong arity, invalid values, conflicting/third definitions and mid-document
changes; expected errors retain source-line diagnostics and old output.

Filesystem additions after the first pass: the duplicate-position probe pair,
six `.stampalbum` inputs and nine native PDFs in this directory; shared source/
test changes and the separate Denmark v5 bundle. No earlier input, reference,
image or fixture was changed, moved, deleted or relocated. Tests use temporary
outputs cleaned by their normal temporary-directory lifecycle. Logs/previews
remain session-only. DB/ingest impact: none; no database opened, schema changes,
rows inserted/updated/skipped, duplicate checks or inbox targets apply.

Logic risks still open: whole-page title/row/caption parity, broader page/
font/overload/state mappings, language-wide certification, Windows execution
and human 100% Letter print acceptance. The strict third-definition/overlap
behavior and pre-page global-setting restriction are explicit deviations/
boundaries, not evidence of full import compatibility.

```text
environment: Linux; Docker/Node/Playwright not applicable to standalone PDF validation
startup: PASS — standalone rendering through both parsers/shared PDF writer
integration: PASS — exit_code: 0; 108 passed, 0 failed, 17 ignored; scoped Rust regressions
e2e: PASS — exit_code: 0; nine two-page native proofs plus two Denmark inputs; no AlbumEasy launch by native routes
overall: NEEDS_SIGNOFF — native visual/Actual-size print and Windows runtime unverified
```

## Denmark qualification / continuation boundary

[Denmark v5](../denmark-margin-native-v5/README.md) restores only original
line 45 `COLOR_ALBUM_MARGIN_TXT` from [v4](../denmark-settings-native-v4/README.md).
Both routes render 16 pages/41 images/every footer with **zero commented
deferrals**. Denmark has no margin-text definition; this proves colour-setting
acceptance, not visible margin rendering. The three references above prove
that visible behavior. Zero deferrals covers this managed-asset source only,
not the whole AlbumEasy language or historical PDF visual parity.

Continue from the [long-running plan](../../../../docs/reports/NATIVE_PDF_IMPLEMENTATION_PLAN.md)
after review. Preserve all accepted reference versions and the TXT/raw/native
regression gate for every future family. Production integration, PR and OKF
stay deferred until native PDF mapping is complete.

## Maintainer-authorized check-in

After clarifying that Denmark checks only the margin-colour command, the
maintainer requested commit and push on 2026-11-07. The exact focused
regression command above was rerun before committing: exit 0, 108 passed,
17 ignored, zero failed. No persistent fixture files were changed by that
run; temporary test outputs were cleaned normally. No database/ingest impact
or new error/logic risk was observed. Existing compatibility boundaries,
Windows/print gaps and production/PR/OKF deferrals remain unchanged.
