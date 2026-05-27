# Review UI `views.rs` Refactor Plan

Date: 2026-04-09
Target file: `src/review_ui/views.rs` (4360+ lines)

## Goals

- Split large HTML builders into manageable logic chunks.
- Reduce duplication across bootstrap and compare views.
- Make behavior easier to test with focused unit tests.
- Preserve exact runtime behavior while refactoring.

## Current Shape (Key Facts)

- Rust functions: `render_app_footer_html`, `render_bootstrap_html`, `render_html`.
- The size comes from two huge HTML+CSS+JS template strings.
- Most complexity is in inline JavaScript logic (state, polling, UI updates).

## Refactor Strategy (Low-Risk Sequence)

### Phase 1: Safety Baseline

- Keep current behavior and add characterization tests for rendered output markers.
- Do not move logic yet.
- Completed: tests were moved out of `src/review_ui/views.rs` into `src/review_ui/views/tests.rs`.

### Phase 2: Extract Shared Constants and Fragments

- Move static repeated fragments to helpers/constants:
- Shared reconnect banner HTML.
- Shared runtime log panel HTML.
- Shared copy-alert dialog script helper text.
- Shared footer year script (currently duplicated in both pages).

Proposed private helpers in `views.rs`:

- `fn render_footer_year_script() -> &'static str`
- `fn render_reconnect_banner_html(id_suffix: &str, message: &str) -> String`
- `fn render_runtime_log_panel_html(id_suffix: &str) -> String`

### Phase 3: Split Bootstrap View Into Chunks

Refactor `render_bootstrap_html` into composable functions:

- `fn bootstrap_head_css() -> &'static str`
- `fn bootstrap_body_shell(inbox_hint: &str) -> String`
- `fn bootstrap_script_body() -> &'static str`

Rules:

- Keep endpoint strings unchanged.
- Keep element IDs unchanged.
- Keep JS function names unchanged.

### Phase 4: Split Compare View Into Chunks

Refactor `render_html` similarly:

- `fn compare_head_css() -> &'static str`
- `fn compare_body_shell() -> &'static str`
- `fn compare_script_body() -> &'static str`

Optional refinement after chunking:

- Extract JS snippets by concern (navigation, polling, assignment actions, left-row edit dialog, colnect import, pdf controls).

### Phase 5: Reduce JS Duplication

Create shared mini-JS snippets used by both pages:

- Backend health polling and reconnect banner behavior.
- Runtime log polling/copy behavior.
- Clipboard fallback utility.
- Copyright year updater.

Approach:

- Keep them as `const &str` snippets and inject into both pages.
- Avoid introducing external JS files in the first pass.

### Phase 6: Optional File Split

After chunking stabilizes, split file physically into the existing `src/review_ui/views/` directory:

- `src/review_ui/views/mod.rs` or keep `src/review_ui/views.rs` as entry point during transition
- `src/review_ui/views/bootstrap.rs`
- `src/review_ui/views/compare.rs`
- `src/review_ui/views/shared.rs`

This should be done only after tests are stable to minimize merge risk.

## Duplication and Simplification Opportunities

- Duplicate footer year `<script>` block appears more than once per page output.
- Similar reconnect banner structures and health polling flows exist in both pages.
- Similar runtime log sections and copy-to-clipboard utilities exist in both pages.
- Repeated status tone handling (`ok`/`fail`) can be normalized with one helper pattern.
- Inline style blocks are long; shared tokens can be moved to partial helpers.

## Test Plan

## Existing tests added now

In `src/review_ui/views/tests.rs`:

- Bootstrap default inbox hint rendering.
- Bootstrap custom inbox hint rendering.
- Bootstrap health/runtime log hooks present.
- Compare page embeds payload and key panel shell IDs.
- Compare page includes reconnect/retry elements.

## Next tests to add during extraction

- Shared fragment tests:
- Reconnect banner IDs and message interpolation.
- Runtime log panel IDs and checkbox hook presence.

- Contract tests for endpoint and action stability:
- Verify output still contains critical endpoints: `/health`, `/runtime-log`, bootstrap set/pick routes.
- Verify key compare action handlers remain present (`applyMatches`, `clearMatch`, `saveLeftStamp`, etc.).

- Regression tests for payload serialization contract:
- Empty payload.
- Non-empty payload with representative fields.

## Guardrails During Refactor

- Do not use `unwrap()` in app/runtime code paths. Errors must be propagated or handled with explicit fallback behavior.
- Ensure failures are communicated clearly:
- Return structured errors/messages to the app/UI where applicable.
- Write actionable error details to runtime logs so users can understand what failed and why.
- Prefer `?`, `ok_or_else`, `unwrap_or_else`, or explicit `match` with meaningful context instead of panicking.
- No ID/function renames in HTML/JS without coordinated handler updates.
- Make one extraction step per commit-sized unit.
- Run targeted tests after each phase.

## Suggested Execution Order

1. Extract tiny shared script/footer helpers.
2. Extract bootstrap shell/CSS/script helpers.
3. Extract compare shell/CSS/script helpers.
4. Consolidate duplicate JS snippets.
5. Optionally split files into submodules.
6. Expand tests for shared helpers and contract coverage.
