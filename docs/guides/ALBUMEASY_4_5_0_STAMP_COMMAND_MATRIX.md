# AlbumEasy 4.5.0 stamp command mapping — reference verified, application audit open

**Prepared:** 2026-11-06
**Target:** AlbumEasy 4.5.0; AlbumEasy 4.2.1 is retained as legacy coverage.
**Status:** Stamp-family syntax and documented behavior verified against the
installed AlbumEasy 4.5.0 command help. Application support and runtime
conformance remain unverified; this is not a compatibility certification.
**Implementation boundary:** Documentation/audit and compatibility planning
only. This matrix does not certify native import/render support and does not
authorize switching production PDF generation.

## 0. Long-term native-rendering goal

The target workflow accepts either a native `.stampalbum` document or an
AlbumEasy `.txt` document with its referenced images, then creates the PDF
using StampAlbumGen2's native renderer. A `.stampalbum` is parsed directly;
an imported `.txt` is parsed as AlbumEasy commands and mapped into the same
canonical in-memory album model. The original AlbumEasy command text must not
need to be rewritten by hand or executed by AlbumEasy to produce the native
PDF.

Compatibility means that every command in the agreed AlbumEasy version scope,
including raw commands, overloads, state changes, and supported legacy
spellings, receives its mapped native meaning. It does not mean embedding
AlbumEasy syntax as native-only `.stampalbum` commands. Until an input command
has a verified mapping and conformance coverage, report it as unsupported or
unresolved; never silently drop it, guess its effect, or claim full-file
compatibility. AlbumEasy remains a reference tool for verifying behavior, not
a runtime renderer/dependency for the future native PDF workflow.

This is the intended destination, not current capability. The command matrix
records evidence and gaps; production integration and certification remain
deferred until the full language mapping and review are complete. Preserve
image references and resolve them relative to the source album (or its
explicitly managed asset set) while mapping the document.

### Current standalone compatibility slice

The [bounded native interpreter](../../src/native_pdf/albumeasy.rs) and
[standalone example](../../examples/render_stampalbum.rs) now render the
accepted image-controls AlbumEasy TXT and its identical raw-command
`.stampalbum` without AlbumEasy. Basic image stamps, captions, top-aligned
ES/FS rows, solid/blank frames and selected image controls have runnable
coverage. This does not certify all variants, state semantics, default row
alignment or full-file compatibility, and does not switch production routing.

The [Denmark supported test copy](../../tests/fixtures/albumeasy/denmark-txt-native-v1/README.md)
renders 16 pages and 41 images after eight explicitly documented occurrences
of six unsupported commands are commented. Background fills, footer/margin
colours, footer numbering and stamp-settings save/restore remain deferred;
see the [long-running plan](../reports/NATIVE_PDF_IMPLEMENTATION_PLAN.md#required-denmark-mappings-still-deferred).
The untouched original TXT still fails explicitly on these commands.
Standalone coverage here supplements, rather than upgrades, the conservative
production-support labels in the historical tables below.

**Background update (#86):** After acceptance of the
[AlbumEasy reference page](../../tests/fixtures/albumeasy/stamp-background-colours-v1/README.md),
the standalone interpreter maps `COLOR_STAMP_BACKGROUND` and
`COLOUR_STAMP_BACKGROUND` for the currently supported rectangular image boxes.
Colours persist per item, hidden images retain fills, `NO_FRAME` suppresses
fill/outline, and `BLANK` retains fill without outline. Native-language
background and outline controls remain independent. All three input paths
have colour, layering and physical-image-size regressions. The
[Denmark v2 proof](../../tests/fixtures/albumeasy/denmark-background-native-v2/README.md)
restores three background lines, leaving five other deferrals. The preceding
eight-deferral description records the historical v1 slice; backgrounds are
no longer deferred for the validated v2 rectangular cases. Shapes, rotated
boxes and CORNER styles remain outside this implementation boundary.

**Footer update (#87):** After acceptance of the normal
[AlbumEasy footer reference](../../tests/fixtures/albumeasy/footer-numbering-import-v1/README.md),
the standalone interpreter maps `ALBUM_PAGES_FOOTER_NUMBER` (3-6 arguments,
default start 1/empty prefix/suffix) and both footer-colour spellings.
Printed start and final `$PAGES$` count remain independent. Pre-page global
colour is captured; mid-document footer changes fail explicitly. All three
input routes have two-page/12-image proofs, strict-error/overflow tests and
native footer positioning coverage. TXT/raw rasters are identical; normal
footer text bounds match AlbumEasy within 0.003 points. Overall native row/
title layout differences and human visual/print acceptance remain open.
The [Denmark v3 proof](../../tests/fixtures/albumeasy/denmark-footer-native-v3/README.md)
restores footer colour/numbering and leaves three occurrences: margin-text
colour and stamp-settings save/restore. Full overload/state conformance and
production routing remain outside this bounded slice. Year/section-local
numbering is separately deferred under #88 without changing `$PAGES$`.

## 1. Purpose and scope

This is the first bounded command-family pass for the command-mapping work.
It captures stamp construction, stamp-image handling, stamp state, captions,
and border/frame state, plus a static spot-check of current TXT generation,
import validation/reflow, and native-PDF proof work.

Row construction/alignment is related but belongs to its own command-family
pass. Album/page commands, general text/fonts, colors as an independent family,
decorative-border definition files, preprocessing, and the complete 4.5.0
language catalog are not covered here.

**Application support statuses are deliberately conservative:**

- `Observed`: directly present in the inspected repository source or docs.
- `Partial`: a limited path exists but does not implement all documented
  variants or semantics.
- `Not accepted`: the inspected imported-album validator rejects the command.
- `Not assessed`: this pass did not establish support.
- `Verified in 4.5.0 help`: grammar or behavior is stated in the pinned
  reference below; this does not establish that the application implements it.
- `Open`: not specified by the inspected help or not established by this pass.

None of these labels means AlbumEasy-compatible native rendering.

## 2. Source quality and version boundary

1. The project decision is that **AlbumEasy 4.5.0** is the primary
   compatibility/reference renderer and 4.2.1 remains a separate legacy
   baseline. See the [4.5.0 reference update](ALBUMEASY_4_5_0_REFERENCE_UPDATE.md).
2. The authoritative source for this stamp-family pass is the installed
   `AlbumEasy45.0.qch` at `/home/ray/work/AlbumEasy/AlbumEasy45.0.qch`,
   SHA-256
   `8b81707d861aacceb798b9f38efbbfdc3ea86fa49a57b7e343c3a25d5964d5be`.
   The command page is titled **AlbumEasy Commands**. A read-only text
   extraction used for review is
   `/home/ray/.copilot/session-state/80e32404-199a-4106-9649-659ef9662bab/files/albumeasy-450-commands.txt`,
   SHA-256
   `171cdb2e4d3a2a122ea012743f24fd157e81b4ec3313fa81c372ba5ec0c64f8d`.
   Relevant section locators are stamp construction/image commands (extracted
   lines 773–928), image controls (929–988), blank/inner-border and border-style
   commands (989–1093), heading/footer controls (1094–1171), stamp-settings
   save/restore (1172–1205), and obsolete stamp spellings (1367–1416). The
   extracted help is a local audit aid, not a
   repository artifact; the QCH hash is the durable source identity.
3. The detailed in-repository [stamp command breakdown](../breakdown/StampCommands.txt)
   was a candidate source only. Its exact stamp-family grammar has now been
   checked against the pinned 4.5.0 help; it is not itself version authority.
4. The [4.2.1 command inventory](ALBUMEASY_4_2_1_COMMAND_INVENTORY.md) is the
   pinned legacy inventory, not authority for the full 4.5.0 command set.
   The [4.2.1 application gap report](../reports/ALBUMEASY_4_2_1_APPLICATION_GAPS.md)
   is likewise only a static, older-version application audit.
5. The reference confirms command names, parameter forms, selected defaults,
   state rules, and deprecations. It does not specify every implementation
   detail (for example save/restore nesting semantics); those remain open and
   must not be inferred from examples.

## 3. Stamp construction commands

The signatures below are normalized from the **AlbumEasy Commands → stamp
commands** section of the pinned 4.5.0 help. Dimensions and offsets are in
millimetres; text sizes are in points.

| Command | 4.5.0 signature | Documented behavior | Evidence status |
|---|---|---|---|
| `STAMP_ADD` | `(width height inside left centre right)`; legacy form `(width height txt0 txt1 txt2 txtBL txtBC txtBR)` | Rectangular frame with inside and below-frame text. All text arguments are required; use empty strings when unused. Text may be multiline with `\n`. Legacy form remains supported for backwards compatibility. | Verified in 4.5.0 help. |
| `STAMP_ADD_DIAMOND` | `(width height inside left centre right)` | Diamond frame with the common text fields. | Verified in 4.5.0 help. |
| `STAMP_ADD_OVAL` | `(width height inside left centre right)` | Oval frame; equal width and height make a round frame. | Verified in 4.5.0 help. |
| `STAMP_ADD_PENTAGON` | `(width SU\|CU inside left centre right)` | Pentagon with side-up or corner-up orientation; width gives its size. | Verified in 4.5.0 help. |
| `STAMP_ADD_HEXAGON` | `(width SU\|CU inside left centre right)` | Hexagon with side-up or corner-up orientation. | Verified in 4.5.0 help. |
| `STAMP_ADD_OCTAGON` | `(width SU\|CU inside left centre right)` | Octagon with side-up or corner-up orientation. | Verified in 4.5.0 help. |
| `STAMP_ADD_TRIANGLE` | `(width height inside left centre right [apex])` | Triangle; optional apex width flattens the apex. | Verified in 4.5.0 help. |
| `STAMP_ADD_TRIANGLE_INV` | `(width height inside left centre right [apex])` | Inverted triangle; optional apex width applies to its lower end. | Verified in 4.5.0 help. |
| `STAMP_ADD_RT_TRIANGLE` | `(width height SU\|CU L\|R inside left centre right)` | Right triangle with side-up/corner-up orientation and left/right vertical side. | Verified in 4.5.0 help. |
| `STAMP_ADD_BLANK` | `(width height)` | Adds a blank stamp box with no frame or text, useful for leaving a gap in a row. | Verified in 4.5.0 help. |
| `STAMP_ADD_IMG` | See overload table below. | Adds a stamp box containing a PNG or JPEG image; can also place non-stamp images. | Verified in 4.5.0 help. |

### `STAMP_ADD_IMG` overloads

| Form | 4.5.0 argument sequence | Documented behavior |
|---|---|---|
| Basic | `width height image left centre right [NO_FRAME]` | `NO_FRAME` is optional. |
| Image plus inside text | `width height image left centre right inside-text img-vert-offset txt-vert-offset [img-horz-offset txt-horz-offset] [NO_FRAME]` | Horizontal offsets are an optional pair; `NO_FRAME` is independently optional after them. |

The help says offsets may be positive or negative: positive moves down/right,
negative moves up/left. The inside text is horizontally centred; when
`txt-horz-offset` is nonzero, subsequent lines align with the first line.
Image paths are relative to the album text file. The help names PNG and JPEG
as supported formats. `STAMP_BORDER_STYLE (BLANK)` is also documented as an
alternative to `NO_FRAME`. Exact clipping/scaling behavior for oversized
images is not specified here and remains open for renderer tests.

## 4. Image controls and settings state

| Command | 4.5.0 grammar | Documented behavior | Application audit |
|---|---|---|---|
| `ALBUM_STAMP_IMG_NEW_SETTING` | `(horizontal [vertical] [stretch])` | Millimetre padding between image and frame. Omitted vertical padding copies horizontal padding; omitted stretch is `false`. Documented default is equivalent to `(0.7 0.7 false)`. Stretch affects only images smaller than the drawing area: `true` fills it, `false` leaves the image unstretched. | Ordinary TXT generation/tests use this spelling. The imported parser recognizes only the obsolete setting. |
| `ALBUM_STAMP_IMG_NEW_SETTING_PCNT` | `(horizontal [vertical] [stretch])` | Same controls expressed as percentages of box width/height. Omitted vertical padding copies horizontal; omitted stretch is `false` and has the same small-image-only effect. | No imported-parser support observed. |
| `ALBUM_STAMP_IMG_SETTING` | `(horizontal vertical stretch)` | Obsolete; padding values retain the historical 0.35 mm-per-unit interpretation. | Imported parser recognizes three arguments and treats the third as boolean. |
| `ALBUM_STAMP_NEW_IMG_SETTING` | `(horizontal [vertical] [stretch])` | Obsolete spelling, functionally identical to `ALBUM_STAMP_IMG_NEW_SETTING`; the `IMG_NEW` spelling is preferred. | Not accepted by the inspected parser. |
| `ALBUM_STAMP_IMG_ASPECT_RATIO` | `(fixed)` | Boolean. `true` preserves source aspect ratio for subsequent images; `false` fits to the stamp box and is the default. | Observed in generated TXT/config-driven output; not accepted by imported parser. |
| `ALBUM_STAMP_IMG_GREYSCALE_ON` / `ALBUM_STAMP_IMG_GRAYSCALE_ON` | No arguments | Subsequent stamp images display in grayscale; US spelling is accepted. | No imported-parser support observed. |
| `ALBUM_STAMP_IMG_GREYSCALE_OFF` / `ALBUM_STAMP_IMG_GRAYSCALE_OFF` | No arguments | Subsequent stamp images display in their original colors; default behavior. US spelling is accepted. | No imported-parser support observed. |
| `ALBUM_STAMP_IMG_HIDE` / `ALBUM_STAMP_IMG_SHOW` | No arguments | Hide/show subsequent stamp-box images. Does not affect page-background images; images are shown by default. | No imported-parser support observed. |
| `ALBUM_STAMP_SETTINGS_SAVE` / `ALBUM_STAMP_SETTINGS_RESTORE` | No arguments | Save/restore stamp drawing settings, e.g. to temporarily change image or box settings for a map. | Ordinary TXT generation emits these in one generated sequence. Imported parser does not accept them. Nesting/depth and unmatched-command behavior remain open. |

The help explicitly lists the saved values: both image-setting commands;
aspect-ratio mode; image hide/show and grayscale on/off; stamp-box size
adjustment; outer/inner border styles and inner border; heading padding;
footer-text padding/alignment; and stamp border, inner-border, heading, text,
and background colors. The reference does not say whether saving is nested or
single-level; do not infer that from the example.

**AlbumEasy runtime evidence (#89):** The
[save/restore reference](../../tests/fixtures/albumeasy/stamp-settings-save-restore-v1/README.md)
now verifies two sequential save/restore cycles. Its negative probe rejects
a second save before restoration and a restore without saved state:
single outstanding snapshot, consumed by restore, not a nested stack.
AlbumEasy reports these errors and produces no PDF despite launcher exit 0.
After maintainer acceptance, both commands now have bounded standalone
TXT/raw native mappings: image options, size adjustment, outline/background,
border/text/heading colours are captured per stamp and restored from a
consumed single snapshot. Native-language commands capture its mutable
image/outline/background policies; global frame/text styling stays fixed.
All three input routes have complete-page proofs and failure preservation.
Denmark v4 restores both commands, leaving margin-text colour deferred.
This does not update the conservative production-support labels above or
certify native-format frame/caption parity with the reference.
Inner-border/heading/footer controls in the saved-state checklist still need
their own mapping/proof coverage.

## 5. Stamp border, heading, and footer controls

| Command | 4.5.0 grammar | Documented behavior | Application audit |
|---|---|---|---|
| `STAMP_BORDER_STYLE` | `(style [options...])` | Styles: `SOLID`, `DOTTED`, `DOT_DASH`, `DASHED`, `BLANK`, `CORNER`, `SIDES`. Default is `SOLID`; a style persists for succeeding boxes until changed. | Not accepted by imported parser. |
| `STAMP_INNER_BORDER` | `(padding border)` | Adds an inner frame until cancelled with `(0.0 0.0)`. Applies only to rectangular boxes with borders and has no effect on rotated boxes. | Not accepted by imported parser. |
| `STAMP_INNER_BORDER_STYLE` | `(style [options...])` | Sets inner-frame style using the same documented style family. | Not accepted by imported parser. |
| `STAMP_BOXES_SIZE_ADJUST` | `(adjust)` | Signed size adjustment applies to succeeding stamp boxes; positive increases and negative decreases their size. | Observed in generated TXT; parser accepts one numeric argument; reflow reads only the final global adjustment, not ordered state. |
| `STAMP_HEADING` | `(font fontsize text [offset])` | Attach immediately after a stamp-add command. Aligns each heading vertically relative to its own stamp top; offset shifts it horizontally from the centered position. | Ordinary generator emits `STAMP_HEADING_VA` instead; neither is accepted by imported parser. |
| `STAMP_HEADING_VA` | `(font fontsize text [offset])` | Aligns headings vertically with one another; accepts the same optional horizontal offset. | Observed in ordinary TXT generation; not accepted by imported parser. |
| `STAMP_HEADING_PADDING` | `(padding)` | Signed adjustment to heading/frame gap for succeeding boxes; zero restores the default. | Not accepted by imported parser. |
| `STAMP_FOOTER_TXT_ALIGN` | `(align)` | `ALIGN` (default) places left/right text at the corresponding box corners. `CENTRE` left/right-aligns the first row and centers later rows under it; `CNR_CENTRE` centers later rows under the corresponding box corners. US aliases `CENTER` and `CNR_CENTER` are accepted. | Not accepted by imported parser. |
| `STAMP_FOOTER_TXT_PAD` | `(padding)` | Signed adjustment to gap between frame and first caption line; zero restores default. Subsequent-line spacing uses `TEXT_LINE_LEADING`. | Not accepted by imported parser. |

`CORNER` accepts optional `TL`, `TR`, `BL`, and/or `BR` selectors; omitting
them draws all four corners. `SIDES` accepts optional `L`, `R`, `T`, and/or `B`;
omitting them draws no sides (equivalent to `BLANK`), while naming all four
restores the full border. `SIDES` is independent of ordinary border styles and
is not reset when a new style is selected; it does not affect corner markers.
`CORNER` and `SIDES` apply only to rectangular boxes. These rules are verified
in the 4.5.0 help, but parser/reflow support is not established.

## 6. Legacy commands and version reconciliation

The pinned 4.5.0 help marks these commands obsolete:

| Command | 4.5.0 syntax/status | Documented replacement or behavior |
|---|---|---|
| `STAMP_ADDX` | `(width height txt0 txt1 txt2 txtBL txtBC txtBR txt1BL txt1BC txt1BR)` | Three text items inside and two rows of three below; use multiline `STAMP_ADD` instead. |
| `STAMP_ADDX_IMG` | `(width height image txtBL txtBC txtBR txt1BL txt1BC txt1BR [NO_FRAME])` | Two caption rows; use multiline `STAMP_ADD_IMG` instead. |
| `ALBUM_STAMP_IMG_SETTING` | `(horizontal vertical stretch)` | Retained for compatibility; values historically mean 0.35 mm per unit. Superseded by `ALBUM_STAMP_IMG_NEW_SETTING`. |
| `ALBUM_STAMP_NEW_IMG_SETTING` | `(horizontal [vertical] [stretch])` | Functionally identical to the preferred `ALBUM_STAMP_IMG_NEW_SETTING`; obsolete spelling. |
| `STAMP_BOXES_ADJUST_SAVE` / `STAMP_BOXES_ADJUST_RESTORE` | No arguments | Retained for compatibility to temporarily disable/re-enable `STAMP_BOXES_SIZE_ADJUST`; use full `ALBUM_STAMP_SETTINGS_SAVE`/`RESTORE` for broader state. |

`STAMP_ADD_BLANK_IMG` occurs in the legacy application validator but is not
listed in the inspected 4.5.0 command help. Its origin and status are open; do
not present it as a documented 4.5.0 command.

## 7. Static application-support audit

This section records source and existing-test evidence only; it is not
AlbumEasy runtime conformance testing.

| Surface | Verified repository evidence | Status and limits |
|---|---|---|
| Ordinary TXT stamp emission | [`emit_stamp_lines`, `emit_stamp`, and `emit_image_only`](../../src/album_globals/generator_new/emit.rs#L175) emit rectangular `STAMP_ADD`/basic `STAMP_ADD_IMG`, optional `NO_FRAME` for image-only cover elements, and `STAMP_HEADING_VA`. [`globals.rs`](../../src/album_utils/globals.rs#L390) emits `ALBUM_STAMP_IMG_NEW_SETTING`, `STAMP_BOXES_SIZE_ADJUST`, `ALBUM_STAMP_IMG_ASPECT_RATIO`, stamp colors, and cover-image `ALBUM_STAMP_SETTINGS_SAVE`/`RESTORE`. | Partial generation support. No inspected ordinary path emits the documented polygon/oval variants or combined image/inside-text overload. Generated command presence does not verify output under AlbumEasy. |
| Import parser and validator | [`validate`](../../src/external_album/parser.rs#L23) recognizes `STAMP_ADD_IMG` at 6–7 arguments, `STAMP_ADD` and `STAMP_ADD_BLANK` through broad 0–10 placeholder ranges, the undocumented `STAMP_ADD_BLANK_IMG` placeholder, `STAMP_BOXES_SIZE_ADJUST`, and legacy `ALBUM_STAMP_IMG_SETTING`. The optional seventh image argument is not checked: a non-`NO_FRAME` value is explicitly allowed as a placeholder. Other 4.5.0 stamp shapes, headings, border/image controls, and save/restore commands are not accepted. | Partial recognition, not complete grammar. In particular, argument typing for the broad placeholders is absent; the seven-argument image form is not the documented combined overload; and `STAMP_ADD_BLANK_IMG` has no 4.5.0 help evidence. |
| Import page validation and reflow | [`validate_page`](../../src/external_album/parser.rs#L142) allows image, text, row, and placeholder stamp commands. [`reflow`](../../src/external_album/reflow.rs#L12) lays out only `STAMP_ADD_IMG`: it uses the final global `STAMP_BOXES_SIZE_ADJUST`, counts caption lines, and wraps those images into rows. Other stamp commands are copied as non-image page commands without consuming measured stamp width/height. | Partial. Shape geometry, blank stamp footprints, `STAMP_ADD` text footprints, ordered adjustments, image-setting padding/aspect/grayscale/hide state, border state, headings, and footer alignment are not interpreted by this reflow path. |
| Imported image assets | [`asset_errors` and `uploaded_image`](../../src/external_album.rs#L286) check referenced `STAMP_ADD_IMG` paths and recognize `imported-assets/` uploads. The UI/API accepts PNG/JPEG uploads with file-size and image-decode limits ([`external_album_api.rs`](../../src/review_ui/external_album_api.rs#L287)); the original referenced asset check is an existence/path check, not a complete format-conformance test. | Narrow image-path/upload support only. No evidence of image controls (fit/stretch, aspect ratio, grayscale, visibility) being applied by the application reflow/editor. |
| Review UI editing | [`external_album.js`](../../src/review_ui/views/external_album.js#L230) renders existing command arguments as generic text fields. Its add controls provide only basic `STAMP_ADD_IMG`, centered text, and `ROW_START_FS` defaults ([`external_album.html`](../../src/review_ui/views/external_album.html#L86)). | Partial. It can edit retained arguments but exposes no typed shape/heading/style controls and cannot create the extended image overload from the add-stamp form. |
| Imported serialization/export | [`export_layout`](../../src/external_album.rs#L1398) exports the saved command model and assets after validation; [`serialize`](../../src/external_album/parser.rs#L370) validates again and writes normalized commands. | This is not arbitrary AlbumEasy pass-through: unsupported commands are rejected by parsing/validation, and comments/formatting are not preserved. Exported TXT is not proof of semantic parity. |
| Native PDF | Native stamp rendering accepts typed proof settings directly; [`StampImageProofSettings`](../../src/native_pdf/stamp_image.rs#L42) explicitly says its proof policy is not yet a production AlbumEasy command mapping. | Proof/layout capability only. Not an interpreter for these commands and not evidence for the corresponding command semantics. |
| Database stamp lookup | The attached [`get_stamps_by_country_series` API page](../functions/crates/stamp_album_db/src/get_stamps_by_country_series.html) documents retrieval of stamp records; the current implementation selects database rows ([`lib.rs`](../../crates/stamp_album_db/src/lib.rs#L1183)). | Upstream data access, not AlbumEasy command parsing, export, or native-rendering support. Keep the two evidence layers separate. |
| Tests | The `emit.rs` unit test checks generated strings and `STAMP_HEADING_VA`; [`test_album_stamp_img_new_setting_affects_output`](../../tests/album_stamp_img_integration_test.rs#L13) currently runs `cargo run -- --help`, not an album generation. The related format/parameter tests inspect source strings ([`album_stamp_img_new_setting_usage.rs`](../../tests/album_stamp_img_new_setting_usage.rs#L12)). Imported-backend tests use a fake shell renderer that writes a synthetic PDF and appends the TXT ([`external_album_backend.rs`](../../tests/external_album_backend.rs#L53), [round-trip test](../../tests/external_album_backend.rs#L213)). | These tests cover code shape and imported workflow mechanics, not real AlbumEasy 4.5.0 rendering/conformance. No command-family external-renderer fixture was found in the inspected tests. |

The [4.2.1 application-gap report](../reports/ALBUMEASY_4_2_1_APPLICATION_GAPS.md)
is useful historical context but predates this 4.5.0 help audit. Its old
coverage counts and classifications must not be treated as a current test
result.

## 8. Conformance fixture IDs and status

The first two isolated v1 fixtures have been created and reference-rendered with
AlbumEasy 4.5.0. Their inputs and outputs are retained under
[`tests/fixtures/albumeasy`](../../tests/fixtures/albumeasy), and the run evidence
is in [ALBUMEASY_STAMP_COMMANDS_V1.md](../testing/ALBUMEASY_STAMP_COMMANDS_V1.md).
These are reference runs, not approved native-PDF golden baselines.

| Fixture ID | Scope and status |
|---|---|
| `stamp-shapes-v1` | Reference-rendered. Covers the documented shape types, representative polygon orientations, flattened triangle apexes, a blank box, empty text fields, and modern/legacy `STAMP_ADD` forms. Every shape type appears, but not every right-triangle selector combination or multiline text. |
| `stamp-image-overloads-v1` | Reference-rendered. Covers basic PNG/JPEG forms, a basic `NO_FRAME` case, combined inside text, signed vertical offsets, the optional horizontal-offset pair, and representative framed/unframed cases. It does not cover the full independent optional-argument cross-product or missing/invalid assets. |
| `stamp-image-controls-v1` | AlbumEasy 4.5.0 first pass rendered from AlbumEasy-only commands on a complete bordered US Letter page. Covers omitted defaults, millimetre/percent padding, small-image stretch, aspect ratio, grayscale, and hide/show. Native rendering/comparison is gated on maintainer acceptance; page-background image is deferred to a separate realistic case. See the fixture README. |
| `stamp-frame-state-v1` | Outer/inner styles, all style values, `CORNER` and `SIDES` selectors, style interaction/persistence, and rectangular/non-rectangular restrictions. |
| `stamp-headings-captions-v1` | Both heading variants, immediate association, optional offsets, multiline text, padding resets, footer alignment aliases and footer padding. |
| `stamp-settings-save-restore-v1` | Each documented saved setting plus repeated, unmatched and nested save/restore probes; keep unspecified outcomes open until measured. |
| `stamp-box-adjustments-v1` | Positive/negative ordered box adjustment, changes around image and non-image stamps, and deprecated adjustment save/restore. |
| `stamp-obsolete-commands-v1` | `STAMP_ADDX`, `STAMP_ADDX_IMG`, obsolete image-setting spellings/units, and a negative/unknown-command check for `STAMP_ADD_BLANK_IMG`. |

Record the AlbumEasy executable/help hashes, command line, diagnostics, output
PDF/text extraction, and relevant geometry for each reference run. Do not
create or update golden PDFs until the reference output has been reviewed.

## 9. Next action and exit conditions

**Reference-verification checkpoint:** Complete for the stamp command family.
The installed 4.5.0 QCH source and SHA-256, extracted-help hash, section
locators, exact syntax, documented defaults, relevant state rules, and obsolete
commands are recorded above. The extracted text was read-only and no full
manual/help content was copied into the repository.

**Support-audit checkpoint:** Complete as a static source/test review for the
surfaces listed above. The matrix now distinguishes ordinary TXT emission,
import validation, reflow, uploaded assets, UI editing, serialization/export,
native proof rendering, and the kinds of evidence existing tests provide.

**Reference-fixture checkpoint:** The two initial stamp fixtures have been
created, rendered, and inspected against the installed AlbumEasy 4.5.0
reference. This is representative v1 evidence; the uncovered cases in section
8 remain open. No production integration was performed.

**Next action:** Complete actual-size human review of the AlbumEasy
`stamp-image-controls-v1` first pass. Only after acceptance, create the
matching native rendition and compare it. Keep renderer-specific unknowns,
especially save/restore nesting, explicit until measured.

This stamp-family pass is complete only when:

- every listed stamp command, overload, alias, and deprecated spelling has
  verified 4.5.0 syntax and source citations, or a clearly recorded unresolved
  status; **this documentation requirement is now met for the commands in
  scope**;
- 4.2.1 differences and extensions remain separately identifiable;
- application support is attributed to the correct surface and tested evidence;
- fixture requirements cover each grammar variant and state interaction; and
- the maintainer has reviewed the completed family before implementation.

No part of these conditions authorizes switching production PDF generation to
the native backend. That remains deferred until the full command language has
been mapped and compatibility reviewed.
