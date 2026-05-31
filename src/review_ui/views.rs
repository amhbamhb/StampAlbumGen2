mod shared;
mod bootstrap;
mod bootstrap_assets;
mod compare;
mod compare_assets;

use super::{render_config_editor_assets, MatcherPayload};
use shared::{
    render_app_footer_html, render_footer_year_script, render_reconnect_banner_html,
    render_runtime_log_panel_html, FOOTER_YEAR_UPDATE_JS,
};

pub(super) fn render_bootstrap_html(default_inbox_path: Option<&str>) -> String {
    bootstrap::render_bootstrap_html(default_inbox_path)
}

pub(super) fn render_html(payload: &MatcherPayload) -> String {
    compare::render_html(payload)
}

const BACKEND_NOT_RESPONDING_MESSAGE: &str = "The Review UI backend is not responding. Automatic retries are in progress.   This might be caused by loading of files, especially if you have many colnect files to import .  You can wait for recovery or reload now.";

#[cfg(test)]
mod tests;
