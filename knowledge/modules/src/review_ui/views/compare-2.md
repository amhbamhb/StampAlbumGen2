---
type: Rust Module
title: compare
resource: src/review_ui/views/compare.rs#L1-L360
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/super-render-app-footer-html-render-config-editor-assets-render-footer-year-script-render-reconnect-banner-html-render-runtime-log-panel-html-matcherpayload-backend-not-responding-message-footer-year-update-js
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-compare-assets-compare-head-css-render-add-modal-html-render-compare-action-rows-html-render-compare-filters-html-render-compare-navigation-html-render-compare-panels-html-render-compare-top-html-render-op-busy-modal-html
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-review-ui-views-shared-reconnectbannerparams
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-review-ui-views-shared-runtimelogpanelparams
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [render_html](../../../../functions/src/review_ui/views/compare/render_html.md)

# Imports

- `super::{
    render_app_footer_html, render_config_editor_assets, render_footer_year_script,
    render_reconnect_banner_html, render_runtime_log_panel_html, MatcherPayload,
    BACKEND_NOT_RESPONDING_MESSAGE, FOOTER_YEAR_UPDATE_JS,
}`
- `super::compare_assets::{
    compare_head_css, render_add_modal_html, render_compare_action_rows_html,
    render_compare_filters_html, render_compare_navigation_html, render_compare_panels_html,
    render_compare_top_html, render_op_busy_modal_html,
}`
- `crate::review_ui::views::shared::ReconnectBannerParams`
- `crate::review_ui::views::shared::RuntimeLogPanelParams`

# Member of

- [stamp_album_gen2](../../../../packages/stamp_album_gen2.md)