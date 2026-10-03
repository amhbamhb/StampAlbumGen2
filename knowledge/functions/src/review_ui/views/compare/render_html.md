---
type: Rust Function
title: render_html
resource: src/review_ui/views/compare.rs#L16-L360
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/compare_assets/compare_head_css
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/shared/render_reconnect_banner_html
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/shared/render_runtime_log_panel_html
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare_assets/render_compare_top_html
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare_assets/render_compare_navigation_html
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare_assets/render_compare_filters_html
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare_assets/render_compare_action_rows_html
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare_assets/render_compare_panels_html
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare_assets/render_add_modal_html
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare_assets/render_op_busy_modal_html
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/shared/render_app_footer_html
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/shared/render_footer_year_script
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/render_config_editor_assets
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn render_html(payload: &MatcherPayload) -> String`

# Calls

- [compare_head_css](../../../../../functions/src/review_ui/views/compare_assets/compare_head_css.md)
- [render_reconnect_banner_html](../../../../../functions/src/review_ui/views/shared/render_reconnect_banner_html.md)
- [render_runtime_log_panel_html](../../../../../functions/src/review_ui/views/shared/render_runtime_log_panel_html.md)
- [render_compare_top_html](../../../../../functions/src/review_ui/views/compare_assets/render_compare_top_html.md)
- [render_compare_navigation_html](../../../../../functions/src/review_ui/views/compare_assets/render_compare_navigation_html.md)
- [render_compare_filters_html](../../../../../functions/src/review_ui/views/compare_assets/render_compare_filters_html.md)
- [render_compare_action_rows_html](../../../../../functions/src/review_ui/views/compare_assets/render_compare_action_rows_html.md)
- [render_compare_panels_html](../../../../../functions/src/review_ui/views/compare_assets/render_compare_panels_html.md)
- [render_add_modal_html](../../../../../functions/src/review_ui/views/compare_assets/render_add_modal_html.md)
- [render_op_busy_modal_html](../../../../../functions/src/review_ui/views/compare_assets/render_op_busy_modal_html.md)
- [render_app_footer_html](../../../../../functions/src/review_ui/views/shared/render_app_footer_html.md)
- [render_footer_year_script](../../../../../functions/src/review_ui/views/shared/render_footer_year_script.md)
- [render_config_editor_assets](../../../../../functions/src/review_ui/config_editor/render_config_editor_assets.md)