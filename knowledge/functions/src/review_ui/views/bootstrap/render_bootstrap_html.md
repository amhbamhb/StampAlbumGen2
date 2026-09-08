---
type: Rust Function
title: render_bootstrap_html
resource: src/review_ui/views/bootstrap.rs#L12-L65
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/shared/render_reconnect_banner_html
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/shared/render_runtime_log_panel_html
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap_assets/render_bootstrap_body_html
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap_assets/render_bootstrap_script_html
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn render_bootstrap_html(default_inbox_path: Option<&str>) -> String`

# Calls

- [render_reconnect_banner_html](../../../../../functions/src/review_ui/views/shared/render_reconnect_banner_html.md)
- [render_runtime_log_panel_html](../../../../../functions/src/review_ui/views/shared/render_runtime_log_panel_html.md)
- [render_bootstrap_body_html](../../../../../functions/src/review_ui/views/bootstrap_assets/render_bootstrap_body_html.md)
- [render_bootstrap_script_html](../../../../../functions/src/review_ui/views/bootstrap_assets/render_bootstrap_script_html.md)