use super::{BACKEND_NOT_RESPONDING_MESSAGE, FOOTER_YEAR_UPDATE_JS};

pub(super) fn render_bootstrap_script_html() -> String {
    let script_template = format!(
        "{}\n{}\n{}\n{}",
        include_str!("country_url.js"),
        include_str!("stampworld_start_error.js"),
        include_str!("bootstrap.js"),
        include_str!("external_album.js")
    );
    let script_content = script_template
        .replace(
            "{backend_not_responding_message}",
            BACKEND_NOT_RESPONDING_MESSAGE,
        )
        .replace("{footer_year_update_js}", FOOTER_YEAR_UPDATE_JS);
    format!("<script>\n{}\n</script>", script_content)
}

pub(super) fn bootstrap_head_css() -> String {
    format!(
        "{}\n{}\n{}",
        include_str!("bootstrap.css"),
        include_str!("country_url.css"),
        include_str!("external_album.css")
    )
}

pub(super) fn render_bootstrap_body_html(
    inbox_hint: &str,
    reconnect_banner_html: &str,
    runtime_log_panel_html: &str,
) -> String {
    let html_template = include_str!("bootstrap.html");
    html_template
        .replace(
            "{external_album_html}",
            include_str!("external_album.html"),
        )
        .replace("{country_url_controls}", include_str!("country_url.html"))
        .replace("{inbox_hint}", inbox_hint)
        .replace("{reconnect_banner_html}", reconnect_banner_html)
        .replace("{runtime_log_panel_html}", runtime_log_panel_html)
}
