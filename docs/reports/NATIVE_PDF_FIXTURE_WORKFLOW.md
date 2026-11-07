# Phased AlbumEasy and native-PDF fixture workflow

**Prepared:** 2026-11-07
**Branch:** `issue/84-native-albumeasy-renderer`
**Tracking:** [Issue #84](https://github.com/amhbamhb/StampAlbumGen2/issues/84)

## Purpose and required sequence

### Long-term input and rendering contract

The end goal is to generate PDFs natively from either `.stampalbum` files or
imported AlbumEasy `.txt` files and their images, without using AlbumEasy in
the rendering workflow. Native documents are parsed directly; AlbumEasy input
is interpreted and mapped into the same canonical album model, then rendered
by the same native PDF backend. Its raw AlbumEasy commands must retain their
mapped behavior; they are not added as application-specific commands to the
AlbumEasy `.txt`.

The compatibility target is the complete, agreed AlbumEasy command/version
scope, including overloads, stateful behavior, and legacy spellings that the
importer supports. A command without a verified mapping must be called out as
unsupported or unresolved, not ignored or approximated silently. AlbumEasy
4.5.0 remains the reference renderer for the first-pass fixtures and behavior
checks, but not a future runtime dependency for native PDF output. This is a
future goal, not a claim that the current importer or renderer already handles
all AlbumEasy commands.

Every command-family example proceeds in two deliberate passes. The collector
must first accept a complete AlbumEasy-rendered page before the corresponding
native-PDF implementation and comparison begins. A native rendering or a
native test is not a substitute for the AlbumEasy first pass.

The AlbumEasy `.txt` is an AlbumEasy document only: it may contain only
commands verified for the pinned AlbumEasy version. Never put program-native
commands (`NATIVE_*`, `.stampalbum` syntax, or other application extensions)
in this file. Render the first-pass `.txt` with AlbumEasy itself and do not run
native rendering/tests as evidence for that pass.

Each first-pass input should be a complete, realistic album page rather than a
bare command strip: include the intended page size, margins, border, row
spacing, real measured stamp items/images, correct physical and frame sizing,
and readable headings/captions or other relevant page text. Keep the visual
context that a collector will actually judge while adding the next command
case. A focused edge case may be isolated only when required to expose the
behavior, and should still use a deliberate album-page layout.

An existing pre-phase example is
[`tests/fixtures/albumeasy/native-real-stamps-v1`](../../tests/fixtures/albumeasy/native-real-stamps-v1/README.md).
It pairs AlbumEasy `album.txt` and its reference `album.pdf` with native
`album.stampalbum` and its `album-native.pdf`. Both files use the same four
real US stamps and recorded Colnect physical sizes. The PDFs have been
generated and structurally checked as one-page US Letter outputs with four
embedded images. The PDFs were also rasterized for a screen preview. They have
not yet been printed at actual size or visually accepted by the maintainer;
do not describe them as approved or as rendering-parity evidence. This older
pair predates the required AlbumEasy-first acceptance gate and is not the
sequence to follow for new fixtures.

## Required fixture contents by pass

Keep all source inputs and outputs together under
`tests/fixtures/albumeasy/<fixture-id>/`.

**Pass 1 — AlbumEasy reference (required first):**

- AlbumEasy `.txt`, containing only target-version AlbumEasy commands.
- AlbumEasy `.pdf`, rendered from that `.txt` by the actual AlbumEasy program.
- Complete page context, with genuine measured stamp data/images, borders,
  margins, row spacing, frame/physical sizing, and relevant text.
- README with AlbumEasy version/command, source provenance, structural checks,
  Actual-size print instructions, and explicit human-review status.

Do not create a native PDF as part of this first-pass evidence.

**Pass 2 — native rendition and comparison (only after Pass 1 is accepted):**

- Human-readable `.stampalbum` native input; do not use TOML as the native
  album language. TOML in older fixtures is historical.
- Native `.pdf` rendered directly from `.stampalbum`, never through AlbumEasy
  TXT or the AlbumEasy renderer.
- Matching page content and real stamp data so the collector can compare the
  same page at Actual size. Record differences as well as matches.

**Pass 2 compatibility inputs:** Also test the unchanged AlbumEasy `.txt`
directly through the native backend, plus a byte-identical `.stampalbum`
containing only raw AlbumEasy commands (no native header or commands).
Keep their directly rendered PDFs alongside the native-language input/output.
These tests are second-pass evidence only; they never replace the accepted
AlbumEasy first pass or claim that all command families are implemented.

**Ongoing compatibility gate:** Repeat all three native input paths for every
remaining command-family update: original AlbumEasy TXT, native-language
`.stampalbum`, and raw-AlbumEasy `.stampalbum`. Keep the accepted reference
TXT/PDF unchanged and rerun existing compatibility regressions, including the
image-controls and Denmark fixtures. Verify page/image counts and layout/
state behavior, inspect comparable PDF pages, and retain human print-review
status. A successful render alone does not prove correct import behavior.
Newly mapped commands must gain regression coverage and be restored in the
adapted test copy only after validation; remove their deferral notes only when
their behavior is verified. Any remaining unsupported commands must stay
explicitly listed and commented solely in the adapted copy. Never weaken
tests or silently add omissions to make the next update pass.

When testing an existing album with unmapped commands, leave its source
untouched. Record every unsupported command and original line in the
long-running plan, then comment those lines explicitly in a separately named
supported test copy. Never silently strip commands at render time. Preserve
relative image references by copying only referenced assets with checksums.
Document omitted behavior, source/PDF provenance and human review status.
The [Denmark example](../../tests/fixtures/albumeasy/denmark-txt-native-v1/README.md)
demonstrates this adapted-input process; its historical PDF is not a newly
verified first-pass render from the current TXT.

The current image-controls native pass exercises version-1 commands for a
page header/title, optional total frame additions in `NATIVE_STAMP_FRAME`,
explicit row boundaries, and multiline image captions. `NATIVE_ROW_BREAK ()`
ends the current image row; `STAMP_IMAGE` may include caption font, point size,
and quoted caption text. Use `\n` inside quoted text for caption line breaks.
These native-only commands belong in `.stampalbum`, never in the AlbumEasy
`.txt`.

The README and source manifest should retain both passes' reproduction steps,
source record IDs, Colnect dimensions, image dimensions, source locations, and
checksums. Reuse checked-in real image assets where practical. Select only
stamps with positive recorded Colnect dimensions and an available associated
image. Read source databases read-only; never change database rows or schemas
to prepare a visual fixture.

## Process followed for the real-stamp comparison

1. **Select measured source stamps.** Reuse records 150, 151, 152, and 156 from
   the United States album database. Their recorded physical sizes are 52x29,
   26x29, 26x29, and 52x29 mm. Their image provenance and pixel dimensions are
   in [`stamp-pagination-v1/source-records.json`](../../tests/fixtures/albumeasy/stamp-pagination-v1/source-records.json).
   Query the database in read-only mode and verify its checksum is unchanged.
2. **Author the complete AlbumEasy page only.** Use documented commands and
   familiar album-page composition. Include appropriate borders, margins,
   spacing, stamp sizing, and explanatory/collector-facing text. Do not add
   native-only commands to `album.txt`.
3. **Complete Pass 1.** Run AlbumEasy 4.5.0 on `album.txt`. Inspect the actual
   generated page, check the PDF structure, print at Actual size, and record
   maintainer feedback. Stop here until the AlbumEasy page is accepted.
4. **Begin Pass 2 after acceptance.** Create the corresponding `.stampalbum`
   and native PDF. Render it directly with the native renderer; do not use
   AlbumEasy as an intermediate.
5. **Compare only after both passes exist.** Compare equivalent page content,
   real stamp dimensions/images, text, spacing, and borders. Record visual
   differences and the human decision; automated tests alone are insufficient.
6. **Preserve evidence.** Once accepted, treat inputs and reviewed PDFs as
   versioned fixtures. For intentional changes, create a new fixture version
   rather than silently replacing accepted evidence.

## Reproduction commands used

From the fixture directory, the AlbumEasy 4.5.0 reference run on this Linux
host was:

```sh
timeout 60s xvfb-run -a -s '-screen 0 1280x1024x24' \
  env QT_QPA_PLATFORM=xcb /home/ray/work/AlbumEasy/AlbumEasy.sh -c album.txt
```

The first-pass AlbumEasy command is shown above. Only after the AlbumEasy page
has been accepted, render the native output from the repository root:

```sh
cargo run --example render_stampalbum -- \
  tests/fixtures/albumeasy/native-real-stamps-v1/album.stampalbum \
  tests/fixtures/albumeasy/native-real-stamps-v1/album-native.pdf
```

Native API and geometry checks are for Pass 2 only:

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

- The AlbumEasy first pass establishes only what its pinned renderer produced;
  it does not establish native parity or application integration.
- Native command mapping and rendering are a separate second pass, gated on
  acceptance of the complete AlbumEasy page.
- Production application generation remains separate and gated on completing
  and reviewing the command mapping.
- Website galleries pairing command text with output are a later work item;
  keep this fixture process independent of website implementation.
- Do not run `okf-rs generate` until the native PDF work is finished, as
  directed by the maintainer.
