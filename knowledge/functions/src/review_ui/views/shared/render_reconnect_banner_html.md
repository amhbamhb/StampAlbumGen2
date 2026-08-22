---
type: Rust Function
title: render_reconnect_banner_html
resource: src/review_ui/views/shared.rs#L19-L53
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

`pub(super) fn render_reconnect_banner_html( banner_id: &str, title_id: &str, text_id: Option<&str>, text_html: Option<&str>, extra_html: &str, retry_fn: &str, reload_fn: &str, meta_id: &str, ) -> String`

# Called by

- [render_bootstrap_html](../../../../../functions/src/review_ui/views/bootstrap/render_bootstrap_html.md)
- [render_html](../../../../../functions/src/review_ui/views/compare/render_html.md)