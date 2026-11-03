# Native PDF generation with AlbumEasy TXT export

**Created:** 2026-11-01  
**Last updated:** 2026-11-03
**Revision:** 10
**Status:** Incremental implementation authorized; isolated native page-settings slice verified; production backend not integrated
**Scope:** Enhanced native-first PDF creation, compatibility-aware AlbumEasy export, and eventual complete AlbumEasy import/edit/render support without the external executable. AlbumEasy is retained as a transitional backend and optional comparison tool, not a permanent import dependency.

### Current implementation checkpoint — verify and implement together

The final goal is to **replace external AlbumEasy PDF generation with internal
PDF generation**. The maintainer now authorizes small vertical slices: verify a
command's semantics, implement a reusable compiled module, compare with frozen
fixtures, validate/commit/merge locally, then stop for review. One file per
AlbumEasy command, grouped by command family, is the approved structure.
This supersedes fixture-only work for approved slices, not all remaining gates.
Production backend/CLI/UI/config changes remain unapproved in this increment.
The integration branch is `development/native-pdf`, created from
`docs/albumeasy-421-command-inventory`. Phase 1 epic is GitHub #55; the bounded
T003 sub-issue is #56, branch `issue/56-t003-baseline-drift-checker`.
The maintainer reports visual and 100%-scale print acceptance of core-v1;
Windows execution remains unverified.

The next approved baby step (#58) verifies `ALBUM_PAGES_SIZE` only: installed-help
semantics, existing configuration mapping and an isolated
[page-size-v1 fixture bundle/report](../testing/ALBUMEASY_PAGE_SIZE_V1.md).
Four valid dimensions and two explicit failure cases were measured without
renderer/application changes. The findings include AlbumEasy's zero exit code on
semantic errors and the current emitter's one-decimal precision mismatch with
fractional configuration/layout values. Neither issue is silently fixed here.

The subsequent margins-only increment (#59) adds
[margins-v1 evidence](../testing/ALBUMEASY_MARGINS_V1.md): symmetric/asymmetric,
fractional and explicit even-page-override references plus two failure cases.
Eight page borders were measured, including the half-stroke inset from margins.
No automatic even-page mirroring was observed without MARGINSE. These command
bundles remain measured specifications, not part of the core-specific test runner.

The spacing-only increment (#60) adds
[spacing-v1 evidence](../testing/ALBUMEASY_SPACING_V1.md): zero, fractional
horizontal-only/vertical-only and combined probes, plus two failure cases.
Measured text/frame deltas isolate both axes. Misleading config descriptions,
default discrepancies and layout-accounting differences are recorded without
changing configuration, application or renderer behavior.

The first compiled slice (#61) implements page size, base/even-page margins and
spacing under `src/native_pdf/commands/page/`, with shared `units.rs`,
`document.rs` and a small proof renderer. Commands validate already-numeric
arguments; there is no second production TXT tokenizer/importer. Required
settings are explicit; no global config is read or silently defaulted.

```text
src/native_pdf/
  mod.rs
  units.rs
  document.rs
  renderer.rs
  commands/
    mod.rs
    page/
      mod.rs
      album_pages_size.rs
      album_pages_margins.rs
      album_pages_marginse.rs
      album_pages_spacing.rs
```

Use the [native proof instructions](../guides/ALBUMEASY_BASELINE_MANUAL_CLI.md#native-rust-page-settings-proof).
The example renders three geometry-only pages (margin border/content guide);
text, rows, images, full border semantics, TXT export and application wiring
are not implemented. Even-page selection uses one-based physical page numbers;
custom footer/parity semantics remain unverified. Vertical spacing is
inter-element state, not an implicit top/bottom inset.

T004/T005/T006/T007 have partial evidence only; none is declared complete.
Do not treat four command modules as complete command-language coverage.

#### #61 verification and implementation record

- Branch: `issue/61-native-page-settings`; integration: `development/native-pdf`.
- Traceability: REQ-001/shared settings, REQ-002/native generation,
  REQ-006/physical dimensions, REQ-007/errors and REQ-009/portability;
  partial T004/T005/T006/T007 only.
- Canonical geometry is unrounded `f64` millimetres, PDF bottom-left origin;
  conversion is `mm * (72 / 25.4)`. Preserve explicit fractional values and
  historical config defaults rather than normalizing either silently.
- A resolved page contains margin/content bounds; horizontal spacing insets both
  content edges. Vertical spacing advances between elements. Border occupancy
  is explicit; the proof uses a fixed single 0.2 mm border plus a diagnostic
  0.1 mm guide, not a command-language border implementation.
- Required size/margins/spacing are constructor inputs. Missing/incorrect
  arguments, non-finite/negative values, zero size, conversion overflow,
  invalid page number and unusable area return explicit errors. Even-page
  settings are distinct. All pages are validated before output writing.
- Scope excludes TXT parsing, persistence/provenance, global config snapshots,
  text/fonts/images/row layout, automatic pagination, compatibility export,
  full border semantics and production routing.

Exact Linux acceptance commands (each final exit code 0):

```bash
ALBUMEASY_BASELINE_CANDIDATE=/home/ray/.copilot/session-state/80e32404-199a-4106-9649-659ef9662bab/files/t003-core-verification-6RHMrG/first/album.pdf cargo test --test native_pdf_page_settings --test albumeasy_baseline -- --include-ignored --nocapture
cargo test --test external_album_backend -- --nocapture
cargo clippy --test native_pdf_page_settings --example native_pdf_page_settings
cargo check --lib --example native_pdf_page_settings --target x86_64-pc-windows-gnu --no-default-features
cargo run --example native_pdf_page_settings -- /home/ray/.copilot/session-state/80e32404-199a-4106-9649-659ef9662bab/files/native-page-settings-first.pdf
```

Results: **6 native + 6 core tests passed**, no skips in that acceptance run;
**23 existing external-album tests passed**, three unrelated opt-in Brazil
tests ignored. Four native sizes are read back using `oxidize-pdf` and compared
with actual AlbumEasy reference MediaBoxes. Margin/even-page layouts and spacing
state use the frozen JSON expectations. Independent MuPDF inspection checks the
actual native margin/content strokes, widths and page transforms across eight
valid margin/spacing cases (12 pages, two rectangles each), within **0.01 mm**.
The extra guide intentionally differs from the reference; strict identical
whole-page traces are not claimed for native proofs.

Negative controls cover all six existing missing-command/argument inputs,
invalid numeric values, unusable even-page geometry, zero page count and output
write failure. The invalid second-page test verifies no partial PDF is written.
Initial TDD compilation failed with E0433 (native_pdf absent) before implementation;
final checks passed. Clippy reports only existing application warnings.
MuPDF still warns `ICC support is not available`; no color-profile claim.

Filesystem impact: new focused Rust command/shared modules, integration tests
and an example; library exports the module but does not call it from app flows.
Tests create self-cleaning temporary PDFs/one invalid file; manual proof/logs
persist only in session files. No frozen fixture/reference, input, production
album, database or local config moved/deleted/changed. No ingest/schema operation
occurred. The single example is explicitly tracked despite the repository's
general `examples/` ignore rule; no ignore policy is changed.

Windows GNU library/example cross-check passed with optional rfd disabled;
Windows execution, default-feature Windows build, native print and font/image
fidelity remain unverified. Portable Path APIs are used without platform
branches. Application flow is unchanged, so no baseline draw.io copy is needed.
Stop for review after this local checkpoint.

Start with the supplied [AlbumEasy 4.2.1 manual](../AlbumEasy_4.2.1.pdf), not just
the commands this application currently emits. The new
[command inventory](../guides/ALBUMEASY_4_2_1_COMMAND_INVENTORY.md) identifies
**92 canonical names**, including **9 obsolete commands** and **11 commands in
the separate decorative-border language**. Twelve documented `COLOR_` aliases
bring the total to **104 spellings**. These counts describe the manual, not
implemented support.

The [application/configuration gap analysis](ALBUMEASY_4_2_1_APPLICATION_GAPS.md)
adds a static matrix for all 92 names: 24 occur in imported-command validation,
including six placeholders; 68 are absent. It also maps existing global and
album-local `config.toml` behavior to command families and flags generator/manual
syntax mismatches. No native album renderer is wired in the inspected paths.
Do not treat these name counts as support percentages.

For each subsequent implementation slice:

1. Review the inventory and resolve its documented syntax/behavior gaps.
2. Define per-command arguments, defaults, state, units, assets and interactions.
3. Audit application support across parsing, layout, native rendering, editing
   and export; do not count accepted names/placeholders as implemented behavior.
4. Define conformance examples and identify missing PDF-library capabilities.
5. Implement a small compiled command module and shared layout action, then test
   the generated PDF against the reference before moving to the next item.
   Record gaps explicitly; do not silently ignore unsupported commands.

The existing phase/task IDs below are retained for continuity, not treated as an
approved execution queue. T036's inventory work is pulled forward into this
checkpoint; full command semantics still need planning before the model is fixed.
T001 and T036 remain unchecked because the detailed matrix/audit is incomplete.
The primary target is now installed **AlbumEasy 4.5.0**, with 4.2.1 retained as
legacy coverage. The [4.5.0 reference update](../guides/ALBUMEASY_4_5_0_REFERENCE_UPDATE.md)
records new command families and corrects version-related gap findings using
the installed help archive. A full 4.5.0 coverage matrix remains to be completed;
the existing 92-name denominator is legacy-only. Prior codebase observations and effort
estimates below require reconfirmation before implementation.

## 1. Goal and recommended direction

Generate enhanced printable stamp-album PDFs directly in Rust using
`oxidize-pdf`, then export editable AlbumEasy-compatible TXT as a companion.
Native capabilities must not be restricted to features AlbumEasy can reproduce.

Use one authoritative editable document and shared layout engine. Native PDF
uses the full document. TXT export uses an explicit compatibility projection
when the target command language cannot express a native feature:

```text
Country data + configuration        AlbumEasy TXT + dependencies
              |                                |
       Document builder             Versioned command interpreter
              \                                /
          Authoritative editable album document + assets
                              |
                  Shared layout and font metrics
                              |
                Enhanced native PDF (primary output)
                              |
        Compatibility projection from document/layout, NOT PDF
                              |
           AlbumEasy TXT + assets + divergence report

Transition only: unsupported imports remain explicit drafts or use
the user-selected external AlbumEasy path; never silently drop commands.
```

Generate the native PDF first, then export TXT from the same frozen document
revision during the same operation. The PDF is not parsed to reconstruct TXT.
Retain explicit TXT-only generation. An approved compatibility adaptation can
have different appearance or pagination; it must not modify the source document
or the native PDF. Reuse shared layout rules for any adapted layout rather than
adding an unrelated second pagination engine.

**Do not replace AlbumEasy immediately.** Initially keep AlbumEasy as the default.
Offer native PDF as an opt-in backend for ordinary generated albums. Promote it
after the ordinary-album gates pass, independently of the later full-import gate.
The eventual goal is native creation and AlbumEasy import without installation
of AlbumEasy. The external program may remain an optional legacy renderer and
development comparison oracle.

### First enhancement: icons on either side of the main title

Support independent left/right icons, or both together, with explicit asset
selection, physical dimensions, aspect ratio, gaps and vertical alignment.
Keep the title centered in a defined title area; reserve sufficient heading
height and prevent overlap with text, margins or the page border. Missing assets,
oversized icons and long/wrapped titles require actionable validation.
Persist the settings in the authoritative album document; expose them in the
appropriate configuration/editor UI.

Determine whether the selected AlbumEasy version can reproduce each title
composition. If not, offer a clearly reported approximation or omission.
Do not assume icons are inherently unsupported, rasterize a whole page silently,
or constrain native layout to achieve cosmetic parity.
The supplied manual already demonstrates title-side images using composed text,
spacing and frameless-image commands (pp. 61–62), and reusable page-start groups
(pp. 23–24). Treat native title-icon controls as a usability/layout enhancement,
not automatically as an AlbumEasy-incompatible feature.

Other enhancements are a backlog, not approved implementation scope: decorative
headers/footers, richer typography, section badges, backgrounds and flexible
image/text compositions. Each needs its own acceptance criteria and export policy.

## 2. Previously inspected starting point — reverify before implementation

| Area | Current implementation | Consequence for this plan |
|------|------------------------|---------------------------|
| PDF library | `oxidize-pdf` is included in [Cargo.toml](../../Cargo.toml); [PDF extraction](../../src/review_ui/pdf_parser/extraction.rs) uses its image operations. | Inclusion/image extraction does not prove that generation, fonts and layout meet our needs; verify these independently. |
| Output orchestration | [album_globals/mod.rs](../../src/album_globals/mod.rs) creates TXT and invokes AlbumEasy to obtain PDF. | This is the main dispatch/result integration point. |
| Ordinary album generation | [generator.rs](../../src/album_globals/generator.rs) remains the active implementation; the entry point in [generator_new/mod.rs](../../src/album_globals/generator_new/mod.rs) is still a stub/fallback. | Do not assume the refactored entry point already provides a complete layout model. Extract behavior from the active implementation. |
| Existing layout helpers | [generator_new/](../../src/album_globals/generator_new/) contains layout, emission, commemorative-layout, helper and postprocessing modules. | Reuse suitable helpers; do not create a competing second layout subsystem. |
| Imported albums | [external_album.rs](../../src/external_album.rs) regenerates from database-backed commands and has its own rendering/approval lifecycle. | Preserve source references, identities, editing and approval; replace the external rendering dependency in a later gated phase. |
| Command validation | [external_album/parser.rs](../../src/external_album/parser.rs) uses a command allowlist; some accepted commands have placeholder argument rules. | Recognition is not implementation. Full support requires grammar, semantics, editing, rendering and export evidence for every command. |
| Settings | [config.rs](../../src/config.rs) supports AlbumEasy executable, automatic PDF generation and diagnostics. | Preserve existing configuration semantics and introduce backend selection additively. |

This is an implementation plan, not a claim that the library's rendering
capabilities or output fidelity have been certified.

## 3. Requirements and boundaries

| ID | Requirement |
|----|-------------|
| REQ-001 | Maintain an authoritative editable document and shared layout for native generation and compatibility projection. |
| REQ-002 | Render ordinary albums to PDF directly with native Rust code. |
| REQ-003 | After native PDF creation, export compatible TXT from the same document revision, with explicit divergence reporting; preserve TXT-only generation. |
| REQ-004 | Add explicit backend selection without changing existing defaults during rollout. |
| REQ-005 | Preserve imported-album source/layout/editing fidelity during transition and ultimately import, edit and render without the external program. |
| REQ-006 | Preserve print geometry and source semantics for the common feature subset; separately approve enhanced-export differences. |
| REQ-007 | Surface unsupported features and output failures; never silently change backend or return a stale PDF as success. |
| REQ-008 | Keep CLI, startup UI, country review UI, configuration and diagnostics consistent. |
| REQ-009 | Maintain Windows/Linux portability and validate both supported code paths. |
| REQ-010 | Support configurable main-title icons on the left, right or both sides without clipping or overlap. |
| REQ-011 | Support the installed AlbumEasy 4.5.0 command reference as the primary target, retaining documented 4.2.1 legacy coverage, including variants, preprocessing, border language, defaults and interactions across import, validation, editing, native rendering and export. Resolve documented ambiguities before claiming complete support. |
| REQ-012 | Produce per-feature/page/source-location export compatibility findings; never silently omit or approximate native enhancements. |
| REQ-013 | Preserve the full editable native document and assets so lossy TXT export/reimport cannot destroy native-only features. |
| REQ-014 | Preserve and document effective configuration semantics: existing sectioned/legacy keys, defaults, units, album overrides and per-run settings; resolve one snapshot for both outputs and explicitly reconcile imported commands with user overrides. |

### Scope boundaries and staged exclusions

- A complete versioned AlbumEasy interpreter is now an eventual deliverable,
  not part of the first ordinary-album release.
- The compatibility baseline means the complete documented album command language,
  parameters, defaults, supported legacy aliases and file/dependency semantics
  for the installed 4.5.0 reference with 4.2.1 legacy coverage. Inventory examples and undocumented behavior
  separately; do not make an unbounded claim about every historical version.
- No recreation of unrelated AlbumEasy desktop UI or undocumented internals.
- No changes to StampWorld/Colnect import semantics.
- No unrelated generator cleanup, database redesign or layout-editor rewrite.
- No OCR, PDF-to-TXT reconstruction, or new cloud service.
- No promise of byte-identical PDFs; compare observable layout and print output.

### Proposed backend policy — approve before coding

Use a typed setting such as `pdf_backend = "albumeasy" | "native"`.
The exact key/CLI naming must follow the project's current configuration patterns.

| Album/action | AlbumEasy selected | Native selected |
|--------------|-------------------|-----------------|
| Ordinary album, PDF requested | Existing TXT + AlbumEasy PDF; warn about unrepresentable enhancements | Enhanced native PDF, then compatibility TXT and report |
| Ordinary album, TXT only | TXT; no PDF | TXT; no PDF |
| Imported album, transition | Existing saved-layout pipeline | Preflight command coverage; native only for certified input, otherwise explicit blocker/draft or user-selected legacy renderer. |
| Imported album, full-support milestone | Optional legacy renderer | Import/edit/render/export natively without AlbumEasy installed. |
| Native capability/render failure | Not applicable | Return a visible error. Do not automatically invoke AlbumEasy. |

Backend choice and PDF enablement are separate settings. Keep existing
`albumeasy_auto_generate_pdf` behavior until a reviewed compatibility migration is
defined; changing a backend must not unexpectedly turn PDF generation on.
An imported-album exception must be visible in the UI/result, not a silent fallback.
During transition, uncertified imports may still require AlbumEasy. At the final
milestone no import/render/approval/edit action may require or launch it. Offline
source-to-document semantic validation and user-visible fidelity approval replace
mandatory two-render comparison; external comparisons remain development evidence,
not a runtime requirement.

### Compatibility export contract

| Classification | Required behavior |
|----------------|-------------------|
| Exact/compatible | Emit supported AlbumEasy commands and preserve meaning within certified tolerances. |
| Approximated | Offer a documented mapping and identify affected features/pages and expected differences. |
| Omitted | Explicitly identify native-only content excluded from TXT; require an approved policy. |
| Blocked | If no approved safe mapping exists, fail TXT export visibly; native PDF can remain a separately successful artifact. |

Default to strict export until an explicit approximation/omission policy is
approved. A saved album policy may avoid repeated prompts, but every run still
reports applied losses. Do not silently flatten text or pages into images.
Rasterized feature export is a future opt-in option only if readability,
editability, asset licensing and print quality are addressed.

Report source document revision, target language version, command/feature,
page/element or source location, action, reason and severity. Show summary in
CLI/UI and persist a companion report (format/path to be agreed). Keep native
document/database plus assets authoritative; TXT alone is not a lossless backup.

## 4. Work breakdown and implementation sequence

All tasks start unchecked. IDs remain stable as the document is updated.
Suggested new files below are proposals, not an instruction to add unnecessary
abstractions. Reuse existing modules wherever they already own the behavior.

### Phase 1 — inventory, baseline and capability proof

**Plan P1.1:** Freeze observable behavior and pin the full target language before altering the generator.
**Plan P1.2:** Prove that `oxidize-pdf` can render the minimum required album page.
**Plan P1.3:** Audit configuration semantics and approve a backwards-compatible native mapping (REQ-014).

- [ ] T001 [Plan:1.1] Complete the command-first audit using the installed 4.5.0 help, `docs/guides/ALBUMEASY_4_5_0_REFERENCE_UPDATE.md` and the retained 4.2.1 inventory/manual. Inventory emitted commands/current layout/configuration in `src/album_globals/generator.rs`, `generator_new/emit.rs`, `generator_new/layout.rs`, `generator_new/commemorative_layout.rs`, and `generator_new/postprocess.rs`; record versioned support evidence, missing behaviors and unresolved gaps. Legacy name inventory and 4.5.0 corrections exist; full modern matrix/audit remains open.
- [ ] T002 [Plan:1.1] Inventory all ordinary/filtered generation callers in `src/album_globals/mod.rs`, `src/modes.rs`, `src/review_ui/http_handlers.rs`, and generation helpers. Record imported-album dispatch separately from ordinary albums.
- [ ] T003 [Plan:1.1] Capture deterministic baseline TXT and fresh AlbumEasy PDFs for representative ordinary albums; retain existing imported-album fixtures as regression references. The initial frozen `tests/fixtures/albumeasy/core-v1/album.txt` and `expected.json` establish basic geometry/text/row comparisons; see `docs/testing/ALBUMEASY_CORE_V1_BASELINE.md` for measured 4.5.0 reference evidence. Full command-family, ordinary-generated, asset and native comparisons remain open. Never mutate production albums.
- [ ] T004 [Plan:1.2] Verify the installed `oxidize-pdf` generation API and license, font embedding/measurement, Unicode, images/transparency, page sizes, metadata and write errors. Record supported/unsupported features and exact limitations in this document.
- [ ] T005 [Plan:1.2] Build an isolated proof page with known millimetre dimensions, a stamp image, caption, heading and border. Verify PDF dimensions, text extraction and a 100%-scale printed ruler.
- [ ] T046 [Plan:1.3] Complete and approve the K1–K12 configuration contract in `docs/reports/ALBUMEASY_4_2_1_APPLICATION_GAPS.md`: map local `config.toml`, `src/config.rs`, `config_template.toml`, `config_full_example.toml` and `docs/guides/CONFIG_PARAMETERS_INVENTORY.md` to actual command/layout semantics, defaults, units, alias precedence and album/per-run overrides. Resolve image-padding/stretch/aspect, header/title and row-field naming conflicts before model design; preserve credentials and keep the local config untouched. Define fixture expectations and migration decisions, not implementation. ID appended for stability; planning precedes Phase 2.

**Exit gate:** Command/feature and configuration contracts are approved; native proof page opens and prints
correctly; required missing capabilities are identified before the larger refactor.
Stop for a maintainer decision if the library cannot meet a core requirement.

### Phase 2 — shared layout and font metrics

**Plan P2.1:** Establish an authoritative editable document and resolved layout, with native-only features and provenance.
**Plan P2.2:** Move layout decisions out of TXT emission without changing behavior.

- [ ] T006 [Plan:2.1] Implement shared document/layout under approved `src/native_pdf/document.rs`, with one file per AlbumEasy command grouped under `commands/`: album/page settings, rows, placements, text styles, title compositions, borders and resolved assets. #61 implements explicit page settings only. Preserve stable element identity, imported source locations and document revision; persist native-only settings additively without discarding command provenance. Resolve one effective config snapshot per run; explicit imported settings remain authoritative unless a recorded user override changes them.
- [ ] T007 [Plan:2.1] Define canonical units, coordinate origin, rounding, page/margin bounds and physical-versus-presentation boxes. #61 implements `src/native_pdf/units.rs` with unrounded f64 mm, PDF bottom-left coordinates, point conversion and validated page/margin/content bounds; wider precision/export/box policies remain open.
- [ ] T008 [Plan:2.2] Establish a shared font measurement/wrapping policy. Map current AlbumEasy fonts explicitly to licensed, available fonts; specify embedding and substitution behavior. Never silently change fonts.
- [ ] T009 [Plan:2.2] Extract ordering, stamp selection, row packing, spacing modes, caption heights, page breaks, even-page margins and image-box sizing from the active generator into the shared builder, reusing existing helpers.
- [ ] T010 [Plan:2.2] Ensure the new path returns an actual layout/result rather than relying on `generator_new/mod.rs`'s empty-path fallback. Preserve callbacks, progress, filter behavior and existing output naming.

**Exit gate:** Shared layouts reproduce baseline order, page count and dimensions;
TXT behavior is still preserved. No backend-specific pagination remains hidden in
the native renderer.

### Phase 3 — AlbumEasy TXT exporter

**Plan P3.1:** Export a version-targeted compatibility projection of the native document/layout.

- [ ] T011 [Plan:3.1] Adapt `generator_new/emit.rs` and related emit/postprocess helpers to consume a compatibility projection of the shared document/layout; classify each feature as exact, approximated, omitted or blocked before writing target-version commands.
- [ ] T012 [Plan:3.1] Preserve escaping, relative image paths, command order, row spacing semantics, page breaks, fonts, numbering and all supported global settings. Keep asset paths portable.
- [ ] T013 [Plan:3.1] Validate emitted TXT against the pinned language and, during development, compare AlbumEasy-rendered outputs against the common-feature baseline. Persist divergence findings and require explicit approval for lossy mappings; native-only enhancements need not match AlbumEasy appearance.

**Exit gate:** Exported TXT opens/renders in AlbumEasy. Preserve byte-normalized
baseline TXT where feasible; review intentional differences and prove their effect.
The TXT is useful for editing, not merely syntactically valid.

### Phase 4 — native PDF renderer

**Plan P4.1:** Render the shared layout using `oxidize-pdf`.  
**Plan P4.2:** Produce reliable paired outputs with explicit diagnostics.

- [ ] T014 [Plan:4.1] Implement a focused renderer, for example `src/album_globals/native_pdf.rs`, consuming resolved pages/placements without repeating selection or pagination.
- [ ] T015 [Plan:4.1] Render page sizes, margins, borders, headings, page numbers, stamp/image boxes, aspect-ratio-preserving images, captions, alignment and supported colors. Reject unsupported commands/styles explicitly.
- [ ] T016 [Plan:4.1] Implement font embedding and glyph validation using the Phase 2 policy; verify accented country names, punctuation, denomination fractions and currency symbols.
- [ ] T017 [Plan:4.2] Freeze one document revision, stage/validate the native PDF first, then generate compatible TXT/assets/report. Distinguish PDF success/TXT blocked, PDF failure and publication failure; do not label a pair complete when TXT failed. Preserve previous files and report cleanup failures. Do not promise multi-file atomicity across a crash.
- [ ] T018 [Plan:4.2] Extend `GeneratedAlbumArtifacts`/diagnostics in `src/album_globals/mod.rs` to report document revision, selected/actual backend, individual output status, fresh paths, target language and compatibility report. Avoid success-shaped empty paths.

**Exit gate:** Native rendering produces fresh, valid PDFs with correct page
geometry; failure paths cannot masquerade as success or overwrite a valid prior
artifact with a partial file.

### Phase 5 — optional backend rollout and imported-album protection

**Plan P5.1:** Wire configuration, CLI and UI consistently.  
**Plan P5.2:** Preserve imported albums on the legacy path until their native coverage is certified.

- [ ] T019 [Plan:5.1] Add typed backend selection to `src/config.rs`, `config_template.toml`, `config_full_example.toml`, and `docs/guides/CONFIG_PARAMETERS_INVENTORY.md`; preserve missing-key defaults and album-local overrides.
- [ ] T020 [Plan:5.1] Wire a CLI backend override through `src/cli.rs`, `src/main.rs`, `src/modes.rs`, and shared orchestration. Regenerate `docs/CLI_USAGE.md` and update `docs/guides/CLI_OPTIONS_INVENTORY.md`.
- [ ] T021 [Plan:5.1] Update startup/country-review controls in `src/review_ui/views/bootstrap.html`, `bootstrap.js`, `compare.html`, `compare.js`, and `http_handlers.rs`; check configuration editor behavior and all Generate PDF actions.
- [ ] T022 [Plan:5.2] Preserve `src/external_album.rs` saved-command data and approval/revision lifecycle during transition. In `src/review_ui/views/external_album.html`, explain current coverage and external dependency; keep explicit fallback until Phases 7-9 certify native import.
- [ ] T023 [Plan:5.1,5.2] Update executable-required checks, status/help messages, progress, open-output behavior and diagnostics: native ordinary generation must work with no AlbumEasy installation; imported validation remains explicitly dependent on it.

**Exit gate:** Backend selection is consistent across entry points. Existing
configuration still follows the existing backend. Unsupported imported-album
native requests cannot silently render a different document.

### Phase 6 — parity certification, documentation and default decision

**Plan P6.1:** Validate print fidelity and portability.  
**Plan P6.2:** Decide whether native becomes the default.

- [ ] T024 [Plan:6.1] Add focused layout/export/native renderer checks in `tests/` and inline test modules using existing tooling. Cover boundaries, text wrapping, images, Unicode, paired-output failure and repeat generation.
- [ ] T025 [Plan:6.1] Exercise CLI and both web generation paths, filters, TXT-only requests, backend overrides, AlbumEasy unavailable, native errors and imported-album preservation.
- [ ] T026 [Plan:6.1] Compare native and AlbumEasy-rendered TXT outputs for representative albums on Windows/Linux; record geometric/visual differences and 100%-scale print checks.
- [ ] T027 [Plan:6.1] Measure time, peak memory and output size for small/medium/large albums. Investigate regressions before rollout; do not claim speed or smaller files without measurements.
- [ ] T028 [Plan:6.2] Update user guides/release notes and create a derivative of the historical Draw.io baseline for the completed flow. Leave `app-logic-before.drawio` untouched.
- [ ] T029 [Plan:6.2] Regenerate `knowledge/` using `okf-rs generate`, complete maintainer review and record the default decision here. If approved, make native default for ordinary albums in a separate reviewed change; keep explicit AlbumEasy selection.

**Exit gate:** Maintainer accepts the parity evidence and documented limitations.
If not accepted, native stays opt-in and AlbumEasy remains the default.
This certifies ordinary generation only, not full AlbumEasy import support.

### Phase 7 — enhanced title composition and compatibility reporting

**Plan P7.1:** Implement left/right title icons as the first enhanced feature (REQ-010, REQ-013).
**Plan P7.2:** Complete enhancement/export divergence handling (REQ-003, REQ-012).

- [ ] T030 [Plan:7.1] Extend the Phase 2 document/layout in `src/album_globals/album_layout.rs` (or its established equivalent) with independent left/right title icon assets, dimensions, aspect ratio, gaps, vertical alignment and title-area sizing; preserve native settings in the album database.
- [ ] T031 [Plan:7.1] Implement heading measurement and native rendering in `src/album_globals/native_pdf.rs` and shared layout helpers. Cover long/wrapped titles, one/both icons, even-page margins and explicit failure on missing/oversized assets.
- [ ] T032 [Plan:7.1] Expose title icon controls through the owning album/config editor and `src/review_ui/http_handlers.rs`; wire applicable `src/config.rs`, template/example/inventory settings and portable asset resolution.
- [ ] T033 [Plan:7.2] Extend `generator_new/emit.rs` compatibility projection: prove any exact AlbumEasy icon mapping or offer explicit approximation/omission/blocking; retain the enhanced native original.
- [ ] T034 [Plan:7.2] Wire per-output compatibility results and saved loss policies through `src/album_globals/mod.rs`, `src/cli.rs` and both web generation flows. Add title-icon and strict/lossy-export regressions under `tests/`.
- [ ] T035 [Plan:7.1,7.2] Add an enhancement backlog and capability/export classifications to this document. Only title icons are committed scope; obtain approval before implementing other ideas.

**Exit gate:** Icons render and print correctly, settings survive save/reload,
TXT remains valid, and every intentional export difference is reported. The
native document can regenerate the enhanced PDF even after a lossy TXT export.

### Phase 8 — complete AlbumEasy language and application support

**Plan P8.1:** Implement a versioned command language, not an allowlist of names (REQ-011).
**Plan P8.2:** Complete command-to-document/render/editor/export support (REQ-005, REQ-011, REQ-013).

- [ ] T036 [Plan:8.1] Extend the coverage matrix to installed 4.5.0 using `docs/guides/ALBUMEASY_4_5_0_REFERENCE_UPDATE.md` and exact installed-help grammar; preserve `docs/guides/ALBUMEASY_4_2_1_COMMAND_INVENTORY.md` as the legacy 92-name/12-alias baseline. Reconcile renamed/obsolete names, additions and overloads; add per-command units, defaults, scope/state, interactions, parse/validation/edit/render/export evidence and conformance fixtures. Planning portion is pulled forward before Phase 2; implementation remains deferred.
- [ ] T037 [Plan:8.1] Replace placeholder validation in `src/external_album/parser.rs` with documented argument rules and precise source diagnostics. Implement documented quoting/escapes, comments, ordering, includes/dependencies and command state semantics where present; reject unknown commands without silently ignoring them.
- [ ] T038 [Plan:8.2] Implement command families incrementally across `src/external_album/parser.rs`, `src/external_album/reflow.rs`, shared document/layout and native rendering: page/global settings, text/fonts, rows/alignment, stamp/blank/image placements, borders/colors/decorations, and every additional family discovered by inventory.
- [ ] T039 [Plan:8.2] Extend `src/review_ui/views/external_album.html` and owning API for typed editing of every documented supported command/item, preserving order, identity, scope, revisions and source provenance. Recognition or raw-text passthrough alone does not count as application support.
- [ ] T040 [Plan:8.1,8.2] Add per-command and interaction conformance fixtures under `tests/` and `samples/`; validate import -> edit -> native render -> TXT export -> reimport. Include defaults, aliases, legacy units, invalid arguments and page-spanning state.

**Exit gate:** Every command/item in the pinned reference is implemented and
evidenced across all required surfaces. No placeholder grammar, parse-only
support or unsupported silent passthrough may be counted as complete. Split
T037-T040 into bounded command-family tasks after inventory; do not attempt one
large speculative rewrite.

### Phase 9 — standalone native AlbumEasy import and final certification

**Plan P9.1:** Remove mandatory external rendering from the complete import lifecycle (REQ-005, REQ-007, REQ-008).
**Plan P9.2:** Certify and release standalone compatibility (REQ-006, REQ-009, REQ-011).

- [ ] T041 [Plan:9.1] Adapt `src/external_album.rs` inspection/import/validation to parse original TXT and dependencies into the authoritative document, preserving copied inputs, asset checksums, source diagnostics and additive schema compatibility. Keep unsupported/failed imports explicit drafts.
- [ ] T042 [Plan:9.1] Replace mandatory external render/compare in imported-album approval and page/album edit regeneration with native semantic validation and rendering. Preserve manual fidelity approval, stale-write rejection, page-local reflow and current-PDF status; explain validation limits when no independent reference exists.
- [ ] T043 [Plan:9.1] Remove unconditional executable-required gates from import/editor/generation APIs and UI. On a machine with no AlbumEasy installed, verify inspect -> import -> approve -> edit -> regenerate -> export -> reopen; no subprocess invocation is allowed on the native path.
- [ ] T044 [Plan:9.2] Certify the full command matrix, interaction corpus and representative real albums on Windows/Linux, using development-time reference renders when available. Resolve unexplained semantic/geometry differences; report intentional native-only export divergence separately.
- [ ] T045 [Plan:9.2] Update user guides, compatibility/version statement, migration notes, derivative flow diagram and `knowledge/`; record standalone-import acceptance and default-backend decision separately from the earlier ordinary-album rollout.

**Exit gate:** Supported-version AlbumEasy files import/edit/render/export
without the external program, including a clean-machine test. Full compatibility
is a versioned conformance claim, not a promise of byte-identical PDFs or support
for undocumented commands. The external backend becomes optional.

## 5. Estimated work involved

These are **planning ranges**, not measured estimates or a delivery promise.
One experienced developer working full-time; a day means approximately eight
focused engineering hours. Revise after Phase 1.

| Phase | Complexity | Estimated effort |
|-------|------------|------------------|
| 1. Inventory, baseline and library proof | Medium | 2–4 days |
| 2. Shared layout extraction and font metrics | High | 6–12 days |
| 3. Compatible TXT exporter | Medium/high | 3–6 days |
| 4. Native renderer and reliable outputs | High | 5–10 days |
| 5. Backend controls and transitional import protection | Medium | 3–5 days |
| 6. Parity, cross-platform validation and rollout | High | 5–10 days |
| **Ordinary-album foundation (Phases 1–6)** | | **24–47 developer-days** |
| 7. Title icons and divergence handling | Medium/high | 4–8 days |
| 8. Full command language, rendering and editing | Very high; inventory-dependent | 20–45 days |
| 9. Standalone import lifecycle and full conformance | High | 8–16 days |
| **Expanded total before contingency** | | **56–116 developer-days** |

Allow roughly 20–30% contingency: approximately **67–151 developer-days overall**.
The previous **29–61 day** range covered the ordinary-album foundation only.
Full language support materially expands the project; revise the new estimate
after the versioned command matrix is complete. Unspecified future enhancements
are excluded.

**Dependencies:** Phase 1 precedes Phase 2; Phase 2 precedes Phases 3/4. Native
and TXT exporters can then be developed independently against a frozen layout
contract. Phase 5 needs both exporters and stable generation results. Phase 6
requires the integrated paths; start collecting evidence throughout.
Phase 7 builds on the Phase 2/4 foundation and compatibility exporter; it can
precede Phase 6 certification if included in the first native release. Phase 8
can start inventory/parser work after Phase 1, but complete semantics requires
the shared document/renderer. Phase 9 depends on Phase 8 coverage and integrated
editor support. Preserve task IDs; phase dependencies, not numeric IDs alone,
determine execution order.
The maintainer approved incremental compiled slices alongside command analysis.
T001/T036/T046 contracts still gate broad backend/import/config integration, not
isolated tested page primitives. Each slice must record resolved semantics and
unresolved interactions; do not require re-analysis of already verified behavior.

## 6. Validation strategy and proposed acceptance criteria

Use existing Cargo/Node test infrastructure; no new test framework by default.
Add runnable regression coverage as part of implementation, not after rollout.
Use isolated temporary albums/databases and preserve all user inputs.

### Three independent certification tracks

1. **Common-feature baseline:** compare native and AlbumEasy outputs for
   geometry, ordering, wrapping and pagination within approved tolerances.
2. **Enhanced native documents:** certify the native result independently;
   verify TXT validity and the accuracy/approval of every declared export loss.
   Pixel or pagination parity with the enhanced PDF is not required.
3. **Imported AlbumEasy conformance:** certify every pinned command and its
   interactions, source roundtrips and standalone editing/rendering. Passing
   ordinary-album tests does not satisfy this track.

### Representative coverage

- Small ordinary album and multi-page ordinary album.
- Dense captions, long series titles and wrapping at a page boundary.
- Mixed stamp sizes, wide/tall stamps, fixed-spacing rows and commemorative layouts.
- Alternate/even-page margins, headings, borders and page numbering.
- Images with transparency, missing assets, non-ASCII filenames and path spaces.
- Unicode text, fractions/currency symbols and unavailable font glyphs.
- Empty selection, year/catalogue filters and TXT-only requests.
- Existing imported albums, repeated placements and page-local saved edits.
- Title icons (left/right/both/none), missing/oversized assets, long headings and
  save/reload of native-only settings.
- Strict and approved lossy exports; exact report findings; native PDF success
  with TXT export failure; preservation of enhanced originals.
- Every pinned command, defaults/aliases and state interactions; unknown command
  errors and include/dependency path handling.
- Standalone import/edit/render/export on Windows/Linux with no AlbumEasy executable.

### Proposed thresholds — approve in Phase 1

| Check | Proposed acceptance |
|-------|---------------------|
| Page count and stamp selection/order | Exact match for common-feature/import conformance fixtures; enhanced export differences require an explicit compatibility finding and approval. |
| Paper size and physical stamp/box dimensions | Within 0.1 mm of specified geometry. |
| Placement and margin/border geometry | Within 0.25 mm of the approved baseline. |
| Text wrapping | No clipping, overlap or omitted glyphs; line-break differences require explicit review. |
| Export compatibility | Companion TXT is valid for the pinned target version; development reference renders have no command errors. Native-only differences are itemized rather than required to be visually identical. |
| Visual comparison | Review at a fixed rendering scale/DPI; approve intentional differences. Do not rely solely on a pixel score. |
| Printed output | Ruler/box check at 100% scale with printer fit-to-page disabled. |
| Error behavior | Failed rendering/publishing gives explicit failure; no stale/partial PDF reported as newly generated. |
| Windows/Linux | Relevant tests and build/type-check pass on both; disclose any unverified runtime/print checks. |
| Command coverage | Every pinned command/variant has parse, semantics, editor, native-render and export evidence; no placeholder support. |
| Standalone import | Full native lifecycle succeeds without locating or launching AlbumEasy; source/asset/revision fidelity remains intact. |
| Enhanced title | Correct placement/print dimensions, no overlap/clipping, persistent settings and accurate export compatibility findings. |
| Loss reporting | Every applied approximation/omission is visible and recorded; strict export blocks unsupported mappings without destroying the native output/document. |

Run the smallest relevant checks per phase, then the relevant regression suites
before release. Record commands, exit codes, pass/fail counts, temporary file/DB
impacts and discovered logic risks in `docs/testing/`.

## 7. Main risks and decisions still open

| Risk/decision | Required action |
|---------------|-----------------|
| Font metrics differ from AlbumEasy, changing pagination. | Prove shared wrapping/measurement and font mapping before certifying the layout. |
| TXT row commands cause AlbumEasy to calculate positions differently. | Compare rendered companion TXT, not just command strings; identify representability gaps. |
| Active generator mixes selection, placement and emission. | Extract incrementally with baseline parity; avoid completing an unrelated full refactor. |
| Imported command state differs from ordinary layout. | Preserve provenance and semantics through an explicit interpreter/document adapter; retain the old path until command coverage is certified. |
| Library image support does not meet print quality or format needs. | Verify formats/resolution/transparency; reuse existing image helpers where suitable. |
| A saved/global backend preference accidentally affects import validation. | Make the imported-album exception explicit and test every imported render/validate/edit action. |
| Partial paired-file publication. | Define accurate partial-success reporting and staging/cleanup; never claim impossible cross-file atomicity. |
| Missing fonts, assets or external AlbumEasy executable. | Return actionable diagnostics; no silent substitutions or automatic fallback. |
| Full language scope/version drift. | Pin the reference version and count every command/variant; treat later versions as explicit follow-on work. |
| Enhanced PDF and TXT differ intentionally. | Keep the native original authoritative and report exact/approximated/omitted/blocked outcomes; do not present lossy roundtrips as lossless. |
| Validation without an independent renderer cannot prove visual equivalence. | Use semantic/geometry conformance fixtures and development reference evidence; retain user fidelity approval and disclose gaps. |

**Approval record**

- Backend setting/override naming: **Pending**
- Initial font family mapping and embedding license: **Pending**
- Required initial command/style subset: **Pending Phase 1 inventory**
- Geometry/visual thresholds: **Proposed; pending approval**
- Native-default promotion: **Not approved; AlbumEasy remains default**
- Enhanced native PDF with explicitly divergent compatibility export: **Requested direction**
- Main-title left/right icons: **Requested scope; detailed design pending**
- Complete 4.2.1 AlbumEasy application support and standalone import: **Requested scope; initial manual inventory recorded, detailed coverage/ambiguities pending**
- Export loss approval/report format: **Proposed strict default; design pending**
- Other native enhancement ideas: **Backlog only; not approved**

## 8. Requirement mapping

| Requirement | Plan items | Completion evidence |
|-------------|------------|---------------------|
| REQ-001 | P2.1, P2.2 | Shared layout builder; parity checks; T006–T010 |
| REQ-002 | P1.2, P4.1 | Native proof and renderer; T004–T005, T014–T016 |
| REQ-003 | P3.1, P4.2 | Companion TXT and paired results; T011–T013, T017–T018 |
| REQ-004 | P5.1, P6.2 | Typed backend settings and reviewed default; T019–T021, T029 |
| REQ-005 | P5.2, P8.2, P9.1 | Transitional protection then standalone lifecycle; T022–T023, T038–T043 |
| REQ-006 | P2.2, P3.1, P4.1, P6.1 | Geometry, text and print parity evidence; T008–T016, T024–T026 |
| REQ-007 | P4.2, P5.2, P6.1, P9.1 | Failure diagnostics, staging and unsupported-feature behavior; T017–T018, T022–T026, T041–T043 |
| REQ-008 | P5.1, P5.2, P7.2, P9.1 | CLI/UI/help/config consistency and standalone gates; T019–T023, T025, T034, T041–T043 |
| REQ-009 | P6.1, P9.2 | Windows/Linux build, runtime and print evidence; T024–T027, T044 |
| REQ-010 | P2.1, P7.1 | Persistent title composition and renderer/editor checks; T006, T030–T032, T035 |
| REQ-011 | P1.1, P8.1, P8.2, P9.2 | Pinned command matrix and complete conformance evidence; T001, T036–T040, T044–T045 |
| REQ-012 | P3.1, P4.2, P7.2 | Compatibility projection and visible persisted findings; T011–T013, T017–T018, T033–T035 |
| REQ-013 | P2.1, P7.1, P8.2 | Editable native-only settings, stable identities and provenance; T006, T030–T032, T038–T040 |
| REQ-014 | P1.3, P2.1, P5.1 | Configuration semantic/precedence map, snapshot and migration checks; T046, T006, T019–T021 |

## 9. How to keep this plan up to date

1. Update **Last updated**, phase status and task checkboxes after each work batch.
2. Keep task IDs stable; add new tasks rather than renumbering completed ones.
3. Mark a task complete only when its outcome and applicable checks are recorded.
4. Record deviations, blockers and changed estimates before starting affected work.
5. Link evidence, commits or local branch names in the progress log.
6. Do not mark native as default until T029 approval is recorded.
7. Track standalone-import approval separately at T045; ordinary native PDF
   approval does not mean every AlbumEasy command is supported.
8. Revisit estimates when the pinned command matrix or enhancement scope changes.

### Recorded execution workflow — approved for bounded verified vertical slices

- Use GitHub for this roadmap, as explicitly requested: one parent issue (epic)
  per phase and a sub-issue/branch per T-task. P-items group tasks; they do not
  require their own branches.
- During implementation, start each task from the designated main development
  branch, validate its acceptance criteria, commit the task, merge locally back
  to that integration branch, then start the next dependency-ready task there.
- Task commits and local merges may be automatic after required checks pass.
  Ask before any push. Do not commit failing or unvalidated implementation.
- Record epic/sub-issue, branch, checks, commit and merge evidence per task.
- The maintainer selected `development/native-pdf` from the current documentation
  branch. Preserve accumulated planning/baseline deliverables in the validated
  T003 increment; exclude unrelated manual/autosave files.
- GitHub #55 is the Phase 1 parent and #56 its T003 checker sub-issue. Full T003
  coverage remains incomplete after this subset; stop for review before the next
  increment. See the [baseline report](../testing/ALBUMEASY_CORE_V1_BASELINE.md)
  for commands, counts, negative controls and limits.

| Phase | Status | Owner | Evidence / blocker |
|-------|--------|-------|--------------------|
| 1 | Planning inventory in progress | Unassigned | Initial 4.2.1 name inventory recorded; command behavior/support audit remains open |
| 2 | Page-settings foundation started | Unassigned | #61: four small command modules, f64 units/state; full shared document/font/pagination contracts open |
| 3 | Not started | Unassigned | Depends on shared layout |
| 4 | Isolated geometry proof started | Unassigned | #61: native page sizes, margin borders and content guides; no production backend |
| 5 | Not started | Unassigned | Depends on both exporters |
| 6 | Not started | Unassigned | Depends on integrated generation paths |
| 7 | Not started | Unassigned | Shared document/native renderer/export foundation; can precede Phase 6 |
| 8 | Inventory planning pulled forward | Unassigned | T036 initial inventory exists; full matrix and implementation remain open |
| 9 | Not started | Unassigned | Complete command support and editor/native integration |

### Progress and decision log

| Date | Tasks / decision | Outcome / evidence | Next action |
|------|------------------|--------------------|-------------|
| 2026-11-01 | Plan created for options 1, 2 and 4 | Repository generation/import paths inspected; no rendering implementation changed. | Approve boundaries and start Phase 1. |
| 2026-11-01 | Revision 2: expanded native-first scope | Added title icons, explicit export divergence, complete versioned AlbumEasy support and standalone import. T001–T029 retained; T030–T045 added. Previous permanent import exclusions superseded. | Pin target language/reference, create coverage matrix and approve compatibility-loss policy. |
| 2026-11-03 | Revision 3: manual-first planning checkpoint | Repaired duplicated merge-conflict text; inventoried supplied 4.2.1 manual (92 canonical names, 12 additional spelling aliases); recorded unresolved grammar discrepancies and existing title-icon composition. No application changes or commits. | Review command/behavior matrix and remaining maintainer ideas before approving implementation order. |
| 2026-11-03 | Revision 4: application/configuration gap audit | Added per-command static gap matrix, 24-name validator breakdown and K1–K12 config mapping; identified emitted/imported/reference mismatches. Added REQ-014/P1.3/T046 without renumbering existing tasks. No application/config changes or runtime certification. | Resolve syntax/image semantics and approve configuration precedence before shared-model implementation. |
| 2026-11-03 | Revision 5: reference version and core baseline | Selected 4.5.0 as primary with 4.2.1 legacy coverage; installed help resolves several alleged syntax gaps. Added core-v1 TXT and measured reference expectations without application code changes. | Preserve frozen reference; extend the versioned command matrix and add separate command-family fixtures. |
| 2026-11-03 | Revision 6: approved T003 baby step | Maintainer accepted visual/100%-scale print baseline and selected `development/native-pdf`. Added Cargo checksum, geometry/text and strict same-renderer drift checks; two fresh AlbumEasy renders match the frozen reference. Epic #55 / sub-issue #56. Native application/config changes remain paused. | Commit and merge validated checker locally, then stop for maintainer review. Full T003 and command/config contracts remain open. |
| 2026-11-03 | T003 extraction-standard follow-up (#57) | Maintainer requested existing `oxidize-pdf` for baseline text extraction instead of `pdftotext`; all 20 text assertions preserved, with extraction failure/page-count/truncation gates. MuPDF geometry/trace checks retained. Six explicit checker tests pass. | Stop for review; native application/config changes and broader import-parser migration remain outside this increment. |
| 2026-11-03 | Revision 7: page-size-only conformance (#58) | Verified A4 portrait/landscape, exact Letter and fractional dimensions; captured four references and two missing-input diagnostics. Mapped existing config precedence/defaults/emission, recording precision and exit-code risks. No renderer/application changes. | Preserve separate page-size-v1 bundle and stop for review. Full T001/T003/T036/T046 remain incomplete; no generalized fixture runner or native renderer is claimed. |
| 2026-11-03 | Revision 8: margins-only conformance (#59) | Verified left/right/top/bottom order, asymmetric/fractional margins, explicit even-page override and two failure diagnostics. Measured eight page borders and repeated four renders; traced config mapping and border half-stroke inset. No renderer/application changes. | Preserve margins-v1 and stop for review. Generalized fixture automation, native implementation and full command/config contracts remain open. |
| 2026-11-03 | Revision 9: spacing-only conformance (#60) | Verified zero/fractional independent horizontal/vertical spacing with 16 frames and eight text anchors, four repeat renders and two failure cases. Documented misleading config descriptions, differing defaults and width-accounting risks. No application changes. | Preserve spacing-v1 and stop for review; generalized fixture runner and full command/config contracts remain open. |
| 2026-11-03 | Revision 10: compiled page-settings slice (#61) | Approved one-command-per-file vertical slices. Added four typed command modules, explicit shared settings/f64 mm geometry and oxidize-pdf geometry proof/example. Six native tests plus six core tests pass, including actual PDF path checks. Windows GNU cross-check passed. | Review native proof; next implement another command with reference and native comparisons together. No production integration/full TXT parser or full task completion claimed. |
