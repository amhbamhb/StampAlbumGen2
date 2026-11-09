# AlbumEasy 4.5.0 stamp reference fixture run

**Run date:** 2026-11-06
**Renderer:** AlbumEasy v4.5.0
**Scope:** External reference rendering only; no StampAlbumGen2 production
integration or behavior change.

## Result

Both fixture inputs rendered successfully with the installed AlbumEasy 4.5.0
renderer. The fixtures use AlbumEasy commands only. No application-specific
commands or syntax were added.

| Fixture | Result | PDF pages and media | Output |
|---|---|---|---|
| [`stamp-shapes-v1`](../../tests/fixtures/albumeasy/stamp-shapes-v1/README.md) | Pass; renderer exited 0 | 2 pages, US Letter (612 x 792 pt) | [`album.pdf`](../../tests/fixtures/albumeasy/stamp-shapes-v1/album.pdf) |
| [`stamp-image-overloads-v1`](../../tests/fixtures/albumeasy/stamp-image-overloads-v1/README.md) | Pass; renderer exited 0 | 1 page, US Letter (612 x 792 pt) | [`album.pdf`](../../tests/fixtures/albumeasy/stamp-image-overloads-v1/album.pdf) |

The renderer logs for each run ended with `Successfully created ... album.pdf`.
The generated PDFs contain AlbumEasy 4.5.0 creator metadata.

## Exact render commands

The renderer wrapper and help archive used for the run were:

- `/home/ray/work/AlbumEasy/AlbumEasy.sh` — SHA-256:
  `76122bc30566a6daf86312e2b5110f02b1531bdef0a47a96846f0a9684ce3336`
- `/home/ray/work/AlbumEasy/AlbumEasy45.0.qch` — SHA-256:
  `8b81707d861aacceb798b9f38efbbfdc3ea86fa49a57b7e343c3a25d5964d5be`

Commands executed:

```sh
(cd tests/fixtures/albumeasy/stamp-shapes-v1 && xvfb-run -a -s '-screen 0 1280x1024x24' env QT_QPA_PLATFORM=xcb timeout 60s /home/ray/work/AlbumEasy/AlbumEasy.sh -c album.txt)
(cd tests/fixtures/albumeasy/stamp-image-overloads-v1 && xvfb-run -a -s '-screen 0 1280x1024x24' env QT_QPA_PLATFORM=xcb timeout 60s /home/ray/work/AlbumEasy/AlbumEasy.sh -c album.txt)
```

Both commands exited successfully. Renderer output identified `AlbumEasy
v4.5.0`, loaded `album.txt`, and reported successful PDF creation.

## Verification performed

- `pdfinfo` confirmed the page count and US Letter size for each output.
- `pdftotext -layout` checks found all expected fixture labels, captions, and
  inside-text markers:
  - Shapes: rectangular modern/legacy text, diamond, oval, polygon orientation,
    triangle, inverted triangle, right triangle, blank-box fixture caption.
  - Images: PNG/JPEG captions, `NO_FRAME` caption, and both combined-overload
    inside-text markers.
- `mutool info` confirmed four embedded 64 x 40 RGB images in the image PDF:
  two PNG image occurrences and two JPEG image occurrences.
- MuPDF trace extraction and 100-DPI raster inspection were performed on both
  PDFs. Visual inspection showed the expected stamp outlines, image patterns,
  captions, and frame/no-frame cases. This was a visual check, not a measured
  geometry/parity test.
- A command-name audit extracted the uppercase command names from both fixture
  inputs and found every one in the inspected AlbumEasy 4.5.0 help text. This
  verifies reference documentation presence, not full command semantics.

The fixtures are representative v1 reference cases, not exhaustive grammar
coverage. In particular, the shapes input does not cover every right-triangle
selector combination or multiline text; the image input does not cover the
complete independent cross-product of optional horizontal offsets and
`NO_FRAME`, or missing/invalid image assets. These remain explicit follow-up
coverage rather than inferred behavior.

## Execution report

| Area | Outcome |
|---|---|
| Start/end | Both AlbumEasy reference renders started and completed successfully; exit code 0 for each. |
| Filesystem | Created the two fixture directories, README/input files, two reference PDFs, and the synthetic PNG/JPEG assets listed below. No source files or production configuration were changed by this rendering step. |
| Database | No database opened or modified. |
| Errors | No AlbumEasy errors in either final run. MuPDF trace/raster tools emitted `warning: ICC support is not available`; output extraction and inspection still completed. |
| Environment probes | The Qt `offscreen` probe could not use the installed xcb-only plugin; the direct display/help probe timed out. The actual fixture runs succeeded under `xvfb-run` with `QT_QPA_PLATFORM=xcb`. These probes were not failures of the final fixture runs. |
| Logic risks | The render proves only that these particular inputs are accepted by AlbumEasy 4.5.0. It does not establish StampAlbumGen2 importer, editor, exporter, or native-PDF support, nor full behavioral parity. PDF creation timestamps make output PDF hashes run-specific. |

The only source assets are synthetic asymmetric test patterns; they are not
stamp images. Recorded SHA-256 values:

| File | SHA-256 |
|---|---|
| `stamp-shapes-v1/album.txt` | `e39756706018cc8524e690e8e454dbe8f0cfce657d37f6c8daa80caa66b85ed1` |
| `stamp-shapes-v1/album.pdf` | `05d61dc1a06a3c55a9261c53bd387fba4264986b89625dd604732c8cae30d6b8` |
| `stamp-image-overloads-v1/album.txt` | `50632f0c0ca652245ed5a9fec16d2a202ae87514e0f489e7a541daa80c4f03d0` |
| `stamp-image-overloads-v1/album.pdf` | `28b2f0a92a734ea8d84fa400437fd29db4f59335000cae126d21771304c748e3` |
| `stamp-image-overloads-v1/assets/synthetic-asymmetric.png` | `45161284be0a73a8e34f9aed158c4530e2faf1f06a16ae103301c83b16d01bd3` |
| `stamp-image-overloads-v1/assets/synthetic-asymmetric.jpg` | `2c36998f63ffe6584d9871d5fd42967597d1ee3da3f30515825e9b0bec497846` |

The PDF hashes identify these particular outputs only; regenerated PDFs may
differ because of metadata timestamps. The frozen TXT and raster-asset hashes
identify the stable test inputs.

At the time of the v1 fixture run, no Rust test suite was run because that
task only created and externally rendered reference fixtures; no Rust source
or application behavior changed.
Production native-PDF integration remains deferred until the complete
AlbumEasy command language and compatibility gaps have been mapped and
reviewed.

## Supplemental v2: combined `STAMP_ADD_IMG` overloads

The v2 input exercises all four documented combinations of the optional
horizontal-offset pair and `NO_FRAME`, with multiline inside text and signed
vertical offsets. It reuses the synthetic asymmetric PNG from v1. The frozen
v1 input and assets were not changed.

The external AlbumEasy 4.5.0 reference run completed successfully:

```sh
(cd tests/fixtures/albumeasy/stamp-image-overloads-v2 && xvfb-run -a -s '-screen 0 1280x1024x24' env QT_QPA_PLATFORM=xcb timeout 60s /home/ray/work/AlbumEasy/AlbumEasy.sh -c album.txt)
```

`pdfinfo` confirmed one US Letter page (612 x 792 pt), and
`pdftotext -layout` found each line of the four multiline inside-text labels.
The [reference PDF](../../tests/fixtures/albumeasy/stamp-image-overloads-v2/album.pdf)
and [rendered preview](../../tests/fixtures/albumeasy/stamp-image-overloads-v2/album-reference-preview.png)
are checked in.

The standalone native compatibility implementation is covered by:

```sh
cargo test --lib native_pdf::albumeasy::tests
cargo test --test native_pdf_albumeasy combined_stamp_add_img_overloads_render_images_and_multiline_inside_text
cargo test --test native_pdf_albumeasy generate_native_image_overload_v2_proof -- --ignored
```

All commands passed (3 unit tests, 1 integration test, and 1 proof-generation
test). The [native PDF](../../tests/fixtures/albumeasy/stamp-image-overloads-v2/album-native.pdf)
and [native preview](../../tests/fixtures/albumeasy/stamp-image-overloads-v2/album-native-preview.png)
are available for visual review. This evidence does not establish pixel parity
or production integration; the imported-album parser/UI and ordinary PDF
routing remain unchanged.
