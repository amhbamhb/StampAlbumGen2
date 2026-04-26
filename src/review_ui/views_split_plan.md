# Plan to Split `views.rs` (Stamp Album Generator 2)

## Goals

- Improve maintainability and readability by splitting the 4,000+ line `views.rs` into logical modules.
- Each split is incremental: after each step, the app must build, run, and produce identical output.
- Only proceed to the next section after explicit user approval.

## Proposed Sections

1. **HTML Template Rendering**
   - Move all functions that generate static or mostly-static HTML (e.g., `render_app_footer_html`, `render_bootstrap_html`) to a new `html_templates.rs`.

2. **JavaScript/CSS Embedding**
   - Move embedded JS/CSS string generation to a new `assets.rs` or `embedded_scripts.rs`.

3. **Country/Album Panel Logic**
   - Move functions related to rendering and updating the country/album panels to `country_panel.rs` and/or `album_panel.rs`.

4. **StampWorld Panel Logic**
   - Move all StampWorld-specific UI logic to `stampworld_panel.rs`.

5. **Colnect/Import Panel Logic**
   - Move Colnect import UI and logic to `colnect_panel.rs`.

6. **Utility Functions**
   - Move shared helpers (e.g., string escaping, formatting) to a new `ui_utils.rs`.

7. **Tests**
   - Move or create tests for each module in a `tests/` subdirectory or inline as appropriate.

## File Structure Example

```
src/review_ui/
    views.rs                # Main entry point, re-exports, glue code
    html_templates.rs       # Static HTML rendering
    embedded_scripts.rs     # JS/CSS string generation
    country_panel.rs        # Country/album panel logic
    stampworld_panel.rs     # StampWorld panel logic
    colnect_panel.rs        # Colnect import logic
    ui_utils.rs             # Shared helpers/utilities
    tests/
        ...                # (optional) module-specific tests
```

## Workflow

- After each section is moved:
  - The code must build and run with no errors.
  - The UI output must be visually and functionally identical.
  - Only proceed to the next section after user review and approval.
