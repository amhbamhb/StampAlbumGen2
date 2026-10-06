---
type: Rust Function
title: render_bootstrap_body_html
resource: src/review_ui/views/bootstrap_assets.rs#L29-L44
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/views/bootstrap/render_bootstrap_html
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn render_bootstrap_body_html( inbox_hint: &str, reconnect_banner_html: &str, runtime_log_panel_html: &str, ) -> String`

# Called by

- [render_bootstrap_html](../../../../../functions/src/review_ui/views/bootstrap/render_bootstrap_html.md)