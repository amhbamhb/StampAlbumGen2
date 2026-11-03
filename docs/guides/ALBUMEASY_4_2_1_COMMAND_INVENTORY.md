# AlbumEasy 4.2.1 command inventory

**Reviewed:** 2026-11-03

**Legacy reference:** Primary compatibility now targets installed AlbumEasy
4.5.0. See the [version update](ALBUMEASY_4_5_0_REFERENCE_UPDATE.md); the 92-name
inventory and ambiguity notes below remain specifically scoped to 4.2.1.
The installed help resolves several ambiguities, including both mm image-setting
spellings and the combined image/text overload; full implementation remains open.

**Purpose:** Planning baseline for replacing the external AlbumEasy renderer with
internal PDF generation. This is a command/behavior inventory, not an
implementation or a declaration of application support.

## 1. Source and counting rules

Source: [AlbumEasy_4.2.1.pdf](../AlbumEasy_4.2.1.pdf), 88 PDF pages. Its cover says
V4.2; page 2 explicitly includes the V4.2.1 correction. Pin this supplied reference
as **4.2.1**, not as a claim about the newest available AlbumEasy release.
Page references below are PDF page numbers, matching the printed numbering.

Source SHA-256:
`e7cb37652977832f26f008c7abd7f97282efdd0e4e6cdfbbe1edfc71af15bff7`

The inventory summarizes factual syntax and behavior in our own words; consult
the manual for its diagrams and examples.

| Family | Distinct documented names |
|--------|----------------------------|
| Album settings/metadata, excluding font definition | 22 |
| Page commands, including obsolete text variants | 18 |
| Row commands | 6 |
| Stamp commands, including obsolete variants | 20 |
| Font definition and text metrics | 3 |
| Colour commands, `COLOUR_` spelling | 12 |
| Decorative-border definition commands | 11 |
| **Canonical documented names** | **92** |
| Additional `COLOR_` spelling aliases | 12 |
| **Documented spellings including colour aliases** | **104** |

The 92 include **9 explicitly obsolete names**; keep these for import
compatibility. Parameter variants do not count as separate command names.
Decorative-border files have their own 11-command language and must not be
mistaken for ordinary album commands. Command-like typos and unresolved names
are recorded separately, not added to the supported language without evidence.

**Implementation status for every row:** not certified. Before implementation,
give each row separate evidence for parsing, argument validation, state/layout,
native rendering, editing, export and conformance fixtures. A parser whitelist
entry is not proof of any of these.

The [application/configuration gap analysis](../reports/ALBUMEASY_4_2_1_APPLICATION_GAPS.md)
now maps all 92 names to existing output and imported-command validation,
distinguishes placeholders from constrained validation, and includes the current
`config.toml` layout/settings without exposing sensitive values. This is static
evidence, not runtime conformance certification.

## 2. Album settings and metadata — 22 names

Parameter descriptions are summaries, not a complete formal grammar.

| Command | Parameters / behavior to preserve | PDF pages |
|---------|-----------------------------------|-----------|
| `ALBUM_PAGES_SIZE` | Width, height in mm; required page geometry. | 14 |
| `ALBUM_PAGES_MARGINS` | Left, right, top, bottom; required margins. | 14 |
| `ALBUM_PAGES_MARGINSE` | Alternate margins on even pages. | 14 |
| `ALBUM_PAGES_SPACING` | Default horizontal inset and vertical element spacing. | 14 |
| `ALBUM_PAGES_TITLE` | Font, point size, title; repeated page title. | 14 |
| `ALBUM_PAGES_BORDER` | Line widths and separation; accept both legacy 3-argument and current 4-argument forms. | 15 |
| `ALBUM_PAGES_DECORATIVE_BORDER` | Border-definition filename; separate asset/file resolution rules. | 15, 43–46 |
| `ALBUM_PAGES_HEADER` | Font, size, position, text. | 15–16 |
| `ALBUM_PAGES_HEADER_NUMBER` | Font, size, position; optional starting number, prefix, suffix. | 15–16 |
| `ALBUM_PAGES_HEADER_DATE` | Font, size, position, date format; optional separator, prefix, suffix. | 15–16 |
| `ALBUM_PAGES_FOOTER` | Font, size, position, text. | 15–16 |
| `ALBUM_PAGES_FOOTER_NUMBER` | Footer numbering, including total-page substitution. | 15–16 |
| `ALBUM_PAGES_FOOTER_DATE` | Footer generation date and optional formatting fields. | 15–17 |
| `ALBUM_PAGES_HEADER_PAD` | Album-wide header distance from border. | 17 |
| `ALBUM_PAGES_FOOTER_PAD` | Album-wide footer distance from border. | 17 |
| `ALBUM_AUTHOR` | PDF author metadata; not printed text. | 17 |
| `ALBUM_TITLE` | PDF title metadata; not the visible page title. | 17 |
| `ALBUM_STAMP_NEW_IMG_SETTING` | Horizontal/vertical image padding in mm and stretch flag; documented default 0.7, 0.7, false. | 17–18 |
| `ALBUM_PAGES_BORDER3` | **Obsolete:** four-argument border spelling; retain import support. | 41 |
| `ALBUM_PAGES_NUMBER` | **Obsolete:** footer-number predecessor. | 15–17, 42 |
| `ALBUM_PAGES_DATE` | **Obsolete:** footer-date predecessor. | 15–17, 42 |
| `ALBUM_STAMP_IMG_SETTING` | **Obsolete:** legacy padding uses 0.35 mm units; do not treat as a simple mm alias. | 17–18, 31, 42 |

Header/footer positions: `L`, `R`, `C`, `LR`, `RL`; odd/even behavior matters.
Date formats: `YMD`, `DMY`, `MDY`. `$PAGES$` requires total-page knowledge.
Overlapping header/footer slots need validation. Preserve when global text
metrics/colors are captured, rather than applying the final state indiscriminately.

## 3. Page commands — 18 names

| Command | Parameters / behavior to preserve | PDF pages |
|---------|-----------------------------------|-----------|
| `PAGE_START` | Start page; also accepts arguments for page-start-group placeholders. At least one page start is required. | 19, 23–24 |
| `PAGE_START_VAR` | Page-specific horizontal/vertical spacing; group substitution arguments also need investigation. | 19, 23 |
| `PAGE_COLUMN_START` | Start two-column mode; optional padding applies to each side of the inter-column gap. | 19 |
| `PAGE_COLUMN_NEXT` | Move output to the second column. | 19 |
| `PAGE_COLUMN_STOP` | Leave column mode early; page end also ends it. | 19 |
| `PAGE_TEXT` | Font, size, text; optional following vertical space; left aligned. | 20–21 |
| `PAGE_TEXT_CENTRE` | Same text parameters, centered. | 20–21 |
| `PAGE_TEXT_RIGHT` | Same text parameters, right aligned. | 20–21 |
| `PAGE_TEXT_PARAGRAPH_START` | Font, size, alignment; optional side padding and following space; begins raw paragraph content. | 21–22 |
| `PAGE_TEXT_PARAGRAPH_END` | Ends paragraph-content mode. | 21–22 |
| `PAGE_RULE_H` | Line thickness and preceding spacing; optional margin inset; spacing combines with default vertical spacing. | 22 |
| `PAGE_VSPACE` | Positive or negative vertical cursor movement. | 22 |
| `PAGE_START_GROUP_BEGIN` | Define the sole repeated page-start command block before the first page. | 23–24 |
| `PAGE_START_GROUP_END` | End repeated block; preserve nested commands and positional substitution. | 23–24 |
| `PAGE_BACKGROUND_IMG` | Image; optional scale and paired horizontal/vertical offsets. Center/fit within margins and preserve relative paths. | 24–25 |
| `PAGE_TEXT_VSPACE` | **Obsolete:** left-aligned text with explicit following space. | 41 |
| `PAGE_TEXT_CENTRE_VSPACE` | **Obsolete:** centered text with explicit following space. | 41 |
| `PAGE_TEXT_RIGHT_VSPACE` | **Obsolete:** right-aligned text with explicit following space. | 41 |

Paragraph alignments: `LEFT`, `RIGHT`, `CENTRE`, `CENTER`, `JUSTIFIED`.
Paragraph physical newlines concatenate without automatically inserting spaces;
explicit `\n` breaks lines. Page-start placeholders are `$$1` through `$$9`.
Negative spacing is supported behavior, not automatically invalid input.

## 4. Row commands — 6 names

| Command | Parameters / behavior to preserve | PDF page |
|---------|-----------------------------------|----------|
| `ROW_START_ES` | Font, size, border width; equal gaps including the outer gaps. | 27 |
| `ROW_START_JS` | Font, size, border width; outside boxes meet row bounds, inside gaps are equal. | 27 |
| `ROW_START_FS` | Font, size, border width, fixed gap; center the complete row. | 27 |
| `ROW_ALIGN_TOP` | Persistent top alignment; documented default. | 27 |
| `ROW_ALIGN_MIDDLE` | Persistent vertical middle alignment. | 27 |
| `ROW_ALIGN_BOTTOM` | Persistent bottom alignment. | 27 |

## 5. Stamp commands — 20 names

| Command | Parameters / behavior to preserve | PDF pages |
|---------|-----------------------------------|-----------|
| `STAMP_ADD` | Rectangular frame: width, height, inside text and left/center/right captions. Retain the legacy six-text-field form. | 28–29 |
| `STAMP_ADD_DIAMOND` | Diamond frame; dimensions and caption semantics. | 28–29 |
| `STAMP_ADD_OVAL` | Oval frame; equal dimensions make a circle. | 28–29 |
| `STAMP_ADD_TRIANGLE` | Triangle; optional flattened-apex width. | 28–29 |
| `STAMP_ADD_TRIANGLE_INV` | Inverted triangle; optional flattened-apex width. | 28–29 |
| `STAMP_ADD_HEXAGON` | Width and `SU`/`CU` orientation, followed by text fields. | 28 |
| `STAMP_ADD_OCTAGON` | Width and `SU`/`CU` orientation, followed by text fields. | 28 |
| `STAMP_ADD_BLANK` | Width and height; occupy row space without frame/text. | 30 |
| `STAMP_ADD_IMG` | Width, height, image and captions; optional `NO_FRAME`. Combined inside-text/image syntax remains an open documentation gap. | 2, 30–31 |
| `STAMP_HEADING` | Font, size, text; optional horizontal offset; attaches to preceding stamp. | 32–33 |
| `STAMP_HEADING_VA` | Heading variant vertically aligned across a row. | 32–33 |
| `STAMP_HEADING_PADDING` | Persistent heading-to-box spacing adjustment; positive or negative. | 33 |
| `STAMP_INNER_BORDER` | Padding and inner line width; zero/zero cancels; rectangular bordered boxes only. | 35 |
| `STAMP_BORDER_STYLE` | Line style, including optional selected corner markers. | 36–37 |
| `STAMP_INNER_BORDER_STYLE` | Inner-frame style; preserve applicable shape restrictions. | 35–36 |
| `STAMP_BOXES_SIZE_ADJUST` | Add signed amount to box width/height, not once per edge. | 37 |
| `STAMP_BOXES_ADJUST_SAVE` | Temporarily disable size adjustment while retaining its value. | 37 |
| `STAMP_BOXES_ADJUST_RESTORE` | Restore saved size adjustment; nesting behavior needs confirmation. | 37 |
| `STAMP_ADDX` | **Obsolete:** three inside lines and two rows of three captions. | 41 |
| `STAMP_ADDX_IMG` | **Obsolete:** image and two rows of three captions; optional `NO_FRAME`. | 41 |

Styles: `SOLID`, `DOTTED`, `DOT_DASH`, `DASHED`, `BLANK`, `CORNER`.
Corner selectors: `TL`, `TR`, `BL`, `BR`. `CORNER` falls back to solid for
nonrectangular shapes. Background fill and inner frames have additional shape
restrictions; do not render them on every shape merely because the command parses.
Images are documented as PNG/JPEG; physical sizing, image padding, scaling,
caption placement and optional frame suppression are independent concerns.

## 6. Fonts and text metrics — 3 names

| Command | Parameters / behavior to preserve | PDF pages |
|---------|-----------------------------------|-----------|
| `ALBUM_DEFINE_FONT` | User font identifier and quoted TrueType font name; old encoding argument is accepted but ignored from V4.0 onward. | 40 |
| `TEXT_CHAR_SPACING` | Signed tracking in mm; excludes inside/below-stamp text; persistent and sometimes captured by global commands. | 25–26, 34–35 |
| `TEXT_LINE_LEADING` | Signed extra multiline leading in mm, not paragraph spacing; persistent with global capture rules. | 26 |

Built-in font identifiers: `CN`, `CB`, `CI`, `CS`, `TN`, `TB`, `TI`, `TS`,
`HN`, `HB`, `HI`, `HS`. These identify normal/bold/italic/bold-italic variants of
Courier, Times and Helvetica. Font sizes are points, unlike geometry in mm.
Custom identifiers are documented as 2–12 characters. The manual's 16-custom-font
session limit is an external implementation restriction: decide explicitly
whether native retains it or reports it as an export-compatibility constraint.

## 7. Colours — 12 canonical names plus 12 aliases

Each takes one colour value. Every `COLOUR_` name below also has a documented
`COLOR_` spelling with the same suffix and semantics (pp. 38, 48).

| Command | State / affected content | PDF pages |
|---------|--------------------------|-----------|
| `COLOUR_ALBUM_BORDER` | Album-wide ordinary border. | 38 |
| `COLOUR_ALBUM_DECORATIVE_BORDER` | Album-wide decorative lines, not recoloring ornament images. | 38 |
| `COLOUR_ALBUM_FOOTER` | Album-wide footer text. | 38 |
| `COLOUR_ALBUM_HEADER` | Album-wide header text. | 38 |
| `COLOUR_ALBUM_TITLE` | Album-wide title text. | 38 |
| `COLOUR_PAGE_RULE_H` | Subsequent horizontal rules. | 38 |
| `COLOUR_PAGE_TEXT` | Subsequent ordinary page text. | 38 |
| `COLOUR_STAMP_BORDER` | Subsequent stamp outer borders. | 38 |
| `COLOUR_STAMP_INNER_BORDER` | Subsequent stamp inner borders. | 38 |
| `COLOUR_STAMP_BACKGROUND` | Subsequent rectangular framed-box fill; excludes corner-only style. | 38–39 |
| `COLOUR_STAMP_HEADING` | Subsequent stamp headings. | 38–39 |
| `COLOUR_STAMP_TEXT` | Subsequent text inside/below stamp boxes. | 38–39 |

Values include named colours (table pp. 49–57) and `$RRGGBB` hexadecimal notation,
not `#RRGGBB`, because `#` introduces comments. Record named-value RGB mappings
and aliases when turning this inventory into an executable grammar; do not
assume an arbitrary graphics library's colour table is equivalent.

## 8. Decorative-border language — 11 names

These belong in a separate border-definition parser, reached by
`ALBUM_PAGES_DECORATIVE_BORDER`.

| Command | Parameters / behavior to preserve | PDF pages |
|---------|-----------------------------------|-----------|
| `IMAGE_SCALE` | Ornament scale factor. | 44 |
| `IMAGE_CORNER` | Top-left-oriented source image; transform for other corners. | 44 |
| `IMAGE_CORNER_PAD` | Horizontal/vertical corner offsets. | 45 |
| `IMAGE_EDGE_LEFT` | Left edge ornament image. | 45 |
| `IMAGE_EDGE_RIGHT` | Right edge ornament; source uses left-edge orientation. | 45 |
| `IMAGE_EDGE_TOP` | Top edge ornament. | 45 |
| `IMAGE_EDGE_BOTTOM` | Bottom ornament; source uses top-edge orientation. | 45 |
| `IMAGE_EDGE_PAD` | Horizontal/vertical edge-image offsets; confirm axis wording. | 45 |
| `BORDER_LINES` | Up to three line widths and line separation. | 45 |
| `BORDER_PAD` | Horizontal/vertical displacement from margin-centered border. | 45 |
| `BORDER_END_OFFSETS` | Shorten line endpoints to avoid corner ornaments. | 45–46 |

At least one corner/edge image is required. Ornaments are transparent PNG files
relative to the border definition. Bare border filenames search the installed
border directory before the album directory; filenames with directories are
album-relative (p. 43). Native needs its own explicit portable resource lookup,
not a hidden dependency on an AlbumEasy installation.

## 9. Language behavior outside individual command names

| Area | Planning requirement | PDF pages |
|------|----------------------|-----------|
| Comments and quoting | `#` comments, quoted text, escaped quotes/backslashes; determine quoted `#` behavior explicitly. | 13 |
| Continuations | Quoted text continuation with trailing backslash; distinguish from paragraph-content mode and ordinary physical wrapping in the manual. | 13, 21–22 |
| Text escapes | `\n`, `\"`, `\\`; distinguish raw source from interpreted text. | 13 |
| Units | Millimetres for geometry; points for fonts; legacy image-padding units differ. | 13, 17, 40, 42 |
| Encoding | UTF-8 and the mode called “Latin 1,” which actually means Windows-1252; preserve euro and extended characters. | 47 |
| Paths/assets | Album-relative images, border-specific lookup, font-name resolution and missing-resource diagnostics on both OSes. | 25, 30, 40, 43–45 |
| State and ordering | Global, page, column, row and stamp state; repeated groups and captured metrics; no indiscriminate flattening into final settings. | 14–40 |
| Deferred content | `$PAGES$`, dates and group substitutions; generation must use a consistent snapshot/time policy. | 16, 23–24 |
| Composition | Existing commands can construct title-side icons and asymmetric rows; inventory combinations as well as standalone commands. | 61–64 |

The external GUI, wizard, editor shortcuts and console switches are not album
language commands. Inventory them separately only when a corresponding workflow
must be replaced; they do not increase the 92-command total.

## 10. Ambiguities and discrepancies to resolve before claiming full support

| ID | Evidence / issue | Required resolution |
|----|------------------|---------------------|
| AE-GAP-01 | P. 2 advertises combined inside text/image support and its 4.2.1 centering fix, but pp. 30–31 retain the older image-only signature. | Obtain the exact 4.2.1 grammar and a tiny reference fixture; do not invent argument positions. |
| AE-GAP-02 | P. 31 repeats old image-padding/default wording while pp. 17–18 and 42 explain the 0.35 mm legacy behavior and new command. | Prioritize the explicit correction, then verify both commands/defaults with a reference case. |
| AE-GAP-03 | P. 48 uses `ALBUM_COLOUR_PAGE_BORDER`, `ALBUM_COLOR_PAGE_BORDER`, `ALBUM_COLOUR_PAGE_TEXT`, `ALBUM_COLOR_PAGE_TEXT`, unlike the formal colour list on p. 38. | Treat these four as unverified names/likely editorial errors, not automatically accepted aliases. |
| AE-GAP-04 | `PAGE_START_VAR` also participates in placeholder substitution, but precise placement of spacing versus group arguments is not demonstrated. | Confirm full argument ordering and missing/extra-substitution handling. |
| AE-GAP-05 | Legacy six-text-field forms are described generally for stamp commands; applicability to newer shapes is not fully spelled out. | Establish a per-shape accepted-argument matrix rather than applying every overload everywhere. |
| AE-GAP-06 | P. 45's `IMAGE_EDGE_PAD` prose mixes horizontal/vertical descriptions; endpoint wording on p. 46 is incomplete. | Verify axes/signs against a small labeled border fixture. |
| AE-GAP-07 | `ROW_START` appears as a family label outside the formal list; `ROW_ALIGN_` is also a prefix, not a complete command. | Do not implement as standalone commands without further evidence. |
| AE-GAP-08 | Current application validation includes `STAMP_ADD_BLANK_IMG`, absent from this manual's command descriptions/examples. | Classify as app extension/unverified; find provenance before treating it as AlbumEasy 4.2.1 syntax. |
| AE-GAP-09 | Nesting/reset rules, automatic overflow behavior, quoted-comment handling and some defaults are not fully formalized here. | Gather isolated evidence for each behavior; report unknowns instead of guessing. |

Current code spot-check: [parser.rs](../../src/external_album/parser.rs) contains
limited validation and placeholder argument ranges. It also restricts some
negative values and page-start forms. This flags audit work, not a complete
support assessment; no application code was changed.

## 11. Command-first planning gate

- [x] Inventory names from command chapters, obsolete section and border language.
- [x] Cross-check examples/history for additional names and record discrepancies.
- [ ] Resolve AE-GAP-01 through AE-GAP-09 with explicit evidence/decisions.
- [ ] Expand each row into precise argument/default/state rules and named-value tables.
- [ ] Complete behavior certification per command across parse/layout/render/edit/export. Initial static coverage/config mapping is recorded in the linked gap analysis; runtime/visual conformance remains open.
- [ ] Define a minimal conformance fixture for every command, overload and interaction.
- [ ] Review the complete matrix before choosing renderer/model implementation order.

This completes the initial manual inventory only. It does **not** complete T001,
certify 4.2.1 compatibility, or authorize native-renderer implementation.
