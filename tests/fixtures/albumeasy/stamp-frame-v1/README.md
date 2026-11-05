# Native stamp-frame visual proofs

Start with [all-cases.pdf](all-cases.pdf), a ten-page **Letter (215.9 x 279.4 mm)** proof.
Labels are 10 pt (previously 4 pt); case headings are 14 pt (previously 7 pt).
Print **Actual size / 100%**, with Fit/Shrink/borderless scaling disabled.
Page 10 has a **100 x 100 mm** calibration square: measure both axes from
stroke centreline to centreline before assessing mounting dimensions.

- Black outline: mounting-frame geometry.
- Blue outline: physical item dimensions, including outer perforation tips.
- Patterned picture: original synthetic test asset, not a real stamp scan.
- At zero clearance, the blue and black outlines coincide.
- Caption text is diagnostic and intentionally outside the frame; it is not
  an implementation of configurable inside/outside stamp text.

## Files and cases

- [Measurement ledger](measurements.txt): physical size, configured total
  additions, adjustment, calculated frame size, per-edge clearance, image size.
- [Independent verification](verification.txt): actual MuPDF-traced dimensions,
  all ten pages, tolerance and limitations.
- [AlbumEasy comparison input](frame-reference-albumeasy.txt): one page per
  case, with separate frame-size and physical-size rectangles. This is a
  **geometry comparison**, not an equivalent export of native image positioning
  or text. It has not been reference-rendered.
- Individual numbered PDFs correspond to the same pages in the combined PDF.
- Each numbered PDF has a matching AlbumEasy TXT geometry comparison input.
- [Expected geometry](expected.json) records calculated sizes and tolerance;
  it is not an AlbumEasy-rendered reference.
- [synthetic-stamp.png](synthetic-stamp.png): shared 220 x 250 px image.

| Page | Physical mm | Purpose |
|---|---|---|
| 1 | 22 x 25 | Small natural-size image, 28 x 31 mm frame |
| 2 | 22 x 25 | Larger source fitted down; same physical/frame dimensions |
| 3 | 22.5 x 25 | Fractional size, unequal extras, shared 0.5 mm adjustment |
| 4 | 25 x 40 | Portrait |
| 5 | 40 x 25 | Landscape physical item; picture retains its portrait aspect |
| 6 | 44 x 90 | Booklet-size item, no 25 mm physical height cap |
| 7 | 125 x 66 | Large se-tenant-size item |
| 8 | 103 x 51 | Wide item |
| 9 | 22 x 25 | Zero clearance |
| 10 | 100 x 100 | Two-axis physical print calibration |

The shared portrait image deliberately does not match every physical item's
shape. For example, page 7 retains a 125 x 66 mm physical area but fits the
picture to approximately 58.08 x 66 mm. It must not stretch the image or shrink
the mounting dimensions. These are geometry fixtures, not a claim that the
picture depicts the item being mounted.

## Reproduce

From the repository root, on Windows or Linux:

```text
cargo run --example stamp_frame_proofs -- tests/fixtures/albumeasy/stamp-frame-v1
```

This writes ten individual PDF/TXT pairs, expected geometry, a synthetic PNG,
the measurement ledger, and the combined AlbumEasy TXT. To regenerate the
combined PDF, with MuPDF installed:

```text
mutool merge -o tests/fixtures/albumeasy/stamp-frame-v1/all-cases.pdf tests/fixtures/albumeasy/stamp-frame-v1/01-standard-small-image.pdf tests/fixtures/albumeasy/stamp-frame-v1/02-standard-fit-image.pdf tests/fixtures/albumeasy/stamp-frame-v1/03-fractional.pdf tests/fixtures/albumeasy/stamp-frame-v1/04-portrait.pdf tests/fixtures/albumeasy/stamp-frame-v1/05-landscape.pdf tests/fixtures/albumeasy/stamp-frame-v1/06-booklet.pdf tests/fixtures/albumeasy/stamp-frame-v1/07-se-tenant.pdf tests/fixtures/albumeasy/stamp-frame-v1/08-wide.pdf tests/fixtures/albumeasy/stamp-frame-v1/09-zero-clearance.pdf tests/fixtures/albumeasy/stamp-frame-v1/10-calibration.pdf
```

Inspect a new trace with `mutool draw -F trace -o frame.trace.xml` followed by
the combined PDF path. The saved verification describes this generation only;
if inputs/code change, recompute its measurements before claiming it passes.

## Execution record

Commit-gate commands, all successful:

```text
cargo test --test native_pdf_stamp_box --test native_pdf_stamp_image
cargo test --lib native_pdf::
cargo test --test 'native_pdf_*'
cargo clippy --example stamp_frame_proofs --test native_pdf_stamp_box --test native_pdf_stamp_image
rustfmt --edition 2021 --check src/native_pdf/stamp_box.rs src/native_pdf/stamp_image.rs src/native_pdf/renderer.rs tests/native_pdf_stamp_box.rs tests/native_pdf_stamp_image.rs examples/stamp_frame_proofs.rs
cargo run --example stamp_frame_proofs -- tests/fixtures/albumeasy/stamp-frame-v1
cargo clippy --example stamp_frame_proofs
git diff --check
```

Focused integration tests: 8 passed. Native library tests: 3 passed.
Full native integration regression: 98 passed, 17 existing ignored, 0 failed.
Clippy reports existing unused/dead-code warnings and PDF-import parser
`redundant_field_names`/`needless_borrow` warnings; none blocks validation.

Filesystem impact: moved these proofs from `docs/testing/native-stamp-frame-v1`
into this fixture directory, regenerated its ten individual PDFs and combined
PDF, and added matching numbered TXT inputs and `expected.json`. No album
inputs were deleted or relocated. Automated tests use temporary directories
and clean them on completion; build artifacts remain under Cargo's target
directory. Database impact: none; no schema checks, inserts, updates, duplicate
processing, or country ingest were performed.

Generation, merge, MuPDF trace, and rasterization completed successfully.
MuPDF emitted `warning: ICC support is not available`; geometry checks passed.
All ten Letter pages match expected physical/frame extents within 0.004 mm and
image dimensions within 0.002 mm. Label font sizes and baseline positions inside
page margins were verified from the trace. Revised pages 1 and 10 were rasterized
at 100 DPI and opened for visual inspection. No AlbumEasy render was tested.
After relocation/regeneration, the combined PDF was rebuilt and all ten pages
were independently checked against `expected.json`: Letter page size,
physical/frame/image dimensions and 10/14 pt fonts pass.
Logic risks: reference-rendered AlbumEasy parity, text-aware row footprints,
production sizing integration and revised physical printer calibration remain
unverified; these proofs do not claim those features.

### User print feedback and Letter revision

The original A4 page 1 was printed on Letter paper. The reported measurements:
image 11 x 13 mm (calculated 11 x 12.5 mm), physical outline 22 x 25 mm,
frame 28 x 31 mm. The bottom page border was missing, consistent with A4's
297 mm height exceeding Letter's 279.4 mm height at actual size.
This confirms the reported physical/frame dimensions for that print, not
full printer calibration; the image-height discrepancy remains recorded.

The examples now use Letter paper with the page border about 12 mm inside
each edge. Stamp, frame, clearance, and image dimensions are unchanged;
only horizontal centering follows the new page width. Larger labels use
shorter explicit lines rather than enlarging mounting boxes to fit text.
Reprint page 1 and page 10 at actual size to check readability, the complete
page border, and both axes of the 100 mm calibration square.

Only this fixture directory was populated; temporary test/raster/trace files
were confined to test/session storage. No album input was moved, deleted, or
changed; no database was opened or updated by proof generation.
