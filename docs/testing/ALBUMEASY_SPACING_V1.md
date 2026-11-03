# ALBUM_PAGES_SPACING: verified contract and spacing-v1 fixtures

**Date:** 2026-11-03
**Scope:** GitHub #60, Phase 1 epic #55; bounded command-first T003/T001 evidence.
**Status:** Measured Linux references using AlbumEasy 4.5.0.
No application, renderer, parser or config code changed.

## Contract

```text
ALBUM_PAGES_SPACING (horizontal vertical)
```

Installed 4.5.0 help documents this mandatory command with two millimetre
arguments. Horizontal spacing separates text/stamps from left/right borders;
vertical spacing separates stamp rows or text blocks. These are **within-page
content gaps**, not gaps between separate pages or sheets.

The syntax is also recorded in the
[4.2.1 inventory](../guides/ALBUMEASY_4_2_1_COMMAND_INVENTORY.md); no older
executable was run. No alias/default is documented for this command.

## Controlled probes and results

All four valid inputs have identical content: two HN 10 PAGE_TEXT blocks with
no per-command vspace, then two top-aligned justified rows. Each row has an
empty-caption 25 x 20 mm frame and a 30 x 25 mm frame. A4 page size, 15 mm
margins, single 0.2 mm border and zero box adjustment are fixed.

| Case | Horizontal / vertical (mm) | Measured result versus zero |
|---|---|---|
| Zero | 0 / 0 | Valid control; one page |
| Horizontal only | 6.25 / 0 | Both text anchors and left frames move right 6.25 mm; right frames move left 6.25 mm; vertical coordinates unchanged |
| Vertical only | 0 / 8.75 | First text anchor unchanged; second moves down 8.75 mm; row 1 down 17.5 mm; row 2 down 26.25 mm; horizontal coordinates unchanged |
| Combined | 6.25 / 8.75 | Same independent horizontal and vertical effects combined |
| Missing command | Absent | No PDF; stderr `PDF generation error: Page spacing not set.` |
| Missing vertical | One argument | No PDF; stderr `Error in line 4: ALBUM_PAGES_SPACING command - Incorrect number or format of parameters.` |

Both invalid cases return **exit 0**. Inspect diagnostics and fresh output;
do not accept exit status alone.

All **16 frame tuples**, **eight text baseline anchors**, four page counts and
four A4 MediaBoxes meet a **0.01 mm** tolerance. Frame sizes do not change.
Each valid case was rendered twice; all four complete traces match byte-for-byte
when both inspections use the same `album.pdf` filename.

### Measurement interpretation

Coordinates in [expected.json](../../tests/fixtures/albumeasy/spacing-v1/expected.json)
are PDF bottom-left millimetres. Text anchors are the first glyph baseline,
not glyph bounding boxes. Frame tuples are `[left, bottom, width, height]` from
stroke centrelines, excluding extended corner endpoints.

In this fixture the left content anchor is `15 + 0.2 + horizontal` mm; the
rightmost frame centreline is `210 - 15 - 0.2 - horizontal` mm. Do not confuse
these content coordinates with the page-border centreline, which is inset only
0.1 mm from the margin (see [margins evidence](ALBUMEASY_MARGINS_V1.md)).

For these two 10-point text blocks, baseline separation is
`10 * 25.4 / 72 + vertical` mm. The first baseline does not gain a leading
vertical gap. Two text-block gaps precede the first row; a third gap precedes
row 2. The tallest first-row frame bottom and second-row top have precisely
the configured vertical gap. These are **observations for this sequence**,
not a universal font/line-height/layout formula.

Unverified: negative/non-finite values, maximum usable gaps/overflow, extra or
non-numeric arguments, repeated/in-page changes, interactions with explicit
vspace, multiline text, columns, images, captioned rows, non-JS row modes and
automatic page breaks. Zero and fractional spacing are verified only for these
inputs. This is not complete command or native-renderer certification.

## Configuration mapping and discrepancies

Static trace only; no config-generated album was executed. Only the two
non-sensitive local numeric settings were inspected; local config was untouched.

| Surface | Current behavior |
|---|---|
| [Config fields](../../src/config.rs:514) | Optional `f32` `album_pages_spacing_h` / `album_pages_spacing_v` |
| [Schema defaults](../../src/config.rs:1002) | Horizontal 3.0; vertical 0.0 |
| [Effective loader](../../src/config.rs:2588) | Global/scoped album config; [section promotion](../../src/config.rs:1875) and [album override](../../src/config.rs:2620) rules unchanged |
| [Canonical writer reads](../../src/album_utils/globals.rs:83) | Horizontal/vertical each fall back to 0.0 if absent on the effective Config |
| [Emitter](../../src/album_utils/globals.rs:331) | Horizontal then vertical, each formatted to one decimal |
| [Template](../../config_template.toml:35) | 3.0 / 3.0; comments misleadingly describe gaps between album pages |
| [Full example](../../config_full_example.toml:29) | 0.0 / 0.0, unlike schema/template |
| [Inventory](../guides/CONFIG_PARAMETERS_INVENTORY.md:66) | Horizontal default 0.0, vertical unspecified; descriptions also say between pages |
| Selected local config | 3.0 / 3.0, not modified |
| [Ordinary usable-width math](../../src/album_globals/generator.rs:1183) | Subtracts horizontal spacing once in the inspected formula, alongside total-border terms |
| [Ordinary usable-height math](../../src/album_globals/globals.rs:35) | Subtracts vertical spacing in its reserved-height calculation |
| [Imported reflow](../../src/external_album/reflow.rs:20) | Width subtracts twice horizontal spacing; vertical value feeds its layout |
| [Import validator](../../src/external_album/parser.rs:29) | Exactly two numeric arguments; rejects non-finite/quoted numbers and negative values |

Recorded risks, not fixes:

1. Misleading descriptions can encourage incorrect user configuration.
2. Defaults differ across schema/template/full example/inventory and writer
   fallbacks. Do not silently choose a new default during native migration.
3. Fractional config values enter layout before one-decimal TXT formatting.
4. Ordinary and imported width formulas account for spacing differently; the
   measured bilateral inset is evidence to use in a later detailed layout audit,
   not proof that subtracting once is an isolated root-cause bug (other terms
   and consumers must be reconciled).

Explicit imported spacing must remain document state, not silently replaced by
unrelated global defaults. Future shared layout/export policy remains gated.

## Frozen bundle and manual check

[spacing-v1](../../tests/fixtures/albumeasy/spacing-v1) contains six independent
TXT inputs, four generated PDFs, local PDF binary attributes and a six-case
manifest with all ten hashes and measured anchors/frame tuples.

Linux, from the repository root:

```bash
work=$(mktemp -d /tmp/albumeasy-spacing-XXXXXX)
cp tests/fixtures/albumeasy/spacing-v1/combined.txt "$work/album.txt"
(cd "$work" && QT_QPA_PLATFORM=offscreen timeout 60s \
  /home/ray/work/AlbumEasy/AlbumEasy.sh -c album.txt \
  >renderer.stdout.log 2>renderer.stderr.log)
cat "$work/renderer.stdout.log" "$work/renderer.stderr.log"
test -s "$work/album.pdf" &&
  (cd "$work" && mutool draw -q -F trace -o trace.xml album.pdf)
```

Use a new empty directory per input. Convert MuPDF point coordinates to mm
with `points * 25.4 / 72`. Do not flip the raw path coordinates again when using
the recorded PDF bottom-left expectations. The combined first text anchor is
`[21.45, 278.27224]` mm; its first frame is
`[21.45, 237.24444, 25, 20]` mm.

Windows: follow the [manual CLI guide](../guides/ALBUMEASY_BASELINE_MANUAL_CLI.md)
with a spacing-v1 input and your installed `AlbumEasy.exe -c .\album.txt`.
Inspect fresh PDFs and diagnostics even if `$LASTEXITCODE` is zero. Windows
execution and physical printing of this bundle remain unverified.

The core Cargo runner is core-specific: this measured bundle is **not wired
into it**. Generalized command-fixture automation is a separate increment.

## Execution evidence and filesystem impact

Run root:
`/home/ray/.copilot/session-state/80e32404-199a-4106-9649-659ef9662bab/files/spacing-v1-PDqUpJ`

Each initial case used exactly:

```bash
QT_QPA_PLATFORM=offscreen timeout 60s /home/ray/work/AlbumEasy/AlbumEasy.sh -c album.txt >renderer.stdout.log 2>renderer.stderr.log
mutool draw -q -F trace -o trace.xml album.pdf
```

Inspection ran only when a PDF existed. Valid cases repeated both commands in
independent `repeat` directories; all **4/4** comparisons passed:

```bash
cmp trace.xml repeat/trace.xml
```

Start/end: all ten renderer invocations exited 0; four initial and four repeat
PDFs were created. Four valid cases pass measured geometry/delta assertions,
including unchanged frame dimensions; two invalid cases have expected stderr
and no PDF. Measurements are retained as per-case `measurements.log` in the
session run directory.

Manifest/identity validation (exit 0; six cases, ten matching hashes):

```bash
jq -e '.cases | length == 6' tests/fixtures/albumeasy/spacing-v1/expected.json
(cd tests/fixtures/albumeasy/spacing-v1 &&
  jq -r '.cases[] | (.input_sha256 + "  " + .input), (select(.reference != null) | .reference_sha256 + "  " + .reference)' expected.json |
  sha256sum --check)
git diff --check
```

Final unchanged core-baseline acceptance (exit 0; **6 passed, 0 failed,
0 ignored**):

```bash
ALBUMEASY_BASELINE_CANDIDATE=/home/ray/.copilot/session-state/80e32404-199a-4106-9649-659ef9662bab/files/t003-core-verification-6RHMrG/first/album.pdf cargo test --test albumeasy_baseline -- --include-ignored --nocapture
```

Filesystem impact: six case directories and four repeat directories, ten input
copies, eight PDFs, renderer logs, trace XML and four measurement logs. Only
four initial PDFs were copied into the new fixture bundle. No existing input
was moved/deleted; prior fixtures, local config, production albums and databases
were untouched. No country-ingest/schema operation occurred.

Error inventory: expected missing-command/argument diagnostics above; MuPDF
`warning: ICC support is not available`. Color-profile fidelity is not claimed.
No `pdftotext` was used; existing core text extraction remains `oxidize-pdf`.
