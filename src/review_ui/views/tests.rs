use super::{render_bootstrap_html, render_html};
use crate::review_ui::MatcherPayload;

fn minimal_payload() -> MatcherPayload {
    MatcherPayload {
        left: vec![],
        right: vec![],
        album_name: "Testland".to_string(),
        colnect_dir: "Albums/Testland/colnect_inbox".to_string(),
        colnect_file_count: 0,
        show_imageless_left_rows: false,
        default_catalog_code: "sg".to_string(),
        default_sort_mode: "year_series_date_catalog".to_string(),
        odd_row_color: "#ffffff".to_string(),
        even_row_color: "#f2f2f2".to_string(),
        series_default_checked: true,
        description_default_checked: true,
        watermark_default_checked: true,
        design_default_checked: true,
        engraving_default_checked: true,
        perforation_default_checked: true,
        context_default_checked: true,
        colnect_links_default_checked: true,
        colnect_image_url_default_checked: true,
        colnect_metadata_default_checked: true,
        replace_sg_number_default_checked: false,
        right_unmatched_only_default_checked: false,
        albumeasy_auto_generate_pdf_default: false,
        albumeasy_cli_configured: false,
        catalog_family_options: vec![],
    }
}

#[test]
fn bootstrap_html_uses_default_inbox_hint_when_missing() {
    let html = render_bootstrap_html(None);
    assert!(html.contains("Colnect inbox hint: samples/colnect_inbox"));
}

#[test]
fn bootstrap_html_uses_custom_inbox_hint_when_present() {
    let html = render_bootstrap_html(Some("Albums/Testland/custom_inbox"));
    assert!(html.contains("Colnect inbox hint: Albums/Testland/custom_inbox"));
}

#[test]
fn bootstrap_html_contains_health_and_runtime_log_hooks() {
    let html = render_bootstrap_html(None);
    assert!(html.contains("/health"));
    assert!(html.contains("/runtime-log"));
    assert!(html.contains("id='runtimeLog'"));
}

#[test]
fn bootstrap_html_exposes_edit_config_action() {
    let html = render_bootstrap_html(None);
    assert!(html.contains("openConfigEditor()"));
    assert!(html.contains("Edit Config"));
}

#[test]
fn compare_html_embeds_payload_and_core_shell() {
    let payload = minimal_payload();
    let html = render_html(&payload);
    assert!(html.contains("const payload ="));
    assert!(html.contains("\"album_name\":\"Testland\""));
    assert!(html.contains("id='leftPanel'"));
    assert!(html.contains("id='rightPanel'"));
}

#[test]
fn compare_html_contains_backend_recovery_banner() {
    let payload = minimal_payload();
    let html = render_html(&payload);
    assert!(html.contains("id='reconnectBannerCompare'"));
    assert!(html.contains("retryBackendHealthCheckCompare"));
    assert!(html.contains("Connection lost, retrying..."));
}

#[test]
fn compare_html_contains_runtime_log_hooks() {
    let payload = minimal_payload();
    let html = render_html(&payload);
    assert!(html.contains("id='runtimeLogCompare'"));
    assert!(html.contains("id='showRuntimeLogCompare'"));
    assert!(html.contains("copyRuntimeLogCompare()"));
}