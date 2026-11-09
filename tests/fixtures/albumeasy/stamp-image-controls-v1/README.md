# AlbumEasy 4.5.0 image-controls fixture

This fixture follows the required two-pass workflow. Its first-pass
[`album.txt`](album.txt) contains AlbumEasy 4.5.0 commands only and was rendered
with AlbumEasy 4.5.0. The maintainer accepted that complete one-page US Letter
page for proceeding to the native pass; actual-size print review remains
pending.

The separate [`album.stampalbum`](album.stampalbum) was rendered directly by
the native renderer into [`album-native.pdf`](album-native.pdf). It was not
used to generate or validate the AlbumEasy first-pass PDF.

## Overall native-import goal

This paired example contributes to the larger goal: in the future, either a
`.stampalbum` document or an imported AlbumEasy `.txt` and its images should
produce a PDF through StampAlbumGen2's native renderer, without invoking
AlbumEasy. The `.txt` path must map the original AlbumEasy commands and their
state into the same canonical album model used by `.stampalbum`; native-only
features remain in `.stampalbum`, not in AlbumEasy reference files. All
commands in the agreed AlbumEasy version scope—including raw commands and
documented variants—must have explicit native mappings and conformance tests
before full TXT compatibility is claimed. Unmapped commands must be reported,
not silently discarded. AlbumEasy remains a reference tool for mapping and
checking behavior, not the future PDF runtime.

| Pass | Input | PDF | Saved preview | Extracted text |
|---|---|---|---|---|
| AlbumEasy reference | AlbumEasy [`album.txt`](album.txt) | [`album.pdf`](album.pdf) | [`album-preview.png`](album-preview.png) | [`album-pdf-text.txt`](album-pdf-text.txt) |
| Native rendition | Native [`album.stampalbum`](album.stampalbum) | [`album-native.pdf`](album-native.pdf) | [`album-native-preview.png`](album-native-preview.png) | [`album-native-pdf-text.txt`](album-native-pdf-text.txt) |
| Native interpretation of original TXT | Unchanged [`album.txt`](album.txt) | [`album-txt-native.pdf`](album-txt-native.pdf) | [`album-txt-native-preview.png`](album-txt-native-preview.png) | [`album-txt-native-pdf-text.txt`](album-txt-native-pdf-text.txt) |
| Native interpretation of raw commands | Raw [`album-raw.stampalbum`](album-raw.stampalbum) | [`album-raw-native.pdf`](album-raw-native.pdf) | [`album-raw-native-preview.png`](album-raw-native-preview.png) | [`album-raw-native-pdf-text.txt`](album-raw-native-pdf-text.txt) |

## Page and command coverage

The Letter page uses 15 mm margins, a 0.2 mm page border, 4 mm horizontal and
14 mm vertical row spacing, and an album title. Stamp boxes use the same
mounting convention as the accepted row-alignment example: box dimensions are
6 mm larger than the recorded stamp dimensions, giving 3 mm clearance at each
edge. Each item has a legible three-line caption with its record/SG identity,
Colnect size, and the control being demonstrated.

The seven stamp frames cover:

- AlbumEasy's omitted image-setting default.
- Millimetre padding.
- Percentage padding with omitted optional arguments.
- Small-image stretch off/on using the same reduced real-stamp image.
- Native-only, small-font labels above and below the small no-stretch image.
- Aspect-ratio preservation and grayscale.
- Hide/show and grayscale reset behavior.

The native `.stampalbum` adds `SG 1137b` in the open space above and below the
small/no-stretch image. The labels are inside the frame and do not cover the
stamp image or replace the descriptive caption below the frame. This behavior
is native-only; the AlbumEasy reference input and PDF remain unchanged. The
native small/no-stretch image uses a 0.3 mm-per-pixel scale to leave readable
space around the two labels while retaining the no-stretch behavior.

The native command is attached immediately after its `STAMP_IMAGE`:

```text
NATIVE_STAMP_IMAGE_LABEL (HN 3.5 black "SG 1137b" "SG 1137b")
```

Its arguments are font, point size, colour, above-image text and below-image
text. Leave either quoted string empty to omit that side. Labels are centered
in the available whitespace with at least 1 mm clearance from the image.
Rendering fails if they cannot fit while preserving that clearance.

The reduced `us-152-small.png` (50 x 55 pixels, 72 dpi) is derived at the
original image's aspect ratio from the genuine record-152 stamp image solely
to exercise the documented small-image behavior. Its SHA-256 is
`cd8074ddd1052921b998725372c6cab025ae7a49bfd0c6a190120f4f86324515`.
All stamp identities and physical sizes remain sourced from the real measured
records below. The separate page-background-image visual case is deferred; a
background image does not belong in this album-page rendition.

## Real-stamp provenance

The full-size images are reused from
[`stamp-pagination-v1`](../stamp-pagination-v1/README.md). That fixture's
source manifest records Colnect IDs, dimensions, image pixel sizes, hashes,
and source paths. The source database was read-only; no database or source
image was modified.

| DB ID | SG number | Colnect ID | Physical size | AlbumEasy box |
|---:|---|---:|---:|---:|
| 150 | 1125a | 700029 | 52 x 29 mm | 58 x 35 mm |
| 151 | 1137a | 700273 | 26 x 29 mm | 32 x 35 mm |
| 152 | 1137b | 700274 | 26 x 29 mm | 32 x 35 mm |
| 156 | 1148a | 700272 | 52 x 29 mm | 58 x 35 mm |

## Reproduction and review status

Each retained PDF has a 1200-pixel-wide PNG page preview and a layout-preserving
text extract beside it. These files are derived from the checked-in PDFs; they
make screen review and text comparison possible without relying on temporary
session files. Recreate them from the repository root with:

```sh
pdftoppm -f 1 -l 1 -scale-to 1200 -singlefile -png tests/fixtures/albumeasy/stamp-image-controls-v1/album.pdf tests/fixtures/albumeasy/stamp-image-controls-v1/album-preview
pdftotext -layout tests/fixtures/albumeasy/stamp-image-controls-v1/album.pdf tests/fixtures/albumeasy/stamp-image-controls-v1/album-pdf-text.txt
pdftoppm -f 1 -l 1 -scale-to 1200 -singlefile -png tests/fixtures/albumeasy/stamp-image-controls-v1/album-native.pdf tests/fixtures/albumeasy/stamp-image-controls-v1/album-native-preview
pdftotext -layout tests/fixtures/albumeasy/stamp-image-controls-v1/album-native.pdf tests/fixtures/albumeasy/stamp-image-controls-v1/album-native-pdf-text.txt
pdftoppm -f 1 -l 1 -scale-to 1200 -singlefile -png tests/fixtures/albumeasy/stamp-image-controls-v1/album-txt-native.pdf tests/fixtures/albumeasy/stamp-image-controls-v1/album-txt-native-preview
pdftotext -layout tests/fixtures/albumeasy/stamp-image-controls-v1/album-txt-native.pdf tests/fixtures/albumeasy/stamp-image-controls-v1/album-txt-native-pdf-text.txt
pdftoppm -f 1 -l 1 -scale-to 1200 -singlefile -png tests/fixtures/albumeasy/stamp-image-controls-v1/album-raw-native.pdf tests/fixtures/albumeasy/stamp-image-controls-v1/album-raw-native-preview
pdftotext -layout tests/fixtures/albumeasy/stamp-image-controls-v1/album-raw-native.pdf tests/fixtures/albumeasy/stamp-image-controls-v1/album-raw-native-pdf-text.txt
```

Run AlbumEasy 4.5.0 from this directory to reproduce the first pass:

```sh
timeout 60s xvfb-run -a -s '-screen 0 1280x1024x24' \
  env QT_QPA_PLATFORM=xcb /home/ray/work/AlbumEasy/AlbumEasy.sh -c album.txt
```

AlbumEasy successfully generated a one-page Letter PDF. `pdfinfo` and MuPDF
confirmed the page size and six embedded visible images. The screen preview was
inspected: the page border, title, spacing, captions, grayscale, hidden frame,
and the difference between small-image stretch off/on are visible. The
maintainer accepted this rendition as the first pass; it has not been printed
at actual size.

Render the native pass directly from the repository root:

```sh
cargo run --example render_stampalbum -- \
  tests/fixtures/albumeasy/stamp-image-controls-v1/album.stampalbum \
  tests/fixtures/albumeasy/stamp-image-controls-v1/album-native.pdf
```

The native renderer produced one US Letter page with seven frames and six
visible stamp images. Compare the saved
[`album-preview.png`](album-preview.png) and
[`album-native-preview.png`](album-native-preview.png). Both pages include the
title, border, measured stamp boxes, real images, and item captions. The native
`.stampalbum` page places its centered subtitle in the top margin and uses item
captions to identify each control rather than repeating AlbumEasy's in-page
subtitle and three section headings. Its first row also starts higher, with
slight row-placement and image-fit differences. This is a visual comparison,
not a claim of exact rendering parity.

Print the AlbumEasy reference and the native `.stampalbum` PDF on US Letter at
**Actual size / 100%**, with Fit, Shrink, and other automatic scaling disabled.
The saved PNGs are screen previews, not substitutes for the printable PDFs.
Physical print review of both renditions and native-pass acceptance remain
pending.

## Input-path test results

The requested input cases were checked against the current implementation:

| Input case | Current path and result |
|---|---|
| Native `.stampalbum` using the program's version-1 native commands | **Pass.** `render_stampalbum` generated a one-page Letter PDF from this fixture's `album.stampalbum`. The focused `native_pdf_album` tests also pass. |
| AlbumEasy `.txt` through the standalone native example | **Pass for this fixture, unchanged.** The example dispatches to the bounded AlbumEasy interpreter and writes `album-txt-native.pdf`: one Letter page, six visible images, no AlbumEasy process. |
| `.stampalbum` containing only raw AlbumEasy commands | **Pass for this fixture.** `album-raw.stampalbum` is byte-identical to `album.txt`, without `STAMPALBUM 1` or native commands. The example produces `album-raw-native.pdf`: one Letter page, six visible images, no AlbumEasy process. |
| Existing production external-album workflow | **Unchanged, not native-only.** Its 23 passing automated tests use a mocked renderer. The real workflow still launches the configured AlbumEasy executable. |

The earlier extension/header rejection results are superseded by the
standalone dispatch and compatibility interpreter, not by a relaxation of the
native-language parser. Unsupported AlbumEasy commands still fail explicitly;
these passes are not certification of arbitrary TXT or production integration.

Reproduce the added second-pass outputs from the repository root:

```sh
cargo run --example render_stampalbum -- \
  tests/fixtures/albumeasy/stamp-image-controls-v1/album.txt \
  tests/fixtures/albumeasy/stamp-image-controls-v1/album-txt-native.pdf
cargo run --example render_stampalbum -- \
  tests/fixtures/albumeasy/stamp-image-controls-v1/album-raw.stampalbum \
  tests/fixtures/albumeasy/stamp-image-controls-v1/album-raw-native.pdf
```

The TXT/raw outputs have identical extracted text and identical 1200-pixel
page rasters. The raster was inspected for borders, spacing, captions and
image controls. This checks extension-independent interpretation, not exact
AlbumEasy parity. No reference TXT/PDF, stamp images or database was changed.
Only the native outputs were regenerated, and the raw copy/output were added.
See the [Denmark execution report](../denmark-txt-native-v1/README.md#execution-report)
for the shared test commands and failure inventory.

## Status

- AlbumEasy-only `.txt`: rendered successfully with AlbumEasy 4.5.0 and
  accepted by the maintainer to proceed to the native pass.
- AlbumEasy PDF: one US Letter page; screen-level layout/control inspection
  complete; actual-size print review pending.
- Native `.stampalbum` and PDF: rendered directly; one US Letter page;
  screen-level comparison complete; acceptance and actual-size print review
  pending.
- TXT and raw-command `.stampalbum` native PDFs: standalone paths pass;
  matching native raster/text verified; human acceptance and print review
  pending. These were created only after the reference first pass was accepted.
