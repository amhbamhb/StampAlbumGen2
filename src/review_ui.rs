use base64::Engine;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tiny_http::{Header, Method, Response, Server, StatusCode};

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
struct SelectedMatch {
    left_rowid: i64,
    colnect_id: String,
    selected_catalog_code: Option<String>,
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
}

#[derive(Debug, Serialize)]
struct ShutdownResponse {
    ok: bool,
    message: String,
    generated_output_path: Option<String>,
    generated_pdf_path: Option<String>,
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
body {{ font-family: sans-serif; margin: 20px; background: linear-gradient(180deg, #edf4f8, #f8fbfd); color: #1e3548; }}
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
</style>
</head>
<body>
<div class='card'>
    <h1>Start Review UI</h1>
    <p>No database was selected on startup. Pick a country to open its Review UI.</p>
    <div class='row'>
        <label for='countrySel'>Country</label>
        <select id='countrySel'></select>
        <button onclick='refreshState()'>Refresh</button>
        <button class='primary' onclick='startReview()'>Open Review</button>
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
            <input id='swStartYear' type='number' value='1840' />
            <label>End Year</label>
            <input id='swEndYear' type='number' value='2026' />
        </div>
        <div class='row'>
            <span class='badge'>Colnect inbox hint: {}</span>
        </div>
        <div class='row'>
            <strong>Suggested command:</strong>
            <code id='swCmd'></code>
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

function setMovingDots(active) {{
    const dots = document.getElementById('loadingDotsTitle');
    if (!dots) return;

    if (!active) {{
        if (loadingDotsTimer) {{
            clearInterval(loadingDotsTimer);
            loadingDotsTimer = null;
        }}
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

function suggestedStampworldCommand(country) {{
    const c = document.getElementById('swCountry').value || country || '';
    const y1 = document.getElementById('swStartYear').value || '1840';
    const y2 = document.getElementById('swEndYear').value || '2026';
    return `cargo run -- --source stampworld --site-name "${{c}}" --album-name "${{c}}" --start-year ${{y1}} --end-year ${{y2}}`;
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
    setBusy(true, `Preparing database and compare view for ${{sel.value}}. Watch the terminal for progress updates.`);
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
        document.getElementById('status').innerText = j.message || 'Launching review...';
        setTimeout(() => {{ window.location.href = '/'; }}, 1000);
    }} catch (e) {{
        setBusy(false, '');
        document.getElementById('status').innerText = `Failed to start: ${{e}}`;
    }}
}}

document.getElementById('countrySel').addEventListener('change', renderCountryPanel);
document.getElementById('swCountry').addEventListener('input', () => {{
    const country = document.getElementById('countrySel').value;
    document.getElementById('swCmd').innerText = suggestedStampworldCommand(country);
}});
document.getElementById('swStartYear').addEventListener('input', () => {{
    const country = document.getElementById('countrySel').value;
    document.getElementById('swCmd').innerText = suggestedStampworldCommand(country);
}});
document.getElementById('swEndYear').addEventListener('input', () => {{
    const country = document.getElementById('countrySel').value;
    document.getElementById('swCmd').innerText = suggestedStampworldCommand(country);
}});

refreshState();
</script>
</body>
</html>"#,
        inbox_hint
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
) -> Result<Vec<LeftStamp>, Box<dyn std::error::Error>> {
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
         WHERE TRIM(COALESCE(image_path, '')) != ''
           AND LOWER(COALESCE(image_path, '')) NOT LIKE 'images/no_image%'
         ORDER BY year, sg_number",
    )?;

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
        let image_url = if !page_url.trim().is_empty() {
            crate::colnect::image_fetcher::resolve_colnect_image_url(&page_url).unwrap_or_default()
        } else {
            String::new()
        };
        let description = {
            let d = row.get::<_, String>(5)?;
            if d.trim().is_empty() {
                row.get::<_, String>(6)?
            } else {
                d
            }
        };
        let image_path = row.get::<_, String>(12)?;

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
    matches: Vec<SelectedMatch>,
    fields: FieldSelection,
    right_map: &HashMap<String, RightCandidate>,
) -> ApplyMatchesResponse {
    fn normalize_catalog_code(raw: &str) -> String {
        let mut s = raw.trim().to_string();
        if let Some((_, rest)) = s.split_once(':') {
            s = rest.trim().to_string();
        }
        let mut parts = s.split_whitespace();
        if let Some(first) = parts.next() {
            if first.len() == 2 && first.chars().all(|c| c.is_ascii_alphabetic()) {
                let rest = s[first.len()..].trim_start();
                if !rest.is_empty() {
                    return rest.to_string();
                }
            }
        }
        s
    }

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
            normalize_catalog_code(&m.selected_catalog_code.unwrap_or_default());
        let selected_width = m.width.as_deref().unwrap_or(c.width.as_str());
        let selected_height = m.height.as_deref().unwrap_or(c.height.as_str());

        let res = conn.execute(
            "UPDATE stamps
             SET sg_number = CASE WHEN ?1 = 1 AND TRIM(?2) != '' THEN ?2 ELSE sg_number END,
                 series = CASE WHEN ?3 = 1 AND ?4 != '' THEN ?4 ELSE series END,
                 uniq_key = lower(coalesce(country,'')) || '|' ||
                            CASE WHEN ?3 = 1 AND ?4 != '' THEN ?4 ELSE coalesce(series,'') END || '|' ||
                            CASE WHEN ?1 = 1 AND TRIM(?2) != '' THEN ?2 ELSE coalesce(sg_number,'') END,
                 description = CASE WHEN ?5 = 1 AND ?6 != '' THEN ?6 ELSE description END,
                 watermark = CASE WHEN ?7 = 1 AND ?8 != '' THEN ?8 ELSE watermark END,
                 design = CASE WHEN ?9 = 1 AND ?10 != '' THEN ?10 ELSE design END,
                 engraving = CASE WHEN ?11 = 1 AND ?12 != '' THEN ?12 ELSE engraving END,
                 perforation = CASE WHEN ?13 = 1 AND ?14 != '' THEN ?14 ELSE perforation END,
                 context = CASE WHEN ?15 = 1 AND ?16 != '' THEN ?16 ELSE context END,
                 colnect_page_url = CASE WHEN ?17 = 1 AND ?18 != '' THEN ?18 ELSE colnect_page_url END,
                 original_image_url = CASE WHEN ?17 = 1 AND ?18 != '' THEN ?18 ELSE original_image_url END,
                 colnect_image_url = CASE WHEN ?19 = 1 AND ?20 != '' THEN ?20 ELSE colnect_image_url END,
                 remote_image_url = CASE
                    WHEN (?19 = 1) AND (?20 != '') AND (TRIM(COALESCE(remote_image_url, '')) = '' OR remote_image_url LIKE '%colnect.com/stamps/stamp/%') THEN ?20
                    ELSE remote_image_url
                 END,
                 colnect_id = CASE WHEN ?21 = 1 THEN ?22 ELSE colnect_id END,
                 colnect_name = CASE WHEN ?21 = 1 THEN ?23 ELSE colnect_name END,
                 colnect_catalog_codes = CASE WHEN ?21 = 1 THEN ?24 ELSE colnect_catalog_codes END,
                 colnect_country = CASE WHEN ?21 = 1 THEN ?25 ELSE colnect_country END,
                 colnect_width = CASE WHEN ?21 = 1 THEN ?26 ELSE colnect_width END,
                 colnect_height = CASE WHEN ?21 = 1 THEN ?27 ELSE colnect_height END,
                 colnect_format = CASE WHEN ?21 = 1 THEN ?28 ELSE colnect_format END,
                 colnect_emission = CASE WHEN ?21 = 1 THEN ?29 ELSE colnect_emission END,
                 colnect_printing = CASE WHEN ?21 = 1 THEN ?30 ELSE colnect_printing END,
                 colnect_paper = CASE WHEN ?21 = 1 THEN ?31 ELSE colnect_paper END,
                 colnect_gum = CASE WHEN ?21 = 1 THEN ?32 ELSE colnect_gum END,
                 colnect_ord_mi = CASE WHEN ?21 = 1 THEN ?33 ELSE colnect_ord_mi END,
                 colnect_ord_sn = CASE WHEN ?21 = 1 THEN ?34 ELSE colnect_ord_sn END,
                 colnect_ord_yt = CASE WHEN ?21 = 1 THEN ?35 ELSE colnect_ord_yt END,
                 colnect_ord_sg = CASE WHEN ?21 = 1 THEN ?36 ELSE colnect_ord_sg END,
                 colnect_expiry_date = CASE WHEN ?21 = 1 THEN ?37 ELSE colnect_expiry_date END,
                 colnect_print_run = CASE WHEN ?21 = 1 THEN ?38 ELSE colnect_print_run END,
                 colnect_face_value = CASE WHEN ?21 = 1 THEN ?39 ELSE colnect_face_value END,
                 colnect_currency = CASE WHEN ?21 = 1 THEN ?40 ELSE colnect_currency END,
                 colnect_colors = CASE WHEN ?21 = 1 THEN ?41 ELSE colnect_colors END,
                 colnect_parent_item = CASE WHEN ?21 = 1 THEN ?42 ELSE colnect_parent_item END,
                 colnect_variant = CASE WHEN ?21 = 1 THEN ?43 ELSE colnect_variant END,
                 colnect_overprint_variant = CASE WHEN ?21 = 1 THEN ?44 ELSE colnect_overprint_variant END,
                 colnect_score = CASE WHEN ?21 = 1 THEN ?45 ELSE colnect_score END,
                 colnect_accuracy = CASE WHEN ?21 = 1 THEN ?46 ELSE colnect_accuracy END,
                 colnect_list = CASE WHEN ?21 = 1 THEN ?47 ELSE colnect_list END,
                 colnect_quantity = CASE WHEN ?21 = 1 THEN ?48 ELSE colnect_quantity END,
                 colnect_condition = CASE WHEN ?21 = 1 THEN ?49 ELSE colnect_condition END,
                 colnect_public_note = CASE WHEN ?21 = 1 THEN ?50 ELSE colnect_public_note END,
                 colnect_private_note = CASE WHEN ?21 = 1 THEN ?51 ELSE colnect_private_note END
             WHERE rowid = ?52",
            params![
                if fields.replace_sg_number { 1 } else { 0 },
                selected_catalog_code,
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
.filtersCard {{ border: 1px solid #dbe4ec; border-left: 5px solid var(--accent-blue); border-radius: 10px; background: #f8fbfe; padding: 10px; margin-bottom: 12px; }}
.filtersTitle {{ margin: 0 0 8px 0; font-size: 13px; color: #1f4663; }}
.filtersGrid {{ display: flex; gap: 10px; align-items: center; flex-wrap: wrap; }}
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
.modal {{ background: #fff; border-radius: 10px; border: 1px solid #ddd; width: min(640px, 95vw); padding: 14px; box-sizing: border-box; }}
.modal h4 {{ margin: 0 0 10px 0; }}
.modal .rowx {{ display: flex; gap: 10px; margin-bottom: 8px; flex-wrap: wrap; }}
.modal .rowx label {{ display: inline-flex; align-items: center; gap: 6px; }}
.modal input[type='text'], .modal input[type='number'] {{ padding: 6px 8px; border: 1px solid #ccc; border-radius: 6px; min-width: 140px; }}
@media (max-width: 980px) {{
    .row {{ flex-direction: column; }}
    .panelBody {{ max-height: 42vh; }}
}}
</style>
</head>
<body>
<div class='wrap'>
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
        <div class='groupCard match'>
            <h4>Match Actions</h4>
            <div class='groupBtns'>
                <button class='success' onclick='assignCurrent()'>Assign Candidate</button>
                <button onclick='clearAssignment()'>Clear Assignment</button>
                <button class='primary' onclick='applyMatches()'>Apply Matches To DB</button>
            </div>
        </div>
        <div class='groupCard candidate'>
            <h4>Candidate Management</h4>
            <div class='groupBtns'>
                <button class='warn' onclick='openAddDialog()'>Add Colnect As Stamp</button>
                <button class='danger' onclick='deleteCurrentCandidate()'>Delete Candidate</button>
                <button onclick='undoLastDelete()'>Undo Delete</button>
            </div>
        </div>
        <div class='groupCard session'>
            <h4>Session</h4>
            <div class='groupBtns'>
                <button class='neutral' onclick='closeReviewUi()'>Close Review UI</button>
                <span id='status' class='badge'>Ready</span>
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
      </div>
    </div>
  <div id='comparePanels' class='row'>
    <div class='col'>
            <div class='colNav'>
                <button class='navBtn' onclick='prevLeft()'>Prev Left</button>
                <button class='navBtn' onclick='nextLeft()'>Next Left</button>
            </div>
            <div class='titleRow'>
                <h3>Left: StampWorld Base</h3>
                <span id='leftPos' class='badge'></span>
            </div>
      <div id='leftPanel'></div>
    </div>
    <div class='col'>
            <div class='colNav'>
                <button class='navBtn' onclick='prevRight()'>Prev Candidate</button>
                <button class='navBtn' onclick='nextRight()'>Next Candidate</button>
            </div>
            <div class='titleRow'>
                <h3>Right: Colnect Candidate</h3>
                <span id='rightPos' class='badge'></span>
            </div>
      <div id='rightPanel'></div>
    </div>
  </div>
</div>

<div id='addModalBackdrop' class='modal-backdrop'>
    <div class='modal'>
        <h4>Add Colnect Candidate To stamps</h4>
        <div class='rowx'>
            <label>SG <input id='addSg' type='text' /></label>
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
<script>
const payload = {json};
let left = payload.left || [];
let rightAll = payload.right || [];
let leftIdx = 0;
let rightIdxByLeft = {{}};
let assignedByLeft = {{}}; // left_rowid -> colnect_id
let selectedCatalogCodeByLeft = {{}}; // left_rowid -> chosen code
let deletedCandidatesStack = []; // stack entries: candidate + removedAssignments
let guideDismissed = false;

function removeCandidateFromUi(colnectId) {{
    if (!colnectId) return;
    rightAll = rightAll.filter(c => c.colnect_id !== colnectId);
    Object.keys(assignedByLeft).forEach(k => {{
        if (assignedByLeft[k] === colnectId) delete assignedByLeft[k];
    }});
    Object.keys(selectedCatalogCodeByLeft).forEach(k => {{
        if (!assignedByLeft[k]) delete selectedCatalogCodeByLeft[k];
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
function imageBox(src, label) {{
  if (b(src)) return `<div class='imgbox'><img src='${{src}}' alt='${{html(label)}}'/></div>`;
  return `<div class='imgbox'><div class='imgph'>${{html(label)}}<br/>No image available</div></div>`;
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
    return rightAll.filter(c => {{
        const keepUsed = (!used.has(c.colnect_id) || c.colnect_id === assignedForCurrent);
        if (!keepUsed) return false;
        if (!unmatchedOnly) return true;
        if (c.colnect_id === assignedForCurrent) return true;
        return !matchedInDb.has(String(c.colnect_id || '').trim());
    }});
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
    const sgLike = pairs.find(p => String(p.catalog_type || '').toLowerCase() === 'sg');
    return sgLike ? sgLike.code : (pairs[0].code || '');
}}

function render() {{
    updateGuide();
  if (!left.length) {{
        document.getElementById('leftPos').innerText = 'Left 0/0';
        document.getElementById('rightPos').innerText = 'Candidate 0/0';
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
    return;
  }}

  const key = String(l.rowid);
  document.getElementById('rightPos').innerText = `Candidate ${{(rightIdxByLeft[key] || 0) + 1}}/${{avail.length}}`;
  const rightImg = b(r.image_abs) ? `/image?path=${{encodeURIComponent(r.image_abs)}}` : r.image_url;
    const catPairs = (r.catalog_code_pairs || []).map(p =>
        `<option value='${{html(p.code)}}'>${{html(p.catalog_type)}}: ${{html(p.code)}}</option>`
    ).join('');
    const selectedCode = selectedCatalogCodeForCurrent(r);
  document.getElementById('rightPanel').innerHTML = `
        <div class='panelCard'>
            <div class='panelHead'>
                <div><strong>Candidate Controls</strong></div>
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
    document.getElementById('addSg').value = r.sg_number || '';
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
            removedAssignments.push([k, assignedByLeft[k], selectedCatalogCodeByLeft[k] || '']);
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
    for (const [k, cid, code] of last.removedAssignments || []) {{
        assignedByLeft[k] = cid;
        if (code) selectedCatalogCodeByLeft[k] = code;
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
        body: JSON.stringify({{ generate_album: generateAlbum, generate_pdf: choice.generatePdf }})
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

render();
</script>
</body>
</html>"#,
        json = json
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

fn album_dir_for_ui(db_path: &str, album_name: &str) -> PathBuf {
    let album_slug = album_name.trim().replace(' ', "-");
    let preferred = Path::new("Albums").join(&album_slug);
    if preferred.exists() {
        return preferred;
    }
    Path::new(db_path)
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."))
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

    let sg_number = req
        .target_sg_number
        .unwrap_or_else(|| candidate.sg_number.clone());
    let year = req.target_year.unwrap_or(candidate.year);
    let series = req
        .target_series
        .unwrap_or_else(|| candidate.series.clone());

    let insert_res = conn.execute(
        "INSERT INTO stamps (
            country, series, year, sg_number, uniq_key,
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
            ?1, ?2, ?3, ?4, ?5,
            ?6, ?7, ?8, ?9,
            ?10, ?11, ?12, ?13,
            ?14, ?15, ?16, ?17,
            ?18, ?19, ?20, ?21,
            ?22, ?23, ?24, ?25,
            ?26, ?27, ?28, ?29,
            ?30, ?31, ?32, ?33,
            ?34, ?35, ?36, ?37,
            ?38, ?39, ?40, ?41,
            ?42, ?43, ?44, ?45, ?46
         )",
        params![
            candidate.country,
            series.clone(),
            year,
            sg_number.clone(),
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

pub fn run_review_ui_bootstrap(
    bind_addr_opt: Option<&str>,
    default_inbox_path: Option<&str>,
) -> Result<Option<(String, String)>, Box<dyn std::error::Error>> {
    let html = render_bootstrap_html(default_inbox_path);
    let (server, bind) = bind_review_server(bind_addr_opt)?;

    let url = format!("http://{}/", bind);
    eprintln!("Review UI Setup: {}", url);
    let _ = webbrowser::open(&url);

    let mut selected: Option<(String, String)> = None;

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
                            selected = Some((c.db_path.clone(), c.country.clone()));
                            BootstrapStartResponse {
                                ok: true,
                                message: format!("starting review for {}", c.country),
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

        let _ = request.respond(Response::empty(StatusCode(404)));
    }

    Ok(selected)
}

pub fn run_review_ui(
    db_path: &str,
    album_name_opt: Option<&str>,
    bind_addr_opt: Option<&str>,
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
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = crate::db_utils::init_db(db_path)?;
    sync_processed_colnect_candidates(&conn)?;

    let album_name = album_name_opt
        .map(|s| s.to_string())
        .or_else(|| {
            Path::new(db_path)
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
        })
        .unwrap_or_else(|| "Album".to_string());

    let left = load_left_stamps(&conn, db_path, &album_name)?;
    let right = load_right_candidates(&conn, db_path, &album_name)?;

    let right_map: HashMap<String, RightCandidate> = right
        .iter()
        .cloned()
        .map(|c| (c.colnect_id.clone(), c))
        .collect();

    let payload = MatcherPayload {
        left,
        right,
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
    };
    let html = render_html(&payload);

    let (server, bind) = bind_review_server(bind_addr_opt)?;

    let url = format!("http://{}/", bind);
    eprintln!("Review UI: {}", url);
    let _ = webbrowser::open(&url);

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
                                    None,
                                    None,
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

    Ok(())
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
