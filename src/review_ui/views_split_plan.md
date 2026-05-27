# Plan to Split `views.rs` (Stamp Album Generator 2)

Last updated: 2026-05-26
Target: `src/review_ui/views.rs`

## Goals

- Reduce the 4,000+ line `views.rs` into smaller review-ui-specific modules.
- Keep every extraction incremental, buildable, and behavior-preserving.
- Track completed work directly in this plan so the current state stays visible.
- Prefer a dedicated `src/review_ui/views/` logic directory over adding more large top-level files.
- Only proceed to the next extraction section after explicit user approval.

## Current State

- `src/review_ui/views.rs` still owns the render functions and the large inline HTML/CSS/JS templates.
- `src/review_ui/views/` now exists and already contains `tests.rs`.
- `src/review_ui/html_templates.rs` already exists as an empty placeholder, so this plan should treat it as optional rather than assume it must be created.
- The first cleanup step is done: tests were moved out of `views.rs` into `src/review_ui/views/tests.rs`.

## Status Tracker

1. Test extraction to separate file: completed.
2. Shared fragment extraction: completed.
3. Bootstrap view chunking: completed.
4. Compare view chunking: not started.
5. Shared JS/CSS snippet consolidation: not started.
6. Physical file split from `views.rs` to `views/mod.rs` plus submodules: not started.

## Revised Target Structure

Preferred end state:

```
src/review_ui/
    views.rs or views/mod.rs        # final entry point, depending on extraction timing
    views/
        tests.rs                    # characterization tests
        shared.rs                   # shared HTML/script fragments and helpers
        bootstrap.rs                # bootstrap page assembly
        compare.rs                  # compare/review page assembly
```

Optional later files only if they clearly reduce coupling:

- `src/review_ui/views/bootstrap_assets.rs`
- `src/review_ui/views/compare_assets.rs`
- `src/review_ui/html_templates.rs`

The plan should avoid prematurely scattering logic into many tiny files unless the split lines are already stable.

## Execution Order

### Phase 1: Baseline and Tracking

- Keep characterization tests in `src/review_ui/views/tests.rs` up to date.
- Add narrow tests only when they protect the next extraction step.
- Update this plan after each completed phase.

### Phase 2: Shared Fragments Inside `views.rs`

- Extract repeated output fragments first while keeping everything in one file:
- footer year script
- reconnect banner HTML
- runtime log panel HTML
- copy-alert helper fragment if it is meaningfully duplicated

Status:

- Completed on 2026-05-26.
- Extracted local helpers in `src/review_ui/views.rs` for footer year script, reconnect banner HTML, and runtime log panel HTML.
- Focused validation passed with `cargo test review_ui::views::tests --lib`.

Verification:

- `cargo test review_ui::views::tests --lib`

### Phase 3: Chunk Bootstrap Rendering

- Split `render_bootstrap_html` into local helpers with stable IDs and endpoints:
- bootstrap CSS helper
- bootstrap body helper
- bootstrap script helper

Rules:

- No endpoint string changes.
- No HTML ID changes.
- No JavaScript function rename.

Verification:

- existing focused tests
- add one narrow contract test only if a moved fragment loses coverage

Status:

+- Completed as of 2026-05-27.
+- Extracted bootstrap CSS (`bootstrap_head_css`), bootstrap body shell (`render_bootstrap_body_html`), and bootstrap script (`render_bootstrap_script_html`) in `src/review_ui/views.rs`.
+- `render_bootstrap_html` is now a thin assembler.
+- Focused validation passed with `cargo test review_ui::views::tests --lib`.

### Phase 4: Chunk Compare Rendering

- Split `render_html` into local helpers with the same constraints:
- compare CSS helper
- compare body helper
- compare script helper

Verification:

- `cargo test review_ui::views::tests --lib`
- add compare-specific output marker tests only where needed

### Phase 5: Move Helpers Into `src/review_ui/views/`

- After the helper seams are stable, move code physically into:
- `shared.rs`
- `bootstrap.rs`
- `compare.rs`

Preferred transition:

- either keep `src/review_ui/views.rs` as the entry point that `mod`s the new files
- or rename to `src/review_ui/views/mod.rs` once the module split is settled

This phase should happen after chunking, not before, to minimize churn.

### Phase 6: Optional Deeper Splits

- Only split by concern such as StampWorld, Colnect import, or panel-specific assets if those sections remain too large after the page-level split.
- Prefer concern-specific helper modules under `src/review_ui/views/` rather than adding unrelated top-level review_ui files.

## Guardrails

- Every extraction must preserve rendered output and behavior.
- Keep changes surgical; do not rename handlers, routes, or element IDs without necessity.
- Avoid unchecked `unwrap()` in runtime code paths.
- Prefer one commit-sized extraction step at a time.
- Run focused validation immediately after each substantive edit.

## Workflow

- After each approved section is moved:
- the code must build and the targeted tests must pass
- the UI output should remain functionally identical
- this plan must be updated to reflect what was completed and what remains next
