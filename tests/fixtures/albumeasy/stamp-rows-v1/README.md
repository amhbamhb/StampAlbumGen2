# Frame-only row-fitting proofs

Open [all-cases.pdf](all-cases.pdf). Print **US Letter, Actual size / 100%**,
with Fit/Shrink disabled. Black outlines are mounting frames; blue outlines
are physical stamp bounds including outer perforation tips. Legends are 10 pt.
There are deliberately no stamp images or per-stamp captions in this slice.

| Page / PDF-TXT pair | Input physical sizes, mm | Items per row |
|---|---|---|
| [standard-wrap](standard-wrap.pdf) / [TXT](standard-wrap.txt) | Seven 22x25 stamps | 6, 1 |
| [mixed-heights](mixed-heights.pdf) / [TXT](mixed-heights.txt) | 22x25, 25x40, 40x25, 44x90, 103x51, 125x66 | 4, 1, 1 |
| [large-wrap](large-wrap.pdf) / [TXT](large-wrap.txt) | 125x66, 22x25, 103x51, 25x40 | 2, 2 |
| [zero-clearance](zero-clearance.pdf) / [TXT](zero-clearance.txt) | 22.5x25, 40x25, 125x66 | 2, 1 |

The first three cases add 6 mm to each total physical dimension (3 mm per
edge). Zero-clearance physical/frame outlines coincide. Frame gaps are 3 mm
horizontally and 4 mm vertically. Rows are left-aligned, frames top-aligned,
and the next row starts below the tallest preceding frame. Input order is
preserved; nothing is rotated or shrunk to make it fit.

The available content area comes from resolved page settings, including
title/header/footer reservations. A frame wider than the content area or
below its bottom returns an error; this is a bounded single-page fitter,
not automatic pagination. Normal row wrapping needs no warning.

[expected.json](expected.json) records calculated positions;
[verification.txt](verification.txt) records independent MuPDF trace checks.
The TXT files compare frame dimensions and row membership only. AlbumEasy
positions/text/physical outlines are **not** claimed equivalent, and the TXT
inputs have not been reference-rendered.

## Reproduce (Windows or Linux)

From the repository root:

```text
cargo test --test native_pdf_stamp_rows
cargo test --test native_pdf_stamp_rows regenerate_letter_row_fixtures -- --ignored --exact
cargo test --test 'native_pdf_*'
cargo clippy --test native_pdf_stamp_rows
cargo check --lib --target x86_64-pc-windows-gnu
rustfmt --edition 2021 --check src/native_pdf/stamp_rows.rs src/native_pdf/renderer.rs tests/native_pdf_stamp_rows.rs
git diff --check
mutool merge -o tests/fixtures/albumeasy/stamp-rows-v1/all-cases.pdf tests/fixtures/albumeasy/stamp-rows-v1/standard-wrap.pdf tests/fixtures/albumeasy/stamp-rows-v1/mixed-heights.pdf tests/fixtures/albumeasy/stamp-rows-v1/large-wrap.pdf tests/fixtures/albumeasy/stamp-rows-v1/zero-clearance.pdf
```

For an independent trace, run `mutool draw -F trace -o TRACE_PATH` followed by
the combined PDF path; choose an output path outside the repository.
Recheck the trace against the JSON after changing inputs or code before
claiming verification. The ordinary test suite does not rewrite these files.

## Execution record

Initial red command: `cargo test --test native_pdf_stamp_rows`.
Expected failure: `error[E0432]: unresolved import
stamp_album_gen2::native_pdf::stamp_rows`.

After implementation:

```text
cargo test --test native_pdf_stamp_rows --test native_pdf_stamp_box --test native_pdf_stamp_image
cargo test --test native_pdf_stamp_rows regenerate_letter_row_fixtures -- --ignored --exact
```

14 focused tests pass; explicit fixture generation passes. Four merged pages
pass independent Letter media-size, physical/frame-coordinate, gap and
top-alignment checks. Bounds tolerance is 0.004 mm; gaps allow 0.006 mm
because a gap can combine three operands rounded by the PDF writer to
0.01 pt (maximum combined error approximately 0.0053 mm).
An initial 0.004 mm gap threshold rejected a 4.004027 mm rendered gap;
the corrected bound accounts for actual serialization precision rather than
changing layout dimensions. Pages 1 and 2 were rasterized at 100 DPI and
visually inspected. MuPDF emits `warning: ICC support is not available`.

Full native regression: 104 passed, 18 ignored (including the opt-in fixture
generator), 0 failed. Clippy and formatting pass with existing application
unused/dead-code and PDF-import parser warnings. Windows GNU library
cross-check passes, with existing `winres compile failed: No such file or
directory (os error 2)` warning; Windows runtime/resource embedding is not
validated on this Linux host. No OKF regeneration was run.

Filesystem impact: created this fixture directory with four PDF/TXT pairs,
expected geometry, verification and combined PDF. Test scratch PDFs live in
temporary directories and are cleaned automatically. No inputs were moved
or deleted. Database impact: none; no schema changes, row writes or ingest.

Logic risks / limits: text footprints, illustrations, production backend
integration, row justification/alignment options, automatic page breaks,
AlbumEasy-rendered parity and physical printing remain unverified/deferred.
The single-page overflow error intentionally prevents silent omission or
shrinking; callers must eventually handle pagination explicitly.
