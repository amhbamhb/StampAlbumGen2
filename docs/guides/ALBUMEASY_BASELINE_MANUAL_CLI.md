# Manually checking the AlbumEasy baseline

A native page-geometry proof now exists alongside the frozen AlbumEasy baselines.
It can also render an optional repeating page title with measured space reservation.
There is **no native-PDF application CLI option yet**. Sections 1–5 below generate
external AlbumEasy comparisons; the isolated Rust proof runs separately.

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
`overridden_keys`, never their values. General text, custom fonts, header numbers/dates, images,
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

Colour comes from `[colors].color_album_footer`; padding/minimum height come
from `[native_pdf.footer]`, which must exist even with its text disabled.
Zero hint uses measured height; positive reserves at least that height upward,
without shifting number glyphs. A text footer may coexist at a non-overlapping
position; measured line-box overlap returns an actionable error instead of
drawing text over numbers. Future date/header-number commands remain separate.

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
production app/UI/CLI wiring, header numbering and dates remain pending.

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
