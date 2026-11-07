# Native PDF fixture and human-review workflow

**Prepared:** 2026-11-07
**Branch:** `issue/84-native-albumeasy-renderer`
**Tracking:** [Issue #84](https://github.com/amhbamhb/StampAlbumGen2/issues/84)

## Purpose and current checkpoint

Use this workflow for native PDF command examples and visual proofs. Each
example is intended to show both editable input and rendered output, while
keeping the collector's visual judgment as the final acceptance gate.

The current example is
[`tests/fixtures/albumeasy/native-real-stamps-v1`](../../tests/fixtures/albumeasy/native-real-stamps-v1/README.md).
It pairs AlbumEasy `album.txt` and its reference `album.pdf` with native
`album.stampalbum` and its `album-native.pdf`. Both files use the same four
real US stamps and recorded Colnect physical sizes. The PDFs have been
generated and structurally checked as one-page US Letter outputs with four
embedded images. The PDFs were also rasterized for a screen preview. They have
not yet been printed at actual size or visually accepted by the maintainer;
do not describe them as approved or as rendering-parity evidence.

## Required fixture contents

For each paired example, keep the source and output together under
`tests/fixtures/albumeasy/<fixture-id>/`:

- AlbumEasy `.txt` input using only commands verified for the target version.
- AlbumEasy reference `.pdf` produced from that input.
- Human-readable `.stampalbum` native input. Do not use TOML as the native
  album language; TOML in older fixtures is historical.
- Native `.pdf` produced directly from the `.stampalbum` document, not through
  an intermediate AlbumEasy file.
- README with command scope, source provenance, reproduction steps, automated
  checks, print settings, and explicit human-review status.
- Provenance manifest or a link to one, recording source record IDs, Colnect
  dimensions, image dimensions, source locations, and checksums.

Reuse checked-in real image assets when practical instead of copying them into
every fixture. Select only stamps with positive recorded Colnect dimensions
and an available associated image. Read source databases read-only; never
change database rows or schemas to prepare a visual fixture.

## Process followed for the real-stamp comparison

1. **Select measured source stamps.** Reuse records 150, 151, 152, and 156 from
   the United States album database. Their recorded physical sizes are 52x29,
   26x29, 26x29, and 52x29 mm. Their image provenance and pixel dimensions are
   in [`stamp-pagination-v1/source-records.json`](../../tests/fixtures/albumeasy/stamp-pagination-v1/source-records.json).
   Query the database in read-only mode and verify its checksum is unchanged.
2. **Write parallel inputs.** In `album.txt`, express the known-size images
   with documented AlbumEasy 4.5.0 `STAMP_ADD_IMG` syntax. In
   `album.stampalbum`, use the `STAMPALBUM 1` header, explicit page settings,
   explicit native policies, and `STAMP_IMAGE` records with dimensions and
   image scale. Keep each format's semantics clear; sharing a source image
   does not imply identical layout behavior.
3. **Render independently.** Run AlbumEasy 4.5.0 on the `.txt`. Run
   StampAlbumGen2's standalone native example on the `.stampalbum`. Do not
   route native PDF generation through AlbumEasy or exported TXT.
4. **Run structural checks.** Confirm both PDFs open, have the expected page
   count and US Letter media box (612x792 points), and contain the expected
   number of images. Native tests should also assert physical stamp bounds
   against the recorded Colnect dimensions.
5. **Preview, then print.** A raster preview is only a screen-level sanity
   check. Print each PDF on US Letter at **Actual size / 100%**, with Fit,
   Shrink, and other automatic scaling disabled. Inspect stamp appearance and
   physical size. Record the maintainer's acceptance or requested corrections
   in the fixture README; automated success cannot substitute for it.
6. **Preserve evidence.** Once accepted, treat the source input and reviewed
   PDF as a versioned fixture. For intentional changes, create a new fixture
   version rather than silently replacing accepted evidence.

## Reproduction commands used

From the fixture directory, the AlbumEasy 4.5.0 reference run on this Linux
host was:

```sh
timeout 60s xvfb-run -a -s '-screen 0 1280x1024x24' \
  env QT_QPA_PLATFORM=xcb /home/ray/work/AlbumEasy/AlbumEasy.sh -c album.txt
```

From the repository root, render the native output:

```sh
cargo run --example render_stampalbum -- \
  tests/fixtures/albumeasy/native-real-stamps-v1/album.stampalbum \
  tests/fixtures/albumeasy/native-real-stamps-v1/album-native.pdf
```

Run the native API and geometry checks:

```sh
cargo test --test native_pdf_album
```

Useful independent PDF checks:

```sh
pdfinfo tests/fixtures/albumeasy/native-real-stamps-v1/album.pdf
pdfinfo tests/fixtures/albumeasy/native-real-stamps-v1/album-native.pdf
mutool info tests/fixtures/albumeasy/native-real-stamps-v1/album.pdf
mutool info tests/fixtures/albumeasy/native-real-stamps-v1/album-native.pdf
```

Replace the AlbumEasy executable path with the installed renderer path on
another machine. Preserve the target version and record the actual command and
renderer identity with the fixture.

## Boundaries and future work

- These files demonstrate one image-only native slice; they do not establish
  full AlbumEasy import, export, command coverage, or visual parity.
- Production application generation remains separate and gated on completing
  and reviewing the command mapping.
- Website galleries pairing command text with output are a later work item;
  keep this fixture process independent of website implementation.
- Do not run `okf-rs generate` until the native PDF work is finished, as
  directed by the maintainer.
