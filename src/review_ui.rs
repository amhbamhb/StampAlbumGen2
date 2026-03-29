use base64::Engine;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use tiny_http::{Header, Method, Response, Server, StatusCode};

mod views;
use views::{render_bootstrap_html, render_html};

mod config_editor;
use config_editor::{
    render_config_editor_assets, build_config_editor_data_for_album,
    save_album_config_diff, build_config_editor_data, save_full_config_values,
    update_default_catalog_code_for_album,
};

mod matcher_service;
use matcher_service::{
    add_colnect_stamp_to_stamps, add_left_stamp, apply_matches, clear_left_colnect_link,
    delete_colnect_candidate, delete_left_stamp, load_left_stamps, load_right_candidates,
    save_left_stamp, sync_processed_colnect_candidates, undo_colnect_delete,
};
mod http_handlers;
use http_handlers::{
    serve_bootstrap_requests, serve_review_requests,
};

static PDF_GENERATION_IN_PROGRESS: AtomicBool = AtomicBool::new(false);

struct PdfGenerationGuard;

impl PdfGenerationGuard {
    fn try_acquire() -> Option<Self> {
        if PDF_GENERATION_IN_PROGRESS
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            Some(Self)
        } else {
            None
        }
    }
}

impl Drop for PdfGenerationGuard {
    fn drop(&mut self) {
        PDF_GENERATION_IN_PROGRESS.store(false, Ordering::Release);
    }
}

#[derive(Debug, Clone, Serialize)]
struct DbField {
    key: String,
    value: String,
}

#[derive(Debug, Clone, Serialize)]
struct LeftStamp {
    rowid: i64,
    sg_number: String,
    year: i32,
    series: String,
    image_path: String,
    image_abs: String,
    stampworld_image_url: String,
    description: String,
    watermark: String,
    design: String,
    engraving: String,
    perforation: String,
    context: String,
    colnect_page_url: String,
    colnect_image_url: String,
    colnect_id: String,
    colnect_name: String,
    colnect_catalog_codes: String,
    colnect_country: String,
    colnect_width: String,
    colnect_height: String,
    colnect_format: String,
    colnect_emission: String,
    colnect_printing: String,
    colnect_paper: String,
    colnect_gum: String,
    colnect_ord_mi: String,
    colnect_ord_sn: String,
    colnect_ord_yt: String,
    colnect_face_value: String,
    colnect_currency: String,
    colnect_ord_sg: String,
    colnect_expiry_date: String,
    colnect_print_run: String,
    colnect_colors: String,
    colnect_parent_item: String,
    colnect_variant: String,
    colnect_overprint_variant: String,
    colnect_score: String,
    colnect_accuracy: String,
    colnect_list: String,
    colnect_quantity: String,
    colnect_condition: String,
    colnect_public_note: String,
    colnect_private_note: String,
    all_db_fields: Vec<DbField>,
}

#[derive(Debug, Clone, Serialize)]
struct RightCandidate {
    colnect_id: String,
    sg_number: String,
    year: i32,
    series: String,
    page_url: String,
    image_url: String,
    image_path: String,
    image_abs: String,
    description: String,
    watermark: String,
    design: String,
    engraving: String,
    perforation: String,
    context: String,
    name: String,
    catalog_codes_text: String,
    country: String,
    width: String,
    height: String,
    format: String,
    emission: String,
    printing: String,
    paper: String,
    gum: String,
    ord_mi: String,
    ord_sn: String,
    ord_yt: String,
    ord_sg: String,
    expiry_date: String,
    print_run: String,
    face_value: String,
    currency: String,
    colors: String,
    parent_item: String,
    variant: String,
    overprint_variant: String,
    score: String,
    accuracy: String,
    list: String,
    quantity: String,
    condition: String,
    public_note: String,
    private_note: String,
    catalog_code_pairs: Vec<CatalogCodePair>,
}

#[derive(Debug, Clone, Serialize)]
struct CatalogCodePair {
    catalog_type: String,
    code: String,
}

#[derive(Debug, Serialize)]
struct MatcherPayload {
    left: Vec<LeftStamp>,
    right: Vec<RightCandidate>,
    album_name: String,
    show_imageless_left_rows: bool,
    default_catalog_code: String,
    odd_row_color: String,
    even_row_color: String,
    series_default_checked: bool,
    description_default_checked: bool,
    watermark_default_checked: bool,
    design_default_checked: bool,
    engraving_default_checked: bool,
    perforation_default_checked: bool,
    context_default_checked: bool,
    colnect_links_default_checked: bool,
    colnect_image_url_default_checked: bool,
    colnect_metadata_default_checked: bool,
    replace_sg_number_default_checked: bool,
    right_unmatched_only_default_checked: bool,
    albumeasy_auto_generate_pdf_default: bool,
    albumeasy_cli_configured: bool,
}

#[derive(Debug, Clone, Serialize)]
struct BootstrapCountryInfo {
    country: String,
    db_path: String,
    colnect_file_count: usize,
    colnect_candidate_count: usize,
}

#[derive(Debug, Serialize)]
struct BootstrapStateResponse {
    countries: Vec<BootstrapCountryInfo>,
}

#[derive(Debug, Deserialize)]
struct BootstrapStartRequest {
    country: String,
}

#[derive(Debug, Serialize)]
struct BootstrapStartResponse {
    ok: bool,
    message: String,
    country: Option<String>,
    db_path: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BootstrapGeneratePdfRequest {
    country: String,
    #[serde(default)]
    start_year: Option<i32>,
    #[serde(default)]
    end_year: Option<i32>,
}

#[derive(Debug, Serialize)]
struct BootstrapGeneratePdfResponse {
    ok: bool,
    message: String,
    generated_output_path: Option<String>,
    generated_pdf_path: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SelectedMatch {
    left_rowid: i64,
    colnect_id: String,
    selected_catalog_code: Option<String>,
    selected_catalog_type: Option<String>,
    width: Option<String>,
    height: Option<String>,
}

#[derive(Debug, Deserialize, Default, Clone)]
struct FieldSelection {
    update_series: bool,
    update_description: bool,
    update_watermark: bool,
    update_design: bool,
    update_engraving: bool,
    update_perforation: bool,
    update_context: bool,
    update_colnect_links: bool,
    update_colnect_image_url: bool,
    update_colnect_metadata: bool,
    replace_sg_number: bool,
}

#[derive(Debug, Deserialize)]
struct ApplyMatchesEnvelope {
    matches: Vec<SelectedMatch>,
    fields: FieldSelection,
}

#[derive(Debug, Serialize)]
struct ApplyMatchesResponse {
    updated_rows: usize,
    skipped_rows: usize,
    processed_colnect_ids: Vec<String>,
    errors: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct AddColnectStampRequest {
    colnect_id: String,
    source_left_rowid: Option<i64>,
    target_sg_number: Option<String>,
    target_year: Option<i32>,
    target_series: Option<String>,
    associate_image: bool,
    image_mode: Option<String>,
    upload_data_url: Option<String>,
    upload_filename: Option<String>,
}

#[derive(Debug, Serialize)]
struct AddColnectStampResponse {
    ok: bool,
    message: String,
    added_left: Option<LeftStamp>,
    processed_colnect_id: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
struct SaveLeftStampRequest {
    rowid: i64,
    sg_number: String,
    year: i32,
    series: String,
    description: String,
    watermark: String,
    design: String,
    engraving: String,
    perforation: String,
    context: String,
    width: String,
    height: String,
    colnect_page_url: String,
    colnect_image_url: String,
    colnect_id: String,
    colnect_name: String,
    colnect_catalog_codes: String,
    colnect_country: String,
    colnect_width: String,
    colnect_height: String,
    colnect_format: String,
    colnect_emission: String,
    colnect_printing: String,
    colnect_paper: String,
    colnect_gum: String,
    colnect_ord_mi: String,
    colnect_ord_sn: String,
    colnect_ord_yt: String,
    colnect_face_value: String,
    colnect_currency: String,
    colnect_ord_sg: String,
    colnect_expiry_date: String,
    colnect_print_run: String,
    colnect_colors: String,
    colnect_parent_item: String,
    colnect_variant: String,
    colnect_overprint_variant: String,
    colnect_score: String,
    colnect_accuracy: String,
    colnect_list: String,
    colnect_quantity: String,
    colnect_condition: String,
    colnect_public_note: String,
    colnect_private_note: String,
    image_mode: Option<String>,
    upload_data_url: Option<String>,
    upload_filename: Option<String>,
}

#[derive(Debug, Serialize)]
struct SaveLeftStampResponse {
    ok: bool,
    message: String,
    updated_left: Option<LeftStamp>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
struct AddLeftStampRequest {
    source_rowid: Option<i64>,
    sg_number: String,
    year: i32,
    series: String,
    description: String,
    watermark: String,
    design: String,
    engraving: String,
    perforation: String,
    context: String,
    width: String,
    height: String,
    colnect_page_url: String,
    colnect_image_url: String,
    colnect_id: String,
    colnect_name: String,
    colnect_catalog_codes: String,
    colnect_country: String,
    colnect_width: String,
    colnect_height: String,
    colnect_format: String,
    colnect_emission: String,
    colnect_printing: String,
    colnect_paper: String,
    colnect_gum: String,
    colnect_ord_mi: String,
    colnect_ord_sn: String,
    colnect_ord_yt: String,
    colnect_face_value: String,
    colnect_currency: String,
    colnect_ord_sg: String,
    colnect_expiry_date: String,
    colnect_print_run: String,
    colnect_colors: String,
    colnect_parent_item: String,
    colnect_variant: String,
    colnect_overprint_variant: String,
    colnect_score: String,
    colnect_accuracy: String,
    colnect_list: String,
    colnect_quantity: String,
    colnect_condition: String,
    colnect_public_note: String,
    colnect_private_note: String,
    image_mode: Option<String>,
    upload_data_url: Option<String>,
    upload_filename: Option<String>,
}

#[derive(Debug, Serialize)]
struct AddLeftStampResponse {
    ok: bool,
    message: String,
    added_left: Option<LeftStamp>,
}

#[derive(Debug, Deserialize)]
struct DeleteLeftStampRequest {
    rowid: i64,
}

#[derive(Debug, Serialize)]
struct DeleteLeftStampResponse {
    ok: bool,
    message: String,
    deleted_rowid: Option<i64>,
    restored_colnect_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ClearLeftColnectLinkRequest {
    rowid: i64,
}

#[derive(Debug, Serialize)]
struct ClearLeftColnectLinkResponse {
    ok: bool,
    message: String,
    updated_left: Option<LeftStamp>,
    restored_colnect_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DeleteColnectCandidateRequest {
    colnect_id: String,
}

#[derive(Debug, Serialize)]
struct DeleteColnectCandidateResponse {
    ok: bool,
    message: String,
    deleted_stamp_rows: usize,
    deleted_code_rows: usize,
}

#[derive(Debug, Deserialize)]
struct UndoColnectDeleteRequest {
    colnect_id: String,
}

#[derive(Debug, Serialize)]
struct UndoColnectDeleteResponse {
    ok: bool,
    message: String,
    restored_rows: usize,
}

#[derive(Debug, Deserialize)]
struct ShutdownRequest {
    generate_album: bool,
    #[serde(default)]
    generate_pdf: bool,
    #[serde(default)]
    start_year: Option<i32>,
    #[serde(default)]
    end_year: Option<i32>,
}

#[derive(Debug, Serialize)]
struct ShutdownResponse {
    ok: bool,
    message: String,
    generated_output_path: Option<String>,
    generated_pdf_path: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ColnectImportRequest {
    // Empty request - just triggers import from configured/default colnect folder
}

#[derive(Debug, Serialize)]
struct ColnectImportResponse {
    ok: bool,
    message: String,
    files_processed: usize,
    stamps_imported: usize,
    errors: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct UpdateDefaultCatalogRequest {
    default_catalog_code: String,
}

#[derive(Debug, Serialize)]
struct UpdateDefaultCatalogResponse {
    ok: bool,
    message: String,
}

#[derive(Debug, Clone, Serialize)]
struct ConfigEditorField {
    id: String,
    section_id: String,
    section_title: String,
    key: String,
    template_value: String,
    current_value: String,
    /// Value from the global config.toml before any album override is applied.
    /// Equals `current_value` when there is no album override for this field.
    global_value: String,
    /// True when this field is explicitly set in the album-local config.toml.
    is_album_override: bool,
    comments: Vec<String>,
    from_template: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ConfigEditorSection {
    id: String,
    title: String,
    comments: Vec<String>,
    fields: Vec<ConfigEditorField>,
}

#[derive(Debug, Serialize)]
struct ConfigEditorDataResponse {
    ok: bool,
    message: String,
    sections: Vec<ConfigEditorSection>,
    /// Empty string when there is no active album (bootstrap UI).
    album_name: String,
    /// True when Albums/<album_name>/config.toml already exists.
    has_album_override: bool,
}

#[derive(Debug, Deserialize)]
struct SaveConfigEditorRequest {
    values: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
struct SaveConfigEditorResponse {
    ok: bool,
    message: String,
    reload_required: bool,
}

pub enum ReviewUiBootstrapOutcome {
    Exit,
    ReloadRequested,
    Selected {
        db_path: String,
        country: String,
        server: Server,
        bind: String,
    },
}

pub enum ReviewUiRunOutcome {
    Exit,
    ReloadRequested,
    ReturnToStartRequested,
}

fn path_to_file_url(path: &Path) -> Option<String> {
    let canonical = path.canonicalize().ok()?;
    let encoded = canonical
        .to_string_lossy()
        .split('/')
        .map(|segment| urlencoding::encode(segment).into_owned())
        .collect::<Vec<_>>()
        .join("/");
    Some(format!("file://{}", encoded))
}

fn open_local_path(path: &Path) {
    let Some(url) = path_to_file_url(path) else {
        log::warn!("Failed to build file URL for {}", path.display());
        return;
    };

    if let Err(error) = webbrowser::open(&url) {
        log::warn!(
            "Failed to open {} in default viewer: {}",
            path.display(),
            error
        );
    }
}

fn mime_from_path(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase()
        .as_str()
    {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "webp" => "image/webp",
        _ => "application/octet-stream",
    }
}

fn bind_review_server(
    bind_addr_opt: Option<&str>,
) -> Result<(Server, String), Box<dyn std::error::Error>> {
    if let Some(explicit) = bind_addr_opt {
        let srv = Server::http(explicit).map_err(|e| {
            std::io::Error::other(format!(
                "failed to start review UI server on {}: {}",
                explicit, e
            ))
        })?;
        return Ok((srv, explicit.to_string()));
    }

    const BASE_PORT: u16 = 7878;
    const MAX_TRIES: u16 = 10;
    let mut last_err = String::new();
    for offset in 0..MAX_TRIES {
        let candidate = format!("127.0.0.1:{}", BASE_PORT + offset);
        match Server::http(&candidate) {
            Ok(srv) => return Ok((srv, candidate)),
            Err(e) => {
                last_err = format!("{}", e);
            }
        }
    }

    Err(std::io::Error::other(format!(
        "failed to start review UI server: all ports {}-{} are in use (last error: {}). \
                 Kill the existing process with `fuser -k {}/tcp` or specify a free port with \
                 `--review-ui-bind 127.0.0.1:<PORT>`.",
        BASE_PORT,
        BASE_PORT + MAX_TRIES - 1,
        last_err,
        BASE_PORT,
    ))
    .into())
}

fn is_colnect_csv_file(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
        return false;
    };
    let lower = name.to_ascii_lowercase();
    lower.ends_with(".csv")
        && (lower.contains("colnect") || lower.starts_with("en_stamps_csv_list_countr"))
}

fn count_colnect_csv_files_rec(root: &Path) -> usize {
    if !root.exists() {
        return 0;
    }
    let Ok(entries) = fs::read_dir(root) else {
        return 0;
    };

    let mut count = 0usize;
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            count += count_colnect_csv_files_rec(&p);
        } else if is_colnect_csv_file(&p) {
            count += 1;
        }
    }
    count
}

fn count_colnect_candidates_in_db(db_path: &Path) -> usize {
    let Ok(conn) = Connection::open(db_path) else {
        return 0;
    };
    let has_table: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='colnect_stamps'",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    if has_table <= 0 {
        return 0;
    }
    conn.query_row(
        "SELECT COUNT(*) FROM colnect_stamps WHERE COALESCE(is_deleted, 0) = 0",
        [],
        |r| r.get::<_, i64>(0),
    )
    .unwrap_or(0)
    .max(0) as usize
}

fn scan_bootstrap_countries() -> Vec<BootstrapCountryInfo> {
    let albums_root = Path::new("Albums");
    let Ok(entries) = fs::read_dir(albums_root) else {
        return Vec::new();
    };

    let mut out = Vec::new();
    for entry in entries.flatten() {
        let dir = entry.path();
        if !dir.is_dir() {
            continue;
        }
        let Some(slug) = dir.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        let db_path = dir.join(format!("{}.db", slug));
        if !db_path.exists() {
            continue;
        }

        let country = slug.replace('-', " ");
        let colnect_dir = dir.join("colnect");
        let colnect_file_count = count_colnect_csv_files_rec(&colnect_dir);
        let colnect_candidate_count = count_colnect_candidates_in_db(&db_path);
        out.push(BootstrapCountryInfo {
            country,
            db_path: db_path.to_string_lossy().to_string(),
            colnect_file_count,
            colnect_candidate_count,
        });
    }

    out.sort_by(|a, b| a.country.to_lowercase().cmp(&b.country.to_lowercase()));
    out
}




fn resolve_image_abs(db_path: &str, album_name: &str, rel_image_path: &str) -> String {
    let rel = rel_image_path.trim();
    if rel.is_empty() {
        return String::new();
    }

    let p1 = Path::new("Albums")
        .join(album_name.replace(' ', "-"))
        .join(rel);
    if p1.exists() {
        return p1.to_string_lossy().to_string();
    }

    if let Some(parent) = Path::new(db_path).parent() {
        let p2 = parent.join(rel);
        if p2.exists() {
            return p2.to_string_lossy().to_string();
        }
    }

    String::new()
}

fn value_ref_to_string(v: rusqlite::types::ValueRef<'_>) -> String {
    match v {
        rusqlite::types::ValueRef::Null => String::new(),
        rusqlite::types::ValueRef::Integer(n) => n.to_string(),
        rusqlite::types::ValueRef::Real(f) => f.to_string(),
        rusqlite::types::ValueRef::Text(t) => String::from_utf8_lossy(t).to_string(),
        rusqlite::types::ValueRef::Blob(b) => format!("<{} bytes>", b.len()),
    }
}

fn load_stamp_all_fields(
    conn: &Connection,
    rowid: i64,
) -> Result<Vec<DbField>, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("SELECT * FROM stamps WHERE rowid = ?1 LIMIT 1")?;
    let column_names: Vec<String> = stmt
        .column_names()
        .iter()
        .map(|n| (*n).to_string())
        .collect();

    let mut rows = stmt.query(params![rowid])?;
    let Some(row) = rows.next()? else {
        return Ok(Vec::new());
    };

    let mut fields = Vec::with_capacity(column_names.len() + 1);
    fields.push(DbField {
        key: "rowid".to_string(),
        value: rowid.to_string(),
    });

    for (idx, key) in column_names.iter().enumerate() {
        let value = value_ref_to_string(row.get_ref(idx)?);
        fields.push(DbField {
            key: key.clone(),
            value,
        });
    }

    Ok(fields)
}





fn parse_query_param(url: &str, key: &str) -> Option<String> {
    let (_, query) = url.split_once('?')?;
    for pair in query.split('&') {
        let (k, v) = pair.split_once('=').unwrap_or((pair, ""));
        if k == key {
            return urlencoding::decode(v).ok().map(|s| s.to_string());
        }
    }
    None
}

fn parse_query_flag(url: &str, key: &str) -> bool {
    match parse_query_param(url, key) {
        Some(value) => {
            let normalized = value.trim().to_ascii_lowercase();
            normalized.is_empty()
                || normalized == "1"
                || normalized == "true"
                || normalized == "yes"
                || normalized == "on"
        }
        None => false,
    }
}

fn build_matcher_payload(
    conn: &Connection,
    db_path: &str,
    album_name: &str,
    default_catalog_code: &str,
    odd_row_color: &str,
    even_row_color: &str,
    series_default_checked: bool,
    description_default_checked: bool,
    watermark_default_checked: bool,
    design_default_checked: bool,
    engraving_default_checked: bool,
    perforation_default_checked: bool,
    context_default_checked: bool,
    colnect_links_default_checked: bool,
    colnect_image_url_default_checked: bool,
    colnect_metadata_default_checked: bool,
    replace_sg_number_default_checked: bool,
    right_unmatched_only_default_checked: bool,
    albumeasy_auto_generate_pdf_default: bool,
    albumeasy_cli_configured: bool,
    show_imageless_left_rows: bool,
) -> Result<MatcherPayload, Box<dyn std::error::Error>> {
    let left = load_left_stamps(conn, db_path, album_name, show_imageless_left_rows)?;
    let right = load_right_candidates(conn, db_path, album_name)?;

    Ok(MatcherPayload {
        left,
        right,
        album_name: album_name.to_string(),
        show_imageless_left_rows,
        default_catalog_code: default_catalog_code.to_string(),
        odd_row_color: odd_row_color.to_string(),
        even_row_color: even_row_color.to_string(),
        series_default_checked,
        description_default_checked,
        watermark_default_checked,
        design_default_checked,
        engraving_default_checked,
        perforation_default_checked,
        context_default_checked,
        colnect_links_default_checked,
        colnect_image_url_default_checked,
        colnect_metadata_default_checked,
        replace_sg_number_default_checked,
        right_unmatched_only_default_checked,
        albumeasy_auto_generate_pdf_default,
        albumeasy_cli_configured,
    })
}

fn album_dir_for_ui(db_path: &str, _album_name: &str) -> PathBuf {
    // Always derive the album directory from the db file's parent.
    // An earlier version checked for a local relative `Albums/<name>/` path
    // first, but that caused writes to go to the workspace test directory
    // instead of the actual album directory when an external db is used.
    Path::new(db_path)
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Load config, merging any album-local `config.toml` override on top of the
/// global one.  This is the preferred way to obtain a `Config` inside any
/// handler that already knows `db_path` and `album_name`.
fn load_album_config(db_path: &str, album_name: &str) -> crate::config::Config {
    let album_dir = album_dir_for_ui(db_path, album_name);
    crate::config::load_config_with_override("config.toml", &album_dir)
}

fn safe_filename_token(raw: &str) -> String {
    let mut out = String::new();
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else if ch == '-' || ch == '_' {
            out.push(ch);
        } else if ch.is_whitespace() {
            out.push('_');
        }
    }
    if out.is_empty() {
        "stamp".to_string()
    } else {
        out
    }
}

fn ext_from_data_url(data_url: &str, fallback_name: &str) -> &'static str {
    if data_url.starts_with("data:image/png") {
        return "png";
    }
    if data_url.starts_with("data:image/webp") {
        return "webp";
    }
    if data_url.starts_with("data:image/jpeg") || data_url.starts_with("data:image/jpg") {
        return "jpg";
    }

    let lower = fallback_name.to_ascii_lowercase();
    if lower.ends_with(".png") {
        "png"
    } else if lower.ends_with(".webp") {
        "webp"
    } else {
        "jpg"
    }
}

fn store_uploaded_image_from_data_url(
    db_path: &str,
    album_name: &str,
    colnect_id: &str,
    upload_data_url: &str,
    upload_filename: Option<&str>,
) -> Result<String, String> {
    let (header, payload) = upload_data_url
        .split_once(',')
        .ok_or_else(|| "upload data URL missing comma separator".to_string())?;

    if !header.contains(";base64") {
        return Err("upload data URL must be base64-encoded".to_string());
    }

    let bytes = base64::engine::general_purpose::STANDARD
        .decode(payload.trim())
        .map_err(|e| format!("failed to decode upload bytes: {}", e))?;

    if bytes.is_empty() {
        return Err("upload bytes are empty".to_string());
    }

    if image::load_from_memory(&bytes).is_err() {
        return Err("uploaded bytes are not a valid image".to_string());
    }

    let fallback_name = upload_filename.unwrap_or("upload.jpg");
    let ext = ext_from_data_url(upload_data_url, fallback_name);

    let ts = chrono::Utc::now().format("%Y%m%d%H%M%S");
    let cid = safe_filename_token(colnect_id);
    let file_name = format!("colnect_{}_{}.{}", cid, ts, ext);

    let album_dir = album_dir_for_ui(db_path, album_name);
    let images_dir = album_dir.join("images");
    fs::create_dir_all(&images_dir).map_err(|e| {
        format!(
            "failed to create images directory {}: {}",
            images_dir.display(),
            e
        )
    })?;

    let abs_path = images_dir.join(&file_name);
    fs::write(&abs_path, &bytes).map_err(|e| {
        format!(
            "failed to write uploaded image {}: {}",
            abs_path.display(),
            e
        )
    })?;

    Ok(format!("images/{}", file_name))
}













fn colnect_import_data(
    conn: &Connection,
    _req: ColnectImportRequest,
    db_path: &str,
    album_name: &str,
) -> ColnectImportResponse {
    // Load config and resolve candidate inbox paths.
    // Priority:
    // 1) configured colnect_inbox_path (if present)
    // 2) active album folder (<album>/colnect)
    let config = load_album_config(db_path, album_name);

    let mut inbox_candidates: Vec<PathBuf> = Vec::new();
    if let Some(configured_path) = config.effective_colnect_inbox_path() {
        inbox_candidates.push(PathBuf::from(configured_path));
    }

    let album_colnect_dir = album_dir_for_ui(db_path, album_name).join("colnect");
    if !inbox_candidates.iter().any(|p| p == &album_colnect_dir) {
        inbox_candidates.push(album_colnect_dir);
    }

    if inbox_candidates.is_empty() {
        return ColnectImportResponse {
            ok: false,
            message: "No colnect inbox path candidates could be resolved".to_string(),
            files_processed: 0,
            stamps_imported: 0,
            errors: vec!["No configured path and no album path available".to_string()],
        };
    }

    // Find CSV files in the first candidate folder that actually contains CSV files.
    let mut csv_files = Vec::new();
    let mut errors = Vec::new();
    let mut checked_dirs: Vec<String> = Vec::new();
    let mut used_inbox_dir: Option<PathBuf> = None;

    for inbox_dir in inbox_candidates {
        checked_dirs.push(inbox_dir.display().to_string());

        if !inbox_dir.exists() {
            errors.push(format!("Directory not found: {}", inbox_dir.display()));
            continue;
        }

        let mut local_csvs = Vec::new();
        match std::fs::read_dir(&inbox_dir) {
            Ok(entries) => {
                for entry_result in entries {
                    match entry_result {
                        Ok(entry) => {
                            let path = entry.path();
                            if path.is_file() {
                                if let Some(ext) = path.extension() {
                                    if ext.eq_ignore_ascii_case("csv") {
                                        local_csvs.push(path);
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            errors.push(format!(
                                "Failed to read directory entry in {}: {}",
                                inbox_dir.display(),
                                e
                            ));
                        }
                    }
                }
            }
            Err(e) => {
                errors.push(format!(
                    "Failed to read colnect inbox directory {}: {}",
                    inbox_dir.display(),
                    e
                ));
                continue;
            }
        }

        if !local_csvs.is_empty() {
            used_inbox_dir = Some(inbox_dir);
            csv_files = local_csvs;
            break;
        }
    }

    if csv_files.is_empty() {
        errors.push(format!("Checked folders: {}", checked_dirs.join(" | ")));
        return ColnectImportResponse {
            ok: true,
            message: "No CSV files found in colnect inbox".to_string(),
            files_processed: 0,
            stamps_imported: 0,
            errors,
        };
    }

    if let Some(dir) = used_inbox_dir {
        log::info!("Colnect import using inbox folder: {}", dir.display());
    }

    // Parse and ingest CSV files
    let mut total_stamps_imported = 0;
    let mut files_processed = 0;

    for csv_file in &csv_files {
        match crate::colnect::parser::parse_csv(csv_file) {
            Ok(stamps) => {
                let count = stamps.len();
                match crate::colnect::ingest::ingest_colnect_data_with_mode(
                    conn, &stamps, "sg",
                    false, // Don't clear existing - we're adding to what's already there
                ) {
                    Ok(_) => {
                        files_processed += 1;
                        total_stamps_imported += count;
                        log::info!("Imported {} stamps from {}", count, csv_file.display());
                    }
                    Err(e) => {
                        errors.push(format!("Failed to ingest {}: {}", csv_file.display(), e));
                    }
                }
            }
            Err(e) => {
                errors.push(format!("Failed to parse {}: {}", csv_file.display(), e));
            }
        }
    }

    ColnectImportResponse {
        ok: errors.is_empty(),
        message: format!(
            "Imported {} stamps from {} CSV file(s)",
            total_stamps_imported, files_processed
        ),
        files_processed,
        stamps_imported: total_stamps_imported,
        errors,
    }
}

/// Write `default_catalog_code` into the album-local config override
/// (`Albums/<name>/config.toml`), preserving any other per-album overrides
/// already present and only storing values that differ from the global config.
















/// Build config editor data for a country review UI session.
/// Returns `(sections, has_album_override)` where sections have `current_value`
/// set to the merged (effective) value and `global_value` / `is_album_override`
/// populated correctly.

/// Write a minimal album-local config.toml that stores only the fields that
/// differ from the global config.  The file is written into `album_dir`.
/// Returns the number of fields written (0 = nothing changed vs global).


pub fn run_review_ui_bootstrap(
    bind_addr_opt: Option<&str>,
    default_inbox_path: Option<&str>,
    open_browser: bool,
) -> Result<ReviewUiBootstrapOutcome, Box<dyn std::error::Error>> {
    eprintln!("Review UI: starting startup selector...");
    let html = render_bootstrap_html(default_inbox_path);
    let (server, bind) = bind_review_server(bind_addr_opt)?;

    let url = format!("http://{}/", bind);
    eprintln!("Review UI Setup: {}", url);
    if open_browser {
        let _ = webbrowser::open(&url);
    }

    let bootstrap_state = serve_bootstrap_requests(&server, &html);
    let selected = bootstrap_state.selected;
    let logout_requested = bootstrap_state.logout_requested;
    let reload_requested = bootstrap_state.reload_requested;

    if logout_requested {
        return Ok(ReviewUiBootstrapOutcome::Exit);
    }

    if reload_requested {
        return Ok(ReviewUiBootstrapOutcome::ReloadRequested);
    }

    Ok(match selected {
        Some((db_path, country)) => ReviewUiBootstrapOutcome::Selected {
            db_path,
            country,
            server,
            bind,
        },
        None => ReviewUiBootstrapOutcome::Exit,
    })
}

pub fn run_review_ui(
    db_path: &str,
    album_name_opt: Option<&str>,
    prebound_server: Option<(Server, String)>,
    bind_addr_opt: Option<&str>,
    open_browser: bool,
    default_catalog_code: &str,
    odd_row_color: &str,
    even_row_color: &str,
    series_default_checked: bool,
    description_default_checked: bool,
    watermark_default_checked: bool,
    design_default_checked: bool,
    engraving_default_checked: bool,
    perforation_default_checked: bool,
    context_default_checked: bool,
    colnect_links_default_checked: bool,
    colnect_image_url_default_checked: bool,
    colnect_metadata_default_checked: bool,
    replace_sg_number_default_checked: bool,
    right_unmatched_only_default_checked: bool,
    albumeasy_auto_generate_pdf_default: bool,
    albumeasy_cli_configured: bool,
) -> Result<ReviewUiRunOutcome, Box<dyn std::error::Error>> {
    eprintln!("Review UI: preparing data from {}", db_path);
    eprintln!("Review UI: opening database...");
    let conn = crate::db_utils::init_db(db_path)?;
    let _ = conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_stamps_review_sg_year ON stamps (sg_number, year)",
        [],
    );
    let _ = conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_colnect_review_queue ON colnect_stamps (is_deleted, is_processed, issued_on, sg_number)",
        [],
    );
    eprintln!("Review UI: syncing processed Colnect candidates...");
    sync_processed_colnect_candidates(&conn)?;

    let album_name = album_name_opt
        .map(|s| s.to_string())
        .or_else(|| {
            Path::new(db_path)
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
        })
        .unwrap_or_else(|| "Album".to_string());

    let initial_show_imageless_left_rows = false;
    eprintln!(
        "Review UI: loading left StampWorld rows (show image-less: {})...",
        initial_show_imageless_left_rows
    );
    let payload = build_matcher_payload(
        &conn,
        db_path,
        &album_name,
        default_catalog_code,
        odd_row_color,
        even_row_color,
        series_default_checked,
        description_default_checked,
        watermark_default_checked,
        design_default_checked,
        engraving_default_checked,
        perforation_default_checked,
        context_default_checked,
        colnect_links_default_checked,
        colnect_image_url_default_checked,
        colnect_metadata_default_checked,
        replace_sg_number_default_checked,
        right_unmatched_only_default_checked,
        albumeasy_auto_generate_pdf_default,
        albumeasy_cli_configured,
        initial_show_imageless_left_rows,
    )?;
    eprintln!(
        "Review UI: loaded {} left rows and {} right candidates",
        payload.left.len(),
        payload.right.len()
    );

    let right_map: HashMap<String, RightCandidate> = payload
        .right
        .iter()
        .cloned()
        .map(|c| (c.colnect_id.clone(), c))
        .collect();

    let html = render_html(&payload);

    eprintln!("Review UI: starting local web server...");
    let (server, bind) = if let Some(pb) = prebound_server {
        pb
    } else {
        bind_review_server(bind_addr_opt)?
    };

    let url = format!("http://{}/", bind);
    eprintln!("Review UI: {}", url);
    if open_browser {
        eprintln!("Review UI: opening browser...");
        let _ = webbrowser::open(&url);
    } else {
        eprintln!("Review UI: waiting for bootstrap tab to load compare view...");
    }

    let review_state = serve_review_requests(
        &server,
        &conn,
        db_path,
        &album_name,
        &html,
        &right_map,
        default_catalog_code,
        odd_row_color,
        even_row_color,
        series_default_checked,
        description_default_checked,
        watermark_default_checked,
        design_default_checked,
        engraving_default_checked,
        perforation_default_checked,
        context_default_checked,
        colnect_links_default_checked,
        colnect_image_url_default_checked,
        colnect_metadata_default_checked,
        replace_sg_number_default_checked,
        right_unmatched_only_default_checked,
        albumeasy_auto_generate_pdf_default,
        albumeasy_cli_configured,
    );
    let reload_requested = review_state.reload_requested;
    let return_to_start_requested = review_state.return_to_start_requested;

    if reload_requested {
        return Ok(ReviewUiRunOutcome::ReloadRequested);
    }

    if return_to_start_requested {
        return Ok(ReviewUiRunOutcome::ReturnToStartRequested);
    }

    Ok(ReviewUiRunOutcome::Exit)
}

#[cfg(test)]
mod tests {
    use super::matcher_service::{
        mark_colnect_candidate_processed, sync_processed_colnect_candidates,
    };
    use rusqlite::Connection;

    #[test]
    fn sync_processed_colnect_candidates_marks_existing_stamp_links() {
        let conn = Connection::open_in_memory().expect("in-memory db");
        conn.execute("CREATE TABLE stamps (colnect_id TEXT)", [])
            .expect("create stamps");
        conn.execute(
            "CREATE TABLE colnect_stamps (colnect_id TEXT, is_processed INTEGER DEFAULT 0)",
            [],
        )
        .expect("create colnect_stamps");

        conn.execute("INSERT INTO stamps (colnect_id) VALUES ('CID-1')", [])
            .expect("insert stamp");
        conn.execute(
            "INSERT INTO colnect_stamps (colnect_id, is_processed) VALUES ('CID-1', 0), ('CID-2', 1)",
            [],
        )
        .expect("insert candidates");

        sync_processed_colnect_candidates(&conn).expect("sync processed flags");

        let cid1: i64 = conn
            .query_row(
                "SELECT is_processed FROM colnect_stamps WHERE colnect_id = 'CID-1'",
                [],
                |row| row.get(0),
            )
            .expect("cid1 processed state");
        let cid2: i64 = conn
            .query_row(
                "SELECT is_processed FROM colnect_stamps WHERE colnect_id = 'CID-2'",
                [],
                |row| row.get(0),
            )
            .expect("cid2 processed state");

        assert_eq!(cid1, 1);
        assert_eq!(cid2, 0);
    }

    #[test]
    fn mark_colnect_candidate_processed_sets_flag_for_matching_candidate() {
        let conn = Connection::open_in_memory().expect("in-memory db");
        conn.execute(
            "CREATE TABLE colnect_stamps (colnect_id TEXT, is_processed INTEGER DEFAULT 0)",
            [],
        )
        .expect("create colnect_stamps");
        conn.execute(
            "INSERT INTO colnect_stamps (colnect_id, is_processed) VALUES ('CID-9', 0)",
            [],
        )
        .expect("insert candidate");

        let updated = mark_colnect_candidate_processed(&conn, "CID-9").expect("mark processed");
        let processed: i64 = conn
            .query_row(
                "SELECT is_processed FROM colnect_stamps WHERE colnect_id = 'CID-9'",
                [],
                |row| row.get(0),
            )
            .expect("processed state");

        assert_eq!(updated, 1);
        assert_eq!(processed, 1);
    }
}
