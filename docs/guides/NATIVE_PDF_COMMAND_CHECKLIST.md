# AlbumEasy and native PDF command checklist

**Updated:** 2026-11-09. **Tracking:** #102.
**References:** AlbumEasy **4.2.1** (legacy) and **4.5.0** (primary), not 4.3.
**Purpose:** one working checkoff list; update it after each command slice.
Production native-PDF integration remains deferred.

### Command keyword case (#105)

Standalone TXT, raw-command `.stampalbum` and native `STAMPALBUM 1` inputs
accept uppercase, lowercase and mixed-case **command keywords**, including
page-start group delimiters and the native version header. Normalize keywords
at parsing, not entire lines: captions, titles, paths, font/style arguments and
other values retain their original case and existing validation.
Unsupported commands still fail; case tolerance does not add unmapped commands.
The standalone example detects the native header case-insensitively.

**Future output rule (not implemented):** when integrated with the main app,
write command keywords in uppercase for consistency. Preserve all values,
visible text, paths and identifiers; do not uppercase album content.

Evidence: [case regression tests](../../tests/native_pdf_command_case.rs).
Tests render lowercase and mixed-case variants and compare decoded PDF content
streams exactly with uppercase baselines, including real lowercase image paths.
No reference input/PDF is rewritten and no production import/UI route changes.

Validation on Linux (2026-11-09), all commands exited 0:

```sh
cargo test --test native_pdf_command_case --test native_pdf_page_start_var --test native_pdf_page_start --test native_pdf_albumeasy --test native_pdf_album
cargo test --test native_pdf_command_case
cargo clippy --lib --test native_pdf_command_case --example render_stampalbum
cargo build --example render_stampalbum
cargo fmt --all -- --check
git diff --check
```

Outcome: **41 passed, 6 ignored, 0 failed** across the five suites; final
case-only rerun **2 passed** (including bare group delimiters). Clippy/build
passed with existing unrelated warnings. Six actual CLI invocations of
`target/debug/examples/render_stampalbum <input> <output.pdf>` covered
lowercase/mixed-case TXT, raw `.stampalbum` and native `.stampalbum`;
each returned four pages, exit 0. Exact resolved commands/results are retained
in the session execution log `command-case-cli.log`. PDF content streams match
uppercase baselines byte-for-byte; this is not a new visual-parity claim.

Filesystem impact: added the case regression test; updated the shared parser,
TXT admission, standalone example and directly related documentation. Existing
uncommitted #104 work is preserved. Temporary case inputs/PDFs were auto-cleaned;
Cargo build artifacts/logs are ignored or session-local. No inputs moved/deleted
and no source/reference PDFs changed. DB impact: none (no connections, schema
changes, inserts, updates or duplicates applicable). Final error inventory:
none. Logic risks addressed: admission before dispatch, bare group delimiters,
case-sensitive equality/suffix checks and native-header routing. Windows was
not executed; normalization uses portable ASCII/string and Path APIs.

## How to read and update this list

- **[x] Mapped:** a bounded standalone native mapping exists, with tests.
  This does **not** mean all overloads, production integration or print approval.
- **[~] Partial:** only part of the command works, or **L** = lower-level
  API/TOML proof only; the standalone TXT interpreter still lacks the command.
- **[ ] TODO:** no native mapping established. A reference PDF alone is not done.
- **Native** identifies the native document equivalent or lower-level surface;
  a different command spelling is not necessarily semantically identical.

Do not check a command just because its name appears in a parser. Verify its
arguments, defaults, units, state, ordering, layout and errors. Keep unresolved
variants in the notes. All rows still require the appropriate human/print and
Windows/Linux acceptance before full compatibility certification.

**Versions/counts:** section 1 lists all **92 canonical 4.2.1 names**, also
present in installed 4.5.0 help. Section 2 adds **29 canonical names/directives**
documented in 4.5.0 but absent from that legacy inventory. This gives **121**
canonical entries including seven preprocessing directives and eleven separate
border-definition commands. Section 3 records aliases: 104 documented spellings
for 4.2.1; 137 for this 4.5.0 inventory. Parameter variants are not new names.
`ROW_START` / `ROW_ALIGN` in help prose are family labels, not extra commands.

This is a current implementation inventory, **not** evidence that the program
was tested against a running 4.2.1 renderer.

## 1. Commands documented in both references

### Album/page settings — 22

| Check | AlbumEasy command | Native / remaining work | Evidence |
|---|---|---|---|
| [x] | `ALBUM_PAGES_SIZE` | Same native spelling; explicit dimensions. | [page-size](../../tests/fixtures/albumeasy/page-size-v1/) |
| [x] | `ALBUM_PAGES_MARGINS` | Same native spelling. | [margins](../../tests/fixtures/albumeasy/margins-v1/) |
| [~] L | `ALBUM_PAGES_MARGINSE` | API/TOML even margins; missing TXT/native-document mapping. BRA-03. | [page settings tests](../../tests/native_pdf_page_settings.rs) |
| [x] | `ALBUM_PAGES_SPACING` | Same native spelling; remaining interaction parity. | [spacing](../../tests/fixtures/albumeasy/spacing-v1/) |
| [x] | `ALBUM_PAGES_TITLE` | `NATIVE_PAGE_TITLE`; TXT following gap includes vertical spacing plus title extra (#103); native explicit gap is the total. Color/config parity remains bounded. | [title](../../tests/fixtures/albumeasy/title-v1/), [page-start proof](../../tests/fixtures/albumeasy/page-start-v1/) |
| [x] | `ALBUM_PAGES_BORDER` | Same native spelling; 3/4 numeric arguments. | [border](../../tests/fixtures/albumeasy/border-v1/) |
| [~] L | `ALBUM_PAGES_DECORATIVE_BORDER` | Definition parser/API/TOML works; neither album loader wired. CIS-01/02. | [decorative border](../../tests/fixtures/albumeasy/decorative-border-v1/) |
| [x] | `ALBUM_PAGES_HEADER` | `NATIVE_PAGE_HEADER`; full state/default grammar still needs reconciliation. | [header](../../tests/fixtures/albumeasy/header-v1/) |
| [~] L | `ALBUM_PAGES_HEADER_NUMBER` | API/TOML only; map TXT/native-document surface. | [header numbering](../../tests/fixtures/albumeasy/header-number-v1/) |
| [~] L | `ALBUM_PAGES_HEADER_DATE` | API/TOML only; map TXT/native-document surface. | [header date](../../tests/fixtures/albumeasy/header-date-v1/) |
| [~] L | `ALBUM_PAGES_FOOTER` | API/TOML only; plain repeating footer lacks loader mapping. | [footer](../../tests/fixtures/albumeasy/footer-v1/) |
| [x] | `ALBUM_PAGES_FOOTER_NUMBER` | `NATIVE_PAGE_FOOTER_NUMBER`; 3–6 TXT arguments, total count; native empty-affix issue. | [footer import](../../tests/fixtures/albumeasy/footer-numbering-import-v1/) |
| [~] L | `ALBUM_PAGES_FOOTER_DATE` | API/TOML only; map date command into loaders. | [footer date](../../tests/fixtures/albumeasy/footer-date-v1/) |
| [~] L | `ALBUM_PAGES_HEADER_PAD` | API/TOML padding exists; native header includes explicit gaps. TXT command missing. | [header tests](../../tests/native_pdf_page_header.rs) |
| [~] L | `ALBUM_PAGES_FOOTER_PAD` | API/TOML padding exists; native footer includes explicit gaps. TXT command missing. CIS-04. | [footer tests](../../tests/native_pdf_page_footer.rs) |
| [ ] | `ALBUM_AUTHOR` | Source-controlled PDF metadata mapping missing. | [legacy inventory](ALBUMEASY_4_2_1_COMMAND_INVENTORY.md) |
| [ ] | `ALBUM_TITLE` | PDF metadata, not visible title; mapping missing. | [legacy inventory](ALBUMEASY_4_2_1_COMMAND_INVENTORY.md) |
| [ ] | `ALBUM_STAMP_NEW_IMG_SETTING` | Old mm spelling; map explicitly to preferred image policy. | [version update](ALBUMEASY_4_5_0_REFERENCE_UPDATE.md) |
| [ ] | `ALBUM_PAGES_BORDER3` | Obsolete alias; do not infer support from ordinary border module. | [legacy inventory](ALBUMEASY_4_2_1_COMMAND_INVENTORY.md) |
| [ ] | `ALBUM_PAGES_NUMBER` | Obsolete alias; local tests adapt name, production interpreter rejects. BRA-02. | [TODO plan](../reports/NATIVE_PDF_IMPLEMENTATION_PLAN.md) |
| [ ] | `ALBUM_PAGES_DATE` | Obsolete footer-date alias; mapping missing. | [legacy inventory](ALBUMEASY_4_2_1_COMMAND_INVENTORY.md) |
| [ ] | `ALBUM_STAMP_IMG_SETTING` | Legacy **0.35 mm units**; local adaptation is not support. BRA-01. | [image matrix](ALBUMEASY_4_5_0_STAMP_COMMAND_MATRIX.md) |

### Page flow/text — 18

| Check | AlbumEasy command | Native / remaining work | Evidence |
|---|---|---|---|
| [x] | `PAGE_START` | TXT explicit starts and bounded group arguments/substitutions work; native `NATIVE_PAGE_BREAK` preserves explicit breaks and intentional empty pages. Native free page text and full group-body language remain open. | [page-start tests](../../tests/native_pdf_page_start.rs), [accepted reference / native proofs](../../tests/fixtures/albumeasy/page-start-v1/) |
| [x] | `PAGE_START_VAR` | #104: page-local nonnegative H/V, zero overrides, ordinary-start reset; two spacing arguments precede up to nine group values. Title retains global V; following content uses local V. Absolute font/row metrics and native acceptance remain open (VAR-01–04). | [reference / native proofs](../../tests/fixtures/albumeasy/page-start-var-v1/README.md) |
| [ ] | `PAGE_COLUMN_START` | Column widths/padding and flow missing. | [version update](ALBUMEASY_4_5_0_REFERENCE_UPDATE.md) |
| [ ] | `PAGE_COLUMN_NEXT` | Column flow missing. | [legacy inventory](ALBUMEASY_4_2_1_COMMAND_INVENTORY.md) |
| [ ] | `PAGE_COLUMN_STOP` | Column flow missing. | [legacy inventory](ALBUMEASY_4_2_1_COMMAND_INVENTORY.md) |
| [ ] | `PAGE_TEXT` | Free-standing left text not mapped. | [legacy inventory](ALBUMEASY_4_2_1_COMMAND_INVENTORY.md) |
| [~] | `PAGE_TEXT_CENTRE` | TXT explicit multiline works; automatic wrap/empty spacers fail; no equivalent native free text. BRA-05/CIS-03. | [compatibility code](../../src/native_pdf/albumeasy.rs) |
| [ ] | `PAGE_TEXT_RIGHT` | Free-standing right text missing. | [legacy inventory](ALBUMEASY_4_2_1_COMMAND_INVENTORY.md) |
| [ ] | `PAGE_TEXT_PARAGRAPH_START` | Paragraph mode, alignments, independent right padding missing. | [version update](ALBUMEASY_4_5_0_REFERENCE_UPDATE.md) |
| [ ] | `PAGE_TEXT_PARAGRAPH_END` | Paragraph mode missing. | [legacy inventory](ALBUMEASY_4_2_1_COMMAND_INVENTORY.md) |
| [ ] | `PAGE_RULE_H` | Rule geometry, color and spacing missing. | [legacy inventory](ALBUMEASY_4_2_1_COMMAND_INVENTORY.md) |
| [~] | `PAGE_VSPACE` | Signed TXT cursor movement supported within bounds; native free-space mapping missing. | [compatibility code](../../src/native_pdf/albumeasy.rs) |
| [x] | `PAGE_START_GROUP_BEGIN` | One pre-page group, existing supported body commands, quote-preserving single-pass `$$1`–`$$9`; no nested/recursive starts or unsupported-body approximation. Missing values fail. | [page-start proof](../../tests/fixtures/albumeasy/page-start-v1/) |
| [x] | `PAGE_START_GROUP_END` | Validates matching close, declaration order and duplicate definitions. PAGE_START_VAR shares bounded expansion; PAGE_START_INFO remains open. | [page-start tests](../../tests/native_pdf_page_start.rs) |
| [~] | `PAGE_BACKGROUND_IMG` | Native explicit bounds via `NATIVE_PAGE_BACKGROUND_IMAGE`; TXT fit/scale/offset mapping missing. | [image-control tests](../../tests/native_pdf_stamp_image.rs) |
| [ ] | `PAGE_TEXT_VSPACE` | Obsolete left-text alias missing. | [legacy inventory](ALBUMEASY_4_2_1_COMMAND_INVENTORY.md) |
| [ ] | `PAGE_TEXT_CENTRE_VSPACE` | Obsolete centered-text alias missing. | [legacy inventory](ALBUMEASY_4_2_1_COMMAND_INVENTORY.md) |
| [ ] | `PAGE_TEXT_RIGHT_VSPACE` | Obsolete right-text alias missing. | [legacy inventory](ALBUMEASY_4_2_1_COMMAND_INVENTORY.md) |

### Rows — 6

| Check | AlbumEasy command | Native / remaining work | Evidence |
|---|---|---|---|
| [~] | `ROW_START_ES` | TXT three arguments mapped; Australia fourth argument unresolved. Native automatic rows are not this grammar. | [compatibility tests](../../tests/native_pdf_albumeasy.rs) |
| [ ] | `ROW_START_JS` | Justified gaps touching outer row bounds missing. | [legacy inventory](ALBUMEASY_4_2_1_COMMAND_INVENTORY.md) |
| [x] | `ROW_START_FS` | TXT fixed-gap rows mapped; native row distribution differs; rotated row variants remain open. | [row tests](../../tests/native_pdf_stamp_rows.rs) |
| [x] | `ROW_ALIGN_TOP` | `NATIVE_ROW_VERTICAL_ALIGNMENT (TOP)`; mixed-case aliases unverified. | [heading tests](../../tests/native_pdf_stamp_headings.rs) |
| [x] | `ROW_ALIGN_MIDDLE` | `NATIVE_ROW_VERTICAL_ALIGNMENT (MIDDLE)`. | [heading tests](../../tests/native_pdf_stamp_headings.rs) |
| [x] | `ROW_ALIGN_BOTTOM` | `NATIVE_ROW_VERTICAL_ALIGNMENT (BOTTOM)`. | [heading tests](../../tests/native_pdf_stamp_headings.rs) |

### Stamps — 20

| Check | AlbumEasy command | Native / remaining work | Evidence |
|---|---|---|---|
| [ ] | `STAMP_ADD` | Text-only rectangular command not mapped; frame geometry APIs do not establish support. | [shape reference only](../../tests/fixtures/albumeasy/stamp-shapes-v1/) |
| [ ] | `STAMP_ADD_DIAMOND` | Shape/captions missing. | [shape reference only](../../tests/fixtures/albumeasy/stamp-shapes-v1/) |
| [ ] | `STAMP_ADD_OVAL` | Oval/circle missing. | [shape reference only](../../tests/fixtures/albumeasy/stamp-shapes-v1/) |
| [ ] | `STAMP_ADD_TRIANGLE` | Triangle/flattened apex missing. | [shape reference only](../../tests/fixtures/albumeasy/stamp-shapes-v1/) |
| [ ] | `STAMP_ADD_TRIANGLE_INV` | Inverted triangle missing. | [shape reference only](../../tests/fixtures/albumeasy/stamp-shapes-v1/) |
| [ ] | `STAMP_ADD_HEXAGON` | SU/CU polygon orientation missing. | [shape reference only](../../tests/fixtures/albumeasy/stamp-shapes-v1/) |
| [ ] | `STAMP_ADD_OCTAGON` | SU/CU polygon orientation missing. | [shape reference only](../../tests/fixtures/albumeasy/stamp-shapes-v1/) |
| [ ] | `STAMP_ADD_BLANK` | Spacer-only stamp missing. | [shape reference only](../../tests/fixtures/albumeasy/stamp-shapes-v1/) |
| [x] | `STAMP_ADD_IMG` | Basic, NO_FRAME and combined inside text/signed offsets mapped; `STAMP_IMAGE` is native representation, not identical grammar. | [image overloads v2](../../tests/fixtures/albumeasy/stamp-image-overloads-v2/) |
| [x] | `STAMP_HEADING` | `NATIVE_STAMP_HEADING (LOCAL ...)`; signed offsets, multiline; attachment composition gap. | [headings](../../tests/fixtures/albumeasy/stamp-headings-v1/) |
| [x] | `STAMP_HEADING_VA` | `NATIVE_STAMP_HEADING (VA ...)`. | [headings](../../tests/fixtures/albumeasy/stamp-headings-v1/) |
| [x] | `STAMP_HEADING_PADDING` | `NATIVE_STAMP_HEADING_PADDING`; signed padding/state. | [headings](../../tests/fixtures/albumeasy/stamp-headings-v1/) |
| [x] | `STAMP_INNER_BORDER` | `NATIVE_STAMP_INNER_BORDER`; rectangular image frames only. | [inner borders](../../tests/fixtures/albumeasy/stamp-inner-borders-v1/) |
| [x] | `STAMP_BORDER_STYLE` | `NATIVE_STAMP_BORDER_STYLE`; ordinary styles/CORNER/SIDES; nonrectangular cases missing. | [border styles](../../tests/fixtures/albumeasy/stamp-border-styles-v1/) |
| [x] | `STAMP_INNER_BORDER_STYLE` | `NATIVE_STAMP_INNER_BORDER_STYLE`; bounded rectangular frames. | [border styles](../../tests/fixtures/albumeasy/stamp-border-styles-v1/) |
| [~] | `STAMP_BOXES_SIZE_ADJUST` | Signed TXT total additions; native `NATIVE_STAMP_FRAME` accepts nonnegative additions only. | [frame geometry tests](../../tests/native_pdf_stamp_box.rs) |
| [ ] | `STAMP_BOXES_ADJUST_SAVE` | Old adjustment-only save behavior missing; not equivalent to full settings save. | [image matrix](ALBUMEASY_4_5_0_STAMP_COMMAND_MATRIX.md) |
| [ ] | `STAMP_BOXES_ADJUST_RESTORE` | Old adjustment-only restore missing. | [image matrix](ALBUMEASY_4_5_0_STAMP_COMMAND_MATRIX.md) |
| [ ] | `STAMP_ADDX` | Obsolete multirow inside/below text missing. | [legacy inventory](ALBUMEASY_4_2_1_COMMAND_INVENTORY.md) |
| [ ] | `STAMP_ADDX_IMG` | Obsolete multirow image/caption form missing. | [legacy inventory](ALBUMEASY_4_2_1_COMMAND_INVENTORY.md) |

### Fonts/text metrics — 3

| Check | AlbumEasy command | Native / remaining work | Evidence |
|---|---|---|---|
| [ ] | `ALBUM_DEFINE_FONT` | User TrueType identifiers, embedding/resolution and encoding contract missing. | [text implementation](../../src/native_pdf/text.rs) |
| [ ] | `TEXT_CHAR_SPACING` | Tracking and state-capture rules missing. | [legacy inventory](ALBUMEASY_4_2_1_COMMAND_INVENTORY.md) |
| [~] | `TEXT_LINE_LEADING` | TXT zero reset only; nonzero explicitly rejected. | [footer-text tests](../../tests/native_pdf_stamp_text.rs) |

### Colors — 12 canonical names (aliases in section 3)

| Check | Canonical AlbumEasy command | Actual TXT/native surface and remaining work | Evidence |
|---|---|---|---|
| [x] | `COLOUR_ALBUM_BORDER` | Both spellings in TXT; native uses COLOUR spelling. | [border color](../../tests/fixtures/albumeasy/border-colour-v1/) |
| [~] L | `COLOUR_ALBUM_DECORATIVE_BORDER` | TOML/API line color; loader command missing; ornaments retain their pixels. | [decorative border tests](../../tests/native_pdf_decorative_border.rs) |
| [x] | `COLOUR_ALBUM_FOOTER` | Both TXT spellings for numbered footer; native footer explicit color. | [footer import](../../tests/native_pdf_footer_import.rs) |
| [~] | `COLOUR_ALBUM_HEADER` | TXT accepts **COLOR only**; COLOUR alias missing. Native header explicit color. | [interpreter](../../src/native_pdf/albumeasy.rs) |
| [~] | `COLOUR_ALBUM_TITLE` | TXT accepts **COLOR only**; COLOUR alias missing. Native title currently black. | [interpreter](../../src/native_pdf/albumeasy.rs) |
| [ ] | `COLOUR_PAGE_RULE_H` | Both spellings unmapped with rules. | [legacy inventory](ALBUMEASY_4_2_1_COMMAND_INVENTORY.md) |
| [ ] | `COLOUR_PAGE_TEXT` | Both spellings unmapped; stamp text color is not this command. | [interpreter](../../src/native_pdf/albumeasy.rs) |
| [~] | `COLOUR_STAMP_BORDER` | TXT accepts **COLOR only**; COLOUR alias missing. Native frame color supported. | [interpreter](../../src/native_pdf/albumeasy.rs) |
| [x] | `COLOUR_STAMP_INNER_BORDER` | Both TXT spellings; native inner-border color. | [inner-border tests](../../tests/native_pdf_stamp_inner_border.rs) |
| [x] | `COLOUR_STAMP_BACKGROUND` | Both spellings; native independent background/outline, rectangular restrictions. | [background](../../tests/fixtures/albumeasy/stamp-background-colours-v1/) |
| [x] | `COLOUR_STAMP_HEADING` | Both spellings; native heading explicit color. | [headings](../../tests/fixtures/albumeasy/stamp-headings-v1/) |
| [~] | `COLOUR_STAMP_TEXT` | TXT accepts **COLOR only**; COLOUR alias missing; native image caption defaults black. | [interpreter](../../src/native_pdf/albumeasy.rs) |

### Border-definition language — 11

These are **not album commands**. All have lower-level parser/render tests;
the album command that loads them is still missing from both document loaders.

| Check | Definition command | Existing bounded behavior | Evidence |
|---|---|---|---|
| [~] L | `IMAGE_SCALE` | Explicit scale/definition override. | [definition parser](../../src/native_pdf/commands/page/album_pages_decorative_border.rs) |
| [~] L | `IMAGE_CORNER` | PNG corner, mirrored four ways. | [decorative tests](../../tests/native_pdf_decorative_border.rs) |
| [~] L | `IMAGE_CORNER_PAD` | Signed independent H/V offsets. | [decorative tests](../../tests/native_pdf_decorative_border.rs) |
| [~] L | `IMAGE_EDGE_LEFT` | Left edge ornament. | [decorative tests](../../tests/native_pdf_decorative_border.rs) |
| [~] L | `IMAGE_EDGE_RIGHT` | Mirrored right edge. | [decorative tests](../../tests/native_pdf_decorative_border.rs) |
| [~] L | `IMAGE_EDGE_TOP` | Top edge ornament. | [decorative tests](../../tests/native_pdf_decorative_border.rs) |
| [~] L | `IMAGE_EDGE_BOTTOM` | Mirrored bottom edge. | [decorative tests](../../tests/native_pdf_decorative_border.rs) |
| [~] L | `IMAGE_EDGE_PAD` | H/V edge offsets. | [decorative tests](../../tests/native_pdf_decorative_border.rs) |
| [~] L | `BORDER_LINES` | Three width slots and spacing; exact four-argument parser, other defaults unresolved. | [definition parser](../../src/native_pdf/commands/page/album_pages_decorative_border.rs) |
| [~] L | `BORDER_PAD` | H/V connecting-line displacement. | [decorative tests](../../tests/native_pdf_decorative_border.rs) |
| [~] L | `BORDER_END_OFFSETS` | Independent line-end offsets. | [decorative tests](../../tests/native_pdf_decorative_border.rs) |

## 2. Additional 4.5.0 reference entries — 29

“Additional” means absent from our 4.2.1 inventory, not a claim about the exact
release that first introduced the feature. Retain the old mm spelling in
section 1, now obsolete; retain old adjustment-save names as separate behavior.

| Check | AlbumEasy command/directive | Native / remaining work | Evidence |
|---|---|---|---|
| [x] | `ALBUM_STAMP_IMG_NEW_SETTING` | `NATIVE_STAMP_IMAGE_POLICY`; mm padding and stretch. | [image controls](../../tests/fixtures/albumeasy/stamp-image-controls-v1/) |
| [x] | `ALBUM_STAMP_IMG_NEW_SETTING_PCNT` | Native percent padding policy. | [image controls](../../tests/fixtures/albumeasy/stamp-image-controls-v1/) |
| [x] | `ALBUM_STAMP_IMG_ASPECT_RATIO` | Native preserve/fit independently of stretch. | [image controls](../../tests/fixtures/albumeasy/stamp-image-controls-v1/) |
| [x] | `ALBUM_STAMP_IMG_GREYSCALE_ON` | Native grayscale policy; GRAYSCALE alias also accepted. | [image tests](../../tests/native_pdf_stamp_image.rs) |
| [x] | `ALBUM_STAMP_IMG_GREYSCALE_OFF` | Native colour policy; GRAYSCALE alias also accepted. | [image tests](../../tests/native_pdf_stamp_image.rs) |
| [x] | `ALBUM_STAMP_IMG_HIDE` | Hide illustration without removing physical frame. | [image controls](../../tests/fixtures/albumeasy/stamp-image-controls-v1/) |
| [x] | `ALBUM_STAMP_IMG_SHOW` | Restore illustration visibility. | [image controls](../../tests/fixtures/albumeasy/stamp-image-controls-v1/) |
| [x] | `ALBUM_STAMP_SETTINGS_SAVE` | `NATIVE_STAMP_SETTINGS_SAVE`; bounded supported state, not unimplemented commands. | [settings](../../tests/fixtures/albumeasy/stamp-settings-save-restore-v1/) |
| [x] | `ALBUM_STAMP_SETTINGS_RESTORE` | `NATIVE_STAMP_SETTINGS_RESTORE`; nesting/error semantics bounded. | [settings tests](../../tests/native_pdf_stamp_settings.rs) |
| [x] | `STAMP_FOOTER_TXT_ALIGN` | `NATIVE_STAMP_FOOTER_TEXT_ALIGN`; supported alignment aliases. | [footer text](../../tests/fixtures/albumeasy/stamp-footer-text-v1/) |
| [x] | `STAMP_FOOTER_TXT_PAD` | `NATIVE_STAMP_FOOTER_TEXT_PAD`; signed padding/state. | [footer text](../../tests/fixtures/albumeasy/stamp-footer-text-v1/) |
| [ ] | `STAMP_ADD_RT_TRIANGLE` | Shape/orientation mapping missing. | [shape reference only](../../tests/fixtures/albumeasy/stamp-shapes-v1/) |
| [ ] | `STAMP_ADD_PENTAGON` | Shape/orientation mapping missing. | [shape reference only](../../tests/fixtures/albumeasy/stamp-shapes-v1/) |
| [ ] | `PAGE_START_INFO` | Informational-page numbering/count behavior missing. | [version update](ALBUMEASY_4_5_0_REFERENCE_UPDATE.md) |
| [ ] | `PAGE_SET_VERTICAL_POS` | Absolute cursor positioning missing. | [version update](ALBUMEASY_4_5_0_REFERENCE_UPDATE.md) |
| [ ] | `PAGE_QUADRILLE` | Grid geometry/state/color missing. | [version update](ALBUMEASY_4_5_0_REFERENCE_UPDATE.md) |
| [ ] | `PAGE_ADD_BOX` | Free-positioned box mapping missing. | [version update](ALBUMEASY_4_5_0_REFERENCE_UPDATE.md) |
| [~] L | `ALBUM_PAGES_CROP_MARKS` | API/TOML clipping policy/proofs; missing album-loader mapping. | [crop marks](../../tests/fixtures/albumeasy/crop-marks-v1/) |
| [x] | `ALBUM_PAGES_MARGIN_TXT` | `NATIVE_PAGE_MARGIN_TEXT`; max two definitions; rotations/direction. | [margin text](../../tests/fixtures/albumeasy/margin-text-v1/) |
| [x] | `ALBUM_PAGES_MARGIN_TXT_PAD` | TXT global padding; native explicit border gap. | [margin tests](../../tests/native_pdf_margin_text.rs) |
| [x] | `COLOUR_ALBUM_MARGIN_TXT` | Both TXT spellings; native per-definition color. | [margin text](../../tests/fixtures/albumeasy/margin-text-v1/) |
| [ ] | `COLOUR_PAGE_QUADRILLE` | Both color spellings missing with grid mapping. | [version update](ALBUMEASY_4_5_0_REFERENCE_UPDATE.md) |
| [ ] | `$INCLUDE` | Dependency resolution/source locations/preprocessing missing. | [version update](ALBUMEASY_4_5_0_REFERENCE_UPDATE.md) |
| [ ] | `$DEFINE` | Macro scope/expansion missing. | [version update](ALBUMEASY_4_5_0_REFERENCE_UPDATE.md) |
| [ ] | `$UNDEFINE` | Macro removal missing. | [version update](ALBUMEASY_4_5_0_REFERENCE_UPDATE.md) |
| [ ] | `$IFDEF` | Conditional preprocessing missing. | [version update](ALBUMEASY_4_5_0_REFERENCE_UPDATE.md) |
| [ ] | `$ELSE` | Conditional preprocessing missing. | [version update](ALBUMEASY_4_5_0_REFERENCE_UPDATE.md) |
| [ ] | `$ELSEIF` | Conditional preprocessing missing. | [version update](ALBUMEASY_4_5_0_REFERENCE_UPDATE.md) |
| [ ] | `$ENDIF` | Conditional preprocessing missing. | [version update](ALBUMEASY_4_5_0_REFERENCE_UPDATE.md) |

## 3. Spelling aliases — do not silently assume support

Every listed `COLOUR_` command has a documented `COLOR_` alias: 12 pairs in
4.2.1, 14 in 4.5.0. Their checkoff status is explicit here.

| Check | Additional spelling | Versions / current TXT behavior |
|---|---|---|
| [x] | `COLOR_ALBUM_BORDER` | Both versions; accepted. |
| [ ] | `COLOR_ALBUM_DECORATIVE_BORDER` | Both; missing loader mapping. |
| [x] | `COLOR_ALBUM_FOOTER` | Both; bounded numbered-footer support. |
| [x] | `COLOR_ALBUM_HEADER` | Both; accepted (COLOUR counterpart still missing). |
| [x] | `COLOR_ALBUM_TITLE` | Both; accepted (COLOUR counterpart still missing). |
| [ ] | `COLOR_PAGE_RULE_H` | Both; missing. |
| [ ] | `COLOR_PAGE_TEXT` | Both; missing. |
| [x] | `COLOR_STAMP_BORDER` | Both; accepted (COLOUR counterpart still missing). |
| [x] | `COLOR_STAMP_INNER_BORDER` | Both; accepted. |
| [x] | `COLOR_STAMP_BACKGROUND` | Both; bounded rectangular images. |
| [x] | `COLOR_STAMP_HEADING` | Both; accepted. |
| [x] | `COLOR_STAMP_TEXT` | Both; accepted (COLOUR counterpart still missing). |
| [x] | `COLOR_ALBUM_MARGIN_TXT` | 4.5 inventory addition; accepted. |
| [ ] | `COLOR_PAGE_QUADRILLE` | 4.5 inventory addition; missing. |
| [x] | `ALBUM_STAMP_IMG_GRAYSCALE_ON` | 4.5; accepted US spelling. |
| [x] | `ALBUM_STAMP_IMG_GRAYSCALE_OFF` | 4.5; accepted US spelling. |

Obsolete command names remain separate rows in section 1; notably legacy
image units and adjustment-only save/restore are not name-only aliases.

## 4. Native-only commands

These belong in a versioned `.stampalbum` document headed **`STAMPALBUM 1`**,
never in AlbumEasy reference TXT. [Native loader](../../src/native_pdf/album.rs)
is the authority for exact syntax. “Implemented” is standalone, not production.
The five shared native spellings are `ALBUM_PAGES_SIZE`, `ALBUM_PAGES_MARGINS`,
`ALBUM_PAGES_SPACING`, `ALBUM_PAGES_BORDER` and `COLOUR_ALBUM_BORDER`.

| Check | Native command | Capability / remaining boundary |
|---|---|---|
| [x] | `STAMP_IMAGE` | Explicit path, frame dimensions, image scale; optional nonempty caption. |
| [x] | `NATIVE_PAGE_TITLE` | Visible title; black currently, explicit following space. |
| [x] | `NATIVE_PAGE_HEADER` | Positioned header with explicit gap/color. |
| [x] | `NATIVE_PAGE_FOOTER_NUMBER` | Numbering/gaps/color; empty quoted prefix/suffix bug remains. |
| [x] | `NATIVE_PAGE_MARGIN_TEXT` | Two positioned/rotated definitions with explicit color/gap. |
| [x] | `NATIVE_PAGE_BACKGROUND_IMAGE` | Explicit image rectangle; not AlbumEasy automatic background grammar. |
| [x] | `NATIVE_ROW_ALIGNMENT` | Native horizontal row distribution; distinct from vertical alignment. |
| [x] | `NATIVE_ROW_VERTICAL_ALIGNMENT` | TOP/MIDDLE/BOTTOM. |
| [x] | `NATIVE_ROW_BREAK` | End current row, not explicit page break. |
| [x] | `NATIVE_PAGE_BREAK` | `()` resets document spacing; `(horizontal vertical)` selects page-local spacing inherited by automatic continuations. Flushes rows; leading/repeated/trailing breaks preserve intentional empty pages and numbering. Native first-page override remains deferred. |
| [x] | `NATIVE_PAGE_NUMBER_SECTION` | Explicit section boundary; TOML numbering scope required. |
| [x] | `NATIVE_STAMP_FRAME` | Line/color/optional nonnegative total frame additions. |
| [x] | `NATIVE_STAMP_TEXT_POLICY` | Wrap/expanded width, padding/gap and width limits. |
| [x] | `NATIVE_STAMP_IMAGE_POLICY` | Millimetre/percent padding, stretch, aspect, grayscale, visibility. |
| [x] | `NATIVE_STAMP_SETTINGS_SAVE` | Save currently modeled stamp settings. |
| [x] | `NATIVE_STAMP_SETTINGS_RESTORE` | Restore currently modeled stamp settings. |
| [x] | `NATIVE_STAMP_BACKGROUND` | Independent rectangular frame fill. |
| [x] | `NATIVE_STAMP_OUTLINE` | Independent visible/hidden outline. |
| [x] | `NATIVE_STAMP_INNER_BORDER` | Rectangular inner frame dimensions/color. |
| [x] | `NATIVE_STAMP_BORDER_STYLE` | Outer styles/selectors. |
| [x] | `NATIVE_STAMP_INNER_BORDER_STYLE` | Inner styles/selectors. |
| [x] | `NATIVE_STAMP_HEADING` | LOCAL/VA headings; combining with image labels still limited. |
| [x] | `NATIVE_STAMP_HEADING_PADDING` | Signed heading padding. |
| [x] | `NATIVE_STAMP_FOOTER_TEXT` | Explicit per-image left/centre/right below-frame attachment. |
| [x] | `NATIVE_STAMP_FOOTER_TEXT_ALIGN` | Alignment state. |
| [x] | `NATIVE_STAMP_FOOTER_TEXT_PAD` | Signed below-frame padding. |
| [x] | `NATIVE_STAMP_IMAGE_LABEL` | Text inside above/below illustration, with clearance. |
| [x] | `NATIVE_STAMP_IMAGE_SIDE_LABEL` | Left/right/both labels; TOML font/wrap/line limits; full footprint reserved. |

Evidence: [native fixture inputs](../../tests/fixtures/native_pdf/),
[image-control proofs](../../tests/fixtures/albumeasy/stamp-image-controls-v1/),
[section numbering](../../tests/fixtures/albumeasy/native-section-numbering-v1/),
[native album tests](../../tests/native_pdf_album.rs),
[text tests](../../tests/native_pdf_stamp_text.rs).

## 5. Extras and extra commands still to define

Do not invent command names before design/approval. These are capabilities or
requested work, **not additional accepted AlbumEasy spellings**.

| Check | Extra / command surface | Next step |
|---|---|---|
| [x] | `$SECTION_PAGE$`, `$SECTION_PAGES$`, `$DOCUMENT_PAGES$` native tokens | Retain section/document distinction; not standalone commands or AlbumEasy tokens. |
| [x] | TOML side-label font/wrap/line policies | Keep defaults explicit and test item overrides/errors; no new AlbumEasy command. |
| [~] | Wrapped captions/side labels and continuation pagination | Existing native behavior; print/readability review and dense-row fidelity remain. |
| [x] | Native explicit page-break command | #103 intentional empty pages/caption continuation; #104 optional H/V override, automatic inheritance and default reset. |
| [ ] | Native free-standing page text and vertical-space commands | AUS-03; preserve order and source text without relocating it into stamp captions/headings. |
| [ ] | Combined heading + inside + side-label attachment | AUS-05; preserve all footprints without resizing physical frames. |
| [ ] | Left/right title-icon commands | Requested direction; detailed command design/implementation pending. |
| [ ] | Country-review preview command/UI | Idea only; no command/API approved. |
| [ ] | Canonical generation, TXT export loss report, typed editor and backend rollout | Separate gated roadmap work, not completed by standalone proofs. |

Detailed outstanding country findings: AUS-01–AUS-08, BRA-01–BRA-07 and
CIS-01–CIS-05 in the [implementation plan](../reports/NATIVE_PDF_IMPLEMENTATION_PLAN.md).
Country proof data/tests/reports are local-only; their absence on another
checkout must not be interpreted as missing tracked conformance fixtures.

## 6. Variants and shared language rules

These are not new command names; keep them open even when a bounded command
above is checked.

| Check | Cross-cutting variant / rule | Remaining work |
|---|---|---|
| [~] | Command case, quoted strings, multiline text and comments | Current loaders have different grammars; semicolon annotations and case differences remain explicit country adaptations, not compatibility. BRA-04/CIS-04. |
| [ ] | Column widths and independent padding | Map complete 4.5 column flow. |
| [ ] | Paragraph/rule independent right padding | Map 4.5 optional arguments and wrap/geometry. |
| [~] | Multiline titles/headers/footers and title following space | Bounded modules/native text exist; reconcile full TXT grammar, defaults and state. |
| [ ] | Informational pages and group-start substitutions | Define page counts and ordering for these AlbumEasy variants. |
| [~] | Modern/legacy stamp text/image overloads | Combined image text is mapped; text-only/obsolete forms and nonrectangular edge choices remain. |
| [~] | Shared canonical model and pagination | TXT currently builds positioned scenes; native documents use automatic flow. Do not claim identical canonical layout yet. |
| [ ] | Final conformance/export/editor/OS/print acceptance | Remains separate from bounded renderer tests; see roadmap. |

## 7. Simple workflow for every update

1. **Pick one bounded command/variant.** Verify pinned help/manual, units,
   defaults/state/aliases. Create/link its issue and branch.
2. **AlbumEasy first.** Create a realistic reference-only TXT under
   `tests/fixtures/albumeasy/<family>-vN/`; run local pinned AlbumEasy, inspect
   PDF and obtain reference acceptance before native implementation.
3. **Map natively.** Keep TXT and native-language grammar separate; preserve
   frames/assets and report unsupported behavior. No production routing yet.
4. **Compare all applicable inputs:** original TXT, byte-identical raw-command
   `.stampalbum`, and genuine `STAMPALBUM 1` native document. Keep accepted
   references immutable; use a new fixture version for intentional changes.
5. **Validate.** Add scoped Cargo tests for state/errors/geometry; count parsed
   PDF draw operations, inspect matched pages, record exact commands/results,
   OS coverage, warnings, filesystem/DB impact and pending print acceptance.
6. **Update this checklist and the detailed plan.** Check only the bounded
   surface actually proved; retain unfinished overloads/aliases/loader wiring.
   Record human acceptance separately. Commit/push only when requested.

Use the [tracked fixture workflow](../reports/NATIVE_PDF_FIXTURE_WORKFLOW.md)
for reference commands and review rules. Country explorations stay local and
use ignored naming patterns; do not force-add their data or tests. Their local
process guide is optional and is not required to understand this checklist.

### Where to look

| Surface | Location / meaning |
|---|---|
| TXT compatibility interpretation | [albumeasy.rs](../../src/native_pdf/albumeasy.rs): actual accepted names, argument checks, state and layout. |
| Native `.stampalbum` grammar | [album.rs](../../src/native_pdf/album.rs): version header and native commands. |
| Lower-level page commands | [commands/](../../src/native_pdf/commands/): capability does not imply loader support. |
| TOML policies | [config.rs](../../src/native_pdf/config.rs), [template](../../config_template.toml), [full example](../../config_full_example.toml). |
| Layout/rendering | [native_pdf/](../../src/native_pdf/): document, rows, text, images, pagination and renderer. |
| Standalone entry point | [render_stampalbum.rs](../../examples/render_stampalbum.rs): TXT/raw/native dispatch, no production switch. |
| Tracked reference/native proofs | [AlbumEasy fixtures](../../tests/fixtures/albumeasy/), [native fixtures](../../tests/fixtures/native_pdf/), `tests/native_pdf_*.rs`. |
| Production import/editor | [external_album.rs](../../src/external_album.rs), [import parser](../../src/external_album/parser.rs), [Review UI](../../src/review_ui/). Still separate integration work. |
| Full roadmap and discovered gaps | [implementation plan](../reports/NATIVE_PDF_IMPLEMENTATION_PLAN.md). |

### Reference sources and verification

- [4.2.1 inventory](ALBUMEASY_4_2_1_COMMAND_INVENTORY.md): 92 names and source
  manual hash, including obsolete commands and 12 color aliases.
- [4.5.0 update](ALBUMEASY_4_5_0_REFERENCE_UPDATE.md) and
  [stamp grammar matrix](ALBUMEASY_4_5_0_STAMP_COMMAND_MATRIX.md).
- Installed 4.5.0 Qt help: read-only `commands.html`, `borders.html`,
  `fonts.html`; QCH SHA-256
  `8b81707d861aacceb798b9f38efbbfdc3ea86fa49a57b7e343c3a25d5964d5be`.
  The manual/help text is not copied into this document; descriptions here
  summarize behavior and implementation status.
- Snapshot checked against native source and existing test/fixture contents.
  This documentation update does not rerun or certify all historical proofs.

**Update rule:** change a row's check, note and evidence together. Preserve
version distinctions; add real commands/aliases separately from overloads or
ideas. Do not equate this checklist's mapped rows with the roadmap's completed
task count.
