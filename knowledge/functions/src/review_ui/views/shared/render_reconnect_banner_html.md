---
type: Rust Function
title: render_reconnect_banner_html
resource: src/review_ui/views/shared.rs#L29-L53
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

`pub(super) fn render_reconnect_banner_html(params: &ReconnectBannerParams) -> String`

# Called by

- [render_bootstrap_html](../../../../../functions/src/review_ui/views/bootstrap/render_bootstrap_html.md)
- [render_html](../../../../../functions/src/review_ui/views/compare/render_html.md)