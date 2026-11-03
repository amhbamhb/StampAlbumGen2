# Manually checking the AlbumEasy baseline

A native page-geometry proof now exists alongside the frozen AlbumEasy baselines.
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
darkblue border (0.1/0.5/0.1 mm strokes, 1 mm gaps), horizontal/vertical spacing 3/3 mm,
and a **0.1 mm diagnostic content guide**. Those historical values are preserved,
not normalized to exact Letter (215.9 mm) or A4. They contain no text, stamp rows,
images or export. Horizontal spacing is reflected in the content guide; vertical
spacing is implemented/tested as an inter-element gap, not an outer-page inset.
This is diagnostic geometry, not an album-rendering backend. The numeric
`ALBUM_PAGES_BORDER` command now supports legacy three-argument double borders,
four-argument single/double/triple borders and omitted/disabled borders.
See the [border verification report](../testing/ALBUMEASY_BORDER_V1.md) for
AlbumEasy's sparse-slot behavior and remaining interactions.

### User configuration to command state

The [native config bridge](../../src/native_pdf/config.rs) reads the existing
page keys directly as `f64` millimetres and constructs the same validated size,
margins, spacing and border command types used by the fixture tests.
There is no intermediate `f32` conversion or decimal formatting. It resolves
and validates base and even-page geometry before the renderer receives state.

Required: width, height, all four margins and both spacing values. Existing
flat root keys are accepted; explicit `[page]` keys win over stale root values,
and the example reports overridden key names. Invalid winning values fail,
not fall back to valid root values. Border keys are optional as a **complete
four-key group**; omit all four for no border, or set all widths to zero.

Border colour is now configurable in the existing section:

```toml
[colors]
color_album_border = "darkblue"
```

Edit the existing key/table rather than adding a duplicate. Opaque SVG named
colours are case-insensitive (`black`, `green`, `navy`, `lightgrey`, `white`).
The legacy root key is also supported; `[colors]` wins and the overridden name
is reported. An omitted border colour means black. Invalid names, hex, RGB
functions and transparency fail explicitly without echoing the value.
The three border lines share that colour; the diagnostic content guide stays
black. Colour never changes border thickness, spacing or content bounds.

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
features. The example explicitly warns that they are not applied (240 with
the current template). The bridge exposes sorted `unapplied_keys` and
`overridden_keys`, never their values. Text, fonts, headers, footers, images,
runtime options and `max_page_vspace_mm` do not affect this proof yet.
Errors do not echo source lines or credential values.

`NativePdfConfig::load(&Path)` returns a per-run snapshot. The application-facing
contract is `loaded.page_settings`, passed to `write_page_geometry_proof`.
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
