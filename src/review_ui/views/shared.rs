pub(super) const FOOTER_YEAR_UPDATE_JS: &str = "document.querySelectorAll('.copyrightCurrentYear').forEach((el) => {\n    el.textContent = String(new Date().getFullYear());\n});";

pub(super) fn render_app_footer_html() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!(
        "<div class='appFooter' style='margin-top:16px; font-size:12px; color:#5b6f81; text-align:center;'>\
<div>Copyright &copy; 2025 &#x2013; <span class='copyrightCurrentYear'></span> Laser Spider LLC. All rights reserved.</div>\
<div style='margin-top:4px;'>Stamp Album Generator 2 | Version {} </div>\
</div>",
        version
    )
}

pub(super) fn render_footer_year_script() -> String {
    format!("<script>\n{}\n</script>", FOOTER_YEAR_UPDATE_JS)
}
/// Params for `render_reconnect_banner_html`
pub struct ReconnectBannerParams<'a> {
    pub banner_id: &'a str,
    pub title_id: &'a str,
    pub text_id: Option<&'a str>,
    pub text_html: Option<&'a str>,
    pub extra_html: &'a str,
    pub retry_fn: &'a str,
    pub reload_fn: &'a str,
    pub meta_id: &'a str,
}

pub(super) fn render_reconnect_banner_html(params: &ReconnectBannerParams) -> String {
    let text_section = match (params.text_id, params.text_html) {
        (Some(text_id), Some(text_html)) => {
            format!("\n        <div id='{text_id}'>{text_html}</div>")
        }
        _ => String::new(),
    };
    format!(
        r#"<div id='{banner_id}' class='reconnectBanner' role='status' aria-live='polite'>
        <div id='{title_id}' class='title'>Connection lost, retrying...</div>{text_section}{extra_html}
        <div class='actions'>
            <button type='button' onclick='{retry_fn}()'>Retry Now</button>
            <button type='button' onclick='{reload_fn}()'>Reload Page</button>
            <span id='{meta_id}' class='meta'></span>
        </div>
    </div>"#,
        banner_id = params.banner_id,
        title_id = params.title_id,
        text_section = text_section,
        extra_html = params.extra_html,
        retry_fn = params.retry_fn,
        reload_fn = params.reload_fn,
        meta_id = params.meta_id,
    )
}

/// Params for `render_runtime_log_panel_html`
pub struct RuntimeLogPanelParams<'a> {
    pub panel_id: &'a str,
    pub panel_style: &'a str,
    pub header_style: &'a str,
    pub controls_style: &'a str,
    pub body_id: &'a str,
    pub textarea_id: &'a str,
    pub checkbox_id: &'a str,
    pub toggle_fn: &'a str,
    pub copy_fn: &'a str,
}

pub(super) fn render_runtime_log_panel_html(params: &RuntimeLogPanelParams) -> String {
    format!(
        r#"<div id='{panel_id}' style='{panel_style}'>
        <div style='{header_style}'>
            <strong>Runtime Log</strong>
            <div style='{controls_style}'>
                <label style='font-size:12px;font-weight:600;display:flex;align-items:center;gap:6px;'>
                    <input id='{checkbox_id}' type='checkbox' checked onchange='{toggle_fn}()' />
                    Show Log
                </label>
                <button onclick='{copy_fn}()'>Copy Log</button>
            </div>
        </div>
        <div id='{body_id}'>
            <textarea id='{textarea_id}' readonly style='width:100%;min-height:120px;max-height:220px;resize:vertical;padding:8px;border:1px solid #c8d5e2;border-radius:8px;font-family:monospace;font-size:12px;'></textarea>
        </div>
    </div>"#,
        panel_id = params.panel_id,
        panel_style = params.panel_style,
        header_style = params.header_style,
        controls_style = params.controls_style,
        body_id = params.body_id,
        textarea_id = params.textarea_id,
        checkbox_id = params.checkbox_id,
        toggle_fn = params.toggle_fn,
        copy_fn = params.copy_fn,
    )
}
