---
type: Rust Function
title: render_runtime_log_panel_html
resource: src/review_ui/views/shared.rs#L68-L95
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/views/bootstrap/render_bootstrap_html
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/render_html
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn render_runtime_log_panel_html(params: &RuntimeLogPanelParams) -> String`

# Called by

- [render_bootstrap_html](../../../../../functions/src/review_ui/views/bootstrap/render_bootstrap_html.md)
- [render_html](../../../../../functions/src/review_ui/views/compare/render_html.md)