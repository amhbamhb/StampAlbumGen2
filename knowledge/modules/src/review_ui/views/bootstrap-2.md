---
type: Rust Module
title: bootstrap
resource: src/review_ui/views/bootstrap.rs#L1-L65
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/super-render-app-footer-html-render-config-editor-assets-render-footer-year-script-render-reconnect-banner-html-render-runtime-log-panel-html-backend-not-responding-message
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-bootstrap-assets-bootstrap-head-css-render-bootstrap-body-html-render-bootstrap-script-html
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-shared-reconnectbannerparams-runtimelogpanelparams
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [render_bootstrap_html](../../../../functions/src/review_ui/views/bootstrap/render_bootstrap_html.md)

# Imports

- `super::{
    render_app_footer_html, render_config_editor_assets, render_footer_year_script,
    render_reconnect_banner_html, render_runtime_log_panel_html, BACKEND_NOT_RESPONDING_MESSAGE,
}`
- `super::bootstrap_assets::{
    bootstrap_head_css, render_bootstrap_body_html, render_bootstrap_script_html,
}`
- `super::shared::{ReconnectBannerParams, RuntimeLogPanelParams}`

# Member of

- [stamp_album_gen2](../../../../packages/stamp_album_gen2.md)