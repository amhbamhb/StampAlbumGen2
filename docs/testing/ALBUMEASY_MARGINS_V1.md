# ALBUM_PAGES_MARGINS: verified contract and margins-v1 fixtures

**Date:** 2026-11-03
**Scope:** GitHub #59, Phase 1 epic #55; next bounded T003/T001 increment.
**Status:** Measured Linux references using AlbumEasy 4.5.0.
No application, renderer, parser or config code changed.

## Contract and observed behavior

```text
ALBUM_PAGES_MARGINS (left right top bottom)
ALBUM_PAGES_MARGINSE (left right top bottom)
```

Installed 4.5.0 help documents four millimetre arguments, mandatory base margins
and an optional distinct even-page command. `MARGINSE` is **not an alias**.
The [legacy inventory](../guides/ALBUMEASY_4_2_1_COMMAND_INVENTORY.md) records
both names; no older executable was run here.

Measured with explicit 210 x 297 mm pages, 3/4 mm spacing and a single 0.2 mm
border:

| Case | Page count | Margins, left/right/top/bottom (mm) |
|---|---|---|
| Symmetric | 2 | 15 / 15 / 15 / 15 on both pages |
| Asymmetric | 2 | 25 / 12 / 18 / 21 on both pages; no automatic mirroring |
| Fractional | 1 | 12.25 / 17.75 / 20.25 / 23.75 retained |
| Even override | 3 | Base 25 / 12 / 18 / 21 on pages 1 and 3; 12 / 25 / 22 / 16 on page 2 |
| Missing margins | No PDF | stderr: `PDF generation error: Page margins not set.` |
| Missing bottom | No PDF | stderr: `Error in line 3: ALBUM_PAGES_MARGINS command - Incorrect number or format of parameters.` |

Both invalid cases return **process exit 0**. Require fresh output and inspect
diagnostics; a successful process exit does not establish semantic success.

The help says the border is just inside the margins. All eight page-border
paths corroborate this: the **stroke centreline is inset by half the 0.2 mm
stroke**, so its outside edge coincides with the margin. In PDF bottom-left mm:

```text
left   = left_margin + 0.1
bottom = bottom_margin + 0.1
right  = 210 - right_margin - 0.1
top    = 297 - top_margin - 0.1
```

Measurements match each expected edge within **0.01 mm**. Do not equate the
border centreline itself to the configured margin.

Unverified: zero/negative/non-finite margins, oversize margins leaving no usable
area, extra/non-numeric arguments, repeated/in-page changes, absent base margins
with only MARGINSE, custom footer numbering versus even-page parity, implicit
overflow-created pages, other border widths/styles and borderless layout.
The even-page test uses three explicit PAGE_START commands with default numbering;
it is not full MARGINSE certification.

## Existing configuration mapping (static evidence)

| Surface | Current behavior |
|---|---|
| [Config fields](../../src/config.rs:506) | Four optional `f32` fields: `page_left_margin_mm`, `page_right_margin_mm`, `page_top_margin_mm`, `page_bottom_margin_mm` |
| [Defaults](../../src/config.rs:994) | All four 10.0 mm |
| [Canonical writer](../../src/album_utils/globals.rs:65) | Loads effective scoped configuration, then reads [margin fields](../../src/album_utils/globals.rs:75) with 10.0 mm fallbacks |
| [TXT emitter](../../src/album_utils/globals.rs:253) | Emits left/right/top/bottom in that order, each to **one decimal** |
| [Width layout](../../src/album_globals/generator.rs:1177) | Subtracts left/right margins from configured page width, plus other spacing/border terms |
| [Height layout](../../src/album_globals/globals.rs:19) | Subtracts top/bottom margins from configured page height, plus other terms |
| [Section precedence](../../src/config.rs:1875) | `[page]` overrides stale root fields |
| [Album overrides](../../src/config.rs:2620) | Album settings overlay defaulted global values; [sectioned album values](../../src/config.rs:2699) win over stale album root values |
| [Template](../../config_template.toml:15), [full example](../../config_full_example.toml:21), [inventory](../guides/CONFIG_PARAMETERS_INVENTORY.md:62) | All four defaults documented as 10.0 mm |
| Selected local config | Only these four non-sensitive numeric values inspected: all 10.0 mm. File untouched; no other local values copied |
| [Import validator](../../src/external_album/parser.rs:27) | Both commands accept exactly four numeric arguments; [numeric guards](../../src/external_album/parser.rs:3) reject quoted/non-finite values and [negative guard](../../src/external_album/parser.rs:69) rejects negative values |
| [Imported reflow](../../src/external_album/reflow.rs:11) | Selects MARGINSE for an even page index when present, otherwise base margins; checks remaining width/height |

No separate even-page margin config fields or MARGINSE emission were found in
the inspected Config/canonical writer. Existing importer/reflow support is not
proof that ordinary config generation emits the command or that all its layout
semantics are implemented.

As with page size, unrounded config values enter layout while TXT emission
rounds to one decimal. The fractional fixture preserves the direct AlbumEasy
behavior rather than hiding that discrepancy. Shared native/export precision
and even-page configuration require later approved decisions. Explicit imported
commands must not silently acquire unrelated global defaults.

## Frozen bundle and manual reproduction

[margins-v1](../../tests/fixtures/albumeasy/margins-v1) contains six independent
TXT inputs, four generated PDFs, fixture-local PDF binary attributes and
[expected.json](../../tests/fixtures/albumeasy/margins-v1/expected.json).
The manifest records all ten file hashes, page counts, margins, border
centreline coordinates and invalid-case diagnostics. Preserve these bytes;
future changes require an approved fixture revision.

Linux, from the repository root (substitute input/install path as needed):

```bash
work=$(mktemp -d /tmp/albumeasy-margins-XXXXXX)
cp tests/fixtures/albumeasy/margins-v1/asymmetric.txt "$work/album.txt"
(cd "$work" && QT_QPA_PLATFORM=offscreen timeout 60s \
  /home/ray/work/AlbumEasy/AlbumEasy.sh -c album.txt \
  >renderer.stdout.log 2>renderer.stderr.log)
cat "$work/renderer.stdout.log" "$work/renderer.stderr.log"
test -s "$work/album.pdf" &&
  mutool draw -q -F trace -o "$work/trace.xml" "$work/album.pdf"
grep -A8 '<page ' "$work/trace.xml"
```

Use a new empty directory per input. The asymmetric reference has two pages,
each with border edges `[25.1, 21.1, 197.9, 278.9]` mm. MuPDF trace coordinates
are PDF points; convert with `mm = points * 25.4 / 72`. The recorded stroke
transform is `1 0 0 -1 0 841.8898`; the raw path coordinates above express PDF
bottom-left geometry before that display flip. Do not flip them a second time.
Invalid probes must leave **no PDF** and show the exact stderr listed above.

Windows: follow the isolated-directory instructions in the
[CLI guide](../guides/ALBUMEASY_BASELINE_MANUAL_CLI.md), substituting a margins-v1
input, and invoke your installed `AlbumEasy.exe -c .\album.txt`. Inspect the PDF
and diagnostics, not only `$LASTEXITCODE`. Windows execution and physical printing
of this bundle remain unverified.

## Execution evidence and impact

Run root:
`/home/ray/.copilot/session-state/80e32404-199a-4106-9649-659ef9662bab/files/margins-v1-hG1YJK`

All six initial probes used the exact renderer command shown above, with the
input copied as `album.txt` in separate directories. Four valid cases were
rendered again in independent `repeat` subdirectories using the same command.
All ten renderer invocations exited **0**; eight valid PDFs were created.
Invalid cases had the expected stderr and no output.

All eight initial page borders were measured from `moveto`/`lineto` coordinates
with the formula above; page counts and 210 x 297 mm MediaBoxes also passed.
Each valid first/repeat inspection used:

```bash
mutool draw -q -F trace -o trace.xml album.pdf
sed '/^<document filename=/d' trace.xml > normalized.trace
sed '/^<document filename=/d' repeat/trace.xml > repeat/normalized.trace
cmp normalized.trace repeat/normalized.trace
```

All **4/4** normalized repeat comparisons exited 0. Complete traces cover more
than geometry, but text extraction/font fidelity is not separately certified
for this bundle. No `pdftotext` was used; core text checks remain on `oxidize-pdf`.

Manifest and frozen identities (exit 0; six cases, ten matching hashes):

```bash
jq -e '.cases | length == 6' tests/fixtures/albumeasy/margins-v1/expected.json
(cd tests/fixtures/albumeasy/margins-v1 &&
  jq -r '.cases[] | (.input_sha256 + "  " + .input), (select(.reference != null) | .reference_sha256 + "  " + .reference)' expected.json |
  sha256sum --check)
git diff --check
```

Final unchanged core-baseline regression (exit 0; **6 passed, 0 failed,
0 ignored**):

```bash
ALBUMEASY_BASELINE_CANDIDATE=/home/ray/.copilot/session-state/80e32404-199a-4106-9649-659ef9662bab/files/t003-core-verification-6RHMrG/first/album.pdf cargo test --test albumeasy_baseline -- --include-ignored --nocapture
```

Reference executable and installed help SHA-256 identities match the
[core reference installation](ALBUMEASY_CORE_V1_BASELINE.md#frozen-identities).

Filesystem impact: six isolated case directories, four repeat directories,
six copied inputs plus four repeat copies, eight generated PDFs, render logs
and MuPDF traces. Only four initial PDFs were copied into the new bundle.
No existing source fixture/reference, production album or config was changed;
no input moved/deleted, DB operation or country ingest occurred.

Error inventory: expected missing-margins/missing-bottom diagnostics above;
MuPDF emitted `warning: ICC support is not available`. Color-profile fidelity
is not certified. Existing application build warnings are unrelated and remain.

Logic risks: border stroke inset must be included when measuring margins;
MARGINSE is a separate setting, not an alias or automatic mirror; renderer exit
0 can mask semantic errors; fractional config precision can diverge from TXT.
None was silently fixed.

This measured specification is **not yet wired into the core-specific Cargo
checker**. A generalized command-fixture runner and native PDF implementation
remain separate increments.
