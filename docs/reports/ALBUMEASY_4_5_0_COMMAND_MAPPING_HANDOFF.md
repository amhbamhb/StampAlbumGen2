# AlbumEasy 4.5.0 command-mapping handoff

**Prepared:** 2026-11-06
**Latest restart checkpoint:** 2026-11-09, after #105 and the page-frame
alignment backlog request. Read section 0 first; later historical snapshots
must not override it.

## 0. Current restart checkpoint -- read first

**Post-checkpoint follow-up (#106):** maintainer subsequently requested the four
missing COLOUR aliases. Working branch is now
`issue/106-colour-command-aliases`; #105 and the shutdown documentation were
already committed by maintainer actions. TXT/raw support now maps
`COLOUR_ALBUM_HEADER`, `COLOUR_ALBUM_TITLE`, `COLOUR_STAMP_BORDER` and
`COLOUR_STAMP_TEXT` to their existing COLOR handlers. Both canonical and alias
checklist rows are updated; native-language grammar and production routing
are unchanged. Regression evidence is recorded in the live checklist.
The branch/HEAD snapshot below remains historical; inspect git before resuming.

### Start here, without repeating completed investigation

1. Read the [live command checklist](../guides/NATIVE_PDF_COMMAND_CHECKLIST.md)
   for bounded command status and the working workflow.
2. Read the [implementation plan](NATIVE_PDF_IMPLEMENTATION_PLAN.md)
   (revision 67 at this checkpoint) for roadmap scope and country TODOs.
3. Read the [PAGE_START_VAR evidence](../../tests/fixtures/albumeasy/page-start-var-v1/README.md)
   for accepted references, exact commands, hashes, implementation semantics,
   failures/corrections, comparison measurements and remaining gaps.
4. Inspect the worktree and latest commits before changing anything. Do not
   recreate issues, fixtures or proofs already present.

**Verified repository snapshot at shutdown:**

- Active checkout: `/media/ray/2026-01-11/Rust/Rust/StampAlbumGen2`.
  Conversation/IDE links sometimes use `/media/ray/program/Rust/StampAlbumGen2`;
  use the actual active checkout, not a second assumed copy.
- Branch: `issue/105-case-insensitive-pdf-commands`, tracking its origin branch.
- HEAD: `6bdf57f0` (#105 command-case handling).
- Previous: `363f2158` (#104 PAGE_START_VAR), `6161402c` (#103 PAGE_START),
  `0d47cf73` (command checklist).
- These commits and tracked proof files were verified on disk. They appeared
  after maintainer actions; this assistant did not commit or push them.
- Before this handoff edit, the only modified file was the command checklist:
  the new page-frame alignment backlog row. This handoff and its checklist
  link are also local documentation changes. **No commit/push requested.**
- No test/server process was intentionally left running.

### Completed bounded work -- do not reinvent

- **#102:** maintainable command checklist: 92 canonical 4.2.1 names plus
  29 additions for 4.5.0; aliases counted separately. The actual agreed legacy
  version is 4.2.1, not the conversational "4.3".
- **#103:** ordinary PAGE_START, bounded pre-page group declaration and
  quote-preserving $$1--$$9 substitution; native explicit page breaks,
  leading/repeated/trailing intentional empty pages, numbering and continuation.
  Corrected missing TXT title-following vertical gap. See
  [page-start fixture](../../tests/fixtures/albumeasy/page-start-v1/README.md).
- **#104:** PAGE_START_VAR consumes H/V before group arguments; supports zero,
  first/only TXT VAR page, ordinary-start reset and previous-row flush with
  previous page spacing. Global V positions the title; local V controls its
  following content reservation and rows. Native
  `NATIVE_PAGE_BREAK (horizontal vertical)` overrides the next page and its
  automatic continuations; `()` resets defaults. Both paginators and numbering
  re-resolution preserve local bounds. Native first-page override was
  explicitly deferred, not forgotten.
- **#105:** shared parser normalizes ASCII command keywords; TXT admission/
  group delimiters, native version header and standalone CLI dispatch accept
  lowercase/mixed case. Arguments, paths and visible text are not uppercased.
  Existing per-match uppercase conversions are harmless but do not replace
  normalization at admission/parsing. Unknown commands still fail.

### Current proofs and validation

The tracked [page-start-var-v1 fixture](../../tests/fixtures/albumeasy/page-start-var-v1/)
contains immutable accepted AlbumEasy TXT/PDF, a reference-only horizontal
PAGE_TEXT probe, byte-identical raw `.stampalbum`, genuine `STAMPALBUM 1`
input, all three native PDFs and previews. Each comparison is four US Letter
pages with 16 real-image placements. Shared image provenance is in the existing
[pagination manifest](../../tests/fixtures/albumeasy/stamp-pagination-v1/source-records.json);
do not reopen sizing/DB research without changed inputs.

- #104 final regression run: 69 passed, 9 ignored; native library: 17 passed.
- #105 targeted run: 41 passed, 6 ignored; final case-only rerun: 2 passed.
  Decoded PDF content streams match uppercase baselines exactly.
- Six actual CLI tests covered lowercase/mixed-case TXT, raw `.stampalbum`
  and genuine native `.stampalbum`; each exited 0 and produced four pages.
- Clippy/build, formatting and diff checks passed; existing unrelated lint
  warnings remain. These counts are overlapping runs, not additive coverage.
- Exact reproducible Cargo/CLI commands and impact reports are in the
  [checklist case section](../guides/NATIVE_PDF_COMMAND_CHECKLIST.md#command-keyword-case-105)
  and the fixture README. Tests use Cargo, not a separate test framework.
- Linux verified; Windows execution and native actual-size print/screen
  acceptance remain outstanding. Use US Letter at 100%, Fit/Shrink disabled.

**Do not claim absolute visual parity:** local V changes/reset match within
0.02 mm, but TXT/raw year top differs up to 0.059302 mm and inter-row caption
separation up to 1.124533 mm. VAR-01 records a separate font/row-metric follow-up;
VAR-02 keeps native free page text open; VAR-03 defers native first-page override;
VAR-04 records human/platform acceptance. Attached native headings are not a
replacement for faithfully mapped free-standing page text.

### Last requests and next action

- **New backlog only:** native page-frame item alignment **left / centre /
  right**, distinct from row distribution. Added under checklist section 5.
  No syntax, scope or implementation approved; do not implement on restart.
- **Future output rule only:** app-generated album files should write
  **command keywords in uppercase**, preserving values, paths, identifiers
  and visible album text. User explicitly deferred output-generation changes.
- **Shutdown requested:** preserve state and stop. No next implementation
  item was selected. On restart, report this checkpoint, inspect current
  changes, and let the maintainer select proof review, an open mapping, or
  another bounded follow-up; do not assume authorization for integration.

### Non-negotiable continuation boundaries

- Production native PDF integration stays deferred until command mapping and
  review are complete. Standalone example is the testing entry point.
- AlbumEasy reference first, maintainer acceptance, then native mapping and
  all applicable three-input comparisons. Accepted reference bytes stay frozen.
- Native-only commands belong in native documents, never reference TXT.
- Keep tracked command proofs under `tests/fixtures/albumeasy/`; country
  experiments Australia/Brazil/Christmas Island and their process artifacts
  remain intentionally local/ignored. Do not force-add or recreate absent
  local country data; their findings remain in the implementation plan.
- Do not rerun `okf-rs generate` without permission; it was previously declined.
- Preserve maintainer edits, use portable paths, and do not commit/push without
  a current request. Follow issue/branch and test gates for new implementation.

**Optional same-machine evidence:** session logs live under
`/home/ray/.copilot/session-state/a27359db-6a9e-4bfe-8b1f-d01a379ca719/files/`.
Useful names: `page-start-var-resumed-regressions.log`,
`page-start-var-resumed-units.log`, `page-start-var-resumed-clippy.log`,
`page-start-var-measurements.log`, `command-case-tests.log`,
`command-case-final.log`, `command-case-clippy.log`, `command-case-cli.log`.
These logs are supplementary; repository documentation/tests carry the
reproducible knowledge and must suffice on another machine.

**Restart prompt:**

> Read section 0 of docs/reports/ALBUMEASY_4_5_0_COMMAND_MAPPING_HANDOFF.md,
> then the live command checklist and PAGE_START_VAR fixture README. Inspect
> current git status/log before acting. #103--105 are completed bounded slices,
> not production integration or full parity. Preserve the deferred page-frame
> alignment and uppercase-output requests as backlog. Resume with the
> maintainer's selected next task, reusing existing references/tests.

---

## Historical handoff (2026-11-06)

The following records the earlier investigation. Its branch, worktree and
"next action" descriptions are historical; section 0 supersedes them.

**Purpose:** Preserve the current investigation, decisions, evidence, and exact
next action so a future contributor or LLM can resume without reconstructing
the conversation.
**Status:** The stamp command family's documented syntax and a static support
audit are recorded. Two representative stamp fixtures have been rendered and
documented. The complete 4.5.0 command/behavior matrix is not complete. Native
production PDF integration is explicitly deferred.

## 1. Resume here

The **stamp command family's documented syntax and behavior have now been
checked** against the installed AlbumEasy 4.5.0 command reference. Start by
reading [ALBUMEASY_4_5_0_STAMP_COMMAND_MATRIX.md](../guides/ALBUMEASY_4_5_0_STAMP_COMMAND_MATRIX.md),
which also contains the static application-support audit and current fixture
status. Read the [stamp fixture execution report](../testing/ALBUMEASY_STAMP_COMMANDS_V1.md)
for exact commands, outputs, hashes, limitations, and follow-up coverage. The
first two isolated reference fixtures, `stamp-shapes-v1` and
`stamp-image-overloads-v1`, have been rendered against installed AlbumEasy
4.5.0. Continue with the remaining stamp coverage and command families. The
4.2.1 inventory and gap report remain legacy evidence, not the current target
definition.

Do not switch production generation to the native PDF backend, claim AlbumEasy
4.5.0 compatibility, or wire native generation into the application until the
4.5.0 map and its coverage gaps have been reviewed. Native command parsing and
PDF rendering are active implementation work and should proceed incrementally
against verified 4.5.0 grammar and reference fixtures. The native PDF work to
date includes bounded proof/layout capabilities and a first standalone
`.stampalbum` image-to-PDF slice; it is not yet a complete command renderer or
an integrated application replacement.

The maintainer approved `.stampalbum` as the program-owned, human-editable
format. The first direct file-to-native-PDF slice is tracked in
[NATIVE_PDF_IMPLEMENTATION_PLAN.md](./NATIVE_PDF_IMPLEMENTATION_PLAN.md): native
PDF generation comes from the program's canonical document, while
AlbumEasy-compatible TXT is a separate export and existing AlbumEasy TXT is an
import-compatibility path. Production application routing remains gated.

**Starting prompt for a future LLM:**

> Read `docs/reports/ALBUMEASY_4_5_0_COMMAND_MAPPING_HANDOFF.md` and linked
> references. The stamp command family's documented syntax, static
> application-support audit, and initial reference-rendered fixtures are
> recorded in `docs/guides/ALBUMEASY_4_5_0_STAMP_COMMAND_MATRIX.md` and
> `docs/testing/ALBUMEASY_STAMP_COMMANDS_V1.md`. Continue the uncovered stamp
> cases and then map the remaining AlbumEasy 4.5.0 command families. Keep 4.2.1
> as legacy coverage. Record unknowns; do not guess undocumented behavior,
> modify production behavior, or integrate native PDF generation. Preserve
> unrelated worktree changes.

## 2. Decisions and scope

- **Primary compatibility target:** AlbumEasy 4.5.0.
- **Legacy target:** AlbumEasy 4.2.1 remains separately versioned legacy coverage.
- **Purpose of mapping:** Establish the documented input language and the
  application work needed to support it. Listing a command, accepting it in a
  parser, or emitting it in generated TXT does not establish semantic or
  rendering support.
- **Native PDF status:** Experimental/proof work only. Do not integrate it into
  production until the command language is fully mapped and the project has
  reviewed the implementation/compatibility plan.
- **Evidence policy:** Use the installed 4.5.0 command reference for grammar;
  the changelog alone is not authoritative. Record version and source hashes
  for frozen reference runs. Unknown grammar or behavior must remain an explicit
  open question until evidence resolves it.
- **Baseline policy:** External AlbumEasy and any future internal renderer must
  receive identical frozen TXT bytes for a comparison. Keep versioned fixtures
  per command family; do not adjust golden results simply to make a native
  result pass.

The repository calls the product **AlbumEasy**. “StampEasy” in conversation
refers to this same command-mapping effort unless the maintainer specifies a
different product.

## 3. Current repository checkpoint

At the time of this handoff:

- Branch: `issue/82-native-stamp-row-alignment`
- HEAD: `4f6b6e5e` (`feat: add native section row alignment`, 2026-11-06)
- Recent preceding commits:
  - `73a0f882` — format config help and stamp image tests
  - `3f48f945` — paginate native stamp image and text rows
  - `10af27c8` — combine native stamp images and text rows
  - `47530168` — add bounded native stamp text footprints
- The worktree also has unrelated modified/deleted Scribus manual files.
  Leave these untouched; do not clean, restore, or include them as part of this
  mapping work.
- The stamp-family help audit is recorded in
  [ALBUMEASY_4_5_0_STAMP_COMMAND_MATRIX.md](../guides/ALBUMEASY_4_5_0_STAMP_COMMAND_MATRIX.md).
  Its listed 4.5.0 syntax, documented defaults/state rules, and obsolete forms
  were checked against the installed help archive; the QCH hash and extracted
  text hash are included there. Application support and runtime conformance
  remain incomplete. The detailed repository stamp breakdown remains
  unversioned secondary evidence.
- The initial broad audit made no repository edits and ran no tests. The
  follow-up stamp-family pass created and then updated the matrix with pinned
  help evidence. No AlbumEasy rendering commands or tests were run for that
  documentation-only audit. The native test results below
  are transcribed from the existing implementation plan, not newly run here.

Confirm the actual branch, HEAD, worktree state, and linked-file contents before
acting; the checkpoint can change.

## 4. References and how to use them

1. [AlbumEasy 4.5.0 reference update](../guides/ALBUMEASY_4_5_0_REFERENCE_UPDATE.md)
   records the decision to target 4.5.0, confirmed corrections to 4.2.1-era
   findings, additional command families/overloads, and baseline policy. It
   explicitly says the full 4.5.0 command/support matrix remains outstanding.
2. [AlbumEasy 4.2.1 command inventory](../guides/ALBUMEASY_4_2_1_COMMAND_INVENTORY.md)
   inventories the older manual and its unresolved ambiguities. Keep its counts
   and conclusions labeled 4.2.1; do not extend its denominator to 4.5.0.
3. [AlbumEasy 4.2.1 application gap analysis](ALBUMEASY_4_2_1_APPLICATION_GAPS.md)
   is a static application audit of the old target. It maps TXT generation and
   imported-command validation, but is not 4.5.0 coverage or runtime
   certification.
4. [Native PDF implementation plan](NATIVE_PDF_IMPLEMENTATION_PLAN.md)
   records the larger native-PDF direction, implemented proof slices, remaining
   language work, and explicit production-integration boundaries. In
   particular, T036 describes extending the matrix to 4.5.0; T037–T040 describe
   later parser, shared support, editor, and conformance work.
5. [Configuration parameters inventory](../guides/CONFIG_PARAMETERS_INVENTORY.md)
   records current configuration behavior. A configuration value or generated
   TXT command is evidence about that configuration/output path only; it is not
   evidence that imported commands, native rendering, or export semantics are
   implemented.
6. [AlbumEasy 4.2.1 manual](../AlbumEasy_4.2.1.pdf) is the supplied legacy
   reference. The installed `AlbumEasy45.0.qch` compiled help is the 4.5.0
   source described in the reference update. Use the actual installed
   command/reference pages when building the 4.5.0 grammar; do not infer missing
   details from the changelog.
7. [Core cross-renderer fixture](../../tests/fixtures/albumeasy/core-v1/album.txt)
   is a small 4.5.0 baseline input, not a complete command-family suite.
8. [Native row-alignment proof](../../tests/fixtures/albumeasy/row-alignment-v1/README.md),
   [stamp pagination proofs](../../tests/fixtures/albumeasy/stamp-pagination-v1/README.md),
   and [stamp image/text proofs](../../tests/fixtures/albumeasy/stamp-items-v1/README.md)
   demonstrate native geometry/proof work. They do not prove AlbumEasy language
   parity.
9. [AlbumEasy 4.5.0 stamp command matrix](../guides/ALBUMEASY_4_5_0_STAMP_COMMAND_MATRIX.md)
   contains the checked stamp-family syntax and documented behavior, with an
   explicit application-support audit and conformance-fixture backlog. The
   installed QCH and extraction hashes are recorded in the matrix.

The 4.5.0 reference update says the compiled-help database was inspected
read-only and no full manual copy was added to the repository. Record the
installed help/version hashes in each future frozen baseline run.

## 5. Findings established so far

### 5.1 The 4.2.1 inventory is strictly a legacy baseline

The 4.2.1 inventory reports:

- 92 canonical documented names, including 9 explicitly obsolete names.
- 12 additional `COLOR_` spelling aliases, for 104 documented spellings.
- 11 commands in the separate decorative-border definition language.
- Parameter overloads are not counted as separate command names.
- Its rows are planning summaries, not a full formal grammar or proof of
  application support.

The 4.2.1 application gap report found, for that older target, that 24 of the 92
names appear in imported-command validation; only 18 have specific
argument-range/type validation, and 6 use broad placeholders. It reports 68
names absent from that validator, including the separate 11-command border
language, and none of the 12 `COLOR_` aliases accepted there. These are static
4.2.1-era counts only: do not present them as 4.5.0 coverage or a support
percentage.

### 5.2 Confirmed 4.5.0 corrections to old findings

The 4.5.0 installed help resolves several apparent old-manual gaps:

- Preferred image settings command: `ALBUM_STAMP_IMG_NEW_SETTING`.
  `ALBUM_STAMP_NEW_IMG_SETTING` is an obsolete, functionally identical spelling;
  `ALBUM_STAMP_IMG_SETTING` retains legacy 0.35 mm units. The mm variants are
  not implemented in the backend described by the update. Generated use of the
  preferred spelling is valid.
- `ALBUM_STAMP_IMG_ASPECT_RATIO` and margin text colour commands are documented
  in 4.5.0. Their presence in generated output must not be “fixed” by removing
  valid commands; imported validation/native semantics remain work.
- Image padding, stretch-up, and fixed aspect ratio are separate concepts.
  The new image-setting command has optional vertical padding and stretch.
  Configuration meanings must not be conflated.
- `PAGE_TEXT_PARAGRAPH_START` accepts optional independent right padding in
  4.5.0. A six-argument emission is not inherently invalid; a version-aware
  legacy export policy is still needed.
- `STAMP_ADD_IMG` has a combined image/inside-text overload with vertical
  image/text offsets, optional paired horizontal offsets, and optional
  `NO_FRAME`. Its exact parameter grammar, signed-offset direction, line
  alignment behavior, relative image path rule, and supported PNG/JPEG formats
  are now recorded from 4.5.0 help in the stamp matrix. The initial renderer
  fixture covers representative overload cases; the full optional-argument
  cross-product and invalid-asset cases remain open.
- The old obsolete-command count does not apply to 4.5.0; the reference update
  also identifies the earlier mm spelling and
  `STAMP_BOXES_ADJUST_SAVE`/`STAMP_BOXES_ADJUST_RESTORE` as deprecated.

These are documentation findings, not native runtime certification.

### 5.2.1 Stamp-family reference verification checkpoint

The stamp matrix was checked against the installed
`/home/ray/work/AlbumEasy/AlbumEasy45.0.qch` help archive, SHA-256
`8b81707d861aacceb798b9f38efbbfdc3ea86fa49a57b7e343c3a25d5964d5be`.
The read-only extracted command text was
`/home/ray/.copilot/session-state/80e32404-199a-4106-9649-659ef9662bab/files/albumeasy-450-commands.txt`,
SHA-256
`171cdb2e4d3a2a122ea012743f24fd157e81b4ec3313fa81c372ba5ec0c64f8d`.
No full help/manual text was copied into the repository.

Checked items include stamp shape signatures and orientations; `STAMP_ADD_IMG`
overloads and offsets; image-setting units/defaults; aspect ratio, grayscale,
and hide/show controls; inner and outer border rules; headings and footer
controls; saved-state membership; and obsolete/deprecated forms. The help does
not specify save/restore nesting or unmatched-command behavior, nor every
oversized-image/clipping interaction. Those remain open, not assumed.

This checkpoint changed documentation only. No application behavior, native
PDF integration, rendering baseline, or tests were changed/run.

### 5.2.2 Stamp-family application-support audit checkpoint

The static audit in the stamp matrix separates ordinary TXT generation,
import-parser validation, imported reflow, uploaded image handling, Review UI
editing, export serialization, native proof rendering, and existing test
evidence. Key limits established from source inspection:

- Ordinary generation emits common rectangular `STAMP_ADD`/basic
  `STAMP_ADD_IMG`, `STAMP_HEADING_VA`, selected global image/size settings, and
  cover-image save/restore; this is not the full documented family.
- The importer recognizes only a limited subset. `STAMP_ADD` and
  `STAMP_ADD_BLANK` still use broad placeholder arities; `STAMP_ADD_IMG` accepts
  6–7 arguments without validating its optional seventh token; only the legacy
  `ALBUM_STAMP_IMG_SETTING` setting is recognized. `STAMP_ADD_BLANK_IMG` is an
  application placeholder absent from the inspected 4.5.0 help.
- Reflow measures only `STAMP_ADD_IMG` placements and applies the last global
  `STAMP_BOXES_SIZE_ADJUST`; it does not interpret general shape, border,
  heading, footer, or image-control state.
- The UI edits retained command arguments generically and adds only basic image
  stamps, centered text, and row starts. Saved-layout export serializes the
  validated command model and assets, not arbitrary source text.
- Native PDF stamp tests/proofs use typed proof settings, not the AlbumEasy
  language interpreter. Existing image-setting tests either inspect source
  strings or do not run album generation; imported-backend tests use a stub
  renderer, not real AlbumEasy.

The matrix assigns eight fixture IDs. The first two exist and have
representative AlbumEasy 4.5.0 reference runs; the other six remain planned.
The attached `get_stamps_by_country_series` HTML page documents upstream
stamp-record retrieval only; it is not evidence of AlbumEasy command parsing
or rendering.

The fixture runs changed no production behavior. The next concrete step is to
extend uncovered stamp cases and continue the other command families. The
exact inputs, renderer/help hashes, command lines, diagnostics, and output
evidence are recorded in
[`docs/testing/ALBUMEASY_STAMP_COMMANDS_V1.md`](../testing/ALBUMEASY_STAMP_COMMANDS_V1.md).

### 5.3 4.5.0 additions and enhancements already identified

Reconcile each entry against the 4.5.0 reference. Stamp image controls and
stamp-state entries are now checked in the family matrix; the remaining
families still need exact source citations. Keep command names, spelling
aliases, overloads, and parameter/behavior enhancements as distinct catalog
concepts.

| Area | Entries or behavior to map |
|---|---|
| Image controls | `ALBUM_STAMP_IMG_NEW_SETTING`, `ALBUM_STAMP_IMG_NEW_SETTING_PCNT`, `ALBUM_STAMP_IMG_ASPECT_RATIO`, `ALBUM_STAMP_IMG_GREYSCALE_ON`, `ALBUM_STAMP_IMG_GREYSCALE_OFF`, `ALBUM_STAMP_IMG_HIDE`, `ALBUM_STAMP_IMG_SHOW`; also the `GRAYSCALE` spelling aliases. |
| Stamp state and captions | `ALBUM_STAMP_SETTINGS_SAVE`, `ALBUM_STAMP_SETTINGS_RESTORE`, `STAMP_FOOTER_TXT_ALIGN`, `STAMP_FOOTER_TXT_PAD`. |
| Shapes and frames | `STAMP_ADD_RT_TRIANGLE`, `STAMP_ADD_PENTAGON`, and independently selectable frame edges. |
| Page flow and geometry | `PAGE_START_INFO`, `PAGE_SET_VERTICAL_POS`, `PAGE_QUADRILLE`, `PAGE_ADD_BOX`, `ALBUM_PAGES_CROP_MARKS`. |
| Margins and colours | `ALBUM_PAGES_MARGIN_TXT`, `ALBUM_PAGES_MARGIN_TXT_PAD`, `COLOUR_ALBUM_MARGIN_TXT`, `COLOUR_PAGE_QUADRILLE`, and their documented `COLOR_` aliases. |
| Preprocessing | `$INCLUDE`, `$DEFINE`, `$UNDEFINE`, `$IFDEF`, `$ELSE`, `$ELSEIF`, `$ENDIF`; include dependencies, conditional scope, and source locations. |
| Enhanced overloads/parameters | Column widths and independent padding; independent right padding for paragraphs/rules; multiline headers/footers; title vertical spacing; informational-page count behavior. |

This list is the known delta recorded by the reference update, not a complete
4.5.0 command inventory. Confirm each exact spelling, signature, option,
default, and behavior from the installed reference. Do not treat every listed
name as a newly introduced canonical command without checking whether it is an
alias or versioned spelling.

### 5.4 4.2.1 ambiguities that remain useful audit prompts

The old inventory records these unresolved topics. Recheck them against 4.5.0
before carrying forward any old conclusion:

1. Combined `STAMP_ADD_IMG` grammar and argument ordering — resolved in the
   stamp matrix from 4.5.0 help; runtime conformance still needs fixtures.
2. Old/new image-padding units and documented defaults — resolved in the stamp
   matrix; implementation behavior still needs validation.
3. Command-like colour names appearing in the manual that may be editorial
   errors rather than accepted commands.
4. `PAGE_START_VAR` argument ordering when page-start group substitutions are
   used.
5. Which stamp shapes accept each legacy text-field overload.
6. `IMAGE_EDGE_PAD` axis/sign semantics and border-line endpoint clearances.
7. Whether `ROW_START` or the `ROW_ALIGN_` prefix is a family label rather than
   a valid standalone command.
8. Provenance and status of `STAMP_ADD_BLANK_IMG`, which appears in application
   validation but was not found in the 4.2.1 manual.
9. Reset/nesting rules, automatic overflow, quoted-comment handling, and
   incompletely specified defaults.

Do not assume these remain unresolved in 4.5.0; locate evidence and record the
version and source section resolving each, or preserve it as open.

### 5.5 Application support is layered and incomplete

The existing code has separate ordinary AlbumEasy TXT generation and imported
album parsing/validation/editor/reflow paths. Presence in one does not imply
support in another. The legacy gap analysis notes that current ordinary TXT can
emit more than the importer accepts, while the importer may accept commands
ordinary generation does not emit. The native PDF work is a further distinct
surface.

The mapping must therefore keep separate evidence/status for at least:

- documented syntax and version;
- lexer/parser acceptance and precise argument validation;
- defaults, units, state scope, ordering, and interactions;
- configuration mapping and precedence where applicable;
- layout semantics and native rendering;
- imported-album editing and source/provenance handling;
- TXT export, including version-aware compatibility or divergence;
- conformance fixtures and external-renderer observations.

“Parsed”, “recognized”, “passed through”, “generated”, and “rendered correctly”
are not interchangeable support claims. Never count placeholders or silent
passthrough as completed support.

## 6. Native PDF work already present; boundary to preserve

The latest plan records these bounded native proof capabilities:

- Physical stamp size is separated from frame/mounting clearance and image
  fitting.
- Text footprints are measured and bounded; wrapping/expanded text changes
  reserved footprint without changing physical stamp/image dimensions.
- Stamp images and text can share rows, with prepared assets and failure checks
  before output is written.
- Pagination repacks pending items, resolves final page-count-dependent layout,
  and continues text on line boundaries. Real-stamp proof fixtures document
  selected source-record/image provenance.
- Row alignment supports left/right/justified/center sections; the new native
  default is centered. Proofs record the deliberate policy and expected bounds.
- Current examples use Letter-size PDFs and independent MuPDF geometry checks.

The implementation plan reports for the latest bounded alignment slice: 131
native tests passed, 0 failed, 22 ignored across 22 suites; four config-editor
tests passed; format, Clippy, and a Windows GNU library check passed with
pre-existing warnings. The plan also says physical printing and Windows runtime
remain unverified. These historical reported results do not replace validation
for future changes.

Production AlbumEasy generation and global configuration behavior remain
unchanged. No native production switch, integrated AlbumEasy exporter, or
full-language conformance claim is authorized by the proof work.

## 7. Next work: extend stamp coverage and continue the 4.5.0 mapping

Keep the next slice documentation/audit-only unless the maintainer separately
approves implementation. Stamp command syntax and documented behavior are
checked against pinned 4.5.0 help; the matrix also records a static application
audit and fixture IDs. The initial `stamp-shapes-v1` and
`stamp-image-overloads-v1` reference runs are recorded in
[`docs/testing/ALBUMEASY_STAMP_COMMANDS_V1.md`](../testing/ALBUMEASY_STAMP_COMMANDS_V1.md).
Extend their uncovered cases without changing production code, then apply the
evidence process to remaining command families. Work in this order:

1. **Verify sources and pin versions.** For each remaining family, inspect the
   installed AlbumEasy 4.5.0 command-reference pages/help archive and record the
   exact source location/identity and hash in the mapping. The stamp matrix
   already records the installed help hash. Retain the supplied 4.2.1
   inventory/manual as a distinct legacy source. Do not rely on changelog
   summaries alone.
2. **Build the 4.5.0 catalog.** Enumerate canonical command names, obsolete
   names, aliases/spelling variants, separate-language commands (such as
   decorative-border definitions), overloads, and preprocessing directives.
   Record source section/page for every fact. Do not reuse the 92-name 4.2.1
   denominator or silently merge aliases and overloads into the name count.
3. **Record exact command grammar.** For every command and overload, capture
   argument order, required/optional count, types, accepted values, quoting and
   escaping, units, defaults, version status, and invalid-input behavior where
   documented. Include relevant assets and path rules.
4. **Map semantics and state.** Capture global/page/column/row/stamp/border
   scope, persistence, reset/save/restore behavior, order dependence,
   substitution, preprocessing scope, and interactions with other commands.
   Distinguish documented behavior from observed behavior and project policy.
5. **Audit current implementation by surface.** For each remaining matrix entry, trace
   ordinary TXT generation, imported parse/validation, config mapping, layout,
   native PDF, editor, export, and tests separately. Cite code/tests and assign
   clear statuses such as `documented`, `not inspected`, `not implemented`,
   `partial`, `implemented-unverified`, or `verified` with the verification
   method. Use those findings to implement the standalone native parser and
   renderer incrementally; keep application routing separate. Replace these
   labels with an explicit controlled vocabulary if the matrix schema needs it;
   do not use a single “supported” boolean.
6. **Reconcile the 4.2.1 findings.** Mark each old entry as unchanged, changed,
   obsolete, alias, or unresolved in 4.5.0, with evidence. Revisit the remaining
   ambiguity topics in section 5.4; stamp image grammar and image-setting units
   are now resolved in the stamp matrix. Keep 4.2.1 behavior differences
   visible.
7. **Run conformance fixtures.** The first reference pass for
   `stamp-shapes-v1` and `stamp-image-overloads-v1` is complete; the stamp
   matrix records its coverage limits. Extend those cases, then implement the
   other proposed stamp fixture IDs. For later families, identify a minimal isolated
   input per command, overload, alias, state interaction, and cross-cutting
   language feature. Pin/freeze input bytes and 4.5.0 reference hashes for
   actual renderer comparisons. Include negative controls and legacy-version
   cases where relevant. Do not generate or update golden PDFs without
   reference review.
8. **Review and gate.** Have the maintainer review the complete catalog, the
   unresolved-evidence list, and the proposed fixtures before starting
   implementation phases.

### Minimum exit criteria for the mapping phase

- Every 4.5.0 command, alias, overload, and separate embedded language found in
  the reference has a versioned catalog entry and evidence citation.
- Every entry documents syntax and known semantics or explicitly states what
  remains unknown; no guessed grammar is presented as fact.
- 4.2.1 differences, aliases, and deprecated/obsolete items remain visible.
- Current application evidence is attributed to the correct surface and is not
  confused with behavior certification.
- Fixture requirements cover each command/overload and the important state and
  language interactions; baseline identity and renderer version are recorded.
- The matrix has been reviewed and accepted before full command-language
  implementation planning begins.
- Native command parser/renderer implementation is in scope and may proceed in
  verified slices. Wiring it into production application generation remains
  deferred until the command mapping and compatibility coverage have been
  reviewed and approved.

## 8. Things explicitly not to do during the mapping restart

- Do not switch default/production PDF generation to native code.
- Do not write a broad parser/renderer implementation from the partial 4.5.0
  delta list.
- Do not infer native support from native proof fixtures or AlbumEasy TXT
  emission.
- Do not infer 4.5.0 coverage from the 4.2.1 counts or static gap report.
- Do not treat changelog text, command-name presence, parser allowlists, generic
  editor fields, or raw-text passthrough as complete command semantics.
- Do not silently drop, rewrite, or normalize an unsupported command.
- Do not change unrelated worktree files, local config, production albums,
  databases, or frozen baselines as part of this documentation phase.
- Do not commit or push without following project workflow and maintainer
  direction; pushing requires explicit instruction.

## 9. Final state after this handoff

This document records the current findings and defines the restart. The stamp
family's documented 4.5.0 syntax and help-defined behavior have been verified;
a static application-support audit and initial reference-run evidence are also
recorded. The first two representative stamp fixtures have been created and
rendered, but the complete proposed fixture coverage and the full 4.5.0 matrix
remain unfinished. This work does not resolve all language questions, certify
compatibility, authorize native integration, or claim production tests. The
next deliverable should extend stamp reference coverage, followed by the
remaining versioned command/behavior matrix and its explicit evidence/gap list.
