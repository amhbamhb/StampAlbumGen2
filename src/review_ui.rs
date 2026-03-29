use base64::Engine;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
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
mod http_handlers;

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


fn sync_processed_colnect_candidates(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute(
        "UPDATE colnect_stamps
         SET is_processed = CASE
             WHEN TRIM(COALESCE(colnect_id, '')) != ''
              AND EXISTS (
                  SELECT 1
                  FROM stamps s
                  WHERE TRIM(COALESCE(s.colnect_id, '')) = TRIM(COALESCE(colnect_stamps.colnect_id, ''))
                    AND TRIM(COALESCE(s.colnect_id, '')) != ''
              ) THEN 1
             ELSE 0
         END",
        [],
    )?;
    Ok(())
}

fn mark_colnect_candidate_processed(
    conn: &Connection,
    colnect_id: &str,
) -> rusqlite::Result<usize> {
    if colnect_id.trim().is_empty() {
        return Ok(0);
    }

    conn.execute(
        "UPDATE colnect_stamps
         SET is_processed = 1
         WHERE TRIM(COALESCE(colnect_id, '')) = TRIM(?1)",
        params![colnect_id],
    )
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

fn load_left_stamps(
    conn: &Connection,
    db_path: &str,
    album_name: &str,
    include_imageless: bool,
) -> Result<Vec<LeftStamp>, Box<dyn std::error::Error>> {
    let query = if include_imageless {
        "SELECT rowid,
                COALESCE(sg_number, ''),
                COALESCE(year, 0),
                COALESCE(series, ''),
                COALESCE(image_path, ''),
                COALESCE(stampworld_image_url, ''),
                COALESCE(description, ''),
                COALESCE(watermark, ''),
                COALESCE(design, ''),
                COALESCE(engraving, ''),
                COALESCE(perforation, ''),
            COALESCE(context, ''),
            COALESCE(colnect_page_url, ''),
            COALESCE(colnect_image_url, ''),
            COALESCE(colnect_id, ''),
            COALESCE(colnect_name, ''),
            COALESCE(colnect_catalog_codes, ''),
            COALESCE(colnect_country, ''),
            COALESCE(colnect_width, ''),
            COALESCE(colnect_height, ''),
            COALESCE(colnect_format, ''),
            COALESCE(colnect_emission, ''),
            COALESCE(colnect_printing, ''),
            COALESCE(colnect_paper, ''),
            COALESCE(colnect_gum, ''),
            COALESCE(colnect_ord_mi, ''),
            COALESCE(colnect_ord_sn, ''),
            COALESCE(colnect_ord_yt, ''),
            COALESCE(colnect_face_value, ''),
            COALESCE(colnect_currency, ''),
            COALESCE(colnect_ord_sg, ''),
            COALESCE(colnect_expiry_date, ''),
            COALESCE(colnect_print_run, ''),
            COALESCE(colnect_colors, ''),
            COALESCE(colnect_parent_item, ''),
            COALESCE(colnect_variant, ''),
            COALESCE(colnect_overprint_variant, ''),
            COALESCE(colnect_score, ''),
            COALESCE(colnect_accuracy, ''),
            COALESCE(colnect_list, ''),
            COALESCE(colnect_quantity, ''),
            COALESCE(colnect_condition, ''),
            COALESCE(colnect_public_note, ''),
            COALESCE(colnect_private_note, '')
         FROM stamps
         ORDER BY year, sg_number"
    } else {
        "SELECT rowid,
                COALESCE(sg_number, ''),
                COALESCE(year, 0),
                COALESCE(series, ''),
                COALESCE(image_path, ''),
                COALESCE(stampworld_image_url, ''),
                COALESCE(description, ''),
                COALESCE(watermark, ''),
                COALESCE(design, ''),
                COALESCE(engraving, ''),
                COALESCE(perforation, ''),
            COALESCE(context, ''),
            COALESCE(colnect_page_url, ''),
            COALESCE(colnect_image_url, ''),
            COALESCE(colnect_id, ''),
            COALESCE(colnect_name, ''),
            COALESCE(colnect_catalog_codes, ''),
            COALESCE(colnect_country, ''),
            COALESCE(colnect_width, ''),
            COALESCE(colnect_height, ''),
            COALESCE(colnect_format, ''),
            COALESCE(colnect_emission, ''),
            COALESCE(colnect_printing, ''),
            COALESCE(colnect_paper, ''),
            COALESCE(colnect_gum, ''),
            COALESCE(colnect_ord_mi, ''),
            COALESCE(colnect_ord_sn, ''),
            COALESCE(colnect_ord_yt, ''),
            COALESCE(colnect_face_value, ''),
            COALESCE(colnect_currency, ''),
            COALESCE(colnect_ord_sg, ''),
            COALESCE(colnect_expiry_date, ''),
            COALESCE(colnect_print_run, ''),
            COALESCE(colnect_colors, ''),
            COALESCE(colnect_parent_item, ''),
            COALESCE(colnect_variant, ''),
            COALESCE(colnect_overprint_variant, ''),
            COALESCE(colnect_score, ''),
            COALESCE(colnect_accuracy, ''),
            COALESCE(colnect_list, ''),
            COALESCE(colnect_quantity, ''),
            COALESCE(colnect_condition, ''),
            COALESCE(colnect_public_note, ''),
            COALESCE(colnect_private_note, '')
         FROM stamps
         WHERE TRIM(COALESCE(image_path, '')) != ''
           AND LOWER(COALESCE(image_path, '')) NOT LIKE 'images/no_image%'
         ORDER BY year, sg_number"
    };
    let mut stmt = conn.prepare(query)?;

    let rows = stmt.query_map([], |row| {
        let rel_image = row.get::<_, String>(4)?;
        Ok(LeftStamp {
            rowid: row.get(0)?,
            sg_number: row.get(1)?,
            year: row.get(2)?,
            series: row.get(3)?,
            image_path: rel_image.clone(),
            image_abs: resolve_image_abs(db_path, album_name, &rel_image),
            stampworld_image_url: row.get(5)?,
            description: row.get(6)?,
            watermark: row.get(7)?,
            design: row.get(8)?,
            engraving: row.get(9)?,
            perforation: row.get(10)?,
            context: row.get(11)?,
            colnect_page_url: row.get(12)?,
            colnect_image_url: row.get(13)?,
            colnect_id: row.get(14)?,
            colnect_name: row.get(15)?,
            colnect_catalog_codes: row.get(16)?,
            colnect_country: row.get(17)?,
            colnect_width: row.get(18)?,
            colnect_height: row.get(19)?,
            colnect_format: row.get(20)?,
            colnect_emission: row.get(21)?,
            colnect_printing: row.get(22)?,
            colnect_paper: row.get(23)?,
            colnect_gum: row.get(24)?,
            colnect_ord_mi: row.get(25)?,
            colnect_ord_sn: row.get(26)?,
            colnect_ord_yt: row.get(27)?,
            colnect_face_value: row.get(28)?,
            colnect_currency: row.get(29)?,
            colnect_ord_sg: row.get(30)?,
            colnect_expiry_date: row.get(31)?,
            colnect_print_run: row.get(32)?,
            colnect_colors: row.get(33)?,
            colnect_parent_item: row.get(34)?,
            colnect_variant: row.get(35)?,
            colnect_overprint_variant: row.get(36)?,
            colnect_score: row.get(37)?,
            colnect_accuracy: row.get(38)?,
            colnect_list: row.get(39)?,
            colnect_quantity: row.get(40)?,
            colnect_condition: row.get(41)?,
            colnect_public_note: row.get(42)?,
            colnect_private_note: row.get(43)?,
            all_db_fields: Vec::new(),
        })
    })?;

    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    for stamp in &mut out {
        stamp.all_db_fields = load_stamp_all_fields(conn, stamp.rowid).unwrap_or_default();
    }
    Ok(out)
}

fn load_right_candidates(
    conn: &Connection,
    db_path: &str,
    album_name: &str,
) -> Result<Vec<RightCandidate>, Box<dyn std::error::Error>> {
    let mut code_stmt = conn.prepare(
        "SELECT COALESCE(colnect_id, ''), COALESCE(catalog_type, ''), COALESCE(code, '')
         FROM colnect_catalog_codes",
    )?;
    let code_rows = code_stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;
    let mut code_map: HashMap<String, Vec<CatalogCodePair>> = HashMap::new();
    for r in code_rows {
        let (cid, ctype, code) = r?;
        if cid.trim().is_empty() || code.trim().is_empty() {
            continue;
        }
        code_map.entry(cid).or_default().push(CatalogCodePair {
            catalog_type: ctype,
            code,
        });
    }

    let mut stmt = conn.prepare(
        "SELECT COALESCE(c.colnect_id, ''),
                COALESCE(c.sg_number, ''),
                CAST(substr(COALESCE(c.issued_on, ''), 1, 4) AS INTEGER),
                COALESCE(c.series, ''),
                COALESCE(c.colnect_link, ''),
                COALESCE(c.description, ''),
                COALESCE(c.name, ''),
                COALESCE(c.watermark, ''),
                COALESCE(c.designers, ''),
                COALESCE(c.printers, ''),
                COALESCE(c.perforation, ''),
                COALESCE(c.themes, ''),
                COALESCE(c.catalog_codes, ''),
                COALESCE(c.country, ''),
                COALESCE(c.width, ''),
                COALESCE(c.height, ''),
                COALESCE(c.format, ''),
                COALESCE(c.emission, ''),
                COALESCE(c.printing, ''),
                COALESCE(c.paper, ''),
                COALESCE(c.gum, ''),
                COALESCE(c.ord_mi, ''),
                COALESCE(c.ord_sn, ''),
                COALESCE(c.ord_yt, ''),
                COALESCE(c.ord_sg, ''),
                COALESCE(c.expiry_date, ''),
                COALESCE(c.print_run, ''),
                COALESCE(c.face_value, ''),
                COALESCE(c.currency, ''),
                COALESCE(c.colors, ''),
                COALESCE(c.parent_item, ''),
                COALESCE(c.variant, ''),
                COALESCE(c.overprint_variant, ''),
                COALESCE(c.score, ''),
                COALESCE(c.accuracy, ''),
                COALESCE(c.list, ''),
                COALESCE(c.quantity, ''),
                COALESCE(c.condition, ''),
                COALESCE(c.public_note, ''),
                COALESCE(c.private_note, ''),
                COALESCE((
                    SELECT s.image_path
                    FROM stamps s
                    WHERE s.sg_number = c.sg_number
                      AND s.year = CAST(substr(COALESCE(c.issued_on, ''), 1, 4) AS INTEGER)
                      AND TRIM(COALESCE(s.image_path, '')) != ''
                    LIMIT 1
                 ), '')
         FROM colnect_stamps c
         WHERE COALESCE(c.is_deleted, 0) = 0
                     AND COALESCE(c.is_processed, 0) = 0
         ORDER BY CAST(substr(COALESCE(c.issued_on, ''), 1, 4) AS INTEGER), c.sg_number",
    )?;

    let rows = stmt.query_map([], |row| {
        let page_url = row.get::<_, String>(4)?;
        // Resolve Colnect image URLs lazily in the browser to avoid N network
        // requests during startup for every candidate.
        let image_url = String::new();
        let description = {
            let d = row.get::<_, String>(5)?;
            if d.trim().is_empty() {
                row.get::<_, String>(6)?
            } else {
                d
            }
        };
        let image_path = row.get::<_, String>(40)?;

        let colnect_id = row.get::<_, String>(0)?;

        Ok(RightCandidate {
            colnect_id: colnect_id.clone(),
            sg_number: row.get(1)?,
            year: row.get(2)?,
            series: row.get(3)?,
            page_url,
            image_url,
            image_path: image_path.clone(),
            image_abs: resolve_image_abs(db_path, album_name, &image_path),
            name: row.get(6)?,
            description,
            watermark: row.get(7)?,
            design: row.get(8)?,
            engraving: row.get(9)?,
            perforation: row.get(10)?,
            context: row.get(11)?,
            catalog_codes_text: row.get(12)?,
            country: row.get(13)?,
            width: row.get(14)?,
            height: row.get(15)?,
            format: row.get(16)?,
            emission: row.get(17)?,
            printing: row.get(18)?,
            paper: row.get(19)?,
            gum: row.get(20)?,
            ord_mi: row.get(21)?,
            ord_sn: row.get(22)?,
            ord_yt: row.get(23)?,
            ord_sg: row.get(24)?,
            expiry_date: row.get(25)?,
            print_run: row.get(26)?,
            face_value: row.get(27)?,
            currency: row.get(28)?,
            colors: row.get(29)?,
            parent_item: row.get(30)?,
            variant: row.get(31)?,
            overprint_variant: row.get(32)?,
            score: row.get(33)?,
            accuracy: row.get(34)?,
            list: row.get(35)?,
            quantity: row.get(36)?,
            condition: row.get(37)?,
            public_note: row.get(38)?,
            private_note: row.get(39)?,
            catalog_code_pairs: code_map.get(&colnect_id).cloned().unwrap_or_default(),
        })
    })?;

    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

fn apply_matches(
    conn: &Connection,
    db_path: &str,
    album_name: &str,
    matches: Vec<SelectedMatch>,
    fields: FieldSelection,
    right_map: &HashMap<String, RightCandidate>,
) -> ApplyMatchesResponse {
    let default_catalog_code =
        load_album_config(db_path, album_name).effective_default_catalog_code();
    let mut updated_rows = 0usize;
    let mut skipped_rows = 0usize;
    let mut processed_colnect_ids = Vec::new();
    let mut errors = Vec::new();

    for m in matches {
        let Some(c) = right_map.get(&m.colnect_id) else {
            skipped_rows += 1;
            continue;
        };

        let selected_catalog_code =
            crate::catalog::normalize_catalog_number(&m.selected_catalog_code.unwrap_or_default());
        let selected_catalog_type = m
            .selected_catalog_type
            .as_deref()
            .map(crate::catalog::normalize_catalog_type)
            .unwrap_or_else(|| {
                crate::catalog::infer_catalog_type_for_number(
                    &c.catalog_codes_text,
                    &selected_catalog_code,
                    &default_catalog_code,
                )
            });
        let selected_width = m.width.as_deref().unwrap_or(c.width.as_str());
        let selected_height = m.height.as_deref().unwrap_or(c.height.as_str());

        let res = conn.execute(
            "UPDATE stamps
             SET sg_number = CASE WHEN ?1 = 1 AND TRIM(?2) != '' THEN ?2 ELSE sg_number END,
                 catalog_code = CASE WHEN ?1 = 1 AND TRIM(?2) != '' THEN ?3 ELSE catalog_code END,
                      series = CASE WHEN ?4 = 1 AND ?5 != '' THEN ?5 ELSE series END,
                 uniq_key = lower(coalesce(country,'')) || '|' ||
                                     CASE WHEN ?4 = 1 AND ?5 != '' THEN ?5 ELSE coalesce(series,'') END || '|' ||
                            CASE WHEN ?1 = 1 AND TRIM(?2) != '' THEN ?2 ELSE coalesce(sg_number,'') END,
                      description = CASE WHEN ?6 = 1 AND ?7 != '' THEN ?7 ELSE description END,
                      watermark = CASE WHEN ?8 = 1 AND ?9 != '' THEN ?9 ELSE watermark END,
                      design = CASE WHEN ?10 = 1 AND ?11 != '' THEN ?11 ELSE design END,
                      engraving = CASE WHEN ?12 = 1 AND ?13 != '' THEN ?13 ELSE engraving END,
                      perforation = CASE WHEN ?14 = 1 AND ?15 != '' THEN ?15 ELSE perforation END,
                      context = CASE WHEN ?16 = 1 AND ?17 != '' THEN ?17 ELSE context END,
                      colnect_page_url = CASE WHEN ?18 = 1 AND ?19 != '' THEN ?19 ELSE colnect_page_url END,
                      original_image_url = CASE WHEN ?18 = 1 AND ?19 != '' THEN ?19 ELSE original_image_url END,
                      colnect_image_url = CASE WHEN ?20 = 1 AND ?21 != '' THEN ?21 ELSE colnect_image_url END,
                 remote_image_url = CASE
                          WHEN (?20 = 1) AND (?21 != '') AND (TRIM(COALESCE(remote_image_url, '')) = '' OR remote_image_url LIKE '%colnect.com/stamps/stamp/%') THEN ?21
                    ELSE remote_image_url
                 END,
                      colnect_id = CASE WHEN ?22 = 1 THEN ?23 ELSE colnect_id END,
                      colnect_name = CASE WHEN ?22 = 1 THEN ?24 ELSE colnect_name END,
                      colnect_catalog_codes = CASE WHEN ?22 = 1 THEN ?25 ELSE colnect_catalog_codes END,
                      colnect_country = CASE WHEN ?22 = 1 THEN ?26 ELSE colnect_country END,
                      colnect_width = CASE WHEN ?22 = 1 THEN ?27 ELSE colnect_width END,
                      colnect_height = CASE WHEN ?22 = 1 THEN ?28 ELSE colnect_height END,
                      colnect_format = CASE WHEN ?22 = 1 THEN ?29 ELSE colnect_format END,
                      colnect_emission = CASE WHEN ?22 = 1 THEN ?30 ELSE colnect_emission END,
                      colnect_printing = CASE WHEN ?22 = 1 THEN ?31 ELSE colnect_printing END,
                      colnect_paper = CASE WHEN ?22 = 1 THEN ?32 ELSE colnect_paper END,
                      colnect_gum = CASE WHEN ?22 = 1 THEN ?33 ELSE colnect_gum END,
                      colnect_ord_mi = CASE WHEN ?22 = 1 THEN ?34 ELSE colnect_ord_mi END,
                      colnect_ord_sn = CASE WHEN ?22 = 1 THEN ?35 ELSE colnect_ord_sn END,
                      colnect_ord_yt = CASE WHEN ?22 = 1 THEN ?36 ELSE colnect_ord_yt END,
                      colnect_ord_sg = CASE WHEN ?22 = 1 THEN ?37 ELSE colnect_ord_sg END,
                      colnect_expiry_date = CASE WHEN ?22 = 1 THEN ?38 ELSE colnect_expiry_date END,
                      colnect_print_run = CASE WHEN ?22 = 1 THEN ?39 ELSE colnect_print_run END,
                      colnect_face_value = CASE WHEN ?22 = 1 THEN ?40 ELSE colnect_face_value END,
                      colnect_currency = CASE WHEN ?22 = 1 THEN ?41 ELSE colnect_currency END,
                      colnect_colors = CASE WHEN ?22 = 1 THEN ?42 ELSE colnect_colors END,
                      colnect_parent_item = CASE WHEN ?22 = 1 THEN ?43 ELSE colnect_parent_item END,
                      colnect_variant = CASE WHEN ?22 = 1 THEN ?44 ELSE colnect_variant END,
                      colnect_overprint_variant = CASE WHEN ?22 = 1 THEN ?45 ELSE colnect_overprint_variant END,
                      colnect_score = CASE WHEN ?22 = 1 THEN ?46 ELSE colnect_score END,
                      colnect_accuracy = CASE WHEN ?22 = 1 THEN ?47 ELSE colnect_accuracy END,
                      colnect_list = CASE WHEN ?22 = 1 THEN ?48 ELSE colnect_list END,
                      colnect_quantity = CASE WHEN ?22 = 1 THEN ?49 ELSE colnect_quantity END,
                      colnect_condition = CASE WHEN ?22 = 1 THEN ?50 ELSE colnect_condition END,
                      colnect_public_note = CASE WHEN ?22 = 1 THEN ?51 ELSE colnect_public_note END,
                      colnect_private_note = CASE WHEN ?22 = 1 THEN ?52 ELSE colnect_private_note END
                 WHERE rowid = ?53",
            params![
                if fields.replace_sg_number { 1 } else { 0 },
                selected_catalog_code,
                selected_catalog_type,
                if fields.update_series { 1 } else { 0 },
                c.series,
                if fields.update_description { 1 } else { 0 },
                c.description,
                if fields.update_watermark { 1 } else { 0 },
                c.watermark,
                if fields.update_design { 1 } else { 0 },
                c.design,
                if fields.update_engraving { 1 } else { 0 },
                c.engraving,
                if fields.update_perforation { 1 } else { 0 },
                c.perforation,
                if fields.update_context { 1 } else { 0 },
                c.context,
                if fields.update_colnect_links { 1 } else { 0 },
                c.page_url,
                if fields.update_colnect_image_url { 1 } else { 0 },
                c.image_url,
                if fields.update_colnect_metadata { 1 } else { 0 },
                c.colnect_id,
                c.name,
                c.catalog_codes_text,
                c.country,
                selected_width,
                selected_height,
                c.format,
                c.emission,
                c.printing,
                c.paper,
                c.gum,
                c.ord_mi,
                c.ord_sn,
                c.ord_yt,
                c.ord_sg,
                c.expiry_date,
                c.print_run,
                c.face_value,
                c.currency,
                c.colors,
                c.parent_item,
                c.variant,
                c.overprint_variant,
                c.score,
                c.accuracy,
                c.list,
                c.quantity,
                c.condition,
                c.public_note,
                c.private_note,
                m.left_rowid,
            ],
        );

        match res {
            Ok(n) if n > 0 => {
                updated_rows += 1;
                match mark_colnect_candidate_processed(conn, &m.colnect_id) {
                    Ok(_) => processed_colnect_ids.push(m.colnect_id.clone()),
                    Err(e) => errors.push(format!(
                        "updated rowid={} but failed to mark colnect_id={} processed: {}",
                        m.left_rowid, m.colnect_id, e
                    )),
                }
            }
            Ok(_) => skipped_rows += 1,
            Err(e) => errors.push(format!(
                "failed to apply colnect_id={} to rowid={}: {}",
                m.colnect_id, m.left_rowid, e
            )),
        }
    }

    ApplyMatchesResponse {
        updated_rows,
        skipped_rows,
        processed_colnect_ids,
        errors,
    }
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

fn load_left_stamp_by_rowid(
    conn: &Connection,
    db_path: &str,
    album_name: &str,
    rowid: i64,
) -> Result<Option<LeftStamp>, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare(
        "SELECT rowid,
                COALESCE(sg_number, ''),
                COALESCE(year, 0),
                COALESCE(series, ''),
                COALESCE(image_path, ''),
                COALESCE(stampworld_image_url, ''),
                COALESCE(description, ''),
                COALESCE(watermark, ''),
                COALESCE(design, ''),
                COALESCE(engraving, ''),
                COALESCE(perforation, ''),
                COALESCE(context, ''),
                  COALESCE(colnect_page_url, ''),
                  COALESCE(colnect_image_url, ''),
                COALESCE(colnect_id, ''),
                COALESCE(colnect_name, ''),
                COALESCE(colnect_catalog_codes, ''),
                COALESCE(colnect_country, ''),
                  COALESCE(colnect_width, ''),
                  COALESCE(colnect_height, ''),
                  COALESCE(colnect_format, ''),
                  COALESCE(colnect_emission, ''),
                  COALESCE(colnect_printing, ''),
                  COALESCE(colnect_paper, ''),
                  COALESCE(colnect_gum, ''),
                  COALESCE(colnect_ord_mi, ''),
                  COALESCE(colnect_ord_sn, ''),
                  COALESCE(colnect_ord_yt, ''),
                COALESCE(colnect_face_value, ''),
                COALESCE(colnect_currency, ''),
                COALESCE(colnect_ord_sg, ''),
                  COALESCE(colnect_expiry_date, ''),
                COALESCE(colnect_print_run, ''),
                  COALESCE(colnect_colors, ''),
                  COALESCE(colnect_parent_item, ''),
                  COALESCE(colnect_variant, ''),
                  COALESCE(colnect_overprint_variant, ''),
                  COALESCE(colnect_score, ''),
                  COALESCE(colnect_accuracy, ''),
                  COALESCE(colnect_list, ''),
                  COALESCE(colnect_quantity, ''),
                  COALESCE(colnect_condition, ''),
                COALESCE(colnect_public_note, ''),
                COALESCE(colnect_private_note, '')
         FROM stamps
         WHERE rowid = ?1
         LIMIT 1",
    )?;

    let mut rows = stmt.query(params![rowid])?;
    let Some(row) = rows.next()? else {
        return Ok(None);
    };

    let rel_image: String = row.get(4)?;
    let resolved_rowid: i64 = row.get(0)?;
    Ok(Some(LeftStamp {
        rowid: resolved_rowid,
        sg_number: row.get(1)?,
        year: row.get(2)?,
        series: row.get(3)?,
        image_path: rel_image.clone(),
        image_abs: resolve_image_abs(db_path, album_name, &rel_image),
        stampworld_image_url: row.get(5)?,
        description: row.get(6)?,
        watermark: row.get(7)?,
        design: row.get(8)?,
        engraving: row.get(9)?,
        perforation: row.get(10)?,
        context: row.get(11)?,
        colnect_page_url: row.get(12)?,
        colnect_image_url: row.get(13)?,
        colnect_id: row.get(14)?,
        colnect_name: row.get(15)?,
        colnect_catalog_codes: row.get(16)?,
        colnect_country: row.get(17)?,
        colnect_width: row.get(18)?,
        colnect_height: row.get(19)?,
        colnect_format: row.get(20)?,
        colnect_emission: row.get(21)?,
        colnect_printing: row.get(22)?,
        colnect_paper: row.get(23)?,
        colnect_gum: row.get(24)?,
        colnect_ord_mi: row.get(25)?,
        colnect_ord_sn: row.get(26)?,
        colnect_ord_yt: row.get(27)?,
        colnect_face_value: row.get(28)?,
        colnect_currency: row.get(29)?,
        colnect_ord_sg: row.get(30)?,
        colnect_expiry_date: row.get(31)?,
        colnect_print_run: row.get(32)?,
        colnect_colors: row.get(33)?,
        colnect_parent_item: row.get(34)?,
        colnect_variant: row.get(35)?,
        colnect_overprint_variant: row.get(36)?,
        colnect_score: row.get(37)?,
        colnect_accuracy: row.get(38)?,
        colnect_list: row.get(39)?,
        colnect_quantity: row.get(40)?,
        colnect_condition: row.get(41)?,
        colnect_public_note: row.get(42)?,
        colnect_private_note: row.get(43)?,
        all_db_fields: load_stamp_all_fields(conn, resolved_rowid).unwrap_or_default(),
    }))
}

fn load_right_candidate_by_colnect_id(
    conn: &Connection,
    db_path: &str,
    album_name: &str,
    colnect_id: &str,
) -> Result<Option<RightCandidate>, Box<dyn std::error::Error>> {
    let mut code_stmt = conn.prepare(
        "SELECT COALESCE(catalog_type, ''), COALESCE(code, '')
         FROM colnect_catalog_codes
         WHERE COALESCE(colnect_id, '') = ?1",
    )?;
    let code_rows = code_stmt.query_map(params![colnect_id], |row| {
        Ok(CatalogCodePair {
            catalog_type: row.get(0)?,
            code: row.get(1)?,
        })
    })?;
    let mut code_pairs = Vec::new();
    for r in code_rows {
        let p = r?;
        if p.code.trim().is_empty() {
            continue;
        }
        code_pairs.push(p);
    }

    let mut stmt = conn.prepare(
        "SELECT COALESCE(c.colnect_id, ''),
                COALESCE(c.sg_number, ''),
                CAST(substr(COALESCE(c.issued_on, ''), 1, 4) AS INTEGER),
                COALESCE(c.series, ''),
                COALESCE(c.colnect_link, ''),
                COALESCE(c.description, ''),
                COALESCE(c.name, ''),
                COALESCE(c.watermark, ''),
                COALESCE(c.designers, ''),
                COALESCE(c.printers, ''),
                COALESCE(c.perforation, ''),
                COALESCE(c.themes, ''),
                COALESCE(c.catalog_codes, ''),
                COALESCE(c.country, ''),
                COALESCE(c.width, ''),
                COALESCE(c.height, ''),
                COALESCE(c.format, ''),
                COALESCE(c.emission, ''),
                COALESCE(c.printing, ''),
                COALESCE(c.paper, ''),
                COALESCE(c.gum, ''),
                COALESCE(c.ord_mi, ''),
                COALESCE(c.ord_sn, ''),
                COALESCE(c.ord_yt, ''),
                COALESCE(c.ord_sg, ''),
                COALESCE(c.expiry_date, ''),
                COALESCE(c.print_run, ''),
                COALESCE(c.face_value, ''),
                COALESCE(c.currency, ''),
                COALESCE(c.colors, ''),
                COALESCE(c.parent_item, ''),
                COALESCE(c.variant, ''),
                COALESCE(c.overprint_variant, ''),
                COALESCE(c.score, ''),
                COALESCE(c.accuracy, ''),
                COALESCE(c.list, ''),
                COALESCE(c.quantity, ''),
                COALESCE(c.condition, ''),
                COALESCE(c.public_note, ''),
                COALESCE(c.private_note, ''),
                COALESCE((
                    SELECT s.image_path
                    FROM stamps s
                    WHERE s.sg_number = c.sg_number
                      AND s.year = CAST(substr(COALESCE(c.issued_on, ''), 1, 4) AS INTEGER)
                      AND TRIM(COALESCE(s.image_path, '')) != ''
                    LIMIT 1
                ), '')
                 FROM colnect_stamps c
                 WHERE COALESCE(c.colnect_id, '') = ?1
                     AND COALESCE(c.is_deleted, 0) = 0
                     AND COALESCE(c.is_processed, 0) = 0
         LIMIT 1",
    )?;

    let mut rows = stmt.query(params![colnect_id])?;
    let Some(row) = rows.next()? else {
        return Ok(None);
    };

    let page_url: String = row.get(4)?;
    let image_url = if !page_url.trim().is_empty() {
        crate::colnect::image_fetcher::resolve_colnect_image_url(&page_url).unwrap_or_default()
    } else {
        String::new()
    };
    let description = {
        let d: String = row.get(5)?;
        if d.trim().is_empty() {
            row.get::<_, String>(6)?
        } else {
            d
        }
    };
    let image_path: String = row.get(40)?;

    Ok(Some(RightCandidate {
        colnect_id: row.get(0)?,
        sg_number: row.get(1)?,
        year: row.get(2)?,
        series: row.get(3)?,
        page_url,
        image_url,
        image_path: image_path.clone(),
        image_abs: resolve_image_abs(db_path, album_name, &image_path),
        description,
        watermark: row.get(7)?,
        design: row.get(8)?,
        engraving: row.get(9)?,
        perforation: row.get(10)?,
        context: row.get(11)?,
        name: row.get(6)?,
        catalog_codes_text: row.get(12)?,
        country: row.get(13)?,
        width: row.get(14)?,
        height: row.get(15)?,
        format: row.get(16)?,
        emission: row.get(17)?,
        printing: row.get(18)?,
        paper: row.get(19)?,
        gum: row.get(20)?,
        ord_mi: row.get(21)?,
        ord_sn: row.get(22)?,
        ord_yt: row.get(23)?,
        ord_sg: row.get(24)?,
        expiry_date: row.get(25)?,
        print_run: row.get(26)?,
        face_value: row.get(27)?,
        currency: row.get(28)?,
        colors: row.get(29)?,
        parent_item: row.get(30)?,
        variant: row.get(31)?,
        overprint_variant: row.get(32)?,
        score: row.get(33)?,
        accuracy: row.get(34)?,
        list: row.get(35)?,
        quantity: row.get(36)?,
        condition: row.get(37)?,
        public_note: row.get(38)?,
        private_note: row.get(39)?,
        catalog_code_pairs: code_pairs,
    }))
}

fn add_colnect_stamp_to_stamps(
    conn: &Connection,
    db_path: &str,
    album_name: &str,
    req: AddColnectStampRequest,
) -> AddColnectStampResponse {
    if req.colnect_id.trim().is_empty() {
        return AddColnectStampResponse {
            ok: false,
            message: "colnect_id is required".to_string(),
            added_left: None,
            processed_colnect_id: None,
        };
    }

    let candidate =
        match load_right_candidate_by_colnect_id(conn, db_path, album_name, &req.colnect_id) {
            Ok(Some(c)) => c,
            Ok(None) => {
                return AddColnectStampResponse {
                    ok: false,
                    message: format!("colnect candidate not found for id={}", req.colnect_id),
                    added_left: None,
                    processed_colnect_id: None,
                }
            }
            Err(e) => {
                return AddColnectStampResponse {
                    ok: false,
                    message: format!("failed to load colnect candidate: {}", e),
                    added_left: None,
                    processed_colnect_id: None,
                }
            }
        };

    let existing = conn.query_row(
        "SELECT COUNT(*) FROM stamps WHERE TRIM(COALESCE(colnect_id, '')) = ?1",
        params![candidate.colnect_id.clone()],
        |row| row.get::<_, i64>(0),
    );
    if let Ok(count) = existing {
        if count > 0 {
            return AddColnectStampResponse {
                ok: false,
                message: format!(
                    "stamp with colnect_id={} already exists in stamps",
                    candidate.colnect_id
                ),
                added_left: None,
                processed_colnect_id: None,
            };
        }
    }

    let mut image_path = String::new();
    if req.associate_image {
        let mode = req
            .image_mode
            .as_deref()
            .unwrap_or("current_left")
            .to_ascii_lowercase();
        if mode == "current_left" {
            if let Some(left_rowid) = req.source_left_rowid {
                match conn.query_row(
                    "SELECT COALESCE(image_path, '') FROM stamps WHERE rowid = ?1",
                    params![left_rowid],
                    |row| row.get::<_, String>(0),
                ) {
                    Ok(v) if !v.trim().is_empty() => {
                        image_path = v;
                    }
                    Ok(_) => {}
                    Err(e) => {
                        return AddColnectStampResponse {
                            ok: false,
                            message: format!(
                                "failed reading image from source row {}: {}",
                                left_rowid, e
                            ),
                            added_left: None,
                            processed_colnect_id: None,
                        };
                    }
                }
            }
        } else if mode == "upload" {
            let Some(upload_data_url) = req.upload_data_url.as_deref() else {
                return AddColnectStampResponse {
                    ok: false,
                    message: "upload mode selected but no upload data provided".to_string(),
                    added_left: None,
                    processed_colnect_id: None,
                };
            };
            match store_uploaded_image_from_data_url(
                db_path,
                album_name,
                &candidate.colnect_id,
                upload_data_url,
                req.upload_filename.as_deref(),
            ) {
                Ok(rel_path) => image_path = rel_path,
                Err(e) => {
                    return AddColnectStampResponse {
                        ok: false,
                        message: e,
                        added_left: None,
                        processed_colnect_id: None,
                    }
                }
            }
        }
    }

    let default_catalog_code =
        load_album_config(db_path, album_name).effective_default_catalog_code();
    let sg_number = crate::catalog::normalize_catalog_number(
        &req.target_sg_number
            .unwrap_or_else(|| candidate.sg_number.clone()),
    );
    let catalog_code = crate::catalog::infer_catalog_type_for_number(
        &candidate.catalog_codes_text,
        &sg_number,
        &default_catalog_code,
    );
    let year = req.target_year.unwrap_or(candidate.year);
    let series = req
        .target_series
        .unwrap_or_else(|| candidate.series.clone());

    let insert_res = conn.execute(
        "INSERT INTO stamps (
            country, series, year, sg_number, catalog_code, uniq_key,
            description, image_path, remote_image_url, original_image_url,
            colnect_page_url, colnect_image_url, colnect_id, colnect_name,
            colnect_catalog_codes, colnect_country, colnect_width, colnect_height,
            colnect_format, colnect_emission, colnect_printing, colnect_paper,
            colnect_gum, colnect_ord_mi, colnect_ord_sn, colnect_ord_yt,
            colnect_ord_sg, colnect_expiry_date, colnect_print_run, colnect_face_value,
            colnect_currency, colnect_colors, colnect_parent_item, colnect_variant,
            colnect_overprint_variant, colnect_score, colnect_accuracy, colnect_list,
            colnect_quantity, colnect_condition, colnect_public_note, colnect_private_note,
            watermark, design, engraving, perforation, context
         ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6,
                ?7, ?8, ?9, ?10,
                ?11, ?12, ?13, ?14,
                ?15, ?16, ?17, ?18,
                ?19, ?20, ?21, ?22,
                ?23, ?24, ?25, ?26,
                ?27, ?28, ?29, ?30,
                ?31, ?32, ?33, ?34,
                ?35, ?36, ?37, ?38,
                ?39, ?40, ?41, ?42,
                ?43, ?44, ?45, ?46, ?47
         )",
        params![
            candidate.country,
            series.clone(),
            year,
            sg_number.clone(),
            catalog_code,
            format!(
                "{}|{}|{}",
                candidate.country.trim().to_ascii_lowercase(),
                series,
                sg_number
            ),
            candidate.description,
            image_path,
            candidate.image_url,
            candidate.page_url,
            candidate.page_url,
            candidate.image_url,
            candidate.colnect_id,
            candidate.name,
            candidate.catalog_codes_text,
            candidate.country,
            candidate.width,
            candidate.height,
            candidate.format,
            candidate.emission,
            candidate.printing,
            candidate.paper,
            candidate.gum,
            candidate.ord_mi,
            candidate.ord_sn,
            candidate.ord_yt,
            candidate.ord_sg,
            candidate.expiry_date,
            candidate.print_run,
            candidate.face_value,
            candidate.currency,
            candidate.colors,
            candidate.parent_item,
            candidate.variant,
            candidate.overprint_variant,
            candidate.score,
            candidate.accuracy,
            candidate.list,
            candidate.quantity,
            candidate.condition,
            candidate.public_note,
            candidate.private_note,
            candidate.watermark,
            candidate.design,
            candidate.engraving,
            candidate.perforation,
            candidate.context,
        ],
    );

    if let Err(e) = insert_res {
        return AddColnectStampResponse {
            ok: false,
            message: format!("failed to insert into stamps: {}", e),
            added_left: None,
            processed_colnect_id: None,
        };
    }

    let rowid = conn.last_insert_rowid();
    let added_left = load_left_stamp_by_rowid(conn, db_path, album_name, rowid)
        .ok()
        .flatten();

    let processed_colnect_id = match mark_colnect_candidate_processed(conn, &req.colnect_id) {
        Ok(_) => Some(req.colnect_id.clone()),
        Err(_) => None,
    };

    let message = if processed_colnect_id.is_some() {
        format!(
            "added colnect_id={} into stamps rowid={}",
            req.colnect_id, rowid
        )
    } else {
        format!(
            "added colnect_id={} into stamps rowid={} (warning: candidate was not marked processed until next UI sync)",
            req.colnect_id, rowid
        )
    };

    AddColnectStampResponse {
        ok: true,
        message,
        added_left,
        processed_colnect_id,
    }
}

fn restore_colnect_candidate_to_queue(
    conn: &Connection,
    colnect_id: &str,
) -> rusqlite::Result<usize> {
    if colnect_id.trim().is_empty() {
        return Ok(0);
    }

    conn.execute(
        "UPDATE colnect_stamps
         SET is_processed = 0
         WHERE TRIM(COALESCE(colnect_id, '')) = TRIM(?1)",
        params![colnect_id],
    )
}

fn query_stamp_image_path(conn: &Connection, rowid: i64) -> Result<String, String> {
    conn.query_row(
        "SELECT COALESCE(image_path, '') FROM stamps WHERE rowid = ?1",
        params![rowid],
        |row| row.get::<_, String>(0),
    )
    .map_err(|e| format!("failed to read image path from row {}: {}", rowid, e))
}

fn resolve_left_editor_image_path(
    conn: &Connection,
    db_path: &str,
    album_name: &str,
    image_mode: Option<&str>,
    source_rowid: Option<i64>,
    current_image_path: Option<&str>,
    upload_data_url: Option<&str>,
    upload_filename: Option<&str>,
    file_token: &str,
) -> Result<String, String> {
    let mode = image_mode
        .unwrap_or(if current_image_path.is_some() {
            "keep"
        } else {
            "current_left"
        })
        .trim()
        .to_ascii_lowercase();

    match mode.as_str() {
        "keep" => Ok(current_image_path.unwrap_or("").trim().to_string()),
        "current_left" => {
            let Some(rowid) = source_rowid else {
                return Err("current-left image mode requires a source row".to_string());
            };
            let image_path = query_stamp_image_path(conn, rowid)?;
            if image_path.trim().is_empty() {
                return Err(format!("source row {} has no image to reuse", rowid));
            }
            Ok(image_path)
        }
        "upload" => {
            let Some(data_url) = upload_data_url else {
                return Err("upload image mode requires an uploaded file".to_string());
            };
            store_uploaded_image_from_data_url(
                db_path,
                album_name,
                file_token,
                data_url,
                upload_filename,
            )
        }
        other => Err(format!("unsupported image mode: {}", other)),
    }
}

fn save_left_stamp(
    conn: &Connection,
    db_path: &str,
    album_name: &str,
    req: SaveLeftStampRequest,
) -> SaveLeftStampResponse {
    if req.rowid <= 0 {
        return SaveLeftStampResponse {
            ok: false,
            message: "rowid is required".to_string(),
            updated_left: None,
        };
    }
    if req.sg_number.trim().is_empty() {
        return SaveLeftStampResponse {
            ok: false,
            message: "SG / reference code is required".to_string(),
            updated_left: None,
        };
    }

    let existing_image_path = match query_stamp_image_path(conn, req.rowid) {
        Ok(value) => value,
        Err(message) => {
            return SaveLeftStampResponse {
                ok: false,
                message,
                updated_left: None,
            }
        }
    };

    let image_path = match resolve_left_editor_image_path(
        conn,
        db_path,
        album_name,
        req.image_mode.as_deref(),
        Some(req.rowid),
        Some(existing_image_path.as_str()),
        req.upload_data_url.as_deref(),
        req.upload_filename.as_deref(),
        &req.sg_number,
    ) {
        Ok(value) => value,
        Err(message) => {
            return SaveLeftStampResponse {
                ok: false,
                message,
                updated_left: None,
            }
        }
    };

    let resolved_colnect_width = if req.colnect_width.trim().is_empty() {
        req.width.trim().to_string()
    } else {
        req.colnect_width.trim().to_string()
    };
    let resolved_colnect_height = if req.colnect_height.trim().is_empty() {
        req.height.trim().to_string()
    } else {
        req.colnect_height.trim().to_string()
    };

    let update_res = conn.execute(
        "UPDATE stamps
         SET sg_number = ?1,
             year = ?2,
             series = ?3,
             description = ?4,
             watermark = ?5,
             design = ?6,
             engraving = ?7,
             perforation = ?8,
             context = ?9,
             image_path = ?10,
             colnect_page_url = ?11,
             colnect_image_url = ?12,
             colnect_id = ?13,
             colnect_name = ?14,
             colnect_catalog_codes = ?15,
             colnect_country = ?16,
             colnect_width = ?17,
             colnect_height = ?18,
             colnect_format = ?19,
             colnect_emission = ?20,
             colnect_printing = ?21,
             colnect_paper = ?22,
             colnect_gum = ?23,
             colnect_ord_mi = ?24,
             colnect_ord_sn = ?25,
             colnect_ord_yt = ?26,
             colnect_face_value = ?27,
             colnect_currency = ?28,
             colnect_ord_sg = ?29,
             colnect_expiry_date = ?30,
             colnect_print_run = ?31,
             colnect_colors = ?32,
             colnect_parent_item = ?33,
             colnect_variant = ?34,
             colnect_overprint_variant = ?35,
             colnect_score = ?36,
             colnect_accuracy = ?37,
             colnect_list = ?38,
             colnect_quantity = ?39,
             colnect_condition = ?40,
             colnect_public_note = ?41,
             colnect_private_note = ?42,
             uniq_key = lower(coalesce(country, '')) || '|' || ?3 || '|' || ?1
         WHERE rowid = ?43",
        params![
            req.sg_number.trim(),
            req.year,
            req.series.trim(),
            req.description.trim(),
            req.watermark.trim(),
            req.design.trim(),
            req.engraving.trim(),
            req.perforation.trim(),
            req.context.trim(),
            image_path,
            req.colnect_page_url.trim(),
            req.colnect_image_url.trim(),
            req.colnect_id.trim(),
            req.colnect_name.trim(),
            req.colnect_catalog_codes.trim(),
            req.colnect_country.trim(),
            resolved_colnect_width,
            resolved_colnect_height,
            req.colnect_format.trim(),
            req.colnect_emission.trim(),
            req.colnect_printing.trim(),
            req.colnect_paper.trim(),
            req.colnect_gum.trim(),
            req.colnect_ord_mi.trim(),
            req.colnect_ord_sn.trim(),
            req.colnect_ord_yt.trim(),
            req.colnect_face_value.trim(),
            req.colnect_currency.trim(),
            req.colnect_ord_sg.trim(),
            req.colnect_expiry_date.trim(),
            req.colnect_print_run.trim(),
            req.colnect_colors.trim(),
            req.colnect_parent_item.trim(),
            req.colnect_variant.trim(),
            req.colnect_overprint_variant.trim(),
            req.colnect_score.trim(),
            req.colnect_accuracy.trim(),
            req.colnect_list.trim(),
            req.colnect_quantity.trim(),
            req.colnect_condition.trim(),
            req.colnect_public_note.trim(),
            req.colnect_private_note.trim(),
            req.rowid,
        ],
    );

    match update_res {
        Ok(0) => SaveLeftStampResponse {
            ok: false,
            message: format!("no stamp row found for rowid={}", req.rowid),
            updated_left: None,
        },
        Ok(_) => SaveLeftStampResponse {
            ok: true,
            message: format!("updated base stamp rowid={}", req.rowid),
            updated_left: load_left_stamp_by_rowid(conn, db_path, album_name, req.rowid)
                .ok()
                .flatten(),
        },
        Err(e) => SaveLeftStampResponse {
            ok: false,
            message: format!("failed to update rowid={}: {}", req.rowid, e),
            updated_left: None,
        },
    }
}

fn add_left_stamp(
    conn: &Connection,
    db_path: &str,
    album_name: &str,
    req: AddLeftStampRequest,
) -> AddLeftStampResponse {
    if req.sg_number.trim().is_empty() {
        return AddLeftStampResponse {
            ok: false,
            message: "SG / reference code is required".to_string(),
            added_left: None,
        };
    }

    let country = req
        .source_rowid
        .and_then(|rowid| {
            conn.query_row(
                "SELECT COALESCE(country, '') FROM stamps WHERE rowid = ?1",
                params![rowid],
                |row| row.get::<_, String>(0),
            )
            .ok()
        })
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| album_name.to_string());

    let image_path = match resolve_left_editor_image_path(
        conn,
        db_path,
        album_name,
        req.image_mode.as_deref(),
        req.source_rowid,
        None,
        req.upload_data_url.as_deref(),
        req.upload_filename.as_deref(),
        &req.sg_number,
    ) {
        Ok(value) => value,
        Err(message) => {
            return AddLeftStampResponse {
                ok: false,
                message,
                added_left: None,
            }
        }
    };

    let catalog_code = load_album_config(db_path, album_name).effective_default_catalog_code();
    let series = req.series.trim().to_string();
    let sg_number = req.sg_number.trim().to_string();
    let uniq_key = format!(
        "{}|{}|{}",
        country.trim().to_ascii_lowercase(),
        series,
        sg_number
    );

    let resolved_colnect_width = if req.colnect_width.trim().is_empty() {
        req.width.trim().to_string()
    } else {
        req.colnect_width.trim().to_string()
    };
    let resolved_colnect_height = if req.colnect_height.trim().is_empty() {
        req.height.trim().to_string()
    } else {
        req.colnect_height.trim().to_string()
    };

    let insert_res = conn.execute(
        "INSERT INTO stamps (
            country, series, year, sg_number, catalog_code, uniq_key,
            description, image_path, watermark, design, engraving, perforation, context,
            colnect_page_url, colnect_image_url, colnect_id, colnect_name,
            colnect_catalog_codes, colnect_country, colnect_width, colnect_height,
            colnect_format, colnect_emission, colnect_printing, colnect_paper,
            colnect_gum, colnect_ord_mi, colnect_ord_sn, colnect_ord_yt,
            colnect_face_value, colnect_currency, colnect_ord_sg, colnect_expiry_date,
            colnect_print_run, colnect_colors, colnect_parent_item, colnect_variant,
            colnect_overprint_variant, colnect_score, colnect_accuracy, colnect_list,
            colnect_quantity, colnect_condition, colnect_public_note, colnect_private_note
         ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13,
            ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21,
            ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29,
            ?30, ?31, ?32, ?33, ?34, ?35, ?36, ?37,
                ?38, ?39, ?40, ?41, ?42, ?43, ?44, ?45
         )",
        params![
            country,
            series,
            req.year,
            sg_number,
            catalog_code,
            uniq_key,
            req.description.trim(),
            image_path,
            req.watermark.trim(),
            req.design.trim(),
            req.engraving.trim(),
            req.perforation.trim(),
            req.context.trim(),
            req.colnect_page_url.trim(),
            req.colnect_image_url.trim(),
            req.colnect_id.trim(),
            req.colnect_name.trim(),
            req.colnect_catalog_codes.trim(),
            req.colnect_country.trim(),
            resolved_colnect_width,
            resolved_colnect_height,
            req.colnect_format.trim(),
            req.colnect_emission.trim(),
            req.colnect_printing.trim(),
            req.colnect_paper.trim(),
            req.colnect_gum.trim(),
            req.colnect_ord_mi.trim(),
            req.colnect_ord_sn.trim(),
            req.colnect_ord_yt.trim(),
            req.colnect_face_value.trim(),
            req.colnect_currency.trim(),
            req.colnect_ord_sg.trim(),
            req.colnect_expiry_date.trim(),
            req.colnect_print_run.trim(),
            req.colnect_colors.trim(),
            req.colnect_parent_item.trim(),
            req.colnect_variant.trim(),
            req.colnect_overprint_variant.trim(),
            req.colnect_score.trim(),
            req.colnect_accuracy.trim(),
            req.colnect_list.trim(),
            req.colnect_quantity.trim(),
            req.colnect_condition.trim(),
            req.colnect_public_note.trim(),
            req.colnect_private_note.trim(),
        ],
    );

    match insert_res {
        Ok(_) => {
            let rowid = conn.last_insert_rowid();
            AddLeftStampResponse {
                ok: true,
                message: format!("added base/reference item rowid={}", rowid),
                added_left: load_left_stamp_by_rowid(conn, db_path, album_name, rowid)
                    .ok()
                    .flatten(),
            }
        }
        Err(e) => AddLeftStampResponse {
            ok: false,
            message: format!("failed to add base/reference item: {}", e),
            added_left: None,
        },
    }
}

fn delete_left_stamp(conn: &Connection, req: DeleteLeftStampRequest) -> DeleteLeftStampResponse {
    if req.rowid <= 0 {
        return DeleteLeftStampResponse {
            ok: false,
            message: "rowid is required".to_string(),
            deleted_rowid: None,
            restored_colnect_id: None,
        };
    }

    let linked_colnect_id = conn
        .query_row(
            "SELECT COALESCE(colnect_id, '') FROM stamps WHERE rowid = ?1",
            params![req.rowid],
            |row| row.get::<_, String>(0),
        )
        .unwrap_or_default();

    match conn.execute("DELETE FROM stamps WHERE rowid = ?1", params![req.rowid]) {
        Ok(0) => DeleteLeftStampResponse {
            ok: false,
            message: format!("no stamp row found for rowid={}", req.rowid),
            deleted_rowid: None,
            restored_colnect_id: None,
        },
        Ok(_) => {
            let restored_colnect_id = if linked_colnect_id.trim().is_empty() {
                None
            } else {
                let _ = restore_colnect_candidate_to_queue(conn, &linked_colnect_id);
                Some(linked_colnect_id.clone())
            };
            DeleteLeftStampResponse {
                ok: true,
                message: format!("deleted base stamp rowid={}", req.rowid),
                deleted_rowid: Some(req.rowid),
                restored_colnect_id,
            }
        }
        Err(e) => DeleteLeftStampResponse {
            ok: false,
            message: format!("failed to delete rowid={}: {}", req.rowid, e),
            deleted_rowid: None,
            restored_colnect_id: None,
        },
    }
}

fn clear_left_colnect_link(
    conn: &Connection,
    db_path: &str,
    album_name: &str,
    req: ClearLeftColnectLinkRequest,
) -> ClearLeftColnectLinkResponse {
    if req.rowid <= 0 {
        return ClearLeftColnectLinkResponse {
            ok: false,
            message: "rowid is required".to_string(),
            updated_left: None,
            restored_colnect_id: None,
        };
    }

    let linked_colnect_id = conn
        .query_row(
            "SELECT COALESCE(colnect_id, '') FROM stamps WHERE rowid = ?1",
            params![req.rowid],
            |row| row.get::<_, String>(0),
        )
        .unwrap_or_default();

    let update_res = conn.execute(
        "UPDATE stamps
         SET colnect_page_url = '',
             colnect_image_url = '',
             colnect_id = '',
             colnect_name = '',
             colnect_catalog_codes = '',
             colnect_country = '',
             colnect_width = '',
             colnect_height = '',
             colnect_format = '',
             colnect_emission = '',
             colnect_printing = '',
             colnect_paper = '',
             colnect_gum = '',
             colnect_ord_mi = '',
             colnect_ord_sn = '',
             colnect_ord_yt = '',
             colnect_face_value = '',
             colnect_currency = '',
             colnect_ord_sg = '',
             colnect_expiry_date = '',
             colnect_print_run = '',
             colnect_colors = '',
             colnect_parent_item = '',
             colnect_variant = '',
             colnect_overprint_variant = '',
             colnect_score = '',
             colnect_accuracy = '',
             colnect_list = '',
             colnect_quantity = '',
             colnect_condition = '',
             colnect_public_note = '',
             colnect_private_note = ''
         WHERE rowid = ?1",
        params![req.rowid],
    );

    match update_res {
        Ok(0) => ClearLeftColnectLinkResponse {
            ok: false,
            message: format!("no stamp row found for rowid={}", req.rowid),
            updated_left: None,
            restored_colnect_id: None,
        },
        Ok(_) => {
            let restored_colnect_id = if linked_colnect_id.trim().is_empty() {
                None
            } else {
                let _ = restore_colnect_candidate_to_queue(conn, &linked_colnect_id);
                Some(linked_colnect_id.clone())
            };
            ClearLeftColnectLinkResponse {
                ok: true,
                message: if let Some(colnect_id) = restored_colnect_id.as_ref() {
                    format!(
                        "cleared linked Colnect data from rowid={} and returned {} to the candidate list",
                        req.rowid, colnect_id
                    )
                } else {
                    format!("rowid={} had no linked Colnect data to clear", req.rowid)
                },
                updated_left: load_left_stamp_by_rowid(conn, db_path, album_name, req.rowid)
                    .ok()
                    .flatten(),
                restored_colnect_id,
            }
        }
        Err(e) => ClearLeftColnectLinkResponse {
            ok: false,
            message: format!(
                "failed clearing Colnect data for rowid={}: {}",
                req.rowid, e
            ),
            updated_left: None,
            restored_colnect_id: None,
        },
    }
}

fn delete_colnect_candidate(
    conn: &Connection,
    req: DeleteColnectCandidateRequest,
) -> DeleteColnectCandidateResponse {
    if req.colnect_id.trim().is_empty() {
        return DeleteColnectCandidateResponse {
            ok: false,
            message: "colnect_id is required".to_string(),
            deleted_stamp_rows: 0,
            deleted_code_rows: 0,
        };
    }

    let deleted_stamp_rows = match conn.execute(
        "UPDATE colnect_stamps
         SET is_deleted = 1,
             deleted_at = ?2
         WHERE COALESCE(colnect_id, '') = ?1
           AND COALESCE(is_deleted, 0) = 0",
        params![req.colnect_id.clone(), chrono::Utc::now().to_rfc3339()],
    ) {
        Ok(n) => n,
        Err(e) => {
            return DeleteColnectCandidateResponse {
                ok: false,
                message: format!("failed soft-deleting colnect candidate: {}", e),
                deleted_stamp_rows: 0,
                deleted_code_rows: 0,
            };
        }
    };

    DeleteColnectCandidateResponse {
        ok: true,
        message: format!(
            "soft-deleted colnect_id={} (rows={})",
            req.colnect_id, deleted_stamp_rows
        ),
        deleted_stamp_rows,
        deleted_code_rows: 0,
    }
}

fn undo_colnect_delete(
    conn: &Connection,
    req: UndoColnectDeleteRequest,
) -> UndoColnectDeleteResponse {
    if req.colnect_id.trim().is_empty() {
        return UndoColnectDeleteResponse {
            ok: false,
            message: "colnect_id is required".to_string(),
            restored_rows: 0,
        };
    }

    let restored_rows = match conn.execute(
        "UPDATE colnect_stamps
         SET is_deleted = 0,
             deleted_at = ''
         WHERE COALESCE(colnect_id, '') = ?1
           AND COALESCE(is_deleted, 0) = 1",
        params![req.colnect_id.clone()],
    ) {
        Ok(n) => n,
        Err(e) => {
            return UndoColnectDeleteResponse {
                ok: false,
                message: format!("failed undoing soft delete: {}", e),
                restored_rows: 0,
            };
        }
    };

    UndoColnectDeleteResponse {
        ok: true,
        message: format!(
            "restored colnect_id={} (rows={})",
            req.colnect_id, restored_rows
        ),
        restored_rows,
    }
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

    let mut selected: Option<(String, String)> = None;
    let mut logout_requested = false;
    let mut reload_requested = false;

    for mut request in server.incoming_requests() {
        let method = request.method().clone();
        let req_url = request.url().to_string();

        if method == Method::Get && req_url == "/" {
            let mut resp = Response::from_string(html.clone());
            if let Ok(h) =
                Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..])
            {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Get && req_url == "/bootstrap-state" {
            let state = BootstrapStateResponse {
                countries: scan_bootstrap_countries(),
            };
            let mut resp = Response::from_string(
                serde_json::to_string(&state).unwrap_or_else(|_| "{\"countries\":[]}".to_string()),
            );
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Get && req_url == "/api/config-editor" {
            let resp_json = match build_config_editor_data("config.toml", "config_template.toml") {
                Ok(sections) => serde_json::to_string(&ConfigEditorDataResponse {
                    ok: true,
                    message: "Config editor data loaded".to_string(),
                    sections,
                    album_name: String::new(),
                    has_album_override: false,
                })
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"serialization failed\",\"sections\":[],\"album_name\":\"\",\"has_album_override\":false}".to_string()
                }),
                Err(error) => serde_json::to_string(&ConfigEditorDataResponse {
                    ok: false,
                    message: format!("Failed to load config editor data: {}", error),
                    sections: Vec::new(),
                    album_name: String::new(),
                    has_album_override: false,
                })
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"failed to load config editor data\",\"sections\":[],\"album_name\":\"\",\"has_album_override\":false}".to_string()
                }),
            };

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/api/config-editor-save" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<SaveConfigEditorRequest, _> = serde_json::from_str(&body);
            let response = match parsed {
                Ok(req) => match save_full_config_values(
                    "config.toml",
                    "config_template.toml",
                    &req.values,
                ) {
                    Ok(_) => {
                        reload_requested = true;
                        SaveConfigEditorResponse {
                            ok: true,
                            message: "config.toml updated. Reloading the review UI now."
                                .to_string(),
                            reload_required: true,
                        }
                    }
                    Err(error) => SaveConfigEditorResponse {
                        ok: false,
                        message: format!("Failed to save config.toml: {}", error),
                        reload_required: false,
                    },
                },
                Err(error) => SaveConfigEditorResponse {
                    ok: false,
                    message: format!("invalid request: {}", error),
                    reload_required: false,
                },
            };

            let mut resp =
                Response::from_string(serde_json::to_string(&response).unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"serialization failed\",\"reload_required\":false}"
                        .to_string()
                }));
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            if reload_requested {
                break;
            }
            continue;
        }

        if method == Method::Post && req_url == "/start-review" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<BootstrapStartRequest, _> = serde_json::from_str(&body);
            let response = match parsed {
                Ok(req) => {
                    let countries = scan_bootstrap_countries();
                    let found = countries
                        .into_iter()
                        .find(|c| c.country.eq_ignore_ascii_case(req.country.trim()));
                    match found {
                        Some(c) => {
                            eprintln!(
                                "Review UI: country selected '{}'; preparing compare view from {}",
                                c.country, c.db_path
                            );
                            selected = Some((c.db_path.clone(), c.country.clone()));
                            BootstrapStartResponse {
                                ok: true,
                                message: format!(
                                    "Selected {}. Preparing Review UI now...",
                                    c.country
                                ),
                                country: Some(c.country),
                                db_path: Some(c.db_path),
                            }
                        }
                        None => BootstrapStartResponse {
                            ok: false,
                            message: "Selected country not found or missing DB".to_string(),
                            country: None,
                            db_path: None,
                        },
                    }
                }
                Err(e) => BootstrapStartResponse {
                    ok: false,
                    message: format!("invalid request: {}", e),
                    country: None,
                    db_path: None,
                },
            };

            let mut resp = Response::from_string(
                serde_json::to_string(&response).unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"serialization failed\",\"country\":null,\"db_path\":null}"
                        .to_string()
                }),
            );
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);

            if selected.is_some() {
                break;
            }
            continue;
        }

        if method == Method::Post && req_url == "/logout" {
            let response = serde_json::json!({
                "ok": true,
                "message": "Review UI startup closed"
            });
            let mut resp = Response::from_string(response.to_string());
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            logout_requested = true;
            break;
        }

        if method == Method::Post && req_url == "/api/generate-pdf-now" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let Some(_pdf_generation_guard) = PdfGenerationGuard::try_acquire() else {
                let resp_json = serde_json::to_string(&BootstrapGeneratePdfResponse {
                    ok: false,
                    message:
                        "PDF generation already in progress. Please wait for it to complete."
                            .to_string(),
                    generated_output_path: None,
                    generated_pdf_path: None,
                })
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"PDF generation already in progress\",\"generated_output_path\":null,\"generated_pdf_path\":null}"
                        .to_string()
                });

                let mut resp = Response::from_string(resp_json).with_status_code(StatusCode(409));
                if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                    resp = resp.with_header(h);
                }
                let _ = request.respond(resp);
                continue;
            };

            let parsed: Result<BootstrapGeneratePdfRequest, _> = serde_json::from_str(&body);
            let resp_json = match parsed {
                Ok(req) => {
                    let countries = scan_bootstrap_countries();
                    let found = countries
                        .into_iter()
                        .find(|c| c.country.eq_ignore_ascii_case(req.country.trim()));
                    match found {
                        Some(c) => {
                            match crate::db_utils::init_db(&c.db_path) {
                                Ok(conn) => {
                                    let artifacts = crate::album_globals::generate_album_output_conn_with_pdf_override(
                                        &conn,
                                        &c.country,
                                        None::<fn(usize)>,
                                        req.start_year,
                                        req.end_year,
                                        Some(true),
                                    );
                                    if let Some(pdf_path) = artifacts.generated_pdf_path.as_ref() {
                                        open_local_path(pdf_path);
                                    }

                                    serde_json::to_string(&BootstrapGeneratePdfResponse {
                                        ok: true,
                                        message: format!("Generated album output for {}", c.country),
                                        generated_output_path: Some(artifacts.output_txt_path.display().to_string()),
                                        generated_pdf_path: artifacts
                                            .generated_pdf_path
                                            .as_ref()
                                            .map(|p| p.display().to_string()),
                                    })
                                    .unwrap_or_else(|_| {
                                        "{\"ok\":false,\"message\":\"serialization failed\",\"generated_output_path\":null,\"generated_pdf_path\":null}"
                                            .to_string()
                                    })
                                }
                                Err(e) => serde_json::to_string(&BootstrapGeneratePdfResponse {
                                    ok: false,
                                    message: format!("failed to open DB {}: {}", c.db_path, e),
                                    generated_output_path: None,
                                    generated_pdf_path: None,
                                })
                                .unwrap_or_else(|_| {
                                    "{\"ok\":false,\"message\":\"failed to open DB\",\"generated_output_path\":null,\"generated_pdf_path\":null}"
                                        .to_string()
                                }),
                            }
                        }
                        None => serde_json::to_string(&BootstrapGeneratePdfResponse {
                            ok: false,
                            message: "Selected country not found or missing DB".to_string(),
                            generated_output_path: None,
                            generated_pdf_path: None,
                        })
                        .unwrap_or_else(|_| {
                            "{\"ok\":false,\"message\":\"selected country not found\",\"generated_output_path\":null,\"generated_pdf_path\":null}"
                                .to_string()
                        }),
                    }
                }
                Err(e) => serde_json::to_string(&BootstrapGeneratePdfResponse {
                    ok: false,
                    message: format!("invalid request: {}", e),
                    generated_output_path: None,
                    generated_pdf_path: None,
                })
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"invalid request\",\"generated_output_path\":null,\"generated_pdf_path\":null}"
                        .to_string()
                }),
            };

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/api/colnect-import" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<ColnectImportRequest, _> = serde_json::from_str(&body);
            let resp_json = match parsed {
                    Ok(_req) => serde_json::to_string(&ColnectImportResponse {
                        ok: false,
                        message: "Bootstrap stage: please select a country and open Review UI first, then use Import Colnect".to_string(),
                        files_processed: 0,
                        stamps_imported: 0,
                        errors: vec!["Import not available in bootstrap stage".to_string()],
                    })
                    .unwrap_or_else(|_| {
                        "{\"ok\":false,\"message\":\"serialization failed\",\"files_processed\":0,\"stamps_imported\":0,\"errors\":[]}"
                            .to_string()
                    }),
                    Err(e) => serde_json::to_string(&ColnectImportResponse {
                        ok: false,
                        message: format!("invalid request: {}", e),
                        files_processed: 0,
                        stamps_imported: 0,
                        errors: vec![format!("invalid request: {}", e)],
                    })
                    .unwrap_or_else(|_| {
                        "{\"ok\":false,\"message\":\"invalid request\",\"files_processed\":0,\"stamps_imported\":0,\"errors\":[]}"
                            .to_string()
                    }),
                };

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        let _ = request.respond(Response::empty(StatusCode(404)));
    }

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

    let mut reload_requested = false;
    let mut return_to_start_requested = false;

    for mut request in server.incoming_requests() {
        let method = request.method().clone();
        let req_url = request.url().to_string();

        if method == Method::Get && req_url == "/compare-ready" {
            let mut resp = Response::from_string("{\"ready\":true}".to_string());
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Get && req_url == "/api/config-editor" {
            let album_dir = album_dir_for_ui(db_path, &album_name);
            let resp_json = match build_config_editor_data_for_album(
                "config.toml",
                "config_template.toml",
                &album_dir,
            ) {
                Ok((sections, has_override)) => serde_json::to_string(&ConfigEditorDataResponse {
                    ok: true,
                    message: "Config editor data loaded".to_string(),
                    sections,
                    album_name: album_name.clone(),
                    has_album_override: has_override,
                })
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"serialization failed\",\"sections\":[],\"album_name\":\"\",\"has_album_override\":false}".to_string()
                }),
                Err(error) => serde_json::to_string(&ConfigEditorDataResponse {
                    ok: false,
                    message: format!("Failed to load config editor data: {}", error),
                    sections: Vec::new(),
                    album_name: album_name.clone(),
                    has_album_override: false,
                })
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"failed to load config editor data\",\"sections\":[],\"album_name\":\"\",\"has_album_override\":false}".to_string()
                }),
            };

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Get && (req_url == "/" || req_url.starts_with("/?")) {
            let show_imageless_left_rows = parse_query_flag(&req_url, "show_no_image");
            let html = if show_imageless_left_rows {
                match build_matcher_payload(
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
                    show_imageless_left_rows,
                ) {
                    Ok(payload) => render_html(&payload),
                    Err(error) => {
                        eprintln!(
                            "Review UI: failed to reload compare payload with image-less rows: {}",
                            error
                        );
                        html.clone()
                    }
                }
            } else {
                html.clone()
            };

            let mut resp = Response::from_string(html);
            if let Ok(h) =
                Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..])
            {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Get && req_url.starts_with("/image?") {
            let Some(path) = parse_query_param(&req_url, "path") else {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            };
            let p = PathBuf::from(path);
            if !p.exists() {
                let _ = request.respond(Response::empty(StatusCode(404)));
                continue;
            }
            match fs::File::open(&p) {
                Ok(f) => {
                    let mut resp = Response::from_file(f);
                    if let Ok(h) =
                        Header::from_bytes(&b"Content-Type"[..], mime_from_path(&p).as_bytes())
                    {
                        resp = resp.with_header(h);
                    }
                    let _ = request.respond(resp);
                }
                Err(_) => {
                    let _ = request.respond(Response::empty(StatusCode(500)));
                }
            }
            continue;
        }

        if method == Method::Get && req_url.starts_with("/resolve-colnect-image?") {
            let Some(page_url) = parse_query_param(&req_url, "page_url") else {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            };

            let image_url = crate::colnect::image_fetcher::resolve_colnect_image_url(&page_url)
                .unwrap_or_default();
            let resp_json = serde_json::json!({
                "ok": !image_url.trim().is_empty(),
                "image_url": image_url,
            })
            .to_string();

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/apply-matches" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<ApplyMatchesEnvelope, _> = serde_json::from_str(&body);
            let resp_json = match parsed {
                Ok(req) => serde_json::to_string(&apply_matches(
                    &conn,
                    &db_path,
                    &album_name,
                    req.matches,
                    req.fields,
                    &right_map,
                ))
                .unwrap_or_else(|_| {
                    "{\"updated_rows\":0,\"skipped_rows\":0,\"processed_colnect_ids\":[],\"errors\":[\"serialization failed\"]}"
                        .to_string()
                }),
                Err(e) => serde_json::to_string(&ApplyMatchesResponse {
                    updated_rows: 0,
                    skipped_rows: 0,
                    processed_colnect_ids: Vec::new(),
                    errors: vec![format!("invalid request: {}", e)],
                })
                .unwrap_or_else(|_| {
                    "{\"updated_rows\":0,\"skipped_rows\":0,\"processed_colnect_ids\":[],\"errors\":[\"invalid request\"]}"
                        .to_string()
                }),
            };

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/save-left-stamp" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<SaveLeftStampRequest, _> = serde_json::from_str(&body);
            let resp_json = match parsed {
                Ok(req) => serde_json::to_string(&save_left_stamp(
                    &conn,
                    db_path,
                    &album_name,
                    req,
                ))
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"serialization failed\",\"updated_left\":null}"
                        .to_string()
                }),
                Err(e) => serde_json::to_string(&SaveLeftStampResponse {
                    ok: false,
                    message: format!("invalid request: {}", e),
                    updated_left: None,
                })
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"invalid request\",\"updated_left\":null}"
                        .to_string()
                }),
            };

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/add-left-stamp" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<AddLeftStampRequest, _> = serde_json::from_str(&body);
            let resp_json = match parsed {
                Ok(req) => serde_json::to_string(&add_left_stamp(&conn, db_path, &album_name, req))
                    .unwrap_or_else(|_| {
                        "{\"ok\":false,\"message\":\"serialization failed\",\"added_left\":null}"
                            .to_string()
                    }),
                Err(e) => serde_json::to_string(&AddLeftStampResponse {
                    ok: false,
                    message: format!("invalid request: {}", e),
                    added_left: None,
                })
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"invalid request\",\"added_left\":null}".to_string()
                }),
            };

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/delete-left-stamp" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<DeleteLeftStampRequest, _> = serde_json::from_str(&body);
            let resp_json = match parsed {
                Ok(req) => serde_json::to_string(&delete_left_stamp(&conn, req)).unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"serialization failed\",\"deleted_rowid\":null,\"restored_colnect_id\":null}".to_string()
                }),
                Err(e) => serde_json::to_string(&DeleteLeftStampResponse {
                    ok: false,
                    message: format!("invalid request: {}", e),
                    deleted_rowid: None,
                    restored_colnect_id: None,
                })
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"invalid request\",\"deleted_rowid\":null,\"restored_colnect_id\":null}".to_string()
                }),
            };

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/clear-left-colnect-link" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<ClearLeftColnectLinkRequest, _> = serde_json::from_str(&body);
            let resp_json = match parsed {
                Ok(req) => serde_json::to_string(&clear_left_colnect_link(
                    &conn,
                    db_path,
                    &album_name,
                    req,
                ))
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"serialization failed\",\"updated_left\":null,\"restored_colnect_id\":null}"
                        .to_string()
                }),
                Err(e) => serde_json::to_string(&ClearLeftColnectLinkResponse {
                    ok: false,
                    message: format!("invalid request: {}", e),
                    updated_left: None,
                    restored_colnect_id: None,
                })
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"invalid request\",\"updated_left\":null,\"restored_colnect_id\":null}"
                        .to_string()
                }),
            };

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/add-colnect-stamp" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<AddColnectStampRequest, _> = serde_json::from_str(&body);
            let resp_json = match parsed {
                Ok(req) => serde_json::to_string(&add_colnect_stamp_to_stamps(
                    &conn,
                    db_path,
                    &album_name,
                    req,
                ))
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"serialization failed\",\"added_left\":null,\"processed_colnect_id\":null}"
                        .to_string()
                }),
                Err(e) => serde_json::to_string(&AddColnectStampResponse {
                    ok: false,
                    message: format!("invalid request: {}", e),
                    added_left: None,
                    processed_colnect_id: None,
                })
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"invalid request\",\"added_left\":null,\"processed_colnect_id\":null}".to_string()
                }),
            };

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/delete-colnect-candidate" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<DeleteColnectCandidateRequest, _> = serde_json::from_str(&body);
            let resp_json = match parsed {
                Ok(req) => serde_json::to_string(&delete_colnect_candidate(&conn, req)).unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"serialization failed\",\"deleted_stamp_rows\":0,\"deleted_code_rows\":0}".to_string()
                }),
                Err(e) => serde_json::to_string(&DeleteColnectCandidateResponse {
                    ok: false,
                    message: format!("invalid request: {}", e),
                    deleted_stamp_rows: 0,
                    deleted_code_rows: 0,
                })
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"invalid request\",\"deleted_stamp_rows\":0,\"deleted_code_rows\":0}".to_string()
                }),
            };

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/undo-colnect-delete" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<UndoColnectDeleteRequest, _> = serde_json::from_str(&body);
            let resp_json = match parsed {
                Ok(req) => {
                    serde_json::to_string(&undo_colnect_delete(&conn, req)).unwrap_or_else(|_| {
                        "{\"ok\":false,\"message\":\"serialization failed\",\"restored_rows\":0}"
                            .to_string()
                    })
                }
                Err(e) => serde_json::to_string(&UndoColnectDeleteResponse {
                    ok: false,
                    message: format!("invalid request: {}", e),
                    restored_rows: 0,
                })
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"invalid request\",\"restored_rows\":0}".to_string()
                }),
            };

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/api/colnect-import" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<ColnectImportRequest, _> = serde_json::from_str(&body);
            let resp_json = match parsed {
                Ok(req) => serde_json::to_string(&colnect_import_data(
                    &conn,
                    req,
                    db_path,
                    &album_name,
                ))
                    .unwrap_or_else(|_| {
                        "{\"ok\":false,\"message\":\"serialization failed\",\"files_processed\":0,\"stamps_imported\":0,\"errors\":[]}"
                            .to_string()
                    }),
                Err(e) => serde_json::to_string(&ColnectImportResponse {
                    ok: false,
                    message: format!("invalid request: {}", e),
                    files_processed: 0,
                    stamps_imported: 0,
                    errors: vec![format!("invalid request: {}", e)],
                })
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"invalid request\",\"files_processed\":0,\"stamps_imported\":0,\"errors\":[]}"
                        .to_string()
                }),
            };

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/api/default-catalog-config" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<UpdateDefaultCatalogRequest, _> = serde_json::from_str(&body);
            let album_dir = album_dir_for_ui(db_path, &album_name);
            let resp_json = match parsed {
                Ok(req) => {
                    match update_default_catalog_code_for_album(
                        &album_dir,
                        &album_name,
                        "config.toml",
                        "config_template.toml",
                        &req.default_catalog_code,
                    ) {
                        Ok(saved) => serde_json::to_string(&UpdateDefaultCatalogResponse {
                            ok: true,
                            message: format!("Updated album default_catalog_code to '{}'", saved),
                        })
                        .unwrap_or_else(|_| {
                            "{\"ok\":false,\"message\":\"serialization failed\"}".to_string()
                        }),
                        Err(e) => serde_json::to_string(&UpdateDefaultCatalogResponse {
                            ok: false,
                            message: format!("Failed to update config: {}", e),
                        })
                        .unwrap_or_else(|_| {
                            "{\"ok\":false,\"message\":\"failed to update config\"}".to_string()
                        }),
                    }
                }
                Err(e) => serde_json::to_string(&UpdateDefaultCatalogResponse {
                    ok: false,
                    message: format!("invalid request: {}", e),
                })
                .unwrap_or_else(|_| "{\"ok\":false,\"message\":\"invalid request\"}".to_string()),
            };

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/api/config-editor-save-album" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }
            let parsed: Result<SaveConfigEditorRequest, _> = serde_json::from_str(&body);
            let album_dir = album_dir_for_ui(db_path, &album_name);
            let response = match parsed {
                Ok(req) => match save_album_config_diff(
                    &album_dir,
                    &album_name,
                    "config.toml",
                    "config_template.toml",
                    &req.values,
                ) {
                    Ok(n) => {
                        reload_requested = true;
                        SaveConfigEditorResponse {
                            ok: true,
                            message: format!(
                                "Album override saved ({} changed field{}) for {}. Reloading now.",
                                n,
                                if n == 1 { "" } else { "s" },
                                album_name
                            ),
                            reload_required: true,
                        }
                    }
                    Err(error) => SaveConfigEditorResponse {
                        ok: false,
                        message: format!("Failed to save album config override: {}", error),
                        reload_required: false,
                    },
                },
                Err(error) => SaveConfigEditorResponse {
                    ok: false,
                    message: format!("invalid request: {}", error),
                    reload_required: false,
                },
            };
            let mut resp =
                Response::from_string(serde_json::to_string(&response).unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"serialization failed\",\"reload_required\":false}"
                        .to_string()
                }));
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            if reload_requested {
                break;
            }
            continue;
        }

        if method == Method::Post && req_url == "/api/config-editor-delete-album-override" {
            let album_dir = album_dir_for_ui(db_path, &album_name);
            let override_path = album_dir.join("config.toml");
            let response = if override_path.exists() {
                match fs::remove_file(&override_path) {
                    Ok(_) => {
                        reload_requested = true;
                        SaveConfigEditorResponse {
                            ok: true,
                            message: format!(
                                "Album config override for {} deleted. Reloading now.",
                                album_name
                            ),
                            reload_required: true,
                        }
                    }
                    Err(e) => SaveConfigEditorResponse {
                        ok: false,
                        message: format!("Failed to delete album override: {}", e),
                        reload_required: false,
                    },
                }
            } else {
                SaveConfigEditorResponse {
                    ok: false,
                    message: format!("No album config override found for {}.", album_name),
                    reload_required: false,
                }
            };
            let mut resp =
                Response::from_string(serde_json::to_string(&response).unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"serialization failed\",\"reload_required\":false}"
                        .to_string()
                }));
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            if reload_requested {
                break;
            }
            continue;
        }

        if method == Method::Post && req_url == "/api/config-editor-save" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<SaveConfigEditorRequest, _> = serde_json::from_str(&body);
            let response = match parsed {
                Ok(req) => match save_full_config_values(
                    "config.toml",
                    "config_template.toml",
                    &req.values,
                ) {
                    Ok(_) => {
                        reload_requested = true;
                        SaveConfigEditorResponse {
                            ok: true,
                            message: "config.toml updated. Reloading the review UI now."
                                .to_string(),
                            reload_required: true,
                        }
                    }
                    Err(error) => SaveConfigEditorResponse {
                        ok: false,
                        message: format!("Failed to save config.toml: {}", error),
                        reload_required: false,
                    },
                },
                Err(error) => SaveConfigEditorResponse {
                    ok: false,
                    message: format!("invalid request: {}", error),
                    reload_required: false,
                },
            };

            let mut resp =
                Response::from_string(serde_json::to_string(&response).unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"serialization failed\",\"reload_required\":false}"
                        .to_string()
                }));
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            if reload_requested {
                break;
            }
            continue;
        }

        if method == Method::Post && req_url == "/return-to-start" {
            return_to_start_requested = true;
            let mut resp = Response::from_string(
                "{\"ok\":true,\"message\":\"Returning to Start Review page...\"}".to_string(),
            );
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            break;
        }

        if method == Method::Post && req_url == "/generate-pdf-now" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let Some(_pdf_generation_guard) = PdfGenerationGuard::try_acquire() else {
                let resp_json = serde_json::to_string(&BootstrapGeneratePdfResponse {
                    ok: false,
                    message:
                        "PDF generation already in progress. Please wait for it to complete."
                            .to_string(),
                    generated_output_path: None,
                    generated_pdf_path: None,
                })
                .unwrap_or_else(|_| {
                    "{\"ok\":false,\"message\":\"PDF generation already in progress\",\"generated_output_path\":null,\"generated_pdf_path\":null}"
                        .to_string()
                });

                let mut resp = Response::from_string(resp_json).with_status_code(StatusCode(409));
                if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                    resp = resp.with_header(h);
                }
                let _ = request.respond(resp);
                continue;
            };

            let parsed_body: Result<serde_json::Value, _> = serde_json::from_str(&body);
            let (req_start_year, req_end_year) = parsed_body
                .as_ref()
                .ok()
                .map(|v| {
                    let sy = v
                        .get("start_year")
                        .and_then(|x| x.as_i64())
                        .map(|x| x as i32);
                    let ey = v.get("end_year").and_then(|x| x.as_i64()).map(|x| x as i32);
                    (sy, ey)
                })
                .unwrap_or((None, None));
            let mut generated_output_path = None;
            let mut generated_pdf_path = None;
            let message = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                crate::album_globals::generate_album_output_conn_with_pdf_override(
                    &conn,
                    &album_name,
                    None::<fn(usize)>,
                    req_start_year,
                    req_end_year,
                    Some(true),
                )
            })) {
                Ok(artifacts) => {
                    generated_output_path = Some(artifacts.output_txt_path.display().to_string());
                    if let Some(pdf_path) = artifacts.generated_pdf_path {
                        open_local_path(&pdf_path);
                        generated_pdf_path = Some(pdf_path.display().to_string());
                        "Album text and PDF generation completed.".to_string()
                    } else {
                        "Album text generated, but no PDF was produced. Check AlbumEasy CLI configuration/logs."
                            .to_string()
                    }
                }
                Err(_) => "Album/PDF generation failed unexpectedly (panic).".to_string(),
            };

            let resp_json = serde_json::to_string(&BootstrapGeneratePdfResponse {
                ok: generated_output_path.is_some(),
                message,
                generated_output_path,
                generated_pdf_path,
            })
            .unwrap_or_else(|_| {
                "{\"ok\":false,\"message\":\"serialization failed\",\"generated_output_path\":null,\"generated_pdf_path\":null}"
                    .to_string()
            });

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            continue;
        }

        if method == Method::Post && req_url == "/shutdown" {
            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let _ = request.respond(Response::empty(StatusCode(400)));
                continue;
            }

            let parsed: Result<ShutdownRequest, _> = serde_json::from_str(&body);
            let mut generated_output_path = None;
            let mut generated_pdf_path = None;
            let message = match parsed {
                Ok(req) => {
                    if req.generate_album {
                        let generated =
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                crate::album_globals::generate_album_output_conn_with_pdf_override(
                                    &conn,
                                    &album_name,
                                    None::<fn(usize)>,
                                    req.start_year,
                                    req.end_year,
                                    Some(req.generate_pdf),
                                )
                            }));
                        match generated {
                            Ok(artifacts) => {
                                generated_output_path =
                                    Some(artifacts.output_txt_path.display().to_string());
                                if let Some(pdf_path) = artifacts.generated_pdf_path {
                                    open_local_path(&pdf_path);
                                    generated_pdf_path = Some(pdf_path.display().to_string());
                                }
                                match (req.generate_pdf, generated_pdf_path.is_some()) {
                                    (true, true) => {
                                        "Review UI closed. Album text and PDF generation completed."
                                            .to_string()
                                    }
                                    (true, false) => {
                                        "Review UI closed. Album text generation completed, but no PDF was opened. Check AlbumEasy CLI configuration or logs."
                                            .to_string()
                                    }
                                    (false, _) => {
                                        "Review UI closed. Album text generation completed without PDF generation."
                                            .to_string()
                                    }
                                }
                            }
                            Err(_) => {
                                "Review UI closed. Album generation failed unexpectedly (panic)."
                                    .to_string()
                            }
                        }
                    } else {
                        "Review UI closed without album generation.".to_string()
                    }
                }
                Err(e) => format!("Review UI closed (invalid shutdown payload: {}).", e),
            };

            let resp_json = serde_json::to_string(&ShutdownResponse {
                ok: true,
                message,
                generated_output_path,
                generated_pdf_path,
            })
            .unwrap_or_else(|_| {
                "{\"ok\":true,\"message\":\"Review UI closed\",\"generated_output_path\":null,\"generated_pdf_path\":null}"
                    .to_string()
            });

            let mut resp = Response::from_string(resp_json);
            if let Ok(h) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
                resp = resp.with_header(h);
            }
            let _ = request.respond(resp);
            break;
        }

        let _ = request.respond(Response::empty(StatusCode(404)));
    }

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
    use super::{mark_colnect_candidate_processed, sync_processed_colnect_candidates};
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
