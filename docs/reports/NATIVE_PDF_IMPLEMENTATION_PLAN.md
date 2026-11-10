# Native PDF generation with AlbumEasy TXT export

**Created:** 2026-11-01  
**Last updated:** 2026-11-09
**Revision:** 65
**Status:** Bounded standalone renderer and command-family slices are implemented. Against the 46 roadmap tasks, 19 are partially evidenced, 27 remain unsatisfied, and none yet meets its full task acceptance criteria. Full AlbumEasy compatibility, canonical application integration, export and production routing remain incomplete.
**Scope:** Build StampAlbumGen2's own canonical album document and native PDF renderer. Generate the PDF directly from that document, then optionally export an AlbumEasy-compatible TXT projection. Continue to accept/import existing user-authored AlbumEasy TXT as a separate compatibility path. AlbumEasy 4.5.0 is a reference renderer and export target, not the canonical model or a permanent rendering dependency.

**Command-level checkoff:** [AlbumEasy 4.2.1 / 4.5.0, native-only and extras commands](../guides/NATIVE_PDF_COMMAND_CHECKLIST.md).
Use that compact list for current command coverage; retain this plan for
task acceptance, detailed gaps and history. Its checkmarks do not change the
46-task acceptance counts above.

### Canonical document and output contract

The program's own album document is authoritative; neither AlbumEasy TXT nor
the current database representation is assumed to define every capability of
the new format. Ordinary album creation builds this document and resolves its
layout once. The native PDF renderer and AlbumEasy TXT exporter consume that
same resolved document independently; native PDF generation must not depend on
running AlbumEasy or on first producing TXT.

Existing AlbumEasy `.txt` files remain importable through the existing
external-album workflow and are progressively mapped into the native document.
Native-only and expanded commands belong to the program's document model and
its own serializable text format, not to AlbumEasy-compatible TXT. Export must
classify every feature as exact, approximated, omitted, or blocked and report
losses explicitly; it must never silently claim that a divergent export is
equivalent.

### One native PDF path for both input formats

The long-term goal is that users can provide either a program-owned
`.stampalbum` document or an AlbumEasy `.txt` album, together with its referenced
images, and generate the PDF entirely within StampAlbumGen2. Both inputs must
converge on the same canonical in-memory album model and the same native PDF
renderer: `.stampalbum` is parsed directly, while `.txt` is parsed and mapped
from AlbumEasy commands into that model. Import must preserve command ordering,
state changes, image references, and the observable layout semantics of every
mapped command; it must not invoke AlbumEasy to interpret or render the file.

The target for AlbumEasy compatibility is complete coverage of the documented
command language and supported legacy spellings in the agreed version scope,
including stamp shapes, image overloads, captions, frame settings, and
state/save-restore behavior. “Raw AlbumEasy codes work” means the original
AlbumEasy commands in an imported `.txt` are interpreted and rendered natively,
not copied into `.stampalbum` or passed through an AlbumEasy executable.
Unmapped or unsupported input must be reported explicitly rather than silently
ignored or rendered with success-shaped defaults. AlbumEasy remains useful as
a pinned reference renderer for mapping and visual conformance, but must not be
a runtime dependency of PDF generation.

This is the compatibility destination, not a claim about current support. The
command matrix, reference fixtures, command mapping, and conformance tests must
be completed incrementally before imported `.txt` files can be certified for
native rendering. Production integration remains deferred until that mapping
and review are complete. Native-only features continue to use `.stampalbum`;
AlbumEasy reference `.txt` files remain free of application-specific commands.

**Native file extension:** `.stampalbum` is approved by the maintainer for the
program-owned text document. Version 1 has a UTF-8 command-line format and
`STAMPALBUM 1` header. Keep `.txt` for imported AlbumEasy input and
AlbumEasy-compatible output.

**Execution order clarified:** The canonical native document and direct native
PDF path come first. AlbumEasy-compatible TXT export is a separate projection
from that document; importing user-authored AlbumEasy TXT is a distinct
parse-and-map path into the same canonical model. Neither PDF path may depend
on first producing another text format or invoking AlbumEasy. The historical
phase/task IDs below are retained for traceability and must not reverse this
implementation order.

The standalone code slice is implemented in
[`src/native_pdf/album.rs`](../../src/native_pdf/album.rs), with the
`render_stampalbum` example and a fixture under
[`tests/fixtures/native_pdf/stampalbum-v1`](../../tests/fixtures/native_pdf/stampalbum-v1/README.md).
It parses explicit page/border/row/text-policy commands and `STAMP_IMAGE`,
resolves relative raster paths from the `.stampalbum` file, and writes a real
native PDF directly without AlbumEasy or diagnostic proof overlays. Current
version-1 support includes stamp-image policies, multiline captions, page
titles and headers, explicit row/page breaks (including intentional empty
content pages), and total-addition frame sizing.
It does not itself parse AlbumEasy commands. The separate bounded interpreter
described below shares its native PDF writer; compatible TXT export, other
stamp shapes, and application generation are not connected.

The [real-stamp comparison fixture](../../tests/fixtures/albumeasy/native-real-stamps-v1/README.md)
pairs an AlbumEasy 4.5.0 `.txt`/PDF with a native `.stampalbum`/PDF using four
real US stamps with verified Colnect dimensions. Its native test checks the
recorded physical bounds and the one-page Letter output. Printing and
maintainer visual acceptance remain pending; this example does not claim
renderer parity.

The AlbumEasy-first image-control fixture is documented in
[`stamp-image-controls-v1`](../../tests/fixtures/albumeasy/stamp-image-controls-v1/README.md).
Its AlbumEasy-only `.txt` now renders a complete bordered Letter page with real
measured stamps, spacing, frame sizing and readable captions. AlbumEasy 4.5.0
was the only renderer used for that first pass, which the maintainer accepted
before the native pass began. The separate `.stampalbum` now renders directly
to `album-native.pdf`; a screen-level comparison records the layout and image
fit differences. Native-pass and actual-size print acceptance are still
pending. Page-background-image behavior is deferred to a separate realistic
fixture.

### Standalone TXT and raw-AlbumEasy checkpoint (#84)

The [standalone example](../../examples/render_stampalbum.rs) now dispatches
by the first nonblank, noncomment source line:

- `STAMPALBUM ...`: the existing native-language loader, retaining version
  checks and the `.stampalbum` extension requirement.
- Otherwise: the [bounded AlbumEasy interpreter](../../src/native_pdf/albumeasy.rs),
  accepting `.txt` or a `.stampalbum` containing only AlbumEasy commands.

Neither path launches AlbumEasy. Both use the shared native PDF writer, but
the compatibility path currently builds positioned imported-page scenes,
not the final unified canonical document. This is experimental standalone
support, not a change to the production external-album importer, which still
uses AlbumEasy.

The accepted image-controls reference TXT renders **without edits**. Its TXT
and byte-identical raw-AlbumEasy `.stampalbum` each produce one Letter page
with six visible images; their extracted text and 1200-pixel page raster are
identical. The native-language `.stampalbum` also renders one Letter page,
with its previously recorded layout differences.

The [Denmark 1989 fixture](../../tests/fixtures/albumeasy/denmark-txt-native-v1/README.md)
preserves the supplied album's explicit 16 page boundaries and 41 image
placements. All 41 referenced images are copied with checksums, so both its
prepared TXT and identical raw `.stampalbum` render from the fixture directory.
The original TXT, images, database, and historical PDF are not modified.
The original TXT **does not yet pass unchanged**: the eight occurrences below
are commented only in the clearly named supported test copy. The historical
PDF is a comparison aid, not newly generated AlbumEasy 4.5.0 conformance proof.

#### Required Denmark mappings still deferred

**Historical v1 inventory:** All eight occurrences below have now been
restored in the [v5 derivative](../../tests/fixtures/albumeasy/denmark-margin-native-v5/README.md).
Current progression is v1=8, v2=5, v3=3, v4=1, **v5=0** commented deferrals.
The v1 descriptions/risks below record that earlier checkpoint, not current
v5 capability. Zero deferrals means this managed-asset command sequence
passes; full language/default/overload/state mapping remains incomplete.

| Command | Original lines | Occurrences | Required follow-up |
|---|---|---:|---|
| `COLOR_ALBUM_FOOTER` | 42 | 1 | Map footer colour into native footer rendering. |
| `COLOR_ALBUM_MARGIN_TXT` | 45 | 1 | Map margin-text colour together with margin text commands. |
| `COLOR_STAMP_BACKGROUND` | 49, 72, 79 | 3 | Add stateful stamp background fills; preserve image/frame layering. |
| `ALBUM_PAGES_FOOTER_NUMBER` | 52 | 1 | Map positioned page numbering, prefix/suffix and `$PAGES$`. |
| `ALBUM_STAMP_SETTINGS_SAVE` | 68 | 1 | Capture the complete documented stamp-settings state. |
| `ALBUM_STAMP_SETTINGS_RESTORE` | 83 | 1 | Restore that state; verify reference save/restore semantics first. |

Normal rendering rejects unsupported commands; it never auto-comments them.
`commented_test_source` explicitly prepares a test copy and returns a
line/name/source loss inventory. It checks supported **names**, not every
overload: unsupported variants or malformed arguments still fail rendering
and must not be described as compatible.

#### Current bounded behavior and remaining risks

- Supported families: explicit page size/margins/spacing/border, title/header,
  selected colours, top-aligned ES/FS rows, centered page text/vertical space,
  basic image stamps/`NO_FRAME`, three-argument `STAMP_HEADING_VA`, frame size
  adjustment, solid/blank frames, and image padding/stretch/aspect/grayscale/
  visibility controls. See the interpreter for exact accepted arguments.
- Page settings must precede `PAGE_START`; mid-document changes fail.
  Required geometry is explicit in the input. Reference-language image
  defaults are distinct from native-language TOML settings.
- Source page breaks are preserved. Oversize rows error rather than
  silently reflowing. Mixed-height row reservation uses each item's own
  frame-plus-caption footprint, not the tallest frame plus another caption.
- #103 adds one pre-page start group with single-pass, quote-preserving
  `$$1`–`$$9` arguments over supported body commands. Correct TXT title
  reservation includes ordinary vertical spacing after the title as well as
  its explicit extra gap. Native explicit title-gap semantics are unchanged.
- Indexed/low-bit PNGs are decoded with the existing image crate; RGBA
  transparency is retained. Image-fit comparisons allow only `1e-9` mm
  floating-point roundoff. Missing images remain explicit errors and
  preserve existing output files.
- Text uses AFM metrics. Caption widths/wrapping, baseline placement, default
  row-alignment semantics, image DPI metadata, advanced image overloads,
  shapes, state/save-restore and exact visual parity remain open.
  The compatibility image scale is currently one point per pixel (72 dpi).
- Denmark's source width is **216.0 mm**, not 215.9 mm. Its 216.0 x 279.4 mm
  geometry is preserved; the image-controls proofs are exact US Letter.
  Explicit TXT frame dimensions are not independently verified physical
  Colnect stamp measurements.
- The supplied Denmark PDF differs from the current TXT/native output:
  black backgrounds and the numbered footer are absent in the adapted
  native proof, while the native proof includes the TXT's page border/header.
  Dense captions also need human review. Do not certify visual parity.
- Each Denmark native PDF is approximately 44.7 MB; compression/asset
  optimization is a later task, not grounds for changing source images.
- Switzerland fixture selection remains separate. The earlier two candidate
  rows had identity/measurement concerns; do not use them as authoritative
  early-to-modern Colnect examples without verification.

Validation: 77 focused tests passed, 11 ignored; the opt-in Denmark test
passed separately. Five real standalone invocations covered the three
image-controls inputs and the two Denmark inputs. See the fixture README for
commands, image/source checks, failed attempts and filesystem/database impact.
Linux was exercised; Windows execution and human actual-size print/visual
acceptance remain pending. No production integration, OKF regeneration or PR
is authorized by this checkpoint.

### Future native visual-fixture and print-review requirements

**Completed reference and native slice (#93): stamp headings, offsets and padding.**
Branch `issue/93-stamp-headings` started from pushed #92 commit `41cba21a`.
The [AlbumEasy-only heading references](../../tests/fixtures/albumeasy/stamp-headings-v1/README.md)
contain two complete two-page Letter albums, each with 12 real images.
Unequal measured 40 x 26 / 50 x 40 mm stamps retain 6 mm total frame
additions. `alignment` compares local/VA headings with TOP/MIDDLE/BOTTOM
rows and unequal line counts; `padding-state` covers omitted/zero/signed
offsets, signed padding and saved/restored heading colour/padding.
All four final pages inspected and actual frame/caption bounds checked.
First attempts overflowed despite successful renders; corrected only
fixture furniture, not physical sizes. README records attempts/commands.

Key findings: local/VA headings change the row's reserved heading
footprint; even TOP rows can have different bare frame tops with local
headings of different line counts. Padding affects heading/frame footprint;
offsets are per heading and do not persist. Save/restore returns the
measured gap and colour. Native #93 mapping now covers standalone
compatibility input (`.txt` and raw-command `.stampalbum`) and explicit
native derivatives with a dedicated heading regression suite. Imported
parser and production routing remain out of scope. The two future ideas
recorded below remain backlog only.

**Current implementation slice (#94): stamp footer text alignment and padding.**
Branch `issue/94-footer-text-alignment-padding` contains the accepted
AlbumEasy reference pass and its standalone native implementation. The
[reference fixture](../../tests/fixtures/albumeasy/stamp-footer-text-v1/README.md)
records AlbumEasy PDFs and native compatibility-renderer proofs; the explicit
native-format input and proof are in
[stamp-footer-text-v1](../../tests/fixtures/native_pdf/stamp-footer-text-v1/).
Coverage includes `ALIGN`, `CENTRE`, `CNR_CENTRE`, US aliases, signed padding,
multiline captions, save/restore, `.txt`, raw-command `.stampalbum`, and an
explicit `STAMPALBUM 1` footer attachment that coexists with a heading.
`TEXT_LINE_LEADING (0)` is accepted as the default reset required by the
reference examples; nonzero leading remains an explicit unsupported case.

The implementation and proof generation are standalone. Do not route normal
application imports through this renderer until the full required AlbumEasy
stamp-command mapping is complete. Windows execution and actual-size print
review are still outstanding.

**Native checkpoint (#92): outer/inner border styles, CORNER and SIDES.**
Branch `issue/92-stamp-border-styles` starts from pushed #91 commit `f5ef92bc`.
The [AlbumEasy-only references](../../tests/fixtures/albumeasy/stamp-border-styles-v1/README.md)
contain two realistic two-page Letter albums, each with 12 verified real
images, physical frame clearance, borders, readable captions, margin text
and numbered footer. Outer ordinary styles/CORNER selectors/SIDES persistence
are covered in `album`; inner selectors and independent outer/inner
mask/style save/restore in `inner-state`. AlbumEasy renders both successfully;
all four reference page rasters inspected. Maintainer approved continuation
on 2026-11-07. Six native PDFs now cover both albums through TXT, identical
raw-command `.stampalbum` and explicit native-language `.stampalbum`.

Measured evidence: SIDES masks survive ordinary-style changes; CORNER ignores
empty masks; outer/inner masks are independent and restore with styles/colour.
Corners are joined two-leg 2 mm paths, not two separately stroked sides.
Corners use the same edge-gap centreline inset as ordinary inner borders,
but joined vertices do not use the full-side half-thickness endpoint
extensions. Unequal-width/hairline reference probes verify this distinction.
Actual paths are compared relative to each real image centre within 0.02 pt;
physical image sizes, dash arrays, colours, selectors and restored state
match both references. Trace stroke totals including page border: 23/11 for
`album`, 37/29 for `inner-state`. Exact commands, no-DB/filesystem impact,
tool warnings and follow-up risks are in the fixture README. Duplicate
selectors are idempotent, confirmed by a supplemental AlbumEasy reference;
unknown/malformed selectors fail with source lines before saving output.
Final scoped regression gate: 120 passed/19 ignored/zero failed; affected
gate: 22 passed/4 ignored/zero failed. All-target check, focused Clippy,
format/whitespace checks and six actual standalone dispatch runs passed.
Native-only proofs explicitly reserve omitted row-label space in a
22.175 mm vertical gap, keeping two pages/six images per page without
changing physical frames/images. TXT/raw retain the original 14 mm spacing
and all row labels; their four page rasters are identical. Whole-page
native/AlbumEasy layout parity, nonrectangular/rotated/text-only frames,
tiny-corner boxes, Windows execution and human native print acceptance
remain unverified. No production routing, PR or OKF regeneration.
Commit/push awaits explicit instruction.

**Native checkpoint (#91): stamp inner borders, ordinary styles and colour.**
Branch `issue/91-stamp-inner-borders` starts from pushed #90 commit `697eb0d3`.
The [AlbumEasy-only reference](../../tests/fixtures/albumeasy/stamp-inner-borders-v1/README.md)
has two complete Letter pages/12 real images, borders, physical frame sizes,
readable captions, footer and visible margin text. It covers 2 mm/0.2 mm
inner borders, SOLID/DOTTED/DASHED/DOT_DASH/BLANK, both colour spellings,
persistence, temporary 1 mm/0.35 mm settings, save/restore and cancellation.
The maintainer approved: "yes approved. continue with the process".
Shared typed inner state/layout and ordinary styles/colours now work through
TXT/raw/native inputs. Explicit four-argument
`NATIVE_STAMP_INNER_BORDER (padding-mm thickness-mm style colour)` captures
per-item state; both save/restore paths include inner dimensions/style/colour.
No new TOML keys or silent native numeric defaults are introduced.

Reference boundary evidence: `NO_FRAME` suppresses both frames, but outer
BLANK retains the inner frame. Actual inner sides have half-thickness
endpoint offsets; do not assume a naive inset rectangle. Trace confirms
20/16 coloured side strokes and six images on pages 1/2. Ordinary dash
arrays are measured in points. CORNER/SIDES, nonrectangular and rotated
interactions remain later obligations. Three persistent native main PDFs
retain all images, margins, footer and per-item inner strokes. Actual
reference/native side geometry agrees after translation within 0.02 pt,
thickness within 0.01 pt, colours within 0.001 RGB and identical dash arrays.
TXT/raw page rasters are identical; physical image dimensions across
native-language/TXT agree within 0.02 pt. Absolute row/title/caption placement
still differs; do not claim whole-page conformance.

A separate one-page/four-image zero probe establishes `(0 positive)` as a
zero-gap frame and `(positive 0)` as a PDF hairline; only `(0 0)` cancels.
The native model/writer preserves these meanings, with strict negative/
nonfinite/overflow and unmapped-style errors rather than silent omission.
Six new tests include saved-style restoration, output preservation and
non-mutating Denmark v5 rerenders (16 pages/41 images/all footers/no deferrals).
The expanded scoped gate passed 114 tests/18 ignored/zero failed; final
affected-suite rerun passed 16/3 ignored. All-target checking passed;
final focused lint/format/whitespace evidence is in the fixture README.
Accepted references and earlier fixtures remain immutable. Exact commands,
tool warning, geometry risks, development errors/fixes and filesystem/no-DB
report are in the README. Native visual/Actual-size print and Windows runtime
remain unverified.
No commit/push, production integration, PR or OKF regeneration in this pass.

**Native checkpoint (#90): vertical margin text, padding and colour.**
Branch `issue/90-margin-text` begins from `773747f9`. The
[AlbumEasy-only first pass](../../tests/fixtures/albumeasy/margin-text-v1/README.md)
has three two-page Letter references, each with 12 verified real images,
complete borders, frame clearance, row spacing, captions and numbered footer.
Cases: default L/R centred/UP darkblue; multiline TOP/DOWN and BOTTOM/UP
darkred with +2 mm padding/COLOUR spelling; alternating LR/RL darkgreen
with -1 mm padding on physical odd/even pages. AlbumEasy 4.5.0 rendered all
three successfully; complete-page rasters were inspected. The maintainer's
"continue" was interpreted and announced as reference acceptance.

Shared typed layout/writing now maps `ALBUM_PAGES_MARGIN_TXT`, signed padding
and both colour spellings. Explicit native `NATIVE_PAGE_MARGIN_TEXT` takes
eight arguments: font, point size, quoted text, L/R/LR/RL position,
TOP/CENTRE/BOTTOM alignment, UP/DOWN direction, total border-to-text
baseline-model gap mm and named colour. Native gap is not additional
AlbumEasy padding; compatibility uses the observed 2 mm base plus padding.
Native settings are explicit; no TOML/default keys changed.

Measured multiline columns move outward, parity uses physical page index
even with printed start 10, and final pre-page colour/padding applies
globally. Physical-page/vertical-margin overflow, overlap and mid-document
changes fail with preserved output. Native rejects a third definition;
the AlbumEasy duplicate-position probe warns/ignores it and creates a PDF
with exit 0. This strict deviation is documented, not exact conformance.

Nine persistent native PDFs cover TXT/raw/native inputs for all three cases.
Actual rotated reference/native transforms on both pages match within
0.01 pt. All retain 12 images/12 frames/two borders/every footer; physical
image dimensions agree across native-language/TXT routes within 0.02 pt.
Every TXT/raw page raster at 1200 pixels is byte-identical. Overall native
row/title/caption differences are not erased by these margin checks.

The [Denmark v5 proof](../../tests/fixtures/albumeasy/denmark-margin-native-v5/README.md)
restores only original line 45 `COLOR_ALBUM_MARGIN_TXT`: 16 pages/41 images/
all footers and zero commented deferrals. Denmark has no active margin text;
visible coverage belongs to the three new cases, not this colour-only line.
All v1-v4 bundles stay immutable. Zero deferrals does not certify full
language support, unprepared paths or historical PDF visual parity.

Validation: 108 passed/17 ignored/zero failed in the focused regression gate;
final strengthened margin tests 5 passed/1 ignored; Denmark generator 1
passed. All-target checking, focused Clippy, format/whitespace pass with
existing warnings. Two early assertions were corrected for the backend's
two-decimal graphics-transform serialization; no runtime workaround.
Exact commands, output table, filesystem/no-DB impact, error inventory and
continuation risks are in the fixture README. Linux execution is verified;
Windows runtime and native human visual/Actual-size Letter print acceptance
remain pending. No commit/push, production integration, PR or OKF regeneration
is included in this checkpoint.

**Next item started (#89): stamp-settings save/restore.** Branch
`issue/89-stamp-settings-save-restore` begins from `e090368e`.
The [AlbumEasy-only reference](../../tests/fixtures/albumeasy/stamp-settings-save-restore-v1/README.md)
renders two complete Letter pages, 12 measured stamp placements/eight visible
images, borders/spacing/captions and numbered footers. Baseline/temporary/
restored rows exercise size adjustment, image mm/percentage padding, aspect
ratio, grayscale, visibility, solid/blank borders and border/text/background
colours. This first pass was rendered only with AlbumEasy 4.5.0; the
maintainer accepted it: "Accept reference; proceed with native mapping."

Important new reference evidence: AlbumEasy rejects saving while a snapshot
already exists and restoring without a saved snapshot. The repeated-save
probe reports both errors, creates no PDF, but returns exit 0. Use semantic
output/diagnostics validation, not exit status alone. Save/restore is a single
outstanding snapshot consumed by restoration, not a nested stack.
The pinned help's full saved-state checklist also includes inner borders,
heading/footer padding/alignment and related colours; these remain additional
mapping/proof obligations rather than silently omitted supported behavior.
**Native checkpoint (#89):** Both compatibility commands now capture/restore
all currently mapped stamp drawing state (image options, adjustment, outline,
background and border/text/heading colours). Shared single-snapshot helpers
reject repeated saves or restores without a snapshot, with source-line
diagnostics; nonempty arguments fail. Per-item captured values are preserved.
Native `NATIVE_STAMP_SETTINGS_SAVE/RESTORE ()` capture its mutable image and
outline/background policies. Document-global frame/text policies remain
fixed; no speculative native style controls are added.

All three standalone routes render two Letter pages, 12 placements and eight
visible images. TXT/raw rasters are identical. Tests verify actual 58 x 35 /
32 x 35 mm baseline/restored frames and 60 x 37 / 34 x 37 mm temporary frames
within 0.02 points, restored image dimensions within 0.01 points, colours/
heading state, hidden-image behavior and lifecycle/error preservation.
The native-format fixture intentionally retains fixed darkblue frames,
black captions and 6 mm additions; it does not reproduce TXT's temporary
red frame/caption or +2 mm addition. Its temporary percentage padding is
additional to reserved geometry clearance, so temporary image sizes differ.
Keep these style/geometry capability gaps explicit rather than claiming
native-format/reference parity. General page-text colour coupling, row/
title/caption placement differences and complete inner-border/heading/footer
saved-state coverage remain future obligations.

The [Denmark v4 derivative](../../tests/fixtures/albumeasy/denmark-settings-native-v4/README.md)
restores save/restore, renders 16 pages/41 images/every footer and leaves
only margin-text colour (original line 45) deferred. All v1-v3 proofs remain
unchanged. Final focused gate: 103 passed, 16 ignored, 0 failed; inline
native captured-state test 1 passed; Denmark opt-in 1 passed. All-target
check, focused Clippy, formatting and whitespace checks passed (existing
library warnings). A stale unknown-command regression used newly supported
SAVE; changed only that test sentinel to still-unmapped inner-border syntax.
Fixture README records exact commands, development error fixes and no
database/source-image impact. Native human visual/print and Windows runtime
acceptance remain pending. The maintainer subsequently requested commit/push
on `issue/89-stamp-settings-save-restore`: precommit checks passed (unit 1,
integration 26 passed/4 ignored, formatting and whitespace checks exit 0).
Production integration, PR creation and OKF regeneration remain deferred.

**Next item started (#87): footer numbering and footer colour.** Branch
`issue/87-footer-numbering` begins from `4c94e22e`.
The [new AlbumEasy-only reference](../../tests/fixtures/albumeasy/footer-numbering-import-v1/README.md)
contains two complete Letter pages, 12 real-stamp images, borders, measured
frames and captions. AlbumEasy 4.5.0 rendered both explicit/default variants:
the main footer reads `Page 7 of 2` then `Page 8 of 2`; the default form
numbers pages 1 and 2. COLOR/COLOUR footer spellings are exercised in separate
variants. The maintainer accepted the subsequently added normal reference:
"Accept reference; proceed with native mapping." Actual-size print and
native visual acceptance remain pending.

**Native checkpoint (#87):** The bounded interpreter now maps
`ALBUM_PAGES_FOOTER_NUMBER` with its 3-6 arguments/defaults and both
`COLOR_ALBUM_FOOTER`/`COLOUR_ALBUM_FOOTER` spellings. Pre-page colour is
captured globally; changes after `PAGE_START` fail explicitly.
Native `NATIVE_PAGE_FOOTER_NUMBER` supplies all nine settings explicitly:
font, size, position, signed start, quoted prefix/suffix, padding,
minimum height and colour. Existing typed footer/numbering primitives
resolve `$PAGES$` from final pagination, independently of displayed start.
No new TOML settings, production routing or AlbumEasy subprocess is used.

Normal/custom-start/default compatibility inputs and native-language
counterparts produce two exact Letter pages and 12 images. Tests cover
footer colour/position/labels, borders, physical image-size equivalence,
optional arguments and alternating positions, invalid settings/duplicates,
overflow and output preservation. TXT/raw rasters are identical; measured
normal footer text bounds match AlbumEasy within 0.003 points.
Native sections paginate 8/4 images versus TXT's 6/6 explicit pages;
earlier rows, title repetition and native section-heading omissions remain
documented differences. Do not claim overall visual parity.

The [Denmark v3 derivative](../../tests/fixtures/albumeasy/denmark-footer-native-v3/README.md)
restores the two footer occurrences, preserving 16 pages/41 images and
`Page N of 16` on every page. Three occurrences remain deferred:
margin-text colour (line 45), settings save (68) and restore (83).
The preceding eight-deferral table and subsequent five-deferral v2 report
are historical evidence, not the current count. Previous bundles stay
immutable. Focused validation: 99 passed, 15 ignored; Denmark opt-in 1 passed;
all-target check and focused Clippy passed with pre-existing warnings.
Exact commands, failure fixes and filesystem/database impact are in the
footer fixture README. Windows runtime and human native print review remain
unverified. The maintainer subsequently requested commit/push of this
checkpoint; precommit validation passed (33 tests, 4 ignored), formatting
and whitespace checks passed. PR creation and OKF regeneration remain
deferred until the native PDF work is complete.

During #87 reference review the maintainer requested retaining `Page 7 of 2`
as an import edge case and adding a normal `Page 1 of 2`/`Page 2 of 2`
collector-facing reference. Keep that distinction explicit: AlbumEasy's
optional start number does not change the actual `$PAGES$` document length.

**Bounded native section numbering completed (#88).** Native `.stampalbum`
documents can opt into section-local footer labels through
`[native_pdf.footer_number]`; `NATIVE_PAGE_NUMBER_SECTION` explicitly starts
the next section. A physical page containing the marker remains in the
preceding section, and numbering resets on the next page. The configured
`section_start` defaults to 1 and supports a custom value such as 10.
`$SECTION_PAGE$`, `$SECTION_PAGES$`, and `$DOCUMENT_PAGES$` expose the section
ordinal, section length, and final document length; `$PAGES$` remains the
document-length alias. Ten pages starting at 10 end at 19. Years are not
inferred from captions. AlbumEasy TXT and raw-command routes retain their
literal token behavior, and application routing remains out of scope. The
standalone renderer accepts an optional TOML path for native `.stampalbum`
input only. This completes the bounded #88 slice, not the broader language or
application roadmap tasks.

**Next item started (#86): stamp background colours.** Branch
`issue/86-stamp-background-colours` begins from `47b35606`.
The [AlbumEasy-only first-pass fixture](../../tests/fixtures/albumeasy/stamp-background-colours-v1/README.md)
renders one exact Letter page with eight real-stamp placements (seven visible
images), three rows, border/title and readable measured-size captions.
It covers default/black/persistent/lightblue/white backgrounds, both colour
spellings, `NO_FRAME`, `BLANK` and hidden images. AlbumEasy 4.5.0 rendered it
successfully; the maintainer accepted this first pass before implementation.
Actual-size print review and native-pass acceptance remain pending.

Important observed mapping distinction: `NO_FRAME` suppresses both fill and
outline, while `STAMP_BORDER_STYLE (BLANK)` suppresses the outline **but keeps
the fill**. Hidden images also retain their filled frame. The current native
no-outline boolean conflated the first two; the new imported-page background
state preserves their distinct behavior. Native documents now support
`NATIVE_STAMP_BACKGROUND (colour|none)` and independent
`NATIVE_STAMP_OUTLINE (show|hide)`, captured per item without changing physical
geometry or existing default no-fill behavior. Fill painting precedes images
and outlines/text remain visible.

All three standalone inputs now pass the background fixture: one exact Letter
page, seven visible images, six backgrounds and six outlines plus page border.
Tests assert exact colour/persistence order and paint layering; TXT/raw
content streams and 1400-pixel rasters are identical. Native visible image
widths/heights match the TXT native route within 0.02 points. The native frame
already reserves mounting clearance, so the fixture uses zero **additional**
image padding rather than doubling that clearance.

The new [Denmark v2 derivative](../../tests/fixtures/albumeasy/denmark-background-native-v2/README.md)
restores all three background occurrences and renders 16 pages/41 images
from each compatibility input. It retains five commented occurrences:
footer colour, margin-text colour, footer numbering and save/restore.
The eight-deferral table above is the historical #84 checkpoint; its
background row is now implemented for these rectangular image-box cases.
At the #86 checkpoint those five occurrences remained follow-up work; #87
above restores the two footer occurrences, leaving three. No arbitrary-shape,
rotated-box or CORNER behavior is certified by this bounded slice.

Validation: 80 focused tests passed, 12 ignored; Denmark v2 opt-in passed
separately. All-target type-check and focused Clippy passed with existing
warnings. Background tests additionally verify invalid colours preserve
output. Reference/native screen comparison records earlier native row/title
placement, omitted native-language section headings and other spacing
differences rather than claiming exact parity. Human native visual/print
acceptance and Windows runtime verification remain pending. Existing v1
fixtures, production routing and the OKF bundle remain untouched. The
maintainer requested commit/push after this checkpoint; PR creation remains
deferred until native PDF work is complete.

For future paired examples, keep both passes in an inspectable fixture bundle
under `tests/fixtures/`. First render and review the AlbumEasy-only `.txt` and
its reference `.pdf` on a complete realistic album page. Only after that page
is accepted, create the program-owned `.stampalbum` and directly rendered
native `.pdf` for comparison. The older TOML fragments in existing fixture sets
predate the approved native file format; use `.stampalbum` for new native
documents rather than treating TOML as the native album language.

Use real stamp images with positive recorded Colnect physical dimensions,
preferably records and associated images already available in the local
`Album` databases. Keep source record IDs, measured dimensions, and asset
provenance with the fixture, and read source databases without modifying them.
Automated checks should verify the document and rendered geometry, but they do
not replace the human review: print each native PDF at actual size, inspect the
page and stamp appearance, and record the maintainer's visual acceptance before
considering the example complete.

Website-ready galleries showing command text alongside rendered output are a
later, separate work item. Do not expand the current renderer slice to build
that gallery.

### Current implementation checkpoint — verify and implement together

#### Section row alignment and new native centered default (#82)

After pagination commits `3f48f945` and `73a0f882`, the maintainer selected row
alignment. Native `left`/`right` keep configured internal gaps and anchor the
complete row at the respective usable-page edge. `justified` spreads spare width
between items, with no outside gaps. The approved NEW NATIVE DEFAULT, `center`,
distributes equal internal and outside gaps, rather than centering a compact
fixed-gap row. If those equal gaps would fall below the configured minimum
internal spacing, retain the minimum and split only the remaining width at the
two outer edges. Both center and justified center a singleton.

Alignment applies to complete text/frame footprints, not image rectangles;
physical sizes, mounting clearance, text metrics and vertical layout do not change.
Each explicit `StampRowSection` starts a new row and applies its alignment to
all rows, including short final rows and text-only continuations. A later section
can change policy on the same page. Page parity, final-count reservations,
asset validation and image fitting remain shared with the existing paginator.
Historical low-level left-fitting APIs stay available; settings-based native
proof writers use the new centered default. Existing proofs explicitly retain
their original left policy so old ledgers and reproducible fixture generation
do not change silently.

Optional `[native_pdf.rows].alignment` is validated by the native adapter;
absent table means the approved center default, while an invalid/missing key in
a present table is an error. Both templates contain matching detailed per-key
UI help, checked through the existing config-editor parser. Legacy production
AlbumEasy generation and global config behavior are unchanged.

[Real Letter PDF/TXT/TOML examples](../../tests/fixtures/albumeasy/row-alignment-v1/README.md)
reuse five known-size US records/images without database writes or asset copies.
Four mode proofs, mixed sections and a three-page centered caption continuation
have independently verified image transforms, all physical/frame/footprint bounds
and every text baseline using MuPDF. No AlbumEasy parity is claimed for adapted TXT.

**Future native commands (planning only):** justified with last-row-left,
justified with last-row-center, and justified-all-rows. Do not overload current
`justified` with an automatic last-row exception. When compatibility export is
implemented, explicitly project these enhancements to the three main AlbumEasy
modes, with divergence findings. Exact names/mappings remain to be designed;
no unsupported commands are emitted now.

Branch `issue/82-native-stamp-row-alignment`, based on `73a0f882`. No OKF
regeneration, production backend switch, commit or push in this slice.
Final native regression: 131 passed, 0 failed, 22 ignored across 22 suites;
four config-editor tests pass. Clippy, formatting and Windows GNU library
checks pass with pre-existing warnings. Physical printing and Windows execution
remain unverified.
T009/T014/T015 remain partial outside the bounded alignment work.

#### Paginated real-stamp image/text proofs with line-boundary continuation (#81)

After #80, pagination was implemented for complete stamp/image/text footprints
using per-page resolved content bounds (including odd/even margins, title/header/footer
reservations and final page count). Pending items are repacked on each new page
with preserved input order; no scaling or geometry mutation is used to force fit.
Frames are indivisible and appear once. When a complete item footprint exceeds a
fresh page, heading/caption text continues at measured line boundaries across pages
without dropping/reordering lines, while the stamp frame remains on the first page
where enough vertical space exists.

Final page-count dependent layout is resolved iteratively until converged, then
rendered from prevalidated page plans. All page layouts and every image asset are
prepared before writing output, so failures on later pages do not truncate/overwrite
existing files. Existing single-page proof APIs remain unchanged.

Real source proofs now use 24 pre-1978 US records with positive recorded
`colnect_width`/`colnect_height` and existing local PNG assets. Selection is
read-only from `/home/ray/AlbumGen2/Albums/United-States/United-States.db`,
with source checksum unchanged before/after
`0e06f8b7f5ec92d8e220a6324018459e31679341360fbf203893e44f7c663e55`.
Provenance and checksums for each selected record/image are stored in
[source-records.json](../../tests/fixtures/albumeasy/stamp-pagination-v1/source-records.json).

[Letter PDF/TXT proofs](../../tests/fixtures/albumeasy/stamp-pagination-v1/README.md)
cover wrapped, expanded and overflow-caption continuation cases. Independent matrix
tests verify every image appears once on the correct page with expected bounds; footer
numbering matches final page count; long captions continue exactly once per line.
MuPDF confirms Letter mediaboxes and expected page counts.

Validation: pagination+proof suites pass; neighboring native row/text/image suites
pass; Windows GNU compile check passes with existing warnings. No OKF regeneration.
Branch `issue/81-native-stamp-row-pagination`, based on `10af27c8`; maintainer
selected repack-on-new-page and overflow text continuation.

#### Combined stamp images and measured text rows (#80)

After #79 was committed as `47530168`, the maintainer selected image/text rows
before pagination. `StampImageRowItem` supplies an existing known-size/text
item, portable image path and explicit image scale. The diagnostic renderer
uses the shared text-footprint fitter and existing image preparation without
recalculating physical sizes. Images fit proportionally inside physical bounds
using frame clearance, remain centered, and never upscale. Wrapped/expanded
text affects reserved row footprints only, not image or mounting dimensions.

All images are prepared before PDF output; missing/corrupt last-item assets
or invalid scales fail without creating/truncating output. Each placement has
a unique image resource in multi-image proofs; the historical single-image
resource name stays unchanged. The slice remains explicitly single-page.

[Letter PDF/TXT proofs](../../tests/fixtures/albumeasy/stamp-items-v1/README.md)
show eight items per policy, with small and fitted portrait/landscape images
and two rows. Three tests check actual multi-image PDF matrices/resource
names and safe failure behavior. Independent MuPDF traces verify all sixteen
image dimensions/positions/aspects, all footprint/frame/physical outlines,
every text line/position, 10 pt fonts and 3 mm gaps. Both pages were visually
inspected. Synthetic assets avoid modifying any real album/image/database.
Validation: 117 native tests pass, 20 ignored; explicit fixture regeneration
passes. Clippy/format checks pass with existing warnings. Windows GNU library
check passes with the existing missing `winres` warning; Windows runtime and
physical printing remain unverified.
TXT companions explicitly preserve images/text in adapted separate blocks;
native-position/page parity and reference rendering are not claimed.

Branch `issue/80-native-images-text-rows`, based on `47530168`; maintainer
approved this slice for commit, nothing pushed. No OKF regeneration. Next bounded
layout step: automatic row-to-page overflow with resolved header/footer and
even-page reservations, without shrinking stamps or dropping text.

#### Native stamp-text policies and UI-ready configuration help (#79)

Approved approach: extend the existing native TOML mechanism, not a second
configuration system. Optional `[native_pdf.stamp_text]` explicitly opts in
through `enabled` and requires `width_policy`, `horizontal_padding_mm`,
`vertical_padding_mm` and `text_gap_mm` when present. Both templates keep it
disabled and supply the same detailed comments immediately before each key.
The existing config editor uses those comments as field help; a regression
test checks per-key delivery and agreement between both template files.
The settings remain proof/layout-only, not a production backend switch.

`wrap_to_frame` measures and wraps heading/caption text to mounting-frame width,
preserving explicit newlines and blank lines, normalizing repeated spaces
and splitting overlong words without reducing fonts. A glyph wider than the
frame fails explicitly. This is compatibility-oriented, not certified
AlbumEasy parity. `expand_footprint` allows wider text up to the explicit
`expanded_width_limit`/`expanded_width_value` cap, then wraps while preserving
explicit newlines. The cap is either percent of mounting-frame width
(120 = 20% wider) or absolute mm (30 = 30 mm), before padding. A cap below
the frame or overflowing PDF units is an error, never a request to shrink.
Both reserve complete measured multiline height without a fixed line-count
limit; single-page overflow remains explicit, not truncation/pagination.
Footprint width is max(frame width, widest text line) plus twice horizontal
padding. Height includes frame, each present text block/gap and twice vertical
padding. Padding is per edge, independent of physical clearance/page margins;
inter-item/row gaps apply outside the complete footprint. Physical stamps,
mounting frames and illustration scale are unchanged.

The shared frame fitter packs these complete footprints, preserving order;
frames are centered horizontally within them. Footprints, not frame tops,
are top-aligned. Text baselines use the same verified standard-font metrics
as drawing. Existing single-stamp/page proofs are unchanged. Invalid settings,
overwide footprints and single-page height overflow fail before output writes.
No automatic pagination or production image/text-aware generation is claimed.

Compatibility contract: native policy keys must never be emitted as unknown
AlbumEasy commands. Preserve actual text; identify every adaptation in TXT
comments and compatibility findings. Keep authoritative native settings in TOML,
not only comments. Production TXT export is unchanged. Current
[Letter comparison proofs](../../tests/fixtures/albumeasy/stamp-text-v1/README.md)
contain native PDF/TOML fragments plus deliberately adapted TXT inputs
that retain all text as separate page-text blocks without claiming matching
native positions or enlargement of mounting frames.

Validation: ten text tests pass; three config-editor tests pass (including
help delivery). The commit-gate native regression passed 114 tests with 19
ignored; all ten text tests pass. Explicit fixture generation passes separately.
Independent MuPDF verification confirms all physical/frame/footprint bounds,
every stamp text line/baseline/left position, 10 pt fonts and row gaps on all four
Letter pages. Wrap mode has rows 5+1, expanded mode at a 200% cap has rows 3+3
for the same six stamps. Two additional pages exercise four-line long text at
120% and 30 mm caps; regression coverage verifies 30 lines without a line cap
and distinct measurable percentage/mm wrap boundaries.
Clippy and Windows GNU library checks pass with existing warnings; Windows
runtime, physical printing and AlbumEasy reference rendering remain unverified.
No OKF regeneration. Branch `issue/79-native-stamp-text-footprints`; approved
for commit, nothing pushed. T008/T009 and production compatibility export
remain partial.

#### Frame-only known-size row fitting (#78, bounded T009 slice)

The maintainer selected geometry-only fitting first, deferring per-stamp text
footprints. `fit_known_size_rows` preserves input order and physical/frame
dimensions, places frames left-to-right, and wraps to the next row without a
warning or scaling. This proof uses left-aligned rows and top-aligned frames;
each row advances below its tallest frame by the configured vertical gap.
There is no trailing gap requirement at the content edge.

The fitter uses resolved content bounds, not raw paper dimensions, so existing
title/header/footer reservations remain respected. Empty input returns no rows;
invalid bounds, an overwide frame or insufficient single-page height fail
explicitly. Automatic pagination, images and per-stamp text are not included.
The shared diagnostic renderer accepts the placed geometries without changing
existing image/page proofs. Validation errors occur before creating/truncating
output files.

Six row-fitting tests cover exact width/height fits, gap accounting, ordered
wrapping, mixed heights, fractional/zero clearance, invalid/oversized input,
unchanged physical sizes and actual Letter PDF rectangle coordinates.
[Four PDF/TXT proof pairs](../../tests/fixtures/albumeasy/stamp-rows-v1/README.md)
and a combined PDF cover ordinary wrapping, mixed heights, large items and
zero clearance. Independent MuPDF traces confirm coordinates, alignment and
gaps; pages 1 and 2 were visually inspected. The explicit fixture-regeneration
test lives under `tests/`; regular tests do not overwrite saved proofs.
Validation: 104 native integration tests pass, 18 ignored (including opt-in
fixture regeneration); the explicit generator passes separately. Focused
Clippy/format checks pass with existing warnings. Windows GNU library check
passes with the pre-existing missing `winres` warning; Windows runtime and
revised physical printing are not verified. No OKF regeneration was performed
for this slice.

Branch `issue/78-native-known-size-row-fitting`, based on committed #77
(`f5e3ffcf`). Maintainer approved this slice for commit; nothing pushed.
Full T009 remains partial. Next: decide and measure per-stamp text footprints
before text-aware packing/pagination or production integration.

#### Known-size physical bounds and frame clearance (#77, T007 slice)

Added `StampBoxGeometry` with validated f64 millimetre bounds in PDF bottom-left
coordinates. Physical bounds include outer perforation tips and remain
unchanged. The existing settings retain their approved total-addition meaning:
frame width = physical width + `extra_width_mm` + `stamp_box_adjust`;
height uses `extra_height_mm` analogously. Each total addition is split equally
between its two edges. Thus 22x25 mm plus 6 mm on each total dimension produces
a 28x31 mm frame, not 34x37 mm.

Signed components remain supported when the resulting net additions are
nonnegative. The isolated geometry rejects net negative clearance rather than
shrinking a known physical stamp; no legacy generator validation was changed.
NaN, infinity, inverted/empty bounds and PDF-coordinate/extent overflow fail
explicitly. Sizes are never capped at 25 mm or rotated to fit.

`StampImageProofSettings::with_stamp_box` uses the mounting frame for
heading/caption anchors and its per-edge clearance to fit the illustration
within the physical bounds. It preserves the caller's image scale and the
existing no-upscale rule. PDF read-back verifies both small natural-size and
down-to-fit image transforms/centres while the 22x25 mm physical size and
28x31 mm frame stay fixed. This does not yet draw a stamp-frame stroke, reserve
interior text, infer unknown sizes, pack rows, or wire production/CLI/UI/config.

Validation:

- `cargo test --test native_pdf_stamp_box --test native_pdf_stamp_image`:
  8 passed, 0 failed, 0 ignored.
- `cargo test --lib native_pdf::stamp_image::tests`:
  2 passed, 0 failed, 0 ignored.
- Tests cover 22.5 mm fractional sizes, landscape stamps, 44x90 mm booklets,
  125x66 mm se-tenant items, zero/unequal clearance and signed adjustments.
  PDF matrix checks allow only the writer's 0.005-point rounding bound.
- Focused rustfmt and Clippy pass; pre-existing application/parser warnings
  remain. Windows GNU library check passes with the existing unavailable
  `winres` resource-tool warning; Windows runtime and physical printing are
  not verified. The local `knowledge/` bundle was regenerated; its validator
  reports reciprocal-relationship/orphan warnings in unrelated concepts.
- Tests use temporary synthetic images/PDFs; no album DB, image, config or
  saved PDF was modified. Initial red tests exposed the missing module, then
  two test-expectation issues (a finite overflow fixture and PDF rounding);
  both expectations were corrected and the final suite is green.

Persistent [PDF/TXT visual proofs](../../tests/fixtures/albumeasy/stamp-frame-v1/README.md)
now supplement those temporary tests. The reproducible `stamp_frame_proofs`
example writes ten labelled Letter PDFs, a synthetic image, measurements and an
AlbumEasy frame/physical-rectangle comparison TXT. The saved combined PDF has
all ten cases, including a 100x100 mm print calibration square. A dedicated
diagnostic renderer draws black frame and blue physical outlines; production
frame styling remains unimplemented. Independent MuPDF traces verify every
page's extents within 0.004 mm and image sizes within 0.002 mm; three raster
pages were visually inspected. The AlbumEasy TXT is not yet reference-rendered
and must not be described as native image/text parity. Physical printing is
still pending maintainer review.

Print feedback: the original A4 proof's page 1 printed on Letter measured
22x25 mm physical and 28x31 mm frame, matching calculations. Image measured
11x13 mm versus calculated 11x12.5 mm; no correction was inferred from this.
The missing bottom page border is consistent with the paper-size mismatch.
The proof example now uses Letter (215.9x279.4 mm), 10 pt diagnostic labels
instead of 4 pt and 14 pt headings instead of 7 pt, with unchanged stamp/frame
dimensions. All ten revised pages pass independent geometry/font checks;
revised Letter printing and two-axis calibration remain pending.

Branch `issue/77-native-stamp-frame-geometry`, based on `e9b7ca4e`.
The GitHub instruction correction is included with this slice;
manual autosave changes are unrelated and excluded. No merge or push.
Next: bounded known-size row fitting using frame extents without scaling
physical mounting dimensions; resolve text/frame footprint rules before
broader T009 packing. Full T005/T007 remain partial.

Project verification conventions: keep new test/proof inputs and generated
TXT/PDF fixtures under `tests/`, following the versioned directory/file style
of `tests/fixtures/albumeasy/border-v1`, not under `docs/`. Use US Letter
(215.9x279.4 mm) for new print proofs. A future reusable sample area (website
or application) is deferred; do not build it as part of this conversion.
Defer OKF knowledge regeneration until the native PDF conversion ends unless
needed for the work. Already-generated bundle changes predate that decision
and are left uncommitted for the end-of-conversion refresh.

#### Shared physical-sizing evidence and layout requirements

The [stamp physical-sizing analysis](STAMP_PHYSICAL_SIZING_ANALYSIS.md) is the
preserved project-wide baseline for sizing, mounting spaces, and future layout
work, not only the native proof. Its complete
[per-record results](../testing/us-sizing-20261105T031956Z.csv) and
[machine-readable breakdown](../testing/us-sizing-20261105T031956Z.json) are
versionable project artifacts. Read and reuse them before further T005 sizing
or row-layout work; do not repeat the completed experiment unless inputs,
configuration, estimator, or validation scope changes.

Agreed requirements: known sizes include outer perforation tips; preserve
physical mounting dimensions independently of illustration scaling, text,
and configurable frame clearance. Move items to the next row when remaining
space is insufficient, without warning for normal wrapping. Infer unknown
sizes automatically from comparable items and reference samples; fine-tuning
follows viewing/printing. Same-series references are preferred, but single,
pair, block, pane, and booklet identity must be distinguished.

The US held-out comparison covers 248 known-size records. Hierarchical peers
predict 229, with 196 within 1 mm on both dimensions; excluding the entire
target series gives 127 within 1 mm out of 187 predictions. These are
conditional experimental results, not approved mounting tolerances. Recorded
risks include stale pixel dimensions, annotated images, single/block ambiguity,
and sparse metadata. No production estimator was changed or approved.

#### Native stamp-image box proof (T005 slice)

Added `write_stamp_image_proof` as a diagnostic-only extension of the native
renderer. It places one PNG/JPEG in explicit millimetre bounds, preserves its
aspect ratio, applies caller-supplied padding and source point-per-pixel scale,
and does not enlarge the source. Measured heading and caption lines are placed
above and below the box; the configured page border and content guide remain
unchanged. Asset, geometry, and text validation happens before output is
written, so an invalid/missing image does not create or truncate the PDF.

The image geometry now has a focused comparison against two image footprints
measured in the Great Britain AlbumEasy PDF: 398x473 px renders at about
17.67x21 mm, and 117x140 px at about 17.55x21 mm, each inside a 24x27 mm
`STAMP_ADD_IMG` box. The regression test uses synthetic images with those
raster dimensions, derives scale from the embedded 120x140 reference image at
25x30 mm, and applies 3 mm of padding per side; both results match the
measured footprints within 0.01 mm. This validates the proof geometry for
these sample cases, not a universal AlbumEasy padding policy or real-world
stamp-size accuracy. Text placement still requires reference-renderer
comparison before production wiring. This is one image, not row packing; it
does not add CLI/UI/config integration or alter PDF import.

The read-only Great Britain database check found 1,317 stamp rows: 1,077
nonempty image paths (all resolve), 240 rows with pixel-dimension pairs but no
image path, and 20 rows with Colnect physical-dimension pairs; none has both
dimension pairs.

Validation: the focused image geometry tests, both stamp-image integration
tests, 20 neighboring config/page/title/border tests, and all 11 decorative
renderer comparisons (including the `mutool` raster/trace checks) pass.
Targeted Clippy reports no new warnings; existing application/PDF-parser
warnings remain. The Windows GNU library check succeeds, but `winres` is not
available in this environment, so Windows resource embedding/runtime and
physical printing remain unverified.

Validated on `feature/native-stamp-image-layout`, based on `d422da10`.
The physical-versus-presentation box geometry follow-up for known sizes
(T007) is recorded above, keeping clearance independent from image scale.
The experimental unknown-size estimator remains unapproved; full T005,
including physical print verification, remains partial. No merge or push.

#### Decorative edge ornaments (#76)

The strict decorative-border definition parser now supports `IMAGE_EDGE_LEFT`,
`IMAGE_EDGE_RIGHT`, `IMAGE_EDGE_TOP`, `IMAGE_EDGE_BOTTOM`, and
`IMAGE_EDGE_PAD(h v)` alongside the corner and connecting-line commands.
Definitions may contain any combination of corner and edge PNGs but must define
at least one image. PNGs remain fixed-size; right/bottom artwork mirrors the
corresponding left/top asset. Horizontal edge padding moves left/right artwork;
vertical padding moves top/bottom artwork. Measured placement matches
AlbumEasy 4.5 on asymmetric odd/even margins.

The explicit native policy now has thirteen numeric values, including signed
horizontal/vertical edge padding; both templates and the local PDF config
provide the two zero-valued settings. Disabled configuration still opens no
files, definitions override their matching policy values, and missing,
corrupt, or non-alpha PNGs fail explicitly for all edge directions.

Seven synthetic cases/eight pages were independently repeated with identical
complete page traces; 24 hashes freeze the reference sources, definitions,
PNGs, and PDFs. Native comparisons verify edge/corner transforms, image sizes,
line geometry/colour, MediaBox, and content anchors. Raster checks sample
corners plus edge-only pages with odd/even margins. Edges, like corners, do
not reserve content space; conservative footprints report clipping or possible
content overlap without silently moving artwork or text.

Focused validation: **11 passed, 0 failed, 0 ignored** when including the two
`mutool`-dependent checks. Changed Rust files pass formatting and targeted
Clippy; workspace-wide formatting still reports unrelated whitespace in
`tests/album_stamp_img_integration_test.rs`. ICC and pre-existing application
warnings remain. Windows runtime and physical printing are not verified.

Branch `issue/76-native-pdf-decorative-edges`; no commit, merge, or push.
Production CLI/UI/backend integration remains pending.

#### Decorative corner ornaments and connecting lines (#75)

The maintainer approved `ALBUM_PAGES_DECORATIVE_BORDER` and selected corner
ornaments plus connecting lines first. A separate command module loads a strict
definition containing `IMAGE_CORNER`, `IMAGE_SCALE`, `IMAGE_CORNER_PAD`,
`BORDER_LINES`, `BORDER_PAD` and `BORDER_END_OFFSETS`. Other unsupported
commands fail with definition path/line rather than being silently omitted.
Transparent PNG artwork in the fixtures is original synthetic test material,
not copied AlbumEasy artwork.

The optional complete `[native_pdf.decorative_border]` table initially had
fourteen explicit fields: disabled/empty definition filename, scale 1, all padding,
width/spacing/end-offset values 0, and black connecting lines. All fields
validate even when disabled; absent tables preserve older configurations.
Definition commands override their corresponding explicit omitted-command
policy. Enabled relative definitions resolve beside the config, PNGs beside
the definition; in-memory relative definitions require an explicit base path.
There is no global/AlbumEasy directory search. Definition and PNG are loaded
once and snapshotted, so later asset deletion does not change rendering.
Errors identify settings without echoing arbitrary configuration/source values.

Five AlbumEasy 4.5 cases/six pages were independently repeated with identical
complete page traces; sixteen hashes freeze source/definition/PDF/PNG assets.
Actual image and alpha-mask transforms, connecting stroke topology/endpoints,
widths/colours, MediaBox and content anchors match. Pixel dimensions represent
points multiplied by scale; corners mirror rather than rotate. Positive
corner/line padding moves outward. Line centrelines advance by slot index
times spacing, not cumulative width; disabled slots retain their spacing.
Line endpoints remain based on unpadded margin corners. Sparse lines, signed
padding, asymmetric even pages, fractional landscape paper and coexistence
with a red numeric border are covered. Independent native line colour does
not recolour PNGs. Raster samples verify opaque, partially transparent and
clear pixels on mirrored corners, within two RGB levels per channel.

Decorative borders intentionally reserve no content space. Numeric borders
retain their existing content-inset semantics. Conservative rectangular PNG
footprints and exact butt-capped stroke footprints produce page-specific
clipping/content-intersection warnings without moving/resizing anything.
Transparent pixels may be clear; this is not exact alpha/text collision
certification. Dedicated header/footer/title collision checks against artwork
are not implemented. A future UI TODO records correction choices; users must
check text clearance. Existing text-overlap errors and all-page preflight remain.

Validation: **117 passed, 0 failed, 0 ignored** (106 native integration tests,
including ten decorative tests; ten main-config tests; one collision unit test).
Additional checks cover strict syntax/unsupported commands, duplicate overrides,
missing/corrupt/no-alpha PNGs, config/definition paths with spaces, asset
snapshots, redaction, missing/disabled/template/unknown keys, finite geometry,
overflow and later-page failure preserving new/existing output.
Focused Clippy/format/scoped whitespace checks and Windows GNU library/example
compilation pass. Windows runtime/default-feature build and physical printing
remain unverified; existing application/ICC warnings remain.

The three-page demo was visually inspected with mirrored corners, connecting
lines, crop marks, header/date/number/title/footer and asymmetric even margins
coexisting without visible text collision. Page 1 content bottom/top stay
**36.0936/265.1549 mm**; no demo clipping/content warnings. The ignored local
config has only a disabled decorative table appended; existing user values and
three-page geometry-only output/227 unapplied keys are preserved.
An intentional two-page oversized-artwork run emits exactly four actionable
warnings (clipping plus content intersection on each page) while both pages
retain their requested 100 x 150 mm MediaBox.

Branch `issue/75-native-pdf-decorative-border`; no commit/merge/push requested.
Broad T005/T006/T007/T046 remain partial. Production routes/import/pagination,
databases, historical diagram and tagged album-globals test remain untouched.
Unrelated manual/design/autosave staging and the existing roadmap EOF cleanup
are preserved. Session evidence includes demos, traces, reports and isolated
regenerated knowledge. Crop marks were locally committed as `68102d2a`.

#### Crop marks with explicit clipping policy (#74)

The maintainer approved `ALBUM_PAGES_CROP_MARKS`, then explicitly chose to match
AlbumEasy's unchanged MediaBox/partially clipped arms rather than silently
enlarging paper. The separate command module resolves four crosses into the
shared page layout; drawing uses those exact endpoints/width/colour. The complete
optional `[native_pdf.crop_marks]` table has shipped defaults disabled, line
0.1 mm, horizontal/vertical padding 1/1 mm, arms 5 mm, black. All six fields,
including disabled settings, validate; absence retains older geometry-only
behavior. No numeric/colour fallback exists in the command/loader. Black is
independent of album border colour; editable named colour is a native extension.

Nine AlbumEasy 4.5 cases/11 pages were independently repeated with identical
complete page traces. Eighteen hashes freeze source/PDF pairs. References cover
omitted/default/one-/two-/three-/four-argument commands, custom dimensions,
fractional landscape pages, asymmetric even margins, zero dimensions and pink
album border with black crop marks. Actual native/reference stroke topology,
affine-transformed endpoints, widths and colours match within 0.01 mm/0.00051
RGB; PDF page dimensions remain exactly those requested within that tolerance.
The fixture-only optional argument adapter is not a production TXT importer.

Actual centres are padding minus half line width inward from physical page
edges; arms are half-lengths of each disconnected segment. Zero width means a
PDF device hairline, not off. Zero arms are degenerate butt-capped segments
that paint nothing. Physical-page clipping is measured from the finite stroke
footprint (hairlines use centreline; device pixels are not certified). Defaults
clip; native emits one actionable warning per affected physical page with
padding/arms/line-width keys. A future UI TODO records correction choices.
No automatic resizing, trim-box feature, margin shifts or content reservation
is introduced. Crop/text/border collision avoidance is not implemented; users
must choose sufficient clear margins/padding. Existing text-overlap errors and
all-page preflight remain unchanged.

Validation: **107 passed, 0 failed, 0 ignored** (96 native integration tests,
including ten crop-mark tests; ten main-config tests; one collision unit test).
The crop suite also checks exact edge touching, invalid/missing/disabled keys,
redaction, explicit template defaults, unknown-key reporting, independent
snapshots/colour, PDF-coordinate overflow and new/existing output preservation.
All seven measured title/header/footer text/date/number components retain
their glyph positions and content bounds on every page with crop marks enabled.
Focused Clippy/format/scoped whitespace checks and Windows GNU library/example
compilation pass. Windows runtime/default-feature build, physical printing and
device hairline footprint remain unverified. Existing app/ICC warnings remain.

The three-page demo uses fully visible black 0.25 mm crosses, padding 8/9 mm
and arms 2.5 mm; all three rasters were inspected with no text/border collision.
Asymmetric margins do not move marks away from the physical corners. The
separate default-settings demo emits exactly three page-specific clipping
warnings, while the visible demo emits none. Header/date/number/title/footer
coexist; content bottom/top stay **36.0936/265.1549 mm** on page 1. The ignored
local config has only a disabled crop table appended; existing user settings
and geometry-only three-page output/227 unapplied keys are preserved.

Branch `issue/74-native-pdf-crop-marks`; maintainer approved the completed slice
and requested a local commit. All 107 relevant tests passed again before
committing; no merge/push requested.
Broad T005/T006/T007/T046 remain partial. No production routes, parser/import,
database or historical diagram changes; unrelated manual/design/autosave and
tagged album-globals test remain untouched. Synthetic fixtures were copied;
no inputs moved/deleted. Session-only evidence includes reports, demos, traces
and isolated regenerated knowledge. Footer-date work was locally committed
as `ca5a01b3` before this slice.

#### Footer generation dates and shared date text (#73)

The maintainer approved `ALBUM_PAGES_FOOTER_DATE`. Its separate command module
reuses footer placement/measurement and shared date text/source/format logic.
Explicit `[native_pdf.footer_date]` defaults are disabled/HN/8 pt/L/YMD/"-"/
"Generated: "/empty suffix/generation/2026-01-01 fixed override. The existing
footer policy supplies padding/minimum height/colour, even if plain text is
disabled. No hidden rendering values are added and local edits are preserved.
Date font/size/position/text are independent of plain footer and numbering.

Header/footer generation modes share a single local calendar date captured
outside the page loop; either may independently use its fixed override.
Footer-only generation captures a date too. Shared `DateText` owns formatting,
fragments and source resolution rather than duplicating the header logic.
Keyed helper entry points report the appropriate footer/header settings while
preserving the original public header API. Every explicit field and inactive
override validates even when disabled, with strict YYYY-MM-DD/leap-year/range
checks and redacted errors. Literal prefixes/suffixes do not expand number tokens.

Seven actual AlbumEasy 4.5 cases/11 pages were independently repeated with
identical full-page traces. Observed date 2026-11-03 is frozen for deterministic
native tests; fifteen source/PDF/date hashes are checked. Actual glyph/font/size
comparisons pass within 0.01 mm for all formats/positions, multiline TB16,
HN8/HN25, custom/empty separators, asymmetric even margins and all implemented
header/footer text/numbers/dates/title coexisting. Reference order is header
number, footer number, header date, footer date, plain header/footer, title.
Pink footer colour and hint/content guide boundaries are checked separately.

The footer's one-em plus 0.5 mm below-margin baseline is reused; multiline dates
grow downwards. Height hints extend reservation upwards without moving glyphs.
Date reservation constrains the content bottom before title layout. Date vs
footer text/number/title uses the shared nonblank per-line AFM checker; touching
boxes and reservation-only intersections are not collisions. Page-specific
errors name correction keys and a TODO records future UI choices. There is no
automatic movement or implemented UI. Invalid later-page dates create no new
PDF and preserve existing output. Signed padding border overlap remains
intentional; native rejects off-page descenders even if AlbumEasy accepts them.
The large-font reference uses safe 3 mm padding, not the clipping 5 mm setting.

Validation: **97 passed, 0 failed, 0 ignored** (86 native integration tests,
including ten new footer-date tests; one shared collision unit test; ten
main-config tests). Focused Clippy, formatting/scoped diff checks and all frozen
hashes pass. Windows GNU library/example compilation passes without optional
rfd; Windows runtime/default-feature build and printing remain unverified.
Mixed date sources, header/footer date consistency on every page, injected
year-boundary contexts, footer-only capture, calendar/redacted errors,
defaults/precedence/snapshots and all twelve fonts' exact-width checks pass.
Existing app warnings and MuPDF's unavailable-ICC warning remain.

The visually inspected three-page demo has black header date/text/numbers and
25 pt title, pink footer date left, text centre and numbered labels right.
Both fixed dates match on every page; header labels start at 8 and footer at 1.
The shared 25 mm footer hint leaves page-1 content bottom **36.0936 mm**;
content top after title is **265.1549 mm**. The ignored local config remains
three geometry-only pages with all text/number/date extensions disabled and
227 unapplied keys.

Branch `issue/73-native-pdf-footer-date`; maintainer approved the demo and
requested a local commit. All 97 relevant tests passed again before committing;
no merge/push requested.
Broad T005/T006/T007/T046 remain partial; production app/UI/backend, full
pagination/count convergence, custom fonts/wrapping and exact glyph-outline/
italic-overhang certification remain pending. No imports/databases/production
routes or historical diagram changes. Unrelated user manual/design/autosave
and the tagged album-globals test are untouched. Synthetic fixtures are copied,
no inputs moved/deleted; test tempdirs self-clean. Evidence/demo/reports/
isolated regenerated knowledge stay session-only.

#### Header generation date and deterministic date context (#72)

The maintainer approved header dates, all three formats and a generation-date
default with fixed-date override. Separate `ALBUM_PAGES_HEADER_DATE` command
state reuses measured header placement, colour, padding and minimum-height hints.
Explicit `[native_pdf.header_date]` defaults are disabled/HN/8 pt/L/YMD/"-"/
"Generated:" followed by a space/empty suffix/generation/2026-01-01 fixed override.
No rendering fallback values are supplied. All fields and the inactive override
are validated even when disabled; local user settings are preserved.

Generation mode uses local calendar date captured once per document outside the
page loop; fixed mode uses strict valid YYYY-MM-DD in years 0001–9999.
YMD/DMY/MDY pad month/day to two and year to four digits; separator and prefix/
suffix remain independent literal WinAnsi strings, empty allowed. Configuration
loading uses a separate preview date for validation, not a frozen rendering date.
`resolve_with_context` and the explicit-date proof entry point let callers use
the same date for layout/drawing/reporting. Legacy resolving refuses generation
mode without a context. The ordinary writer captures once; fixed mode ignores
supplied generation context. No silent local-to-UTC fallback is introduced.

Seven actual AlbumEasy 4.5 cases/11 pages were independently repeated with
identical complete page traces. Observed date 2026-11-03 is frozen separately;
15 hashes cover the source/PDF/date bundle. Native tests use that fixed date,
not today's clock. Every reference date/header/number/title glyph/font/size
matches within 0.01 mm for all formats, L/C/R/LR/RL, custom/empty separators,
multiline 16 pt Times Bold, 25 pt Helvetica, asymmetric even margins and
coexistence. Actual pink colour and measured hint/content guides are checked.
Reference ordering is header number, footer number, header date, plain header/
footer and title; date reservation applies before title and stamp content.

The shared AFM line-box check now includes date vs header text, date vs header
number and date vs title. Errors identify physical page/keys; code TODO records
future UI notification/correction. No automatic movement or UI is implemented.
Every page is preflighted before saving; an invalid page-2 date preserves
existing output and creates no new output. Signed padding border overlap is
still intentional; hint rectangles alone are not glyph collisions.

Validation: **87 passed, 0 failed, 0 ignored** (76 native integration tests
including ten new date tests, one shared collision unit test and ten main-config
tests). Focused Clippy, formatting and scoped diff checks pass. Windows GNU
library/example compilation passes without optional rfd; Windows runtime/
default-feature build/printing remain unverified. Calendar/leap-year/redacted
errors, immutable snapshots, explicit fixed/generation contexts across a year
boundary and all twelve built-in fonts' exact-width boundaries are covered.
Existing app warnings and MuPDF unavailable-ICC warning remain.

The three-page demo is visually checked: pink date left, "Collection" centre,
header numbers 8/9/10 right, black 25 pt title, independent footer labels 1/2/3.
The fixed date is the same on every page; a shared 25 mm hint leaves page-1
content top **244.3813 mm**. Ignored local config remains three geometry-only
pages, disabled text/date/numbers and 227 unapplied keys.

Branch `issue/72-native-pdf-header-date`; maintainer requested a scoped local
commit after validation. No merge/push requested.
Broad T005/T006/T007/T046 remain partial; footer dates, custom fonts/wrapping,
exact glyph-outline/italic-overhang certification, full pagination/count
convergence and production app/UI/backend remain pending. Source imports,
databases, staged user manual/design/autosave work and baseline diagram are
untouched. Synthetic fixtures are copied, no inputs moved/deleted; test tempdirs
self-clean, while evidence/demo/reports/isolated knowledge stay session-only.

#### Header page numbering and overlap diagnostics (#71)

The maintainer approved independent header numbering and checking other
implemented header values for overlaps. A separate `ALBUM_PAGES_HEADER_NUMBER`
module uses shared checked numbering and existing header measurement/placement.
Explicit `[native_pdf.header_number]` defaults are disabled/HN/8 pt/R/start1/
"Page "/" of $PAGES$". Header padding/hint/colour are shared explicit TOML policy;
the header policy table is required even if its text is disabled. No fallback
rendering values are added. Footer APIs/labels remain independent and unchanged.

Seven synthetic AlbumEasy 4.5 cases/14 pages were independently rendered twice
with identical complete page traces; fourteen input/PDF hashes are frozen.
Actual header-number glyph/font/size comparison passes within 0.01 mm for
L/C/R/LR/RL, multiline Times Bold, signed starts, repeated total tokens,
asymmetric even margins, 9-to-10 labels and header/title/footer coexistence.
Header numbers draw before footer numbers, then plain header/footer and title,
matching the reference order. Pink colour and hint guide boundaries are checked.

Shared line-box intersection replaces footer-only duplicate logic and checks
header text vs number, and each vs title. Blank lines/touching edges are
excluded; hint rectangles are not glyph boxes. Explicit errors name physical
page, conflicting components and TOML correction keys. Renderer preflights
every page before saving; a tested page-3 width collision preserves existing
output and creates no new output. A code TODO requires a future UI notification
and correction dialog. For now users edit TOML and rerun, not interactively
select an alternate layout. Signed padding border overlap remains intentional.
AFM boxes are not exact glyph/italic-overhang certification; date/image checks,
full stamp-pagination count convergence and production app/CLI/UI remain pending.
Broad T005/T006/T007/T046 remain partial.

Branch `issue/71-native-pdf-header-number`; maintainer requested a scoped local
commit after validation. No merge/push requested.
Unrelated user manual/design/evolving autosave work is untouched. No input
imports, database rows, production routes or historical diagram are changed.
Tests use self-cleaning tempdirs; frozen fixtures are copied synthetic inputs.
Validation evidence, proofs and isolated regenerated knowledge stay session-only.

Validation: **77 passed, 0 failed, 0 ignored** (66 native integration tests
including nine new header-number tests, one shared collision unit test, and
ten main-config tests). Focused Clippy/formatting/scoped diff checks and all
fourteen frozen hashes pass. Windows GNU library/example compilation passes
with optional rfd disabled; Windows runtime/default-feature build and printing
remain unverified. Existing app warnings and MuPDF's unavailable-ICC warning
remain. Early TDD missing-field and misplaced test-module compilation failures
were fixed before the complete green validation.

The visually inspected three-page demo uses pink 16 pt LR header numbering
starting at 8, centred 8 pt "Collection", black 25 pt title and independent
8 pt footer numbering starting at 1. Pages show "Page 8 of 3", "Page 9 of 3",
"Page 10 of 3" above "Sheet 1 of 3" through "Sheet 3 of 3". All top text is
separate; a shared 25 mm header hint leaves page-1 content top **244.3813 mm**.
The actual ignored local config retains three geometry-only pages with
text/numbering disabled and 227 unapplied keys; local colours are preserved.

#### Measured footer page numbering (#70)

The maintainer approved `ALBUM_PAGES_FOOTER_NUMBER` after committing #69 as
`55cb720c`. Its separate command module reuses footer placement/text measurement
and rendering. Existing `[footer].album_pages_footer_number_*` fields supply
enablement/font/size/position/start/before/after, with section-over-root
precedence. Explicit native opt-in defaults false to preserve prior proof
output despite the main template's existing true number-enabled field.
Shipped numeric style defaults are HN/8 pt/C/start1/"Page "/" of $PAGES$";
footer padding/hint/colour are shared explicit TOML values. Plain-footer
font/position/text remain independent. Local edits are preserved.

Reference verification: seven AlbumEasy 4.5 cases/15 pages independently
rendered twice with identical complete page traces. Starting labels apply from
physical page 1; zero/negative starts are supported. LR/RL follow physical
parity, not printed labels. Every exact `$PAGES$` occurrence expands to the
actual count, even with a non-1 start and multiple prefix/suffix tokens.
Omitted reference arguments are compared against explicit native TOML choices;
no runtime rendering defaults are reintroduced.

`resolve_with_page_count` requires a positive final PDF page count and a valid
physical page. Existing `resolve` refuses active numbering instead of guessing
a total. The loader samples first/second/last numbered pages; renderer
preflights all pages before saving. Checked signed 32-bit arithmetic prevents
label overflow. Number width is measured after expansion for each page.
Footer-number reservation uses the same resolved result as drawing.
Plain and numbered footer line-box overlaps produce actionable config errors.
Neither auto-pagination/count convergence nor full stamp-row engine is claimed.

Native comparisons verify every number/header/footer/title glyph within
0.01 mm, including signed labels, 9-to-10 width changes, asymmetric margins,
LR/RL and coexistence. Actual pink footer colour and guide top/bottom verify
TOML policy. Exact-width tests span all twelve built-in fonts.
The AFM line/advance and clipping/reservation safety limitations still apply;
custom fonts, wrapping, header numbers/dates and production app/CLI/UI remain
pending. Full T005/T006/T007/T046 tasks remain partial.

Branch `issue/70-native-pdf-footer-number`; no commit/merge/push yet.
Unrelated user manual/design/evolving autosave files remain untouched.
No source/input databases or production routing/baseline diagram are changed.

Validation: **67 passed, 0 failed, 0 ignored** (eight numbering tests,
49 preceding native tests and ten main-config tests). Targeted Clippy,
focused formatting/scoped diff checks and fourteen frozen hashes pass.
Windows GNU library/example compilation passes with optional rfd disabled;
Windows runtime/default-feature build and physical printing remain unverified.
Existing app warnings and MuPDF unavailable-ICC warning remain.

The three-page demo uses pink 16 pt LR numbering starting at 8, displaying
"Page 8 of 3", "Page 9 of 3", "Page 10 of 3", alongside an independent centred
8 pt "Collection" footer and black header/title. All three rasters were
visually inspected; numbering alternates left/right and no text overlaps.
Shared 25 mm footer hints leave content bottom **36.0936 mm** and title leaves
content top **271.5420 mm**. The actual ignored local config remains three
geometry-only pages, text/number extensions disabled, 227 unapplied keys.
Test tempdirs self-clean; copied synthetic fixtures preserve reference inputs;
proofs/logs/reports/checkpoint and regenerated knowledge remain session-only.
No database rows are inserted, updated or deleted.

#### Measured repeating footer and padding (#69)

The maintainer approved the text footer/padding slice after committing #68
as `ba03fae9`. Separate `ALBUM_PAGES_FOOTER` and `ALBUM_PAGES_FOOTER_PAD`
modules reuse shared standard-font measurement, positioned lines and drawing.
Horizontal/parity alignment is extracted into shared `TextPosition`, retaining
the existing `HeaderPosition` alias/public parser and header error keys.
Footer parsing uses its own keyed `native_pdf.footer.position` error.

All footer inputs have explicit editable TOML defaults: disabled/HN/8 pt/C/
"Stamp Album Footer"/0 mm/automatic; existing footer colour spelling defaults
to black in shipped templates. Section colour wins over root. Local config
receives only the footer table and preserves previous colour/settings.
The new text position is independent of existing footer-number position;
number/date keys remain unapplied rather than being silently repurposed.

Seven synthetic AlbumEasy 4.5 cases/nine pages were independently rendered
twice, with identical complete page traces. The first baseline is verified at
bottom margin minus one font-size step (converted to mm), minus 0.5 mm,
minus padding. Multiline text grows down; positive padding moves down away
from the border. Frozen cases include signed/omitted padding, 8/25 pt
Helvetica, multiline 16 pt Times Bold, asymmetric even margins and LR/RL,
plus simultaneous header/title. Fourteen source/PDF hashes are frozen.

**Intentional safety difference:** the initial 25 pt/5 mm padding probe put
descenders off the physical page while AlbumEasy reported success. Native
rejects it; the large-font frozen comparison instead uses 3 mm padding.
Minimum-height reservation is max(measured, hint); extra extends upwards
without moving glyphs. Normal footers do not consume content. Spill raises
its bottom boundary, alongside the header's top constraint before title layout.
Off-page text and unusable content are errors, preflighted before PDF saving.
AFM line/advance measurements do not certify exact glyph outlines or italic
side overhang. General stamp/row layout, page numbering/date expansion,
custom fonts, automatic wrapping and production app/CLI/UI integration
remain pending; full T005/T006/T007/T046 tasks are still partial.

Branch `issue/69-native-pdf-page-footer`; no commit/merge/push is authorized
for this slice. User manual/design/autosave changes stay untouched and staged
as they were; no baseline diagram copy is warranted by isolated proof work.

Validation: **59 passed, 0 failed, 0 ignored** (eight footer tests, 41 prior
native tests and ten main-config tests). Targeted Clippy, focused rustfmt and
scoped whitespace/hash checks pass with pre-existing application warnings.
Windows GNU library/example cross-compilation passes with optional rfd
disabled; Windows execution/default-feature builds and physical printing
remain unverified. MuPDF reports unavailable ICC support as before.

The saved two-page demo uses a 16 pt pink LR footer, 1 mm padding and a 25 mm
minimum hint, alongside a black header/title and green remaining-content guide.
Page 1 measured footer height is **5.2211 mm**; reservation/content bottom is
**31.6872 mm**; title leaves content top at **271.5420 mm**. Both page rasters
were visually inspected: odd footer left/even right, no visible clipping.
The actual local configuration still renders three geometry-only pages with
all text extensions disabled and reports 234 unapplied keys. No database/input
data is modified. Frozen references are copied; test tempdirs self-clean;
proofs, logs, reports and regenerated knowledge stay in the session workspace.

#### Measured repeating header and padding (#68)

The maintainer approved `ALBUM_PAGES_HEADER` and `ALBUM_PAGES_HEADER_PAD`.
Each owns a separate compiled command module. Header position supports L/C/R,
LR (odd-left/even-right physical pages), and RL (reverse). Header font, size,
text, signed padding and minimum-height hint are independent TOML values;
existing section-over-root position/colour spellings are reused. Shipped
defaults are disabled/HN/8 pt/"Stamp Album Header"/0 mm/automatic/C/black.
The local file receives only the native header table; existing choices remain.
Missing/invalid fields return keyed redacted errors without hidden defaults.

Seven isolated AlbumEasy 4.5 cases/nine pages verify 8/25 pt Helvetica,
multiline 16 pt Times Bold plus title, signed/omitted padding, and LR/RL
with asymmetric even margins. Independent renders matched complete page traces.
Fourteen source/PDF hashes are frozen in the new header-v1 fixture directory.

**Reference correction found by actual PDF testing:** the bottom baseline is
2 **mm**, not 2 PDF points, above the top margin boundary plus configured padding.
Multiline headers grow upwards; horizontal alignment uses margin bounds.
These are measured command semantics, not configurable policy defaults.
Native glyph checks now match each header font/size/position within 0.01 mm.
Title interaction retains previously verified coordinates.

Shared positioned text lines and drawing serve header and title. Normal headers
do not push content down. Native minimum hints extend reservation downwards
without moving glyphs; only spill into content lowers its boundary before title
layout. This is an explicit safety difference from AlbumEasy, not identical
pagination. Physical overflow, over-wide text and unusable content are errors.
All pages preflight before saving, including invalid later-page headers.
Standard-font AFM measurements remain line boxes, not exact glyph-outline/
italic-overhang certification. Automatic wrapping, custom fonts, header date/
number expansion, footer and general stamp/row layout remain unimplemented.

Branch `issue/68-native-pdf-page-header`; no production app/CLI/UI routing,
merge or push. Existing diagram baseline and unrelated manual/design/autosave
files are unchanged by this slice. Full roadmap text/row/integration items
remain partial; this is an isolated reusable command checkpoint.

Validation: **51 passed, 0 failed, 0 ignored** (eight header, 33 preceding
native tests and ten main-config tests). Targeted Clippy/focused formatting
and scoped diff checks pass with pre-existing application warnings only.
Windows GNU library/example cross-compilation passes with optional rfd disabled;
Windows runtime execution and physical printing remain unverified.
MuPDF reports unavailable ICC support; no physical-colour match is claimed.

The saved three-page demonstration uses a 16 pt LR header, 3 mm padding,
25 mm minimum reservation, and a separately configured title. Header measured
height is **5.2211 mm**, reservation bottom **253.4527 mm**, and remaining content
top after the title **246.1947 mm**. Its raster was visually inspected.
The actual local configuration still renders three geometry-only pages with
title/header disabled and reports 235 unapplied settings. No database rows or
source inputs are modified/moved; test tempdirs clean themselves up, references
are copied, and generated proofs/logs/knowledge stay in the session workspace.

#### Measured repeating page title (#67)

The maintainer approved `ALBUM_PAGES_TITLE` as the next slice, with measured text
height plus an optional TOML minimum-height hint to improve future stamp-layout
calculations. `minimum_height_mm = 0.0` means automatic; a positive value reserves
at least that height, never shrinks measured text, and never changes glyph size
or position. Following spacing remains independently configured.

`text.rs` measures standard-font advance widths and ascent/descent once.
`commands/page/album_pages_title.rs` owns the command and resolves positioned
lines plus measured/reserved heights and the next content boundary.
`PageSettings::resolve` returns the same title result used by the renderer and
reduces its usable content bounds. The diagnostic guide therefore represents
remaining room for future stamps; no stamp-row engine is implemented here.

Configuration reuses existing `[header].album_pages_title_font/size` and
`[colors].color_album_title`, including section-over-root precedence.
`[native_pdf.title]` supplies `enabled`, `text`, `vspace_after_mm` and
`minimum_height_mm`, all explicit editable TOML fields. Templates ship title
disabled, HN/10 pt/black, text "Stamp Album", zero extra gap and automatic height.
The ignored local file received only the new table; existing title font/size/
colour edits are preserved. Missing/invalid fields fail with keyed errors.
No native settings UI or production CLI/backend wiring is introduced.

Five isolated AlbumEasy 4.5 reference cases cover 8/25 pt Helvetica, two-line
16 pt Times Bold, omitted/zero gap equivalence and negative spacing. Six pages
were rendered twice; complete MuPDF page traces matched on repeat. Ten source/
PDF hashes are frozen in `tests/fixtures/albumeasy/title-v1/sha256.txt`.
Native comparisons check every title glyph's font, size and position within
0.01 mm, on two native pages per case, with both automatic and 25 mm hints.

**Intentional native safety difference:** title positions follow AlbumEasy, but
native content reservation includes the final font descent. The measured
AlbumEasy cursor stops at the last baseline; claiming identical remaining-page
space/pagination would be incorrect. The additional descent prevents following
stamp boxes from consuming the descender region under normal nonnegative gaps.
Negative following gaps explicitly permit overlap. AFM advance/line metrics do
not certify exact glyph-outline bounds/italic side overhang.

Scope: 12 standard AlbumEasy font tokens, WinAnsi text, explicit multiline
titles. Custom fonts, unsupported Unicode/shaping, automatic wrapping and
over-wide titles fail rather than silently substitute/clip. Full text/row layout,
information pages, source-command parsing/export and production integration
remain pending. Shared measurement state is reusable by later text/stamp slices.
The existing application layout engine and baseline draw.io are unchanged.
Branch `issue/67-native-pdf-page-title` carries the uncommitted #66 foundation;
the maintainer subsequently authorized committing both slices together before
starting another command. Commit-gate validation reran all 33 native tests and
ten main-config tests: **43 passed, 0 failed, 0 ignored**. The checkpoint excludes
the ignored local configuration and unrelated manual/design/autosave files.
No merge or push is authorized.

Validation: **43 passed, 0 failed, 0 ignored**: seven title tests, 26 prior native
tests and ten existing main-config tests. Targeted Clippy and focused formatting
passed (existing application warnings remain); Windows GNU library/example
cross-compilation passed with optional rfd disabled. Windows execution and
physical print are unverified. Reference repeats matched complete page traces.
MuPDF still reports unavailable ICC support; no physical colour claim.

The runnable demo uses a 25 pt title measured at **10.6451 mm**, a **25 mm**
minimum reservation, independent **3 mm** following space and a green remaining
content guide at **249.8 mm** from the page bottom. Title positions stay unchanged
when the hint grows. Its raster was visually inspected. The local config's
default-disabled title preserves the three-page geometry-only proof and now
reports 237 unapplied keys rather than 240 (font/size/title colour are consumed).
Test tempdirs self-clean; references are copied, not moved; no source inputs or
databases are modified. Existing unrelated manual/autosave edits are preserved.

#### Explicit TOML policy retrofit (#66)

Maintainer clarification: defaults belong **in TOML**, editable as in the
regular application, not hidden in code. All implemented values have explicit
template entries; border/guide colours default to black. Missing/empty/invalid
required values must become user-visible settings errors when the UI is added.
The native loader now contains a TODO for that future UI behavior; no UI
implementation is claimed.

The approved follow-up implements independent border colours under
`[native_pdf.border_colours]`: `outer`, `inner1`, `inner2`, all default black
in both shipped examples and the local native config. All three are required
when the table is present; absent table uses the explicit legacy shared TOML
colour. Per-line values take precedence. Existing local shared-colour edits
are preserved, but the new table deliberately controls native border colours.
Disabled slots keep their identities, so disabling outer/inner1 cannot move
another colour to the wrong stroke. This is a native extension; AlbumEasy's
shared `COLOUR_ALBUM_BORDER` command and frozen compatibility tests are unchanged.

The maintainer requires rendering settings/defaults to originate in TOML;
stamp/layout records may supply item-specific data. This applies to every future
native command, not just this proof. Mathematical/unit constants, command syntax
semantics and reference-test values remain code.

The native adapter now requires all four border arguments and border colour.
Zero border widths explicitly select no border; missing colour is no longer
silently black. `PageSettings::new` requires explicit border/colour and their
types no longer implement `Default`. Command modules remain independent of files.
The optional even-margin table still selects explicit overrides; otherwise
pages use the base margins from TOML, never numeric fallback margins.

`proof.rs` owns diagnostic-only typed settings, separate from album command
state. `[native_pdf.proof]` requires `page_count`, `guide_enabled`,
`guide_width_mm` and `guide_colour`; width/colour remain required when disabled.
The example passes loaded settings to the renderer rather than literal page
counts, guide widths or colours. Both shipped templates and the existing ignored
local configuration contain explicit values preserving previous geometry.
Border defaults are now black as explicitly requested, instead of darkblue.
No new main `Config`, application CLI/UI or production routing is introduced.

New no-fallback tests exercise missing/invalid settings and serialized PDF
output: two pages, pink borders, independent green 0.35 mm guides, and disabled
guides. Existing frozen size/margins/spacing/border/colour comparisons retain
explicit reference choices in test-only inputs; no golden fixture is changed.
Branch: `issue/66-native-pdf-explicit-toml`. Historical checkpoints below retain
their original omitted-default behavior descriptions; this contract supersedes
those descriptions.

Validation: **36 passed, 0 failed, 0 ignored** (26 native tests, including actual
MuPDF trace checks, plus 10 existing main-config tests). The local-config example,
focused formatting, Clippy and Windows GNU library/example cross-check passed.
Workspace-wide formatting still reports pre-existing differences in
`tests/album_stamp_img_integration_test.rs`; that unrelated file is unchanged.
Existing application warnings and MuPDF's unavailable-ICC warning remain.
Windows execution/default-feature build and physical printing are unverified.
Test tempdirs were cleaned, no source inputs moved/deleted, and no databases
were accessed. The ignored local config received the new proof and per-line colour tables;
existing user values, main config and credentials were preserved. No new
dependency, fixture, commit, merge or push belongs to this retrofit checkpoint.

#### Commit checkpoint (#65)

The maintainer requested two commits: fix the acceptance regression first, then
commit the accumulated native border/configuration/colour work. The regression
was a stale assertion, not a renderer failure: `render_pair` intentionally
reuses `build/reconstructed/Brazil.pdf`. The test now captures original PDF
bytes before editing and checks changed contents, retaining all raster checks.
Commit `333c398b` records this fix. The repository pre-commit hook also staged
the pending direct `svgtypes` declaration from Cargo.toml in that commit.

All **26 external-backend tests**, including every opt-in test, passed with the
real AlbumEasy launcher. Reference/reconstructed output had **85 identical
raster pages**; the edit changed page 1 and preserved all 84 others. Original
source checksums and placement/database invariants passed. The previous failure
records below are historical and no longer block the checkpoint.

Current branch is `fix/native-pdf-commit-checkpoint`. The second commit captures
#62/#63/#64 together, excluding the ignored user `config-pdf.toml` and unrelated
autosave. No push or production integration is authorized by this checkpoint.

The final goal is to **replace external AlbumEasy PDF generation with internal
PDF generation**. The maintainer now authorizes small vertical slices: verify a
command's semantics, implement a reusable compiled module, compare with frozen
fixtures, validate/commit/merge locally, then stop for review. One file per
AlbumEasy command, grouped by command family, is the approved structure.
This supersedes fixture-only work for approved slices, not all remaining gates.
Production backend/CLI/UI changes remain unapproved in this increment. The
maintainer has separately approved a local `config-pdf.toml` bridge for the
isolated native proof, without changing the main `config.toml` loader.
The integration branch is `development/native-pdf`, created from
`docs/albumeasy-421-command-inventory`. Phase 1 epic is GitHub #55; the bounded
T003 sub-issue is #56, branch `issue/56-t003-baseline-drift-checker`.
The maintainer reports visual and 100%-scale print acceptance of core-v1;
Windows execution remains unverified.

The next approved baby step (#58) verifies `ALBUM_PAGES_SIZE` only: installed-help
semantics, existing configuration mapping and an isolated
[page-size-v1 fixture bundle/report](../testing/ALBUMEASY_PAGE_SIZE_V1.md).
Four valid dimensions and two explicit failure cases were measured without
renderer/application changes. The findings include AlbumEasy's zero exit code on
semantic errors and the current emitter's one-decimal precision mismatch with
fractional configuration/layout values. Neither issue is silently fixed here.

The subsequent margins-only increment (#59) adds
[margins-v1 evidence](../testing/ALBUMEASY_MARGINS_V1.md): symmetric/asymmetric,
fractional and explicit even-page-override references plus two failure cases.
Eight page borders were measured, including the half-stroke inset from margins.
No automatic even-page mirroring was observed without MARGINSE. These command
bundles remain measured specifications, not part of the core-specific test runner.

The spacing-only increment (#60) adds
[spacing-v1 evidence](../testing/ALBUMEASY_SPACING_V1.md): zero, fractional
horizontal-only/vertical-only and combined probes, plus two failure cases.
Measured text/frame deltas isolate both axes. Misleading config descriptions,
default discrepancies and layout-accounting differences are recorded without
changing configuration, application or renderer behavior.

The first compiled slice (#61) implements page size, base/even-page margins and
spacing under `src/native_pdf/commands/page/`, with shared `units.rs`,
`document.rs` and a small proof renderer. Commands validate already-numeric
arguments; there is no second production TXT tokenizer/importer. Required
settings are explicit; no global config is read or silently defaulted.

```text
src/native_pdf/
  mod.rs
  units.rs
  colour.rs
  document.rs
  renderer.rs
  commands/
    mod.rs
    colour/
      mod.rs
      colour_album_border.rs
    page/
      mod.rs
      album_pages_size.rs
      album_pages_margins.rs
      album_pages_marginse.rs
      album_pages_spacing.rs
      album_pages_border.rs
```

The shared native folder also contains `config.rs`, a per-run configuration
adapter, not an AlbumEasy command module.

Use the [native proof instructions](../guides/ALBUMEASY_BASELINE_MANUAL_CLI.md#native-rust-page-settings-proof).
The example renders three geometry-only pages (margin border/content guide);
text, rows, images, decorative borders, other colour commands, TXT export and
application wiring are not implemented. Even-page selection uses one-based physical page numbers;
custom footer/parity semantics remain unverified. Vertical spacing is
inter-element state, not an implicit top/bottom inset.

T004/T005/T006/T007 have partial evidence only; none is declared complete.
Do not treat six command modules as complete command-language coverage.

#### #64 page-border colour and configuration

The next compiled command is `COLOUR_ALBUM_BORDER` (including the
`COLOR_ALBUM_BORDER` spelling alias), in its own file under `commands/colour/`.
A small shared `colour.rs` represents opaque SVG named RGB colours using the
existing transitive `svgtypes` parser, now declared directly. No new package or
version is introduced. Defaults are explicitly black when the command/config
key is absent, matching the reference renderer.

Six valid references (seven pages, 21 border strokes) and six failure inputs
live in `tests/fixtures/albumeasy/border-colour-v1/`. Each valid case has an
independently repeated identical page trace. Cases verify black default,
case-insensitive darkblue, green via alias, lightgrey, white and a final navy
command after page starts. AlbumEasy applies the last border colour globally,
including preceding pages. A future native TXT interpreter must retain that
global behavior; the current test-only fixture adapter is not a new importer.

**Compatibility discovery:** AlbumEasy 4.5 rejected bare hex as a comment/
missing-parenthesis error and quoted hex as an invalid colour name. Native
code therefore supports opaque named colours only for this command and rejects
hex/RGB functions/transparency explicitly. Previous template hex-support claims
are corrected, without changing production TXT emission. A future hex extension
would need an explicit compatibility policy rather than silent divergent output.

The existing `[colors].color_album_border` now reaches native strokes through
the same snapshot; `[colors]` overrides a legacy root key with reporting.
The local `config-pdf.toml` value remains `"darkblue"` and now visibly affects
output. Both templates/inventory/manual are updated. Other colour keys remain
unapplied; the current full template now has 240 unapplied settings.
The diagnostic guide is explicitly reset to black, including when borders are
disabled. Colour does not alter any geometry or reserve additional space.

Four new tests verify frozen identities/text, command negatives, config
precedence/redaction, rendered RGB and unchanged path geometry. Native RGB is
serialized by oxidize-pdf to three decimals: use an explicit 0.00051 component
tolerance (less than one 8-bit channel step), while parsed values match the
reference to 0.00001 and geometry retains 0.01 mm. These are DeviceRGB checks,
not ICC/display/print colour certification. Tests and execution details are
recorded in the [border report follow-up](../testing/ALBUMEASY_BORDER_V1.md#64-native-border-colour-follow-up).
Full roadmap tasks remain open. No production config/CLI/UI/backend routing
changes or diagram changes are introduced.

Branch `issue/64-native-border-colour` carries the prior uncommitted #62/#63
dependencies. The earlier opt-in Brazil path assertion still blocks committing/
merging; no unrelated fix or push is made.

#### #63 explicit user-config bridge

The maintainer requested introducing configuration now, copied from the template
as local `config-pdf.toml`, eventually renamed/reused as `config.toml`.
`src/native_pdf/config.rs` now loads an explicit `Path` into the compiled command
types. This completes the bounded bridge, not full T046 or backend integration.

- Current working branch: `issue/63-native-pdf-config`, carrying the uncommitted
  #62 border dependency. No commit/merge/push; the prior unrelated opt-in
  Brazil assertion still blocks the commit quality gate.
- `config-pdf.toml` was copied from the template, remains Git-ignored and is
  intentionally not embedded/committed. Templates remain the shipped reference.
  The user's main `config.toml`, credentials, albums and autosave are untouched.
- Required size, base margins and spacing use existing names; border is an
  optional all-or-none group. `[page]` overrides stale root keys just as in the
  current app. Invalid winning values fail. Values remain unrounded `f64` mm.
- Optional native-only `[page.even_margins]` requires `left_mm`, `right_mm`,
  `top_mm`, `bottom_mm`; no table means no mirroring. Both page parities and
  stroke footprints are validated at load. The main `Config` struct is not
  extended until production integration is approved.
- No global searches, implicit defaults, environment/album overlays or imported
  source merging occur. This avoids coupling to the current loader's `f32`
  conversions and fallback/logging behavior. No alternative production importer
  is introduced. Parsed state goes directly to existing command/renderer APIs.
- Every unconsumed key is returned in a sorted `unapplied_keys` list; overridden
  root names are returned separately. The example warns that only page geometry
  is applied (241 other template settings currently unsupported). Neither
  parse diagnostics nor reports expose configuration values/source snippets.
- The example accepts `<new-output.pdf> [config-path]`; its default is exactly
  `config-pdf.toml` in the working directory. Editing/reloading changes output;
  an existing job retains its snapshot. Output overwrite refusal is preserved.
- Both templates document the native-only even-margin table. Spacing comments
  now describe measured within-page behavior; existing numeric defaults remain.
  See the [manual guide](../guides/ALBUMEASY_BASELINE_MANUAL_CLI.md) and
  [parameter inventory](../guides/CONFIG_PARAMETERS_INVENTORY.md).

Verification: **7 configuration + 10 native command + 6 core tests passed**,
including actual configured stroke coordinates/widths on three pages to 0.01 mm.
The config tests load both templates, cover section/root precedence, per-run
snapshots/user edits, missing/malformed/type/numeric/geometry errors, redacted
diagnostics and incomplete optional groups. Template output and an explicit
custom path containing spaces were executed manually; invalid config creates no
PDF and repeated output is refused without changing its SHA256.
Existing main-config unit tests: **10 passed**; external-album default suite:
**23 passed, 3 unrelated opt-in tests ignored**. Total: **56 passed** across
the focused native/core and existing suites.

Exact focused commands:

```bash
ALBUMEASY_BASELINE_CANDIDATE=/home/ray/.copilot/session-state/80e32404-199a-4106-9649-659ef9662bab/files/t003-core-verification-6RHMrG/first/album.pdf cargo test --test native_pdf_config --test native_pdf_page_settings --test native_pdf_page_border --test albumeasy_baseline -- --include-ignored --nocapture
cargo clippy --test native_pdf_config --test native_pdf_page_settings --test native_pdf_page_border --example native_pdf_page_settings
cargo check --lib --example native_pdf_page_settings --target x86_64-pc-windows-gnu --no-default-features
cargo test --lib config::tests -- --nocapture
cargo test --test external_album_backend -- --nocapture
```

Clippy and Windows GNU cross-compilation pass with existing application warnings.
Windows execution/default-feature build and physical printing remain unverified.
Detailed commands/logs, test impacts and traceability remain in session
`files/native-config/`; the prior #62 opt-in failure is not suppressed or patched.
No database migration, ingest, country processing, production runtime flow or
diagram change is introduced. Traceability: partial REQ-001/006/007/008/009/014,
T005/T006/T007/T046; all full tasks stay open. The next command should consume this
same explicit snapshot rather than independently reread global configuration.

#### #62 border verification and implementation record

The next compiled slice implements numeric `ALBUM_PAGES_BORDER` in its own
command file, replacing the proof's hardcoded border with explicit border state.
The optional command defaults to no border; existing page-settings tests now
explicitly request their original 0.2 mm border. The example demonstrates a
triple border. The isolated resolver now obtains occupancy from border state
instead of accepting an unrelated scalar. No production caller is affected.

The [border-v1 report](../testing/ALBUMEASY_BORDER_V1.md) records ten frozen,
independently repeated references, fifteen measured strokes, ten text anchors
and a missing-argument failure. It captures the legacy three-argument overload,
disabled strokes and AlbumEasy's distinct drawing-slot/content-gap behavior.
Native paths and content guides match to 0.01 mm. Four new tests and six existing
native tests pass, as do all six core tests and 23 default external-album tests.
Clippy and Windows GNU library/example compilation pass with existing warnings.
The report includes commands, filesystem/database impact and the error inventory.

Branch `issue/62-native-page-border` is intentionally **not committed or merged**:
an additionally enabled, unrelated real-Brazil opt-in test fails at its existing
generated-PDF-path inequality assertion after comparing 85 identical reference/
reconstructed raster pages. Its initial missing `ALBUMEASY_TEST_CLI` environment
was corrected before retrying. No unrelated code/test was changed to force green.
The commit quality gate therefore remains blocked. No push occurred.

Traceability remains partial T004/T005/T006/T007 and REQ-001/002/006/007/009;
colour/decorative borders, mid-document state transitions, TXT grammar,
production configuration integration and Windows runtime/printing remain open.
The historical #61 record below describes the earlier fixed-border proof.

#### #61 verification and implementation record

- Branch: `issue/61-native-page-settings`; integration: `development/native-pdf`.
- Traceability: REQ-001/shared settings, REQ-002/native generation,
  REQ-006/physical dimensions, REQ-007/errors and REQ-009/portability;
  partial T004/T005/T006/T007 only.
- Canonical geometry is unrounded `f64` millimetres, PDF bottom-left origin;
  conversion is `mm * (72 / 25.4)`. Preserve explicit fractional values and
  historical config defaults rather than normalizing either silently.
- A resolved page contains margin/content bounds; horizontal spacing insets both
  content edges. Vertical spacing advances between elements. Border occupancy
  is explicit; the proof uses a fixed single 0.2 mm border plus a diagnostic
  0.1 mm guide, not a command-language border implementation.
- Required size/margins/spacing are constructor inputs. Missing/incorrect
  arguments, non-finite/negative values, zero size, conversion overflow,
  invalid page number and unusable area return explicit errors. Even-page
  settings are distinct. All pages are validated before output writing.
- Scope excludes TXT parsing, persistence/provenance, global config snapshots,
  text/fonts/images/row layout, automatic pagination, compatibility export,
  full border semantics and production routing.

Exact Linux acceptance commands (each final exit code 0):

```bash
ALBUMEASY_BASELINE_CANDIDATE=/home/ray/.copilot/session-state/80e32404-199a-4106-9649-659ef9662bab/files/t003-core-verification-6RHMrG/first/album.pdf cargo test --test native_pdf_page_settings --test albumeasy_baseline -- --include-ignored --nocapture
cargo test --test external_album_backend -- --nocapture
cargo clippy --test native_pdf_page_settings --example native_pdf_page_settings
cargo check --lib --example native_pdf_page_settings --target x86_64-pc-windows-gnu --no-default-features
cargo run --example native_pdf_page_settings -- /home/ray/.copilot/session-state/80e32404-199a-4106-9649-659ef9662bab/files/native-page-settings-first.pdf
```

Results: **6 native + 6 core tests passed**, no skips in that acceptance run;
**23 existing external-album tests passed**, three unrelated opt-in Brazil
tests ignored. Four native sizes are read back using `oxidize-pdf` and compared
with actual AlbumEasy reference MediaBoxes. Margin/even-page layouts and spacing
state use the frozen JSON expectations. Independent MuPDF inspection checks the
actual native margin/content strokes, widths and page transforms across eight
valid margin/spacing cases (12 pages, two rectangles each), within **0.01 mm**.
The extra guide intentionally differs from the reference; strict identical
whole-page traces are not claimed for native proofs.

Negative controls cover all six existing missing-command/argument inputs,
invalid numeric values, unusable even-page geometry, zero page count and output
write failure. The invalid second-page test verifies no partial PDF is written.
Initial TDD compilation failed with E0433 (native_pdf absent) before implementation;
final checks passed. Clippy reports only existing application warnings.
MuPDF still warns `ICC support is not available`; no color-profile claim.

Filesystem impact: new focused Rust command/shared modules, integration tests
and an example; library exports the module but does not call it from app flows.
Tests create self-cleaning temporary PDFs/one invalid file; manual proof/logs
persist only in session files. No frozen fixture/reference, input, production
album, database or local config moved/deleted/changed. No ingest/schema operation
occurred. The single example is explicitly tracked despite the repository's
general `examples/` ignore rule; no ignore policy is changed.

Windows GNU library/example cross-check passed with optional rfd disabled;
Windows execution, default-feature Windows build, native print and font/image
fidelity remain unverified. Portable Path APIs are used without platform
branches. Application flow is unchanged, so no baseline draw.io copy is needed.
Stop for review after this local checkpoint.

Start with the supplied [AlbumEasy 4.2.1 manual](../AlbumEasy_4.2.1.pdf), not just
the commands this application currently emits. The new
[command inventory](../guides/ALBUMEASY_4_2_1_COMMAND_INVENTORY.md) identifies
**92 canonical names**, including **9 obsolete commands** and **11 commands in
the separate decorative-border language**. Twelve documented `COLOR_` aliases
bring the total to **104 spellings**. These counts describe the manual, not
implemented support.

The [application/configuration gap analysis](ALBUMEASY_4_2_1_APPLICATION_GAPS.md)
adds a static matrix for all 92 names: 24 occur in imported-command validation,
including six placeholders; 68 are absent. It also maps existing global and
album-local `config.toml` behavior to command families and flags generator/manual
syntax mismatches. No native album renderer is wired in the inspected paths.
Do not treat these name counts as support percentages.

For each subsequent implementation slice:

1. Review the inventory and resolve its documented syntax/behavior gaps.
2. Define per-command arguments, defaults, state, units, assets and interactions.
3. Audit application support across parsing, layout, native rendering, editing
   and export; do not count accepted names/placeholders as implemented behavior.
4. Define conformance examples and identify missing PDF-library capabilities.
5. Implement a small compiled command module and shared layout action, then test
   the generated PDF against the reference before moving to the next item.
   Record gaps explicitly; do not silently ignore unsupported commands.

The existing phase/task IDs below are retained for continuity, not treated as an
approved execution queue. T036's inventory work is pulled forward into this
checkpoint; full command semantics still need planning before the model is fixed.
T001 and T036 remain unchecked because the detailed matrix/audit is incomplete.
The primary target is now installed **AlbumEasy 4.5.0**, with 4.2.1 retained as
legacy coverage. The [4.5.0 reference update](../guides/ALBUMEASY_4_5_0_REFERENCE_UPDATE.md)
records new command families and corrects version-related gap findings using
the installed help archive. A full 4.5.0 coverage matrix remains to be completed;
the existing 92-name denominator is legacy-only. Prior codebase observations and effort
estimates below require reconfirmation before implementation.

## 1. Goal and recommended direction

Generate enhanced printable stamp-album PDFs directly in Rust using
`oxidize-pdf`, then export editable AlbumEasy-compatible TXT as a companion.
Native capabilities must not be restricted to features AlbumEasy can reproduce.

Use one authoritative editable document and shared layout engine. Native PDF
uses the full document. TXT export uses an explicit compatibility projection
when the target command language cannot express a native feature:

```text
Country data + configuration        AlbumEasy TXT + dependencies
              |                                |
       Document builder             Versioned command interpreter
              \                                /
          Authoritative editable album document + assets
                              |
                  Shared layout and font metrics
                              |
                Enhanced native PDF (primary output)
                              |
        Compatibility projection from document/layout, NOT PDF
                              |
           AlbumEasy TXT + assets + divergence report

Transition only: unsupported imports remain explicit drafts or use
the user-selected external AlbumEasy path; never silently drop commands.
```

Generate the native PDF first, then export TXT from the same frozen document
revision during the same operation. The PDF is not parsed to reconstruct TXT.
Retain explicit TXT-only generation. An approved compatibility adaptation can
have different appearance or pagination; it must not modify the source document
or the native PDF. Reuse shared layout rules for any adapted layout rather than
adding an unrelated second pagination engine.

**Do not replace AlbumEasy immediately.** Initially keep AlbumEasy as the default.
Offer native PDF as an opt-in backend for ordinary generated albums. Promote it
after the ordinary-album gates pass, independently of the later full-import gate.
The eventual goal is native creation and AlbumEasy import without installation
of AlbumEasy. The external program may remain an optional legacy renderer and
development comparison oracle.

### First enhancement: icons on either side of the main title

Support independent left/right icons, or both together, with explicit asset
selection, physical dimensions, aspect ratio, gaps and vertical alignment.
Keep the title centered in a defined title area; reserve sufficient heading
height and prevent overlap with text, margins or the page border. Missing assets,
oversized icons and long/wrapped titles require actionable validation.
Persist the settings in the authoritative album document; expose them in the
appropriate configuration/editor UI.

Determine whether the selected AlbumEasy version can reproduce each title
composition. If not, offer a clearly reported approximation or omission.
Do not assume icons are inherently unsupported, rasterize a whole page silently,
or constrain native layout to achieve cosmetic parity.
The supplied manual already demonstrates title-side images using composed text,
spacing and frameless-image commands (pp. 61–62), and reusable page-start groups
(pp. 23–24). Treat native title-icon controls as a usability/layout enhancement,
not automatically as an AlbumEasy-incompatible feature.

Other enhancements are a backlog, not approved implementation scope: decorative
headers/footers, richer typography, section badges, backgrounds and flexible
image/text compositions. Each needs its own acceptance criteria and export policy.

## 2. Previously inspected starting point — reverify before implementation

| Area | Current implementation | Consequence for this plan |
|------|------------------------|---------------------------|
| PDF library | `oxidize-pdf` is included in [Cargo.toml](../../Cargo.toml); [PDF extraction](../../src/review_ui/pdf_parser/extraction.rs) uses its image operations. | Inclusion/image extraction does not prove that generation, fonts and layout meet our needs; verify these independently. |
| Output orchestration | [album_globals/mod.rs](../../src/album_globals/mod.rs) creates TXT and invokes AlbumEasy to obtain PDF. | This is the main dispatch/result integration point. |
| Ordinary album generation | [generator.rs](../../src/album_globals/generator.rs) remains the active implementation; the entry point in [generator_new/mod.rs](../../src/album_globals/generator_new/mod.rs) is still a stub/fallback. | Do not assume the refactored entry point already provides a complete layout model. Extract behavior from the active implementation. |
| Existing layout helpers | [generator_new/](../../src/album_globals/generator_new/) contains layout, emission, commemorative-layout, helper and postprocessing modules. | Reuse suitable helpers; do not create a competing second layout subsystem. |
| Imported albums | [external_album.rs](../../src/external_album.rs) regenerates from database-backed commands and has its own rendering/approval lifecycle. | Preserve source references, identities, editing and approval; replace the external rendering dependency in a later gated phase. |
| Command validation | [external_album/parser.rs](../../src/external_album/parser.rs) uses a command allowlist; some accepted commands have placeholder argument rules. | Recognition is not implementation. Full support requires grammar, semantics, editing, rendering and export evidence for every command. |
| Settings | [config.rs](../../src/config.rs) supports AlbumEasy executable, automatic PDF generation and diagnostics. | Preserve existing configuration semantics and introduce backend selection additively. |

This is an implementation plan, not a claim that the library's rendering
capabilities or output fidelity have been certified.

## 3. Requirements and boundaries

| ID | Requirement |
|----|-------------|
| REQ-001 | Maintain an authoritative editable document and shared layout for native generation and compatibility projection. |
| REQ-002 | Render ordinary albums to PDF directly with native Rust code. |
| REQ-003 | After native PDF creation, export compatible TXT from the same document revision, with explicit divergence reporting; preserve TXT-only generation. |
| REQ-004 | Add explicit backend selection without changing existing defaults during rollout. |
| REQ-005 | Preserve imported-album source/layout/editing fidelity during transition and ultimately import, edit and render without the external program. |
| REQ-006 | Preserve print geometry and source semantics for the common feature subset; separately approve enhanced-export differences. |
| REQ-007 | Surface unsupported features and output failures; never silently change backend or return a stale PDF as success. |
| REQ-008 | Keep CLI, startup UI, country review UI, configuration and diagnostics consistent. |
| REQ-009 | Maintain Windows/Linux portability and validate both supported code paths. |
| REQ-010 | Support configurable main-title icons on the left, right or both sides without clipping or overlap. |
| REQ-011 | Support the installed AlbumEasy 4.5.0 command reference as the primary target, retaining documented 4.2.1 legacy coverage, including variants, preprocessing, border language, defaults and interactions across import, validation, editing, native rendering and export. Resolve documented ambiguities before claiming complete support. |
| REQ-012 | Produce per-feature/page/source-location export compatibility findings; never silently omit or approximate native enhancements. |
| REQ-013 | Preserve the full editable native document and assets so lossy TXT export/reimport cannot destroy native-only features. |
| REQ-014 | Preserve and document effective configuration semantics: existing sectioned/legacy keys, defaults, units, album overrides and per-run settings; resolve one snapshot for both outputs and explicitly reconcile imported commands with user overrides. |

### Scope boundaries and staged exclusions

- A complete versioned AlbumEasy interpreter is now an eventual deliverable,
  not part of the first ordinary-album release.
- The compatibility baseline means the complete documented album command language,
  parameters, defaults, supported legacy aliases and file/dependency semantics
  for the installed 4.5.0 reference with 4.2.1 legacy coverage. Inventory examples and undocumented behavior
  separately; do not make an unbounded claim about every historical version.
- No recreation of unrelated AlbumEasy desktop UI or undocumented internals.
- No changes to StampWorld/Colnect import semantics.
- No unrelated generator cleanup, database redesign or layout-editor rewrite.
- No OCR, PDF-to-TXT reconstruction, or new cloud service.
- No promise of byte-identical PDFs; compare observable layout and print output.

### Proposed backend policy — approve before coding

Use a typed setting such as `pdf_backend = "albumeasy" | "native"`.
The exact key/CLI naming must follow the project's current configuration patterns.

| Album/action | AlbumEasy selected | Native selected |
|--------------|-------------------|-----------------|
| Ordinary album, PDF requested | Existing TXT + AlbumEasy PDF; warn about unrepresentable enhancements | Enhanced native PDF, then compatibility TXT and report |
| Ordinary album, TXT only | TXT; no PDF | TXT; no PDF |
| Imported album, transition | Existing saved-layout pipeline | Preflight command coverage; native only for certified input, otherwise explicit blocker/draft or user-selected legacy renderer. |
| Imported album, full-support milestone | Optional legacy renderer | Import/edit/render/export natively without AlbumEasy installed. |
| Native capability/render failure | Not applicable | Return a visible error. Do not automatically invoke AlbumEasy. |

Backend choice and PDF enablement are separate settings. Keep existing
`albumeasy_auto_generate_pdf` behavior until a reviewed compatibility migration is
defined; changing a backend must not unexpectedly turn PDF generation on.
An imported-album exception must be visible in the UI/result, not a silent fallback.
During transition, uncertified imports may still require AlbumEasy. At the final
milestone no import/render/approval/edit action may require or launch it. Offline
source-to-document semantic validation and user-visible fidelity approval replace
mandatory two-render comparison; external comparisons remain development evidence,
not a runtime requirement.

### Compatibility export contract

| Classification | Required behavior |
|----------------|-------------------|
| Exact/compatible | Emit supported AlbumEasy commands and preserve meaning within certified tolerances. |
| Approximated | Offer a documented mapping and identify affected features/pages and expected differences. |
| Omitted | Explicitly identify native-only content excluded from TXT; require an approved policy. |
| Blocked | If no approved safe mapping exists, fail TXT export visibly; native PDF can remain a separately successful artifact. |

Default to strict export until an explicit approximation/omission policy is
approved. A saved album policy may avoid repeated prompts, but every run still
reports applied losses. Do not silently flatten text or pages into images.
Rasterized feature export is a future opt-in option only if readability,
editability, asset licensing and print quality are addressed.

Report source document revision, target language version, command/feature,
page/element or source location, action, reason and severity. Show summary in
CLI/UI and persist a companion report (format/path to be agreed). Keep native
document/database plus assets authoritative; TXT alone is not a lossless backup.

## 4. Work breakdown and implementation sequence

Task status markers are maintained against each task's complete acceptance
criteria: `[x]` means complete and verified; `[~]` means partially implemented
or evidenced with material acceptance work still open; `[ ]` means no
task-level deliverable is currently credited. A bounded command slice or
passing focused tests do not complete a broader task unless all its criteria
are met. Reconcile these markers after each implementation slice.
IDs remain stable as the document is updated.
Suggested new files below are proposals, not an instruction to add unnecessary
abstractions. Reuse existing modules wherever they already own the behavior.

### Phase 1 — inventory, baseline and capability proof

**Plan P1.1:** Freeze observable behavior and pin the full target language before altering the generator.
**Plan P1.2:** Prove that `oxidize-pdf` can render the minimum required album page.
**Plan P1.3:** Audit configuration semantics and approve a backwards-compatible native mapping (REQ-014).

- [~] T001 [Plan:1.1] Complete the command-first audit using the installed 4.5.0 help, `docs/guides/ALBUMEASY_4_5_0_REFERENCE_UPDATE.md` and the retained 4.2.1 inventory/manual. Inventory emitted commands/current layout/configuration in `src/album_globals/generator.rs`, `generator_new/emit.rs`, `generator_new/layout.rs`, `generator_new/commemorative_layout.rs`, and `generator_new/postprocess.rs`; record versioned support evidence, missing behaviors and unresolved gaps. Legacy name inventory, a 4.5.0 stamp-family matrix and version corrections exist; the full modern command/configuration audit remains open.
- [~] T002 [Plan:1.1] Inventory all ordinary/filtered generation callers in `src/album_globals/mod.rs`, `src/modes.rs`, `src/review_ui/http_handlers.rs`, and generation helpers. Record imported-album dispatch separately from ordinary albums. Starting paths and dispatch distinctions are documented; a complete caller/flow inventory is still outstanding.
- [~] T003 [Plan:1.1] Capture deterministic baseline TXT and fresh AlbumEasy PDFs for representative ordinary albums; retain existing imported-album fixtures as regression references. The initial frozen `tests/fixtures/albumeasy/core-v1/album.txt` and `expected.json` establish basic geometry/text/row comparisons; see `docs/testing/ALBUMEASY_CORE_V1_BASELINE.md` for measured 4.5.0 reference evidence. Command-family references now exist for bounded slices; broad ordinary-generated, asset and native comparisons remain open. Never mutate production albums.
- [~] T004 [Plan:1.2] Verify the installed `oxidize-pdf` generation API and license, font embedding/measurement, Unicode, images/transparency, page sizes, metadata and write errors. PDF generation, image/transparency, text extraction and geometry have been exercised in slices; a complete API/license/font-embedding/Unicode/metadata assessment remains open.
- [~] T005 [Plan:1.2] Build an isolated proof page with known millimetre dimensions, a stamp image, caption, heading and border. Verify PDF dimensions, text extraction and a 100%-scale printed ruler. Native proof generation and PDF geometry/text/image checks exist; a physical 100%-scale ruler check is not recorded.
- [~] T046 [Plan:1.3] Complete and approve the K1–K12 configuration contract in `docs/reports/ALBUMEASY_4_2_1_APPLICATION_GAPS.md`: map local `config.toml`, `src/config.rs`, `config_template.toml`, `config_full_example.toml` and `docs/guides/CONFIG_PARAMETERS_INVENTORY.md` to actual command/layout semantics, defaults, units, alias precedence and album/per-run overrides. Resolve image-padding/stretch/aspect, header/title and row-field naming conflicts before model design; preserve credentials and keep the local config untouched. #63 implements only the separately authorized explicit native page-settings adapter and tests; the broad contract, overlays and migration decisions remain open. ID appended for stability; planning precedes broad Phase 2 integration.

**Exit gate:** Command/feature and configuration contracts are approved; native proof page opens and prints
correctly; required missing capabilities are identified before the larger refactor.
Stop for a maintainer decision if the library cannot meet a core requirement.

### Phase 2 — shared layout and font metrics

**Plan P2.1:** Establish an authoritative editable document and resolved layout, with native-only features and provenance.
**Plan P2.2:** Move layout decisions out of TXT emission without changing behavior.

- [~] T006 [Plan:2.1] Implement shared document/layout under approved `src/native_pdf/document.rs`, with one file per AlbumEasy command grouped under `commands/`: album/page settings, rows, placements, text styles, title compositions, borders and resolved assets. #61 implements explicit page settings, #62 adds numeric borders, #63 adds a separate user-config-to-settings snapshot, and a standalone `.stampalbum` model exists. Preserve stable element identity, imported source locations and document revision; persist native-only settings additively without discarding command provenance. Resolve one effective config snapshot per run; explicit imported settings remain authoritative unless a recorded user override changes them. The canonical application-wide document/model and provenance contract remain incomplete.
- [~] T007 [Plan:2.1] Define canonical units, coordinate origin, rounding, page/margin bounds and physical-versus-presentation boxes. #61 implements `src/native_pdf/units.rs` with unrounded f64 mm, PDF bottom-left coordinates, point conversion and validated page/margin/content bounds; stamp-frame slices exercise physical/presentation separation. Wider precision, export and box policies remain open.
- [~] T008 [Plan:2.2] Establish a shared font measurement/wrapping policy. AFM metrics and bounded wrapping are implemented for the standalone renderer. Map current AlbumEasy fonts explicitly to licensed, available fonts; specify embedding and substitution behavior. Never silently change fonts. Embedding, substitution/license decisions and broad glyph validation remain open.
- [~] T009 [Plan:2.2] Extract ordering, stamp selection, row packing, spacing modes, caption heights, page breaks, even-page margins and image-box sizing from the active generator into the shared builder, reusing existing helpers. Standalone stamp row packing/alignment, pagination, captions and sizing exist; extraction from the active ordinary-album generator into a shared builder remains open.
- [~] T010 [Plan:2.2] Ensure the new path returns an actual layout/result rather than relying on `generator_new/mod.rs`'s empty-path fallback. The standalone parser/example returns rendered PDF/page results; ordinary application generation still uses the active legacy generator and the new generator entry remains incomplete.

**Exit gate:** Shared layouts reproduce baseline order, page count and dimensions;
TXT behavior is still preserved. No backend-specific pagination remains hidden in
the native renderer.

### Phase 3 — AlbumEasy TXT exporter

**Plan P3.1:** Export a version-targeted compatibility projection of the native document/layout.

- [ ] T011 [Plan:3.1] Adapt `generator_new/emit.rs` and related emit/postprocess helpers to consume a compatibility projection of the shared document/layout; classify each feature as exact, approximated, omitted or blocked before writing target-version commands.
- [ ] T012 [Plan:3.1] Preserve escaping, relative image paths, command order, row spacing semantics, page breaks, fonts, numbering and all supported global settings. Keep asset paths portable.
- [ ] T013 [Plan:3.1] Validate emitted TXT against the pinned language and, during development, compare AlbumEasy-rendered outputs against the common-feature baseline. Persist divergence findings and require explicit approval for lossy mappings; native-only enhancements need not match AlbumEasy appearance.

**Exit gate:** Exported TXT opens/renders in AlbumEasy. Preserve byte-normalized
baseline TXT where feasible; review intentional differences and prove their effect.
The TXT is useful for editing, not merely syntactically valid.

### Phase 4 — native PDF renderer

**Plan P4.1:** Render the shared layout using `oxidize-pdf`.  
**Plan P4.2:** Produce reliable paired outputs with explicit diagnostics.

- [~] T014 [Plan:4.1] Implement a focused renderer, for example `src/album_globals/native_pdf.rs`, consuming resolved pages/placements without repeating selection or pagination. A standalone renderer exists under `src/native_pdf/` and produces PDFs for explicit native and bounded compatibility inputs; it is not wired to the ordinary shared application layout.
- [~] T015 [Plan:4.1] Render page sizes, margins, borders, headings, page numbers, stamp/image boxes, aspect-ratio-preserving images, captions, alignment and supported colors. Reject unsupported commands/styles explicitly. These capabilities are covered in bounded proof and command-family slices; the full canonical feature set and command language are not implemented.
- [ ] T016 [Plan:4.1] Implement font embedding and glyph validation using the Phase 2 policy; verify accented country names, punctuation, denomination fractions and currency symbols.
- [ ] T017 [Plan:4.2] Freeze one document revision, stage/validate the native PDF first, then generate compatible TXT/assets/report. Distinguish PDF success/TXT blocked, PDF failure and publication failure; do not label a pair complete when TXT failed. Preserve previous files and report cleanup failures. Do not promise multi-file atomicity across a crash.
- [ ] T018 [Plan:4.2] Extend `GeneratedAlbumArtifacts`/diagnostics in `src/album_globals/mod.rs` to report document revision, selected/actual backend, individual output status, fresh paths, target language and compatibility report. Avoid success-shaped empty paths.

**Exit gate:** Native rendering produces fresh, valid PDFs with correct page
geometry; failure paths cannot masquerade as success or overwrite a valid prior
artifact with a partial file.

### Phase 5 — optional backend rollout and imported-album protection

**Plan P5.1:** Wire configuration, CLI and UI consistently.  
**Plan P5.2:** Preserve imported albums on the legacy path until their native coverage is certified.

- [ ] T019 [Plan:5.1] Add typed backend selection to `src/config.rs`, `config_template.toml`, `config_full_example.toml`, and `docs/guides/CONFIG_PARAMETERS_INVENTORY.md`; preserve missing-key defaults and album-local overrides.
- [ ] T020 [Plan:5.1] Wire a CLI backend override through `src/cli.rs`, `src/main.rs`, `src/modes.rs`, and shared orchestration. Regenerate `docs/CLI_USAGE.md` and update `docs/guides/CLI_OPTIONS_INVENTORY.md`.
- [ ] T021 [Plan:5.1] Update startup/country-review controls in `src/review_ui/views/bootstrap.html`, `bootstrap.js`, `compare.html`, `compare.js`, and `http_handlers.rs`; check configuration editor behavior and all Generate PDF actions.
- [ ] T022 [Plan:5.2] Preserve `src/external_album.rs` saved-command data and approval/revision lifecycle during transition. In `src/review_ui/views/external_album.html`, explain current coverage and external dependency; keep explicit fallback until Phases 7-9 certify native import.
- [ ] T023 [Plan:5.1,5.2] Update executable-required checks, status/help messages, progress, open-output behavior and diagnostics: native ordinary generation must work with no AlbumEasy installation; imported validation remains explicitly dependent on it.

**Exit gate:** Backend selection is consistent across entry points. Existing
configuration still follows the existing backend. Unsupported imported-album
native requests cannot silently render a different document.

### Phase 6 — parity certification, documentation and default decision

**Plan P6.1:** Validate print fidelity and portability.  
**Plan P6.2:** Decide whether native becomes the default.

- [~] T024 [Plan:6.1] Add focused layout/export/native renderer checks in `tests/` and inline test modules using existing tooling. Bounded geometry, text, image, failure-preservation, repeat-render and command-family regressions exist; the complete acceptance set including Unicode and paired-output integration remains open.
- [ ] T025 [Plan:6.1] Exercise CLI and both web generation paths, filters, TXT-only requests, backend overrides, AlbumEasy unavailable, native errors and imported-album preservation. The standalone renderer example is not the application-level CLI/web/backend rollout this task requires.
- [~] T026 [Plan:6.1] Compare native and AlbumEasy-rendered TXT outputs for representative albums on Windows/Linux; record geometric/visual differences and 100%-scale print checks. Several accepted command-family references have measured Linux comparisons; broad ordinary-album comparisons, Windows runtime and actual-size print validation remain open.
- [ ] T027 [Plan:6.1] Measure time, peak memory and output size for small/medium/large albums. Investigate regressions before rollout; do not claim speed or smaller files without measurements.
- [ ] T028 [Plan:6.2] Update user guides/release notes and create a derivative of the historical Draw.io baseline for the completed flow. Leave `app-logic-before.drawio` untouched.
- [ ] T029 [Plan:6.2] Regenerate `knowledge/` using `okf-rs generate`, complete maintainer review and record the default decision here. If approved, make native default for ordinary albums in a separate reviewed change; keep explicit AlbumEasy selection.

**Exit gate:** Maintainer accepts the parity evidence and documented limitations.
If not accepted, native stays opt-in and AlbumEasy remains the default.
This certifies ordinary generation only, not full AlbumEasy import support.

### Phase 7 — enhanced title composition and compatibility reporting

**Plan P7.1:** Implement left/right title icons as the first enhanced feature (REQ-010, REQ-013).
**Plan P7.2:** Complete enhancement/export divergence handling (REQ-003, REQ-012).

- [ ] T030 [Plan:7.1] Extend the Phase 2 document/layout in `src/album_globals/album_layout.rs` (or its established equivalent) with independent left/right title icon assets, dimensions, aspect ratio, gaps, vertical alignment and title-area sizing; preserve native settings in the album database.
- [ ] T031 [Plan:7.1] Implement heading measurement and native rendering in `src/album_globals/native_pdf.rs` and shared layout helpers. Cover long/wrapped titles, one/both icons, even-page margins and explicit failure on missing/oversized assets.
- [ ] T032 [Plan:7.1] Expose title icon controls through the owning album/config editor and `src/review_ui/http_handlers.rs`; wire applicable `src/config.rs`, template/example/inventory settings and portable asset resolution.
- [ ] T033 [Plan:7.2] Extend `generator_new/emit.rs` compatibility projection: prove any exact AlbumEasy icon mapping or offer explicit approximation/omission/blocking; retain the enhanced native original.
- [ ] T034 [Plan:7.2] Wire per-output compatibility results and saved loss policies through `src/album_globals/mod.rs`, `src/cli.rs` and both web generation flows. Add title-icon and strict/lossy-export regressions under `tests/`.
- [ ] T035 [Plan:7.1,7.2] Add an enhancement backlog and capability/export classifications to this document. Only title icons are committed scope; obtain approval before implementing other ideas.

**Exit gate:** Icons render and print correctly, settings survive save/reload,
TXT remains valid, and every intentional export difference is reported. The
native document can regenerate the enhanced PDF even after a lossy TXT export.

### Phase 8 — complete AlbumEasy language and application support

**Plan P8.1:** Implement a versioned command language, not an allowlist of names (REQ-011).
**Plan P8.2:** Complete command-to-document/render/editor/export support (REQ-005, REQ-011, REQ-013).

- [~] T036 [Plan:8.1] Extend the coverage matrix to installed 4.5.0 using `docs/guides/ALBUMEASY_4_5_0_REFERENCE_UPDATE.md` and exact installed-help grammar; preserve `docs/guides/ALBUMEASY_4_2_1_COMMAND_INVENTORY.md` as the legacy 92-name/12-alias baseline. A 4.5.0 stamp-family matrix, multiple reference fixtures and bounded mapping evidence now exist. Reconcile the complete command language, overloads, units/defaults/state/interactions and parse/validation/edit/render/export evidence; use verified contracts to decide exact/lossy/blocked export. Full matrix and production review remain open.
- [~] T037 [Plan:8.1] Keep the AlbumEasy `.txt` importer as a compatibility adapter into the canonical native document. The standalone interpreter implements a bounded command subset with strict diagnostics and rejects unsupported commands. Replace placeholder validation in `src/external_album/parser.rs` with complete documented argument rules and precise source diagnostics; implement full quoting/escapes, preprocessing/includes/dependencies, ordering and command-state semantics into the canonical document. This task remains partial.
- [~] T038 [Plan:8.2] Implement command families incrementally across the canonical document/layout and native renderer: page/global settings, text/fonts, rows/alignment, stamp/blank/image placements, borders/colors/decorations, and every additional native command discovered by inventory. Bounded standalone page/image/text/row, background, footer, margin, border and heading slices are implemented; complete command-language coverage and canonical application rendering remain open. Do not require AlbumEasy TXT as an intermediate for native PDF generation.
- [ ] T039 [Plan:8.2] Extend `src/review_ui/views/external_album.html` and owning API for typed editing of every documented supported command/item, preserving order, identity, scope, revisions and source provenance. Recognition or raw-text passthrough alone does not count as application support.
- [~] T040 [Plan:8.1,8.2] Add per-command and interaction conformance fixtures under `tests/` and `samples/`; bounded command slices now have versioned references and native regressions, including defaults/state/invalid-input cases for mapped behavior. Full per-command coverage and the end-to-end import -> edit -> native render -> TXT export -> reimport workflow remain open.

**Exit gate:** Every command/item in the pinned reference is implemented and
evidenced across all required surfaces. No placeholder grammar, parse-only
support or unsupported silent passthrough may be counted as complete. Split
T037-T040 into bounded command-family tasks after inventory; do not attempt one
large speculative rewrite.

### Phase 9 — standalone native AlbumEasy import and final certification

**Plan P9.1:** Remove mandatory external rendering from the complete import lifecycle (REQ-005, REQ-007, REQ-008).
**Plan P9.2:** Certify and release standalone compatibility (REQ-006, REQ-009, REQ-011).

- [ ] T041 [Plan:9.1] Adapt `src/external_album.rs` inspection/import/validation to parse original TXT and dependencies into the authoritative document, preserving copied inputs, asset checksums, source diagnostics and additive schema compatibility. Keep unsupported/failed imports explicit drafts.
- [ ] T042 [Plan:9.1] Replace mandatory external render/compare in imported-album approval and page/album edit regeneration with native semantic validation and rendering. Preserve manual fidelity approval, stale-write rejection, page-local reflow and current-PDF status; explain validation limits when no independent reference exists.
- [ ] T043 [Plan:9.1] Remove unconditional executable-required gates from import/editor/generation APIs and UI. On a machine with no AlbumEasy installed, verify inspect -> import -> approve -> edit -> regenerate -> export -> reopen; no subprocess invocation is allowed on the native path.
- [ ] T044 [Plan:9.2] Certify the full command matrix, interaction corpus and representative real albums on Windows/Linux, using development-time reference renders when available. Resolve unexplained semantic/geometry differences; report intentional native-only export divergence separately.
- [ ] T045 [Plan:9.2] Update user guides, compatibility/version statement, migration notes, derivative flow diagram and `knowledge/`; record standalone-import acceptance and default-backend decision separately from the earlier ordinary-album rollout.

**Exit gate:** Supported-version AlbumEasy files import/edit/render/export
without the external program, including a clean-machine test. Full compatibility
is a versioned conformance claim, not a promise of byte-identical PDFs or support
for undocumented commands. The external backend becomes optional.

## 5. Estimated work involved

These are **planning ranges**, not measured estimates or a delivery promise.
One experienced developer working full-time; a day means approximately eight
focused engineering hours. Revise after Phase 1.

| Phase | Complexity | Estimated effort |
|-------|------------|------------------|
| 1. Inventory, baseline and library proof | Medium | 2–4 days |
| 2. Shared layout extraction and font metrics | High | 6–12 days |
| 3. Compatible TXT exporter | Medium/high | 3–6 days |
| 4. Native renderer and reliable outputs | High | 5–10 days |
| 5. Backend controls and transitional import protection | Medium | 3–5 days |
| 6. Parity, cross-platform validation and rollout | High | 5–10 days |
| **Ordinary-album foundation (Phases 1–6)** | | **24–47 developer-days** |
| 7. Title icons and divergence handling | Medium/high | 4–8 days |
| 8. Full command language, rendering and editing | Very high; inventory-dependent | 20–45 days |
| 9. Standalone import lifecycle and full conformance | High | 8–16 days |
| **Expanded total before contingency** | | **56–116 developer-days** |

Allow roughly 20–30% contingency: approximately **67–151 developer-days overall**.
The previous **29–61 day** range covered the ordinary-album foundation only.
Full language support materially expands the project; revise the new estimate
after the versioned command matrix is complete. Unspecified future enhancements
are excluded.

**Dependencies:** Phase 1 precedes Phase 2; Phase 2 precedes Phases 3/4. Native
and TXT exporters can then be developed independently against a frozen layout
contract. Phase 5 needs both exporters and stable generation results. Phase 6
requires the integrated paths; start collecting evidence throughout.
Phase 7 builds on the Phase 2/4 foundation and compatibility exporter; it can
precede Phase 6 certification if included in the first native release. Phase 8
can start inventory/parser work after Phase 1, but complete semantics requires
the shared document/renderer. Phase 9 depends on Phase 8 coverage and integrated
editor support. Preserve task IDs; phase dependencies, not numeric IDs alone,
determine execution order.
The maintainer approved incremental compiled slices alongside command analysis.
T001/T036/T046 contracts still gate broad backend/import/config integration, not
isolated tested page primitives. Each slice must record resolved semantics and
unresolved interactions; do not require re-analysis of already verified behavior.

## 6. Validation strategy and proposed acceptance criteria

Use existing Cargo/Node test infrastructure; no new test framework by default.
Add runnable regression coverage as part of implementation, not after rollout.
Use isolated temporary albums/databases and preserve all user inputs.

### Three independent certification tracks

1. **Common-feature baseline:** compare native and AlbumEasy outputs for
   geometry, ordering, wrapping and pagination within approved tolerances.
2. **Enhanced native documents:** certify the native result independently;
   verify TXT validity and the accuracy/approval of every declared export loss.
   Pixel or pagination parity with the enhanced PDF is not required.
3. **Imported AlbumEasy conformance:** certify every pinned command and its
   interactions, source roundtrips and standalone editing/rendering. Passing
   ordinary-album tests does not satisfy this track.

### Representative coverage

- Small ordinary album and multi-page ordinary album.
- Dense captions, long series titles and wrapping at a page boundary.
- Mixed stamp sizes, wide/tall stamps, fixed-spacing rows and commemorative layouts.
- Alternate/even-page margins, headings, borders and page numbering.
- Images with transparency, missing assets, non-ASCII filenames and path spaces.
- Unicode text, fractions/currency symbols and unavailable font glyphs.
- Empty selection, year/catalogue filters and TXT-only requests.
- Existing imported albums, repeated placements and page-local saved edits.
- Title icons (left/right/both/none), missing/oversized assets, long headings and
  save/reload of native-only settings.
- Strict and approved lossy exports; exact report findings; native PDF success
  with TXT export failure; preservation of enhanced originals.
- Every pinned command, defaults/aliases and state interactions; unknown command
  errors and include/dependency path handling.
- Standalone import/edit/render/export on Windows/Linux with no AlbumEasy executable.

### Proposed thresholds — approve in Phase 1

| Check | Proposed acceptance |
|-------|---------------------|
| Page count and stamp selection/order | Exact match for common-feature/import conformance fixtures; enhanced export differences require an explicit compatibility finding and approval. |
| Paper size and physical stamp/box dimensions | Within 0.1 mm of specified geometry. |
| Placement and margin/border geometry | Within 0.25 mm of the approved baseline. |
| Text wrapping | No clipping, overlap or omitted glyphs; line-break differences require explicit review. |
| Export compatibility | Companion TXT is valid for the pinned target version; development reference renders have no command errors. Native-only differences are itemized rather than required to be visually identical. |
| Visual comparison | Review at a fixed rendering scale/DPI; approve intentional differences. Do not rely solely on a pixel score. |
| Printed output | Ruler/box check at 100% scale with printer fit-to-page disabled. |
| Error behavior | Failed rendering/publishing gives explicit failure; no stale/partial PDF reported as newly generated. |
| Windows/Linux | Relevant tests and build/type-check pass on both; disclose any unverified runtime/print checks. |
| Command coverage | Every pinned command/variant has parse, semantics, editor, native-render and export evidence; no placeholder support. |
| Standalone import | Full native lifecycle succeeds without locating or launching AlbumEasy; source/asset/revision fidelity remains intact. |
| Enhanced title | Correct placement/print dimensions, no overlap/clipping, persistent settings and accurate export compatibility findings. |
| Loss reporting | Every applied approximation/omission is visible and recorded; strict export blocks unsupported mappings without destroying the native output/document. |

Run the smallest relevant checks per phase, then the relevant regression suites
before release. Record commands, exit codes, pass/fail counts, temporary file/DB
impacts and discovered logic risks in `docs/testing/`.

## 7. Main risks and decisions still open

| Risk/decision | Required action |
|---------------|-----------------|
| Font metrics differ from AlbumEasy, changing pagination. | Prove shared wrapping/measurement and font mapping before certifying the layout. |
| TXT row commands cause AlbumEasy to calculate positions differently. | Compare rendered companion TXT, not just command strings; identify representability gaps. |
| Active generator mixes selection, placement and emission. | Extract incrementally with baseline parity; avoid completing an unrelated full refactor. |
| Imported command state differs from ordinary layout. | Preserve provenance and semantics through an explicit interpreter/document adapter; retain the old path until command coverage is certified. |
| Library image support does not meet print quality or format needs. | Verify formats/resolution/transparency; reuse existing image helpers where suitable. |
| A saved/global backend preference accidentally affects import validation. | Make the imported-album exception explicit and test every imported render/validate/edit action. |
| Partial paired-file publication. | Define accurate partial-success reporting and staging/cleanup; never claim impossible cross-file atomicity. |
| Missing fonts, assets or external AlbumEasy executable. | Return actionable diagnostics; no silent substitutions or automatic fallback. |
| Full language scope/version drift. | Pin the reference version and count every command/variant; treat later versions as explicit follow-on work. |
| Enhanced PDF and TXT differ intentionally. | Keep the native original authoritative and report exact/approximated/omitted/blocked outcomes; do not present lossy roundtrips as lossless. |
| Validation without an independent renderer cannot prove visual equivalence. | Use semantic/geometry conformance fixtures and development reference evidence; retain user fidelity approval and disclose gaps. |

**Approval record**

- Backend setting/override naming: **Pending**
- Initial font family mapping and embedding license: **Pending**
- Required initial command/style subset: **Pending Phase 1 inventory**
- Geometry/visual thresholds: **Proposed; pending approval**
- Native-default promotion: **Not approved; AlbumEasy remains default**
- Enhanced native PDF with explicitly divergent compatibility export: **Requested direction**
- Main-title left/right icons: **Requested scope; detailed design pending**
- Complete 4.2.1 AlbumEasy application support and standalone import: **Requested scope; initial manual inventory recorded, detailed coverage/ambiguities pending**
- Export loss approval/report format: **Proposed strict default; design pending**
- Other native enhancement ideas: **Backlog only; not approved**

### Native-only side labels — issue #97

`NATIVE_STAMP_IMAGE_SIDE_LABEL` maps left, right, or both text labels beside a
stamp frame. Labels are outside the frame, vertically centered, and separated
from it by at least 1 mm; native row layout reserves their width and rendering
rejects labels that exceed the page content area. TOML config defines the
default font size, wrap width and line limits; optional per-item values can
override them. Wrapped labels reserve their complete footprint, so labels
taller than a frame expand the row and reposition later items without resizing
the stamp. The saved US-Letter proof uses the TOML font-size default for its
longer description-style labels on both sides of the no-stretch example in
`tests/fixtures/albumeasy/stamp-image-controls-v1/`. The native library suite
and seven affected integration suites passed 150 tests (0 failed, 4 ignored);
maintainer and actual-size print acceptance remain pending. This is a
native-only capability slice, not an AlbumEasy command mapping or production
integration; it does not change any numbered roadmap task's completion status.

### Australia comparison follow-up TODOs — issue #99

Recorded from the [first-20-source-page comparison report](../../tests/fixtures/albumeasy/australia-first20-v1/README.md).
The historical AlbumEasy reference and native compatibility proof each contain
20 pages; the enhanced native proof contains 25 after reflow. All retain 190
image placements. Eleven automated tests pass, but these counts do not certify
visual parity or physical print acceptance. The items below refine existing
T036-T040/T044 work; they do not add numbered roadmap tasks, mark any complete,
or authorize production integration.

#### Compatibility and native-language gaps

- [ ] **AUS-01 — Four-argument `ROW_START_ES`** (T036-T038, T040):
  verify the fourth argument's grammar, units, defaults and effect against
  pinned AlbumEasy help and a fresh focused reference. The Australia source
  contains 80 occurrences; today's compatibility copy explicitly removes
  `6.0`, whose historical effect is unverified. Implement the verified mapping
  and regression tests before removing those adaptations. Acceptance: the
  original selected TXT renders unchanged with all 190 placements and verified
  row behavior; preserve the historical adapted fixture as evidence.
- [ ] **AUS-02 — Caption, baseline and row-spacing fidelity** (T038, T040,
  T044): measure the compact/shifted compatibility layout and crowded captions
  against comparable reference pages. Distinguish source crowding from native
  regressions; add wrapping/footprint/position assertions for dense and
  mixed-height rows. Do not resize mounting frames to make text fit. Acceptance:
  unexplained layout differences are resolved or explicitly classified with
  measured evidence and maintainer review.
- [ ] **AUS-03 — Native free-standing page text and vertical spacing**
  (T038, T040): define native equivalents for `PAGE_TEXT_CENTRE` and
  `PAGE_VSPACE`, retaining order, font, positioning and source provenance.
  The enhanced demonstration currently relocates 181 page-text entries into
  captions and does not reproduce explicit vertical spaces. Acceptance:
  reproduce representative source page-text/spacing behavior without using
  stamp captions as a substitute, then retain all selected source metadata.
- [ ] **AUS-04 — Explicit native page breaks and unified album pagination**
  (T038, T040): add a deliberate source-page-break mapping so one native
  document can represent the 20 source pages without 20 separate documents
  and an external PDF merge. Define source-page versus physical-page numbering
  under reflow, including global totals and source provenance. Acceptance:
  test breaks, continuation pages and footers in one native render; no
  AlbumEasy rendering dependency. The current local numbering is intentional
  fixture behavior, not a compatibility-renderer numbering defect.
  Bounded progress (#103): `NATIVE_PAGE_BREAK ()` now flushes rows, retains
  automatic overflow/caption continuation and intentionally empty pages
  (leading, repeated, trailing breaks). Page totals and numbering are tested.
  The maintainer accepted the two AlbumEasy PAGE_START references and requested
  intentional blank-page support. TXT groups support one pre-page definition
  and quote-preserving `$$1`–`$$9` substitution over already supported body
  commands. Reference probes also identified the omitted vertical gap after a
  TXT page title; the compatibility reservation is corrected without changing
  native explicit title-gap semantics. See the
  [proofs and run evidence](../../tests/fixtures/albumeasy/page-start-v1/README.md).
  Full Australian source-page provenance/reflow and free page-text fidelity
  remain open; do not close this whole follow-up or its roadmap tasks.
- [ ] **AUS-05 — Compose headings with inside and side labels** (T038, T040):
  remove the attachment-order limitation that currently moves each source
  page's first stamp heading into its caption. Acceptance: one image/frame can
  retain its heading plus above/below inside labels and left/right wrapped
  labels, with their full measured footprint, no overlaps and unchanged frame
  dimensions.

#### Demonstration fidelity and acceptance

- [ ] **AUS-06 — Separate fidelity proofs from enhanced presentation**
  (T040, T044): retain the existing demonstration, but add a source-style native
  comparison when the required mappings are ready. Account explicitly for
  white rather than black frame interiors, black rather than dark-blue header,
  stacked captions, 5 mm rather than 3 mm row spacing, isolated label rows and
  60% demonstration illustrations. These are intentional fixture choices, not
  established renderer defects. Acceptance: each remaining difference is
  labelled exact/intentional/unresolved; no silent approximation or source
  modification, and all 190 source frames/images remain traceable.
- [ ] **AUS-07 — Full-size visual and physical print review** (T040, T044):
  review all three PDFs, especially tiny 3.5 pt labels, dense captions and the
  five enhanced continuation pages (source pages 2, 11, 15, 16 and 19). Confirm
  wrapping limits, clearance and readability at Actual size on US Letter.
  Preserve the source's 216.0 x 279.4 mm document geometry; do not silently
  normalize its width or claim source frame sizes are certified physical stamp
  dimensions. Acceptance: record maintainer visual/print decisions and any
  resulting defects; automated counts/contact sheets are not approval.
- [ ] **AUS-08 — Windows comparison validation** (T040, T044): run the Australia
  integration tests and reproduction steps on Windows, checking relative
  assets, source checksums, page/image counts and PDF combination tooling.
  Acceptance: retain Windows results alongside the existing Linux evidence,
  or document an explicit unresolved platform blocker.

Keep the supplied historical PDF unchanged, including its `of 82` footer.
Use fresh, separately named AlbumEasy references when investigating semantics;
the historical PDF has not been freshly certified against the current TXT.
Production integration remains deferred until command mapping and review are
complete. This checklist records follow-up work only.

### External Brazil comparison follow-up TODOs — issue #100

The [Brazil first-20 proof report](../../tests/fixtures/albumeasy/brazil-first20-v1/README.md)
adds externally authored source evidence: three 20-page renditions, each with
84 image placements, from a supplied 136-page AlbumEasy 4.5.0 PDF. Its original
TXT requires explicit test-copy adaptations; this is not unchanged-input
compatibility certification. Add these cases to T036-T040/T044:

- [ ] **BRA-01 — Legacy image padding units:** implement
  `ALBUM_STAMP_IMG_SETTING` using documented 0.35 mm units, preserving stretch
  and state behavior. The fixture explicitly converts `(13 13 False)` to
  `ALBUM_STAMP_IMG_NEW_SETTING (4.55 4.55 False)`; do not treat it as a
  name-only alias or interpret 13 as millimetres. Verify reference image bounds.
- [ ] **BRA-02 — Legacy page numbering:** map `ALBUM_PAGES_NUMBER` with verified
  obsolete-name semantics and unchanged arguments. Brazil starts at `BR 85`;
  confirm pages 1-20 end at `BR 104`. The current supported copy substitutes
  `ALBUM_PAGES_FOOTER_NUMBER`; the original spelling is still rejected.
- [ ] **BRA-03 — Alternating margins in the compatibility interpreter:**
  support `ALBUM_PAGES_MARGINSE` with physical-page parity. Brazil's odd margins
  are `(20 10 10 10)` and even margins `(10 20 10 10)`. The compatibility proof
  explicitly defers this command and uses odd margins throughout; the native
  demonstration selects source parity separately for each input. Neither is
  proof of automatic alternating-margin import under reflow.
- [ ] **BRA-04 — Semicolon annotations:** verify source grammar against the
  pinned reference and handle trailing `;` and `;date` annotations outside
  quoted strings, without corrupting quoted punctuation. All 74 source page
  text lines need explicit annotation removal in today's supported copy;
  original annotations remain in the ledger. Add parser regression cases.
- [ ] **BRA-05 — Automatic centered page-text wrapping:** verify the reference
  wrapping width, word breaks, baseline and reserved height. Source page 9's
  long Franciscan Province heading wraps in the historical PDF but is rejected
  unchanged by the native compatibility renderer. Today's test copy inserts
  an explicit newline using measured native content width. Fix the mapping,
  not the source; link spacing/baseline work to AUS-02/AUS-03.
- [ ] **BRA-06 — External-album acceptance:** review paired pages 1, 9 and 20
  plus all-page sheets, measure fixed-spacing rows and legacy image-fit
  differences, and run Actual-size US Letter print and Windows checks.
  Preserve the source's **217 x 280 mm** document size, which is not exact
  Letter; disclose printer nonprintable-area limits. Frame sizes are source
  box dimensions, not independently measured stamps. Series text relocated
  below frames, smaller label demonstrations and isolated rows are intentional
  native presentation changes, not established compatibility defects.
- [ ] **BRA-07 — Accurate empty-caption diagnostics:** the native seven-argument
  `STAMP_IMAGE` form rejects an empty caption with an image-path error because
  it reuses a nonempty quoted-string guard. Keep the four-argument caption-free
  form supported; when this parser is next changed, report the correct field
  and document whether an explicitly empty optional caption is valid. Add
  separate invalid-path and empty-caption regression cases.

These are follow-up TODOs only. Production integration, roadmap completion
counts, and the original source files remain unchanged.

### Christmas Island decorative-border follow-up TODOs — issue #101

The maintainer approved a two-source-page **A4** test of existing support only:
retain explicit border omissions in TXT/native-language proofs and add a separate
lower-level native Classic border proof. See the
[fixture report](../../tests/fixtures/albumeasy/christmas-island-first2-v1/README.md).
The supplied folder lacks `Classic.txt`/`Classic.png`; the installed AlbumEasy
assets are copied with hashes and checked against historical border geometry.
This exception does not authorize production mapping or a new format.

- [ ] **CIS-01 — Decorative border across both loaders** (T037-T040): map
  `ALBUM_PAGES_DECORATIVE_BORDER` into the canonical document and native
  `.stampalbum` representation. The lower-level renderer already supports Classic,
  but the TXT interpreter and native document loader do not wire it through.
  Acceptance: render both original source pages with all 21 stamps, four
  correctly mirrored corners and eight connecting segments per page; do not
  silently substitute an ordinary border or claim the separate border-only
  proof establishes loader support.
- [ ] **CIS-02 — Decorative dependency provenance** (T037, T040, T041): define
  explicit resolution and packaging of named border definitions and referenced
  PNGs when absent from the source directory. Verify installed Classic assets
  against the supplied historical PDF before claiming a match; missing files
  must produce actionable errors rather than an implicit system fallback.
- [ ] **CIS-03 — Empty page-text spacers** (T036-T040): verify how
  `PAGE_TEXT_CENTRE (HN 2 "")` reserves height and map that behavior without a
  nonempty-text failure. Three occurrences currently need explicit approximate
  2 pt `PAGE_VSPACE` replacements in the supported copy. Keep the approximation
  disclosed until reference baseline/spacing measurements establish semantics.
- [ ] **CIS-04 — Footer padding, case and empty affixes** (T036-T040):
  verify `ALBUM_PAGES_FOOTER_PAD (3.0)`, legacy numbering with lowercase `c`,
  mixed-case `ROW_ALIGN_top`, and native empty before/after strings. The loader
  proof currently defers footer padding, normalizes command/position case and
  uses nonempty native provenance affixes because empty affixes trigger a
  misleading image-path diagnostic. Link the quoted-field diagnostic work to
  BRA-07; test actual footer positions rather than just successful parsing.
- [ ] **CIS-05 — Border conformance and print acceptance** (T040, T044):
  preserve the original A4 reference; compare actual PDF ornament transforms,
  opacity/masks, connecting-line endpoints/widths, and page-content clearance.
  Explicitly distinguish the no-stamp border-only diagnostic from full album
  proofs. Record maintainer visual approval and Windows validation; any future
  Letter derivative must be separately named and must not overwrite A4 evidence.

Brazil's BRA-01–BRA-07 entries above remain open. The later resolution check
found no demonstrated JPEG pixel loss: the checked Brazil image retained its
original byte stream and dimensions. Do not record a lower-resolution PDF
defect without evidence; preview raster size and physical image scale are
separate from embedded image quality.

### Future idea recorded 2026-11-08 — no implementation authorized

**Country Review UI preview:** Consider showing a preview while reviewing a
country. Whether this should show a single stamp/image, the current album
page, or offer both remains undecided. Feasibility, preview source,
interaction and scope have not been investigated or approved.

Do not implement the Review UI idea or change production behavior without a
subsequent maintainer request. This idea does not change the current
reference-first command-mapping work or authorize production native-PDF
integration.

## 8. Requirement mapping

| Requirement | Plan items | Completion evidence |
|-------------|------------|---------------------|
| REQ-001 | P2.1, P2.2 | Shared layout builder; parity checks; T006–T010 |
| REQ-002 | P1.2, P4.1 | Native proof and renderer; T004–T005, T014–T016 |
| REQ-003 | P3.1, P4.2 | Companion TXT and paired results; T011–T013, T017–T018 |
| REQ-004 | P5.1, P6.2 | Typed backend settings and reviewed default; T019–T021, T029 |
| REQ-005 | P5.2, P8.2, P9.1 | Transitional protection then standalone lifecycle; T022–T023, T038–T043 |
| REQ-006 | P2.2, P3.1, P4.1, P6.1 | Geometry, text and print parity evidence; T008–T016, T024–T026 |
| REQ-007 | P4.2, P5.2, P6.1, P9.1 | Failure diagnostics, staging and unsupported-feature behavior; T017–T018, T022–T026, T041–T043 |
| REQ-008 | P5.1, P5.2, P7.2, P9.1 | CLI/UI/help/config consistency and standalone gates; T019–T023, T025, T034, T041–T043 |
| REQ-009 | P6.1, P9.2 | Windows/Linux build, runtime and print evidence; T024–T027, T044 |
| REQ-010 | P2.1, P7.1 | Persistent title composition and renderer/editor checks; T006, T030–T032, T035 |
| REQ-011 | P1.1, P8.1, P8.2, P9.2 | Pinned command matrix and complete conformance evidence; T001, T036–T040, T044–T045 |
| REQ-012 | P3.1, P4.2, P7.2 | Compatibility projection and visible persisted findings; T011–T013, T017–T018, T033–T035 |
| REQ-013 | P2.1, P7.1, P8.2 | Editable native-only settings, stable identities and provenance; T006, T030–T032, T038–T040 |
| REQ-014 | P1.3, P2.1, P5.1 | Configuration semantic/precedence map, snapshot and migration checks; T046, T006, T019–T021 |

## 9. How to keep this plan up to date

1. Update **Last updated**, phase status and task checkboxes after each work batch.
2. Keep task IDs stable; add new tasks rather than renumbering completed ones.
3. Mark a task complete only when its outcome and applicable checks are recorded.
4. Record deviations, blockers and changed estimates before starting affected work.
5. Link evidence, commits or local branch names in the progress log.
6. Do not mark native as default until T029 approval is recorded.
7. Track standalone-import approval separately at T045; ordinary native PDF
   approval does not mean every AlbumEasy command is supported.
8. Revisit estimates when the pinned command matrix or enhancement scope changes.

### Recorded execution workflow — approved for bounded verified vertical slices

- Use GitHub for this roadmap, as explicitly requested: one parent issue (epic)
  per phase and a sub-issue/branch per T-task. P-items group tasks; they do not
  require their own branches.
- During implementation, start each task from the designated main development
  branch, validate its acceptance criteria, commit the task, merge locally back
  to that integration branch, then start the next dependency-ready task there.
- Task commits and local merges may be automatic after required checks pass.
  Ask before any push. Do not commit failing or unvalidated implementation.
- Record epic/sub-issue, branch, checks, commit and merge evidence per task.
- The maintainer selected `development/native-pdf` from the current documentation
  branch. Preserve accumulated planning/baseline deliverables in the validated
  T003 increment; exclude unrelated manual/autosave files.
- GitHub #55 is the Phase 1 parent and #56 its T003 checker sub-issue. Full T003
  coverage remains incomplete after this subset; stop for review before the next
  increment. See the [baseline report](../testing/ALBUMEASY_CORE_V1_BASELINE.md)
  for commands, counts, negative controls and limits.

| Phase | Complete | Partial | Remaining | Current evidence / open gate |
|-------|---------:|--------:|----------:|-----------------------------|
| 1. Inventory, baseline, library/config proof | 0/6 | 6/6 | 0/6 | Legacy/4.5.0 stamp inventory, core baseline and explicit settings adapter exist; full language/caller/config contract and print ruler remain open. |
| 2. Shared layout and font metrics | 0/5 | 5/5 | 0/5 | Native units, text, frame and row primitives exist; canonical app model, generator extraction, font embedding/substitution and full layout policy remain open. |
| 3. AlbumEasy TXT exporter | 0/3 | 0/3 | 3/3 | Native document-to-compatible-TXT projection not implemented. |
| 4. Native PDF renderer and reliable outputs | 0/5 | 2/5 | 3/5 | Standalone renderer and many bounded features exist; app orchestration, full feature/font support and paired-output diagnostics remain open. |
| 5. Backend rollout and import protection | 0/5 | 0/5 | 5/5 | No production backend-selection rollout. |
| 6. Parity, cross-platform validation and default decision | 0/6 | 2/6 | 4/6 | Bounded Linux reference comparisons and renderer regressions exist; app flows, Windows/print/performance evidence and default decision remain open. |
| 7. Title composition and compatibility reporting | 0/6 | 0/6 | 6/6 | Title-icon and explicit export-divergence work not started. |
| 8. Full AlbumEasy language/application support | 0/5 | 4/5 | 1/5 | Bounded command parser/renderer and fixtures exist; full 4.5.0 matrix, canonical import, editor and complete conformance remain open. |
| 9. Native import lifecycle and certification | 0/5 | 0/5 | 5/5 | Full import/edit/regenerate lifecycle without AlbumEasy and final certification not started. |
| **Total** | **0/46** | **19/46** | **27/46** | Status is against full task acceptance criteria, not number of delivered incremental issues. |

This snapshot was reconciled on 2026-11-08 against the task descriptions,
recorded implementation evidence, and merged/pushed bounded slices through
#93. `[x]` is reserved for full task acceptance; do not convert bounded issue
completion into full roadmap completion. Recheck and update this table and the
task markers whenever implementation scope changes.

### Progress and decision log

| Date | Tasks / decision | Outcome / evidence | Next action |
|------|------------------|--------------------|-------------|
| 2026-11-09 | Revision 65: bounded PAGE_START groups and native breaks (#103) | Accepted reference first; added bounded TXT group expansion, explicit native breaks including intentional empty pages, and corrected the missing title-following vertical gap. Three native input paths and pagination/error/state regressions are exercised. | Review native proofs; retain full group-body language, PAGE_START_VAR/INFO, free native page text, production integration, Windows and actual-size print acceptance as open. No full roadmap completion. |
| 2026-11-09 | Revision 63: Christmas Island existing-support border test (#101) | Recorded CIS-01–CIS-05 for loader wiring, Classic asset provenance, empty spacers, footer/case/empty-affix semantics and border acceptance. Maintainer selected original A4 and a separate native border-only proof, not production implementation. Confirmed Brazil TODOs remain recorded; no demonstrated embedded-JPEG resolution loss. | Review two-page proofs and complete bounded mappings with reference evidence; no roadmap completion or production integration claimed. |
| 2026-11-09 | Revision 62: externally authored Brazil comparison (#100) | Recorded BRA-01–BRA-07 for legacy image units/numbering, alternating margins, semicolon annotations, automatic page-text wrapping, external-album acceptance and accurate empty-caption diagnostics. Three 20-page proofs retain 84 image placements each; all adaptations are explicit test inputs. | Review PDFs, validate reference semantics and address bounded mappings. No production integration or full task completion claimed. |
| 2026-11-09 | Revision 61: Australia comparison follow-up TODOs (#99) | Recorded AUS-01–AUS-08 for row overloads, layout fidelity, native page text/spacing/breaks, heading-label composition, intentional demonstration differences, print acceptance and Windows validation. Existing roadmap task IDs and completion counts remain unchanged. | Address each through bounded reference-first mapping and review; no production integration or new implementation authorized by this documentation update. |
| 2026-11-01 | Plan created for options 1, 2 and 4 | Repository generation/import paths inspected; no rendering implementation changed. | Approve boundaries and start Phase 1. |
| 2026-11-01 | Revision 2: expanded native-first scope | Added title icons, explicit export divergence, complete versioned AlbumEasy support and standalone import. T001–T029 retained; T030–T045 added. Previous permanent import exclusions superseded. | Pin target language/reference, create coverage matrix and approve compatibility-loss policy. |
| 2026-11-03 | Revision 3: manual-first planning checkpoint | Repaired duplicated merge-conflict text; inventoried supplied 4.2.1 manual (92 canonical names, 12 additional spelling aliases); recorded unresolved grammar discrepancies and existing title-icon composition. No application changes or commits. | Review command/behavior matrix and remaining maintainer ideas before approving implementation order. |
| 2026-11-03 | Revision 4: application/configuration gap audit | Added per-command static gap matrix, 24-name validator breakdown and K1–K12 config mapping; identified emitted/imported/reference mismatches. Added REQ-014/P1.3/T046 without renumbering existing tasks. No application/config changes or runtime certification. | Resolve syntax/image semantics and approve configuration precedence before shared-model implementation. |
| 2026-11-03 | Revision 5: reference version and core baseline | Selected 4.5.0 as primary with 4.2.1 legacy coverage; installed help resolves several alleged syntax gaps. Added core-v1 TXT and measured reference expectations without application code changes. | Preserve frozen reference; extend the versioned command matrix and add separate command-family fixtures. |
| 2026-11-03 | Revision 6: approved T003 baby step | Maintainer accepted visual/100%-scale print baseline and selected `development/native-pdf`. Added Cargo checksum, geometry/text and strict same-renderer drift checks; two fresh AlbumEasy renders match the frozen reference. Epic #55 / sub-issue #56. Native application/config changes remain paused. | Commit and merge validated checker locally, then stop for maintainer review. Full T003 and command/config contracts remain open. |
| 2026-11-03 | T003 extraction-standard follow-up (#57) | Maintainer requested existing `oxidize-pdf` for baseline text extraction instead of `pdftotext`; all 20 text assertions preserved, with extraction failure/page-count/truncation gates. MuPDF geometry/trace checks retained. Six explicit checker tests pass. | Stop for review; native application/config changes and broader import-parser migration remain outside this increment. |
| 2026-11-03 | Revision 7: page-size-only conformance (#58) | Verified A4 portrait/landscape, exact Letter and fractional dimensions; captured four references and two missing-input diagnostics. Mapped existing config precedence/defaults/emission, recording precision and exit-code risks. No renderer/application changes. | Preserve separate page-size-v1 bundle and stop for review. Full T001/T003/T036/T046 remain incomplete; no generalized fixture runner or native renderer is claimed. |
| 2026-11-03 | Revision 8: margins-only conformance (#59) | Verified left/right/top/bottom order, asymmetric/fractional margins, explicit even-page override and two failure diagnostics. Measured eight page borders and repeated four renders; traced config mapping and border half-stroke inset. No renderer/application changes. | Preserve margins-v1 and stop for review. Generalized fixture automation, native implementation and full command/config contracts remain open. |
| 2026-11-03 | Revision 9: spacing-only conformance (#60) | Verified zero/fractional independent horizontal/vertical spacing with 16 frames and eight text anchors, four repeat renders and two failure cases. Documented misleading config descriptions, differing defaults and width-accounting risks. No application changes. | Preserve spacing-v1 and stop for review; generalized fixture runner and full command/config contracts remain open. |
| 2026-11-03 | Revision 10: compiled page-settings slice (#61) | Approved one-command-per-file vertical slices. Added four typed command modules, explicit shared settings/f64 mm geometry and oxidize-pdf geometry proof/example. Six native tests plus six core tests pass, including actual PDF path checks. Windows GNU cross-check passed. | Review native proof; next implement another command with reference and native comparisons together. No production integration/full TXT parser or full task completion claimed. |
| 2026-11-07 | Revision 47: bounded margin-text mapping (#90) | Accepted three AlbumEasy references; nine native PDFs across TXT/raw/native routes. Rotated transforms within 0.01 pt, TXT/raw rasters identical, image dimensions retained. 108 scoped tests passed/17 ignored; Denmark v5 has zero deferrals. Strict third-definition rejection documented. | Review native proofs/Actual-size print. Retain immutable fixtures and three-input gates; no production integration, PR or OKF. Commit/push awaits instruction. |
| 2026-11-07 | Revision 48: inner-border reference pass (#91) | #90 committed/pushed as 697eb0d3 after 108 passing precommit tests. New AlbumEasy-only two-page/12-image reference covers ordinary inner styles, colours, save/restore and cancellation. NO_FRAME removes both borders; outer BLANK retains inner. Actual dash/side geometry recorded. | Stop for reference acceptance before native implementation; no production integration, PR or OKF. |
| 2026-11-07 | Revision 49: accepted bounded inner-border mapping (#91) | Three native input routes, actual translated path/dash/colour checks and immutable reference. Zero probe verifies PDF hairlines. 114 scoped tests passed/18 ignored; Denmark v5 rerenders without source changes. | Review native proofs/Actual-size print; remaining CORNER/SIDES/shape/rotation and broader language mappings stay open. No commit/push, production integration, PR or OKF. |
| 2026-11-07 | Revision 50: outer/inner styles reference pass (#92) | #91 committed/pushed as f5ef92bc after 16 passing affected precommit tests. Two complete AlbumEasy-only reference pairs prove ordinary outer styles, corner selectors, persistent independent side masks and save/restore; four page rasters/actual strokes inspected. | Stop for reference acceptance before native implementation; preserve earlier fixtures and TXT/raw/native gates. No production integration, PR or OKF. |
| 2026-11-07 | Revision 51: approved bounded border-style mapping (#92) | Shared outer/inner style/selector state and paths; six native main proofs plus unequal-width/hairline and duplicate-selector probes. Image-centred path geometry within 0.02 pt, all side/corner masks, strict errors, saved state and inherited native inner styles verified. Broad regression gate and final affected gate recorded in fixture README; Denmark v5 source remains unchanged. | Review native proofs/Actual-size print; retain immutable references and deferred shape/rotation/text-only and broader language mappings. No commit/push, production integration, PR or OKF. |
| 2026-11-08 | Revision 52: future ideas only | Recorded optional text left/right/both beside stamp images/frames and a country Review UI preview (single image versus current page undecided). | No implementation/tasks authorized; preserve current mapping scope. |
| 2026-11-08 | Revision 53: heading reference pass (#93) | #92 committed/pushed as 41cba21a after 120 passing precommit tests. Two AlbumEasy-only two-page/12-image references cover local/VA headings, row alignment, multiline, signed offsets/padding and saved colour/padding. Corrected first-attempt overflow without changing physical sizes; all four final rasters and bounds checked. | Stop for reference acceptance before native code. MIDDLE/BOTTOM rows are a coupled prerequisite; preserve deferred ideas and prior fixtures. No production integration, PR or OKF. |
| 2026-11-08 | Revision 54: standalone heading mapping completed (#93) | Added shared heading-row layout geometry and standalone mapping for `STAMP_HEADING`, `STAMP_HEADING_VA`, `STAMP_HEADING_PADDING`, `ROW_ALIGN_MIDDLE`, and `ROW_ALIGN_BOTTOM` in compatibility and native routes. Added approved-reference derivatives (`alignment*.stampalbum`, `padding-state*.stampalbum`) and new [native heading regression suite](../../tests/native_pdf_stamp_headings.rs). Focused and retained broad native regression gates passed; commit `04c8a093` pushed to `issue/93-stamp-headings`. | Continue only with explicit next-slice approval; imported parser, production integration, PR, and OKF remain pending. |
| 2026-11-08 | Revision 55: roadmap status reconciled | Audited all 46 numbered roadmap tasks against their full acceptance criteria and recorded complete/partial/remaining markers. Totals: 0 fully complete, 19 partially evidenced, 27 remaining. Recent #86–#93 slice status remains 7 complete / 1 deferred (#88); these issue counts are not project-task completion counts. | Update task markers and phase totals after every material slice; do not treat partial evidence as full task completion. |
| 2026-11-08 | Revision 56: bounded native section numbering (#88) | Added explicit native `.stampalbum` section boundaries, section-local footer numbering and count tokens while preserving `$PAGES$` as the document total and leaving AlbumEasy TXT/raw behavior unchanged. Config/help and standalone renderer updated. The affected regression gate passed 73 tests (0 failed, 7 ignored); Clippy and example build passed with pre-existing warnings. Follow-up specifically rendered both default start 1 and custom start 10 across changed pagination: section suite 5 passed, and the standalone example produced a two-page PDF when passed TOML. `git diff --check` passed. Broader roadmap totals remain 0 complete / 19 partial / 27 remaining; the #86–#93 bounded slices are now 8 complete. | Keep production routing, full AlbumEasy language support and roadmap task completion separate; do not infer year boundaries from captions. |
| 2026-11-08 | Revision 57: review PDFs for #88 | Added a reusable 36-stamp native `.stampalbum` fixture and minimal TOML configs for section starts 1 and 10; generated two five-page exact-Letter PDFs. `pdfinfo`/`pdftotext` verified page sizes and all per-page document/section labels; `pdfimages` verified 36 placements per PDF. Representative pages were rasterized and visually checked. The focused section suite passed 5/5. Actual-size print and maintainer acceptance remain pending. | Review both fixture PDFs side by side before closing #88; retain explicit no-inferred-year and mid-page-boundary behavior. |
| 2026-11-09 | Revision 58: native-only side labels (#97) | Added `NATIVE_STAMP_IMAGE_SIDE_LABEL` with independent left/right selection, outside-frame placement, 1 mm clearance, row-width reservation and page-content validation. Updated the native image-controls proof and its saved Letter PDF, preview and text extract; AlbumEasy reference files remain unchanged. Two focused unit tests and 13 native-album tests passed (12 passed, 1 ignored). | Review the native proof before commit; production routing/integration and numbered roadmap completion remain out of scope. |
| 2026-11-09 | Revision 59: wrapped native side labels (#97) | Added TOML defaults/ranges and per-item wrap overrides (5–40 mm; 1–8 lines), multiline side wrapping, and row-footprint expansion/repositioning while preserving stamp/frame dimensions. Regenerated the one-page Letter native proof, preview and text extract; visually checked long left/right labels and verified text extraction. Config, album and five layout-regression suites passed 44 tests (0 failed, 4 ignored); `cargo fmt --all` and `git diff --check` passed. | Maintainer review and actual-size print acceptance remain pending; AlbumEasy inputs and PDF are unchanged. Production integration and numbered roadmap completion remain out of scope. |
| 2026-11-09 | Revision 60: TOML side-label font-size default (#97) | Added `font_size_pt = 3.5` to the TOML defaults; omitted command sizes use it and existing numeric per-item sizes remain overrides. Updated the native proof to exercise the default and tested that config changes affect layout while explicit overrides remain stable. Native library plus seven affected integration suites passed 150 tests (0 failed, 4 ignored); formatting and diff checks passed. | Maintainer review and actual-size print acceptance remain pending; AlbumEasy inputs/PDF and production integration remain untouched. |
