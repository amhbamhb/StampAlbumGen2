# AlbumEasy border-v1: verified command and native implementation

Date: 2026-11-03. Issue #62, parent #55; original branch `issue/62-native-page-border`.
Integration remains `development/native-pdf`. The initial commit blocker below
was resolved in the maintainer-authorized #65 checkpoint; see the final section.
Original failure records are retained as historical evidence.

## Contract and scope

`ALBUM_PAGES_BORDER (outer inner1 inner2 gap)` uses millimetres. The legacy
`(outer inner gap)` form means `(outer inner 0 gap)`. The command is optional;
omitting it or setting all stroke widths to zero reserves/draws no border.
Nonzero gap alone reserves nothing. All native numeric arguments must be finite
and nonnegative. Arity, conversion overflow, unusable content and unusable stroke
footprints fail explicitly before saving a PDF.

Reference: locally installed AlbumEasy 4.5.0 help and executable. No full manual
content was copied into the repository.

- Executable SHA256:
  `cc3badc44e4a2539424148d1038a4a569fa7d63c174d1fca7c8a58bc6dca5ca7`

The authoritative executable identity is the value recorded in the existing
[page-settings evidence](ALBUMEASY_PAGE_SIZE_V1.md); reference fixture identities
are independently guarded in [expected.json](../../tests/fixtures/albumeasy/border-v1/expected.json).

The separate [command module](../../src/native_pdf/commands/page/album_pages_border.rs)
accepts numeric slices, not raw TXT. Shared page state holds the optional border,
derives content occupancy and validates drawable strokes. The
[proof renderer](../../src/native_pdf/renderer.rs) draws those strokes plus a
diagnostic 0.1 mm content guide. No application CLI/UI/backend, importer,
configuration, schema or default changes are made.

## Measured references

All pages are A4 portrait, margins 15 mm, horizontal spacing 3 mm, vertical
spacing 4 mm. Every page uses the same HN10 `BORDER-V1` text. Bounds below are
bottom-left millimetres; only left/bottom centreline insets are tabulated because
the rectangle is symmetric. The JSON preserves all four bounds and glyph x/y.

| Case | Arguments | Stroke widths | Left centreline x | Content occupancy |
|---|---|---|---|---|
| single | 0.25 0 0 1.25 | 0.25 | 15.125 | 0.25 |
| double | 0.5 0.1 0 1.25 | 0.5, 0.1 | 15.25, 16.8 | 1.85 |
| triple | 0.1 0.5 0.2 1.25 | 0.1, 0.5, 0.2 | 15.05, 16.6, 18.2 | 3.3 |
| legacy-double | 0.5 0.1 1.25 | 0.5, 0.1 | 15.25, 16.8 | 1.85 |
| disabled | 0 0 0 1.25 | none | none | 0 |
| omitted | no command | none | none | 0 |
| inner-only | 0 0 0.3 1.25 | 0.3 | 17.65 | 1.55 |
| middle-only | 0 0.3 0 1.25 | 0.3 | 16.4 | 1.55 |
| skipped-middle | 0.5 0 0.3 1.25 | 0.5, 0.3 | 15.25, 18.15 | 2.05 |
| zero-gap | 0.1 0.5 0.2 0 | 0.1, 0.5, 0.2 | 15.05, 15.35, 15.7 | 0.8 |

Fifteen reference strokes and ten first-glyph anchors were measured. Ten
independent rerenders have identical page traces, excluding only the outer
document filename metadata. Full native/AlbumEasy page equality is not expected:
the native proof intentionally has a content guide and no text.

Content left is `15 + occupancy + 3`; top is `282 - occupancy`. The first
reference text baseline is a constant 3.52778 mm below that top for these probes.
For each enabled drawing slot, the centreline is inset by all preceding widths,
all preceding slot gaps, and half its own width.

**Compatibility risk:** drawing always retains the two named inner slots.
Content reserves `outer + (inner1 > 0 ? inner1 + gap : 0) +
(inner2 > 0 ? inner2 + gap : 0)`. Thus a disabled middle line can make the
second-inner border overlap the nominal content region. Native code preserves
this measured behavior rather than compacting slots or silently expanding
content occupancy. The two quantities are deliberately separate.

## Verification commands and outcomes

Reference run: each `name` in the table was copied as `album.txt` into its own
fresh directory under
`/home/ray/.copilot/session-state/80e32404-199a-4106-9649-659ef9662bab/files/native-border/NAME/`.
The same commands were repeated in each case's `repeat/` directory:

```bash
QT_QPA_PLATFORM=offscreen timeout 60s /home/ray/work/AlbumEasy/AlbumEasy.sh -c album.txt >stdout.log 2>stderr.log
mutool draw -F trace -o trace.xml album.pdf
```

Every valid run exited zero, emitted no AlbumEasy stderr diagnostics and produced
one PDF page. XML page subtrees were compared before freezing references;
measurements and SHA256 hashes were generated from the saved outputs. All 21
TXT/PDF identities are checked by a normal Cargo test.

`missing-gap.txt` returns exit **0**, reports the following, and creates no PDF:

```text
Warnings/Errors in file album.txt:
Error in line 5: ALBUM_PAGES_BORDER command - Incorrect number or format of parameters.
```

Native tests first failed to compile because the new border module/state API did
not yet exist (`E0432`, `E0599`, `E0061`). After implementation:

```bash
ALBUMEASY_BASELINE_CANDIDATE=/home/ray/.copilot/session-state/80e32404-199a-4106-9649-659ef9662bab/files/t003-core-verification-6RHMrG/first/album.pdf cargo test --test native_pdf_page_border --test native_pdf_page_settings --test albumeasy_baseline -- --include-ignored --nocapture
cargo test --test external_album_backend -- --nocapture
cargo clippy --test native_pdf_page_border --test native_pdf_page_settings --example native_pdf_page_settings
cargo check --lib --example native_pdf_page_settings --target x86_64-pc-windows-gnu --no-default-features
cargo run --example native_pdf_page_settings -- /home/ray/.copilot/session-state/80e32404-199a-4106-9649-659ef9662bab/files/native-border/native-border-proof.pdf
mutool draw -F trace -o /home/ray/.copilot/session-state/80e32404-199a-4106-9649-659ef9662bab/files/native-border/native-border-proof.trace.xml /home/ray/.copilot/session-state/80e32404-199a-4106-9649-659ef9662bab/files/native-border/native-border-proof.pdf
```

- Border tests: **4 passed**; existing native settings: **6 passed**; core:
  **6 passed**; no ignored tests in that explicit acceptance command.
- Default external-album regressions: **23 passed, 3 opt-in tests ignored**.
- Clippy and Windows GNU library/example cross-check: exit 0, existing
  application warnings only. Windows execution/printing/default-feature build
  are not verified.
- The new proof has three pages, each with three configured border strokes plus
  a content guide; independently read back with MuPDF.
- All ten reference PDFs are read with `oxidize-pdf`: one page, untruncated text,
  expected marker present. No `pdftotext` use.
- Actual native/reference strokes are independently checked for count, width,
  rectangle corners, bounds and page transform within **0.01 mm**. Reference
  glyph positions independently anchor native content guides.
- Negatives cover invalid arity, every invalid numeric position, conversion
  overflow, the frozen missing-gap input and a sparse border whose content fits
  but stroke footprint does not. The latter must not create output.

### Additional opt-in regression failure: commit blocker

The first wider command accidentally enabled all opt-in external-album tests:

```bash
ALBUMEASY_BASELINE_CANDIDATE=/home/ray/.copilot/session-state/80e32404-199a-4106-9649-659ef9662bab/files/t003-core-verification-6RHMrG/first/album.pdf cargo test --test albumeasy_baseline --test external_album_backend -- --include-ignored --nocapture
```

Core: 6 passed. External: 25 passed, 1 failed. Initial exact error:

```text
Set ALBUMEASY_TEST_CLI to the real AlbumEasy executable or launcher: NotPresent
```

Retry with the required environment:

```bash
QT_QPA_PLATFORM=offscreen ALBUMEASY_TEST_CLI=/home/ray/work/AlbumEasy/AlbumEasy.sh cargo test --test external_album_backend brazil_real_albumeasy_import_approval_and_page_edit -- --ignored --nocapture
```

The retry compared **85 identical full-colour raster pages at 72 DPI**, then failed
at the existing path assertion in
[external_album_backend.rs](../../tests/external_album_backend.rs):

```text
assertion `left != right` failed
  left: "build/reconstructed/Brazil.pdf"
 right: "build/reconstructed/Brazil.pdf"
```

No native renderer is routed into that external backend. This is outside the
border slice; it was not suppressed or patched. Commit/merge remains blocked
despite all relevant native/default regression checks passing.

## Filesystem and database impact

- Added the single command file, focused test file and border-v1 bundle:
  eleven TXT inputs, ten PDFs, JSON manifest and local binary PDF attributes.
- Modified native settings/resolver/renderer, the example, existing native
  settings tests, roadmap and CLI guide. Previous frozen bundles are unchanged.
- No inputs moved, deleted or relocated. Original Brazil sources were only read
  and copied by opt-in tests; the user autosave remains untouched.
- Cargo artifacts remain under its existing target directories. Reference
  render/repeat directories, logs, native proof/trace and isolated generated
  knowledge remain under session files. Rust temporary test directories are
  self-cleaning.
- No production database/config/schema changes or country ingest were run.
  Synthetic external-backend tests use temporary databases; those tests exercise
  existing schema/rollback behavior, not a new migration.
- The first wider run's Brazil DB test reported
  `/tmp/.tmp4IJBa0/albums/brazil/brazil.db`: stamps=655, placements=655,
  valid links=655, distinct linked stamps=655, orphan/unlinked=0; repeated
  placements retained, skipped=0. Its temp directory was cleaned by the test.
- Real acceptance used `/tmp/.tmpxYJchk/albums/brazil/`; import/edit and raster
  outputs were temporary and cleaned after failure. The failed test did not
  emit independent DB row-insert/update counts; none are inferred.
- No Colnect inbox processing occurred, so a country ingest table is not
  applicable.

## Error inventory and remaining logic risks

1. AlbumEasy semantic failure has exit zero: gate on diagnostics and fresh PDF,
   not exit status alone.
2. The initial raw-repeat trace comparison differed at document filename metadata,
   not page content; comparing page subtrees removes only that metadata.
3. MuPDF reports `warning: ICC support is not available`; no colour-profile
   certification is claimed.
4. Expected TDD compilation failures and both opt-in regression failures are
   recorded above. Existing application lint/dead-code warnings remain.
5. Sparse border/content overlap is an observed AlbumEasy compatibility quirk,
   not corrected here. Decorative borders, other colour commands, mid-document geometry
   transitions, header/footer interactions and footer-labelled parity remain
   unverified.
6. Existing config maps `page_border_outer/inner1/inner2/spacing` to this command.
   The template uses 0.1/0.5/0.1/1; schema defaults and full example use
   0.5/0.1/0/3. The canonical writer emits two decimals. These existing differences
   are recorded, not normalized. The native proof does not load configuration.
7. Physical print/visual acceptance and Windows runtime remain outstanding.
   Full T004/T005/T006/T007 remain open; this is not a production album renderer.

## #64 native border-colour follow-up

The next command is implemented separately in
[colour_album_border.rs](../../src/native_pdf/commands/colour/colour_album_border.rs),
with the `COLOR_ALBUM_BORDER` alias sharing the same typed operation. Reusable
[colour.rs](../../src/native_pdf/colour.rs) parses opaque SVG named colours using
`svgtypes` (existing transitive version 0.16.1, now a direct dependency).
No new package/version or handwritten named-colour table is added.

The [colour fixtures](../../tests/fixtures/albumeasy/border-colour-v1/) include
six successful cases, seven total pages and 21 border strokes:

| Case | Expected DeviceRGB | Pages | Meaning |
|---|---|---|---|
| default | 0, 0, 0 | 1 | No command means black |
| named | 0, 0, 139/255 | 1 | Mixed-case `DaRkBlUe` |
| alias | 0, 128/255, 0 | 1 | `COLOR_ALBUM_BORDER (green)` |
| light-grey | 211/255, 211/255, 211/255 | 1 | SVG `lightgrey` spelling |
| white | 1, 1, 1 | 1 | White still emits three strokes |
| global-last | 0, 0, 128/255 | 2 | Final navy command after page starts overrides earlier red globally |

These are already-tokenized command arguments, not a production TXT grammar
implementation. The fixture reader recognizing both spellings is test-only.
All six reference page traces match independent repeats, ignoring only outer
document filename metadata. Every TXT/PDF identity is SHA256 guarded (18 files).
Reference text is read using `oxidize-pdf`, not `pdftotext`.

### Compatibility discovery and explicit failures

The former template claimed hex support. Actual AlbumEasy 4.5 results:

| Probe | Exact diagnostic suffix |
|---|---|
| bare `#1E4A7A` | `Parameters require closing parenthesis.` |
| quoted `"#1E4A7A"` | `the specified colour "#1E4A7A" is not a valid colour` |
| quoted `"#F80"` | `the specified colour "#F80" is not a valid colour` |
| quoted `"#GG00FF"` | `the specified colour "#GG00FF" is not a valid colour` |
| `not-a-colour` | `the specified colour not-a-colour is not a valid colour` |
| empty argument | `expecting a colour value` |

Each final failure probe returns **0**, prints `Warnings/Errors in file album.txt:`
and an `Error in line 5: COLOUR_ALBUM_BORDER command - ...` line, and creates
no PDF. Both primary and repeat failure diagnostics agree. Complete strings
are preserved in the fixture JSON. Initial bare/quoted exploratory failures
also remain in session logs; they were not mistaken for successful output.

Native code rejects those forms, unknown names, transparency and RGB functions
with explicit redacted errors rather than silently defaulting to black.
Only named colours are included in this compatibility slice. Templates now
document that limitation. No unrelated external TXT emitter is modified;
broader hex export policy remains an explicit future decision.

### Configuration and renderer

`[colors].color_album_border` and the existing legacy root key now populate
the native per-run snapshot. `[colors]` wins; overridden names are reported,
and invalid winning values fail. Omission defaults to black. The existing main
`Config` field/emitter is unchanged. The local Git-ignored `config-pdf.toml`
retains its user's `darkblue` value; only inaccurate comments were corrected.

The renderer applies colour to each border stroke and resets the diagnostic
content guide to black, even with no border strokes. All previous geometry,
stroke thickness and spacing tests still pass. Other colour settings remain
unapplied; the current template warning decreases from 241 to 240 unsupported
settings. There is no native text/fonts/headers/footer/images support yet.

Parsed 8-bit RGB values match AlbumEasy's serialized reference components within
0.00001. Oxidize-pdf serializes RGB operators to **three decimal places**, so the
actual rendered acceptance bound is **0.00051 per component**, less than one
8-bit channel step (1/255). For example green is `.502` natively versus `.50196`
in the reference. This quantization is measured, not hidden. Geometry tolerance
remains **0.01 mm** for rectangle corners and stroke widths. Both traces use
DeviceRGB; no ICC/display/physical colour equivalence is certified.

### Exact verification commands and outcome

For each fixture name, `album.txt` was copied into fresh session directories
`files/native-colour/NAME/final/` and `repeat/`. Both runs execute:

```bash
QT_QPA_PLATFORM=offscreen timeout 60s /home/ray/work/AlbumEasy/AlbumEasy.sh -c album.txt >stdout.log 2>stderr.log
mutool draw -q -F trace -o trace.xml album.pdf
```

The MuPDF command runs only when a fresh PDF exists. Successful cases have
empty AlbumEasy stderr. Page subtrees are compared independently before PDFs
and checksummed measurements are frozen.

```bash
ALBUMEASY_BASELINE_CANDIDATE=/home/ray/.copilot/session-state/80e32404-199a-4106-9649-659ef9662bab/files/t003-core-verification-6RHMrG/first/album.pdf cargo test --test native_pdf_border_colour --test native_pdf_config --test native_pdf_page_settings --test native_pdf_page_border --test albumeasy_baseline -- --include-ignored --nocapture
cargo test --lib config::tests -- --nocapture
cargo test --test external_album_backend -- --nocapture
cargo clippy --test native_pdf_border_colour --test native_pdf_config --test native_pdf_page_settings --test native_pdf_page_border --example native_pdf_page_settings
cargo check --lib --example native_pdf_page_settings --target x86_64-pc-windows-gnu --no-default-features
cargo run --example native_pdf_page_settings -- /home/ray/.copilot/session-state/80e32404-199a-4106-9649-659ef9662bab/files/native-colour/darkblue-proof.pdf
cargo run --example native_pdf_page_settings -- /home/ray/.copilot/session-state/80e32404-199a-4106-9649-659ef9662bab/files/native-colour/green-proof.pdf /home/ray/.copilot/session-state/80e32404-199a-4106-9649-659ef9662bab/files/native-colour/green.toml
cargo run --example native_pdf_page_settings -- /home/ray/.copilot/session-state/80e32404-199a-4106-9649-659ef9662bab/files/native-colour/invalid-output.pdf /home/ray/.copilot/session-state/80e32404-199a-4106-9649-659ef9662bab/files/native-colour/invalid.toml
```

Results: **60 passed**, no failed tests; three unrelated opt-in tests remain
ignored in the default external suite. Breakdown: 4 new colour, 7 config,
10 existing native geometry, 6 core, 10 main-config and 23 external-backend.
Clippy and Windows GNU cross-check exit 0 with existing application warnings.
The intentionally invalid manual config returns exit 1 and creates no PDF.
Its exact error suffix is:

```text
COLOUR_ALBUM_BORDER: Expected an opaque SVG named colour; hex, RGB functions and transparency are not supported
```

Both manual proofs were independently traced with
`mutool draw -q -F trace -o <proof.trace.xml> <proof.pdf>`. Each contains three
pages, nine configured border strokes and three black guides. Darkblue's RGB
is 0/0/.545 and green's is 0/.502/0, within the measured component tolerance.
The command tests also verify two pages without borders retain black guides.

Initial TDD compile failure identified the missing colour module/state API.
During implementation, a config variable scope error (`E0425`) and test
page-count type mismatch (`E0308`/`E0277`) were corrected before all final tests.
No failures were suppressed. MuPDF still warns `ICC support is not available`.
Windows runtime/default-feature build and physical printing remain unverified.

### Impact, risks and checkpoint

- Added one command module/family and shared colour representation, focused tests
  and 12 TXT/6 PDF fixtures with manifest/binary attributes.
- Updated native document/config/renderer, the example, both templates and
  relevant manual/inventory/roadmap documentation. Existing config key is reused;
  no new production configuration field or schema is introduced.
- Cargo.lock adds only the root crate's direct `svgtypes` edge; the already
  present package/version/dependencies are unchanged.
- Session config/proof/trace/log/knowledge files are new under
  `files/native-colour/`. Temporary test directories are self-cleaning.
- No inputs moved/deleted/relocated; previous frozen bundles, main config,
  production albums/databases, credentials and user autosave are untouched.
- No ingest/country processing or production DB migration occurred. Existing
  external regression tests use temporary synthetic DBs; no new DB row counts
  were independently emitted or inferred. A Colnect country table is inapplicable.
- Named-colour model uses the SVG palette, but only the explicit reference
  samples above have per-colour frozen conformance evidence. Other colour
  commands and decorative borders remain unimplemented.
- No production app flow changes: baseline draw.io remains untouched.
- Branch `issue/64-native-border-colour` retains uncommitted #62/#63 work.
  The earlier unrelated opt-in Brazil path assertion still blocks committing or
  merging; it is not rerun or patched here. No push. Full roadmap tasks stay open.

## #65 commit-gate resolution

The maintainer approved fixing the regression and then creating two commits.
The renderer intentionally overwrites the same reconstructed PDF path after an
edit; the opt-in test incorrectly required a different path. It now captures
the original generated bytes before editing and asserts changed PDF contents.
The full raster checks remain: 85 identical initial pages, edited page 1 changed,
all other 84 unchanged. No production renderer behavior was modified.

Exact command:

```bash
QT_QPA_PLATFORM=offscreen ALBUMEASY_TEST_CLI=/home/ray/work/AlbumEasy/AlbumEasy.sh cargo test --test external_album_backend -- --include-ignored --nocapture
```

Result: **26 passed, 0 failed, 0 ignored**. DB evidence:

| Temporary target | Stamps | Placements | Valid links | Distinct linked stamps | Orphan/unlinked |
|---|---:|---:|---:|---:|---:|
| `/tmp/.tmpHqruuf/albums/brazil/brazil.db` | 655 | 655 | 655 | 655 | 0/0 |
| `/tmp/.tmphKastT/albums/brazil/brazil.db` | 655 | 655 | 655 | 655 | 0/0 |

Duplicate placements were retained, skipped=0. Existing schema/import/edit
assertions and original source checksum checks passed; no production DB or
input changed. Test copies, temporary databases and raster files were created
inside self-cleaning tempdirs; inputs were not moved/deleted/relocated. No
country ingest was run. There were no test errors in this rerun.

Regression commit: `333c398b`. The pre-commit hook also staged the pending
Cargo.toml `svgtypes` direct declaration; remaining native implementation and
lockfile changes form the second checkpoint commit. Branch:
`fix/native-pdf-commit-checkpoint`; local-only, no push. The previous opt-in
failure is resolved, not waived or suppressed. Full native roadmap tasks and
Windows runtime/printing gates remain open.
