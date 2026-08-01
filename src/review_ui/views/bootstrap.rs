use super::{
    render_app_footer_html, render_config_editor_assets, render_footer_year_script,
    render_reconnect_banner_html, render_runtime_log_panel_html, BACKEND_NOT_RESPONDING_MESSAGE,
};

use super::bootstrap_assets::{
    bootstrap_head_css, render_bootstrap_body_html, render_bootstrap_script_html,
};

pub(super) fn render_bootstrap_html(default_inbox_path: Option<&str>) -> String {
    let inbox_hint = default_inbox_path.unwrap_or("samples/colnect_inbox");
    let reconnect_banner_html = render_reconnect_banner_html(
        "reconnectBanner",
        "reconnectBannerTitle",
        Some("reconnectBannerText"),
        Some(BACKEND_NOT_RESPONDING_MESSAGE),
        "",
        "retryBackendHealthCheck",
        "reloadCurrentPage",
        "reconnectBannerMeta",
    );
    let runtime_log_panel_html = render_runtime_log_panel_html(
        "runtimeLogPanel",
        "margin-top:10px;border:1px solid #d7e2eb;border-radius:10px;padding:10px;background:#fbfdff;",
        "display:flex;gap:12px;align-items:center;justify-content:space-between;flex-wrap:wrap;margin-top:0;",
        "display:flex;gap:8px;align-items:center;flex-wrap:wrap;margin-top:0;",
        "runtimeLogBody",
        "runtimeLog",
        "showRuntimeLog",
        "toggleRuntimeLogVisibility",
        "copyRuntimeLog",
    );
    let bootstrap_body_html =
        render_bootstrap_body_html(inbox_hint, &reconnect_banner_html, &runtime_log_panel_html);
    let bootstrap_script_html = render_bootstrap_script_html();

    format!(
        r#"<!doctype html>
<html>
<head>
<meta charset='utf-8'>
<title>Stamp Album Generator 2 Setup</title>
<link rel='icon' type='image/png' href='/favicon.png'>
<style>
{bootstrap_css}
</style>
</head>
<body>
{bootstrap_body_html}
{bootstrap_script_html}
{footer_html}
{footer_year_script}
{config_editor_assets}
</body>
</html>"#,
        bootstrap_css = bootstrap_head_css(),
        bootstrap_body_html = bootstrap_body_html,
        bootstrap_script_html = bootstrap_script_html,
        footer_html = render_app_footer_html(),
        footer_year_script = render_footer_year_script(),
        config_editor_assets = render_config_editor_assets()
    )
}
