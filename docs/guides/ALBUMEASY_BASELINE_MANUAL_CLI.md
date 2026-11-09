# Manually checking the AlbumEasy baseline

A native page-geometry proof now exists alongside the frozen AlbumEasy baselines.
It can also render an optional repeating page title with measured space reservation.
There is **no native-PDF application CLI option yet**. Sections 1–5 below generate
external AlbumEasy comparisons; the isolated Rust proof runs separately.

The standalone `render_stampalbum` example can optionally receive a TOML file
when its input is a native `.stampalbum` document. This supports the bounded
native-only section-numbering setting; it does not add application routing or
change AlbumEasy TXT/raw import behavior:

```bash
cargo run --example render_stampalbum -- input.stampalbum output.pdf config.toml
```

The optional config argument is rejected for AlbumEasy TXT or raw-command
input. Section numbering requires `[native_pdf.footer_number].enabled = true`,
`scope = "section"`, the existing footer-number enablement, and an explicit
`NATIVE_PAGE_NUMBER_SECTION` command in the native document.

## Native Rust page-settings proof

This example uses `oxidize-pdf` directly, without AlbumEasy or external tools:

```bash
cd /media/ray/program/Rust/StampAlbumGen2
test -e config-pdf.toml || cp config_template.toml config-pdf.toml
cargo run --example native_pdf_page_settings -- /tmp/native-page-settings.pdf
xdg-open /tmp/native-page-settings.pdf
```

The local `config-pdf.toml` has been copied from the template and is Git-ignored.
Edit its `[page]` values, choose a **new output filename**, and rerun to see the
result. Do not copy the template over your edited configuration. The main
application still uses its existing `config.toml`; it is not changed here.

PowerShell equivalent (create `C:\Temp` first if necessary):

```powershell
if (-not (Test-Path config-pdf.toml)) {
    Copy-Item config_template.toml config-pdf.toml
}
cargo run --example native_pdf_page_settings -- C:\Temp\native-page-settings.pdf
Start-Process C:\Temp\native-page-settings.pdf
```

By default the example reads exactly `config-pdf.toml` in the current working
directory, not an ancestor/global configuration. An optional second argument
selects another configuration, including a future renamed `config.toml`:

```bash
cargo run --example native_pdf_page_settings -- /tmp/custom-page-settings.pdf "/path/to/user config.toml"
```

Both arguments are portable paths; quote paths containing spaces. Missing,
malformed or invalid configuration is an error, never a fallback to defaults.
Existing output files are refused.

The template's three pages are **216 x 279.4 mm**, with 10 mm margins, a triple
black border (0.1/0.5/0.1 mm strokes, 1 mm gaps), horizontal/vertical spacing 3/3 mm,
and a **0.1 mm black diagnostic content guide**. Geometry values are preserved,
not normalized to exact Letter (215.9 mm) or A4. They contain no text, stamp rows,
images or export. Horizontal spacing is reflected in the content guide; vertical
spacing is implemented/tested as an inter-element gap, not an outer-page inset.
This is diagnostic geometry, not an album-rendering backend. The numeric
`ALBUM_PAGES_BORDER` command now supports legacy three-argument double borders,
four-argument single/double/triple borders and explicitly disabled borders.
Omitted-command reference cases are represented by explicit zero-width test inputs,
not native runtime defaults.
See the [border verification report](../testing/ALBUMEASY_BORDER_V1.md) for
AlbumEasy's sparse-slot behavior and remaining interactions.

### User configuration to command state

The [native config bridge](../../src/native_pdf/config.rs) reads the existing
page keys directly as `f64` millimetres and constructs the same validated size,
margins, spacing and border command types used by the fixture tests.
There is no intermediate `f32` conversion or decimal formatting. It resolves
and validates base and even-page geometry before the renderer receives state.

Required: width, height, all four margins, both spacing values, all four border
arguments, border colour and the proof settings below. Existing
flat root keys are accepted; explicit `[page]` keys win over stale root values,
and the example reports overridden key names. Invalid winning values fail,
not fall back to valid root values. Set all three border widths to zero for no
border; missing border keys fail, even if all four are omitted.

The shared legacy border colour remains configurable in the existing section:

```toml
[colors]
color_album_border = "black"
```

Edit the existing key/table rather than adding a duplicate. Opaque SVG named
colours are case-insensitive (`pink`, `black`, `green`, `navy`, `lightgrey`, `white`).
The legacy root key is also supported; `[colors]` wins and the overridden name
is reported. An omitted border colour is an error. Invalid names, hex, RGB
functions and transparency fail explicitly without echoing the value.
For independent colours, edit the native table now included in both templates
and the local configuration:

```toml
[native_pdf.border_colours]
outer = "black"
inner1 = "black"
inner2 = "black"
```

Each line may have a different colour, for example `outer = "pink"`,
`inner1 = "green"` and `inner2 = "navy"`. All three fields must be populated
when the table is present, even for disabled lines. These native-only values
override the shared colour; without this table, the explicit shared TOML colour
applies to all three lines. Disabled border slots retain their colour identity.
This is a native extension, not a change to `COLOUR_ALBUM_BORDER` syntax;
that AlbumEasy command still sets one shared colour.

The diagnostic guide has its own TOML colour. Colour never changes border
thickness, spacing or content bounds.

The proof's settings are separate from album command state. Both shipped
templates and the local configuration include:

```toml
[native_pdf.proof]
page_count = 3
guide_enabled = true
guide_width_mm = 0.1
guide_colour = "black"
```

Edit each field independently. All four are required, including width/colour
when the guide is disabled. Page count must be a positive integer representable
as a PDF page count; guide width must be finite and positive, and an enabled
guide must fit its content rectangle. Colours use the same opaque SVG names.
For example, `guide_colour = "pink"` changes only the diagnostic guide;
`outer = "pink"` in the border-colour table changes only the outer album border.
The shared `color_album_border` controls native borders only when per-line
settings are absent.

**Native rendering policy:** rendering settings/defaults originate in TOML,
never hidden runtime constants or `Default` implementations. Stamp/layout
records may supply item-specific data. Mathematical conversions, command
semantics and frozen test expectations are not configurable policy. Future
native commands must follow this same explicit-input contract.
Every implemented setting has an editable default value in the TOML templates;
black is the shipped border/guide colour. Missing, empty or invalid required
values return an error rather than secretly substituting black or another value.
A code TODO at the native loader records that future UI integration must display
these errors, identify the affected key and prompt the user to correct their file.
No native settings UI has been implemented yet.

AlbumEasy 4.5 probes **rejected bare and quoted hex forms**, contrary to earlier
template comments. Use named colours for this verified command. Final border
colour applies globally, including pages preceding a later colour command;
the test-only TXT fixture adapter verifies both `COLOUR_ALBUM_BORDER` and
`COLOR_ALBUM_BORDER`. There is still no production native TXT interpreter.

For different even physical-page margins, uncomment the optional template table
or add this after the other `[page]` keys and before the next section:

```toml
[page.even_margins]
left_mm = 12.0
right_mm = 25.0
top_mm = 22.0
bottom_mm = 16.0
```

All four values are required when that table is present. Page 2 uses these
margins; pages 1 and 3 use the base margins. Without the table, every page uses
base margins with no automatic mirroring. This table is native-proof-only;
the main application's configuration loader has not been extended.

The full copied template contains many settings for unimplemented native
features. The example explicitly warns that they are not applied (227 with
the current template). The bridge exposes sorted `unapplied_keys` and
`overridden_keys`, never their values. General text, custom fonts, images,
runtime options and `max_page_vspace_mm` do not affect this proof yet. General
page/stamp text remains unsupported; repeating title/header/footer text and footer page numbers are supported.
Errors do not echo source lines or credential values.

`NativePdfConfig::load(&Path)` returns a per-run snapshot. The application-facing
contract is `loaded.page_settings` plus `loaded.proof`, both passed to
`write_page_geometry_proof`. The typed command/settings constructors require
explicit border and colour inputs and have no implicit rendering defaults.
Existing command modules remain independent of files and globals. Editing the
file affects the next load, not an existing snapshot. Future main-app integration
can supply the actual `config.toml` path without renaming any configuration keys;
album overlays/imported-command precedence remain separate, unfinished work.

Run configuration checks:

```bash
cargo test --test native_pdf_config -- --nocapture
cargo test --test native_pdf_config -- --include-ignored --nocapture
```

Expected: **6 passed, 1 ignored**, then **7 passed, 0 ignored** with `mutool`.
Checks include both shipped template formats, user edits, fractional values,
all implemented command mappings, precedence, redacted errors, incomplete groups,
even margins, real PDF dimensions and actual rectangle coordinates/widths.

Run the no-fallback and proof-output checks:

```bash
cargo test --test native_pdf_explicit_settings -- --include-ignored --nocapture
```

Expected: **5 passed, 0 ignored** with `mutool`. Missing settings and invalid
values fail explicitly; actual PDF traces verify page count, pink borders,
independently green 0.35 mm guides and guide disabling. Additional checks verify
black template defaults and red/green/blue per-line PDF strokes, including
disabled outer/inner slots without colour shifts.

Run border-colour checks:

```bash
cargo test --test native_pdf_border_colour -- --nocapture
cargo test --test native_pdf_border_colour -- --include-ignored --nocapture
```

Expected: **3 passed, 1 ignored**, then **4 passed, 0 ignored** with `mutool`.
Six valid references (seven pages) and six failure inputs cover colours,
defaults, alias, global last-value semantics and unsupported forms. RGB components
match to 0.00051 (the native PDF library serializes RGB to three decimals);
geometry retains its 0.01 mm tolerance. No ICC/profile/physical colour match
is claimed.

### Repeating page header and independent padding

`ALBUM_PAGES_HEADER (Font Fontsize Position Text)` repeats a header in the top
margin. `ALBUM_PAGES_HEADER_PAD (padding)` shifts the entire header in signed
millimetres: positive raises it away from the border; negative lowers it.
Both have independent [command modules](../../src/native_pdf/commands/page/).
Font size is in points; the twelve built-in font tokens and explicit multiline
WinAnsi text use the [shared measurement engine](../../src/native_pdf/text.rs).

Both templates and local `config-pdf.toml` contain:

```toml
[native_pdf.header]
enabled = false
font = "HN"
size_pt = 8.0
text = "Stamp Album Header"
padding_mm = 0.0
minimum_height_mm = 0.0
```

Position uses `[header].album_pages_header_position`: **L**, **C**, **R**,
**LR** (left odd/right even physical pages), or **RL** (reverse).
Colour uses `[colors].color_album_header`, including opaque named colours such
as `"pink"`. Those sections override legacy root values. Shipped position/colour
defaults are C/black; existing local choices are preserved.
Header font/size/text/padding/hint are independent of title settings.
All fields must be valid when the table exists, even when disabled; absent
table means no header for older configurations. Enable it explicitly to draw.

AlbumEasy 4.5 places the final baseline **2 mm above the top margin boundary**
plus padding. Horizontal alignment uses margin bounds, not the content inset.
Multiline headers grow upwards. Seven cases/nine reference pages were rendered
twice and complete page traces matched. Native tests compare every header glyph,
font and point size within 0.01 mm, including title interaction and LR/RL.

**Layout hint:** zero selects measured height; positive reserves
`max(measured_height, minimum_height_mm)`. Extra reservation extends down from
the measured text box without moving/resizing header glyphs. A normal header
does not push content/title down. If its descent or hint spills into content,
the shared layout lowers the content boundary before placing the title.
This safety reservation is a deliberate native extension, not identical
AlbumEasy pagination. Headers beyond physical-page bounds fail before saving.
Automatic wrapping, custom fonts, general page text, page-number/date expansion,
footers and production UI/CLI/backend wiring remain pending.

```bash
cargo test --test native_pdf_page_header -- --include-ignored --nocapture
```

Expected: **8 passed, 0 ignored** with `mutool`. Tests cover frozen source/PDF
hashes, TOML defaults/precedence/snapshots/errors, signed padding, parity,
all font variants, blank lines, physical fit/spill, no partial output on
invalid later pages, actual pink colour and measured remaining-content guides.

### Repeating page footer and independent padding

`ALBUM_PAGES_FOOTER (Font Fontsize Position Text)` repeats text below the
bottom margin. `ALBUM_PAGES_FOOTER_PAD (padding)` moves it farther down for
positive signed millimetres, closer to the border for negative values.
These have separate [footer](../../src/native_pdf/commands/page/album_pages_footer.rs)
and [padding](../../src/native_pdf/commands/page/album_pages_footer_pad.rs) modules,
reusing header/title font measurement, positioned lines and drawing.

Both templates and local `config-pdf.toml` contain:

```toml
[native_pdf.footer]
enabled = false
font = "HN"
size_pt = 8.0
position = "C"
text = "Stamp Album Footer"
padding_mm = 0.0
minimum_height_mm = 0.0
```

Each value is independent; font sizes use points and padding/hints use mm.
Position is L/C/R/LR/RL with physical odd/even parity, using margin bounds.
Colour reuses `[colors].color_album_footer` or its legacy root equivalent;
the section wins. Shipped colour is black; `"pink"` is supported. Existing
local colour choices are preserved. The native `position` field deliberately
does not reuse the existing page-number footer position: text and page-number
commands are distinct and may coexist later.

AlbumEasy 4.5 references establish the first baseline at
`bottom_margin - font_size_in_mm - 0.5 mm - padding_mm`.
Additional lines grow downwards by shared line advance. Normal footers leave
content unchanged. Zero hint uses measured height; positive reserves at least
the requested height, extending **upwards** without moving or resizing glyphs.
Only spill into content raises its bottom boundary. Header and footer
reservations are resolved before title layout and future stamp placement.

Seven cases/nine reference pages cover small/large fonts, signed/omitted
padding, multiline text, asymmetric margins, parity and simultaneous
header/title. Independent runs matched complete reference page traces.
Native comparisons verify every footer/header/title glyph within 0.01 mm,
actual pink footer colour and both remaining-content guide boundaries.

**Safety difference:** a 25 pt footer with 15 mm bottom margin and 5 mm
padding has off-page descenders even though AlbumEasy creates a PDF.
Native rejects it rather than clipping; use more bottom margin, smaller font
or less padding. Frozen large-font comparisons use 3 mm padding.
Over-wide/unsupported text, physical-page overflow and unusable reserved
content fail explicitly. Missing/invalid table fields never get fallback
values, including when disabled. Absent table means no footer.
Header-number/date expansion, automatic wrapping, custom fonts and production
UI/CLI/backend integration remain separate unfinished items.

```bash
cargo test --test native_pdf_page_footer -- --include-ignored --nocapture
```

Expected: **8 passed, 0 ignored** with `mutool`. Checks include frozen hashes,
explicit TOML defaults/errors/precedence/snapshots, all built-in font layouts,
blank lines, parity, physical fit, combined header/footer/title reservation,
and preserving output when a later page fails preflight.

### Footer page numbering with a known final total

`ALBUM_PAGES_FOOTER_NUMBER (Font Fontsize Position [Start] [Before] [After])`
now has its own [native command module](../../src/native_pdf/commands/page/album_pages_footer_number.rs).
It reuses the measured footer placement, padding, colour and height reservation
without conflating page-number font/position/text with a plain text footer.

Both templates and local `config-pdf.toml` add:

```toml
[native_pdf.footer_number]
enabled = false
scope = "document"
section_start = 1
```

To enable native numbering, set this to true **and** keep
`[footer].album_pages_footer_number_enabled = true`. The native opt-in prevents
the main template's existing number-enabled default from unexpectedly changing
the isolated proof. The remaining existing `[footer]` values are consumed:

```toml
[footer]
album_pages_footer_number_enabled = true
album_pages_footer_number_font = "HN"
album_pages_footer_number_size = 8.0
album_pages_footer_number_position = "C"
album_pages_footer_number_start = 1
album_pages_footer_number_before = "Page "
album_pages_footer_number_after = " of $PAGES$"
```

Section values override the equivalent legacy root keys. All fields are
required when the native number table exists, even when disabled. Empty
prefix/suffix strings are valid. Starting at 7 labels physical page 1 as 7,
page 2 as 8; it does not skip pages. Verified zero/negative starting labels
are supported with signed 32-bit checked arithmetic, not wraparound.
LR/RL follow physical page parity, never printed-label parity.

Every exact `$PAGES$` occurrence in prefix/suffix expands to the actual total
document pages. A three-page document starting at 8 reads "Page 8 of 3",
"Page 9 of 3", "Page 10 of 3". This is a count, not the last printed label.
Explicit multiline prefix/suffix uses the shared WinAnsi measurement policy.

The optional `scope` and `section_start` settings affect only native
`.stampalbum` input, never AlbumEasy TXT or raw AlbumEasy command input.
`scope = "document"` preserves the single document-wide sequence. For
`scope = "section"`, add `NATIVE_PAGE_NUMBER_SECTION` between stamp items to
mark each next section. A marker may occur mid-page: that shared page remains
part of the preceding section, and numbering restarts on the next page that
begins with content from the new section. No years are inferred. The first
section is implicit.

In native section mode, `$SECTION_PAGE$` expands to the one-based page ordinal
within that section, `$SECTION_PAGES$` to the number of pages assigned to the
section, and `$DOCUMENT_PAGES$` to the document total. `$PAGES$` remains an
alias for `$DOCUMENT_PAGES$`. The displayed number is still inserted between
the configured prefix and suffix and is `section_start + section ordinal - 1`.
Use the optional TOML path with the standalone renderer to enable native
section numbering; imported AlbumEasy rendering does not consume that setting.
Review the [default-start and custom-start native PDF samples](../../tests/fixtures/albumeasy/native-section-numbering-v1/README.md)
side by side. They demonstrate the mid-page boundary rule, section counts,
document total and repeated local starts with unchanged page layout.

Colour comes from `[colors].color_album_footer`; padding/minimum height come
from `[native_pdf.footer]`, which must exist even with its text disabled.
Zero hint uses measured height; positive reserves at least that height upward,
without shifting number glyphs. A text footer may coexist at a non-overlapping
position; measured line-box overlap returns an actionable error instead of
drawing text over numbers. Header/footer dates use their own separate commands.

The application-facing numbered layout call is
`PageSettings::resolve_with_page_count(physical_page, final_page_count)`.
The older `resolve(physical_page)` still works without active numbering and
errors when numbering needs a total. The proof renderer supplies its configured
actual page count, preflights every page, then draws each resolved result.
The loader additionally checks first/second/last numbered pages. Width changes
from 9 to 10 or a later collision cannot silently clip or leave a partial PDF.
Final pagination/count solving for full stamp albums is not implemented here.

Seven AlbumEasy 4.5 cases/15 pages were rendered independently twice with
identical complete page traces. Native checks compare every number and
coexisting header/footer/title glyph/font/size within 0.01 mm, total substitution,
signed labels, asymmetric even margins, actual pink colour and content guides.

```bash
cargo test --test native_pdf_footer_number -- --include-ignored --nocapture
```

Expected: **8 passed, 0 ignored** with `mutool`. Includes explicit defaults,
two enablement gates, unknown-key reporting, snapshot/precedence, keyed/redacted
errors, invalid totals, signed overflow, all font variants, 9-to-10 exact-width
boundaries, collisions and no output truncation on invalid later pages.
The native clipping/reservation safety differences remain intentional;
production app/UI/CLI wiring remains pending.

### Header page numbering and overlap diagnostics

The separate [header-number command](../../src/native_pdf/commands/page/album_pages_header_number.rs)
implements `ALBUM_PAGES_HEADER_NUMBER (Font Fontsize Position [Start] [Before] [After])`.
Both templates ship these explicit, independently editable values:

```toml
[native_pdf.header_number]
enabled = false
font = "HN"
size_pt = 8.0
position = "R"
start = 1
before = "Page "
after = " of $PAGES$"
```

Enable the table in your local `config-pdf.toml` to test it. Every field must
exist and validate when the table is present, even when disabled. Font, size,
position and signed printed start are independent of plain header text and
footer numbering. L/C/R/LR/RL follow physical-page parity. Empty before/after
are valid; every exact `$PAGES$` expands to the final document count.
The existing `[native_pdf.header]` table must also exist: its padding and
minimum-height hint, plus `[colors].color_album_header`, are shared by text
and numbered headers. No additional hidden defaults are supplied.

The last baseline is 2 mm above the top margin plus signed header padding;
multiline numbering grows upwards. Extra minimum-height reservation extends
downwards without moving glyphs. Both header reservations constrain the title
and future stamp area. Footer numbering retains its independent start/style.

Implemented top text is checked pairwise: header text vs header number,
header text vs title, header number vs title. Checks use nonblank per-line AFM
boxes, not the larger reservation hints; touching boxes are not overlaps.
They are conservative line-box checks, not exact glyph-outline certification.
Signed padding may still intentionally overlap the border.

On a text collision, the proof fails **before saving any PDF**, identifying
the physical page, conflicting elements and relevant TOML correction keys.
For testing, edit the named values and rerun: for example put plain header
text at `album_pages_header_position = "L"` and its number at
`native_pdf.header_number.position = "R"`, shorten text/prefixes or reduce
their font sizes. Changing a shared padding moves both header elements and
does not fix a text-vs-number collision. No automatic movement, clipping or
interactive choice is implemented. A `TODO(native-pdf-ui)` records the future
UI notification/correction dialog; header/footer dates are also checked, while
images remain unimplemented.

Seven AlbumEasy 4.5 cases/14 pages were independently repeated with identical
complete page traces. Actual native glyph/font/size comparisons use 0.01 mm,
including multiline, all positions, signed starts, total tokens, asymmetric
even margins and simultaneous header/title/footer numbering. Pink colour,
hint reservation and guide boundaries are checked separately.

```bash
cargo test --test native_pdf_header_number -- --include-ignored --nocapture
```

Expected: **9 passed, 0 ignored** with `mutool`. Also verifies explicit defaults,
missing/invalid/redacted settings, snapshot/unapplied keys, all twelve fonts'
9-to-10 width boundary, late-page overlaps and preservation of existing output.
The shared line-intersection unit test runs with
`cargo test --lib native_pdf::text::tests`. The same Cargo commands work in
Windows PowerShell with MuPDF on PATH; Windows runtime remains unverified.

### Header generation date and fixed-date override

The separate [header-date command](../../src/native_pdf/commands/page/album_pages_header_date.rs)
implements `ALBUM_PAGES_HEADER_DATE (Font Fontsize Position Format [Separator] [Before] [After])`.
These explicit defaults are supplied by both templates:

```toml
[native_pdf.header_date]
enabled = false
font = "HN"
size_pt = 8.0
position = "L"
format = "YMD"
separator = "-"
before = "Generated: "
after = ""
date_source = "generation"
fixed_date = "2026-01-01"
```

All fields are required and validated when the table exists, even when disabled.
The date uses the independent font/size/position above, and shares
`[native_pdf.header]` padding/minimum height and header colour. That header
policy table is required even when its plain text is disabled.

Formats are `YMD`, `DMY`, `MDY`: for 3 November 2026 they produce `2026-11-03`,
`03-11-2026`, `11-03-2026` with the default separator. Month/day have two digits
and year four. Separator, before and after are editable WinAnsi strings;
empty strings are valid, so an empty separator yields `20261103`.
Explicit newlines use the existing measured multiline header behavior.
Date prefixes/suffixes are literal; `$PAGES$` substitution belongs to numbering.

`date_source = "generation"` captures the local calendar date once for each
document, outside the page loop. It is not taken from the machine repeatedly
for each page and not frozen when configuration is first loaded.
`date_source = "fixed"` uses the explicit ISO `fixed_date` for deterministic
proofs/reprints; it overrides a supplied generation context. The inactive
fixed override is still validated, but it is never used in generation mode.
Valid fixed dates must use exact `YYYY-MM-DD`, years 0001 through 9999, with
calendar/leap-year checks. Invalid date values return keyed errors without
echoing configuration source. No silent switch from local time to UTC occurs.

For callers, `capture_generation_date()` obtains the context;
`resolve_with_context(page, final_count, generation_date)` reuses it for
measurement. `write_page_geometry_proof` captures once itself, or
`write_page_geometry_proof_with_date` accepts the caller's captured date for
deterministic tests and consistent reporting. Old resolving methods still
work for fixed/disabled dates but explicitly refuse a generation-date layout
without its context. Loader geometry validation captures a separate preview
date; the renderer revalidates every page with the actual document date.

Date-vs-header text, date-vs-header number and date-vs-title use the shared
nonblank AFM line-box overlap checker. The date reservation constrains the
title and future stamp area; a height hint does not resize/move date glyphs.
Errors identify physical page and correction keys before saving any PDF.
For testing, place date left, header text centre and number right, then shorten
text or reduce font sizes if they collide. Edit TOML and rerun; the code TODO
requires a future UI warning/correction dialog, not automatic repositioning.

Seven real AlbumEasy cases/11 pages were independently repeated with identical
complete page traces. Their observed generation date is frozen alongside
15 source/PDF/date hashes. Native comparisons use that fixed date, not today's
clock, so they remain repeatable. Checks compare glyph/font/size within
0.01 mm for all formats/positions, multiline, separators, asymmetric even
margins and simultaneous header text/numbers/title/footer numbers. Pink colour
and hint/content-guide boundaries are checked separately.

```bash
cargo test --test native_pdf_header_date -- --include-ignored --nocapture
```

Expected: **10 passed, 0 ignored** with `mutool`. Covers fixed/generation source,
one date on every page, year-boundary injected contexts, leap years, strict
redacted settings, all twelve font variants' exact-width boundary, overlaps,
title separation and no output/truncation on invalid later pages. The same
Cargo command works in Windows PowerShell with MuPDF on PATH; compilation was
checked on Windows GNU, but Windows execution/printing remains unverified.

### Footer generation date and shared document date

The separate [footer-date command](../../src/native_pdf/commands/page/album_pages_footer_date.rs)
implements `ALBUM_PAGES_FOOTER_DATE (Font Fontsize Position Format [Separator] [Before] [After])`.
Both templates provide explicit editable defaults:

```toml
[native_pdf.footer_date]
enabled = false
font = "HN"
size_pt = 8.0
position = "L"
format = "YMD"
separator = "-"
before = "Generated: "
after = ""
date_source = "generation"
fixed_date = "2026-01-01"
```

All fields must exist and validate when the table exists, including when
disabled. The native footer policy table is required even when its text is
disabled. Date font/size/position/format/text are independent of plain footer
and numbering; padding/minimum-height/colour share the explicit footer policy.
The shared date helper now reports the appropriate header/footer keys while
preserving the original public header parsing contract.

`YMD`, `DMY`, `MDY`, editable/empty separators and literal prefixes/suffixes
work as described for header dates. Strict fixed overrides validate leap years
and exact `YYYY-MM-DD` in years 0001 through 9999, even when inactive.
Header and footer in generation mode share **one local date captured for the
document**, not one clock read each. Either can instead use its own explicit
fixed override. Mixed modes preserve that fixed date while the other uses the
generation context. A footer-only generation configuration also captures a
date. The explicit-context resolver/writer supports deterministic testing,
and legacy resolving refuses either generation date without context.

Measured placement reuses the footer's first baseline: bottom margin minus
one font-size step in mm, minus 0.5 mm, minus signed padding. Multiline dates
grow downwards. The minimum-height hint extends reservation upwards, without
moving/resizing glyphs; only content spill raises the lower content boundary.
Native rejects off-page descenders even if AlbumEasy accepts the configuration.
The 25 pt reference uses safe 3 mm padding, not the known clipping 5 mm case.

Footer date is checked against plain footer, footer numbering and title using
the shared nonblank AFM line-box checker. Errors identify the physical page
and TOML correction keys before any PDF is saved; late-page failure preserves
an existing output. Date left, plain footer centre and number right are useful
testing choices, not guaranteed to fit arbitrary text. Shorten the date text/
prefixes or reduce font sizes if needed, then rerun. Shared padding moves all
footer elements, so it does not solve their horizontal collisions. The code
TODO records future UI warnings and correction choices; no interactive UI,
automatic repositioning or exact glyph-outline certification is claimed.

Seven actual AlbumEasy cases/11 pages were independently repeated with
identical full-page traces, with observed generation date frozen for stable
native tests. Fifteen hashes cover input/PDF/date identity. Native comparisons
verify every coexisting glyph/font/size within 0.01 mm, including all formats/
positions, multiline, empty/custom separators, 8/16/25 pt and asymmetric even
margins. Pink footer colour and both content-guide boundaries are checked.

```bash
cargo test --test native_pdf_footer_date -- --include-ignored --nocapture
```

Expected: **10 passed, 0 ignored** with `mutool`. Includes generation/fixed/
mixed contexts, shared date on all pages, year-boundary injection, calendar
validation, keyed/redacted errors, defaults/precedence/snapshot/unapplied keys,
all twelve standard-font exact-width boundaries, overlaps and no output
truncation. The same Cargo command works in Windows PowerShell with MuPDF on
PATH; Windows execution and physical printing remain unverified.

### Crop marks without resizing the page

`ALBUM_PAGES_CROP_MARKS ([line] [horizontal-padding] [vertical-padding] [arms])`
now has a separate native command module. Add the complete optional table to
an older local proof config, or edit the disabled table in either template:

```toml
[native_pdf.crop_marks]
enabled = true
line_width_mm = 0.1
horizontal_padding_mm = 1.0
vertical_padding_mm = 1.0
arms_mm = 5.0
colour = "black"
```

All six fields are required and validated if the table exists, even when
disabled. Absence disables this extension for older configs. Dimensions are
finite nonnegative millimetres and must fit PDF point coordinates after layout.
Shipped values match AlbumEasy 4.5's optional argument defaults, but the native
typed command/loader do not supply hidden defaults. This is not a TXT importer.
`enabled = false` is the off switch; width zero is a PDF device hairline.
`arms_mm` is the half-length of each segment (5 mm means a 10 mm segment).
Zero arms produce degenerate butt-capped segments, not useful cutting guides.
Opaque named `colour` is independently editable; black matches AlbumEasy,
including when its album border is pink. Native colour overrides are an
extension, not a new AlbumEasy colour command.

**Verified behavior differs from the help's apparent intent:** AlbumEasy keeps
the configured PDF page dimensions and content positions. Four crosses are
centred inward from its physical corners by padding minus half the line width.
The outer arms can therefore extend beyond the page, including with the
shipped 1/1 mm padding and 5 mm arms. Native rendering deliberately preserves
that geometry, reports a physical-page warning with correction keys, and
leaves clipping to the PDF page boundary. It does not silently expand paper,
add a trim box, shift text or reserve more stamp space.

For fully visible marks try width **0.25**, horizontal padding **8**, vertical
padding **9**, arms **2.5** mm. Check their position relative to your border and
header/footer: crop marks do **not** participate in text-overlap avoidance or
reserve content space. No automatic repositioning is implemented. A code TODO
records future UI clipping warnings/correction choices. Hairline thickness
depends on the viewing/printing device; its clipping diagnostic uses the path
centreline, not a certified device-pixel footprint.

Nine synthetic AlbumEasy cases/11 pages were independently repeated with
identical full-page traces. Frozen references cover omitted/default/partial
arguments, custom padding/line/arms, fractional landscape pages with asymmetric
even margins, zero dimensions and independent black crop marks/pink borders.
Tests compare actual native/reference stroke topology, transformed endpoints,
widths/colours and MediaBox dimensions, plus unchanged measured text/content,
explicit config/redaction/snapshots and output preservation:

```bash
cargo test --test native_pdf_crop_marks -- --include-ignored
```

The two ignored-by-default path/colour tests require `mutool` on `PATH`; the
remaining tests do not. This remains an isolated native proof, not production
CLI/UI/backend integration or physical print certification.

### Decorative corner and edge borders from a definition file

The bounded native `ALBUM_PAGES_DECORATIVE_BORDER` implementation supports a
transparent PNG corner ornament, optional side ornaments and optional
connecting lines. Definitions need at least one `IMAGE_CORNER` or
`IMAGE_EDGE_LEFT`, `IMAGE_EDGE_RIGHT`, `IMAGE_EDGE_TOP` or `IMAGE_EDGE_BOTTOM`
command. Other supported commands are `IMAGE_SCALE`, `IMAGE_CORNER_PAD`,
`IMAGE_EDGE_PAD`, `BORDER_LINES`, `BORDER_PAD` and `BORDER_END_OFFSETS`.
Unsupported syntax fails explicitly; no partially rendered border is returned
as success. This is a border-definition parser, not a general album TXT
importer. The installed artwork is not copied into this repository. Committed
test artwork is original synthetic material.

Copy the **complete** disabled `[native_pdf.decorative_border]` table from the
template into an older local proof config. Set `enabled = true` and
`definition_file = "my-border.txt"` (or a quoted absolute path). Create that
definition beside the config and put your PNG beside the definition:

```text
IMAGE_CORNER("my-ornament.png")
IMAGE_EDGE_LEFT("my-side-ornament.png")
IMAGE_EDGE_TOP("my-horizontal-ornament.png")
IMAGE_EDGE_PAD(3 -2)
IMAGE_SCALE(1.0)
IMAGE_CORNER_PAD(1 -4)
BORDER_LINES(0.25 0.1 0 1.25)
BORDER_PAD(0 -1)
BORDER_END_OFFSETS(20 15)
```

One command per line is supported, with optional trailing semicolon and `#`
comments outside quoted filenames; later repeated commands win. Filenames are
double-quoted, without quote escapes or multiline continuation. PNGs must
decode successfully and have an alpha channel. PNG dimensions are pixels
treated as PDF points, multiplied by the positive scale factor. Corners are
mirrored left/right and top/bottom, **not rotated**, preserving non-square
artwork proportions. Positive corner/line padding moves outward; negative
moves inward. Line endpoints are offset from the original margins. Connecting
line centrelines advance by spacing alone, including disabled slots; widths
are not added to that advance.

Edge PNGs are positioned at the center of their page side and keep their
intrinsic scaled dimensions; they are neither stretched nor tiled. Right and
bottom edges mirror the corresponding left/top image. `IMAGE_EDGE_PAD(h v)`
moves left/right artwork horizontally by signed `h` mm and top/bottom artwork
vertically by signed `v` mm. Edge placement honors asymmetric margins on odd
and even pages. Edge artwork, like corners, does not reserve content space.

Every TOML table field is explicit and validated even when disabled; disabled
mode does not open a definition/PNG, and its filename may be empty. A missing
table disables the feature for older configs. Definition commands override
the corresponding TOML omitted-command policy: `image_scale` defaults in the
shipped template to 1.0, all numeric padding/line/spacing/end-offset fields
to 0.0, and `colour` to black. There are no runtime numeric fallback defaults.
The independent named `colour` affects connecting lines only, not the artwork.

`NativePdfConfig::load` resolves relative definition paths against the config
file's directory; the PNG resolves against the definition directory. It does
not search AlbumEasy's installation or silently use the working directory.
The in-memory `from_toml` entry point requires absolute asset paths when enabled;
`from_toml_in` accepts an explicit base directory. Use forward slashes in TOML
paths for portable examples, or TOML literal strings for Windows backslashes.
The loaded definition and decoded PNG are snapshotted for the run, so edits or
deletions after loading cannot change its output.

As verified against AlbumEasy, decorative borders coexist with numeric borders
and **do not reserve space or move content**. Base/even margins position them.
Check your title/header/footer and body clearance: conservative image rectangles
(including transparent pixels) and actual connecting-line footprints produce an
actionable warning if they intersect the remaining content area. A separate
warning identifies physical-page clipping. Neither warning silently resizes or
moves anything. Exact alpha-mask/text collision certification is not implemented;
code TODOs retain future UI correction choices. Numeric overflow/unusable line
geometry is an error, and all pages preflight before writing.

Seven synthetic cases/eight pages were independently repeated with identical full
page traces. Tests compare all actual native/reference image and alpha-mask
transforms, dimensions, line topology/width/colour/coordinates, MediaBox and
reference content anchors. Raster samples also verify colored and transparent
pixels across mirrored corners and edges. Parser, explicit config,
relative-path/snapshot, missing/corrupt/no-alpha PNG, numeric safety and
output-preservation tests run with:

```bash
cargo test --test native_pdf_decorative_border -- --include-ignored
```

Two ignored-by-default path/raster tests require `mutool` on `PATH`. Windows
runtime/physical printing and production app/UI/backend integration remain
pending.

### Repeating page title and measured-height hint

`ALBUM_PAGES_TITLE (Font Fontsize Title [vspace])` places a centred title on each
ordinary page. Font sizes are **points**; the optional following space is **mm**.
AlbumEasy 4.5 references verified 8 pt and 25 pt Helvetica, multiline 16 pt
Times Bold, omitted/zero spacing equivalence and negative following spacing.
The [title command](../../src/native_pdf/commands/page/album_pages_title.rs)
uses the [shared font-measurement foundation](../../src/native_pdf/text.rs).

Both templates and local `config-pdf.toml` now contain this table:

```toml
[native_pdf.title]
enabled = false
text = "Stamp Album"
vspace_after_mm = 0.0
minimum_height_mm = 0.0
```

Set `enabled = true` to see the title. All fields are required when the table is
present, even when disabled. An older explicit native configuration without this
extension has no title. `text` uses TOML string escapes: `"First\nSecond"` gives
two lines. Font and size reuse `[header].album_pages_title_font` and
`album_pages_title_size`; colour reuses `[colors].color_album_title`. These keys
also accept legacy root spellings, with section values winning. The shipped
defaults are HN / 10 pt / black; existing local font/size/colour edits are
preserved. Header position does not change the title: the command is centred.

Supported built-in font tokens: HN/HB/HI/HS (Helvetica), TN/TB/TI/TS (Times),
CN/CB/CI/CS (Courier), normal/bold/italic/bold-italic respectively. Text must be
representable in WinAnsi; unsupported characters/fonts, control characters,
empty text and over-wide lines fail explicitly. Custom fonts, shaping and
automatic wrapping are not implemented. Insert explicit newlines or choose a
smaller font rather than relying on clipping, substitution or silent resizing.

The minimum-height value is the requested layout “hint”:

- `0.0`: automatic measured line-box height, including final font descent.
- A positive value: reserve `max(measured_height_mm, minimum_height_mm)`.
- A smaller-than-measured hint never shrinks the text box or changes font size.
- `vspace_after_mm` is separate additional space. Negative values intentionally
  raise the following-content boundary and can overlap the title; use nonnegative
  values when collision-free spacing is desired.

Font advance widths, ascent/descent and line count are measured once in command
state. Resolution centres those same measured lines and returns both their PDF
baselines and the remaining-content boundary. The renderer draws from that
result; the diagnostic guide shows the area left for future stamp rows.
Increasing the hint moves the guide down **without moving or resizing the title**.
Both page parities and guide fit are validated before any PDF is saved.

**Compatibility boundary:** title glyph positions match the reference to 0.01 mm;
native reservation additionally includes the final descent to protect stamp
placement. AlbumEasy's observed cursor stops at the last title baseline, so
native remaining space is deliberately more conservative by that descent
(before any minimum-height extension). This native safety policy is not claimed
as identical AlbumEasy pagination. AFM advance/line metrics are not exact glyph
outline bounds (particularly italic side overhang); physical/font-substitution
equivalence is unverified.

Run the title slice checks:

```bash
cargo test --test native_pdf_page_title -- --include-ignored --nocapture
```

Expected: **7 passed, 0 ignored** with `mutool`. Checks include frozen identities,
12 built-in font tokens, 8/25 pt measurements, multiline/blank lines, keyed errors,
legacy precedence, reload snapshots, exact width fit, just-too-wide rejection,
vertical fit/overflow, smaller/larger hints and no PDF on invalid layout.
Actual PDF traces compare every title glyph's position, font and size against
five frozen AlbumEasy references (six pages); both 0 mm and 25 mm hints verify
the serialized guide boundary. General stamp/row composition and the current
application layout engine remain unchanged.

### Frozen command checks

Run native fixture-based tests without external tools:

```bash
cargo test --test native_pdf_page_settings -- --nocapture
```

Expected: **5 passed, 1 ignored**. To include independent inspection of actual
PDF strokes, put `mutool` on PATH and run:

```bash
cargo test --test native_pdf_page_settings -- --include-ignored --nocapture
```

Expected: **6 passed, 0 ignored**. The test reuses page-size/margins/spacing
fixtures. Only page geometry and spacing-state semantics are implemented;
reference TXT text and rows are not silently discarded by a new importer:
there is no native TXT importer yet. The tiny numeric-fixture reader exists
only in tests, not as production parsing logic.

Run the border tests separately:

```bash
cargo test --test native_pdf_page_border -- --nocapture
cargo test --test native_pdf_page_border -- --include-ignored --nocapture
```

Expected: **3 passed, 1 ignored**, then **4 passed, 0 ignored** with `mutool`.
Ten frozen AlbumEasy PDFs cover 15 border strokes and ten text anchors.
Native stroke widths, positions, content guides and page transforms are checked
within 0.01 mm; whole-page native/reference trace equality is not claimed.

Windows GNU library/example cross-check passed; Windows execution and printing
of the native proof remain unverified.

## Prerequisites

- Rust/Cargo.
- AlbumEasy 4.5.0 for generating a fresh candidate.
- `mutool` (MuPDF) on PATH for geometry and strict page-trace inspection.
- Text extraction uses the application's existing `oxidize-pdf` Rust dependency;
  no `pdftotext` installation is required.

The commands below are for Linux. Run them in the same terminal so the `work`
variable remains available. Do not generate PDFs inside the frozen fixture
directory or overwrite its reference PDF.

## 1. Run basic checks

```bash
cd /media/ray/program/Rust/StampAlbumGen2
cargo test --test albumeasy_baseline -- --nocapture
```

Expected: **4 passed, 0 failed, 2 ignored**.

These check frozen input/PDF checksums, extract and assert reference text with
`oxidize-pdf`, and test rejection of drift, missing text and invalid PDFs.
The ignored tests require MuPDF and, for one test, a candidate PDF.

## 2. Check the saved reference PDF

```bash
cargo test --test albumeasy_baseline reference_geometry_text_and_negative_controls -- --ignored --nocapture
```

Expected: **1 passed**. This checks two pages, page dimensions, 11 frame
positions/dimensions and 20 required-text assertions. Negative controls verify
that changed geometry and page counts are rejected.

## 3. Generate a fresh PDF with AlbumEasy

Substitute your installation path if necessary:

```bash
work=$(mktemp -d /tmp/albumeasy-core-check-XXXXXX)
cp tests/fixtures/albumeasy/core-v1/album.txt "$work/album.txt"

(cd "$work" && QT_QPA_PLATFORM=offscreen \
  /home/ray/work/AlbumEasy/AlbumEasy.sh -c album.txt)
```

Continue only if AlbumEasy succeeds and creates a fresh `album.pdf`:

```bash
test -s "$work/album.pdf" && printf 'Candidate: %s\n' "$work/album.pdf"
```

The source is self-contained; no production album, database or local
configuration needs to be changed.

## 4. Compare the candidate with the frozen reference

Run from the repository root in the same terminal:

```bash
ALBUMEASY_BASELINE_CANDIDATE="$work/album.pdf" \
cargo test --test albumeasy_baseline -- --include-ignored --nocapture
```

Expected: **6 passed, 0 failed, 0 ignored**, with exit code zero.

The strict comparison checks complete MuPDF page traces, including text
positions/fonts and vector geometry/styles. Document filenames and PDF metadata
are excluded; PDF byte checksums are not expected to match because timestamps
can differ.

A mismatch fails the test. It does **not** update the reference. Missing tools
or a missing candidate also fail rather than silently skipping.

This is an AlbumEasy-to-AlbumEasy core-v1 check, not certification of every
AlbumEasy command or a future native renderer.

## 5. Inspect visually

```bash
xdg-open "$work/album.pdf"
```

Compare against the [reference PDF](../../tests/fixtures/albumeasy/core-v1/reference.pdf).
For a physical check, print page 2 at **100% scale**, with fit-to-page disabled:
the horizontal calibration frame is 100 x 10 mm and the vertical frame is
10 x 50 mm.

## Windows PowerShell

From the repository root, run the same basic/reference Cargo commands above.
Create an isolated candidate directory and substitute your AlbumEasy path:

```powershell
$work = Join-Path ([System.IO.Path]::GetTempPath()) ("albumeasy-core-check-" + [guid]::NewGuid())
New-Item -ItemType Directory -Path $work -ErrorAction Stop | Out-Null
Copy-Item tests/fixtures/albumeasy/core-v1/album.txt (Join-Path $work 'album.txt') -ErrorAction Stop
Push-Location $work
try {
    & 'C:\path\to\AlbumEasy\AlbumEasy.exe' -c .\album.txt
    if ($LASTEXITCODE -ne 0) { throw "AlbumEasy failed: $LASTEXITCODE" }
    if (-not (Test-Path .\album.pdf)) { throw 'AlbumEasy did not create album.pdf' }
} finally {
    Pop-Location
}

$env:ALBUMEASY_BASELINE_CANDIDATE = Join-Path $work 'album.pdf'
try {
    cargo test --test albumeasy_baseline -- --include-ignored --nocapture
    if ($LASTEXITCODE -ne 0) { throw "Baseline checks failed: $LASTEXITCODE" }
} finally {
    Remove-Item Env:ALBUMEASY_BASELINE_CANDIDATE
}
Start-Process (Join-Path $work 'album.pdf')
```

Windows execution remains **unverified**; these are portable invocation
instructions, not a claim of a passing Windows run.

## Troubleshooting and evidence

- `cannot run mutool`: install MuPDF and put `mutool` on PATH.
- `oxidize-pdf cannot open` or `oxidize-pdf cannot extract`: preserve the
  candidate and error for investigation. There is no alternate text extractor
  or success-shaped fallback.
- `ALBUMEASY_BASELINE_CANDIDATE is required`: set the environment variable as
  shown above.
- Checksum or geometry/trace mismatch: preserve the candidate and diagnostics
  for investigation; do not edit the golden fixture to make the test pass.
- `warning: ICC support is not available`: the verified Linux run also reported
  this MuPDF warning. Color-profile fidelity is not certified.

Record command exit codes and retain the candidate when investigating a failure.
Temporary candidate files remain in the displayed directory for inspection;
no source input is moved or deleted.

See the [baseline execution report](../testing/ALBUMEASY_CORE_V1_BASELINE.md)
for frozen identities, coverage, tolerances and limitations, and the
[checker source](../../tests/albumeasy_baseline.rs) for assertions.

For the separate page-size command fixtures, measurements, configuration mapping
and manual reproduction commands, see
[ALBUM_PAGES_SIZE conformance](../testing/ALBUMEASY_PAGE_SIZE_V1.md).
That bundle is not yet wired into the core-specific Cargo checker.

The next command bundle,
[ALBUM_PAGES_MARGINS conformance](../testing/ALBUMEASY_MARGINS_V1.md), covers
asymmetric/fractional margins, even-page overrides and failure diagnostics.
It likewise has separate manual checks rather than core-runner integration.

[ALBUM_PAGES_SPACING conformance](../testing/ALBUMEASY_SPACING_V1.md) adds isolated
horizontal/vertical spacing probes, measured text/frame deltas and missing-input
diagnostics. This bundle is also separate from the core-specific Cargo runner.
