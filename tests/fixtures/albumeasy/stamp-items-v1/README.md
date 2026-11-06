# Complete native image/text row proofs

Open [all-cases.pdf](all-cases.pdf). Print **US Letter / Actual size 100%**,
with Fit/Shrink disabled. Legends, headings and captions are 10 pt.

| Page | Native PDF | Adapted AlbumEasy TXT | Items per row |
|---|---|---|---|
| 1 | [Wrapped images/text](wrapped-images.pdf) | [TXT](wrapped-images.txt) | 4, 4 |
| 2 | [Expanded images/text](expanded-images.pdf) | [TXT](expanded-images.txt) | 4, 4 |

Green: complete text/frame footprint. Black: mounting frame. Blue: physical
stamp bounds including perforation tips. Solid coloured rectangles are
synthetic PNG assets, not real stamps or estimates of their dimensions.
The four input cases repeat on a second row to verify image-resource reuse
does not overwrite other placements.

| Item | Physical mm | Frame mm | Image px | Image footprint mm |
|---|---|---|---|---|
| 1 | 22x25 | 28x31 | 220x250 | 11x12.5, no enlargement |
| 2 | 22x25 | 28x31 | 220x250 | 22x25, fitted down |
| 3 | 40x25 | 46x31 | 220x250 | 22x25, portrait preserved |
| 4 | 25x40 | 31x46 | 300x150 | 25x12.5, landscape preserved |

Illustrations are centered and proportionally fitted **inside physical stamp
bounds**, independently of text width/height. Expanded captions use a 120%
mounting-frame-width cap, with padding 2 mm per horizontal side and 1 mm per
vertical edge. Frame/text gaps and row/item gaps are 3 mm. A wider caption
must not enlarge the image, physical stamp or mounting frame.

This combines existing image fitting and measured text-footprint row layout;
it does not introduce another size estimator, new config keys, production
backend wiring or pagination. Any missing/corrupt image or invalid scale,
even on the last item, fails before creating/truncating a PDF.

TXT inputs preserve all images and measured text via deliberately separate
blocks and unchanged mounting rectangles. Native footprint policies are not
emitted as AlbumEasy commands. Inputs explicitly identify adaptation and
are **not reference-rendered or native-position/row/page parity evidence**.

[expected.json](expected.json) records every image/frame/physical/footprint
bound, original pixels, image scale and all text positions.
[verification.txt](verification.txt) records independent MuPDF checks.

## Reproduce (Windows or Linux)

```text
cargo test --test native_pdf_stamp_items
cargo test --test native_pdf_stamp_items regenerate_letter_item_fixtures -- --ignored --exact
mutool merge -o tests/fixtures/albumeasy/stamp-items-v1/all-cases.pdf tests/fixtures/albumeasy/stamp-items-v1/wrapped-images.pdf tests/fixtures/albumeasy/stamp-items-v1/expanded-images.pdf
```

For an independent trace, run `mutool draw -F trace -o TRACE_PATH` followed
by the combined PDF path. Keep trace/raster scratch files outside the repository.
Regular tests do not overwrite these fixtures.

## Development execution record

```text
cargo test --test native_pdf_stamp_items --test native_pdf_stamp_image --test native_pdf_stamp_text
cargo test --test native_pdf_stamp_items regenerate_letter_item_fixtures -- --ignored --exact
cargo test --test 'native_pdf_*'
cargo clippy --test native_pdf_stamp_items
cargo check --lib --target x86_64-pc-windows-gnu
rustfmt --edition 2021 --check src/native_pdf/renderer.rs src/native_pdf/stamp_image.rs tests/native_pdf_stamp_items.rs
git diff --check
```

Three item tests pass; the opt-in fixture generator passes explicitly.
Full native regression: 117 passed, 20 ignored, 0 failed.
Clippy and formatting/whitespace checks pass with pre-existing application
unused/dead-code and PDF-import parser warnings. Windows GNU library check
passes with the existing `winres compile failed: No such file or directory
(os error 2)` warning; Windows runtime/resource embedding is not verified.
Tests inspect actual PDF image-resource names and placement matrices across
both text policies, and test missing/corrupt images, NaN/zero scales, text
overflow and multiple-page rejection without damaging existing output.

Independent MuPDF trace verification passes for both pages: all 16 images
retain expected size/aspect/center, every footprint/frame/physical rectangle
matches the ledger, every text line and baseline is present, all fonts are
10 pt, and actual row/item gaps are correct. Bounds tolerance is 0.004 mm;
gap tolerance is 0.006 mm for multiple rounded PDF operands.
Both pages were rasterized at 120 DPI and visually inspected. MuPDF reports
`warning: ICC support is not available`.

Initial red errors included missing combined item types/renderer and two test
float-literal syntax errors; corrected test literals and implemented the APIs.
Filesystem impact: created this directory with four synthetic PNGs, two PDF/TXT
pairs, combined PDF and expected/verification ledgers. Temporary automated
test files are cleaned on completion. No album inputs moved/deleted, no DB
schema/writes/ingest, and no OKF regeneration. User manual autosaves untouched.

Logic risks / limits: synthetic illustration geometry is not physical-size
estimation accuracy. Production integration, inside-frame text, pagination,
AlbumEasy reference rendering and physical print calibration remain deferred.
