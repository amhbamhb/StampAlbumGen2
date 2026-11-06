# Configuration Parameters Inventory

## Complete Config Parameters from config.rs (UPDATED)

### Isolated native PDF configuration

The native proof now reads a separate, local Git-ignored `config-pdf.toml`,
copied from `config_template.toml`. This is not a switch in the main application:
its existing `config.toml` loader and defaults are unchanged. See the
[manual native proof guide](ALBUMEASY_BASELINE_MANUAL_CLI.md#user-configuration-to-command-state).

The [native bridge](../../src/native_pdf/config.rs) deliberately does not call
the global `Config` loader: it reads only the explicit path, retains `f64`
millimetres, validates required values without silent defaults and never logs
configuration source/credential values. Both `[page]` and legacy root keys are
accepted; `[page]` wins with overridden names reported. `[colors]` likewise
overrides the root `color_album_border` value.

| Native input | Command | Policy |
|---|---|---|
| `[native_pdf.rows].alignment` | Native-only row policy | `left`, `center`, `right`, `justified`; new native default `center` distributes equal internal/outer spaces (retains configured minimum internal gap for nearly full rows). Left/right retain fixed gaps; justified spreads only internal gaps, with centered singleton. Applies to every row, final rows and continuations throughout each explicit section; sections begin new rows. Table optional, key required when present. Legacy generation unchanged; future last-row variants/compatibility mapping not implemented. |
| `[native_pdf.stamp_text].enabled` | Native-only text-footprint proof opt-in | Optional table; false in templates; all fields validated if present; production generator/export unchanged |
| `[native_pdf.stamp_text].width_policy` | Native-only text layout | `wrap_to_frame` wraps to mounting-frame width; `expand_footprint` allows wider text up to its configured cap, then wraps; both preserve explicit newlines and reserve all measured height, with no fixed line-count limit; wrapping normalizes repeated spaces/splits long words without scaling; AlbumEasy compatibility is not certified |
| `[native_pdf.stamp_text].expanded_width_limit` | Native-only expanded text cap unit | Required `percent` or `mm` when table exists; template `percent`; cap applies before padding and is ignored for frame-wrapping, but always validated |
| `[native_pdf.stamp_text].expanded_width_value` | Native-only expanded text cap | Required finite percent >= 100 (120 = 20% wider than mounting frame), or positive absolute mm (30 = 30 mm text width); expanded-mode mm cap below an item's frame is an explicit error; complete footprint also includes per-side padding; template 120 |
| `[native_pdf.stamp_text].horizontal_padding_mm` | Native-only reserved space | Finite nonnegative per-side mm; footprint width = max(frame, text width) + twice padding; independent of physical clearance, page margins and row gaps |
| `[native_pdf.stamp_text].vertical_padding_mm` | Native-only reserved space | Finite nonnegative per-edge mm; footprint includes measured heading/caption heights and gaps plus twice padding; applies to both modes |
| `[native_pdf.stamp_text].text_gap_mm` | Native-only frame/text separation | Finite nonnegative mm between frame and each present text block; absent text adds no gap; template 3 mm |
| `album_page_width_mm`, `album_page_height_mm` | `ALBUM_PAGES_SIZE` | Both required, finite positive mm |
| `page_left_margin_mm`, `page_right_margin_mm`, `page_top_margin_mm`, `page_bottom_margin_mm` | `ALBUM_PAGES_MARGINS` | All required, finite nonnegative mm |
| `album_pages_spacing_h`, `album_pages_spacing_v` | `ALBUM_PAGES_SPACING` | Both required; within-page gaps, not between-sheet spacing |
| `page_border_outer`, `page_border_inner1`, `page_border_inner2`, `page_border_spacing` | `ALBUM_PAGES_BORDER` | All four required; widths zero explicitly disable strokes |
| `[page.even_margins]` with `left_mm`, `right_mm`, `top_mm`, `bottom_mm` | `ALBUM_PAGES_MARGINSE` | Native-only extension; table optional, all four values required if present |
| `[colors].color_album_border` or legacy root `color_album_border` | `COLOUR_ALBUM_BORDER` / `COLOR_ALBUM_BORDER` | Required opaque SVG named colour string; shared legacy colour, template default black |
| `[native_pdf.border_colours]` with `outer`, `inner1`, `inner2` | Native-only extension | Independent named colours; all three required when table present; shipped defaults all black; overrides shared colour |
| `[native_pdf.proof].page_count` | Diagnostic only | Required positive integer PDF page count; template value 3 |
| `[native_pdf.proof].guide_enabled` | Diagnostic only | Required boolean; template value true |
| `[native_pdf.proof].guide_width_mm` | Diagnostic only | Required finite positive mm; template value 0.1; enabled guide must fit content rectangle |
| `[native_pdf.proof].guide_colour` | Diagnostic only | Required opaque SVG named colour; template value black; independent of album border |
| `[native_pdf.crop_marks].enabled` | Native crop-mark opt-in | Optional table; all fields required/validated if present, even disabled; template false |
| `[native_pdf.crop_marks].line_width_mm` | `ALBUM_PAGES_CROP_MARKS` | Finite nonnegative mm fitting PDF points; template 0.1; zero is device hairline, not off |
| `[native_pdf.crop_marks].horizontal_padding_mm`, `.vertical_padding_mm` | `ALBUM_PAGES_CROP_MARKS` | Finite nonnegative mm; templates 1.0/1.0; centres offset inward by padding minus half line width |
| `[native_pdf.crop_marks].arms_mm` | `ALBUM_PAGES_CROP_MARKS` | Finite nonnegative half-length of each cross segment; template 5.0; zero makes degenerate butt-capped segments |
| `[native_pdf.crop_marks].colour` | Native crop colour extension | Required opaque SVG named colour; template black, independent of all border/text colours; AlbumEasy references use black |
| `[native_pdf.decorative_border].enabled` | Native decorative-border opt-in | Optional table; all fields required/validated if present; template false; disabled mode opens no files |
| `[native_pdf.decorative_border].definition_file` | `ALBUM_PAGES_DECORATIVE_BORDER` | String; template empty, nonempty when enabled; relative to explicit config directory, never AlbumEasy/global search; PNG path relative to definition |
| `[native_pdf.decorative_border].image_scale` | Omitted `IMAGE_SCALE` policy | Explicit positive unitless factor; template 1.0; PNG pixels become points before scaling |
| `[native_pdf.decorative_border].corner_padding_h_mm`, `.corner_padding_v_mm` | Omitted `IMAGE_CORNER_PAD` policy | Finite signed mm; templates 0/0; positive moves corners outward |
| `[native_pdf.decorative_border].line_outer_mm`, `.line_inner1_mm`, `.line_inner2_mm`, `.line_spacing_mm` | Omitted `BORDER_LINES` policy | Nonnegative mm; templates all 0; disabled slots retain fixed centreline spacing, not cumulative widths |
| `[native_pdf.decorative_border].line_padding_h_mm`, `.line_padding_v_mm` | Omitted `BORDER_PAD` policy | Signed mm; templates 0/0; positive moves lines outward |
| `[native_pdf.decorative_border].end_offset_h_mm`, `.end_offset_v_mm` | Omitted `BORDER_END_OFFSETS` policy | Nonnegative mm; templates 0/0; endpoints measured from original margin corners |
| `[native_pdf.decorative_border].edge_padding_h_mm`, `.edge_padding_v_mm` | Omitted `IMAGE_EDGE_PAD` policy | Signed mm; templates 0/0; horizontal padding affects left/right edges, vertical padding affects top/bottom edges |
| `[native_pdf.decorative_border].colour` | Native decorative-line colour | Opaque named colour; template black, independent of numeric border; does not recolour PNG artwork |
| `[native_pdf.title].enabled` | Native title extension | Required boolean when title table exists; template false |
| `[native_pdf.title].text` | `ALBUM_PAGES_TITLE` | Required nonempty WinAnsi string; template "Stamp Album"; explicit newlines supported |
| `[native_pdf.title].vspace_after_mm` | `ALBUM_PAGES_TITLE` | Required finite following gap; template 0.0 mm; negative values explicitly allow overlap |
| `[native_pdf.title].minimum_height_mm` | Native reservation hint | Required finite nonnegative mm; template 0.0 = measured height; positive reserves at least requested height |
| `[header].album_pages_title_font`, `.album_pages_title_size` or legacy root keys | `ALBUM_PAGES_TITLE` | Required when title table exists; supported built-in token and finite positive points; templates HN/10 |
| `[colors].color_album_title` or legacy root key | Title colour | Required when title table exists; named opaque colour; shipped black |
| `[native_pdf.header].enabled` | Native header extension | Required boolean when header table exists; template false |
| `[native_pdf.header].font`, `.size_pt` | `ALBUM_PAGES_HEADER` | Independent header font and positive point size; templates HN/8 |
| `[native_pdf.header].text` | `ALBUM_PAGES_HEADER` | Nonempty WinAnsi string with explicit newlines; template "Stamp Album Header" |
| `[header].album_pages_header_position` or legacy root key | `ALBUM_PAGES_HEADER` | L/C/R/LR/RL; templates C; section overrides root |
| `[colors].color_album_header` or legacy root key | Header colour | Named opaque colour; shipped black; section overrides root |
| `[native_pdf.header].padding_mm` | `ALBUM_PAGES_HEADER_PAD` | Finite signed mm; template 0.0; positive raises the header |
| `[native_pdf.header].minimum_height_mm` | Native reservation hint | Finite nonnegative mm; template 0.0 = automatic; positive extends reservation below header |
| `[native_pdf.footer].enabled` | Native footer extension | Required boolean when table exists; template false |
| `[native_pdf.footer].font`, `.size_pt` | `ALBUM_PAGES_FOOTER` | Independent built-in font and positive point size; templates HN/8 |
| `[native_pdf.footer].position` | `ALBUM_PAGES_FOOTER` | L/C/R/LR/RL; template C; independent of page-number position |
| `[native_pdf.footer].text` | `ALBUM_PAGES_FOOTER` | Nonempty WinAnsi string, explicit newlines; template "Stamp Album Footer" |
| `[colors].color_album_footer` or legacy root key | Footer colour | Named opaque colour; shipped black; section overrides root |
| `[native_pdf.footer].padding_mm` | `ALBUM_PAGES_FOOTER_PAD` | Finite signed mm; template 0.0; positive lowers footer away from border |
| `[native_pdf.footer].minimum_height_mm` | Native reservation hint | Finite nonnegative mm; template 0.0 = automatic; positive extends reservation above footer |
| `[native_pdf.footer_number].enabled` | Native numbering opt-in | Required boolean when table exists; template false; also requires existing footer-number enablement |
| `[footer].album_pages_footer_number_enabled` or root key | `ALBUM_PAGES_FOOTER_NUMBER` | Required boolean when native number table exists; template true; section overrides root |
| `[footer].album_pages_footer_number_font`, `.album_pages_footer_number_size` or root keys | `ALBUM_PAGES_FOOTER_NUMBER` | Independent built-in font/positive point size; templates HN/8 |
| `[footer].album_pages_footer_number_position` or root key | `ALBUM_PAGES_FOOTER_NUMBER` | L/C/R/LR/RL using physical parity, not printed label; template C |
| `[footer].album_pages_footer_number_start` or root key | `ALBUM_PAGES_FOOTER_NUMBER` | Signed 32-bit integer; template 1; checked arithmetic; zero/negative labels supported |
| `[footer].album_pages_footer_number_before`, `.album_pages_footer_number_after` or root keys | `ALBUM_PAGES_FOOTER_NUMBER` | Explicit WinAnsi strings, empty allowed; templates "Page " / " of $PAGES$"; every exact `$PAGES$` expands to actual count |
| `[native_pdf.header_number].enabled` | Native header numbering opt-in | Required boolean when table exists; template false |
| `[native_pdf.header_number].font`, `.size_pt` | `ALBUM_PAGES_HEADER_NUMBER` | Independent built-in font/positive point size; templates HN/8 |
| `[native_pdf.header_number].position` | `ALBUM_PAGES_HEADER_NUMBER` | L/C/R/LR/RL using physical parity; template R; independent of plain header/footer |
| `[native_pdf.header_number].start` | `ALBUM_PAGES_HEADER_NUMBER` | Signed 32-bit integer; template 1; checked printed-label arithmetic |
| `[native_pdf.header_number].before`, `.after` | `ALBUM_PAGES_HEADER_NUMBER` | Explicit WinAnsi strings, empty allowed; templates "Page " / " of $PAGES$"; all total tokens expanded |
| `[native_pdf.header_date].enabled` | Native header date opt-in | Required boolean when table exists; template false |
| `[native_pdf.header_date].font`, `.size_pt` | `ALBUM_PAGES_HEADER_DATE` | Independent built-in font/positive point size; templates HN/8 |
| `[native_pdf.header_date].position` | `ALBUM_PAGES_HEADER_DATE` | L/C/R/LR/RL using physical parity; template L |
| `[native_pdf.header_date].format` | `ALBUM_PAGES_HEADER_DATE` | YMD/DMY/MDY, padded fields; template YMD |
| `[native_pdf.header_date].separator`, `.before`, `.after` | `ALBUM_PAGES_HEADER_DATE` | Explicit WinAnsi strings, empty allowed; templates "-" / "Generated: " / ""; literal, no number-token substitution |
| `[native_pdf.header_date].date_source` | Native document date policy | generation/fixed; template generation; local date captured once at generation |
| `[native_pdf.header_date].fixed_date` | Native reproducible override | Strict valid YYYY-MM-DD, years 0001–9999; template 2026-01-01; validated even inactive, used only in fixed mode |
| `[native_pdf.footer_date].enabled` | Native footer date opt-in | Required boolean when table exists; template false |
| `[native_pdf.footer_date].font`, `.size_pt` | `ALBUM_PAGES_FOOTER_DATE` | Independent built-in font/positive point size; templates HN/8 |
| `[native_pdf.footer_date].position` | `ALBUM_PAGES_FOOTER_DATE` | L/C/R/LR/RL using physical parity; template L |
| `[native_pdf.footer_date].format` | `ALBUM_PAGES_FOOTER_DATE` | YMD/DMY/MDY, padded fields; template YMD |
| `[native_pdf.footer_date].separator`, `.before`, `.after` | `ALBUM_PAGES_FOOTER_DATE` | Explicit WinAnsi strings, empty allowed; templates "-" / "Generated: " / ""; literal, no number-token substitution |
| `[native_pdf.footer_date].date_source` | Native document date policy | generation/fixed; template generation; shares one local generation date with header |
| `[native_pdf.footer_date].fixed_date` | Native reproducible override | Strict valid YYYY-MM-DD, years 0001–9999; template 2026-01-01; validated even inactive, used only in fixed mode |

Border colours are case-insensitive. Hex, RGB functions, transparent and unknown
names are rejected, not silently mapped to black. Actual AlbumEasy 4.5 probes
rejected both bare and quoted hex forms, correcting older template claims.
The diagnostic guide uses its own configured colour. Width/colour remain required
even when the guide is disabled. Other colour keys are still unapplied.
The existing main `Config.color_album_border` field/default/emitter is unchanged;
the new native bridge consumes its existing spelling directly.
Without the per-line table, all lines use the explicit shared TOML colour.
When present, the table must be complete, including disabled border slots.
Slot identity is preserved even if an earlier line has zero width. The original
AlbumEasy colour command still sets a single colour, not three arguments.

All implemented native rendering settings/defaults must come from TOML.
Neither the loader nor typed native state provides hidden rendering defaults.
Item-specific values may come from stamp/layout records. Unit conversions,
command semantics and fixed reference-test expectations are not rendering policy.
The values above are template choices, not runtime fallback constants.
The shipped TOML files contain editable defaults for every implemented setting.
Missing, empty or invalid required settings fail explicitly. A TODO in the
native loader requires future UI integration to show these errors with the
affected key and request a correction; that UI is not implemented yet.

Title state uses standard-font advance widths and ascent/descent, explicit line
breaks, and an AlbumEasy-compatible baseline/centering policy. The same resolved
layout drives drawing and the remaining content guide. Reservation includes
final font descent, deliberately more conservative than the reference cursor;
the hint never replaces measured height with a smaller value. The optional
title table is absent only for older/no-title configurations. When present,
every field must be supplied, including while disabled. Custom fonts, automatic
wrapping, general stamp text and production page-break integration remain open.

Headers sit above the top margin boundary, not inside title/stamp space.
The bottom line's baseline is 2 mm above that boundary plus configured padding
(2 mm is verified AlbumEasy command semantics, not a rendering fallback).
Multiline headers grow upwards. Horizontal alignment uses margin bounds:
LR is left on odd physical pages/right on even; RL is the reverse.
The shared measurements drive both drawing and layout reservation. Normal
headers do not consume content; only descent/minimum-hint spill into content
lowers its top boundary before the title is laid out. Physical-page overflow
and unusable content fail explicitly. The optional header table has no header
when absent; all fields remain required when present, including when disabled.
Header numbering is a separate optional command/table. It shares explicit
header padding/minimum height and header colour; the header policy table is
required even when its plain text is disabled. All number fields are required
and validated when the number table exists, including when disabled. Absence
preserves older no-number proofs. Header/footer starts and styles are independent;
both require the same final actual document count. Header dates are separately
supported, as are footer dates; automatic wrapping remains pending.

Header text, header number and title are checked pairwise using nonblank
per-line AFM boxes. Touching edges and overlapping reservation hints alone
are not text collisions. Errors identify the physical page, conflicting
elements and correction keys; every page is preflighted before saving, so
late digit/parity collisions cannot truncate an existing output. Edit the
named TOML values and rerun for now. The code records a future UI warning
and correction dialog TODO; no UI is implemented, and future image elements
are not covered yet. Header date is additionally checked against
header text/number/title using the same mechanism. These checks are not exact glyph outlines.

The optional header-date table requires every explicit field, including when
disabled, and requires the native header padding/hint/colour policy. Generation
mode captures local calendar date once at document creation, not during config
loading or each page. Fixed mode uses its validated override and ignores the
supplied generation context. Both use the same shared header measurement,
reservation and draw state. `resolve_with_context` and the explicit-date writer
allow consistent reporting and deterministic tests; legacy resolve methods
reject generation mode without context. Loader validation uses a separate
preview date and renderer preflights all pages with the actual document date.
Missing/invalid dates are explicit keyed errors with no source values echoed.

Footer dates require the same complete explicit date fields and the native
footer padding/minimum-height/colour policy, independent of text/number style.
Header/footer generation modes share one captured document date, including
footer-only generation. Either may independently choose its own fixed override;
mixed modes preserve fixed values while sharing the generation context where
applicable. Shared date helpers report header/footer-specific keys and retain
the header's original public API. Footer date extends reservation upwards and
is checked against footer text/number/title using actual nonblank AFM line boxes;
the same all-page preflight and future UI correction TODO apply.

Footers sit below the bottom margin. The first baseline is one font-size step
(points converted to mm) plus **0.5 mm** below the boundary, minus padding;
multiline text grows downwards. This verified AlbumEasy placement is command
semantics, not a hidden rendering default. Extra minimum-height reservation
extends upwards without moving glyphs; only spill into content raises its
bottom boundary. Both top/header and bottom/footer constraints apply before
title layout. Off-page descenders and unusable content are explicit errors,
even if AlbumEasy accepts/clips the same configuration.
The optional footer table is absent only for older/no-footer configurations;
all fields and colour are required when present, including when disabled.
Text-footer settings do not borrow or alter the independent page-number
font/size/position/prefix/suffix. Numbering shares explicit footer padding,
minimum-height and colour (the native footer table is required even if text
is disabled). Both native opt-in and existing number-enabled field must be
true to render numbers. All number fields must be present/valid when the
native number table exists, including when disabled. Absent number table
preserves older no-number proofs and reports existing number keys unapplied.
Section-over-root precedence applies to all seven existing number fields.
`PageSettings::resolve_with_page_count` requires the frozen actual total and
valid physical page; `resolve` explicitly refuses active numbering rather
than guessing a total. Loading validates first, second if present, and last
numbered page; rendering preflights every page before saving. A changing
digit count may increase width or cause a collision on a later page.
Measured text/number footer line-box overlaps fail with actionable position/
font/text keys. Negative padding still deliberately permits border overlap.

No automatic even-page mirroring is inferred. All geometry must remain usable;
both base and even-page bounds are validated when loading. Other template
settings are returned as unapplied key names and summarized by the example;
they are not silently advertised as supported native features. The two
templates contain commented examples of the optional even-margin table.

No new fields were added to the main `Config` struct: the new tables currently
belong only to the isolated native configuration bridge. Their application/UI,
album-overlay and imported-command integration remains pending. Renaming the
file later does not require a key migration; pass the intended path explicitly.

### Site Account Credentials

Set `username` and `password` under `[stampworld]` and `[colnect]` in the
local, Git-ignored `config.toml`. The Site Accounts dialog is available on
both the startup page and the album Review page. It loads configured usernames
and indicates whether a password is already saved, without returning passwords
to the browser. Leave a password field blank to retain the saved password;
enter a new password to replace it.

Environment variables take precedence over file settings: `STAMPWORLD_USER`
(or `STAMPWORLD_USERNAME`), `STAMPWORLD_PASS` (or `STAMPWORLD_PASSWORD`),
`COLNECT_USER` (or `COLNECT_USERNAME`), and `COLNECT_PASS`
(or `COLNECT_PASSWORD`).

If Site Accounts reports an empty response, restart with the updated application
binary and reload the browser page. Credential endpoints must be available in
the running server, not just in the page's JavaScript.

Credential regressions can be checked with
`cargo test --lib site_accounts -- --test-threads=1` and, when Node.js is
available, `node --test tests/site_accounts_ui.cjs`.

### AlbumEasy PDF Wait

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `albumeasy_pdf_timeout_secs` | positive integer | 300 | Seconds to wait for a fresh PDF after the AlbumEasy launcher exits |

Set this at the root or under `[runtime]` in global or album configuration.
For large albums on external storage, `600` allows ten minutes. Zero and negative
values are invalid. This setting controls PDF discovery, not a timeout for the
AlbumEasy process itself. An unchanged pre-existing PDF is not accepted.

### Core Stamp Dimensions

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `reference_width_mm` | f32 | 25.0 | Reference stamp width in mm for scaling calculations |
| `reference_height_mm` | f32 | 30.0 | Reference stamp height in mm for scaling calculations |
| `stamp_box_height` | f32 | 30.0 | Maximum stamp height before scaling |
| `stamp_box_adjust` | f32 | 3.0 | Border adjustment around stamps |
| `max_stamps_per_row` | u32 | 5 | Maximum stamps per row |
| `album_stamp_img_new_setting_w_mm` | f32 | - | New stamp image width setting |
| `album_stamp_img_new_setting_h_mm` | f32 | - | New stamp image height setting |
| `album_stamp_img_new_setting_preserve_aspect` | bool | - | Preserve aspect ratio in new stamp settings |
| `row_align_default` | String | "ROW_ALIGN_TOP" | Default vertical alignment for stamp rows. Options: `ROW_ALIGN_TOP`, `ROW_ALIGN_MIDDLE`, `ROW_ALIGN_BOTTOM`. |

| `preserve_aspect` | bool | - | Backwards-compatible alias for preserve aspect ratio |
| `preserve_real_image_size` | bool | false | Don't scale images to stamp_box_height |
| `force_dpi` | f32 | - | Override DPI calculation for pixel->mm conversion |

### Page Layout & Dimensions

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `album_page_width_mm` | f32 | 216.0 | Page width in mm (US Letter) |
| `album_page_height_mm` | f32 | 279.4 | Page height in mm (US Letter) |
| `page_top_margin_mm` | f32 | 10.0 | Top page margin |
| `page_bottom_margin_mm` | f32 | 10.0 | Bottom page margin |
| `page_left_margin_mm` | f32 | 10.0 | Left page margin |
| `page_right_margin_mm` | f32 | 10.0 | Right page margin |
| `album_pages_spacing_h` | f32 | 0.0 | Horizontal within-page spacing, including content insets |
| `album_pages_spacing_v` | f32 | - | Vertical gap between elements on a page |
| `page_fit_conservative_buffer_mm` | f32 | 11.0 | Buffer reserved at bottom of page |
| `usable_height_padding_mm` | f32 | - | Extra padding subtracted from usable height |
| `allow_single_large_stamp_per_page` | bool | false | Allow single large stamp to occupy full page |
| `single_stamp_threshold_pct` | f32 | - | Fraction above which single-row page permitted |
| `max_rows_per_page` | usize | - | Override maximum rows per page |

### Page Borders

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `page_border_outer` | f32 | - | Outer border width |
| `page_border_inner1` | f32 | - | First inner border width |
| `page_border_inner2` | f32 | - | Second inner border width |
| `page_border_spacing` | f32 | - | Spacing between borders |

### AlbumEasy Row Commands

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `row_start_es_enabled` | bool | true | Enable automatic ROW_START_ES |
| `row_start_es_font` | String | "6" | Font for ROW_START_ES |
| `row_start_es_size` | f32 | 6.0 | Font size in points for ROW_START_ES |
| `row_start_es_spacing` | f32 | 1.0 | Horizontal spacing between stamps |
| `row_start_es_vspace` | f32 | 5.0 | Vertical space after ROW_START_ES |
| `row_start_es_marker` | String | "ROW_START_ES" | Custom marker for row starts |

### Header & Footer Configuration

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `album_pages_title_font` | String | "Helvetica-Bold" | Album title font |
| `album_pages_title_size` | f32 | 16.0 | Album title font size |
| `album_pages_header_position` | String | "centre" | Header position: L, R, C, LR, RL |
| `album_pages_footer_number_enabled` | bool | true | Enable page numbering |
| `album_pages_footer_number_font` | String | - | Page number font |
| `album_pages_footer_number_size` | f32 | - | Page number font size |
| `album_pages_footer_number_position` | String | - | Page number position |
| `album_pages_footer_number_start` | i32 | 1 | Starting page number |
| `album_pages_footer_number_before` | String | - | Text before page number |
| `album_pages_footer_number_after` | String | - | Text after page number |

### Text & Spacing Configuration

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `row_font_spacing` | f32 | 2.0 | Spacing between rows |
| `header_font_spacing` | f32 | 3.0 | Header font spacing |
| `global_scale` | f32 | 1.0 | Global scaling factor |
| `header_vspace_before_mm` | f32 | - | Vertical space before headers |
| `header_vspace_after_mm` | f32 | - | Vertical space after headers |
| `text_line_height_mm` | f32 | - | Line height for text |
| `image_spacing_mm` | f32 | - | Spacing between images |
| `median_width_cap_3` | f32 | 36.0 | Width threshold for 3 stamps per row |
| `median_width_cap_4` | f32 | 30.0 | Width threshold for 4 stamps per row |

### Packing & Safety Margins

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `packing_horizontal_safety_mm` | f32 | - | Horizontal safety padding |
| `packing_vertical_safety_mm` | f32 | - | Vertical safety padding |
### Page Text Templates

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `page_text_format` | String | - | Template for PAGE_TEXT commands |
| `page_text_centre_format` | String | - | Template for PAGE_TEXT_CENTRE commands |
| `page_text_right_format` | String | - | Template for PAGE_TEXT_RIGHT commands |
| `page_text_font` | String | "HN" | Default page text font |
| `page_text_size` | f32 | 10.0 | Default page text size |
| `page_text_vspace` | f32 | 1.0 | Default page text vspace |
| `page_text_centre_font` | String | "TN" | Centre text font |
| `page_text_centre_size` | f32 | 8.0 | Centre text size |
| `page_text_centre_vspace` | f32 | 1.0 | Centre text vspace |
| `page_text_right_font` | String | "HN" | Right text font |
| `page_text_right_size` | f32 | 10.0 | Right text size |
| `page_text_right_vspace` | f32 | 1.0 | Right text vspace |

### Stamp Headings

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `stamp_heading_font` | String | "HN" | Font for stamp headings |
| `stamp_heading_size` | f32 | 10.0 | Size for stamp headings |
| `stamp_heading_label` | String | "auto" | Label prefix for stamp headings. Use `auto` to follow catalog family dynamically per stamp; set empty string to hide heading labels. |
| `stamp_heading_in_row_height` | bool | false | Legacy compatibility flag; visible headings are always measured |

Generated stamp rows use `ROW_ALIGN_TOP` with `STAMP_HEADING_VA`, allowing
AlbumEasy to reserve shared heading space and align stamp-box tops even when
captions have different line counts. Caption wording, box dimensions and image
proportions are unchanged; shorter headings leave unused space above their boxes.
This also applies to generated placeholders and commemorative stamp headings.
External imported layouts retain their saved alignment and heading commands.
Regenerate the album TXT/PDF to apply this behavior to existing ordinary albums.

### Review UI Catalog Families

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `review_ui.catalog_family_options` | String[] | Built-in list | Optional searchable catalog family master list for Review UI selectors. Entry format: `code|label|count`. Stamp-specific families are shown first; remaining families appear under "Other catalogs". Use `sn` label as `Scott Number`. |

### Content Control & Header Information

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `include_design` | bool | false | Include design information in headers |
| `include_engraving` | bool | false | Include engraving information in headers |
| `include_watermark` | bool | false | Include watermark information in headers |
| `include_perforation` | bool | false | Include perforation information in headers |
| `include_catalog_numbers` | bool | false | Include catalog numbers in headers |
| `include_series` | bool | true | Include series information in headers |
| `include_country` | bool | true | Include country information in headers |
| `include_date` | bool | true | Include date information in headers |
| `print_flag_cover` | bool | true | Print flag cover page |
| `fetch_wikipedia` | bool | false | Fetch Wikipedia summary |
| `include_wikipedia_content` | bool | false | Legacy Wikipedia alias |

### Color Customization

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `color_album_border` | String | - | Album border color |
| `color_album_footer` | String | - | Album footer color |
| `color_album_header` | String | - | Album header color |
| `color_album_title` | String | - | Album title color |
| `color_album_margin_txt` | String | - | Album margin text color |
| `color_stamp_border` | String | - | Stamp border color |
| `color_stamp_heading` | String | - | Stamp heading color |
| `color_stamp_text` | String | - | Stamp text color |
| `color_stamp_background` | String | - | Stamp background color |

### Flag Cover Settings

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `flag_cover_img_w_mm` | f32 | 120.0 | Flag cover image width |
| `flag_cover_img_h_mm` | f32 | 60.0 | Flag cover image height |
| `flag_cover_page_vspace_top` | f32 | 18.0 | Top space on flag page |
| `flag_cover_image_vspace_after` | f32 | 8.0 | Space after flag image |
| `flag_cover_image_title_font` | String | "Helvetica-Bold" | Flag title font (with image) |
| `flag_cover_image_title_size` | f32 | 36.0 | Flag title size (with image) |
| `flag_cover_image_title_scale` | f32 | 1.0 | Flag title scale (with image) |
| `flag_cover_text_only_vspace_top` | f32 | 24.0 | Top space (text only) |
| `flag_cover_text_only_title_font` | String | "Helvetica-Bold" | Flag title font (text only) |
| `flag_cover_text_only_title_size` | f32 | 40.0 | Flag title size (text only) |
| `flag_cover_text_only_title_scale` | f32 | 1.0 | Flag title scale (text only) |
| `flag_cover_text_only_vspace_after` | f32 | 8.0 | Space after title (text only) |
| `flag_cover_background_color` | String | - | Flag cover background color |

### Series Ordering

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `series_order` | String | "first_date" | Series ordering: first_date, first_sg, alpha, *_desc variants |

### Advanced Layout Control

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `pages_vspace_in_row_height` | bool | false | Legacy compatibility flag; explicit spacing is always measured |

### Debugging & Diagnostics

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `detailed_page_fit_diagnostics` | bool | false | Emit detailed PAGE_FIT diagnostics |
| `detailed_page_fit` | bool | false | Legacy alias for above |
| `emit_placeholder_diagnostics` | bool | false | Emit placeholder expansion diagnostics |
| `placeholder_always_overwrite` | bool | true | When true, placeholders generated with explicit mm/mm_per_px mappings will be recreated/overwritten so their pixel dimensions exactly match emitted STAMP_ADD_IMG box sizes. Set to false to preserve existing files where pixel dimensions already match the desired size. |

### Data Collection & Processing

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `expand_grouped_ranges` | String | "none" | How to expand ranges: none, placeholders, detail_fetch |
| `calibration_csv` | String | - | Path to offline calibration CSV file |

### Custom Markers

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `page_vspace_marker` | String | "" | Legacy compatibility setting; header spacers now use configured numeric values |

### Colnect Pagination and Vertical Spacing

`inline` and `separate_rows` currently place details in the stamp's centre text
column; `minimal` suppresses the detail body. Pagination now counts the exact
formatted text emitted for the selected fields, including explicit blank lines,
instead of independently wrapping Colnect values into shorter estimated lines.
The tallest stamp determines each row's height.

Visible headings are measured using their configured point size. Body text uses
`text_line_height_mm` as a minimum allowance, never smaller than the selected
`row_start_es_size` converted to millimetres. Both legacy height-accounting flags
remain accepted, but neither can disable accounting for visible content.

- `header_vspace_before_mm` and `header_vspace_after_mm` are preserved in output.
- `row_start_es_vspace` is included in row placement.
- `series_row_gap_mm` adds an explicit gap between rows of the same series and
  is included in pagination before any centering is applied.
- `page_fit_conservative_buffer_mm` is honored by every active fit check.
- `max_page_vspace_mm` limits additional top-centering space. Set it to `0` to
  disable centering. Centering never replaces explicit header or row spacing.

Generated `# ROW_HEIGHT` and `# HEADER_HEIGHT` comments let the centering pass
use the same measurements as pagination. `# PAGE_CENTERING` marks only the
automatically added top spacer, allowing repeat processing without accumulating
extra gaps. Horizontal packing and metadata wrapping are unchanged.

Regression commands:

```sh
cargo test --lib layout -- --test-threads=1
cargo test --lib postprocess -- --test-threads=1
cargo test --test page_fit_regression --test page_break_overflow --test usable_height -- --test-threads=1
```

The optional `colnect_layout_spain` test uses `STAMP_LAYOUT_SPAIN_FIXTURE` to
locate a local Spain album. Run it with `--ignored --nocapture`. It copies the
database and images into an isolated workspace and strips login credentials and
runtime paths from copied configurations. `STAMP_LAYOUT_OUTPUT_DIR` can name an
empty directory to retain generated output for PDF inspection. Without that
variable, the temporary workspace is deleted after testing.

## Header Formatting Sub-Section

The `[header_formatting]` section can contain parameters that may override top-level settings:

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `date_format` | String | - | Date format string |
| `label_style` | String | - | Label styling |
| `series_style` | String | - | Series styling |
| `include_currency` | bool | false | Include currency in headers |
| `header_spacing` | f32 | - | Spacing for headers |

*Note: All include_* flags can be placed in header_formatting and will be promoted to top-level*

## Configuration Testing Status

| Parameter | Tested | Valid Values | Invalid Values | Default Verified |
|-----------|--------|--------------|----------------|------------------|
| `reference_width_mm` | ⏳ | 10.0-100.0 | -1, 0, "abc" | ⏳ |
| `reference_height_mm` | ⏳ | 10.0-100.0 | -1, 0, "abc" | ⏳ |
| `stamp_box_height` | ⏳ | 10.0-50.0 | -1, 0, "abc" | ⏳ |
| `max_stamps_per_row` | ⏳ | 1-10 | 0, -1, "abc" | ⏳ |
| `print_flag_cover` | ⏳ | true, false | "yes", 1 | ⏳ |
| `force_dpi` | ⏳ | 72.0-600.0 | -1, 0, "abc" | ⏳ |
| *(All other parameters pending testing)* | | | | |

## Missing Features Analysis

### Potentially Missing Config Parameters

- [ ] `--config PATH` CLI option for custom config file path
- [ ] `--verbose` / `--quiet` CLI logging control  
- [ ] `page_size_preset` - A4, Letter, Legal presets
- [ ] `output_format` - Different output formats
- [ ] `backup_enabled` - Auto-backup before overwrite

### Implementation Priority

1. **High**: Complete config parameter validation
2. **Medium**: Add missing CLI options for config control
3. **Low**: Advanced preset and formatting options

---
**Status**: Config audit in progress - all parameters documented from config.rs
**Next**: Build comprehensive testing matrix for validation
**Last Updated**: 2025-01-11

### Critical Parameters (High Priority)

- [ ] `max_stamps_per_row` - Controls row grouping
- [ ] `row_start_es_spacing` - Controls stamp spacing
- [ ] `reference_width_mm` / `reference_height_mm` - Default dimensions
- [ ] `album_page_width_mm` / `album_page_height_mm` - Page layout
- [ ] `row_start_es_enabled` - Controls ROW_START_ES generation

### Layout Parameters (Medium Priority)  

- [ ] All margin settings (page_*_margin_mm)
- [ ] Border settings (page_border_*)
- [ ] Spacing settings (image_spacing_mm, packing_*_safety_mm)
- [ ] Font and size settings (stamp_heading_*, page_text_*)

### Advanced Features (Lower Priority)

- [ ] Color customization parameters
- [ ] Flag cover settings
- [ ] Wikipedia integration
- [ ] Debugging/diagnostic options

### Parameter Validation Tests

| Test Type | Description | Status |
|-----------|-------------|---------|
| Type validation | Ensure f32/bool/String types accepted | ⏳ |
| Range validation | Test negative/extreme values | ⏳ |
| Missing defaults | Verify fallbacks when None | ⏳ |
| Invalid TOML | Test malformed config files | ⏳ |
| Case sensitivity | Test parameter name variations | ⏳ |

### Parameter Interaction Tests

| Interaction | Expected Behavior | Status |
|-------------|------------------|---------|
| `max_stamps_per_row` + `row_start_es_spacing` | Proper row grouping | ✅ |
| `reference_*_mm` + `album_page_*_mm` | Stamps fit on page | ✅ |
| `force_dpi` + `preserve_real_image_size` | DPI override works | ⏳ |
| Color parameters + AlbumEasy output | Colors applied | ⏳ |

---
**Status**: Config audit complete - 80+ parameters documented  
**Next**: AlbumEasy PDF cross-reference
**Last Updated**: 2025-11-01
