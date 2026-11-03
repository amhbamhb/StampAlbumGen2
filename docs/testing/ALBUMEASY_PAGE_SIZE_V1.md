# ALBUM_PAGES_SIZE: verified contract and page-size-v1 fixtures

**Date:** 2026-11-03
**Scope:** GitHub #58, Phase 1 epic #55; bounded T003/T001 evidence only.
**Status:** Reference fixtures measured on Linux with AlbumEasy 4.5.0.
No application, renderer, parser or configuration code was changed.

## Command contract

```text
ALBUM_PAGES_SIZE (width height)
```

Installed 4.5.0 help identifies this as mandatory, with two dimensions in
millimetres. Width precedes height. No default, alias or optional argument is
documented for this command. A4 is 210 x 297 mm; exact US Letter is
215.9 x 279.4 mm. This syntax also appears in the retained
[4.2.1 inventory](../guides/ALBUMEASY_4_2_1_COMMAND_INVENTORY.md).
The older executable was not run; this is not 4.2.1 runtime certification.

Observed:

- Swapping A4 dimensions produces landscape MediaBox dimensions, without an
  additional orientation setting.
- Fractional dimensions 210.25 x 297.75 mm survive direct AlbumEasy input.
- Missing command and missing height produce explicit stderr diagnostics and
  **no PDF**, despite AlbumEasy returning **exit code 0**.
- Each valid probe produces exactly one page. Four repeated renders have
  identical MuPDF traces after excluding only the document filename line.

Not established: zero/negative/non-finite dimensions, minimum/maximum limits,
quoted numbers, extra arguments, changing size after PAGE_START, repeated size
commands, mixed-size albums, rotation metadata, crop marks, printer paper
selection and precedence over GUI paper settings. Do not invent these semantics
or label this command fully certified.

## Current config-to-output mapping

This is a static trace, not a claim that config-generated albums were run.
Only the two non-sensitive numeric local settings were inspected; local config
was not changed or copied.

| Surface | Evidence / current behavior |
|---|---|
| Fields | [Config](../../src/config.rs:499): optional `f32` width/height in mm |
| Defaults | [with_defaults](../../src/config.rs:989): 216.0 x 279.4 |
| Global section precedence | [section promotion](../../src/config.rs:1875): `[page]` values override stale root keys |
| Album scope | [load_effective_config](../../src/config.rs:2588) selects scoped album overrides when present |
| Album override | [merge contract](../../src/config.rs:2620), [section promotion](../../src/config.rs:2699): album values overlay defaulted global config; omitted keys retain global values |
| Writer | [canonical globals](../../src/album_utils/globals.rs:65) loads effective config; [dimensions](../../src/album_utils/globals.rs:73) fall back to 216.0 x 279.4 |
| TXT formatting | [emitter](../../src/album_utils/globals.rs:249) formats each dimension to **one decimal place** |
| Width layout | [generator](../../src/album_globals/generator.rs:1174) uses configured width in usable-width calculations before that formatting |
| Height layout | [compute_usable_height](../../src/album_globals/globals.rs:19) uses configured height before margins/borders/padding |
| Templates | [template](../../config_template.toml:9), [full example](../../config_full_example.toml:19): 216.0 x 279.4 |
| Config inventory | [page parameters](../guides/CONFIG_PARAMETERS_INVENTORY.md:60): same defaults |
| Selected local values | Width 216.0; height 279.4; no other local values included |
| Existing import validator | [argument count](../../src/external_album/parser.rs:26), [positive numeric validation](../../src/external_album/parser.rs:59): exactly two positive numeric values; [required-command check](../../src/external_album/parser.rs:188) requires page size |

Two important distinctions for future implementation:

1. The application's historical 216.0 mm width is **0.1 mm wider** than exact US
   Letter. Preserve that existing default unless a separately approved migration
   changes it; do not silently relabel or normalize explicit imported dimensions.
2. Direct AlbumEasy supports the tested two-decimal dimensions, while the current
   TXT emitter rounds to one decimal. Layout math uses the unrounded configured
   values. This creates a potential layout/export mismatch for custom fractional
   sizes. No rounding policy was changed here. Decide a shared resolved-size
   precision policy before native rendering; do not hide the mismatch by changing
   these fixtures.

The fixture's explicit command is authoritative for this comparison. Global
config is not an implicit override of imported document dimensions.

## Frozen fixture bundle

[page-size-v1](../../tests/fixtures/albumeasy/page-size-v1) contains six independent
TXT inputs, four generated reference PDFs and
[expected.json](../../tests/fixtures/albumeasy/page-size-v1/expected.json).
Expectations include source/reference SHA-256 identities, measured PDF point
coordinates, geometry tolerance and invalid-input diagnostics.

| Input | Requested mm | Measured mm (rounded to 5 decimals) | Outcome |
|---|---|---|---|
| `a4-portrait.txt` | 210 x 297 | 210.00000 x 297.00001 | One page, exit 0 |
| `a4-landscape.txt` | 297 x 210 | 297.00001 x 210.00000 | One page, exit 0 |
| `us-letter.txt` | 215.9 x 279.4 | 215.90000 x 279.40000 | One page, exit 0 |
| `fractional.txt` | 210.25 x 297.75 | 210.25001 x 297.75001 | One page, exit 0 |
| `missing-size.txt` | Absent | No PDF | Expected semantic failure, process exit 0 |
| `missing-height.txt` | Width only | No PDF | Expected semantic failure, process exit 0 |

Geometry tolerance is **0.01 mm**. Convert MediaBox points using `mm = pt * 25.4 / 72`.
These fixtures establish page-size behavior, not text/font/frame or configuration
precedence certification. Text extraction in the existing core checker remains
`oxidize-pdf`; no `pdftotext` was used for this increment.

Do not modify frozen input or reference bytes to accommodate later drift. Create
a separately approved fixture revision if expectations must change.

## Reproduce one probe manually

From the repository root on Linux:

```bash
work=$(mktemp -d /tmp/albumeasy-page-size-XXXXXX)
cp tests/fixtures/albumeasy/page-size-v1/a4-portrait.txt "$work/album.txt"
(cd "$work" && QT_QPA_PLATFORM=offscreen timeout 60s \
  /home/ray/work/AlbumEasy/AlbumEasy.sh -c album.txt \
  >renderer.stdout.log 2>renderer.stderr.log)
cat "$work/renderer.stdout.log" "$work/renderer.stderr.log"
test -s "$work/album.pdf" &&
  mutool draw -q -F trace -o "$work/trace.xml" "$work/album.pdf"
grep '<page ' "$work/trace.xml"
```

Expected A4 portrait MediaBox: `0 0 595.2756 841.8898`, one page.
For another case, substitute only the input filename. Use a **new empty working
directory per case**. Invalid cases must leave no `album.pdf`, with these exact
diagnostic lines on stderr:

```text
PDF generation error: Page size not defined.
Error in line 2: ALBUM_PAGES_SIZE command - Incorrect number or format of parameters.
```

The first applies to missing-size; the second to missing-height. Both process
exit codes were zero. Never accept a stale PDF or process exit alone as success.

On Windows PowerShell, create a unique directory as in the
[manual CLI guide](../guides/ALBUMEASY_BASELINE_MANUAL_CLI.md), copy the selected
input as `album.txt`, and run your installed AlbumEasy:

```powershell
& 'C:\path\to\AlbumEasy\AlbumEasy.exe' -c .\album.txt
```

Run this from the isolated directory; inspect actual diagnostics and PDF
existence, then inspect MediaBox with `mutool` as above. Windows execution and
physical printing of this new bundle remain unverified.

## Execution report and logic risks

Run root:
`/home/ray/.copilot/session-state/80e32404-199a-4106-9649-659ef9662bab/files/page-size-v1-j527uR`

Each initial case used exactly:

```bash
QT_QPA_PLATFORM=offscreen timeout 60s /home/ray/work/AlbumEasy/AlbumEasy.sh -c album.txt >renderer.stdout.log 2>renderer.stderr.log
```

Each successful case was inspected using:

```bash
mutool draw -q -F trace -o trace.xml album.pdf
```

Valid cases were independently rendered again in their own `repeat` directory.
First/repeat trace comparisons used:

```bash
sed '/^<document filename=/d' trace.xml > normalized.trace
sed '/^<document filename=/d' repeat/trace.xml > repeat/normalized.trace
cmp normalized.trace repeat/normalized.trace
```

Before/after impact: created six isolated case directories, copied six inputs,
created four initial PDFs and four repeat PDFs plus inspection/render logs and
traces. Copied only the four successful initial PDFs into the new fixture bundle.
No input was moved/deleted; core-v1, production albums, databases and local
config were untouched. No country ingest/DB schema operation took place.

Results: 4/4 valid cases meet single-page/MediaBox expectations; 4/4 normalized
repeat traces match; 2/2 invalid cases have expected stderr and no PDF. All
AlbumEasy invocations exited zero, including intentional semantic failures.

Manifest/identity validation (exit 0; six cases, all ten hashes matched):

```bash
jq -e '.cases | length == 6' tests/fixtures/albumeasy/page-size-v1/expected.json
(cd tests/fixtures/albumeasy/page-size-v1 &&
  jq -r '.cases[] | (.input_sha256 + "  " + .input), (select(.reference != null) | .reference_sha256 + "  " + .reference)' expected.json |
  sha256sum --check)
git diff --check
```

Final unchanged core-baseline regression (exit 0; **6 passed, 0 failed,
0 ignored**):

```bash
ALBUMEASY_BASELINE_CANDIDATE=/home/ray/.copilot/session-state/80e32404-199a-4106-9649-659ef9662bab/files/t003-core-verification-6RHMrG/first/album.pdf cargo test --test albumeasy_baseline -- --include-ignored --nocapture
```

Error inventory:

- Initial raw trace comparison failed at byte 43, line 2 because the first
  inspection used an absolute PDF filename and the repeat used a relative name.
  Corrected comparison to exclude only the filename line; all four then passed.
- An initial diagnostic assertion searched stdout; inspection established that
  semantic errors are on stderr. Corrected assertions passed for both cases.
- MuPDF emitted `warning: ICC support is not available`; no color-profile
  fidelity is claimed.
- Staged whitespace validation detected that Git treated these small PDFs as
  text and would normalize line endings in two of them. Added fixture-local
  `.gitattributes` declaring PDFs binary, then re-staged the original generated
  bytes and checked their indexed SHA-256 identities. No commit was made with
  normalized PDF bytes.

Logic risks: the external renderer's zero exit on errors, unrounded layout vs
one-decimal TXT dimensions, and historical rounded Letter width are recorded for
later decisions, not fixed here. Do not extend these findings to untested
command/state interactions. The existing core-v1 Cargo runner is core-specific;
this new bundle is a measured conformance specification, **not automatically
executed by that runner**. A generalized command-fixture runner is a separate
implementation increment.

## Compiled native follow-up (#61)

[Native page-settings tests](../../tests/native_pdf_page_settings.rs) now use
this bundle to render all four valid page sizes through `oxidize-pdf`, read back
their MediaBoxes, and compare them with the real AlbumEasy references. Both
missing-input fixtures exercise native settings rejection. This is page geometry,
not native interpretation/rendering of the fixture's text.
