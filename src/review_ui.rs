use base64::Engine;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use tiny_http::{Header, Method, Response, Server, StatusCode};

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

fn render_bootstrap_html(default_inbox_path: Option<&str>) -> String {
    let inbox_hint = default_inbox_path.unwrap_or("samples/colnect_inbox");
    format!(
        r#"<!doctype html>
<html>
<head>
<meta charset='utf-8'>
<title>Stamp Matcher Setup</title>
<style>
body {{ font-family: sans-serif; margin: 20px; background: linear-gradient(180deg, #edf4f8, #f8fbfd); color: #1e3548; opacity: 0; animation: bootstrapFadeIn 180ms ease-out forwards; }}
.card {{ max-width: 960px; background: #fff; border: 1px solid #d7e2eb; border-radius: 12px; padding: 16px; box-shadow: 0 10px 24px rgba(28, 56, 82, 0.08); }}
h1 {{ margin: 0 0 8px 0; }}
p {{ margin: 0 0 10px 0; color: #395970; }}
.row {{ display: flex; gap: 12px; align-items: center; flex-wrap: wrap; margin-top: 10px; }}
.badge {{ display: inline-block; border: 1px solid #d2dfea; background: #f6fbff; border-radius: 999px; padding: 2px 8px; font-size: 12px; }}
.panel {{ border: 1px solid #e2e9ef; border-left: 5px solid #2f6fa3; border-radius: 10px; padding: 12px; background: #fbfdff; margin-top: 12px; }}
.warn {{ border-left-color: #9b6a20; background: #fffaf2; }}
.ok {{ border-left-color: #2f8f63; background: #f4fcf8; }}
label {{ font-size: 13px; font-weight: 600; }}
select, input {{ padding: 7px 8px; border: 1px solid #c8d5e2; border-radius: 8px; min-width: 170px; }}
button {{ padding: 7px 11px; border-radius: 8px; border: 1px solid #c8d5e2; background: #fff; cursor: pointer; }}
button.primary {{ background: #2f6fa3; color: #fff; border-color: #2f6fa3; }}
button.danger {{ background: #a34545; color: #fff; border-color: #a34545; }}
button:hover {{ background: #f1f6fa; }}
code {{ background: #eef4f8; border: 1px solid #d4e2ee; border-radius: 6px; padding: 2px 6px; }}
.status {{ margin-top: 10px; min-height: 18px; font-size: 13px; color: #35566d; }}
.loadingBox {{ display: none; margin-top: 12px; padding: 12px; border: 1px solid #d7e2eb; border-left: 5px solid #2f6fa3; border-radius: 10px; background: #f5fafe; }}
.spinner {{ width: 16px; height: 16px; border: 3px solid #d7e2eb; border-top-color: #2f6fa3; border-radius: 50%; display: inline-block; animation: spin 0.9s linear infinite; }}
.loadingRow {{ display: flex; align-items: center; gap: 10px; }}
.dots {{ display: inline-block; min-width: 18px; text-align: left; }}
body.busy {{ cursor: progress; }}
body.busy button, body.busy select, body.busy input {{ opacity: 0.65; pointer-events: none; }}
@keyframes spin {{ from {{ transform: rotate(0deg); }} to {{ transform: rotate(360deg); }} }}
@keyframes bootstrapFadeIn {{ from {{ opacity: 0; transform: translateY(2px); }} to {{ opacity: 1; transform: translateY(0); }} }}
</style>
</head>
<body>
<div class='card'>
    <h1>Start Review UI</h1>
    <p>No database was selected on startup. Pick a country to open its Review UI.</p>
    <div class='row'>
        <label for='countrySel'>Country</label>
        <select id='countrySel' onchange='onCountrySelectionChanged()'></select>
        <button onclick='refreshState()'>Refresh</button>
        <button onclick='openConfigEditor()'>Edit Config</button>
        <button class='primary' onclick='startReview()'>Open Review</button>
        <button onclick='generateAlbumPdfNow()'>Generate PDF Now</button>
        <button onclick='importColnectFiles()'>Import Colnect</button>
        <button class='danger' onclick='logoutApp()'>Logout</button>
    </div>
    <div class='row'>
        <label for='countrySearch'>Search</label>
        <input id='countrySearch' type='text' placeholder='Find country...' oninput='updateCountrySearchMatches(true)' />
        <button onclick='stepCountrySearch(-1)'>Back</button>
        <button onclick='stepCountrySearch(1)'>Forward</button>
        <span id='countrySearchStatus' class='badge'>Matches: 0/0</span>
    </div>
    <div class='row' style='align-items:center;gap:0.5em;flex-wrap:wrap;'>
        <label for='pdfStartYear' style='margin:0;'>PDF Year Range</label>
        <input id='pdfStartYear' type='number' placeholder='Start year' min='1840' max='2100' style='width:100px'/>
        <span style='font-size:0.9em;'>to</span>
        <input id='pdfEndYear' type='number' placeholder='End year' min='1840' max='2100' style='width:100px'/>
        <span style='font-size:0.82em;color:#666;'>(leave blank = all years)</span>
        <span id='countryCount' class='badge'>Countries: 0</span>
    </div>

    <div id='countryPanel' class='panel'></div>
    <div id='stampworldPanel' class='panel warn' style='display:none;'>
        <strong>No Colnect files found for this country.</strong>
        <p style='margin-top:6px;'>You can run StampWorld pull first, then import Colnect CSV files and come back.</p>
        <div class='row'>
            <label>StampWorld Country</label>
            <input id='swCountry' type='text' />
            <label>Start Year</label>
            <select id='swStartYear'></select>
            <label>End Year</label>
            <select id='swEndYear'></select>
        </div>
        <div class='row'>
            <span class='badge'>Colnect inbox hint: {}</span>
        </div>
        <div class='row'>
            <strong>Suggested command:</strong>
            <code id='swCmd'></code>
            <button onclick='copySuggestedCommand()'>Copy</button>
        </div>
    </div>
    <div id='status' class='status'></div>
    <div id='loadingBox' class='loadingBox'>
        <div class='loadingRow'>
            <span class='spinner'></span>
            <div>
                <strong>Preparing Review UI<span id='loadingDotsTitle' class='dots'></span></strong><br/>
                <span id='loadingText'>Loading country data and opening the compare view. This may take a few seconds.</span>
            </div>
        </div>
    </div>
</div>

<script>
let countries = [];
let loadingDotsTimer = null;
let loadingPhaseTimer = null;
let bootstrapPdfRequestInFlight = false;
let countrySearchMatches = [];
let countrySearchIndex = -1;

function setCountrySearchStatus(text) {{
    const status = document.getElementById('countrySearchStatus');
    if (status) status.innerText = text;
}}

function onCountrySelectionChanged() {{
    renderCountryPanel();
    updateCountrySearchMatches(true);
}}

function updateCountrySearchMatches(preserveCurrentSelection) {{
    const sel = document.getElementById('countrySel');
    const searchInput = document.getElementById('countrySearch');
    if (!sel || !searchInput) return;

    const query = String(searchInput.value || '').trim().toLowerCase();
    if (!query) {{
        countrySearchMatches = countries.map(c => c.country);
    }} else {{
        countrySearchMatches = countries
            .filter(c => String(c.country || '').toLowerCase().includes(query))
            .map(c => c.country);
    }}

    if (!countrySearchMatches.length) {{
        countrySearchIndex = -1;
        setCountrySearchStatus('Matches: 0/0');
        return;
    }}

    if (preserveCurrentSelection) {{
        const selectedCountry = String(sel.value || '');
        const selectedIdx = countrySearchMatches.findIndex(c => c === selectedCountry);
        if (selectedIdx >= 0) {{
            countrySearchIndex = selectedIdx;
            setCountrySearchStatus(`Matches: ${{countrySearchIndex + 1}}/${{countrySearchMatches.length}}`);
            return;
        }}
    }}

    countrySearchIndex = 0;
    sel.value = countrySearchMatches[0];
    renderCountryPanel();
    setCountrySearchStatus(`Matches: 1/${{countrySearchMatches.length}}`);
}}

function stepCountrySearch(delta) {{
    const sel = document.getElementById('countrySel');
    if (!sel) return;

    if (!countrySearchMatches.length) {{
        setCountrySearchStatus('Matches: 0/0');
        return;
    }}

    if (countrySearchIndex < 0) countrySearchIndex = 0;
    const total = countrySearchMatches.length;
    countrySearchIndex = (countrySearchIndex + delta + total) % total;
    sel.value = countrySearchMatches[countrySearchIndex];
    renderCountryPanel();
    setCountrySearchStatus(`Matches: ${{countrySearchIndex + 1}}/${{total}}`);
}}

function startLoadingPhases(country) {{
    const txt = document.getElementById('loadingText');
    if (!txt) return;

    if (loadingPhaseTimer) {{
        clearInterval(loadingPhaseTimer);
        loadingPhaseTimer = null;
    }}

    const phases = [
        `Preparing data from ${{country}}`,
        'Opening database...',
        'Syncing processed Colnect candidates...',
        'Loading left StampWorld rows...',
        'Loading right Colnect candidates...',
        'Starting local web server...',
        'Opening compare view in browser...'
    ];

    let idx = 0;
    txt.innerText = phases[idx];
    loadingPhaseTimer = setInterval(() => {{
        idx = (idx + 1) % phases.length;
        txt.innerText = phases[idx];
    }}, 900);
}}

function stopLoadingPhases() {{
    if (loadingPhaseTimer) {{
        clearInterval(loadingPhaseTimer);
        loadingPhaseTimer = null;
    }}
}}

function setMovingDots(active) {{
    const dots = document.getElementById('loadingDotsTitle');
    if (!dots) return;

    if (!active) {{
        if (loadingDotsTimer) {{
            clearInterval(loadingDotsTimer);
            loadingDotsTimer = null;
        }}
        stopLoadingPhases();
        dots.innerText = '';
        return;
    }}

    if (loadingDotsTimer) return;
    let idx = 0;
    const frames = ['', '.', '..', '...'];
    loadingDotsTimer = setInterval(() => {{
        idx = (idx + 1) % frames.length;
        dots.innerText = frames[idx];
    }}, 260);
}}

function setBusy(isBusy, message) {{
    document.body.classList.toggle('busy', isBusy);
    const box = document.getElementById('loadingBox');
    const txt = document.getElementById('loadingText');
    if (txt && message) txt.innerText = message;
    if (box) box.style.display = isBusy ? 'block' : 'none';
    setMovingDots(isBusy);
}}

function slugCountry(name) {{
    return String(name || '').trim().replaceAll(' ', '-');
}}

function populateYearSelect(id, startYear, endYear, defaultYear) {{
    const sel = document.getElementById(id);
    if (!sel) return;
    const options = [];
    for (let y = startYear; y <= endYear; y++) {{
        options.push(`<option value="${{y}}">${{y}}</option>`);
    }}
    sel.innerHTML = options.join('');
    const clampedDefault = Math.min(Math.max(defaultYear, startYear), endYear);
    sel.value = String(clampedDefault);
}}

function initYearSelectors() {{
    const minYear = 1840;
    const maxYear = new Date().getFullYear();
    populateYearSelect('swStartYear', minYear, maxYear, 1840);
    populateYearSelect('swEndYear', minYear, maxYear, 2026);
}}

function suggestedStampworldCommand(country) {{
    const c = document.getElementById('swCountry').value || country || '';
    const y1 = document.getElementById('swStartYear').value || '1840';
    const y2 = document.getElementById('swEndYear').value || '2026';
    return `./stamp_album_gen2 --source stampworld --site-name "${{c}}" --album-name "${{c}}" --start-year ${{y1}} --end-year ${{y2}}`;
}}

async function copySuggestedCommand() {{
    const cmd = document.getElementById('swCmd')?.innerText || '';
    const status = document.getElementById('status');
    if (!cmd.trim()) {{
        if (status) status.innerText = 'No command available to copy.';
        return;
    }}

    try {{
        if (navigator.clipboard && navigator.clipboard.writeText) {{
            await navigator.clipboard.writeText(cmd);
        }} else {{
            const ta = document.createElement('textarea');
            ta.value = cmd;
            ta.style.position = 'fixed';
            ta.style.left = '-9999px';
            document.body.appendChild(ta);
            ta.focus();
            ta.select();
            document.execCommand('copy');
            document.body.removeChild(ta);
        }}
        if (status) status.innerText = 'Suggested command copied to clipboard.';
    }} catch (e) {{
        if (status) status.innerText = `Copy failed: ${{e}}`;
    }}
}}

function renderCountryPanel() {{
    const sel = document.getElementById('countrySel');
    const c = countries.find(x => x.country === sel.value);
    if (!c) {{
        document.getElementById('countryPanel').innerHTML = '<em>No countries found under Albums/*/*.db</em>';
        document.getElementById('stampworldPanel').style.display = '';
        document.getElementById('swCmd').innerText = suggestedStampworldCommand('');
        return;
    }}
    document.getElementById('countryPanel').className = c.colnect_file_count > 0 || c.colnect_candidate_count > 0 ? 'panel ok' : 'panel';
    document.getElementById('countryPanel').innerHTML =
        `<strong>${{c.country}}</strong><br/>` +
        `<span class='badge'>DB: ${{c.db_path}}</span> ` +
        `<span class='badge'>Colnect files: ${{c.colnect_file_count}}</span> ` +
        `<span class='badge'>DB candidates: ${{c.colnect_candidate_count}}</span>`;

    document.getElementById('swCountry').value = c.country;
    document.getElementById('swCmd').innerText = suggestedStampworldCommand(c.country);
    document.getElementById('stampworldPanel').style.display = (c.colnect_file_count === 0 && c.colnect_candidate_count === 0) ? '' : 'none';
}}

async function refreshState() {{
    const status = document.getElementById('status');
    status.innerText = 'Scanning Albums...';
    setBusy(false, '');
    try {{
        const r = await fetch('/bootstrap-state');
        const j = await r.json();
        countries = j.countries || [];
        const sel = document.getElementById('countrySel');
        const old = sel.value;
        sel.innerHTML = countries.map(c => `<option value="${{c.country}}">${{c.country}}</option>`).join('');
        if (old && countries.some(c => c.country === old)) sel.value = old;
        if (!sel.value && countries.length) sel.value = countries[0].country;
        document.getElementById('countryCount').innerText = `Countries: ${{countries.length}}`;
        renderCountryPanel();
        updateCountrySearchMatches(true);
        status.innerText = countries.length ? 'Select a country and open review.' : 'No country DBs found. Create one by running a StampWorld pull first.';
    }} catch (e) {{
        status.innerText = `Failed to load startup state: ${{e}}`;
    }}
}}

async function startReview() {{
    const sel = document.getElementById('countrySel');
    if (!sel.value) {{
        document.getElementById('status').innerText = 'Select a country first.';
        return;
    }}
    document.getElementById('status').innerText = `Starting Review UI for ${{sel.value}}...`;
    setBusy(true, `Preparing data from ${{sel.value}}`);
    startLoadingPhases(sel.value);
    try {{
        const r = await fetch('/start-review', {{
            method: 'POST',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{ country: sel.value }})
        }});
        const j = await r.json();
        if (!j.ok) {{
            setBusy(false, '');
            document.getElementById('status').innerText = j.message || 'Failed to start review.';
            return;
        }}
        stopLoadingPhases();
        document.getElementById('loadingText').innerText = 'Launching compare view...';
        document.getElementById('status').innerText = j.message || 'Launching review...';
        // Poll /compare-ready until the review UI server is accepting requests,
        // then redirect. This avoids a race between data loading and the redirect.
        (async function pollReady() {{
            for (let attempt = 0; attempt < 60; attempt++) {{
                await new Promise(r => setTimeout(r, 500));
                try {{
                    const rdy = await fetch('/compare-ready');
                    if (rdy.ok) {{
                        window.location.href = '/';
                        return;
                    }}
                }} catch (_) {{}}
            }}
            // Fallback after 30 s: redirect anyway
            window.location.href = '/';
        }})();
    }} catch (e) {{
        setBusy(false, '');
        document.getElementById('status').innerText = `Failed to start: ${{e}}`;
    }}
}}

async function logoutApp() {{
    const status = document.getElementById('status');
    if (status) status.innerText = 'Logging out and closing Review UI...';
    setBusy(true, 'Closing Review UI...');
    try {{
        await fetch('/logout', {{ method: 'POST' }});
    }} catch (_) {{
        // Even if the response is interrupted, continue with shutdown UX.
    }}
    setBusy(false, '');
    document.body.innerHTML = "<div style='font-family:sans-serif;margin:20px;'><h2>Review UI closed</h2><p>You can close this browser tab now.</p></div>";
}}

async function importColnectFiles() {{
    const sel = document.getElementById('countrySel');
    const country = sel.value;
    const status = document.getElementById('status');
    
    if (!country) {{
        if (status) status.innerText = 'Select a country first.';
        return;
    }}

    if (status) status.innerText = 'Importing Colnect files for ' + country + '...';
    setBusy(true, 'Importing Colnect CSV files for ' + country + '...');

    try {{
        const r = await fetch('/api/colnect-import', {{
            method: 'POST',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{}})
        }});
        const j = await r.json();
        setBusy(false, '');
        if (j.ok && j.stamps_imported > 0) {{
            if (status) status.innerText = 'Imported ' + j.stamps_imported + ' stamps from ' + j.files_processed + ' file(s)';
            alert('Successfully imported ' + j.stamps_imported + ' stamps from ' + j.files_processed + ' file(s).');
            await refreshState();
        }} else if (j.ok && j.stamps_imported === 0) {{
            const msg = j.message || 'No stamps were imported.';
            const details = (j.errors && j.errors.length > 0)
                ? '\n\nDetails:\n' + j.errors.join('\n')
                : '\n\nOpen the Review UI for this country and verify the configured colnect inbox path.';
            if (status) status.innerText = 'No stamps imported';
            alert(msg + details);
        }} else {{
            const msg = j.message || 'Import failed';
            const details = (j.errors && j.errors.length > 0)
                ? '\n\nErrors:\n' + j.errors.join('\n')
                : '';
            if (status) status.innerText = 'Import unavailable: ' + msg;
            alert(msg + details);
        }}
    }} catch (e) {{
        setBusy(false, '');
        if (status) status.innerText = 'Import error';
        alert('Import error: ' + e);
    }}
}}

async function generateAlbumPdfNow() {{
    const sel = document.getElementById('countrySel');
    const country = String((sel && sel.value) || '').trim();
    const status = document.getElementById('status');

    if (bootstrapPdfRequestInFlight) {{
        if (status) status.innerText = 'PDF generation already in progress. Please wait.';
        return;
    }}

    if (!country) {{
        if (status) status.innerText = 'Select a country first.';
        return;
    }}

    const startYearVal = document.getElementById('pdfStartYear')?.value.trim();
    const endYearVal   = document.getElementById('pdfEndYear')?.value.trim();
    const startYear    = startYearVal ? parseInt(startYearVal, 10) : null;
    const endYear      = endYearVal   ? parseInt(endYearVal, 10)   : null;

    const yearDesc = (startYear || endYear)
        ? ` (years ${{startYear || 'start'}}–${{endYear || 'end'}})` : '';
    if (status) status.innerText = 'Generating album text and PDF for ' + country + yearDesc + '...';
    setBusy(true, 'Generating album and PDF for ' + country + yearDesc + '...');
    bootstrapPdfRequestInFlight = true;

    try {{
        const r = await fetch('/api/generate-pdf-now', {{
            method: 'POST',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{ country: country, start_year: startYear, end_year: endYear }})
        }});
        const j = await r.json();
        setBusy(false, '');

        if (!j.ok) {{
            if (status) status.innerText = j.message || 'PDF generation failed.';
            alert('PDF generation failed.\n\n' + (j.message || 'Unknown error.'));
            return;
        }}

        const outputPath = j.generated_output_path ? ('\nOutput: ' + j.generated_output_path) : '';
        const pdfPath = j.generated_pdf_path ? ('\nPDF: ' + j.generated_pdf_path) : '';
        if (status) status.innerText = j.message || 'PDF generation completed.';
        alert((j.message || 'PDF generation completed.') + outputPath + pdfPath);
    }} catch (e) {{
        setBusy(false, '');
        const msg = 'Generate PDF request failed: ' + e;
        if (status) status.innerText = msg;
        alert(msg);
    }} finally {{
        bootstrapPdfRequestInFlight = false;
    }}
}}

document.getElementById('swStartYear').addEventListener('change', () => {{
    const country = document.getElementById('countrySel').value;
    document.getElementById('swCmd').innerText = suggestedStampworldCommand(country);
}});
document.getElementById('swEndYear').addEventListener('change', () => {{
    const country = document.getElementById('countrySel').value;
    document.getElementById('swCmd').innerText = suggestedStampworldCommand(country);
}});

initYearSelectors();
refreshState();
</script>
{config_editor_assets}
</body>
</html>"#,
        inbox_hint,
        config_editor_assets = render_config_editor_assets()
    )
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

fn render_html(payload: &MatcherPayload) -> String {
    let json =
        serde_json::to_string(payload).unwrap_or_else(|_| "{\"left\":[],\"right\":[]}".to_string());
    format!(
        r#"<!doctype html>
<html>
<head>
<meta charset='utf-8'>
<title>Stamp Matcher UI</title>
<style>
body {{
  font-family: sans-serif;
  margin: 16px;
    background: linear-gradient(180deg, #eef4f8 0%, #f7f9fb 100%);
    opacity: 1;
    transition: opacity 170ms ease, filter 170ms ease, transform 170ms ease;
  --row-odd-bg: #ffffff;
  --row-even-bg: #e6eef4;
  --row-odd-key-bg: #f1f3f5;
  --row-even-key-bg: #d7e2eb;
    --accent-blue: #2f6fa3;
    --accent-green: #2f8f63;
    --accent-amber: #9b6a20;
    --accent-red: #a34545;
    --accent-slate: #4b5a68;
}}
body.pageLeaving {{ opacity: 0.2; filter: blur(1px); transform: translateY(2px); }}
body.pageEnter {{ animation: reviewPageIn 170ms ease-out; }}
@keyframes reviewPageIn {{ from {{ opacity: 0.45; transform: translateY(2px); }} to {{ opacity: 1; transform: translateY(0); }} }}
.wrap {{ background: #ffffff; border: 1px solid #dbe4ec; border-radius: 12px; padding: 14px; box-shadow: 0 8px 24px rgba(27, 51, 72, 0.08); }}
.top {{ display: flex; gap: 12px; align-items: center; flex-wrap: wrap; margin-bottom: 10px; }}
.startGuide {{ border: 1px solid #d8e3ed; border-left: 6px solid var(--accent-blue); border-radius: 10px; background: #f5fafe; padding: 12px; margin-bottom: 12px; }}
.startGuide h2 {{ margin: 0 0 4px 0; font-size: 18px; color: #173952; }}
.startGuide p {{ margin: 0; color: #35566d; font-size: 13px; }}
.guideStats {{ display: flex; gap: 8px; align-items: center; flex-wrap: wrap; margin-top: 10px; }}
.guideActions {{ display: flex; gap: 8px; align-items: center; flex-wrap: wrap; margin-top: 10px; }}
.controlGroups {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(240px, 1fr)); gap: 10px; margin-bottom: 10px; }}
.groupCard {{ border: 1px solid #dbe4ec; border-radius: 10px; background: #fbfdff; padding: 10px; }}
.groupCard h4 {{ margin: 0 0 8px 0; font-size: 13px; letter-spacing: 0.2px; }}
.groupCard.match {{ border-left: 5px solid var(--accent-green); }}
.groupCard.candidate {{ border-left: 5px solid var(--accent-amber); }}
.groupCard.session {{ border-left: 5px solid var(--accent-slate); }}
.groupBtns {{ display: flex; gap: 8px; align-items: center; flex-wrap: wrap; }}
.sessionStack {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 8px; width: 100%; }}
.sessionSection {{ border: 1px solid #d7e2eb; border-radius: 8px; background: #f9fcff; padding: 8px; }}
.sessionSection:nth-child(1) {{ background: #f2f8ff; border-color: #cddff2; }}
.sessionSection:nth-child(2) {{ background: #f2fbf6; border-color: #cfe8da; }}
.sessionSection:nth-child(3) {{ background: #fff8ef; border-color: #ead8bc; }}
.sessionSection:nth-child(4) {{ background: #f5f6f8; border-color: #d5dbe3; }}
.sessionSectionTitle {{ margin: 0 0 6px 0; font-size: 12px; font-weight: 700; color: #2a4458; letter-spacing: 0.2px; }}
.sessionRow {{ display: flex; gap: 8px; align-items: center; flex-wrap: wrap; }}
.actionRows {{ display: flex; flex-direction: column; gap: 8px; margin-bottom: 10px; }}
.actionBar {{ border: 1px solid #dbe4ec; border-radius: 10px; padding: 9px 10px; background: #fbfdff; display: flex; gap: 10px; align-items: center; flex-wrap: wrap; }}
.actionBar.match {{ border-left: 5px solid var(--accent-green); }}
.actionBar.candidate {{ border-left: 5px solid var(--accent-amber); }}
.actionBarTitle {{ font-size: 12px; font-weight: 700; color: #2a4458; margin-right: 6px; }}
.filtersCard {{ border: 1px solid #dbe4ec; border-left: 5px solid var(--accent-blue); border-radius: 10px; background: #f8fbfe; padding: 10px; margin-bottom: 12px; }}
.filtersTitle {{ margin: 0 0 8px 0; font-size: 13px; color: #1f4663; }}
.filtersGrid {{ display: flex; gap: 10px; align-items: center; flex-wrap: wrap; }}
.leftActionBanner {{ border: 1px solid #e5dccd; border-left: 5px solid var(--accent-amber); border-radius: 10px; background: linear-gradient(180deg, #fffaf2 0%, #fffdf8 100%); padding: 10px; margin-bottom: 8px; }}
.leftActionTop {{ display: flex; gap: 10px; align-items: center; justify-content: space-between; flex-wrap: wrap; margin-bottom: 6px; }}
.leftActionTop strong {{ color: #5d4116; }}
.leftActionText {{ margin: 0; color: #6c5530; font-size: 12px; line-height: 1.45; }}
.leftActionBtns {{ display: flex; gap: 8px; flex-wrap: wrap; margin-top: 8px; }}
.leftActionNote {{ display: flex; gap: 8px; align-items: center; flex-wrap: wrap; margin-top: 6px; }}
.leftSearchRow {{ display: flex; gap: 8px; align-items: center; flex-wrap: wrap; margin-top: 8px; }}
.leftSearchRow input {{ width: 220px; max-width: 52vw; padding: 5px 7px; border: 1px solid #c8d5e2; border-radius: 7px; }}
.leftNavRow {{ display: flex; gap: 6px; align-items: center; flex-wrap: wrap; margin-top: 8px; background: #fff4e2; border: 1px solid #ead5b4; border-radius: 8px; padding: 6px; }}
.rightActionBanner {{ border: 1px solid #d6e4f2; border-left: 5px solid var(--accent-blue); border-radius: 10px; background: linear-gradient(180deg, #f4f9ff 0%, #f9fcff 100%); padding: 10px; margin-bottom: 8px; }}
.rightActionTop {{ display: flex; gap: 10px; align-items: center; justify-content: space-between; flex-wrap: wrap; margin-bottom: 6px; }}
.rightActionTop strong {{ color: #214d72; }}
.rightBottomBlock {{ display: flex; flex-direction: column; gap: 0; }}
.rightSearchRow {{ display: flex; gap: 8px; align-items: center; flex-wrap: wrap; margin-top: 8px; }}
.rightSearchRow input {{ width: 220px; max-width: 52vw; padding: 5px 7px; border: 1px solid #c8d5e2; border-radius: 7px; }}
.rightNavRow {{ display: flex; gap: 6px; align-items: center; flex-wrap: wrap; margin-top: 8px; background: #eaf4ff; border: 1px solid #c9ddf2; border-radius: 8px; padding: 6px; }}
@media (min-width: 981px) {{
    #comparePanels {{
        display: grid;
        grid-template-columns: 1fr 1fr;
        grid-template-rows: auto auto 1fr;
        gap: 0 16px;
        align-items: start;
    }}
    #comparePanels .col {{
        display: grid;
        grid-template-rows: subgrid;
        grid-row: 1 / span 3;
    }}
    #comparePanels .col:first-child {{ grid-column: 1; }}
    #comparePanels .col:last-child  {{ grid-column: 2; }}
    /* Push right search/nav to the bottom of the shared banner row so it
       sits at the same eye level as the left search/nav (which is naturally
       at the bottom of the taller left action banner). */
    .rightActionBanner {{
        display: flex;
        flex-direction: column;
        justify-content: space-between;
    }}
}}
.leftEditorGrid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 10px; margin-bottom: 8px; }}
.leftEditorGrid label, .leftEditorStack label {{ display: flex; flex-direction: column; gap: 4px; font-size: 12px; font-weight: 600; color: #284357; }}
.leftEditorStack {{ display: flex; flex-direction: column; gap: 8px; margin-bottom: 8px; }}
.leftEditorStack textarea {{ min-height: 74px; resize: vertical; }}
.leftEditorModes {{ display: flex; gap: 12px; flex-wrap: wrap; margin: 8px 0; }}
.leftEditorHint {{ margin: 0 0 4px 0; color: #5d6f7f; font-size: 12px; }}
.leftEditorDialog {{ width: min(980px, 97vw); border: 1px solid #c8d9e5; border-radius: 14px; padding: 0; overflow: hidden; box-shadow: 0 22px 48px rgba(16, 39, 56, 0.28); }}
.leftEditorHead {{ background: linear-gradient(180deg, #f3f8fc 0%, #eaf2f8 100%); padding: 14px 16px 12px; border-bottom: 1px solid #d6e3ed; }}
.leftEditorHead h4 {{ margin: 0 0 6px 0; font-size: 18px; color: #173a4f; }}
.leftEditorBody {{ max-height: 66vh; overflow-y: auto; padding: 14px 16px; background: linear-gradient(180deg, #fbfdff 0%, #f2f7fb 100%); }}
.leftEditorSection {{ border: 1px solid #d5e1ea; border-radius: 12px; background: #ffffff; margin-bottom: 12px; overflow: hidden; }}
.leftEditorSection h5 {{ margin: 0; padding: 10px 12px; font-size: 13px; color: #173a4f; background: #eef5fa; border-bottom: 1px solid #d9e5ef; }}
.leftEditorSectionBody {{ padding: 10px 12px; }}
.leftEditorDimsGrid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 10px; }}
.leftEditorCompact input, .leftEditorCompact textarea {{ background: #fcfeff; border: 1px solid #cddbe6; border-radius: 8px; padding: 7px 9px; }}
.leftEditorCompact input:focus, .leftEditorCompact textarea:focus {{ outline: 2px solid rgba(47, 111, 163, 0.16); border-color: #2f6fa3; }}
.leftEditorActions {{ display: flex; justify-content: flex-end; gap: 10px; padding: 12px 16px; border-top: 1px solid #d6e3ed; background: #f5f9fc; }}
.row {{ display: flex; gap: 16px; align-items: stretch; }}
.col {{ flex: 1; min-width: 280px; display: flex; flex-direction: column; }}
.colNav {{ display: flex; gap: 8px; align-items: center; flex-wrap: wrap; margin-bottom: 6px; }}
.titleRow {{ display: flex; gap: 8px; align-items: center; flex-wrap: wrap; margin-bottom: 8px; }}
.titleRow h3 {{ margin: 0; }}
.panelCard {{ display: flex; flex-direction: column; border: 1px solid #ddd; border-radius: 8px; background: #fff; overflow: hidden; min-height: 0; }}
.panelHead {{ padding: 10px; border-bottom: 1px solid #ececec; background: #fbfcfd; font-size: 12px; }}
.panelBody {{ padding: 8px; overflow: auto; max-height: 56vh; }}
.imgbox {{ min-height: 260px; display: flex; align-items: center; justify-content: center; border: 1px solid #ccc; background: #fff; }}
img {{ max-width: 260px; max-height: 260px; border: 1px solid #ccc; background: #fff; }}
.imgph {{ width: 260px; height: 260px; border: 1px dashed #bbb; background: #f7f7f7; color: #666; display: flex; align-items: center; justify-content: center; text-align: center; font-size: 12px; padding: 8px; box-sizing: border-box; }}
.kv {{ width: 100%; border-collapse: collapse; margin-top: 8px; font-size: 12px; }}
.kv td {{ border: 1px solid #e1e6ea; padding: 4px 6px; vertical-align: top; }}
.kv tr:nth-child(odd) td {{ background: var(--row-odd-bg); }}
.kv tr:nth-child(even) td {{ background: var(--row-even-bg); }}
.kv tr:nth-child(odd) td.k {{ background: var(--row-odd-key-bg); }}
.kv tr:nth-child(even) td.k {{ background: var(--row-even-key-bg); }}
.kv td.k {{ width: 36%; font-weight: 600; }}
.kv td.v {{ width: 64%; word-break: break-word; }}
button {{ padding: 6px 10px; border-radius: 8px; border: 1px solid #c9d5e2; background: #ffffff; color: #1f3448; cursor: pointer; }}
button:hover {{ background: #f1f6fa; }}
button.primary {{ background: #2f6fa3; border-color: #2f6fa3; color: #ffffff; }}
button.primary:hover {{ background: #285f8a; }}
button.success {{ background: #2f8f63; border-color: #2f8f63; color: #ffffff; }}
button.success:hover {{ background: #287953; }}
button.warn {{ background: #f0e1c6; border-color: #ddc79f; color: #694314; }}
button.danger {{ background: #a34545; border-color: #a34545; color: #ffffff; }}
button.danger:hover {{ background: #8f3939; }}
button.neutral {{ background: #4b5a68; border-color: #4b5a68; color: #ffffff; }}
button.navBtn {{ background: #eef4f8; border-color: #d4e2ee; }}
.badge {{ display: inline-block; padding: 2px 8px; border: 1px solid #d3dfe9; border-radius: 999px; background: #f8fbfe; font-size: 12px; color: #2d4760; }}
.modal-backdrop {{ position: fixed; inset: 0; background: rgba(0,0,0,0.45); display: none; align-items: center; justify-content: center; z-index: 1000; }}
.modal {{ background: #fff; border-radius: 10px; border: 1px solid #ddd; width: min(700px, 96vw); max-height: 88vh; overflow-y: auto; padding: 14px; box-sizing: border-box; }}
.modal h4 {{ margin: 0 0 10px 0; }}
.modal .rowx {{ display: flex; gap: 10px; margin-bottom: 8px; flex-wrap: wrap; }}
.modal .rowx label {{ display: inline-flex; align-items: center; gap: 6px; }}
.modal input[type='text'], .modal input[type='number'] {{ padding: 6px 8px; border: 1px solid #ccc; border-radius: 6px; min-width: 140px; }}
.opBusyBackdrop {{
    position: fixed;
    inset: 0;
    display: none;
    align-items: center;
    justify-content: center;
    background: rgba(15, 28, 42, 0.48);
    z-index: 1500;
}}
.opBusyCard {{
    width: min(560px, 92vw);
    border: 1px solid #c9d8e6;
    border-radius: 12px;
    background: #ffffff;
    box-shadow: 0 18px 42px rgba(17, 38, 56, 0.25);
    padding: 14px 16px;
}}
.opBusyRow {{
    display: flex;
    align-items: center;
    gap: 10px;
}}
.opBusySpinner {{
    width: 18px;
    height: 18px;
    border: 3px solid #d7e2eb;
    border-top-color: #2f6fa3;
    border-radius: 50%;
    animation: spin 0.9s linear infinite;
    flex: 0 0 auto;
}}
.opBusyText {{
    color: #1f3b52;
    font-size: 13px;
    line-height: 1.4;
}}
.opBusyDots {{
    display: inline-block;
    min-width: 18px;
    text-align: left;
}}
@media (max-width: 980px) {{
    .row {{ flex-direction: column; }}
    .panelBody {{ max-height: 42vh; }}
}}
</style>
</head>
<body>
<div class='wrap'>
    <div class='top'>
        <button class='neutral' onclick='returnToStartReviewPage()'>Back To Country Selection</button>
        <span class='badge'>Switch country without closing Review UI</span>
    </div>
    <div id='startGuide' class='startGuide'>
        <h2>Start Review Session</h2>
        <p>Use the grouped controls to navigate candidates, assign matches, and apply updates safely without losing CLI/config capabilities.</p>
        <div class='guideStats'>
            <span id='statLeftCount' class='badge'>Left: 0</span>
            <span id='statRightCount' class='badge'>Candidates: 0</span>
            <span id='statAssignedCount' class='badge'>Assigned: 0</span>
        </div>
        <div class='guideActions'>
            <button class='primary' onclick='dismissStartGuide()'>Start Reviewing</button>
            <button onclick='scrollToFilters()'>Review Field Options</button>
            <button onclick='scrollToPanels()'>Jump To Compare Panels</button>
        </div>
  </div>
    <div class='controlGroups'>
        <div class='groupCard session'>
            <h4>Session</h4>
            <div class='sessionStack'>
                <div class='sessionSection'>
                    <div class='sessionSectionTitle'>Configuration</div>
                    <div class='sessionRow'>
                        <button onclick='openConfigEditor()'>Edit Config</button>
                        <label for='defaultCatalogSelect'>Default Catalog</label>
                        <select id='defaultCatalogSelect' onchange='changeCountryDefaultCatalog()'>
                            <option value='sg'>Stanley Gibbons (SG)</option>
                            <option value='sn'>Scott (SN)</option>
                            <option value='mi'>Michel (MI)</option>
                            <option value='yt'>Yvert (YT)</option>
                            <option value='afa'>AFA</option>
                            <option value='nvp'>NVP</option>
                            <option value='un'>Unificato (UN)</option>
                            <option value='col'>Colnect (COL)</option>
                        </select>
                        <button onclick='saveCatalogDefaultToConfig()'>Update Config Default</button>
                    </div>
                </div>
                <div class='sessionSection'>
                    <div class='sessionSectionTitle'>Import Colnect</div>
                    <div class='sessionRow'>
                        <button onclick='importColnectFilesReview()'>Import Colnect</button>
                    </div>
                </div>
                <div class='sessionSection'>
                    <div class='sessionSectionTitle'>PDF</div>
                    <div class='sessionRow'>
                        <button onclick='generatePdfNowReview()'>Generate PDF Now</button>
                        <label for='reviewPdfStartYear' style='font-size:0.9em;margin:0 0 0 0.4em;'>Year</label>
                        <input id='reviewPdfStartYear' type='number' placeholder='Start' min='1840' max='2100' style='width:78px;padding:2px 4px;font-size:0.9em'/>
                        <span style='font-size:0.9em;'>–</span>
                        <input id='reviewPdfEndYear' type='number' placeholder='End' min='1840' max='2100' style='width:78px;padding:2px 4px;font-size:0.9em'/>
                    </div>
                </div>
                <div class='sessionSection'>
                    <div class='sessionSectionTitle'>Navigation</div>
                    <div class='sessionRow'>
                        <button onclick='returnToStartReviewPage()'>Back To Country Selection</button>
                        <button class='neutral' onclick='closeReviewUi()'>Close Review UI</button>
                        <span id='status' class='badge'>Ready</span>
                    </div>
                </div>
            </div>
        </div>
    </div>
        <div id='filtersCard' class='filtersCard'>
            <h4 class='filtersTitle'>Field Update Options</h4>
            <div class='filtersGrid'>
        <label><input type='checkbox' id='f_series'/> series</label>
        <label><input type='checkbox' id='f_description' checked/> description</label>
        <label><input type='checkbox' id='f_watermark'/> watermark</label>
        <label><input type='checkbox' id='f_design'/> design</label>
        <label><input type='checkbox' id='f_engraving'/> engraving</label>
        <label><input type='checkbox' id='f_perforation'/> perforation</label>
        <label><input type='checkbox' id='f_context'/> context</label>
        <label><input type='checkbox' id='f_links' checked/> colnect links</label>
        <label><input type='checkbox' id='f_image_url' checked/> colnect image url</label>
        <label><input type='checkbox' id='f_colnect_meta' checked/> colnect metadata fields</label>
        <label><input type='checkbox' id='f_replace_sg'/> replace SG number</label>
        <label><input type='checkbox' id='f_right_unmatched_only' onchange='render()'/> right unmatched only</label>
        <label><input type='checkbox' id='f_show_imageless_left' onchange='toggleImagelessLeftRows()'/> show left rows without images</label>
                <label>order
                        <select id='sortMode' onchange='changeSortMode()'>
                                <option value='sg'>SG number (default)</option>
                                <option value='year_sg'>Year then SG</option>
                                <option value='series_sg'>Series then SG</option>
                                <option value='rowid'>DB row id</option>
                                <option value='colnect_id'>Colnect ID</option>
                        </select>
                </label>
      </div>
    </div>
    <div class='actionRows'>
        <div class='actionBar match'>
            <span class='actionBarTitle'>Match Actions</span>
            <button class='success' onclick='assignCurrent()'>Assign Candidate</button>
            <button onclick='clearAssignment()'>Clear Assignment</button>
            <button class='primary' onclick='applyMatches()'>Apply Matches To DB</button>
        </div>
        <div class='actionBar candidate'>
            <span class='actionBarTitle'>Candidate Management</span>
            <button class='warn' onclick='openAddDialog()'>Add Colnect As Stamp</button>
            <button class='danger' onclick='deleteCurrentCandidate()'>Delete Candidate</button>
            <button onclick='undoLastDelete()'>Undo Delete</button>
        </div>
    </div>
  <div id='comparePanels' class='row'>
    <div class='col'>
            <div class='titleRow'>
                <h3>Left: StampWorld Base</h3>
                <span id='leftPos' class='badge'></span>
            </div>
                        <div id='leftActionBanner' class='leftActionBanner'></div>
      <div id='leftPanel'></div>
    </div>
    <div class='col'>
            <div class='titleRow'>
                <h3>Right: Colnect Candidate</h3>
                <span id='rightPos' class='badge'></span>
            </div>
                        <div id='rightActionBanner' class='rightActionBanner'></div>
      <div id='rightPanel'></div>
    </div>
  </div>
</div>

<div id='addModalBackdrop' class='modal-backdrop'>
    <div class='modal'>
        <h4>Add Colnect Candidate To stamps</h4>
        <div class='rowx'>
            <label><span id='addCatalogLabel'>Catalog</span> <input id='addSg' type='text' /></label>
            <label>Year <input id='addYear' type='number' /></label>
            <label>Series <input id='addSeries' type='text' /></label>
        </div>
        <div class='rowx'>
            <label><input id='addAssociateImage' type='checkbox' checked onchange='toggleAddImageOptions()'/> Associate image</label>
        </div>
        <div id='addImageOptions'>
            <div class='rowx'>
                <label><input type='radio' name='addImageMode' value='current_left' checked onchange='toggleAddImageOptions()'/> Use current left image</label>
                <label><input type='radio' name='addImageMode' value='upload' onchange='toggleAddImageOptions()'/> Upload image file</label>
                <label><input type='radio' name='addImageMode' value='none' onchange='toggleAddImageOptions()'/> No image</label>
            </div>
            <div class='rowx' id='uploadRow' style='display:none;'>
                <label>Upload <input id='addUploadFile' type='file' accept='image/png,image/jpeg,image/webp'/></label>
            </div>
        </div>
        <div class='rowx' style='justify-content:flex-end; margin-top: 12px;'>
            <button onclick='closeAddDialog()'>Cancel</button>
            <button onclick='addCurrentColnectAsStamp()'>Add</button>
        </div>
    </div>
</div>

<div id='leftEditorBackdrop' class='modal-backdrop'>
    <div class='modal leftEditorDialog leftEditorCompact'>
        <div class='leftEditorHead'>
            <h4 id='leftEditorTitle'>Edit Base Stamp</h4>
            <p id='leftEditorHint' class='leftEditorHint'>Use this for StampWorld corrections, watermark/reference images, or fixing a bad Colnect placement.</p>
        </div>
        <div class='leftEditorBody'>
            <section class='leftEditorSection'>
                <h5>Base Stamp Details</h5>
                <div class='leftEditorSectionBody'>
                    <div class='leftEditorGrid'>
                        <label>SG / Ref Code <input id='leftEditorSg' type='text' /></label>
                        <label>Year <input id='leftEditorYear' type='number' /></label>
                        <label>Series <input id='leftEditorSeries' type='text' /></label>
                    </div>
                    <div class='leftEditorStack'>
                        <label>Description <textarea id='leftEditorDescription'></textarea></label>
                        <label>Watermark <textarea id='leftEditorWatermark'></textarea></label>
                        <label>Design <textarea id='leftEditorDesign'></textarea></label>
                        <label>Engraving <textarea id='leftEditorEngraving'></textarea></label>
                        <label>Perforation <textarea id='leftEditorPerforation'></textarea></label>
                        <label>Context / Notes <textarea id='leftEditorContext'></textarea></label>
                    </div>
                </div>
            </section>
            <section class='leftEditorSection'>
                <h5>Dimensions</h5>
                <div class='leftEditorSectionBody'>
                    <div class='leftEditorDimsGrid'>
                        <label>Width <input id='leftEditorWidth' type='text' placeholder='e.g. 25' /></label>
                        <label>Height <input id='leftEditorHeight' type='text' placeholder='e.g. 30' /></label>
                        <label>Colnect Width <input id='leftEditorColnectWidth' type='text' /></label>
                        <label>Colnect Height <input id='leftEditorColnectHeight' type='text' /></label>
                    </div>
                </div>
            </section>
            <section class='leftEditorSection'>
                <h5>Colnect Metadata</h5>
                <div class='leftEditorSectionBody'>
                    <div class='leftEditorGrid'>
                        <label>Colnect ID <input id='leftEditorColnectId' type='text' /></label>
                        <label>Colnect Name <input id='leftEditorColnectName' type='text' /></label>
                        <label>Colnect Country <input id='leftEditorColnectCountry' type='text' /></label>
                        <label>Catalog Codes <input id='leftEditorColnectCatalogCodes' type='text' /></label>
                        <label>Colnect Page URL <input id='leftEditorColnectPageUrl' type='text' /></label>
                        <label>Colnect Image URL <input id='leftEditorColnectImageUrl' type='text' /></label>
                        <label>Format <input id='leftEditorColnectFormat' type='text' /></label>
                        <label>Emission <input id='leftEditorColnectEmission' type='text' /></label>
                        <label>Printing <input id='leftEditorColnectPrinting' type='text' /></label>
                        <label>Paper <input id='leftEditorColnectPaper' type='text' /></label>
                        <label>Gum <input id='leftEditorColnectGum' type='text' /></label>
                        <label>Face Value <input id='leftEditorColnectFaceValue' type='text' /></label>
                        <label>Currency <input id='leftEditorColnectCurrency' type='text' /></label>
                        <label>Ord MI <input id='leftEditorColnectOrdMi' type='text' /></label>
                        <label>Ord SN <input id='leftEditorColnectOrdSn' type='text' /></label>
                        <label>Ord YT <input id='leftEditorColnectOrdYt' type='text' /></label>
                        <label>Ord SG <input id='leftEditorColnectOrdSg' type='text' /></label>
                        <label>Expiry Date <input id='leftEditorColnectExpiryDate' type='text' /></label>
                        <label>Print Run <input id='leftEditorColnectPrintRun' type='text' /></label>
                        <label>Colors <input id='leftEditorColnectColors' type='text' /></label>
                        <label>Parent Item <input id='leftEditorColnectParentItem' type='text' /></label>
                        <label>Variant <input id='leftEditorColnectVariant' type='text' /></label>
                        <label>Overprint Variant <input id='leftEditorColnectOverprintVariant' type='text' /></label>
                        <label>Score <input id='leftEditorColnectScore' type='text' /></label>
                        <label>Accuracy <input id='leftEditorColnectAccuracy' type='text' /></label>
                        <label>List <input id='leftEditorColnectList' type='text' /></label>
                        <label>Quantity <input id='leftEditorColnectQuantity' type='text' /></label>
                        <label>Condition <input id='leftEditorColnectCondition' type='text' /></label>
                    </div>
                    <div class='leftEditorStack'>
                        <label>Public Note <textarea id='leftEditorColnectPublicNote'></textarea></label>
                        <label>Private Note <textarea id='leftEditorColnectPrivateNote'></textarea></label>
                    </div>
                </div>
            </section>
            <section class='leftEditorSection'>
                <h5>Image Handling</h5>
                <div class='leftEditorSectionBody'>
                    <div class='leftEditorHint'>Items without an image will not stay visible in the left review list.</div>
                    <div class='leftEditorModes'>
                        <label id='leftModeKeepWrap'><input id='leftModeKeep' type='radio' name='leftImageMode' value='keep' checked onchange='toggleLeftEditorImageOptions()'/> Keep current image</label>
                        <label id='leftModeCurrentWrap'><input id='leftModeCurrent' type='radio' name='leftImageMode' value='current_left' onchange='toggleLeftEditorImageOptions()'/> Use current left image</label>
                        <label><input id='leftModeUpload' type='radio' name='leftImageMode' value='upload' onchange='toggleLeftEditorImageOptions()'/> Upload image</label>
                    </div>
                    <div id='leftUploadRow' class='rowx' style='display:none;'>
                        <label>Upload <input id='leftEditorUploadFile' type='file' accept='image/png,image/jpeg,image/webp'/></label>
                    </div>
                </div>
            </section>
        </div>
        <div class='leftEditorActions'>
            <button onclick='closeLeftEditor()'>Cancel</button>
            <button class='primary' onclick='submitLeftEditor()'>Save</button>
        </div>
    </div>
</div>
<div id='opBusyBackdrop' class='opBusyBackdrop'>
    <div class='opBusyCard'>
        <div class='opBusyRow'>
            <span class='opBusySpinner'></span>
            <div class='opBusyText'>
                <strong id='opBusyTitle'>Working<span id='opBusyDots' class='opBusyDots'></span></strong><br/>
                <span id='opBusyText'>Please wait...</span>
            </div>
        </div>
    </div>
</div>
<script>
const payload = {json};
let left = payload.left || [];
let rightAll = payload.right || [];
const showImagelessLeftRowsDefault = payload.show_imageless_left_rows === true;
const configDefaultCatalogCode = String(payload.default_catalog_code || '').toLowerCase();
const countryCatalogStorageKey = 'reviewui.defaultCatalog.' + String(payload.album_name || 'default');
let countryDefaultCatalogCode = '';
let leftIdx = 0;
let rightIdxByLeft = {{}};
let assignedByLeft = {{}}; // left_rowid -> colnect_id
let selectedCatalogCodeByLeft = {{}}; // left_rowid -> chosen code
let selectedCatalogTypeByLeft = {{}}; // left_rowid -> chosen catalog family
let deletedCandidatesStack = []; // stack entries: candidate + removedAssignments
let guideDismissed = false;
let resolvingRightImageById = {{}};
let resolvedRightImageById = {{}};
let leftEditorMode = 'edit';
let leftEditorSourceRowid = null;
let sortMode = 'sg';
let leftSearchMatches = [];
let leftSearchMatchPos = -1;
let leftSearchQuery = '';
let rightSearchQuery = '';
let opBusyDotsTimer = null;
window.__returnToStartInFlight = false;

function setOperationBusy(active, message) {{
    const backdrop = document.getElementById('opBusyBackdrop');
    const text = document.getElementById('opBusyText');
    const dots = document.getElementById('opBusyDots');
    if (!backdrop || !text || !dots) return;

    if (!active) {{
        backdrop.style.display = 'none';
        if (opBusyDotsTimer) {{
            clearInterval(opBusyDotsTimer);
            opBusyDotsTimer = null;
        }}
        dots.innerText = '';
        return;
    }}

    text.innerText = message || 'Please wait...';
    backdrop.style.display = 'flex';
    if (!opBusyDotsTimer) {{
        let idx = 0;
        const frames = ['', '.', '..', '...'];
        opBusyDotsTimer = setInterval(() => {{
            idx = (idx + 1) % frames.length;
            dots.innerText = frames[idx];
        }}, 260);
    }}
}}

async function navigateWithFade(url) {{
    try {{
        if (document.body) document.body.classList.add('pageLeaving');
        await new Promise(resolve => setTimeout(resolve, 140));
    }} catch (_) {{}}
    window.location.href = url;
}}

async function waitForBootstrapReady(maxAttempts, delayMs) {{
    for (let i = 0; i < maxAttempts; i++) {{
        const controller = new AbortController();
        const timer = setTimeout(() => controller.abort(), Math.max(350, delayMs));
        try {{
            const res = await fetch('/bootstrap-state', {{
                cache: 'no-store',
                signal: controller.signal,
            }});
            if (res.ok) return true;
        }} catch (_) {{
            // Ignore transient network errors while the server switches modes.
        }} finally {{
            clearTimeout(timer);
        }}
        await new Promise(resolve => setTimeout(resolve, delayMs));
    }}
    return false;
}}

function removeCandidateFromUi(colnectId) {{
    if (!colnectId) return;
    rightAll = rightAll.filter(c => c.colnect_id !== colnectId);
    Object.keys(assignedByLeft).forEach(k => {{
        if (assignedByLeft[k] === colnectId) delete assignedByLeft[k];
    }});
    Object.keys(selectedCatalogCodeByLeft).forEach(k => {{
        if (!assignedByLeft[k]) delete selectedCatalogCodeByLeft[k];
    }});
    Object.keys(selectedCatalogTypeByLeft).forEach(k => {{
        if (!assignedByLeft[k]) delete selectedCatalogTypeByLeft[k];
    }});
    for (const key of Object.keys(rightIdxByLeft)) {{
        rightIdxByLeft[key] = 0;
    }}
    for (let i = deletedCandidatesStack.length - 1; i >= 0; i -= 1) {{
        if (deletedCandidatesStack[i]?.candidate?.colnect_id === colnectId) {{
            deletedCandidatesStack.splice(i, 1);
        }}
    }}
}}
const rowOddColor = (payload.odd_row_color || '').trim() || '#ffffff';
const rowEvenColor = (payload.even_row_color || '').trim() || '#e6eef4';
const checkboxDefaults = {{
  f_series: payload.series_default_checked !== false,
  f_description: payload.description_default_checked !== false,
  f_watermark: payload.watermark_default_checked !== false,
  f_design: payload.design_default_checked !== false,
  f_engraving: payload.engraving_default_checked !== false,
  f_perforation: payload.perforation_default_checked !== false,
  f_context: payload.context_default_checked !== false,
  f_links: payload.colnect_links_default_checked !== false,
  f_image_url: payload.colnect_image_url_default_checked !== false,
  f_colnect_meta: payload.colnect_metadata_default_checked !== false,
  f_replace_sg: payload.replace_sg_number_default_checked !== false,
  f_right_unmatched_only: payload.right_unmatched_only_default_checked !== false
}};
const pdfGenerationDefaults = {{
    enabledByDefault: payload.albumeasy_auto_generate_pdf_default !== false,
    cliConfigured: payload.albumeasy_cli_configured !== false
}};
document.documentElement.style.setProperty('--row-odd-bg', rowOddColor);
document.documentElement.style.setProperty('--row-even-bg', rowEvenColor);

function b(v) {{ return !!v && String(v).trim().length > 0; }}
function fmt(v) {{ return b(v) ? String(v) : '(no info)'; }}
function html(v) {{ return String(v)
  .replaceAll('&','&amp;')
  .replaceAll('<','&lt;')
  .replaceAll('>','&gt;')
  .replaceAll('"','&quot;'); }}
function row(k, v) {{ return `<tr><td class='k'>${{html(k)}}</td><td class='v'>${{html(fmt(v))}}</td></tr>`; }}
function allDbRows(fields) {{
    if (!Array.isArray(fields) || !fields.length) return row('DB Fields', '(no fields)');
    return fields.map(f => row(`DB:${{f.key || ''}}`, f.value || '')).join('');
}}
function toggleImagelessLeftRows() {{
    const checkbox = document.getElementById('f_show_imageless_left');
    const url = new URL(window.location.href);
    if (checkbox?.checked) {{
        url.searchParams.set('show_no_image', '1');
    }} else {{
        url.searchParams.delete('show_no_image');
    }}
    window.location.href = url.pathname + url.search;
}}
function catalogDisplayName(code) {{
    const c = String(code || '').trim().toLowerCase();
    if (c === 'sn') return 'Scott';
    if (c === 'sg') return 'Stanley Gibbons';
    if (c === 'mi') return 'Michel';
    if (c === 'yt') return 'Yvert';
    if (c === 'afa') return 'AFA';
    if (c === 'nvp') return 'NVP';
    if (c === 'un') return 'Unificato';
    if (c === 'col') return 'Colnect';
    return c ? c.toUpperCase() : 'Catalog';
}}

function getEffectiveDefaultCatalogCode() {{
    return String(countryDefaultCatalogCode || configDefaultCatalogCode || 'sg').toLowerCase();
}}

function changeCountryDefaultCatalog() {{
    const sel = document.getElementById('defaultCatalogSelect');
    if (!sel) return;
    countryDefaultCatalogCode = String(sel.value || 'sg').toLowerCase();
    try {{
        localStorage.setItem(countryCatalogStorageKey, countryDefaultCatalogCode);
    }} catch (_) {{}}
    render();
}}

async function saveCatalogDefaultToConfig() {{
    const sel = document.getElementById('defaultCatalogSelect');
    const statusEl = document.getElementById('status');
    const code = String(sel?.value || '').toLowerCase();
    if (!code) {{
        alert('Choose a catalog first.');
        return;
    }}

    if (statusEl) statusEl.innerText = 'Updating config default catalog...';
    try {{
        const r = await fetch('/api/default-catalog-config', {{
            method: 'POST',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{ default_catalog_code: code }})
        }});
        const j = await r.json();
        if (j.ok) {{
            if (statusEl) statusEl.innerText = 'Config default updated';
            alert(j.message || 'Config updated.');
        }} else {{
            if (statusEl) statusEl.innerText = 'Config update failed';
            alert(j.message || 'Failed to update config default.');
        }}
    }} catch (e) {{
        if (statusEl) statusEl.innerText = 'Config update error';
        alert('Config update error: ' + e);
    }}
}}
function normalizeCatalogNumber(v) {{
    let s = String(v || '').trim();
    if (!s) return '';
    const colonIdx = s.indexOf(':');
    if (colonIdx >= 0) s = s.slice(colonIdx + 1).trim();
    const m = s.match(/^([A-Za-z]+)\s+(.+)$/);
    if (m && m[2]) return String(m[2]).trim();
    return s;
}}
function imageBox(src, label) {{
  if (b(src)) return `<div class='imgbox'><img src='${{src}}' alt='${{html(label)}}'/></div>`;
  return `<div class='imgbox'><div class='imgph'>${{html(label)}}<br/>No image available</div></div>`;
}}

function leftSortKey(item) {{
    const year = Number(item?.year || 0);
    const sg = String(item?.sg_number || '');
    return [year, sg.toLowerCase(), Number(item?.rowid || 0)];
}}

function sgSortText(item) {{
    return normalizeCatalogNumber(String(item?.sg_number || '')).toLowerCase();
}}

function compareBySortMode(a, b, mode) {{
    const m = String(mode || 'sg');
    if (m === 'rowid') {{
        return Number(a?.rowid || 0) - Number(b?.rowid || 0);
    }}
    if (m === 'year_sg') {{
        const ay = Number(a?.year || 0);
        const by = Number(b?.year || 0);
        if (ay !== by) return ay - by;
        const asg = sgSortText(a);
        const bsg = sgSortText(b);
        if (asg !== bsg) return asg.localeCompare(bsg, undefined, {{ numeric: true, sensitivity: 'base' }});
        return Number(a?.rowid || 0) - Number(b?.rowid || 0);
    }}
    if (m === 'series_sg') {{
        const aser = String(a?.series || '').toLowerCase();
        const bser = String(b?.series || '').toLowerCase();
        if (aser !== bser) return aser.localeCompare(bser, undefined, {{ numeric: true, sensitivity: 'base' }});
        const asg = sgSortText(a);
        const bsg = sgSortText(b);
        if (asg !== bsg) return asg.localeCompare(bsg, undefined, {{ numeric: true, sensitivity: 'base' }});
        return Number(a?.rowid || 0) - Number(b?.rowid || 0);
    }}
    if (m === 'colnect_id') {{
        const acid = String(a?.colnect_id || '').toLowerCase();
        const bcid = String(b?.colnect_id || '').toLowerCase();
        if (acid !== bcid) return acid.localeCompare(bcid, undefined, {{ numeric: true, sensitivity: 'base' }});
        return Number(a?.rowid || 0) - Number(b?.rowid || 0);
    }}

    const asg = sgSortText(a);
    const bsg = sgSortText(b);
    if (asg !== bsg) return asg.localeCompare(bsg, undefined, {{ numeric: true, sensitivity: 'base' }});
    const ay = Number(a?.year || 0);
    const by = Number(b?.year || 0);
    if (ay !== by) return ay - by;
    return Number(a?.rowid || 0) - Number(b?.rowid || 0);
}}

function sortLeftRows(preferredRowid) {{
    left.sort((a, b) => compareBySortMode(a, b, sortMode));
    if (!left.length) {{
        leftIdx = 0;
        updateLeftSearchMatches(true);
        return;
    }}
    const idx = preferredRowid == null ? -1 : left.findIndex(item => String(item.rowid) === String(preferredRowid));
    if (idx >= 0) {{
        leftIdx = idx;
    }} else if (leftIdx >= left.length) {{
        leftIdx = left.length - 1;
    }}
    updateLeftSearchMatches(true);
}}

function upsertLeftRow(updatedLeft) {{
    if (!updatedLeft || updatedLeft.rowid == null) return;
    const idx = left.findIndex(item => String(item.rowid) === String(updatedLeft.rowid));
    if (idx >= 0) left[idx] = updatedLeft;
    else left.push(updatedLeft);
    sortLeftRows(updatedLeft.rowid);
}}

function removeLeftRow(rowid) {{
    const before = left.length;
    left = left.filter(item => String(item.rowid) !== String(rowid));
    if (left.length !== before) sortLeftRows(null);
}}

function setLeftSearchStatus(text) {{
    const el = document.getElementById('leftSearchStatus');
    if (el) el.innerText = text;
}}

function leftSearchText(item) {{
    const parts = [
        item?.sg_number,
        item?.series,
        item?.description,
        item?.watermark,
        item?.context,
        item?.colnect_id,
        item?.colnect_name,
        item?.colnect_catalog_codes,
        item?.colnect_country,
        item?.year,
        item?.rowid,
    ];
    return parts
        .map(v => String(v || '').toLowerCase())
        .join(' | ');
}}

function rightSearchText(item) {{
    const parts = [
        item?.colnect_id,
        item?.sg_number,
        item?.series,
        item?.name,
        item?.country,
        item?.catalog_codes_text,
        item?.description,
        item?.watermark,
        item?.format,
        item?.emission,
        item?.printing,
        item?.paper,
        item?.gum,
        item?.context,
        item?.year,
    ];
    return parts
        .map(v => String(v || '').toLowerCase())
        .join(' | ');
}}

function setRightSearchStatus(text) {{
    const el = document.getElementById('rightSearchStatus');
    if (el) el.innerText = text;
}}

function updateRightSearchStatus() {{
    const l = left[leftIdx];
    if (!l) {{
        setRightSearchStatus('Matches: 0/0');
        return;
    }}
    const avail = availableCandidatesForCurrentLeft();
    if (!avail.length) {{
        setRightSearchStatus('Matches: 0/0');
        return;
    }}
    const key = String(l.rowid);
    if (!(key in rightIdxByLeft) || rightIdxByLeft[key] >= avail.length) rightIdxByLeft[key] = 0;
    setRightSearchStatus(`Matches: ${{(rightIdxByLeft[key] || 0) + 1}}/${{avail.length}}`);
}}

function onRightSearchInput() {{
    const input = document.getElementById('rightSearchInput');
    const selStart = input ? input.selectionStart : null;
    const selEnd   = input ? input.selectionEnd   : null;
    if (input) rightSearchQuery = String(input.value || '');
    const l = left[leftIdx];
    if (l) rightIdxByLeft[String(l.rowid)] = 0;
    render();
    const newInput = document.getElementById('rightSearchInput');
    if (newInput) {{
        newInput.focus();
        if (selStart !== null) newInput.setSelectionRange(selStart, selEnd);
    }}
}}

function updateLeftSearchMatches(preserveCurrentSelection) {{
    const input = document.getElementById('leftSearchInput');
    if (input) leftSearchQuery = String(input.value || '');
    const query = String(leftSearchQuery || '').trim().toLowerCase();

    if (!left.length) {{
        leftSearchMatches = [];
        leftSearchMatchPos = -1;
        setLeftSearchStatus('Matches: 0/0');
        return;
    }}

    if (!query) {{
        leftSearchMatches = left.map((_, idx) => idx);
    }} else {{
        leftSearchMatches = left
            .map((item, idx) => (leftSearchText(item).includes(query) ? idx : -1))
            .filter(idx => idx >= 0);
    }}

    if (!leftSearchMatches.length) {{
        leftSearchMatchPos = -1;
        setLeftSearchStatus('Matches: 0/0');
        return;
    }}

    if (preserveCurrentSelection) {{
        const currentPos = leftSearchMatches.indexOf(leftIdx);
        if (currentPos >= 0) {{
            leftSearchMatchPos = currentPos;
            setLeftSearchStatus(`Matches: ${{leftSearchMatchPos + 1}}/${{leftSearchMatches.length}}`);
            return;
        }}
    }}

    leftSearchMatchPos = 0;
    leftIdx = leftSearchMatches[0];
    setLeftSearchStatus(`Matches: 1/${{leftSearchMatches.length}}`);
}}

function onLeftSearchInput() {{
    const input = document.getElementById('leftSearchInput');
    if (input) leftSearchQuery = String(input.value || '');
    const before = leftIdx;
    updateLeftSearchMatches(true);
    if (leftIdx !== before) render();
}}

function stepLeftSearch(delta) {{
    updateLeftSearchMatches(true);
    if (!leftSearchMatches.length) return;

    const total = leftSearchMatches.length;
    if (leftSearchMatchPos < 0) leftSearchMatchPos = 0;
    leftSearchMatchPos = (leftSearchMatchPos + delta + total) % total;
    leftIdx = leftSearchMatches[leftSearchMatchPos];
    setLeftSearchStatus(`Matches: ${{leftSearchMatchPos + 1}}/${{total}}`);
    render();
}}

async function ensureRightImageUrl(r) {{
    if (!r || !b(r.colnect_id) || !b(r.page_url)) return;
    if (b(r.image_abs) || b(r.image_url)) return;

    const key = String(r.colnect_id);
    if (key in resolvedRightImageById) {{
        r.image_url = resolvedRightImageById[key] || '';
        return;
    }}
    if (resolvingRightImageById[key]) return;

    resolvingRightImageById[key] = true;
    try {{
        const res = await fetch(`/resolve-colnect-image?page_url=${{encodeURIComponent(r.page_url)}}`);
        const j = await res.json();
        const url = (j && j.ok && b(j.image_url)) ? String(j.image_url) : '';
        resolvedRightImageById[key] = url;
        r.image_url = url;

        const current = currentRightCandidate();
        if (current && String(current.colnect_id) === key) {{
            render();
        }}
    }} catch (_) {{
        resolvedRightImageById[key] = '';
    }} finally {{
        delete resolvingRightImageById[key];
    }}
}}

function dismissStartGuide() {{
    guideDismissed = true;
    const guide = document.getElementById('startGuide');
    if (guide) guide.style.display = 'none';
}}

function scrollToFilters() {{
    document.getElementById('filtersCard')?.scrollIntoView({{ behavior: 'smooth', block: 'start' }});
}}

function scrollToPanels() {{
    document.getElementById('comparePanels')?.scrollIntoView({{ behavior: 'smooth', block: 'start' }});
}}

function updateGuide() {{
    const leftCount = left.length;
    const rightCount = rightAll.length;
    const assignedCount = Object.keys(assignedByLeft).length;

    const leftBadge = document.getElementById('statLeftCount');
    const rightBadge = document.getElementById('statRightCount');
    const assignedBadge = document.getElementById('statAssignedCount');
    if (leftBadge) leftBadge.innerText = `Left: ${{leftCount}}`;
    if (rightBadge) rightBadge.innerText = `Candidates: ${{rightCount}}`;
    if (assignedBadge) assignedBadge.innerText = `Assigned: ${{assignedCount}}`;

    const guide = document.getElementById('startGuide');
    if (!guide) return;
    if (guideDismissed) {{
        guide.style.display = 'none';
        return;
    }}
    guide.style.display = '';
}}

function usedColnectIds(exceptLeftRowId) {{
  const used = new Set();
  for (const [l, cid] of Object.entries(assignedByLeft)) {{
    if (String(exceptLeftRowId) === String(l)) continue;
    used.add(cid);
  }}
  return used;
}}

function availableCandidatesForCurrentLeft() {{
  if (!left.length) return [];
  const currentLeft = left[leftIdx];
  const used = usedColnectIds(currentLeft.rowid);
  const assignedForCurrent = assignedByLeft[currentLeft.rowid];
    const unmatchedOnly = !!document.getElementById('f_right_unmatched_only')?.checked;
    const matchedInDb = new Set(left.map(s => String(s.colnect_id || '').trim()).filter(v => !!v));
    const filtered = rightAll.filter(c => {{
        const keepUsed = (!used.has(c.colnect_id) || c.colnect_id === assignedForCurrent);
        if (!keepUsed) return false;
        if (!unmatchedOnly) return true;
        if (c.colnect_id === assignedForCurrent) return true;
        return !matchedInDb.has(String(c.colnect_id || '').trim());
    }});
    filtered.sort((a, b) => compareBySortMode(a, b, sortMode));

    const query = String(rightSearchQuery || '').trim().toLowerCase();
    if (!query) return filtered;
    return filtered.filter(item => rightSearchText(item).includes(query));
}}

function normalizeIndex(cur, len) {{
    if (len <= 0) return 0;
    if (!Number.isFinite(cur)) return 0;
    if (cur < 0) return 0;
    if (cur >= len) return len - 1;
    return cur;
}}

function jumpLeft(delta) {{
    if (!left.length) return;
    leftIdx = normalizeIndex(leftIdx + Number(delta || 0), left.length);
    render();
}}

function firstLeft() {{
    if (!left.length) return;
    leftIdx = 0;
    render();
}}

function lastLeft() {{
    if (!left.length) return;
    leftIdx = left.length - 1;
    render();
}}

function jumpRight(delta) {{
    const l = left[leftIdx];
    if (!l) return;
    const avail = availableCandidatesForCurrentLeft();
    if (!avail.length) return;
    const key = String(l.rowid);
    const cur = rightIdxByLeft[key] || 0;
    rightIdxByLeft[key] = normalizeIndex(cur + Number(delta || 0), avail.length);
    render();
}}

function firstRight() {{
    const l = left[leftIdx];
    if (!l) return;
    const avail = availableCandidatesForCurrentLeft();
    if (!avail.length) return;
    rightIdxByLeft[String(l.rowid)] = 0;
    render();
}}

function lastRight() {{
    const l = left[leftIdx];
    if (!l) return;
    const avail = availableCandidatesForCurrentLeft();
    if (!avail.length) return;
    rightIdxByLeft[String(l.rowid)] = avail.length - 1;
    render();
}}

function changeSortMode() {{
    const sel = document.getElementById('sortMode');
    sortMode = String(sel?.value || 'sg');
    const currentRowid = left[leftIdx]?.rowid;
    sortLeftRows(currentRowid);
    for (const key of Object.keys(rightIdxByLeft)) rightIdxByLeft[key] = 0;
    render();
}}

function currentRightCandidate() {{
  const avail = availableCandidatesForCurrentLeft();
  if (!avail.length) return null;
  const key = String(left[leftIdx].rowid);
  if (!(key in rightIdxByLeft) || rightIdxByLeft[key] >= avail.length) rightIdxByLeft[key] = 0;
  return avail[rightIdxByLeft[key]];
}}

function selectedCatalogCodeForCurrent(r) {{
    const l = left[leftIdx];
    if (!l || !r) return '';
    const existing = selectedCatalogCodeByLeft[l.rowid] || '';
    if (existing) return existing;
    const pairs = r.catalog_code_pairs || [];
    if (!pairs.length) return '';
    const preferredCode = getEffectiveDefaultCatalogCode();
    const preferred = pairs.find(p => String(p.catalog_type || '').toLowerCase() === preferredCode);
    return preferred ? preferred.code : (pairs[0].code || '');
}}

function selectedCatalogTypeForCurrent(r) {{
    const l = left[leftIdx];
    if (!l || !r) return '';
    const existingType = selectedCatalogTypeByLeft[l.rowid] || '';
    if (existingType) return String(existingType).toLowerCase();
    const selectedCode = selectedCatalogCodeForCurrent(r);
    const pairs = r.catalog_code_pairs || [];
    if (!pairs.length) return '';
    const matched = pairs.find(p => String(p.code || '') === String(selectedCode || ''));
    if (matched && matched.catalog_type) return String(matched.catalog_type).toLowerCase();
    const preferredCode = getEffectiveDefaultCatalogCode();
    const preferred = pairs.find(p => String(p.catalog_type || '').toLowerCase() === preferredCode);
    if (preferred && preferred.catalog_type) return String(preferred.catalog_type).toLowerCase();
    return String(pairs[0].catalog_type || '').toLowerCase();
}}

function render() {{
    updateGuide();
  if (!left.length) {{
        document.getElementById('leftPos').innerText = 'Left 0/0';
        document.getElementById('rightPos').innerText = 'Candidate 0/0';
    document.getElementById('leftActionBanner').innerHTML = `
        <div class='leftActionTop'>
            <strong>Manual Base Stamp Tools</strong>
            <span class='badge'>No left rows loaded</span>
        </div>
        <p class='leftActionText'>Add a base/reference item here when you need a watermark image or a missing left-side record before matching Colnect data.${{showImagelessLeftRowsDefault ? ' Image-less rows are currently visible.' : ''}}</p>
        <div class='leftActionBtns'>
            <button class='warn' onclick='openLeftEditor("add")'>Add Base / Watermark Item</button>
        </div>`;
    document.getElementById('leftPanel').innerHTML = '<div class="imgph">No StampWorld rows with real images found</div>';
    document.getElementById('rightPanel').innerHTML = '<div class="imgph">No candidates</div>';
    return;
  }}

  const seriesEl = document.getElementById('f_series');
  if (seriesEl && !seriesEl.dataset.userTouched) {{
    seriesEl.checked = checkboxDefaults.f_series;
  }}

  const l = left[leftIdx];
  const leftImg = b(l.image_abs) ? `/image?path=${{encodeURIComponent(l.image_abs)}}` : l.stampworld_image_url;
  document.getElementById('leftPos').innerText = `Left ${{leftIdx + 1}}/${{left.length}} (rowid=${{l.rowid}})`;
  document.getElementById('leftActionBanner').innerHTML = `
        <div class='leftActionTop'>
            <strong>Manual Base Stamp Tools</strong>
            <div class='leftActionNote'>
                <span class='badge'>Current rowid=${{html(l.rowid)}}</span>
                <span class='badge'>${{b(l.colnect_id) ? `Linked Colnect: ${{html(l.colnect_id)}}` : 'No Colnect link'}}</span>
            </div>
        </div>
        <p class='leftActionText'>Use these when the StampWorld base row is wrong, when a watermark/reference image should live on the left, or when a Colnect import landed on the wrong base row.${{showImagelessLeftRowsDefault ? ' Image-less rows are visible in this session.' : ' New left items need an image to remain visible here.'}}</p>
        <div class='leftActionBtns'>
            <button onclick='openLeftEditor("edit")'>Edit Current</button>
            <button class='warn' onclick='openLeftEditor("add")'>Add Base / Watermark Item</button>
            <button onclick='clearCurrentLeftColnectLink()' ${{b(l.colnect_id) ? '' : 'disabled'}}>Clear Colnect Link</button>
            <button class='danger' onclick='deleteCurrentLeft()'>Delete Current</button>
        </div>
        <div class='leftSearchRow'>
            <label for='leftSearchInput'>Search Left</label>
            <input id='leftSearchInput' type='text' placeholder='SG, series, name...' oninput='onLeftSearchInput()' value='${{html(leftSearchQuery)}}'/>
            <button onclick='stepLeftSearch(-1)'>Back</button>
            <button onclick='stepLeftSearch(1)'>Forward</button>
            <span id='leftSearchStatus' class='badge'>Matches: 0/0</span>
        </div>
        <div class='leftNavRow'>
            <button class='navBtn' onclick='firstLeft()'>First</button>
            <button class='navBtn' onclick='jumpLeft(-50)'>-50</button>
            <button class='navBtn' onclick='jumpLeft(-10)'>-10</button>
            <button class='navBtn' onclick='prevLeft()'>Prev Left</button>
            <button class='navBtn' onclick='nextLeft()'>Next Left</button>
            <button class='navBtn' onclick='jumpLeft(10)'>+10</button>
            <button class='navBtn' onclick='jumpLeft(50)'>+50</button>
            <button class='navBtn' onclick='lastLeft()'>Last</button>
        </div>`;
  updateLeftSearchMatches(true);
  document.getElementById('rightActionBanner').innerHTML = `
        <div class='rightActionTop'>
            <strong>Candidate Search And Navigation</strong>
            <span class='badge'>Filtered by current left row</span>
        </div>
        <div class='rightBottomBlock'>
        <div class='rightSearchRow'>
            <label for='rightSearchInput'>Search Right</label>
            <input id='rightSearchInput' type='text' placeholder='ID, SG, name, series...' oninput='onRightSearchInput()' value='${{html(rightSearchQuery)}}'/>
            <button onclick='jumpRight(-1)'>Back</button>
            <button onclick='jumpRight(1)'>Forward</button>
            <span id='rightSearchStatus' class='badge'>Matches: 0/0</span>
        </div>
        <div class='rightNavRow'>
            <button class='navBtn' onclick='firstRight()'>First</button>
            <button class='navBtn' onclick='jumpRight(-50)'>-50</button>
            <button class='navBtn' onclick='jumpRight(-10)'>-10</button>
            <button class='navBtn' onclick='prevRight()'>Prev Candidate</button>
            <button class='navBtn' onclick='nextRight()'>Next Candidate</button>
            <button class='navBtn' onclick='jumpRight(10)'>+10</button>
            <button class='navBtn' onclick='jumpRight(50)'>+50</button>
            <button class='navBtn' onclick='lastRight()'>Last</button>
        </div>
        </div>`;
  updateRightSearchStatus();
  document.getElementById('leftPanel').innerHTML = `
        <div class='panelCard'>
            <div class='panelHead'>
                <strong>Base Stamp</strong> SG=${{html(fmt(l.sg_number))}} | Year=${{html(fmt(l.year))}} | Series=${{html(fmt(l.series))}}
            </div>
            <div class='panelBody'>
                ${{imageBox(leftImg, 'StampWorld image')}}
                <table class='kv'>
                    ${{row('SG', l.sg_number)}}
                    ${{row('Year', l.year)}}
                    ${{row('Series', l.series)}}
                    ${{row('Image Path', l.image_path)}}
                    ${{row('StampWorld URL', l.stampworld_image_url)}}
                    ${{row('Description', l.description)}}
                    ${{row('Watermark', l.watermark)}}
                    ${{row('Design', l.design)}}
                    ${{row('Engraving', l.engraving)}}
                    ${{row('Perforation', l.perforation)}}
                    ${{row('Context', l.context)}}
                    ${{row('colnect_page_url', l.colnect_page_url)}}
                    ${{row('colnect_image_url', l.colnect_image_url)}}
                    ${{row('colnect_id', l.colnect_id)}}
                    ${{row('colnect_name', l.colnect_name)}}
                    ${{row('colnect_catalog_codes', l.colnect_catalog_codes)}}
                    ${{row('colnect_country', l.colnect_country)}}
                    ${{row('colnect_width', l.colnect_width)}}
                    ${{row('colnect_height', l.colnect_height)}}
                    ${{row('colnect_format', l.colnect_format)}}
                    ${{row('colnect_emission', l.colnect_emission)}}
                    ${{row('colnect_printing', l.colnect_printing)}}
                    ${{row('colnect_paper', l.colnect_paper)}}
                    ${{row('colnect_gum', l.colnect_gum)}}
                    ${{row('colnect_ord_mi', l.colnect_ord_mi)}}
                    ${{row('colnect_ord_sn', l.colnect_ord_sn)}}
                    ${{row('colnect_ord_yt', l.colnect_ord_yt)}}
                    ${{row('colnect_face_value', l.colnect_face_value)}}
                    ${{row('colnect_currency', l.colnect_currency)}}
                    ${{row('colnect_ord_sg', l.colnect_ord_sg)}}
                    ${{row('colnect_expiry_date', l.colnect_expiry_date)}}
                    ${{row('colnect_print_run', l.colnect_print_run)}}
                    ${{row('colnect_colors', l.colnect_colors)}}
                    ${{row('colnect_parent_item', l.colnect_parent_item)}}
                    ${{row('colnect_variant', l.colnect_variant)}}
                    ${{row('colnect_overprint_variant', l.colnect_overprint_variant)}}
                    ${{row('colnect_score', l.colnect_score)}}
                    ${{row('colnect_accuracy', l.colnect_accuracy)}}
                    ${{row('colnect_list', l.colnect_list)}}
                    ${{row('colnect_quantity', l.colnect_quantity)}}
                    ${{row('colnect_condition', l.colnect_condition)}}
                    ${{row('colnect_public_note', l.colnect_public_note)}}
                    ${{row('colnect_private_note', l.colnect_private_note)}}
                    ${{allDbRows(l.all_db_fields)}}
                    ${{row('Assigned Colnect ID', assignedByLeft[l.rowid] || '(none)')}}
                </table>
            </div>
        </div>
  `;

  const avail = availableCandidatesForCurrentLeft();
  const r = currentRightCandidate();
  if (!r) {{
    document.getElementById('rightPos').innerText = 'No available candidate';
    document.getElementById('rightPanel').innerHTML = '<div class="imgph">All candidates already assigned</div>';
        updateRightSearchStatus();
    return;
  }}

  const key = String(l.rowid);
  document.getElementById('rightPos').innerText = `Candidate ${{(rightIdxByLeft[key] || 0) + 1}}/${{avail.length}}`;
        updateRightSearchStatus();
    ensureRightImageUrl(r);
  const rightImg = b(r.image_abs) ? `/image?path=${{encodeURIComponent(r.image_abs)}}` : r.image_url;
    const catPairs = (r.catalog_code_pairs || []).map(p =>
        `<option value='${{html(p.code)}}' data-cat='${{html(p.catalog_type)}}'>${{html(catalogDisplayName(p.catalog_type))}} (${{html(String(p.catalog_type || '').toUpperCase())}}): ${{html(p.code)}}</option>`
    ).join('');
    const selectedCode = selectedCatalogCodeForCurrent(r);
  document.getElementById('rightPanel').innerHTML = `
        <div class='panelCard'>
            <div class='panelHead'>
               
                <div style='margin-top:8px;'>
                    <label>Catalog Code (for optional SG replace):</label>
                    <select id='catalogCodeSelect' onchange='rememberCatalogCode()'>
                        <option value=''>-- none --</option>
                        ${{catPairs}}
                    </select>
                </div>
            </div>
            <div class='panelBody'>
                ${{imageBox(rightImg, 'Colnect candidate image')}}
                <table class='kv'>
                    ${{row('Colnect ID', r.colnect_id)}}
                    ${{row('SG', r.sg_number)}}
            ${{row('Catalog Codes Text', r.catalog_codes_text)}}
                    ${{row('Year', r.year)}}
            ${{row('Country', r.country)}}
                    ${{row('Series', r.series)}}
                    ${{row('Page URL', r.page_url)}}
                    ${{row('Image URL', r.image_url)}}
            ${{row('Name', r.name)}}
                    ${{row('Description', r.description)}}
                    ${{row('Watermark', r.watermark)}}
            ${{row('Format', r.format)}}
            ${{row('Emission', r.emission)}}
            ${{row('Printing', r.printing)}}
            ${{row('Paper', r.paper)}}
            ${{row('Gum', r.gum)}}
            ${{row('Width', r.width)}}
            ${{row('Height', r.height)}}
            ${{row('Face Value', r.face_value)}}
            ${{row('Currency', r.currency)}}
            ${{row('Colors', r.colors)}}
      ${{row('Design', r.design)}}
      ${{row('Engraving', r.engraving)}}
      ${{row('Perforation', r.perforation)}}
            ${{row('ORD SG', r.ord_sg)}}
            ${{row('ORD MI', r.ord_mi)}}
            ${{row('ORD SN', r.ord_sn)}}
            ${{row('ORD YT', r.ord_yt)}}
            ${{row('Print Run', r.print_run)}}
            ${{row('Expiry Date', r.expiry_date)}}
            ${{row('Parent Item', r.parent_item)}}
            ${{row('Variant', r.variant)}}
            ${{row('Overprint Variant', r.overprint_variant)}}
            ${{row('Score', r.score)}}
            ${{row('Accuracy', r.accuracy)}}
            ${{row('List', r.list)}}
            ${{row('Quantity', r.quantity)}}
            ${{row('Condition', r.condition)}}
            ${{row('Public Note', r.public_note)}}
            ${{row('Private Note', r.private_note)}}
                    ${{row('Context', r.context)}}
                </table>
            </div>
        </div>
  `;

    const sel = document.getElementById('catalogCodeSelect');
    if (sel && selectedCode) sel.value = selectedCode;
}}

function rememberCatalogCode() {{
    const l = left[leftIdx];
    if (!l) return;
    const sel = document.getElementById('catalogCodeSelect');
    if (!sel) return;
    selectedCatalogCodeByLeft[l.rowid] = sel.value || '';
    const selectedOpt = sel.options[sel.selectedIndex];
    selectedCatalogTypeByLeft[l.rowid] = selectedOpt?.dataset?.cat || '';
}}

function prevLeft() {{
  if (!left.length) return;
  leftIdx = (leftIdx - 1 + left.length) % left.length;
  render();
}}
function nextLeft() {{
  if (!left.length) return;
  leftIdx = (leftIdx + 1) % left.length;
  render();
}}
function prevRight() {{
  const l = left[leftIdx];
  const avail = availableCandidatesForCurrentLeft();
  if (!avail.length) return;
  const key = String(l.rowid);
  const cur = rightIdxByLeft[key] || 0;
  rightIdxByLeft[key] = (cur - 1 + avail.length) % avail.length;
  render();
}}
function nextRight() {{
  const l = left[leftIdx];
  const avail = availableCandidatesForCurrentLeft();
  if (!avail.length) return;
  const key = String(l.rowid);
  const cur = rightIdxByLeft[key] || 0;
  rightIdxByLeft[key] = (cur + 1) % avail.length;
  render();
}}
function assignCurrent() {{
  const l = left[leftIdx];
  const r = currentRightCandidate();
    if (!l || !r) return;
    rememberCatalogCode();
  assignedByLeft[l.rowid] = r.colnect_id;
  document.getElementById('status').innerText = `Assigned rowid=${{l.rowid}} -> colnect_id=${{r.colnect_id}}`;
  render();
}}
function clearAssignment() {{
  const l = left[leftIdx];
  if (!l) return;
  delete assignedByLeft[l.rowid];
    delete selectedCatalogCodeByLeft[l.rowid];
    delete selectedCatalogTypeByLeft[l.rowid];
  document.getElementById('status').innerText = `Cleared assignment for rowid=${{l.rowid}}`;
  render();
}}

async function applyMatches() {{
    const matches = Object.entries(assignedByLeft).map(([leftRowid, colnectId]) => {{
        const candidateMap = {{}};
        rightAll.forEach(c => {{ candidateMap[c.colnect_id] = c; }});
        const candidate = candidateMap[colnectId];
        return {{
    left_rowid: Number(leftRowid),
        colnect_id: colnectId,
                selected_catalog_code: selectedCatalogCodeByLeft[leftRowid] || null,
            selected_catalog_type: selectedCatalogTypeByLeft[leftRowid] || null,
                width: candidate?.width || null,
                height: candidate?.height || null
        }};
    }});

    if (!matches.length) {{
        document.getElementById('status').innerText = 'No assigned matches selected. Use Assign Candidate first.';
        return;
    }}

    const fields = {{
        update_series: !!document.getElementById('f_series')?.checked,
        update_description: !!document.getElementById('f_description')?.checked,
        update_watermark: !!document.getElementById('f_watermark')?.checked,
        update_design: !!document.getElementById('f_design')?.checked,
        update_engraving: !!document.getElementById('f_engraving')?.checked,
        update_perforation: !!document.getElementById('f_perforation')?.checked,
        update_context: !!document.getElementById('f_context')?.checked,
        update_colnect_links: !!document.getElementById('f_links')?.checked,
        update_colnect_image_url: !!document.getElementById('f_image_url')?.checked,
        update_colnect_metadata: !!document.getElementById('f_colnect_meta')?.checked,
        replace_sg_number: !!document.getElementById('f_replace_sg')?.checked
    }};

  const res = await fetch('/apply-matches', {{
    method: 'POST',
    headers: {{ 'Content-Type': 'application/json' }},
        body: JSON.stringify({{ matches, fields }})
  }});
  const j = await res.json();
    for (const colnectId of (j.processed_colnect_ids || [])) {{
        removeCandidateFromUi(colnectId);
    }}
  document.getElementById('status').innerText = `Applied: updated=${{j.updated_rows}}, skipped=${{j.skipped_rows}}, errors=${{(j.errors||[]).length}}`;
  if ((j.errors || []).length) {{
    alert(j.errors.join('\n'));
  }}
    render();
}}

function getSelectedAddImageMode() {{
    const radios = document.querySelectorAll("input[name='addImageMode']");
    for (const r of radios) {{
        if (r.checked) return r.value;
    }}
    return 'current_left';
}}

function toggleAddImageOptions() {{
    const assoc = !!document.getElementById('addAssociateImage')?.checked;
    const options = document.getElementById('addImageOptions');
    if (options) options.style.display = assoc ? '' : 'none';
    const uploadRow = document.getElementById('uploadRow');
    const mode = getSelectedAddImageMode();
    if (uploadRow) uploadRow.style.display = assoc && mode === 'upload' ? '' : 'none';
}}

function openAddDialog() {{
    const r = currentRightCandidate();
    if (!r) {{
        document.getElementById('status').innerText = 'No current candidate to add';
        return;
    }}
    const preferredCode = selectedCatalogCodeForCurrent(r) || r.sg_number || '';
    const preferredType = selectedCatalogTypeForCurrent(r) || getEffectiveDefaultCatalogCode() || '';
    const labelText = preferredType
        ? `${{catalogDisplayName(preferredType)}} (${{String(preferredType).toUpperCase()}})`
        : 'Catalog';
    const labelEl = document.getElementById('addCatalogLabel');
    if (labelEl) labelEl.innerText = labelText;
    document.getElementById('addSg').value = normalizeCatalogNumber(preferredCode);
    document.getElementById('addYear').value = Number.isFinite(r.year) ? r.year : '';
    document.getElementById('addSeries').value = r.series || '';
    document.getElementById('addAssociateImage').checked = true;
    const radioCurrent = document.querySelector("input[name='addImageMode'][value='current_left']");
    if (radioCurrent) radioCurrent.checked = true;
    const upload = document.getElementById('addUploadFile');
    if (upload) upload.value = '';
    toggleAddImageOptions();
    document.getElementById('addModalBackdrop').style.display = 'flex';
}}

function closeAddDialog() {{
    const modal = document.getElementById('addModalBackdrop');
    if (modal) modal.style.display = 'none';
}}

function fileToDataUrl(file) {{
    return new Promise((resolve, reject) => {{
        const reader = new FileReader();
        reader.onload = () => resolve(String(reader.result || ''));
        reader.onerror = () => reject(new Error('failed to read upload file'));
        reader.readAsDataURL(file);
    }});
}}

function getSelectedLeftImageMode() {{
    const radios = document.querySelectorAll("input[name='leftImageMode']");
    for (const radio of radios) {{
        if (radio.checked) return radio.value;
    }}
    return leftEditorMode === 'edit' ? 'keep' : 'upload';
}}

function toggleLeftEditorImageOptions() {{
    const mode = getSelectedLeftImageMode();
    const uploadRow = document.getElementById('leftUploadRow');
    const keepWrap = document.getElementById('leftModeKeepWrap');
    const currentWrap = document.getElementById('leftModeCurrentWrap');
    if (keepWrap) keepWrap.style.display = leftEditorMode === 'edit' ? '' : 'none';
    if (currentWrap) currentWrap.style.display = leftEditorMode === 'add' ? '' : 'none';
    if (uploadRow) uploadRow.style.display = mode === 'upload' ? '' : 'none';
}}

function openLeftEditor(mode) {{
    leftEditorMode = mode === 'add' ? 'add' : 'edit';
    const currentLeft = left[leftIdx] || null;
    leftEditorSourceRowid = currentLeft ? Number(currentLeft.rowid) : null;

    const title = document.getElementById('leftEditorTitle');
    const hint = document.getElementById('leftEditorHint');
    if (title) title.innerText = leftEditorMode === 'add' ? 'Add Base / Watermark Item' : 'Edit Base Stamp';
    if (hint) hint.innerText = leftEditorMode === 'add'
        ? 'Create a left-side reference row for a missing base stamp or watermark image. Use a short reference code such as WM-1 when this is not a true stamp number.'
        : 'Correct the current StampWorld base row without leaving the review page.';

    const base = leftEditorMode === 'edit'
        ? currentLeft
        : {{
            sg_number: '',
            year: currentLeft?.year || 0,
            series: currentLeft?.series || '',
            description: '',
            watermark: '',
            design: '',
            engraving: '',
            perforation: '',
            context: '',
            width: '',
            height: '',
            colnect_page_url: '',
            colnect_image_url: '',
            colnect_id: '',
            colnect_name: '',
            colnect_catalog_codes: '',
            colnect_country: '',
            colnect_width: '',
            colnect_height: '',
            colnect_format: '',
            colnect_emission: '',
            colnect_printing: '',
            colnect_paper: '',
            colnect_gum: '',
            colnect_ord_mi: '',
            colnect_ord_sn: '',
            colnect_ord_yt: '',
            colnect_face_value: '',
            colnect_currency: '',
            colnect_ord_sg: '',
            colnect_expiry_date: '',
            colnect_print_run: '',
            colnect_colors: '',
            colnect_parent_item: '',
            colnect_variant: '',
            colnect_overprint_variant: '',
            colnect_score: '',
            colnect_accuracy: '',
            colnect_list: '',
            colnect_quantity: '',
            colnect_condition: '',
            colnect_public_note: '',
            colnect_private_note: ''
        }};

    document.getElementById('leftEditorSg').value = base?.sg_number || '';
    document.getElementById('leftEditorYear').value = Number(base?.year || 0) || '';
    document.getElementById('leftEditorSeries').value = base?.series || '';
    document.getElementById('leftEditorDescription').value = base?.description || '';
    document.getElementById('leftEditorWatermark').value = base?.watermark || '';
    document.getElementById('leftEditorDesign').value = base?.design || '';
    document.getElementById('leftEditorEngraving').value = base?.engraving || '';
    document.getElementById('leftEditorPerforation').value = base?.perforation || '';
    document.getElementById('leftEditorContext').value = base?.context || '';
    document.getElementById('leftEditorWidth').value = base?.width || base?.colnect_width || '';
    document.getElementById('leftEditorHeight').value = base?.height || base?.colnect_height || '';
    document.getElementById('leftEditorColnectPageUrl').value = base?.colnect_page_url || '';
    document.getElementById('leftEditorColnectImageUrl').value = base?.colnect_image_url || '';
    document.getElementById('leftEditorColnectId').value = base?.colnect_id || '';
    document.getElementById('leftEditorColnectName').value = base?.colnect_name || '';
    document.getElementById('leftEditorColnectCatalogCodes').value = base?.colnect_catalog_codes || '';
    document.getElementById('leftEditorColnectCountry').value = base?.colnect_country || '';
    document.getElementById('leftEditorColnectWidth').value = base?.colnect_width || '';
    document.getElementById('leftEditorColnectHeight').value = base?.colnect_height || '';
    document.getElementById('leftEditorColnectFormat').value = base?.colnect_format || '';
    document.getElementById('leftEditorColnectEmission').value = base?.colnect_emission || '';
    document.getElementById('leftEditorColnectPrinting').value = base?.colnect_printing || '';
    document.getElementById('leftEditorColnectPaper').value = base?.colnect_paper || '';
    document.getElementById('leftEditorColnectGum').value = base?.colnect_gum || '';
    document.getElementById('leftEditorColnectOrdMi').value = base?.colnect_ord_mi || '';
    document.getElementById('leftEditorColnectOrdSn').value = base?.colnect_ord_sn || '';
    document.getElementById('leftEditorColnectOrdYt').value = base?.colnect_ord_yt || '';
    document.getElementById('leftEditorColnectFaceValue').value = base?.colnect_face_value || '';
    document.getElementById('leftEditorColnectCurrency').value = base?.colnect_currency || '';
    document.getElementById('leftEditorColnectOrdSg').value = base?.colnect_ord_sg || '';
    document.getElementById('leftEditorColnectExpiryDate').value = base?.colnect_expiry_date || '';
    document.getElementById('leftEditorColnectPrintRun').value = base?.colnect_print_run || '';
    document.getElementById('leftEditorColnectColors').value = base?.colnect_colors || '';
    document.getElementById('leftEditorColnectParentItem').value = base?.colnect_parent_item || '';
    document.getElementById('leftEditorColnectVariant').value = base?.colnect_variant || '';
    document.getElementById('leftEditorColnectOverprintVariant').value = base?.colnect_overprint_variant || '';
    document.getElementById('leftEditorColnectScore').value = base?.colnect_score || '';
    document.getElementById('leftEditorColnectAccuracy').value = base?.colnect_accuracy || '';
    document.getElementById('leftEditorColnectList').value = base?.colnect_list || '';
    document.getElementById('leftEditorColnectQuantity').value = base?.colnect_quantity || '';
    document.getElementById('leftEditorColnectCondition').value = base?.colnect_condition || '';
    document.getElementById('leftEditorColnectPublicNote').value = base?.colnect_public_note || '';
    document.getElementById('leftEditorColnectPrivateNote').value = base?.colnect_private_note || '';

    const keepRadio = document.getElementById('leftModeKeep');
    const currentRadio = document.getElementById('leftModeCurrent');
    const uploadRadio = document.getElementById('leftModeUpload');
    if (leftEditorMode === 'edit') {{
        if (keepRadio) keepRadio.checked = true;
    }} else if (currentLeft) {{
        if (currentRadio) currentRadio.checked = true;
    }} else if (uploadRadio) {{
        uploadRadio.checked = true;
    }}
    const uploadInput = document.getElementById('leftEditorUploadFile');
    if (uploadInput) uploadInput.value = '';
    toggleLeftEditorImageOptions();
    document.getElementById('leftEditorBackdrop').style.display = 'flex';
}}

function closeLeftEditor() {{
    const backdrop = document.getElementById('leftEditorBackdrop');
    if (backdrop) backdrop.style.display = 'none';
}}

async function submitLeftEditor() {{
    const sgNumber = document.getElementById('leftEditorSg')?.value || '';
    if (!sgNumber.trim()) {{
        alert('Enter an SG / reference code before saving.');
        return;
    }}

    const yearValue = Number(document.getElementById('leftEditorYear')?.value || 0);
    const mode = getSelectedLeftImageMode();
    let uploadDataUrl = null;
    let uploadFilename = null;
    if (mode === 'upload') {{
        const file = document.getElementById('leftEditorUploadFile')?.files?.[0];
        if (!file) {{
            alert('Choose an image file to upload.');
            return;
        }}
        try {{
            uploadDataUrl = await fileToDataUrl(file);
            uploadFilename = file.name || null;
        }} catch (error) {{
            alert(String(error));
            return;
        }}
    }}

    const body = {{
        sg_number: sgNumber,
        year: Number.isFinite(yearValue) ? yearValue : 0,
        series: document.getElementById('leftEditorSeries')?.value || '',
        description: document.getElementById('leftEditorDescription')?.value || '',
        watermark: document.getElementById('leftEditorWatermark')?.value || '',
        design: document.getElementById('leftEditorDesign')?.value || '',
        engraving: document.getElementById('leftEditorEngraving')?.value || '',
        perforation: document.getElementById('leftEditorPerforation')?.value || '',
        context: document.getElementById('leftEditorContext')?.value || '',
        width: document.getElementById('leftEditorWidth')?.value || '',
        height: document.getElementById('leftEditorHeight')?.value || '',
        colnect_page_url: document.getElementById('leftEditorColnectPageUrl')?.value || '',
        colnect_image_url: document.getElementById('leftEditorColnectImageUrl')?.value || '',
        colnect_id: document.getElementById('leftEditorColnectId')?.value || '',
        colnect_name: document.getElementById('leftEditorColnectName')?.value || '',
        colnect_catalog_codes: document.getElementById('leftEditorColnectCatalogCodes')?.value || '',
        colnect_country: document.getElementById('leftEditorColnectCountry')?.value || '',
        colnect_width: document.getElementById('leftEditorColnectWidth')?.value || '',
        colnect_height: document.getElementById('leftEditorColnectHeight')?.value || '',
        colnect_format: document.getElementById('leftEditorColnectFormat')?.value || '',
        colnect_emission: document.getElementById('leftEditorColnectEmission')?.value || '',
        colnect_printing: document.getElementById('leftEditorColnectPrinting')?.value || '',
        colnect_paper: document.getElementById('leftEditorColnectPaper')?.value || '',
        colnect_gum: document.getElementById('leftEditorColnectGum')?.value || '',
        colnect_ord_mi: document.getElementById('leftEditorColnectOrdMi')?.value || '',
        colnect_ord_sn: document.getElementById('leftEditorColnectOrdSn')?.value || '',
        colnect_ord_yt: document.getElementById('leftEditorColnectOrdYt')?.value || '',
        colnect_face_value: document.getElementById('leftEditorColnectFaceValue')?.value || '',
        colnect_currency: document.getElementById('leftEditorColnectCurrency')?.value || '',
        colnect_ord_sg: document.getElementById('leftEditorColnectOrdSg')?.value || '',
        colnect_expiry_date: document.getElementById('leftEditorColnectExpiryDate')?.value || '',
        colnect_print_run: document.getElementById('leftEditorColnectPrintRun')?.value || '',
        colnect_colors: document.getElementById('leftEditorColnectColors')?.value || '',
        colnect_parent_item: document.getElementById('leftEditorColnectParentItem')?.value || '',
        colnect_variant: document.getElementById('leftEditorColnectVariant')?.value || '',
        colnect_overprint_variant: document.getElementById('leftEditorColnectOverprintVariant')?.value || '',
        colnect_score: document.getElementById('leftEditorColnectScore')?.value || '',
        colnect_accuracy: document.getElementById('leftEditorColnectAccuracy')?.value || '',
        colnect_list: document.getElementById('leftEditorColnectList')?.value || '',
        colnect_quantity: document.getElementById('leftEditorColnectQuantity')?.value || '',
        colnect_condition: document.getElementById('leftEditorColnectCondition')?.value || '',
        colnect_public_note: document.getElementById('leftEditorColnectPublicNote')?.value || '',
        colnect_private_note: document.getElementById('leftEditorColnectPrivateNote')?.value || '',
        image_mode: mode,
        upload_data_url: uploadDataUrl,
        upload_filename: uploadFilename
    }};

    let endpoint = '/save-left-stamp';
    if (leftEditorMode === 'edit') {{
        const currentLeft = left[leftIdx];
        if (!currentLeft) {{
            alert('No current left stamp to edit.');
            return;
        }}
        body.rowid = Number(currentLeft.rowid);
    }} else {{
        endpoint = '/add-left-stamp';
        body.source_rowid = leftEditorSourceRowid;
    }}

    const response = await fetch(endpoint, {{
        method: 'POST',
        headers: {{ 'Content-Type': 'application/json' }},
        body: JSON.stringify(body)
    }});
    const payload = await response.json();
    if (!payload.ok) {{
        alert(payload.message || 'Left-side save failed');
        return;
    }}

    const updatedLeft = payload.updated_left || payload.added_left || null;
    if (updatedLeft) upsertLeftRow(updatedLeft);
    closeLeftEditor();
    document.getElementById('status').innerText = payload.message || 'Left-side item saved';
    render();
}}

async function deleteCurrentLeft() {{
    const currentLeft = left[leftIdx];
    if (!currentLeft) {{
        document.getElementById('status').innerText = 'No current left stamp to delete';
        return;
    }}

    const warning = currentLeft.colnect_id
        ? `Deleting this row will also return linked Colnect candidate ${{currentLeft.colnect_id}} to the right side.\n\nContinue?`
        : 'Delete this left-side base/reference row?';
    if (!confirm(warning)) return;

    const response = await fetch('/delete-left-stamp', {{
        method: 'POST',
        headers: {{ 'Content-Type': 'application/json' }},
        body: JSON.stringify({{ rowid: Number(currentLeft.rowid) }})
    }});
    const payload = await response.json();
    if (!payload.ok) {{
        alert(payload.message || 'Delete failed');
        return;
    }}

    if (payload.restored_colnect_id) {{
        location.reload();
        return;
    }}

    if (payload.deleted_rowid != null) removeLeftRow(payload.deleted_rowid);
    document.getElementById('status').innerText = payload.message || 'Left-side row deleted';
    render();
}}

async function clearCurrentLeftColnectLink() {{
    const currentLeft = left[leftIdx];
    if (!currentLeft) {{
        document.getElementById('status').innerText = 'No current left stamp selected';
        return;
    }}
    if (!currentLeft.colnect_id) {{
        document.getElementById('status').innerText = 'Current left stamp has no linked Colnect data';
        return;
    }}
    if (!confirm(`Clear linked Colnect data from rowid=${{currentLeft.rowid}} and return ${{currentLeft.colnect_id}} to the candidate list?`)) return;

    const response = await fetch('/clear-left-colnect-link', {{
        method: 'POST',
        headers: {{ 'Content-Type': 'application/json' }},
        body: JSON.stringify({{ rowid: Number(currentLeft.rowid) }})
    }});
    const payload = await response.json();
    if (!payload.ok) {{
        alert(payload.message || 'Clear link failed');
        return;
    }}

    if (payload.updated_left) upsertLeftRow(payload.updated_left);
    document.getElementById('status').innerText = payload.message || 'Linked Colnect data cleared';
    location.reload();
}}

async function addCurrentColnectAsStamp() {{
    const r = currentRightCandidate();
    if (!r) {{
        document.getElementById('status').innerText = 'No current candidate to add';
        return;
    }}

    const assoc = !!document.getElementById('addAssociateImage')?.checked;
    const mode = getSelectedAddImageMode();
    const l = left[leftIdx];

    let uploadDataUrl = null;
    let uploadFilename = null;
    if (assoc && mode === 'upload') {{
        const input = document.getElementById('addUploadFile');
        const file = input?.files?.[0];
        if (!file) {{
            alert('Choose an image file to upload, or switch image mode.');
            return;
        }}
        try {{
            uploadDataUrl = await fileToDataUrl(file);
            uploadFilename = file.name || null;
        }} catch (e) {{
            alert(String(e));
            return;
        }}
    }}

    const body = {{
        colnect_id: r.colnect_id,
        source_left_rowid: l ? Number(l.rowid) : null,
        target_sg_number: document.getElementById('addSg')?.value || r.sg_number || '',
        target_year: Number(document.getElementById('addYear')?.value || r.year || 0),
        target_series: document.getElementById('addSeries')?.value || r.series || '',
        associate_image: assoc,
        image_mode: assoc ? mode : 'none',
        upload_data_url: uploadDataUrl,
        upload_filename: uploadFilename
    }};

    const res = await fetch('/add-colnect-stamp', {{
        method: 'POST',
        headers: {{ 'Content-Type': 'application/json' }},
        body: JSON.stringify(body)
    }});
    const j = await res.json();
    if (!j.ok) {{
        alert(j.message || 'Add failed');
        return;
    }}

    if (j.added_left) {{
        left.push(j.added_left);
    }}
    if (j.processed_colnect_id) {{
        removeCandidateFromUi(j.processed_colnect_id);
    }}
    closeAddDialog();
    document.getElementById('status').innerText = j.message || 'Added to stamps';
    render();
}}

async function deleteCurrentCandidate() {{
    const r = currentRightCandidate();
    if (!r) {{
        document.getElementById('status').innerText = 'No current candidate to delete';
        return;
    }}

    if (!confirm(`Soft delete colnect candidate ${{r.colnect_id}} from colnect_stamps?`)) {{
        return;
    }}

    const res = await fetch('/delete-colnect-candidate', {{
        method: 'POST',
        headers: {{ 'Content-Type': 'application/json' }},
        body: JSON.stringify({{ colnect_id: r.colnect_id }})
    }});
    const j = await res.json();
    if (!j.ok) {{
        alert(j.message || 'Delete failed');
        return;
    }}

    const removedAssignments = [];
    Object.keys(assignedByLeft).forEach(k => {{
        if (assignedByLeft[k] === r.colnect_id) {{
            removedAssignments.push([k, assignedByLeft[k], selectedCatalogCodeByLeft[k] || '', selectedCatalogTypeByLeft[k] || '']);
        }}
    }});
    deletedCandidatesStack.push({{ candidate: r, removedAssignments }});

    removeCandidateFromUi(r.colnect_id);
    document.getElementById('status').innerText = j.message || 'Soft-deleted candidate';
    render();
}}

async function undoLastDelete() {{
    const last = deletedCandidatesStack.pop();
    if (!last) {{
        document.getElementById('status').innerText = 'Nothing to undo';
        return;
    }}

    const res = await fetch('/undo-colnect-delete', {{
        method: 'POST',
        headers: {{ 'Content-Type': 'application/json' }},
        body: JSON.stringify({{ colnect_id: last.candidate.colnect_id }})
    }});
    const j = await res.json();
    if (!j.ok) {{
        deletedCandidatesStack.push(last);
        alert(j.message || 'Undo failed');
        return;
    }}

    if (!rightAll.some(c => c.colnect_id === last.candidate.colnect_id)) {{
        rightAll.push(last.candidate);
    }}
    for (const [k, cid, code, catalogType] of last.removedAssignments || []) {{
        assignedByLeft[k] = cid;
        if (code) selectedCatalogCodeByLeft[k] = code;
        if (catalogType) selectedCatalogTypeByLeft[k] = catalogType;
    }}
    document.getElementById('status').innerText = j.message || 'Undo complete';
    render();
}}

async function closeReviewUi() {{
    // Show a 3-button modal: Generate & Close | Close Only | Cancel
    const choice = await new Promise(resolve => {{
        const defaultPdfChecked = pdfGenerationDefaults.enabledByDefault && pdfGenerationDefaults.cliConfigured;
        const pdfOptionHtml =
            '<label style="display:flex;align-items:flex-start;gap:0.65em;margin:0 0 1.1em;text-align:left;font-size:0.95em;line-height:1.4">' +
            `<input id="_dlg_pdf" type="checkbox" ${{defaultPdfChecked ? 'checked' : ''}} ${{pdfGenerationDefaults.cliConfigured ? '' : 'disabled'}} style="margin-top:0.2em" />` +
            '<span>Generate PDF with AlbumEasy CLI <code>-c</code> and open it after generation</span>' +
            '</label>' +
            (pdfGenerationDefaults.cliConfigured
                ? ''
                : '<div style="margin:-0.45em 0 1.1em;text-align:left;color:#8a3b12;font-size:0.9em">AlbumEasy CLI is not configured. Set runtime.albumeasy_cli_path in config.toml to enable this.</div>');
        const dlg = document.createElement('dialog');
        dlg.style.cssText = [
            'padding:1.8em 2.2em',
            'border-radius:10px',
            'font-family:sans-serif',
            'border:1px solid #bbb',
            'min-width:320px',
            'text-align:center',
            'box-shadow:0 4px 24px rgba(0,0,0,.25)',
        ].join(';');
        dlg.innerHTML =
            '<p style="margin:0 0 1.2em;font-size:1.05em;font-weight:500">Close Review UI?</p>' +
            pdfOptionHtml +
            '<div style="display:flex;gap:0.6em;justify-content:center;flex-wrap:wrap">' +
            '<button id="_dlg_gen"  style="padding:0.5em 1.1em;background:#1a73e8;color:#fff;border:none;border-radius:5px;cursor:pointer;font-size:.95em">Generate &amp; Close</button>' +
            '<button id="_dlg_cls"  style="padding:0.5em 1.1em;background:#555;color:#fff;border:none;border-radius:5px;cursor:pointer;font-size:.95em">Close / End</button>' +
            '<button id="_dlg_cxl" style="padding:0.5em 1.1em;background:#e9e9e9;color:#333;border:none;border-radius:5px;cursor:pointer;font-size:.95em">Cancel</button>' +
            '</div>';
        document.body.appendChild(dlg);
        dlg.showModal();
        const done = v => {{
            const generatePdf = v === 'generate' && !!dlg.querySelector('#_dlg_pdf')?.checked;
            dlg.close();
            dlg.remove();
            resolve({{ choice: v, generatePdf }});
        }};
        dlg.querySelector('#_dlg_gen').onclick  = () => done('generate');
        dlg.querySelector('#_dlg_cls').onclick  = () => done('close');
        dlg.querySelector('#_dlg_cxl').onclick  = () => done('cancel');
        dlg.addEventListener('cancel', () => done('cancel')); // Esc key
    }});

    if (choice.choice === 'cancel') return;

    const generateAlbum = choice.choice === 'generate';
    const res = await fetch('/shutdown', {{
        method: 'POST',
        headers: {{ 'Content-Type': 'application/json' }},
        body: JSON.stringify({{
            generate_album: generateAlbum,
            generate_pdf: choice.generatePdf,
            start_year: (() => {{ const v = document.getElementById('reviewPdfStartYear')?.value.trim(); return v ? parseInt(v, 10) : null; }})(),
            end_year:   (() => {{ const v = document.getElementById('reviewPdfEndYear')?.value.trim();   return v ? parseInt(v, 10) : null; }})(),
        }})
    }});

    const j = await res.json();
    const details = [
        j.generated_output_path ? `\nOutput: ${{j.generated_output_path}}` : '',
        j.generated_pdf_path ? `\nPDF: ${{j.generated_pdf_path}}` : ''
    ].join('');
    alert((j.message || 'Review UI shutdown request sent.') + details);

    // window.close may be blocked for tabs not opened by script; provide fallback.
    window.close();
    setTimeout(() => {{ window.location.href = 'about:blank'; }}, 150);
}}

async function returnToStartReviewPage() {{
    const statusEl = document.getElementById('status');
    const originalText = statusEl?.innerText || 'Ready';
    if (window.__returnToStartInFlight) {{
        if (statusEl) statusEl.innerText = 'Return to Start is already in progress...';
        return;
    }}
    window.__returnToStartInFlight = true;

    if (statusEl) statusEl.innerText = 'Step 1/2: Switching server mode...';
    setOperationBusy(true, 'Step 1/2: Switching server mode...');

    try {{
        const res = await fetch('/return-to-start', {{
            method: 'POST',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{}})
        }});
        const j = await res.json();
        if (!j.ok) {{
            if (statusEl) statusEl.innerText = 'Return to Start failed';
            setOperationBusy(false, '');
            alert(j.message || 'Failed to return to Start Review page.');
            return;
        }}

        if (statusEl) statusEl.innerText = 'Step 2/2: Loading country selector...';
        setOperationBusy(true, 'Step 2/2: Loading country selector...');

        const ready = await waitForBootstrapReady(20, 250);
        if (ready) {{
            setOperationBusy(true, 'Step 2/2: Loading country selector... Redirecting...');
            await navigateWithFade('/');
            return;
        }}

        setOperationBusy(true, 'Step 2/2: Loading country selector... Redirecting with fallback...');
        await navigateWithFade('/');
    }} catch (e) {{
        if (statusEl) statusEl.innerText = 'Return to Start request failed';
        setOperationBusy(false, '');
        alert('Return to Start request failed: ' + e);
    }} finally {{
        window.__returnToStartInFlight = false;
        setTimeout(() => {{
            if (statusEl) statusEl.innerText = originalText;
        }}, 2200);
    }}
}}

if (document.body) {{
    document.body.classList.add('pageEnter');
    setTimeout(() => document.body.classList.remove('pageEnter'), 260);
}}

async function generatePdfNowReview() {{
    const statusEl = document.getElementById('status');
    const originalText = statusEl?.innerText || 'Ready';
    if (window.__reviewPdfRequestInFlight === true) {{
        if (statusEl) statusEl.innerText = 'PDF generation already in progress. Please wait.';
        return;
    }}

    const startYearVal = document.getElementById('reviewPdfStartYear')?.value.trim();
    const endYearVal   = document.getElementById('reviewPdfEndYear')?.value.trim();
    const startYear    = startYearVal ? parseInt(startYearVal, 10) : null;
    const endYear      = endYearVal   ? parseInt(endYearVal, 10)   : null;
    const yearDesc     = (startYear || endYear) ? ` (years ${{startYear || 'start'}}–${{endYear || 'end'}})` : '';
    if (statusEl) statusEl.innerText = 'Generating album text and PDF' + yearDesc + '...';
    window.__reviewPdfRequestInFlight = true;

    try {{
        const res = await fetch('/generate-pdf-now', {{
            method: 'POST',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{ start_year: startYear, end_year: endYear }})
        }});
        const j = await res.json();
        if (!j.ok) {{
            if (statusEl) statusEl.innerText = 'PDF generation failed';
            alert(j.message || 'PDF generation failed.');
            return;
        }}

        const details = [
            j.generated_output_path ? `\nOutput: ${{j.generated_output_path}}` : '',
            j.generated_pdf_path ? `\nPDF: ${{j.generated_pdf_path}}` : ''
        ].join('');
        if (statusEl) statusEl.innerText = j.message || 'PDF generation completed';
        alert((j.message || 'PDF generation completed.') + details);
    }} catch (e) {{
        if (statusEl) statusEl.innerText = 'PDF generation request failed';
        alert('PDF generation request failed: ' + e);
    }} finally {{
        window.__reviewPdfRequestInFlight = false;
        setTimeout(() => {{
            if (statusEl) statusEl.innerText = originalText;
        }}, 1600);
    }}
}}

async function importColnectFilesReview() {{
    const statusEl = document.getElementById('status');
    const originalText = statusEl?.innerText || 'Ready';
    
    if (statusEl) statusEl.innerText = 'Importing Colnect files...';
    
    try {{
        const r = await fetch('/api/colnect-import', {{
            method: 'POST',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{}})
        }});
        const j = await r.json();
            if (j.ok && j.stamps_imported > 0) {{
                if (statusEl) statusEl.innerText = 'Imported ' + j.stamps_imported + ' stamps';
                alert('Successfully imported ' + j.stamps_imported + ' stamps from ' + j.files_processed + ' file(s)!\n\nRefreshing...');
                setTimeout(() => {{
                    location.reload();
                }}, 500);
            }} else if (j.ok && j.stamps_imported === 0) {{
                if (statusEl) statusEl.innerText = 'No stamps imported';
                const msg = j.message || 'No stamps were imported.';
                const details = (j.errors && j.errors.length > 0)
                    ? '\n\nCheck inbox path and CSV files:\n' + j.errors.join('\n')
                    : '\n\nCheck that CSV files exist in the configured colnect_inbox_path in config.toml.';
                alert(msg + details);
            }} else {{
                if (statusEl) statusEl.innerText = 'Import failed';
                const msg = j.message || 'Import failed';
                const details = (j.errors && j.errors.length > 0)
                    ? '\n\nErrors:\n' + j.errors.join('\n')
                    : '';
                alert(msg + details);
            }}
    }} catch (e) {{
        if (statusEl) statusEl.innerText = 'Import error';
        alert('Import error: ' + e);
    }}
}}

const seriesToggle = document.getElementById('f_series');
if (seriesToggle) {{
  seriesToggle.checked = checkboxDefaults.f_series;
  seriesToggle.addEventListener('change', () => {{
    seriesToggle.dataset.userTouched = '1';
  }});
}}

// Apply all checkbox defaults from config
for (const [checkboxId, checked] of Object.entries(checkboxDefaults)) {{
  const el = document.getElementById(checkboxId);
  if (el) {{
    el.checked = checked;
    el.addEventListener('change', () => {{
      el.dataset.userTouched = '1';
    }});
  }}
}}

try {{
    const storedCatalog = String(localStorage.getItem(countryCatalogStorageKey) || '').toLowerCase();
    countryDefaultCatalogCode = storedCatalog || configDefaultCatalogCode;
}} catch (_) {{
    countryDefaultCatalogCode = configDefaultCatalogCode;
}}

const defaultCatalogSelect = document.getElementById('defaultCatalogSelect');
if (defaultCatalogSelect) {{
    defaultCatalogSelect.value = countryDefaultCatalogCode || 'sg';
}}

const sortModeSelect = document.getElementById('sortMode');
if (sortModeSelect) sortModeSelect.value = sortMode;
const showImagelessLeftToggle = document.getElementById('f_show_imageless_left');
if (showImagelessLeftToggle) showImagelessLeftToggle.checked = showImagelessLeftRowsDefault;
sortLeftRows(left[leftIdx]?.rowid);

render();
</script>
{config_editor_assets}
</body>
</html>"#,
        json = json,
        config_editor_assets = render_config_editor_assets()
    )
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
fn update_default_catalog_code_for_album(
    album_dir: &std::path::Path,
    album_name: &str,
    global_config_path: &str,
    template_path: &str,
    requested_code: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let normalized = crate::catalog::normalize_catalog_type(requested_code);
    if normalized.is_empty() {
        return Err("default catalog code cannot be empty".into());
    }
    let allowed = ["sg", "sn", "mi", "yt", "afa", "nvp", "un", "col"];
    if !allowed.contains(&normalized.as_str()) {
        return Err(format!("unsupported catalog code: {}", normalized).into());
    }

    // Load any existing per-album overrides so they are preserved.
    let album_override_path = album_dir.join("config.toml");
    let mut merged: HashMap<String, String> = if album_override_path.exists() {
        load_current_config_values(album_override_path.to_str().unwrap_or("")).unwrap_or_default()
    } else {
        HashMap::new()
    };

    // Insert the new code (unquoted – save_album_config_diff normalises it).
    merged.insert(
        "stamps.default_catalog_code".to_string(),
        normalized.clone(),
    );

    // Re-write the album override, keeping only fields that differ from global.
    save_album_config_diff(
        album_dir,
        album_name,
        global_config_path,
        template_path,
        &merged,
    )?;

    Ok(normalized)
}

fn config_field_id(section_id: &str, key: &str) -> String {
    if section_id.trim().is_empty() {
        key.trim().to_string()
    } else {
        format!("{}.{}", section_id.trim(), key.trim())
    }
}

fn friendly_config_section_title(section_id: &str) -> String {
    if section_id.trim().is_empty() {
        return "General".to_string();
    }

    section_id
        .split('.')
        .map(|part| {
            part.split('_')
                .filter(|segment| !segment.is_empty())
                .map(|segment| {
                    let mut chars = segment.chars();
                    match chars.next() {
                        Some(first) => {
                            format!("{}{}", first.to_ascii_uppercase(), chars.as_str())
                        }
                        None => String::new(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join(" / ")
}

fn parse_config_key_value_line(line: &str) -> Option<(String, String)> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }
    let (key, value) = trimmed.split_once('=')?;
    let key = key.trim();
    if key.is_empty() || key.starts_with('[') {
        return None;
    }
    Some((key.to_string(), value.trim().to_string()))
}

fn load_example_config_help_map(path: &str) -> HashMap<String, Vec<String>> {
    let Ok(content) = fs::read_to_string(path) else {
        return HashMap::new();
    };

    let mut out: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_section_id = String::new();
    let mut pending_comments: Vec<String> = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            let comment = trimmed.trim_start_matches('#').trim();
            if !comment.is_empty() {
                pending_comments.push(comment.to_string());
            }
            continue;
        }

        if trimmed.is_empty() {
            continue;
        }

        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            current_section_id = trimmed
                .trim_start_matches('[')
                .trim_end_matches(']')
                .trim()
                .to_string();
            pending_comments.clear();
            continue;
        }

        if let Some((key, _)) = parse_config_key_value_line(trimmed) {
            if !pending_comments.is_empty() {
                let full_id = config_field_id(&current_section_id, &key).to_ascii_lowercase();
                let key_only = key.to_ascii_lowercase();
                out.entry(full_id)
                    .or_default()
                    .extend(pending_comments.clone());
                out.entry(key_only)
                    .or_default()
                    .extend(pending_comments.clone());
            }
            pending_comments.clear();
            continue;
        }

        pending_comments.clear();
    }

    out
}

fn enrich_field_help(field: &mut ConfigEditorField, help_map: &HashMap<String, Vec<String>>) {
    let full_id_key = field.id.to_ascii_lowercase();
    let key_only = field.key.to_ascii_lowercase();

    if let Some(extra_help) = help_map
        .get(&full_id_key)
        .or_else(|| help_map.get(&key_only))
    {
        for line in extra_help {
            if !field.comments.iter().any(|existing| existing == line) {
                field.comments.push(line.clone());
            }
        }
    }

    let mut add_unique = |line: &str| {
        if !field.comments.iter().any(|existing| existing == line) {
            field.comments.push(line.to_string());
        }
    };

    if key_only.ends_with("_font") {
        add_unique("AlbumEasy font code. Common values: HN=Helvetica Normal, HB=Helvetica Bold, TN=Times Normal, TB=Times Bold, TR=Times Roman, TI=Times Italic, BI=Bold Italic, CN=Courier Normal, CO=Courier Oblique.");
        add_unique("Tip: keep heading/body font families consistent across related fields to avoid visual mismatch.");
    }

    if key_only.ends_with("_size") {
        add_unique("Font size used by AlbumEasy text rendering. Decimal values are allowed (for example 8.0, 10.5). Larger sizes increase layout pressure and can cause wrapping.");
    }

    if key_only.ends_with("_vspace") || key_only.contains("padding") || key_only.contains("margin")
    {
        add_unique("Spacing value in millimeters (mm). Positive values increase vertical/horizontal whitespace; overly large values may reduce stamps-per-page.");
    }

    if key_only == "page_text_format"
        || key_only == "page_text_centre_format"
        || key_only == "page_text_right_format"
    {
        add_unique(
            "Template must include exactly one '{}' placeholder where the text value is injected.",
        );
        add_unique("Expected command shapes: PAGE_TEXT (<FONT> <SIZE> \"{}\" <VSPACE>), PAGE_TEXT_CENTRE (...), PAGE_TEXT_RIGHT (...).");
        add_unique("Example: PAGE_TEXT (HN 10 \"{}\" 1.0)");
    }

    if key_only == "stamp_heading_font"
        || key_only == "stamp_heading_size"
        || key_only == "stamp_heading_label"
    {
        add_unique(
            "Emitted heading uses STAMP_HEADING(<FONT> <SIZE> \"<LABEL> : <CATALOG_NUMBER>\").",
        );
        add_unique("stamp_heading_label='auto' uses catalog display name from default_catalog_code; empty string hides the label.");
    }

    if key_only == "default_catalog_code" {
        add_unique("Catalog family used for stamp heading and matching preference.");
        add_unique(
            "Common values: sn=Scott, sg=Stanley Gibbons, mi=Michel, yt=Yvert, col=Colnect.",
        );
    }

    if key_only == "stamp_detail_layout" {
        add_unique("inline: details inside stamp text blocks; separate_rows: details in dedicated rows; minimal: reduced metadata output.");
    }

    if key_only == "row_start_es_enabled"
        || key_only == "row_start_es_font"
        || key_only == "row_start_es_size"
        || key_only == "row_start_es_spacing"
        || key_only == "row_start_es_vspace"
        || key_only == "row_start_es_marker"
    {
        add_unique("Controls ROW_START_ES marker output used to anchor row layout in generated AlbumEasy text.");
        add_unique("Disable row_start_es_enabled only if your output pipeline does not rely on row-start markers.");
    }

    if key_only == "albumeasy_cli_path" {
        add_unique("Absolute path to AlbumEasy CLI script/binary used for PDF generation from generated TXT.");
        add_unique("Linux example: /home/<user>/work/AlbumEasy/AlbumEasy.sh");
    }

    if key_only == "albumeasy_auto_generate_pdf" {
        add_unique(
            "When true, tool runs '<albumeasy_cli_path> -c <generated_txt>' after generation.",
        );
        add_unique("Requires albumeasy_cli_path to be configured.");
    }

    if key_only == "expand_grouped_ranges" {
        add_unique("placeholders: split grouped catalog ranges into placeholder rows quickly; detail_fetch: attempt richer detail fetch per item (slower)." );
    }

    if key_only.starts_with("flag_cover_") || full_id_key.starts_with("flag_cover.") {
        add_unique("Flag cover values control cover-page image/text layout and are interpreted in millimeters for size/spacing fields.");
    }
}

fn template_literal_kind(template_value: &str) -> &'static str {
    let t = template_value.trim();
    if t.eq_ignore_ascii_case("true") || t.eq_ignore_ascii_case("false") {
        return "bool";
    }
    if t.parse::<f64>().is_ok() {
        return "number";
    }
    if (t.starts_with('"') && t.ends_with('"')) || (t.starts_with('\'') && t.ends_with('\'')) {
        return "string";
    }
    if t.starts_with('[') && t.ends_with(']') {
        return "array";
    }
    if t.starts_with('{') && t.ends_with('}') {
        return "inline_table";
    }
    "unknown"
}

fn quote_toml_basic_string(input: &str) -> String {
    let escaped = input.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{}\"", escaped)
}

fn normalize_value_for_template(template_value: Option<&String>, raw_value: &str) -> String {
    let trimmed = raw_value.trim();

    if trimmed.is_empty() {
        if let Some(template) = template_value {
            if template_literal_kind(template) == "string" {
                return "\"\"".to_string();
            }
        }
        return String::new();
    }

    if (trimmed.starts_with('"') && trimmed.ends_with('"'))
        || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
    {
        return trimmed.to_string();
    }

    if let Some(template) = template_value {
        match template_literal_kind(template) {
            "bool" => {
                if trimmed.eq_ignore_ascii_case("true") {
                    return "true".to_string();
                }
                if trimmed.eq_ignore_ascii_case("false") {
                    return "false".to_string();
                }
                return trimmed.to_string();
            }
            "number" => {
                return trimmed.to_string();
            }
            "string" => {
                return quote_toml_basic_string(trimmed);
            }
            _ => {}
        }
    }

    if trimmed.eq_ignore_ascii_case("true") || trimmed.eq_ignore_ascii_case("false") {
        return trimmed.to_ascii_lowercase();
    }
    if trimmed.parse::<f64>().is_ok() {
        return trimmed.to_string();
    }
    if (trimmed.starts_with('[') && trimmed.ends_with(']'))
        || (trimmed.starts_with('{') && trimmed.ends_with('}'))
    {
        return trimmed.to_string();
    }

    quote_toml_basic_string(trimmed)
}

fn ensure_config_section<'a>(
    sections: &'a mut Vec<ConfigEditorSection>,
    section_id: &str,
    section_comments: Vec<String>,
) -> &'a mut ConfigEditorSection {
    if let Some(pos) = sections.iter().position(|section| section.id == section_id) {
        return &mut sections[pos];
    }

    sections.push(ConfigEditorSection {
        id: section_id.to_string(),
        title: friendly_config_section_title(section_id),
        comments: section_comments,
        fields: Vec::new(),
    });
    sections.last_mut().expect("section just pushed")
}

fn parse_template_config_sections(template_content: &str) -> Vec<ConfigEditorSection> {
    let mut sections: Vec<ConfigEditorSection> = Vec::new();
    let mut current_section_id = String::new();
    let mut pending_comments: Vec<String> = Vec::new();

    for line in template_content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with('#') {
            let comment = trimmed.trim_start_matches('#').trim();
            if !comment.is_empty() {
                pending_comments.push(comment.to_string());
            }
            continue;
        }

        if trimmed.is_empty() {
            pending_comments.clear();
            continue;
        }

        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            current_section_id = trimmed
                .trim_start_matches('[')
                .trim_end_matches(']')
                .trim()
                .to_string();
            let section_comments = std::mem::take(&mut pending_comments);
            let _ = ensure_config_section(&mut sections, &current_section_id, section_comments);
            continue;
        }

        if let Some((key, value)) = parse_config_key_value_line(trimmed) {
            let field_comments = std::mem::take(&mut pending_comments);
            let section = ensure_config_section(&mut sections, &current_section_id, Vec::new());
            section.fields.push(ConfigEditorField {
                id: config_field_id(&current_section_id, &key),
                section_id: current_section_id.clone(),
                section_title: section.title.clone(),
                key,
                template_value: value.clone(),
                current_value: value.clone(),
                global_value: value,
                is_album_override: false,
                comments: field_comments,
                from_template: true,
            });
            continue;
        }

        pending_comments.clear();
    }

    sections.retain(|section| !section.fields.is_empty());
    sections
}

fn flatten_toml_value_into_map(
    prefix: &str,
    value: &toml::Value,
    out: &mut HashMap<String, String>,
) {
    match value {
        toml::Value::Table(table) => {
            for (key, entry) in table {
                let next_prefix = if prefix.is_empty() {
                    key.to_string()
                } else {
                    format!("{}.{}", prefix, key)
                };
                flatten_toml_value_into_map(&next_prefix, entry, out);
            }
        }
        _ => {
            out.insert(prefix.to_string(), value.to_string());
        }
    }
}

fn load_current_config_values(
    config_path: &str,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(config_path)
        .map_err(|e| format!("failed to read {}: {}", config_path, e))?;
    let parsed: toml::Value =
        toml::from_str(&content).map_err(|e| format!("failed to parse {}: {}", config_path, e))?;
    let mut values = HashMap::new();
    flatten_toml_value_into_map("", &parsed, &mut values);
    Ok(values)
}

fn build_config_editor_data(
    config_path: &str,
    template_path: &str,
) -> Result<Vec<ConfigEditorSection>, Box<dyn std::error::Error>> {
    crate::config::ensure_config_and_template_files(config_path, template_path)?;
    let template_content = crate::config::load_template_content_with_fallback(template_path)?;
    let mut template_sections = parse_template_config_sections(&template_content);
    let example_help_map = load_example_config_help_map("config_full_example.toml");
    let current_values = load_current_config_values(config_path)?;
    let mut used_field_ids = HashSet::new();

    for section in &mut template_sections {
        for field in &mut section.fields {
            if let Some(current_value) = current_values.get(&field.id) {
                field.current_value = current_value.clone();
            }
            enrich_field_help(field, &example_help_map);
            used_field_ids.insert(field.id.clone());
        }
    }

    let mut extra_sections: BTreeMap<String, Vec<ConfigEditorField>> = BTreeMap::new();
    for (field_id, current_value) in current_values {
        if used_field_ids.contains(&field_id) {
            continue;
        }

        let mut parts = field_id.rsplitn(2, '.');
        let key = parts.next().unwrap_or_default().to_string();
        let section_id = parts.next().unwrap_or_default().to_string();
        extra_sections
            .entry(section_id.clone())
            .or_default()
            .push(ConfigEditorField {
                id: field_id.clone(),
                section_id: section_id.clone(),
                section_title: friendly_config_section_title(&section_id),
                key,
                template_value: current_value.clone(),
                current_value: current_value.clone(),
                global_value: current_value,
                is_album_override: false,
                comments: vec!["Preserved from the current config file. No template comment is available for this key.".to_string()],
                from_template: false,
            });
    }

    for (section_id, mut fields) in extra_sections {
        fields.sort_by(|left, right| left.key.cmp(&right.key));
        if let Some(existing_idx) = template_sections
            .iter()
            .position(|section| section.id == section_id)
        {
            for field in &mut fields {
                enrich_field_help(field, &example_help_map);
            }
            template_sections[existing_idx].fields.extend(fields);
            if !template_sections[existing_idx]
                .comments
                .iter()
                .any(|c| c.contains("not documented in config_template.toml"))
            {
                template_sections[existing_idx].comments.push(
                    "Some settings in this section exist in config.toml but are not documented in config_template.toml. They will be preserved on save.".to_string(),
                );
            }
        } else {
            template_sections.push(ConfigEditorSection {
                id: section_id.clone(),
                title: if section_id.is_empty() {
                    "Additional".to_string()
                } else {
                    format!("{} (Preserved)", friendly_config_section_title(&section_id))
                },
                comments: vec!["These settings exist in the active config.toml but are not documented in config_template.toml. They will be preserved on save.".to_string()],
                fields,
            });
        }
    }

    Ok(template_sections)
}

fn render_config_from_template_values(
    template_content: &str,
    values: &HashMap<String, String>,
) -> String {
    let mut rendered_lines: Vec<String> = Vec::new();
    let mut current_section_id = String::new();
    let mut template_field_ids = HashSet::new();
    let mut template_section_ids = HashSet::new();

    for line in template_content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            current_section_id = trimmed
                .trim_start_matches('[')
                .trim_end_matches(']')
                .trim()
                .to_string();
            template_section_ids.insert(current_section_id.clone());
            rendered_lines.push(line.to_string());
            continue;
        }

        if let Some((key, original_value)) = parse_config_key_value_line(line) {
            let field_id = config_field_id(&current_section_id, &key);
            template_field_ids.insert(field_id.clone());
            let value = values
                .get(&field_id)
                .cloned()
                .unwrap_or(original_value)
                .trim()
                .to_string();
            rendered_lines.push(format!("{} = {}", key, value));
            continue;
        }

        rendered_lines.push(line.to_string());
    }

    let mut extra_sections_new_tables: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
    let mut extra_dotted_keys: Vec<(String, String)> = Vec::new();
    for (field_id, value) in values {
        if template_field_ids.contains(field_id) {
            continue;
        }

        let mut parts = field_id.rsplitn(2, '.');
        let key = parts.next().unwrap_or_default().to_string();
        let section_id = parts.next().unwrap_or_default().to_string();
        let trimmed_value = value.trim().to_string();
        if section_id.is_empty() {
            extra_dotted_keys.push((key, trimmed_value));
        } else if template_section_ids.contains(&section_id) {
            extra_dotted_keys.push((format!("{}.{}", section_id, key), trimmed_value));
        } else {
            extra_sections_new_tables
                .entry(section_id)
                .or_default()
                .push((key, trimmed_value));
        }
    }

    if !extra_dotted_keys.is_empty() || !extra_sections_new_tables.is_empty() {
        if rendered_lines
            .last()
            .map(|line| !line.is_empty())
            .unwrap_or(false)
        {
            rendered_lines.push(String::new());
        }
        rendered_lines.push("###################################".to_string());
        rendered_lines.push("# Preserved Extra Keys".to_string());
        rendered_lines.push("###################################".to_string());

        extra_dotted_keys.sort_by(|left, right| left.0.cmp(&right.0));
        for (full_key, value) in extra_dotted_keys {
            rendered_lines.push(format!("{} = {}", full_key, value));
        }

        for (section_id, mut fields) in extra_sections_new_tables {
            fields.sort_by(|left, right| left.0.cmp(&right.0));
            rendered_lines.push(String::new());
            if !section_id.trim().is_empty() {
                rendered_lines.push(format!("[{}]", section_id));
            }
            for (key, value) in fields {
                rendered_lines.push(format!("{} = {}", key, value));
            }
        }
    }

    let mut output = rendered_lines.join("\n");
    output.push('\n');
    output
}

fn save_full_config_values(
    config_path: &str,
    template_path: &str,
    values: &HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    if values.is_empty() {
        return Err("no config values were provided".into());
    }

    crate::config::ensure_config_and_template_files(config_path, template_path)?;
    let template_content = crate::config::load_template_content_with_fallback(template_path)?;
    let template_sections = parse_template_config_sections(&template_content);
    let mut template_defaults_by_id: HashMap<String, String> = HashMap::new();
    for section in &template_sections {
        for field in &section.fields {
            template_defaults_by_id.insert(field.id.clone(), field.template_value.clone());
        }
    }

    let mut normalized_values: HashMap<String, String> = HashMap::new();
    for (field_id, value) in values {
        let normalized = normalize_value_for_template(template_defaults_by_id.get(field_id), value);
        normalized_values.insert(field_id.clone(), normalized);
    }

    let rendered = render_config_from_template_values(&template_content, &normalized_values);

    toml::from_str::<toml::Value>(&rendered)
        .map_err(|e| format!("generated config is not valid TOML: {}", e))?;

    fs::write(config_path, rendered)
        .map_err(|e| format!("failed to write {}: {}", config_path, e))?;
    Ok(())
}

/// Build config editor data for a country review UI session.
/// Returns `(sections, has_album_override)` where sections have `current_value`
/// set to the merged (effective) value and `global_value` / `is_album_override`
/// populated correctly.
fn build_config_editor_data_for_album(
    global_config_path: &str,
    template_path: &str,
    album_dir: &std::path::Path,
) -> Result<(Vec<ConfigEditorSection>, bool), Box<dyn std::error::Error>> {
    // Base: load sections with global config values.
    let mut sections = build_config_editor_data(global_config_path, template_path)?;

    // Set global_value = current_value (baseline before album overlay).
    for section in &mut sections {
        for field in &mut section.fields {
            field.global_value = field.current_value.clone();
        }
    }

    // Check whether an album override file exists.
    let override_path = album_dir.join("config.toml");
    if !override_path.exists() {
        return Ok((sections, false));
    }

    // Load album override raw key/value pairs.
    let override_values = match load_current_config_values(override_path.to_str().unwrap_or("")) {
        Ok(v) => v,
        Err(e) => {
            log::warn!(
                "Could not read album config override '{}': {}",
                override_path.display(),
                e
            );
            return Ok((sections, true));
        }
    };

    // Overlay: for each field whose id appears in the album override, update
    // current_value and mark as album_override.
    for section in &mut sections {
        for field in &mut section.fields {
            if let Some(album_val) = override_values.get(&field.id) {
                field.current_value = album_val.clone();
                field.is_album_override = true;
            }
        }
    }

    Ok((sections, true))
}

/// Write a minimal album-local config.toml that stores only the fields that
/// differ from the global config.  The file is written into `album_dir`.
/// Returns the number of fields written (0 = nothing changed vs global).
fn save_album_config_diff(
    album_dir: &std::path::Path,
    album_name: &str,
    global_config_path: &str,
    template_path: &str,
    all_ui_values: &HashMap<String, String>,
) -> Result<usize, Box<dyn std::error::Error>> {
    // Get template section data for normalization.
    let template_content = crate::config::load_template_content_with_fallback(template_path)?;
    let template_sections = parse_template_config_sections(&template_content);
    let mut template_defaults_by_id: HashMap<String, String> = HashMap::new();
    for section in &template_sections {
        for field in &section.fields {
            template_defaults_by_id.insert(field.id.clone(), field.template_value.clone());
        }
    }

    // Normalize all submitted values.
    let mut normalized: HashMap<String, String> = HashMap::new();
    for (field_id, raw_value) in all_ui_values {
        let n = normalize_value_for_template(template_defaults_by_id.get(field_id), raw_value);
        normalized.insert(field_id.clone(), n);
    }

    // Load the global config current values for comparison.
    let global_values: HashMap<String, String> =
        crate::config::ensure_config_and_template_files(global_config_path, template_path)
            .ok()
            .and_then(|_| load_current_config_values(global_config_path).ok())
            .unwrap_or_default();

    // Collect only the fields that differ from global.
    // Group by template section_id so we write proper section headers.
    let mut diff_by_section: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
    for (field_id, new_val) in &normalized {
        let global_val = global_values
            .get(field_id)
            .map(|s| s.trim().to_string())
            .unwrap_or_default();
        let new_trimmed = new_val.trim().to_string();
        if new_trimmed == global_val {
            continue; // unchanged vs global — skip
        }
        // field_id is like "stamps.default_catalog_code"
        let mut parts = field_id.rsplitn(2, '.');
        let key = parts.next().unwrap_or_default().to_string();
        let section_id = parts.next().unwrap_or_default().to_string();
        diff_by_section
            .entry(section_id)
            .or_default()
            .push((key, new_trimmed));
    }

    let total: usize = diff_by_section.values().map(|v| v.len()).sum();
    if total == 0 {
        // Delete the override file if it now matches global exactly.
        let override_path = album_dir.join("config.toml");
        if override_path.exists() {
            let _ = fs::remove_file(&override_path);
        }
        return Ok(0);
    }

    // Build TOML content with section headers matching the template.
    let mut lines = vec![
        format!("# StampAlbumGen2 album config override: {}", album_name),
        "# Only values that differ from the global config.toml are stored here.".to_string(),
        "# Settings not listed here fall back to the global config.toml.".to_string(),
        String::new(),
    ];

    for (section_id, mut fields) in diff_by_section {
        fields.sort_by(|a, b| a.0.cmp(&b.0));
        if !section_id.is_empty() {
            lines.push(format!("[{}]", section_id));
        }
        for (key, val) in fields {
            lines.push(format!("{} = {}", key, val));
        }
        lines.push(String::new());
    }

    let content = lines.join("\n");
    // Validate before writing.
    toml::from_str::<toml::Value>(&content)
        .map_err(|e| format!("generated album override is not valid TOML: {}", e))?;

    fs::create_dir_all(album_dir).map_err(|e| {
        format!(
            "failed to create album dir '{}': {}",
            album_dir.display(),
            e
        )
    })?;
    let override_path = album_dir.join("config.toml");
    fs::write(&override_path, content).map_err(|e| {
        format!(
            "failed to write album config override '{}': {}",
            override_path.display(),
            e
        )
    })?;

    Ok(total)
}

fn render_config_editor_assets() -> String {
    r#"
<style>
.cfgBackdrop { position: fixed; inset: 0; background: radial-gradient(circle at 20% 20%, rgba(12, 90, 123, 0.25) 0%, rgba(4, 20, 30, 0.7) 48%, rgba(4, 16, 25, 0.86) 100%); display: none; align-items: center; justify-content: center; z-index: 2000; padding: 16px; backdrop-filter: blur(2px); }
.cfgDialog { --cfgPrimary: #0b6a87; --cfgPrimaryHover: #09576f; --cfgTextStrong: #14384a; --cfgTextSoft: #3b5f70; --cfgPanel: #f4f9fc; --cfgPanelDeep: #e8f1f7; width: min(1280px, 96vw); max-height: 92vh; display: flex; flex-direction: column; background: linear-gradient(180deg, #ffffff 0%, #f7fbfe 100%); border: 1px solid #bfd0dc; border-radius: 16px; box-shadow: 0 26px 62px rgba(10, 28, 41, 0.45); overflow: hidden; animation: cfgDialogIn 180ms ease-out; }
.cfgHeader, .cfgFooter { display: flex; align-items: center; justify-content: space-between; gap: 10px; padding: 12px 16px; background: linear-gradient(180deg, #f4fafd 0%, #e9f3f9 100%); border-bottom: 1px solid #d3e0ea; }
.cfgFooter { border-top: 1px solid #e0e8ef; border-bottom: none; }
.cfgTitleBlock h3 { margin: 0; font-size: 20px; letter-spacing: 0.2px; color: var(--cfgTextStrong); }
.cfgTitleBlock p { margin: 4px 0 0 0; font-size: 13px; color: var(--cfgTextSoft); }
.cfgActionRow { display: flex; align-items: center; gap: 8px; flex-wrap: wrap; }
.cfgToolbar { display: flex; align-items: center; gap: 10px; flex-wrap: wrap; padding: 12px 16px; border-bottom: 1px solid #d8e4ec; background: linear-gradient(180deg, #ffffff 0%, #f3f8fb 100%); }
.cfgToolbar input { min-width: 260px; border: 1px solid #bfd0dd; border-radius: 999px; padding: 8px 12px; background: #ffffff; color: #1f4358; }
.cfgToolbar input:focus { outline: 2px solid rgba(11, 106, 135, 0.18); border-color: #0b6a87; }
.cfgBody { overflow: auto; padding: 14px 16px 18px; background: linear-gradient(180deg, #f7fbfe 0%, #eef5fa 100%); }
.cfgLoadState { padding: 24px 10px; color: #36546a; font-size: 14px; }
.cfgSection { border: 1px solid #ccdae4; border-radius: 12px; background: #ffffff; box-shadow: 0 8px 20px rgba(18, 46, 64, 0.09); margin-bottom: 14px; overflow: hidden; }
.cfgSectionHead { display: flex; align-items: center; justify-content: space-between; gap: 10px; padding: 12px 14px; background: linear-gradient(90deg, var(--cfgPanel) 0%, var(--cfgPanelDeep) 100%); border-bottom: 1px solid #d4e0e9; }
.cfgSectionHead h4 { margin: 0; font-size: 15px; color: #163a4d; }
.cfgSectionHead p { margin: 4px 0 0 0; font-size: 12px; color: #406072; }
.cfgFieldRow { display: grid; grid-template-columns: minmax(340px, 42%) minmax(320px, 1fr); gap: 14px; padding: 14px; border-top: 1px solid #edf2f6; }
.cfgFieldRow:first-child { border-top: none; }
.cfgStaticPane { display: flex; flex-direction: column; gap: 8px; }
.cfgEditPane { display: flex; flex-direction: column; gap: 6px; }
.cfgKeyRow { display: flex; align-items: center; gap: 8px; flex-wrap: wrap; }
.cfgKeyRow strong { font-size: 14px; color: #173b4e; }
.cfgTemplateBox { border: 1px solid #d2dee8; border-radius: 10px; background: #f6fbff; padding: 10px; font-size: 12px; color: #355267; }
.cfgTemplateBox code { display: block; white-space: pre-wrap; word-break: break-word; background: #ffffff; border: 1px solid #d7e2eb; border-radius: 8px; padding: 8px; margin-top: 6px; }
.cfgInfoDetail { border: 1px solid #d7e3ec; border-radius: 10px; background: #fbfdff; }
.cfgInfoDetail summary { cursor: pointer; padding: 8px 10px; font-size: 12px; font-weight: 600; color: #284b64; }
.cfgInfoBody { padding: 0 10px 10px; font-size: 12px; color: #46657a; white-space: pre-wrap; }
.cfgValueLabel { font-size: 12px; font-weight: 600; color: #284b64; }
.cfgValueInput { width: 100%; min-height: 40px; padding: 8px 10px; border: 1px solid #cfdbe6; border-radius: 10px; font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; font-size: 12px; line-height: 1.45; background: #ffffff; box-sizing: border-box; }
.cfgValueInput[type='number'] { font-family: ui-sans-serif, system-ui, sans-serif; }
.cfgValueInput:focus { outline: 2px solid rgba(11, 106, 135, 0.2); border-color: #0b6a87; }
.cfgHelpText { margin-top: 6px; font-size: 11px; color: #46657a; line-height: 1.35; }
.cfgTypeTag { background: #f0f7fb; color: #1f4d63; border: 1px solid #c8dce9; border-radius: 999px; font-size: 11px; padding: 2px 8px; }
.cfgDialog .badge { background: #e8f2f9; border: 1px solid #cbdae5; color: #1f4d63; }
.cfgAlbumBadge { background: #fff0dd !important; border-color: #e8c08b !important; color: #7a4a0d !important; }
.cfgAlbumBanner { display: none; border: 1px solid #d9c59e; background: linear-gradient(180deg, #fff8eb 0%, #fff2dd 100%); color: #6d470f; border-radius: 999px; padding: 6px 10px; font-size: 12px; }
.cfgSaveTarget { display: none; align-items: center; gap: 8px; font-size: 12px; color: #35556a; }
.cfgSaveTarget select { border: 1px solid #bfd0dd; border-radius: 8px; padding: 6px 8px; color: #1f4358; background: #ffffff; }
.cfgPrimaryBtn, .cfgSecondaryBtn { border: none; border-radius: 10px; padding: 8px 13px; cursor: pointer; font-weight: 600; font-size: 13px; transition: transform 120ms ease, filter 120ms ease, box-shadow 120ms ease; }
.cfgPrimaryBtn { background: linear-gradient(180deg, var(--cfgPrimary) 0%, #0a5f7a 100%); color: #ffffff; box-shadow: 0 4px 12px rgba(11, 106, 135, 0.25); }
.cfgPrimaryBtn:hover { filter: brightness(1.04); transform: translateY(-1px); }
.cfgSecondaryBtn { background: #e9f1f6; color: #214356; border: 1px solid #c7d7e2; }
.cfgSecondaryBtn:hover { background: #dbe8f1; }
.cfgFooterNote { font-size: 12px; color: #3f6071; max-width: 820px; }
.cfgConfirmDialog { padding: 18px 20px; border-radius: 14px; border: 1px solid #bfd1de; min-width: 340px; max-width: 530px; box-shadow: 0 18px 42px rgba(10, 28, 41, 0.35); background: linear-gradient(180deg, #ffffff 0%, #f4f9fc 100%); }
.cfgConfirmDialog::backdrop { background: rgba(4, 20, 30, 0.55); }
.cfgConfirmTitle { margin: 0 0 10px; font-size: 17px; color: #12364a; }
.cfgConfirmBody { margin: 0 0 14px; color: #406072; font-size: 13px; line-height: 1.45; }
.cfgConfirmActions { display: flex; gap: 8px; justify-content: flex-end; flex-wrap: wrap; }
.cfgDangerBtn { border: none; border-radius: 10px; padding: 8px 12px; cursor: pointer; font-weight: 600; color: #ffffff; background: linear-gradient(180deg, #b45342 0%, #943828 100%); }
.cfgHidden { display: none !important; }
@keyframes cfgDialogIn {
    from { opacity: 0; transform: translateY(7px) scale(0.985); }
    to { opacity: 1; transform: translateY(0) scale(1); }
}
@media (max-width: 960px) {
  .cfgFieldRow { grid-template-columns: 1fr; }
  .cfgDialog { width: min(96vw, 96vw); }
  .cfgToolbar input { min-width: 180px; width: 100%; }
    .cfgHeader, .cfgFooter { align-items: flex-start; }
}
</style>

<div id='configEditorBackdrop' class='cfgBackdrop'>
    <div class='cfgDialog'>
        <div class='cfgHeader'>
            <div class='cfgTitleBlock'>
                <h3 id='cfgEditorTitle'>Edit config.toml</h3>
                <p>Static template defaults and comments are on the left. Editable current values are on the right.</p>
            </div>
            <div class='cfgActionRow'>
                <button class='cfgPrimaryBtn' onclick='saveConfigEditorChanges()'>OK / Save</button>
                <button class='cfgSecondaryBtn' onclick='requestCloseConfigEditor()'>Cancel</button>
            </div>
        </div>
        <div class='cfgToolbar'>
            <span id='cfgEditorStatus' class='badge'>Ready</span>
            <span id='cfgEditorFieldCount' class='badge'>Fields: 0</span>
            <span id='cfgAlbumBanner' class='cfgAlbumBanner'></span>
            <button id='cfgDeleteAlbumBtn' class='cfgSecondaryBtn cfgHidden' onclick='deleteAlbumConfigOverride()'>Delete Album Override</button>
            <input id='cfgSearchInput' type='text' placeholder='Filter by section, key, or comment' oninput='filterConfigEditorFields()' />
        </div>
        <div id='cfgEditorBody' class='cfgBody'>
            <div id='cfgEditorLoadState' class='cfgLoadState'>Load config details to start editing.</div>
            <div id='cfgEditorSections'></div>
        </div>
        <div class='cfgFooter'>
            <div class='cfgFooterNote'>Saving validates the generated config and reloads the review UI so new values take effect.</div>
            <div class='cfgActionRow'>
                <div id='cfgSaveTargetBlock' class='cfgSaveTarget'>
                    <label for='cfgSaveTarget'>Save target:</label>
                    <select id='cfgSaveTarget' onchange='onConfigSaveTargetChanged()'>
                        <option value='album'>Album override</option>
                        <option value='global'>Global config.toml</option>
                    </select>
                </div>
                <button class='cfgPrimaryBtn' onclick='saveConfigEditorChanges()'>OK / Save</button>
                <button class='cfgSecondaryBtn' onclick='requestCloseConfigEditor()'>Cancel</button>
            </div>
        </div>
    </div>
</div>

<script>
const configEditorState = { open: false, loading: false, saving: false, sections: [], originalValues: {}, albumName: '', hasAlbumOverride: false, saveTarget: 'global' };
function cfgHtml(v) { return String(v).replaceAll('&','&amp;').replaceAll('<','&lt;').replaceAll('>','&gt;').replaceAll('"','&quot;'); }
function configEditorBackdrop() { return document.getElementById('configEditorBackdrop'); }
function configEditorSectionsHost() { return document.getElementById('cfgEditorSections'); }
function configEditorLoadState() { return document.getElementById('cfgEditorLoadState'); }
function configEditorStatusBadge() { return document.getElementById('cfgEditorStatus'); }
function configEditorFieldCountBadge() { return document.getElementById('cfgEditorFieldCount'); }
function configEditorSearchInput() { return document.getElementById('cfgSearchInput'); }
function configEditorTitle() { return document.getElementById('cfgEditorTitle'); }
function configEditorAlbumBanner() { return document.getElementById('cfgAlbumBanner'); }
function configEditorSaveTargetBlock() { return document.getElementById('cfgSaveTargetBlock'); }
function configEditorSaveTargetSelect() { return document.getElementById('cfgSaveTarget'); }
function configEditorDeleteAlbumBtn() { return document.getElementById('cfgDeleteAlbumBtn'); }
function setConfigEditorStatus(message) { const badge = configEditorStatusBadge(); if (badge) badge.innerText = message; const statusEl = document.getElementById('status'); if (statusEl) statusEl.innerText = message; }
function onConfigSaveTargetChanged() { const el = configEditorSaveTargetSelect(); if (!el) return; configEditorState.saveTarget = el.value === 'album' ? 'album' : 'global'; const targetName = configEditorState.saveTarget === 'album' ? `Album override (${configEditorState.albumName || 'current'})` : 'global config.toml'; setConfigEditorStatus(`Save target: ${targetName}`); }
function refreshConfigEditorModeUi() { const albumName = String(configEditorState.albumName || '').trim(); const inAlbumMode = albumName.length > 0; const titleEl = configEditorTitle(); if (titleEl) titleEl.innerText = inAlbumMode ? `Edit config for ${albumName}` : 'Edit config.toml'; const banner = configEditorAlbumBanner(); if (banner) { if (inAlbumMode) { banner.style.display = 'inline-flex'; banner.innerText = configEditorState.hasAlbumOverride ? `Album override active: ${albumName}` : `Using global config for ${albumName}`; } else { banner.style.display = 'none'; banner.innerText = ''; } } const saveTargetBlock = configEditorSaveTargetBlock(); if (saveTargetBlock) saveTargetBlock.style.display = inAlbumMode ? 'inline-flex' : 'none'; const saveTargetSelect = configEditorSaveTargetSelect(); if (saveTargetSelect) { if (!inAlbumMode) { configEditorState.saveTarget = 'global'; saveTargetSelect.value = 'global'; } else if (configEditorState.saveTarget !== 'global' && configEditorState.saveTarget !== 'album') { configEditorState.saveTarget = 'album'; saveTargetSelect.value = 'album'; } else { saveTargetSelect.value = configEditorState.saveTarget; } const albumOption = saveTargetSelect.querySelector("option[value='album']"); if (albumOption) albumOption.textContent = `Album override - ${albumName || 'current'}`; } const deleteBtn = configEditorDeleteAlbumBtn(); if (deleteBtn) deleteBtn.classList.toggle('cfgHidden', !(inAlbumMode && configEditorState.hasAlbumOverride)); }
function configFieldInputId(fieldId) { return 'cfg-input-' + String(fieldId || '').replaceAll('.', '__').replaceAll('/', '_'); }
function isBooleanValue(v) { const s = String(v ?? '').trim().toLowerCase(); return s === 'true' || s === 'false'; }
function isNumericValue(v) { return /^-?\d+(\.\d+)?$/.test(String(v ?? '').trim()); }
function isFontField(field) { const key = String(field?.key || '').toLowerCase(); const id = String(field?.id || '').toLowerCase(); return key.endsWith('_font') || id.endsWith('_font'); }
function configFieldKnownOptions(fieldId, key) {
    const id = String(fieldId || '').toLowerCase();
    const k = String(key || '').toLowerCase();
    if (id === 'stamps.stamp_detail_layout' || k === 'stamp_detail_layout') return ['inline', 'separate_rows', 'minimal'];
    if (id === 'header.album_pages_header_position' || id === 'footer.album_pages_footer_number_position' || k.endsWith('_position')) return ['L', 'R', 'C', 'LR', 'RL'];
    if (id === 'data_sources.expand_grouped_ranges' || k === 'expand_grouped_ranges') return ['placeholders', 'detail_fetch'];
    if (id === 'wikipedia.wikipedia_paragraph_alignment' || k.endsWith('_alignment')) return ['JUSTIFIED', 'LEFT', 'CENTER', 'RIGHT'];
    if (id === 'wikipedia.wikipedia_pagination_strategy' || k.endsWith('_pagination_strategy')) return ['balanced', 'conservative', 'strict'];
    return null;
}
function configFieldType(field) {
    const current = String(field?.current_value ?? '').trim();
    const template = String(field?.template_value ?? '').trim();
    if (isBooleanValue(current) || isBooleanValue(template)) return 'bool';
    if (isFontField(field)) return 'font';
    if (configFieldKnownOptions(field?.id, field?.key)) return 'enum';
    if (isNumericValue(current) || isNumericValue(template)) return 'number';
    return 'text';
}
function configFieldHelpHtml(field) {
    const comments = Array.isArray(field?.comments) ? field.comments : [];
    const notes = [];
    if (comments.length) notes.push(...comments);
    if (isFontField(field)) notes.push('AlbumEasy font codes often used: HN, HB, TN, TB, TR, TI, BI, CN, CO.');
    const type = configFieldType(field);
    if (type === 'bool') notes.push('Boolean field: choose true or false.');
    if (type === 'number') notes.push('Numeric field: enter a valid integer or decimal number.');
    if (!notes.length) return '';
    const text = notes.map(line => cfgHtml(line)).join('\n');
    return `<details class='cfgInfoDetail'><summary>Help</summary><div class='cfgInfoBody'>${text}</div></details>`;
}
function renderConfigValueInput(field) {
    const fieldId = String(field?.id || '');
    const inputId = configFieldInputId(fieldId);
    const current = String(field?.current_value ?? '');
    const type = configFieldType(field);
    const knownOptions = configFieldKnownOptions(field?.id, field?.key);
    const fontOptions = ['HN','HB','TN','TB','TR','TI','BI','CN','CO'];
    const normBool = String(current).trim().toLowerCase() === 'true' ? 'true' : 'false';

    if (type === 'bool') {
        return `<select id='${cfgHtml(inputId)}' class='cfgValueInput' data-field-id='${cfgHtml(fieldId)}' oninput='markConfigEditorDirty()'><option value='true' ${normBool === 'true' ? 'selected' : ''}>true</option><option value='false' ${normBool === 'false' ? 'selected' : ''}>false</option></select><div class='cfgHelpText'>Type: boolean</div>`;
    }

    if (type === 'font') {
        const options = fontOptions.map(opt => `<option value='${cfgHtml(opt)}' ${String(current).trim() === opt ? 'selected' : ''}>${cfgHtml(opt)}</option>`).join('');
        return `<select id='${cfgHtml(inputId)}' class='cfgValueInput' data-field-id='${cfgHtml(fieldId)}' oninput='markConfigEditorDirty()'>${options}</select><div class='cfgHelpText'>Type: font code</div>`;
    }

    if (type === 'enum' && Array.isArray(knownOptions) && knownOptions.length) {
        const fallbackOption = knownOptions.includes(current) ? '' : `<option value='${cfgHtml(current)}' selected>${cfgHtml(current || '(empty)')}</option>`;
        const options = knownOptions.map(opt => `<option value='${cfgHtml(opt)}' ${String(current).trim() === opt ? 'selected' : ''}>${cfgHtml(opt)}</option>`).join('');
        return `<select id='${cfgHtml(inputId)}' class='cfgValueInput' data-field-id='${cfgHtml(fieldId)}' oninput='markConfigEditorDirty()'>${fallbackOption}${options}</select><div class='cfgHelpText'>Type: option list</div>`;
    }

    if (type === 'number') {
        return `<input id='${cfgHtml(inputId)}' class='cfgValueInput' data-field-id='${cfgHtml(fieldId)}' type='number' step='any' value='${cfgHtml(current)}' oninput='markConfigEditorDirty()'/><div class='cfgHelpText'>Type: number</div>`;
    }

    return `<input id='${cfgHtml(inputId)}' class='cfgValueInput' data-field-id='${cfgHtml(fieldId)}' type='text' value='${cfgHtml(current)}' oninput='markConfigEditorDirty()'/>`;
}
function showConfigEditorLoading(message) { const loadState = configEditorLoadState(); const host = configEditorSectionsHost(); if (host) host.innerHTML = ''; if (loadState) { loadState.innerText = message; loadState.classList.remove('cfgHidden'); } }
function hideConfigEditorLoading() { const loadState = configEditorLoadState(); if (loadState) loadState.classList.add('cfgHidden'); }
function renderConfigEditorSections() { const host = configEditorSectionsHost(); if (!host) return; const sections = Array.isArray(configEditorState.sections) ? configEditorState.sections : []; let fieldCount = 0; host.innerHTML = sections.map(section => { const fields = Array.isArray(section.fields) ? section.fields : []; fieldCount += fields.length; const sectionComments = Array.isArray(section.comments) && section.comments.length ? `<p>${section.comments.map(line => cfgHtml(line)).join(' ')}</p>` : ''; const rows = fields.map(field => { const searchCorpus = [section.title || '', field.key || '', field.id || '', ...(Array.isArray(field.comments) ? field.comments : [])].join(' ').toLowerCase(); const fieldType = configFieldType(field); const albumBadge = field?.is_album_override ? "<span class='badge cfgAlbumBadge'>album override</span>" : ''; return `<div class='cfgFieldRow' data-config-search='${cfgHtml(searchCorpus)}'><div class='cfgStaticPane'><div class='cfgKeyRow'><strong>${cfgHtml(field.key || '')}</strong><span class='badge'>${cfgHtml(field.from_template === false ? 'Preserved key' : 'Template key')}</span>${albumBadge}<span class='cfgTypeTag'>${cfgHtml(fieldType)}</span></div><div class='cfgTemplateBox'><div>Template value</div><code>${cfgHtml(field.template_value || '')}</code></div>${configFieldHelpHtml(field)}</div><div class='cfgEditPane'><label class='cfgValueLabel' for='${cfgHtml(configFieldInputId(field.id))}'>Current value</label>${renderConfigValueInput(field)}</div></div>`; }).join(''); return `<section class='cfgSection'><div class='cfgSectionHead'><div><h4>${cfgHtml(section.title || 'Section')}</h4>${sectionComments}</div><span class='badge'>Fields: ${fields.length}</span></div>${rows}</section>`; }).join(''); const badge = configEditorFieldCountBadge(); if (badge) badge.innerText = `Fields: ${fieldCount}`; filterConfigEditorFields(); }
function collectConfigEditorValues() { const values = {}; document.querySelectorAll('.cfgValueInput[data-field-id]').forEach(el => { values[el.getAttribute('data-field-id')] = String(el.value ?? ''); }); return values; }
function configEditorIsDirty() { const currentValues = collectConfigEditorValues(); const originalValues = configEditorState.originalValues || {}; const currentKeys = Object.keys(currentValues).sort(); const originalKeys = Object.keys(originalValues).sort(); if (currentKeys.length !== originalKeys.length) return true; for (let idx = 0; idx < currentKeys.length; idx += 1) { if (currentKeys[idx] !== originalKeys[idx]) return true; if (String(currentValues[currentKeys[idx]]) !== String(originalValues[currentKeys[idx]])) return true; } return false; }
function markConfigEditorDirty() { if (!configEditorState.open) return; setConfigEditorStatus(configEditorIsDirty() ? 'Config changes pending' : 'Config unchanged'); }
function openConfigEditorBackdrop() { const backdrop = configEditorBackdrop(); if (backdrop) backdrop.style.display = 'flex'; configEditorState.open = true; }
function closeConfigEditorBackdrop() { const backdrop = configEditorBackdrop(); if (backdrop) backdrop.style.display = 'none'; configEditorState.open = false; }
async function openConfigEditor() { if (configEditorState.loading || configEditorState.saving) return; configEditorState.loading = true; openConfigEditorBackdrop(); showConfigEditorLoading('Loading config template, comments, and current values...'); setConfigEditorStatus('Loading config editor...'); try { const response = await fetch('/api/config-editor'); const payload = await response.json(); if (!payload.ok) throw new Error(payload.message || 'Failed to load config editor data.'); configEditorState.sections = payload.sections || []; configEditorState.originalValues = {}; configEditorState.albumName = String(payload.album_name || ''); configEditorState.hasAlbumOverride = !!payload.has_album_override; configEditorState.saveTarget = configEditorState.albumName ? 'album' : 'global'; (payload.sections || []).forEach(section => { (section.fields || []).forEach(field => { configEditorState.originalValues[field.id] = String(field.current_value ?? ''); }); }); hideConfigEditorLoading(); renderConfigEditorSections(); refreshConfigEditorModeUi(); const searchInput = configEditorSearchInput(); if (searchInput) searchInput.value = ''; setConfigEditorStatus('Config editor ready'); } catch (error) { showConfigEditorLoading('Failed to load config editor: ' + error); setConfigEditorStatus('Config editor failed to load'); } finally { configEditorState.loading = false; } }
function filterConfigEditorFields() { const query = String(configEditorSearchInput()?.value || '').trim().toLowerCase(); document.querySelectorAll('.cfgFieldRow[data-config-search]').forEach(row => { const corpus = String(row.getAttribute('data-config-search') || ''); row.classList.toggle('cfgHidden', !!query && !corpus.includes(query)); }); }
function configEditorExitChoice(message) { return new Promise(resolve => { const dlg = document.createElement('dialog'); dlg.className = 'cfgConfirmDialog'; const target = configEditorState.saveTarget === 'album' ? `album override (${configEditorState.albumName || 'current'})` : 'global config.toml'; dlg.innerHTML = `<h4 class='cfgConfirmTitle'>${cfgHtml(message || 'Save config changes?')}</h4>` + `<p class='cfgConfirmBody'>Save applies changes to ${cfgHtml(target)} and reloads the review UI. Discard closes the editor without saving.</p>` + "<div class='cfgConfirmActions'>" + "<button id='_cfg_cancel' class='cfgSecondaryBtn'>Continue Editing</button>" + "<button id='_cfg_discard' class='cfgDangerBtn'>Discard</button>" + "<button id='_cfg_save' class='cfgPrimaryBtn'>Save</button>" + "</div>"; document.body.appendChild(dlg); dlg.showModal(); const finish = choice => { dlg.close(); dlg.remove(); resolve(choice); }; dlg.querySelector('#_cfg_save').onclick = () => finish('save'); dlg.querySelector('#_cfg_discard').onclick = () => finish('discard'); dlg.querySelector('#_cfg_cancel').onclick = () => finish('cancel'); dlg.addEventListener('cancel', () => finish('cancel')); }); }
async function requestCloseConfigEditor() { if (!configEditorState.open || configEditorState.loading || configEditorState.saving) { closeConfigEditorBackdrop(); return; } if (!configEditorIsDirty()) { closeConfigEditorBackdrop(); setConfigEditorStatus('Config editor closed'); return; } const choice = await configEditorExitChoice('Save config changes before closing?'); if (choice === 'save') { await saveConfigEditorChanges(); return; } if (choice === 'discard') { closeConfigEditorBackdrop(); setConfigEditorStatus('Config changes discarded'); } }
async function saveConfigEditorChanges() { if (configEditorState.saving) return false; const values = collectConfigEditorValues(); configEditorState.saving = true; const saveTarget = configEditorState.saveTarget === 'album' ? 'album' : 'global'; const endpoint = saveTarget === 'album' ? '/api/config-editor-save-album' : '/api/config-editor-save'; const savingLabel = saveTarget === 'album' ? `Saving album override (${configEditorState.albumName || 'current'})...` : 'Saving global config.toml...'; setConfigEditorStatus(savingLabel); try { const response = await fetch(endpoint, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ values }) }); const payload = await response.json(); if (!payload.ok) throw new Error(payload.message || 'Failed to save config values.'); configEditorState.originalValues = { ...values }; closeConfigEditorBackdrop(); setConfigEditorStatus(payload.message || 'Config saved. Reloading...'); if (typeof setBusy === 'function') setBusy(true, 'Config saved. Reloading review UI...'); setTimeout(() => { window.location.reload(); }, payload.reload_required ? 1600 : 250); return true; } catch (error) { setConfigEditorStatus('Config save failed'); alert('Config save failed: ' + error); return false; } finally { configEditorState.saving = false; } }
async function deleteAlbumConfigOverride() { if (configEditorState.saving || configEditorState.loading) return; if (!configEditorState.albumName) return; const ok = confirm(`Delete album override config for ${configEditorState.albumName}?\n\nThis removes Albums/${configEditorState.albumName}/config.toml and falls back to global config.toml.`); if (!ok) return; configEditorState.saving = true; setConfigEditorStatus(`Deleting album override for ${configEditorState.albumName}...`); try { const response = await fetch('/api/config-editor-delete-album-override', { method: 'POST' }); const payload = await response.json(); if (!payload.ok) throw new Error(payload.message || 'Failed to delete album override.'); closeConfigEditorBackdrop(); setConfigEditorStatus(payload.message || 'Album override deleted. Reloading...'); if (typeof setBusy === 'function') setBusy(true, 'Album override deleted. Reloading review UI...'); setTimeout(() => { window.location.reload(); }, payload.reload_required ? 1600 : 250); } catch (error) { setConfigEditorStatus('Delete override failed'); alert('Delete override failed: ' + error); } finally { configEditorState.saving = false; } }
window.addEventListener('beforeunload', event => { if (!configEditorState.open || !configEditorIsDirty()) return; event.preventDefault(); event.returnValue = ''; });
configEditorBackdrop()?.addEventListener('click', event => { if (event.target === configEditorBackdrop()) requestCloseConfigEditor(); });
</script>
"#.to_string()
}

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
