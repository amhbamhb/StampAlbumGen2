# Native PDF generation with AlbumEasy TXT export

**Created:** 2026-11-01  
**Last updated:** 2026-11-01  
**Status:** Draft plan; implementation not started  
**Scope:** Combine the shared-layout approach (option 1), optional native backend rollout (option 2), and AlbumEasy retention for imported albums (option 4).

## 1. Goal and recommended direction

Generate printable stamp-album PDFs directly in Rust using `oxidize-pdf`, while
continuing to export editable, AlbumEasy-compatible TXT files.

Use **one layout calculation and two exporters**, not two independent pagination
engines:

```text
Country database + effective configuration + assets
                       |
               Shared layout builder
                       |
         Album / pages / rows / placements
                /                  \
       Native PDF renderer     AlbumEasy TXT exporter
                \                  /
          Generation result and diagnostics

Imported AlbumEasy album database
                       |
           Existing saved-command export
                       |
               AlbumEasy renderer
```

The TXT file is a companion output, not something reconstructed from the PDF.
Generate/export both from the same layout during one operation. Retain explicit
TXT-only generation when the user does not request a PDF.

**Do not replace AlbumEasy immediately.** Initially keep AlbumEasy as the default.
Offer native PDF as an opt-in backend for ordinary generated albums. Change the
default only after compatibility and print-fidelity gates pass and the maintainerz
approves the change.

## 2. Verified starting point

| Area | Current implementation | Consequence for this plan |
|------|------------------------|---------------------------|
| PDF library | `oxidize-pdf` is included in [Cargo.toml](../../Cargo.toml); [PDF extraction](../../src/review_ui/pdf_parser/extraction.rs) uses its image operations. | Inclusion/image extraction does not prove that generation, fonts and layout meet our needs; verify these independently. |
| Output orchestration | [album_globals/mod.rs](../../src/album_globals/mod.rs) creates TXT and invokes AlbumEasy to obtain PDF. | This is the main dispatch/result integration point. |
| Ordinary album generation | [generator.rs](../../src/album_globals/generator.rs) remains the active implementation; the entry point in [generator_new/mod.rs](../../src/album_globals/generator_new/mod.rs) is still a stub/fallback. | Do not assume the refactored entry point already provides a complete layout model. Extract behavior from the active implementation. |
| Existing layout helpers | [generator_new/](../../src/album_globals/generator_new/) contains layout, emission, commemorative-layout, helper and postprocessing modules. | Reuse suitable helpers; do not create a competing second layout subsystem. |
| Imported albums | [external_album.rs](../../src/external_album.rs) regenerates from database-backed commands and has its own rendering/approval lifecycle. | Preserve that path, its dedicated editor, source references and fidelity approval. |
| Settings | [config.rs](../../src/config.rs) supports AlbumEasy executable, automatic PDF generation and diagnostics. | Preserve existing configuration semantics and introduce backend selection additively. |

This is an implementation plan, not a claim that the library's rendering
capabilities or output fidelity have been certified.

## 3. Requirements and boundaries

| ID | Requirement |
|----|-------------|
| REQ-001 | Calculate ordinary album ordering, pagination and placement once in a shared layout. |
| REQ-002 | Render ordinary albums to PDF directly with native Rust code. |
| REQ-003 | Export valid AlbumEasy TXT from the same layout; preserve TXT-only generation. |
| REQ-004 | Add explicit backend selection without changing existing defaults during rollout. |
| REQ-005 | Keep imported AlbumEasy albums on their existing command/database and AlbumEasy rendering path. |
| REQ-006 | Preserve physical dimensions, layout behavior, text, images and pagination within approved tolerances. |
| REQ-007 | Surface unsupported features and output failures; never silently change backend or return a stale PDF as success. |
| REQ-008 | Keep CLI, startup UI, country review UI, configuration and diagnostics consistent. |
| REQ-009 | Maintain Windows/Linux portability and validate both supported code paths. |

### Initial exclusions

- No general-purpose AlbumEasy interpreter.
- No conversion of arbitrary imported AlbumEasy commands to native primitives.
- No changes to StampWorld/Colnect import semantics.
- No unrelated generator cleanup, database redesign or layout-editor rewrite.
- No OCR, PDF-to-TXT reconstruction, or new cloud service.
- No promise of byte-identical PDFs; compare observable layout and print output.

### Proposed backend policy — approve before coding

Use a typed setting such as `pdf_backend = "albumeasy" | "native"`.
The exact key/CLI naming must follow the project's current configuration patterns.

| Album/action | AlbumEasy selected | Native selected |
|--------------|-------------------|-----------------|
| Ordinary album, PDF requested | Existing TXT + AlbumEasy PDF | Shared-layout TXT + native PDF |
| Ordinary album, TXT only | TXT; no PDF | TXT; no PDF |
| Imported AlbumEasy album | Existing saved-layout pipeline | Explain that native rendering does not yet support imported albums; offer the existing AlbumEasy path explicitly. |
| Native capability/render failure | Not applicable | Return a visible error. Do not automatically invoke AlbumEasy. |

Backend choice and PDF enablement are separate settings. Keep existing
`albumeasy_auto_generate_pdf` behavior until a reviewed compatibility migration is
defined; changing a backend must not unexpectedly turn PDF generation on.
An imported-album exception must be visible in the UI/result, not a silent fallback.
Existing import validation may still require AlbumEasy even when ordinary albums
use native PDF.

## 4. Work breakdown and implementation sequence

All tasks start unchecked. IDs remain stable as the document is updated.
Suggested new files below are proposals, not an instruction to add unnecessary
abstractions. Reuse existing modules wherever they already own the behavior.

### Phase 1 — inventory, baseline and capability proof

**Plan P1.1:** Freeze observable behavior before altering the generator.  
**Plan P1.2:** Prove that `oxidize-pdf` can render the minimum required album page.

- [ ] T001 [Plan:1.1] Inventory emitted AlbumEasy commands and current layout/configuration behavior in `src/album_globals/generator.rs`, `generator_new/emit.rs`, `generator_new/layout.rs`, `generator_new/commemorative_layout.rs`, and `generator_new/postprocess.rs`.
- [ ] T002 [Plan:1.1] Inventory all ordinary/filtered generation callers in `src/album_globals/mod.rs`, `src/modes.rs`, `src/review_ui/http_handlers.rs`, and generation helpers. Record imported-album dispatch separately from ordinary albums.
- [ ] T003 [Plan:1.1] Capture deterministic baseline TXT and fresh AlbumEasy PDFs for representative ordinary albums; retain existing imported-album fixtures as regression references. Put durable evidence in `docs/testing/`; never mutate production albums.
- [ ] T004 [Plan:1.2] Verify the installed `oxidize-pdf` generation API and license, font embedding/measurement, Unicode, images/transparency, page sizes, metadata and write errors. Record supported/unsupported features and exact limitations in this document.
- [ ] T005 [Plan:1.2] Build an isolated proof page with known millimetre dimensions, a stamp image, caption, heading and border. Verify PDF dimensions, text extraction and a 100%-scale printed ruler.

**Exit gate:** Command/feature inventory exists; native proof page opens and prints
correctly; required missing capabilities are identified before the larger refactor.
Stop for a maintainer decision if the library cannot meet a core requirement.

### Phase 2 — shared layout and font metrics

**Plan P2.1:** Extract a minimal backend-neutral representation.  
**Plan P2.2:** Move layout decisions out of TXT emission without changing behavior.

- [ ] T006 [Plan:2.1] Define the minimal shared layout in the existing generation area, or a focused `src/album_globals/album_layout.rs` if needed: album/page settings, rows, placements, text styles, borders and resolved asset references.
- [ ] T007 [Plan:2.1] Define canonical units (prefer millimetres for album layout), conversion to PDF points (`mm * 72 / 25.4`), coordinate origin, floating-point rounding, page/margin bounds and physical-versus-presentation box dimensions.
- [ ] T008 [Plan:2.2] Establish a shared font measurement/wrapping policy. Map current AlbumEasy fonts explicitly to licensed, available fonts; specify embedding and substitution behavior. Never silently change fonts.
- [ ] T009 [Plan:2.2] Extract ordering, stamp selection, row packing, spacing modes, caption heights, page breaks, even-page margins and image-box sizing from the active generator into the shared builder, reusing existing helpers.
- [ ] T010 [Plan:2.2] Ensure the new path returns an actual layout/result rather than relying on `generator_new/mod.rs`'s empty-path fallback. Preserve callbacks, progress, filter behavior and existing output naming.

**Exit gate:** Shared layouts reproduce baseline order, page count and dimensions;
TXT behavior is still preserved. No backend-specific pagination remains hidden in
the native renderer.

### Phase 3 — AlbumEasy TXT exporter

**Plan P3.1:** Export ordinary albums from the shared layout.

- [ ] T011 [Plan:3.1] Adapt `generator_new/emit.rs` and related emit/postprocess helpers to consume the shared layout and emit the existing AlbumEasy command subset.
- [ ] T012 [Plan:3.1] Preserve escaping, relative image paths, command order, row spacing semantics, page breaks, fonts, numbering and all supported global settings. Keep asset paths portable.
- [ ] T013 [Plan:3.1] Validate emitted TXT with AlbumEasy itself and compare resulting PDFs against the frozen baseline. Where TXT semantics cannot express a native placement exactly, document the limitation and gate the feature rather than claiming equivalence.

**Exit gate:** Exported TXT opens/renders in AlbumEasy. Preserve byte-normalized
baseline TXT where feasible; review intentional differences and prove their effect.
The TXT is useful for editing, not merely syntactically valid.

### Phase 4 — native PDF renderer

**Plan P4.1:** Render the shared layout using `oxidize-pdf`.  
**Plan P4.2:** Produce reliable paired outputs with explicit diagnostics.

- [ ] T014 [Plan:4.1] Implement a focused renderer, for example `src/album_globals/native_pdf.rs`, consuming resolved pages/placements without repeating selection or pagination.
- [ ] T015 [Plan:4.1] Render page sizes, margins, borders, headings, page numbers, stamp/image boxes, aspect-ratio-preserving images, captions, alignment and supported colors. Reject unsupported commands/styles explicitly.
- [ ] T016 [Plan:4.1] Implement font embedding and glyph validation using the Phase 2 policy; verify accented country names, punctuation, denomination fractions and currency symbols.
- [ ] T017 [Plan:4.2] Stage and validate outputs before publishing final files. Preserve previous files on failure where practical; distinguish TXT success/PDF failure and report cleanup/publish failures. Do not promise multi-file atomicity across a crash.
- [ ] T018 [Plan:4.2] Extend `GeneratedAlbumArtifacts`/generation diagnostics in `src/album_globals/mod.rs` as needed to report selected/actual backend, fresh output paths, errors and supported warnings. Avoid success-shaped empty paths.

**Exit gate:** Native rendering produces fresh, valid PDFs with correct page
geometry; failure paths cannot masquerade as success or overwrite a valid prior
artifact with a partial file.

### Phase 5 — optional backend rollout and imported-album protection

**Plan P5.1:** Wire configuration, CLI and UI consistently.  
**Plan P5.2:** Preserve imported albums on AlbumEasy.

- [ ] T019 [Plan:5.1] Add typed backend selection to `src/config.rs`, `config_template.toml`, `config_full_example.toml`, and `docs/guides/CONFIG_PARAMETERS_INVENTORY.md`; preserve missing-key defaults and album-local overrides.
- [ ] T020 [Plan:5.1] Wire a CLI backend override through `src/cli.rs`, `src/main.rs`, `src/modes.rs`, and shared orchestration. Regenerate `docs/CLI_USAGE.md` and update `docs/guides/CLI_OPTIONS_INVENTORY.md`.
- [ ] T021 [Plan:5.1] Update startup/country-review controls in `src/review_ui/views/bootstrap.html`, `bootstrap.js`, `compare.html`, `compare.js`, and `http_handlers.rs`; check configuration editor behavior and all Generate PDF actions.
- [ ] T022 [Plan:5.2] Keep `src/external_album.rs` saved-command export/render/approval/revision lifecycle unchanged. In its dedicated UI (`src/review_ui/views/external_album.html`), explain the AlbumEasy requirement and handle native requests without implicit fallback.
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
| 5. Backend controls and imported-album protection | Medium | 3–5 days |
| 6. Parity, cross-platform validation and rollout | High | 5–10 days |
| **Total before contingency** | | **24–47 developer-days** |

Allow roughly 20–30% contingency for font/layout fidelity and generator coupling:
approximately **29–61 developer-days overall**. Arbitrary native rendering of
imported AlbumEasy documents is not included.

**Dependencies:** Phase 1 precedes Phase 2; Phase 2 precedes Phases 3/4. Native
and TXT exporters can then be developed independently against a frozen layout
contract. Phase 5 needs both exporters and stable generation results. Phase 6
requires the integrated paths; start collecting evidence throughout.

## 6. Validation strategy and proposed acceptance criteria

Use existing Cargo/Node test infrastructure; no new test framework by default.
Add runnable regression coverage as part of implementation, not after rollout.
Use isolated temporary albums/databases and preserve all user inputs.

### Representative coverage

- Small ordinary album and multi-page ordinary album.
- Dense captions, long series titles and wrapping at a page boundary.
- Mixed stamp sizes, wide/tall stamps, fixed-spacing rows and commemorative layouts.
- Alternate/even-page margins, headings, borders and page numbering.
- Images with transparency, missing assets, non-ASCII filenames and path spaces.
- Unicode text, fractions/currency symbols and unavailable font glyphs.
- Empty selection, year/catalogue filters and TXT-only requests.
- Existing imported albums, repeated placements and page-local saved edits.

### Proposed thresholds — approve in Phase 1

| Check | Proposed acceptance |
|-------|---------------------|
| Page count and stamp selection/order | Exact match for certified fixtures. |
| Paper size and physical stamp/box dimensions | Within 0.1 mm of specified geometry. |
| Placement and margin/border geometry | Within 0.25 mm of the approved baseline. |
| Text wrapping | No clipping, overlap or omitted glyphs; line-break differences require explicit review. |
| Export compatibility | Companion TXT successfully renders in the supported AlbumEasy version with no command errors. |
| Visual comparison | Review at a fixed rendering scale/DPI; approve intentional differences. Do not rely solely on a pixel score. |
| Printed output | Ruler/box check at 100% scale with printer fit-to-page disabled. |
| Error behavior | Failed rendering/publishing gives explicit failure; no stale/partial PDF reported as newly generated. |
| Windows/Linux | Relevant tests and build/type-check pass on both; disclose any unverified runtime/print checks. |

Run the smallest relevant checks per phase, then the relevant regression suites
before release. Record commands, exit codes, pass/fail counts, temporary file/DB
impacts and discovered logic risks in `docs/testing/`.

## 7. Main risks and decisions still open

| Risk/decision | Required action |
|---------------|-----------------|
| Font metrics differ from AlbumEasy, changing pagination. | Prove shared wrapping/measurement and font mapping before certifying the layout. |
| TXT row commands cause AlbumEasy to calculate positions differently. | Compare rendered companion TXT, not just command strings; identify representability gaps. |
| Active generator mixes selection, placement and emission. | Extract incrementally with baseline parity; avoid completing an unrelated full refactor. |
| Imported command model differs from ordinary layout. | Keep its existing pipeline; do not force it through the new model. |
| Library image support does not meet print quality or format needs. | Verify formats/resolution/transparency; reuse existing image helpers where suitable. |
| A saved/global backend preference accidentally affects import validation. | Make the imported-album exception explicit and test every imported render/validate/edit action. |
| Partial paired-file publication. | Define accurate partial-success reporting and staging/cleanup; never claim impossible cross-file atomicity. |
| Missing fonts, assets or external AlbumEasy executable. | Return actionable diagnostics; no silent substitutions or automatic fallback. |

**Approval record**

- Backend setting/override naming: **Pending**
- Initial font family mapping and embedding license: **Pending**
- Required initial command/style subset: **Pending Phase 1 inventory**
- Geometry/visual thresholds: **Proposed; pending approval**
- Native-default promotion: **Not approved; AlbumEasy remains default**

## 8. Requirement mapping

| Requirement | Plan items | Completion evidence |
|-------------|------------|---------------------|
| REQ-001 | P2.1, P2.2 | Shared layout builder; parity checks; T006–T010 |
| REQ-002 | P1.2, P4.1 | Native proof and renderer; T004–T005, T014–T016 |
| REQ-003 | P3.1, P4.2 | Companion TXT and paired results; T011–T013, T017–T018 |
| REQ-004 | P5.1, P6.2 | Typed backend settings and reviewed default; T019–T021, T029 |
| REQ-005 | P5.2 | Imported saved-layout lifecycle retained; T022–T023, T025 |
| REQ-006 | P2.2, P3.1, P4.1, P6.1 | Geometry, text and print parity evidence; T008–T016, T024–T026 |
| REQ-007 | P4.2, P5.2, P6.1 | Failure diagnostics, output staging and unsupported-feature behavior; T017–T018, T022–T026 |
| REQ-008 | P5.1, P5.2 | CLI/UI/help/config consistency; T019–T023, T025 |
| REQ-009 | P6.1 | Windows/Linux build, runtime and print evidence; T024–T027 |

## 9. How to keep this plan up to date

1. Update **Last updated**, phase status and task checkboxes after each work batch.
2. Keep task IDs stable; add new tasks rather than renumbering completed ones.
3. Mark a task complete only when its outcome and applicable checks are recorded.
4. Record deviations, blockers and changed estimates before starting affected work.
5. Link evidence, commits or local branch names in the progress log.
6. Do not mark native as default until T029 approval is recorded.

| Phase | Status | Owner | Evidence / blocker |
|-------|--------|-------|--------------------|
| 1 | Not started | Unassigned | |
| 2 | Not started | Unassigned | Depends on Phase 1 |
| 3 | Not started | Unassigned | Depends on shared layout |
| 4 | Not started | Unassigned | Depends on shared layout/library proof |
| 5 | Not started | Unassigned | Depends on both exporters |
| 6 | Not started | Unassigned | Depends on integrated generation paths |

### Progress and decision log

| Date | Tasks / decision | Outcome / evidence | Next action |
|------|------------------|--------------------|-------------|
| 2026-11-01 | Plan created for options 1, 2 and 4 | Repository generation/import paths inspected; no rendering implementation changed. | Approve boundaries and start Phase 1. |
