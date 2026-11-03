# AlbumEasy 4.2.1: application and configuration gap analysis

**Reviewed:** 2026-11-03
**Scope:** Static planning audit; no application/configuration changes, rendering
tests, or compatibility certification.

Reference: [command inventory](../guides/ALBUMEASY_4_2_1_COMMAND_INVENTORY.md)
and [native PDF plan](NATIVE_PDF_IMPLEMENTATION_PLAN.md).
The supplied [manual](../AlbumEasy_4.2.1.pdf) defines the compatibility target.

**Version update (2026-11-03):** The maintainer selected installed **4.5.0** as
the primary target, retaining 4.2.1 legacy coverage. The counts/matrix below
remain a 4.2.1-scoped snapshot. See the
[4.5.0 reference update](../guides/ALBUMEASY_4_5_0_REFERENCE_UPDATE.md) for verified
corrections: GAP-A's preferred emitted spelling is valid; GAP-B's commands and
GAP-D's sixth paragraph argument are documented in 4.5.0. They are **not**
established bugs. GAP-C's configuration-semantic distinction still needs work.
Do not delete those valid features to match the older manual.

## 1. What we have versus what we need

| Surface | What exists | What is missing for replacement |
|---------|-------------|--------------------------------|
| Ordinary albums | Database selection, stamp sizing, row/page packing, captions, headings, cover/Wikipedia pages and AlbumEasy TXT emission. | An authoritative backend-neutral document and native drawing/font/layout implementation. Existing layout estimates are not PDF rendering. |
| PDF creation | External AlbumEasy invocation, diagnostics, fresh-file checks and TXT/PDF artifact results. | Internal album PDF generation; per-output compatibility results and selectable native backend. |
| PDF library | `oxidize-pdf` image extraction in the PDF importer. | Proven album-generation API, font metrics/embedding, shapes, images, Unicode and print-fidelity evidence. |
| Imported albums | Original source/assets, database-backed commands, revision checks, draft/ready states, manual approval and saved-command TXT export. | Full grammar/state interpreter and standalone validation/rendering; current approval/edit regeneration still needs external AlbumEasy. |
| Imported editor | Generic argument editing, move/remove controls; add image stamp, centered text and fixed row. | Typed creation/editing and layout behavior for the entire command language. Generic fields do not implement a command. |
| Configuration | Sectioned global config, defaults, album-local overrides, configuration editor, templates and reference documentation. | Versioned mapping into a shared document, missing feature controls, ambiguity resolution and snapshot/precedence rules for native rendering. |
| Tests | Generation/config regressions and imported lifecycle tests; some mock renderers, opt-in real AlbumEasy checks. | Per-command/overload/interactions conformance, native PDFs and real geometry/print checks on Windows/Linux. |

### Measured name coverage — not a support percentage

Of the **92 canonical documented names**:

- **24** appear in the imported-command validator.
- **18** have specific argument ranges/types, but still have semantic/variant gaps.
- **6** use broad placeholder ranges: `PAGE_START_VAR`, `STAMP_ADD`,
  `STAMP_ADD_BLANK`, `COLOUR_STAMP_BORDER`, `ALBUM_PAGES_DECORATIVE_BORDER`,
  `ALBUM_PAGES_FOOTER_PAD`.
- **68** are absent from that validator, including all 11 border-file commands.
- None of the 12 additional `COLOR_` aliases is accepted by the current
  validator. Uppercasing a name does not normalize `COLOR_` to `COLOUR_`.
- `STAMP_ADD_BLANK_IMG` is an additional placeholder outside the manual inventory.

**No command is certified here as natively rendered.** Current generated TXT
can express more than the importer accepts, while the importer accepts some
commands ordinary generation does not emit. Neither set proves external-free
support.

## 2. Evidence map

Use these source references with the matrix below; line numbers refer to the
reviewed working tree, not future refactors.

| ID | Source / evidence |
|----|-------------------|
| S1 | [Output orchestration](../../src/album_globals/mod.rs#L103): external `Command`, `-c`, diagnostics and fresh PDF checks; [artifact dispatch](../../src/album_globals/mod.rs#L267). |
| S2 | [Canonical global writer](../../src/album_utils/globals.rs#L244): size, margins, border, header, spacing, colours, footer numbers, image settings and box adjustment. |
| S3 | [Generation fallback stub](../../src/album_globals/generator_new/mod.rs#L17), [active generator](../../src/album_globals/generator.rs#L1129), [stamp emission](../../src/album_globals/generator_new/emit.rs#L302). |
| S4 | [Row helper](../../src/album_globals/generator_new/helpers.rs#L47), [text templates](../../src/album_globals/generator_new/helpers.rs#L111), [page/layout helpers](../../src/album_globals/generator_new/layout.rs#L1), [commemorative output](../../src/album_globals/commemorative_generator.rs#L350). |
| S5 | [Imported validation](../../src/external_album/parser.rs#L23), [page validation](../../src/external_album/parser.rs#L141), [lexer/serializer](../../src/external_album/parser.rs#L210). |
| S6 | [Imported reflow](../../src/external_album/reflow.rs#L11): fixed-row image placements and estimated centered-text height, not a general command interpreter. |
| S7 | [Asset checks](../../src/external_album.rs#L285), [inspection](../../src/external_album.rs#L395), [reference/reconstructed external renders](../../src/external_album.rs#L948), [export/generation](../../src/external_album.rs#L1398). |
| S8 | [Editor argument fields](../../src/review_ui/views/external_album.js#L230), [three add-command defaults](../../src/review_ui/views/external_album.js#L277), [UI controls](../../src/review_ui/views/external_album.html#L86). |
| S9 | [Config schema](../../src/config.rs#L492), [global loading](../../src/config.rs#L1757), [section promotion](../../src/config.rs#L1879), [effective context](../../src/config.rs#L2588), [album overrides](../../src/config.rs#L2636). |
| S10 | [Template-driven config editor](../../src/review_ui/config_editor.rs#L519), [template](../../config_template.toml), [full example](../../config_full_example.toml), [config parameter inventory](../guides/CONFIG_PARAMETERS_INVENTORY.md). |
| S11 | [Imported lifecycle/mock renderer tests](../../tests/external_album_backend.rs#L7), [real-render opt-in](../../tests/external_album_backend.rs#L881), [source-string image-setting test](../../tests/album_stamp_img_integration_test.rs#L69). |
| S12 | [PDF image extraction](../../src/review_ui/pdf_parser/extraction.rs#L2): current `oxidize-pdf` usage, not album PDF creation. |

## 3. Command-by-command gap matrix

**TXT column:** `Emit` = specific existing output path; `Helper` = helper/template
or specialized path, not universal support; `Comment` = only a commented example
in the reviewed global writer; `—` = no implementation found in the reviewed
ordinary-generation modules. Arbitrary user-supplied text templates can contain
commands; that is not evidence of typed support.

**Import column:** `Limited` = validator has specific argument rules;
`Placeholder` = broad `0..10` range; `—` = absent from validator.
All `Limited` rows remain partial until tested against the reference.
Every row still needs native rendering and conformance evidence.

### 3.1 Album settings and metadata — 22

| Command | TXT | Import | Gap / relevant config |
|---------|-----|--------|-----------------------|
| `ALBUM_PAGES_SIZE` | Emit S2 | Limited | K1; native geometry and bounds. |
| `ALBUM_PAGES_MARGINS` | Emit S2 | Limited | K1; use identical dimensions in fit and draw. |
| `ALBUM_PAGES_MARGINSE` | — | Limited | S6 reads even-page margins; no dedicated ordinary config/emitter found. |
| `ALBUM_PAGES_SPACING` | Emit S2 | Limited | K1; inset/element gap, not multi-up page separation. |
| `ALBUM_PAGES_TITLE` | — | Limited | S2 emits `HEADER` instead; K2 naming/meaning must be disentangled. |
| `ALBUM_PAGES_BORDER` | Emit S2 | Limited | K1; three/four-argument variants, border geometry. |
| `ALBUM_PAGES_DECORATIVE_BORDER` | — | Placeholder | No linked border-language interpreter/resource resolver; K11. |
| `ALBUM_PAGES_HEADER` | Emit S2 | — | K2; app-generated header is rejected on import. |
| `ALBUM_PAGES_HEADER_NUMBER` | — | — | Add independent header numbering controls/state. |
| `ALBUM_PAGES_HEADER_DATE` | — | — | Date format/time policy and header slots. |
| `ALBUM_PAGES_FOOTER` | — | — | Independent footer text slots, not just numbers. |
| `ALBUM_PAGES_FOOTER_NUMBER` | Emit S2 | — | K3; app-generated footer numbering is rejected on import. |
| `ALBUM_PAGES_FOOTER_DATE` | — | — | Footer generation-date settings/state. |
| `ALBUM_PAGES_HEADER_PAD` | — | — | Add explicit margin-header padding semantics. |
| `ALBUM_PAGES_FOOTER_PAD` | — | Placeholder | No typed parameter or measured footer layout. |
| `ALBUM_AUTHOR` | — | — | PDF metadata settings distinct from printed text. |
| `ALBUM_TITLE` | — | — | PDF metadata title distinct from album/page headings. |
| `ALBUM_STAMP_NEW_IMG_SETTING` | — | — | K4; writer emits differently ordered name; resolve GAP-A. |
| `ALBUM_PAGES_BORDER3` | Comment S2 | — | Obsolete alias; retain syntax on import, normalize deliberately. |
| `ALBUM_PAGES_NUMBER` | Comment S2 | Limited | Legacy footer number; extra accepted positions need reference evidence. |
| `ALBUM_PAGES_DATE` | — | — | Obsolete footer date alias. |
| `ALBUM_STAMP_IMG_SETTING` | — | Limited | Legacy padding retained as arguments, not interpreted as native 0.35 mm geometry. |

### 3.2 Pages — 18

| Command | TXT | Import | Gap / relevant config |
|---------|-----|--------|-----------------------|
| `PAGE_START` | Emit S4 | Limited | Zero-argument validation only; group arguments missing. |
| `PAGE_START_VAR` | — | Placeholder | Parser only splits pages on exact `PAGE_START`; no variable-spacing page semantics. |
| `PAGE_COLUMN_START` | — | — | Column state, gap, bounding area and editor controls. |
| `PAGE_COLUMN_NEXT` | — | — | Second-column state and cursor. |
| `PAGE_COLUMN_STOP` | — | — | Restore full-width flow correctly. |
| `PAGE_TEXT` | Helper S4 | — | K7; native wrapping and left alignment. |
| `PAGE_TEXT_CENTRE` | Emit S3/S4 | Limited | K7; negative following spacing rejected; max arity too broad. |
| `PAGE_TEXT_RIGHT` | Helper S4 | — | K7; template exists, not general import support. |
| `PAGE_TEXT_PARAGRAPH_START` | Emit S2 (Wikipedia) | — | K7; writer has an extra argument not in the reference signature (GAP-D). |
| `PAGE_TEXT_PARAGRAPH_END` | Emit S2 (Wikipedia) | — | Raw paragraph mode and multiline lexer missing. |
| `PAGE_RULE_H` | Helper S4 | — | Specialized output; typed rule width/inset/spacing needed. |
| `PAGE_VSPACE` | Emit S4 | — | K5; signed movement, page-fit integration and import. |
| `PAGE_START_GROUP_BEGIN` | — | — | Block capture, scope, command expansion, substitution. |
| `PAGE_START_GROUP_END` | — | — | Block validation and repeated-page composition. |
| `PAGE_BACKGROUND_IMG` | — | — | Background layer, scale/offsets and asset discovery. |
| `PAGE_TEXT_VSPACE` | — | — | Obsolete left-text alias. |
| `PAGE_TEXT_CENTRE_VSPACE` | — | — | Obsolete centered-text alias. |
| `PAGE_TEXT_RIGHT_VSPACE` | — | — | Obsolete right-text alias. |

### 3.3 Rows — 6

| Command | TXT | Import | Gap / relevant config |
|---------|-----|--------|-----------------------|
| `ROW_START_ES` | Helper S4 | Limited | K5; imported reflow does not establish an ES image row. |
| `ROW_START_JS` | Helper S4 | Limited | K5; imported reflow does not establish a JS image row. |
| `ROW_START_FS` | Emit S3/S4 | Limited | K5; reflow handles this subset; no shared native measured placement. |
| `ROW_ALIGN_TOP` | Emit S2/S4 | Limited | Default hardcoded; native alignment state needed. |
| `ROW_ALIGN_MIDDLE` | — | Limited | Imported argument preserved; S6 does not model alignment geometry. |
| `ROW_ALIGN_BOTTOM` | — | Limited | Same; imported renderer currently supplies visual behavior. |

### 3.4 Stamps — 20

| Command | TXT | Import | Gap / relevant config |
|---------|-----|--------|-----------------------|
| `STAMP_ADD` | Emit S3 | Placeholder | K4/K6; typed modern/legacy text variants and layout. |
| `STAMP_ADD_DIAMOND` | — | — | Shape geometry, captions and editor shape choice. |
| `STAMP_ADD_OVAL` | — | — | Oval/circle geometry and text bounds. |
| `STAMP_ADD_TRIANGLE` | — | — | Triangle/trapezoid geometry and apex variant. |
| `STAMP_ADD_TRIANGLE_INV` | — | — | Inverted variants. |
| `STAMP_ADD_HEXAGON` | — | — | `SU`/`CU`, dimensions and geometry. |
| `STAMP_ADD_OCTAGON` | — | — | `SU`/`CU`, dimensions and geometry. |
| `STAMP_ADD_BLANK` | — | Placeholder | Must consume row width/height; current S6 measures images only. |
| `STAMP_ADD_IMG` | Emit S3 | Limited | K4; frame flag not enforced; inside-text/image syntax unresolved in manual. |
| `STAMP_HEADING` | — | — | Active helper emits `STAMP_HEADING_VA`, not this variant; need stamp association/offset semantics. |
| `STAMP_HEADING_VA` | Emit S3 | — | K6; generated heading cannot roundtrip through importer. |
| `STAMP_HEADING_PADDING` | — | — | Persistent signed padding setting. |
| `STAMP_INNER_BORDER` | — | — | Second frame, reset and shape restrictions. |
| `STAMP_BORDER_STYLE` | — | — | Styles/corner masks and shape fallback. |
| `STAMP_INNER_BORDER_STYLE` | — | — | Independent inner-style state. |
| `STAMP_BOXES_SIZE_ADJUST` | Emit S2 | Limited | K4; S6 reads final global adjustment, not full ordered state. |
| `STAMP_BOXES_ADJUST_SAVE` | — | — | Saved adjustment state and temporary disable. |
| `STAMP_BOXES_ADJUST_RESTORE` | — | — | Restore semantics/nesting decision. |
| `STAMP_ADDX` | — | — | Legacy text layout conversion. |
| `STAMP_ADDX_IMG` | — | — | Legacy image plus multirow-caption conversion. |

### 3.5 Fonts and text metrics — 3

| Command | TXT | Import | Gap / relevant config |
|---------|-----|--------|-----------------------|
| `ALBUM_DEFINE_FONT` | — | — | K10; font resolution/embedding/licensing; old encoding argument. |
| `TEXT_CHAR_SPACING` | — | — | `row_font_spacing` is not this command; state and capture rules missing. |
| `TEXT_LINE_LEADING` | — | — | `text_line_height_mm` is only an estimate, not the command's signed extra leading. |

### 3.6 Colours — 12

TXT colour output uses `COLOR_` aliases. The import counts below refer to the
canonical `COLOUR_` names; all 12 `COLOR_` spellings are absent from validation.

| Command | TXT | Import | Gap / relevant config |
|---------|-----|--------|-----------------------|
| `COLOUR_ALBUM_BORDER` | Emit S2 | — | K8; aliases, colour values and album scope. |
| `COLOUR_ALBUM_DECORATIVE_BORDER` | — | — | Border lines only; ornament images must not be recoloured implicitly. |
| `COLOUR_ALBUM_FOOTER` | Emit S2 | — | K8; album-scope text colour. |
| `COLOUR_ALBUM_HEADER` | Emit S2 | — | K8; album-scope text colour. |
| `COLOUR_ALBUM_TITLE` | Emit S2 | — | K8; visible title distinct from header. |
| `COLOUR_PAGE_RULE_H` | — | — | Independent persistent rule colour. |
| `COLOUR_PAGE_TEXT` | — | — | Independent persistent page-text colour. |
| `COLOUR_STAMP_BORDER` | Emit S2/S4 | Placeholder | K8; emitted alias rejected; canonical value/state not validated. |
| `COLOUR_STAMP_INNER_BORDER` | — | — | Independent inner-frame colour. |
| `COLOUR_STAMP_BACKGROUND` | Emit S2/S4 | — | K8; rectangle/frame restrictions. |
| `COLOUR_STAMP_HEADING` | Emit S2 | — | K8; independent heading colour. |
| `COLOUR_STAMP_TEXT` | Emit S2 | — | K8; inside/below text colour. |

### 3.7 Decorative-border definition files — 11

No separate border-language parser/layout/editor was found on the inspected
import/generation paths. Copying a source directory is not semantic support.

| Command | TXT | Import | Missing behavior |
|---------|-----|--------|------------------|
| `IMAGE_SCALE` | — | — | Ornament scale, K11. |
| `IMAGE_CORNER` | — | — | Asset lookup and per-corner transforms. |
| `IMAGE_CORNER_PAD` | — | — | Corner offsets. |
| `IMAGE_EDGE_LEFT` | — | — | Left ornament placement. |
| `IMAGE_EDGE_RIGHT` | — | — | Right ornament transforms. |
| `IMAGE_EDGE_TOP` | — | — | Top ornament placement. |
| `IMAGE_EDGE_BOTTOM` | — | — | Bottom ornament transforms. |
| `IMAGE_EDGE_PAD` | — | — | Edge offsets and documented-axis ambiguity. |
| `BORDER_LINES` | — | — | Up to three strokes and separation. |
| `BORDER_PAD` | — | — | Inward/outward border offsets. |
| `BORDER_END_OFFSETS` | — | — | Line-end clearances around ornaments. |

## 4. Cross-cutting gaps and contradictions

These are planning findings, not fixes made in this audit.

| ID | What exists / mismatch | Required next decision or work |
|----|------------------------|--------------------------------|
| GAP-A | S2 emits `ALBUM_STAMP_IMG_NEW_SETTING`; the manual documents `ALBUM_STAMP_NEW_IMG_SETTING`. The editor labels the latter, but the backend accepts neither. | Establish exact supported syntax with a reference case; choose canonical spelling and migration/export policy. Do not silently rename without checking historical files/versions. |
| GAP-B | S2 emits `ALBUM_STAMP_IMG_ASPECT_RATIO` and `COLOR_ALBUM_MARGIN_TXT`, absent from this manual's inventory. | Identify provenance/version or classify as explicit extensions; a config field is not proof of 4.2.1 compatibility. |
| GAP-C | `extra_width_mm`/`extra_height_mm` affect stamp sizing and are also emitted as image-setting arguments; the third argument comes from a field named “preserve aspect,” while the reference describes “stretch.” | Separate physical box allowance, image padding, stretch-up and aspect preservation. Define backward migration and measurable effects before native design. |
| GAP-D | S2's Wikipedia paragraph start emits six arguments (including right padding); the manual describes at most five. | Classify as a version extension or invalid target syntax using evidence, not assumptions. |
| GAP-E | S5's page splitting recognizes only `PAGE_START`; `PAGE_START_VAR` is merely accepted as a placeholder and cannot open its own validated page. | Implement grammar and true page-boundary behavior together. |
| GAP-F | Negative text spacing is documented but rejected for centered text. `STAMP_ADD_IMG` accepts an arbitrary seventh argument; rows are not fully sequence-validated. | Formalize arities, signed fields, flags and command ordering before treating import as validated. |
| GAP-G | S6 only opens image-row reflow on `ROW_START_FS`; ES/JS are accepted but not interpreted equivalently. Blank/text stamps and many settings are not measured. | Replace subset estimates with ordered, measured layout for all supported commands. |
| GAP-H | S7 checks named image dependencies only for `STAMP_ADD_IMG`; inspection reads UTF-8 via `read_to_string`. | Add background/border/font dependencies and explicit Windows-1252 decoding; preserve original source encoding/provenance. |
| GAP-I | Quoted strings/comments exist, but no complete continuation/paragraph/group interpreter. Re-serialization normalizes text rather than preserving comments/formatting. | Preserve original snapshots, implement semantic roundtrips, and state whether edited TXT retains comments/formatting. |
| GAP-J | Ordinary emission includes header/footer, colours and headings rejected by import. | Require generated TXT -> import -> edit -> export conformance; test defaults as well as optional features. |
| GAP-K | `generator_new` top-level functions return empty paths; working generation falls back to the original generator using selected new helpers. | Extract actual active behavior, not an assumed completed refactor. |
| GAP-L | External renders are required for imported approval and edit regeneration. No native album renderer/backend setting is wired in the inspected paths. | Add shared document/rendering, preserve revisions/approval, and certify with AlbumEasy absent. |

All AE-GAP-01 through AE-GAP-09 in the manual inventory remain open. Source
inspection alone cannot resolve undocumented external-program behavior.

## 5. Configuration is part of the compatibility contract

### 5.1 Files and precedence we already have

- [Local config.toml](../../config.toml): reviewed via parsed section/key names
  and selected non-sensitive layout values. No paths, credentials or arbitrary
  text values are reproduced here. Leave the local file unchanged/uncommitted.
- [config_template.toml](../../config_template.toml): normal setup/UI schema and
  defaults reference; [config_full_example.toml](../../config_full_example.toml):
  extended reference; [Config](../../src/config.rs#L492): actual typed schema.
- Global loading searches relative to the working directory and executable;
  absolute paths are exact. Known convenience sections are promoted into the
  flat `Config` fields, with sectioned values winning stale root values (S9).
- Album-local `config.toml` is deep-merged over the defaulted global config.
  Scoped effective-config loading enables this in generation callers. CLI/action
  overrides, such as a PDF checkbox, can override particular settings afterward;
  there is no universal precedence rule proven for every setting.
- The config editor is template-driven and preserves additional fields (S10).
  Field visibility/persistence does not demonstrate renderer consumption.

**Native planning rule:** resolve one effective configuration snapshot before
layout. Both exporters use that snapshot/document revision. Global defaults
must not silently override an imported album's explicit commands. Decide how
user-applied overrides become editable document changes and trigger reapproval.

### 5.2 Existing keys mapped to behavior and remaining work

Sections below are the current local layout, not proposed new TOML syntax.
Key names are existing; absence below is not evidence a setting was removed.

| ID | Current section and keys | Existing behavior / gap |
|----|--------------------------|-------------------------|
| K1 | `[page]`: `album_page_width_mm`, `album_page_height_mm`, `page_*_margin_mm`, `page_border_outer`, `page_border_inner1`, `page_border_inner2`, `page_border_spacing`, `album_pages_spacing_h`, `album_pages_spacing_v`, `max_page_vspace_mm` | Emit size/margins/border/spacing and inform packing. Need one coordinate/unit model, even-page margin settings and measured border/content bounds. Template currently describes spacing as between pages/sheets, but AlbumEasy defines content inset/element spacing. |
| K2 | `[header]`: `album_pages_title_font`, `album_pages_title_size`, `album_pages_header_position`, `header_vspace_before_mm`, `header_vspace_after_mm`, `include_*` metadata toggles | “Title” font/size feed `ALBUM_PAGES_HEADER`; ordinary visible body headings have separate templates. Separate header, repeated title, first-page title and stamp/series heading semantics. Add left/right icon controls only after this distinction. |
| K3 | `[footer]`: `album_pages_footer_number_enabled`, `album_pages_footer_number_font`, `album_pages_footer_number_size`, `album_pages_footer_number_position`, `album_pages_footer_number_start`, `album_pages_footer_number_before`, `album_pages_footer_number_after` | Configurable footer numbers exist. Need text/date/header-number variants, independent slots, padding and total-page resolution. |
| K4 | `[stamps]`: `reference_width_mm`, `reference_height_mm`, `target_stamp_height_mm`, `stamp_box_adjust`, `extra_width_mm`, `extra_height_mm`, `album_stamp_img_new_setting_preserve_aspect`, `preserve_aspect`, `album_stamp_img_aspect_ratio`, `global_scale`; `[images]`: `force_dpi`, `sizing_mode`, `placeholder_always_overwrite` | Physical sizing/placeholder/image policy exists. Resolve GAP-A/B/C and alias precedence; do not make native stretching invert aspect preservation. Verify whether “global scale” applies to all documented geometry or only particular calculations. |
| K5 | `[stamps]`: `max_stamps_per_row`, `max_rows_per_page`, `packing_horizontal_safety_mm`, `packing_vertical_safety_mm`, `series_row_gap_mm`, `allow_single_large_stamp_per_page`, `single_stamp_threshold_pct`, `usable_height_padding_mm`; `[layout]`: `row_start_es_*`, `row_start_es_enabled`, `enable_row_centering`, `row_font_spacing`, `header_font_spacing`, `page_fit_conservative_buffer_mm`, `series_order` | Existing packing policy. In the shared row helper, `row_start_es_spacing` is the row border width and `row_start_es_vspace` is the FS horizontal gap, despite their names. `row_font_spacing` contributes to row height, not character tracking. Row mode/alignment need explicit typed policies; `row_align_default` is only a commented template suggestion. |
| K6 | `[stamps]`: `stamp_heading_font`, `stamp_heading_size`, `stamp_heading_label`, `default_catalog_code`, `stamp_detail_layout`, `stamp_detail_font_size`, `stamp_detail_vspace`, field-visibility toggles, `text_line_height_mm`, `image_spacing_mm` | Stamp captions/details/headings already configurable. Need true text metrics, heading offsets/padding/alignment and shape-aware bounds; do not replace data/detail policies with generic draw commands. |
| K7 | `[text_templates]`: `page_text_*`, `page_text_centre_*`, `page_text_right_*`, per-series/date/watermark/engraving overrides; `[wikipedia]`: paragraph/font/alignment/padding settings | Existing string templates and Wikipedia paragraphs. Parse or migrate configurable command templates into typed text styles; preserve content and intentional spacing, diagnose commands not representable natively. Add actual tracking/leading and paragraph state. |
| K8 | `[colors]`: `color_album_border`, `color_album_footer`, `color_album_header`, `color_album_title`, `color_album_margin_txt`, `color_stamp_border`, `color_stamp_heading`, `color_stamp_text`, `color_stamp_background` | Eight documented colour targets emitted via aliases, plus unverified margin-text extension. Need page text/rule, inner-border and decorative-border colour settings; exact named/hex conversion and scope. |
| K9 | `[flag_cover]`: `print_flag_cover`, `flag_cover_elements`, image/coat-of-arms dimensions, cover spacing/title styles/background | Existing cover composition is reusable input, not a general left/right repeated-title icon model. Preserve ordering, physical asset dimensions, relative assets and layout options. |
| K10 | Font tokens across header/footer/stamp/text config | Built-in font IDs are passed to AlbumEasy. Need portable font-family/style resolution, custom font definitions, glyph validation, embedding/substitution policy and license review. No typed custom-font collection was found in the inspected `Config`. |
| K11 | Decorative border/background/columns/shapes/inner borders | No complete typed configuration surfaces for these command families found. Decide which are album/document properties versus global defaults; do not add one root key per command blindly. |
| K12 | `[runtime]`: `albumeasy_cli_path`, `albumeasy_auto_generate_pdf`, `albumeasy_pdf_timeout_secs`, `pdf_dx_logging`; diagnostic settings | External rendering controls exist. Need backend/PDF-enable separation, native output/export policy and structured per-artifact errors; preserve old configs and make external execution optional at the final milestone. |

### 5.3 Safe local configuration baseline observed

This is a non-sensitive planning snapshot, not the canonical defaults for every
user or a reason to commit the local config:

| Aspect | Current local value |
|--------|---------------------|
| Page | 216.0 x 279.4 mm; all four margins 10.0 mm. |
| Border | Outer/inner1/inner2: 0.1/0.5/0.1 mm; gap 1.0 mm. |
| Album content spacing | Horizontal/vertical 3.0/3.0 mm. |
| Header/footer fonts | Header HN 10 pt, centered; footer HN 8 pt, centered, numbering enabled. |
| Stamp sizing | Reference 19 x 25 mm; target height 25 mm; extra width/height 3/3 mm; box adjustment 0. |
| Image sizing | `force_dpi = 96`, `sizing_mode = "fit_box"`; aspect-preserving toggles true. |
| Packing/detail mode | At most 5 stamps/row and 5 rows/page; `separate_rows`, detail font 6 pt. |
| Row helper inputs | Font HN 6; `row_start_es_spacing = 0.1`, `row_start_es_vspace = 6.0`; actual meaning described in K5. |
| External PDF | Auto-generation false; wait timeout 300 seconds; executable path intentionally omitted. |

Do not silently migrate these inputs to different physical results. In
particular, distinguish source-image DPI, actual printed size, mount allowances,
image padding and whole-layout scaling.

### 5.4 Configuration work needed before implementation approval

1. Define units/type/range/default/scope for every existing layout-related key.
2. Record precedence for global, album-local, imported commands, saved editor
   changes and per-run overrides. Include sectioned/legacy-root aliases.
3. Separate semantic values from historical names/command templates; document
   migrations without breaking existing configurations.
4. Decide new controls for native-only features, command families and export
   strictness. No proposed key is implemented by this report.
5. Wire future keys through `Config`, defaults, templates/full example,
   parameter inventory, UI, CLI where appropriate, and both exporters.
6. Preserve config-editor saves and imported-album revision/approval behavior.
   Never export credentials or unrelated source settings in compatibility reports.
7. Test equivalent effective configs from root/sectioned/album-local files;
   verify actual PDF geometry and TXT meaning, not only key presence.

## 6. Recommended planning order and acceptance evidence

| Priority | Bounded planning work | Completion evidence | Plan tasks |
|----------|-----------------------|---------------------|------------|
| 1 | Resolve command spelling/version discrepancies and image semantics. | Decisions/reference cases for GAP-A through GAP-D and AE-GAP-01/02/03. | T001, T036 |
| 2 | Finish config semantic/precedence map and imported-source authority policy. | Approved K1–K12 key/default/unit/scope map and migration decisions. | T046, T006–T008, T019 |
| 3 | Formalize lexer/state, rows/text/images and overloads; identify required primitives. | Per-command grammar, missing-behavior matrix and small conformance cases. | T036–T040 |
| 4 | Choose shared document and prove the PDF library against actual requirements. | Measurement/font/image/shape proof plus agreed missing-capability strategy. | T004–T010 |
| 5 | Plan feature-family increments and eventual standalone lifecycle. | Branch-sized tasks with config/UI/import/export/native evidence, then no-executable acceptance. | T014–T029, T041–T045 |

This order is a recommendation, not authorization to implement. Future
enhancements can be added after the compatibility/configuration contracts are
clear. Do not equate 24 recognized names with 24 completed tasks.

### Verification performed for this report

- Read active generation/global/helper, imported validator/reflow/lifecycle,
  editor, config loader/schema and relevant existing test sources.
- Parsed local/template/full-example TOML without modifying them.
- Matched the 92 inventory rows against validator names; checked 12 colour
  aliases separately and distinguished six placeholder entries.
- No Cargo/Node tests or external AlbumEasy runs were performed: this is a
  documentation-only static audit. Existing tests cited above were inspected,
  not reported as passing; mock PDF headers do not certify visual fidelity.
- No application/database writes or input relocation occurred. Documentation
  changes only; local credentials/paths are not reproduced.
