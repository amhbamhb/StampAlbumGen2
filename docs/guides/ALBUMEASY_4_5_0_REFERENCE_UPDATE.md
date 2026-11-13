# AlbumEasy 4.5.0 reference update

**Decision:** 2026-11-03. AlbumEasy **4.5.0** is the primary compatibility and
baseline renderer. Retain the [4.2.1 inventory](ALBUMEASY_4_2_1_COMMAND_INVENTORY.md)
as a versioned legacy reference, not the complete current target.

**Current command status:** the [unified checkoff list](NATIVE_PDF_COMMAND_CHECKLIST.md)
now inventories both references, spelling aliases, bounded native mappings
and remaining work. Gap statements below describe the original reference
update, not the latest renderer implementation.

## Local help is available without printing

The installed `AlbumEasy45.0.qch` is a Qt compiled-help SQLite database.
Its `FileNameTable` maps filenames to `FileDataTable` records. HTML data uses
Qt compression: a four-byte uncompressed-size prefix followed by zlib data.
Read-only inspection recovered command, font, border and history text into
private session files; the installation and attached changelog were not modified.
No full manual copy is added to the repository.

Use the command reference, not the changelog alone, to establish grammar.
Record the installed help/version hashes in each frozen baseline run.

## Confirmed corrections to earlier gap findings

| Earlier finding | Installed 4.5.0 help clarification | Remaining application work |
|-----------------|------------------------------------|----------------------------|
| GAP-A: image-setting name differs from 4.2.1 | `ALBUM_STAMP_IMG_NEW_SETTING` is preferred. `ALBUM_STAMP_NEW_IMG_SETTING` is an obsolete but functionally identical spelling. `ALBUM_STAMP_IMG_SETTING` retains legacy 0.35 mm units. | Accept/interpret all documented spellings with their actual units; backend currently lacks both mm variants. Generated preferred name is not an error. |
| GAP-B: aspect ratio and margin colour absent from old manual | `ALBUM_STAMP_IMG_ASPECT_RATIO` and `COLOUR_ALBUM_MARGIN_TXT` / `COLOR_ALBUM_MARGIN_TXT` are documented. | Native semantics and imported validation still missing; do not remove these generated commands. |
| GAP-C: padding/stretch/aspect conflation | Stretch-up and fixed aspect ratio are separate settings. New image settings accept optional vertical padding and stretch. | Separate config meanings and preserve intended physical results. |
| GAP-D: six paragraph arguments | `PAGE_TEXT_PARAGRAPH_START` accepts optional independent right padding in 4.5.0. | Existing six-argument emission is not inherently invalid; implement grammar/layout and retain a version-aware legacy export policy. |
| AE-GAP-01: combined image/text syntax | `STAMP_ADD_IMG` has an overload adding inside text, vertical image/text offsets and optional horizontal offsets before optional `NO_FRAME`. | Add exact typed overloads and fixtures; current 6–7 argument validator does not cover this. |
| Obsolete-command count | 4.5.0 also deprecates the earlier mm spelling and `STAMP_BOXES_ADJUST_SAVE` / `RESTORE`. | Do not apply the old inventory's obsolete count to the new target. |

These are documentation-based resolutions, not native runtime certification.
The [application gap report](../reports/ALBUMEASY_4_2_1_APPLICATION_GAPS.md)
retains its legacy 92-name counts; a full 4.5.0 command/support matrix is still
required before claiming complete language support.

## Added capabilities to include in the next matrix

The following are distinct reference additions relative to the old inventory;
parameter enhancements and alternate spellings must be tracked separately.

| Family | Commands / changes to cover |
|--------|----------------------------|
| Image controls | `ALBUM_STAMP_IMG_NEW_SETTING`, `ALBUM_STAMP_IMG_NEW_SETTING_PCNT`, `ALBUM_STAMP_IMG_ASPECT_RATIO`, `ALBUM_STAMP_IMG_GREYSCALE_ON`, `ALBUM_STAMP_IMG_GREYSCALE_OFF`, `ALBUM_STAMP_IMG_HIDE`, `ALBUM_STAMP_IMG_SHOW`; `GRAYSCALE` spelling aliases. |
| Stamp state and captions | `ALBUM_STAMP_SETTINGS_SAVE`, `ALBUM_STAMP_SETTINGS_RESTORE`, `STAMP_FOOTER_TXT_ALIGN`, `STAMP_FOOTER_TXT_PAD`. |
| Shapes | `STAMP_ADD_RT_TRIANGLE`, `STAMP_ADD_PENTAGON`; independently selectable frame edges. |
| Page flow and geometry | `PAGE_START_INFO`, `PAGE_SET_VERTICAL_POS`, `PAGE_QUADRILLE`, `PAGE_ADD_BOX`, `ALBUM_PAGES_CROP_MARKS`. |
| Margins and colours | `ALBUM_PAGES_MARGIN_TXT`, `ALBUM_PAGES_MARGIN_TXT_PAD`, `COLOUR_ALBUM_MARGIN_TXT`, `COLOUR_PAGE_QUADRILLE`, with `COLOR_` aliases. |
| Preprocessing | `$INCLUDE`, `$DEFINE`, `$UNDEFINE`, `$IFDEF`, `$ELSE`, `$ELSEIF`, `$ENDIF`; dependencies, conditional scope and source locations. |
| Enhanced overloads | Column widths/independent padding; independent right padding for paragraphs/rules; multiline headers/footers; title vertical spacing; informational-page count behavior. |

Do not silently expand the original 92-name denominator. Version the new matrix
and distinguish command names, spelling aliases, overloads and semantics.

## Baseline policy

The [core-v1 TXT](../../tests/fixtures/albumeasy/core-v1/album.txt) is the first
small cross-renderer input, rendered with 4.5.0. It does not exercise every
addition above or certify rendering under 4.2.1. Both the external renderer and
future internal renderer must receive the **same frozen bytes**.

Add separate versioned fixtures for each command family. Never adjust a golden
reference merely to make a drifting native result pass. Record a deliberate
specification change and review a new baseline version instead.
