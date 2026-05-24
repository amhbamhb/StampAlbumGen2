use base64::Engine;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::Duration;
use tiny_http::{Header, Method, Response, Server, StatusCode};

mod views;
use views::{render_bootstrap_html, render_html};

mod config_editor;
use config_editor::{
    build_config_editor_data, build_config_editor_data_for_album, render_config_editor_assets,
    save_album_config_diff, save_full_config_values, update_default_catalog_code_for_album,
};

mod matcher_service;
use matcher_service::{
    add_colnect_stamp_to_stamps, add_left_stamp, apply_matches, clear_left_colnect_link,
    delete_colnect_candidate, delete_left_stamp, load_left_stamps, load_right_candidates,
    save_left_stamp, sync_processed_colnect_candidates, undo_colnect_delete,
};
mod http_handlers;
use http_handlers::{serve_bootstrap_requests, serve_review_requests};

static PDF_GENERATION_IN_PROGRESS: AtomicBool = AtomicBool::new(false);
const RUNTIME_LOG_CAP: usize = 800;
static REVIEW_UI_RUNTIME_LOG: OnceLock<Mutex<VecDeque<String>>> = OnceLock::new();
static STAMPWORLD_JOB_STATE: OnceLock<Mutex<StampWorldJobState>> = OnceLock::new();
static BOOTSTRAP_ALBUMS_ROOT_OVERRIDE: OnceLock<Mutex<Option<PathBuf>>> = OnceLock::new();

fn runtime_log_buffer() -> &'static Mutex<VecDeque<String>> {
    REVIEW_UI_RUNTIME_LOG.get_or_init(|| Mutex::new(VecDeque::with_capacity(RUNTIME_LOG_CAP)))
}

fn stampworld_job_state() -> &'static Mutex<StampWorldJobState> {
    STAMPWORLD_JOB_STATE.get_or_init(|| Mutex::new(StampWorldJobState::Idle))
}

fn bootstrap_albums_root_override() -> &'static Mutex<Option<PathBuf>> {
    BOOTSTRAP_ALBUMS_ROOT_OVERRIDE.get_or_init(|| Mutex::new(None))
}

pub fn log_runtime_event(message: &str) {
    eprintln!("{}", message);
    if let Ok(mut q) = runtime_log_buffer().lock() {
        q.push_back(message.to_string());
        while q.len() > RUNTIME_LOG_CAP {
            let _ = q.pop_front();
        }
    }
}

pub fn get_runtime_log_lines() -> Vec<String> {
    runtime_log_buffer()
        .lock()
        .map(|q| q.iter().cloned().collect())
        .unwrap_or_default()
}

/// Tail the most recent log file in the Logs/ directory, returning the last N lines.
/// Used during StampWorld pulls to display real-time feedback of DB inserts and progress.
pub fn tail_current_log(lines_count: usize) -> Vec<String> {
    // Find the most recent log file in Logs/
    let logs_dir = std::path::Path::new("Logs");
    if !logs_dir.exists() {
        return vec![];
    }

    let mut log_files: Vec<_> = fs::read_dir(logs_dir)
        .ok()
        .and_then(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter(|e| e.file_name().to_string_lossy().ends_with(".log"))
                .collect::<Vec<_>>()
                .into_iter()
                .filter_map(|e| {
                    e.metadata()
                        .ok()
                        .map(|m| (e.path(), m.modified().unwrap_or(std::time::UNIX_EPOCH)))
                })
                .collect::<Vec<_>>()
                .into()
        })
        .unwrap_or_default();

    if log_files.is_empty() {
        return vec![];
    }

    // Sort by modification time, most recent first
    log_files.sort_by(|a, b| b.1.cmp(&a.1));
    let log_path = &log_files[0].0;

    // Read the last N lines efficiently
    match fs::File::open(log_path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
            lines
                .iter()
                .rev()
                .take(lines_count)
                .rev()
                .cloned()
                .collect()
        }
        Err(_) => vec![],
    }
}

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
    catalog_code: String,
    year: i32,
    month_num: i32,
    day: String,
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
    colnect_dir: String,
    colnect_file_count: usize,
    show_imageless_left_rows: bool,
    default_catalog_code: String,
    default_sort_mode: String,
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
    catalog_family_options: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct BootstrapCountryInfo {
    country: String,
    db_path: String,
    colnect_file_count: usize,
    colnect_candidate_count: usize,
    colnect_dir: String,
}

#[derive(Debug, Serialize)]
struct BootstrapStateResponse {
    countries: Vec<BootstrapCountryInfo>,
    albums_root: Option<String>,
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
struct BootstrapStampWorldRunRequest {
    country: String,
    start_year: i32,
    end_year: i32,
    #[serde(default)]
    preset: Option<String>,
    #[serde(default)]
    extra_flags: Option<String>,
}

#[derive(Debug, Serialize)]
struct BootstrapStampWorldRunResponse {
    ok: bool,
    message: String,
    started: bool,
}

#[derive(Debug, Serialize)]
struct BootstrapStampWorldStatusResponse {
    ok: bool,
    running: bool,
    finished: bool,
    success: Option<bool>,
    message: String,
    country: Option<String>,
    db_path: Option<String>,
}

#[derive(Debug, Serialize)]
struct BootstrapAlbumEasyStatusResponse {
    ok: bool,
    configured: bool,
    cli_path: Option<String>,
    path_exists: bool,
    message: String,
    download_url: String,
}

#[derive(Debug, Serialize)]
struct StampWorldLogTailResponse {
    ok: bool,
    lines: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct BootstrapAlbumEasySetPathRequest {
    cli_path: String,
}

#[derive(Debug, Deserialize)]
struct BootstrapAlbumEasyPickPathRequest {
    current_path: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BootstrapSetAlbumsRootRequest {
    path: String,
}

#[derive(Debug, Serialize)]
struct BootstrapSetAlbumsRootResponse {
    ok: bool,
    message: String,
    albums_root: Option<String>,
    countries_count: usize,
}

#[derive(Debug, Serialize)]
struct BootstrapPickAlbumsRootResponse {
    ok: bool,
    picked: bool,
    message: String,
    albums_root: Option<String>,
    countries_count: usize,
}

#[derive(Debug, Deserialize)]
struct BootstrapPickColnectFileRequest {
    country: String,
}

#[derive(Debug, Serialize)]
struct BootstrapPickColnectFileResponse {
    ok: bool,
    picked: bool,
    message: String,
    file_name: Option<String>,
    dest_path: Option<String>,
    colnect_dir: Option<String>,
}

#[derive(Debug, Serialize)]
struct BootstrapAlbumEasySetPathResponse {
    ok: bool,
    message: String,
}

#[derive(Debug, Serialize)]
struct BootstrapAlbumEasyPickPathResponse {
    ok: bool,
    picked: bool,
    message: String,
    cli_path: Option<String>,
}

#[derive(Debug, Clone)]
enum StampWorldJobState {
    Idle,
    Running {
        country: String,
        db_path: String,
    },
    Finished {
        success: bool,
        country: String,
        db_path: String,
        message: String,
    },
}

fn parse_extra_flags(extra_flags: &Option<String>) -> Vec<String> {
    extra_flags
        .as_deref()
        .unwrap_or("")
        .split_whitespace()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string())
        .collect()
}

fn preset_flags(preset: &Option<String>) -> Vec<String> {
    match preset.as_deref().unwrap_or("basic") {
        "overwrite_album" => vec!["--overwrite-album".to_string()],
        "overwrite_all" => vec![
            "--overwrite-album".to_string(),
            "--overwrite-images".to_string(),
        ],
        "bounded_pages" => vec![
            "--max-pages".to_string(),
            "25".to_string(),
            "--stop-after-no-hits".to_string(),
            "3".to_string(),
        ],
        "accept_typo" => vec!["--yes".to_string()],
        _ => Vec::new(),
    }
}

fn build_stampworld_args(req: &BootstrapStampWorldRunRequest) -> Vec<String> {
    let country = req.country.trim().to_string();
    let mut args = vec![
        "--source".to_string(),
        "stampworld".to_string(),
        "--site-name".to_string(),
        country.clone(),
        "--album-name".to_string(),
        country,
        "--start-year".to_string(),
        req.start_year.to_string(),
        "--end-year".to_string(),
        req.end_year.to_string(),
    ];
    args.extend(preset_flags(&req.preset));
    args.extend(parse_extra_flags(&req.extra_flags));
    args
}

fn quote_for_log(arg: &str) -> String {
    if arg.contains(' ') || arg.contains('"') {
        format!("\"{}\"", arg.replace('"', "\\\""))
    } else {
        arg.to_string()
    }
}

fn set_stampworld_state(new_state: StampWorldJobState) {
    if let Ok(mut state) = stampworld_job_state().lock() {
        *state = new_state;
    }
}

fn find_global_config_path() -> Option<std::path::PathBuf> {
    // Search up from cwd, then from exe location — mirrors load_config() logic.
    let mut search_dirs: Vec<std::path::PathBuf> = Vec::new();
    if let Ok(mut dir) = std::env::current_dir() {
        for _ in 0..8 {
            search_dirs.push(dir.join("config.toml"));
            if !dir.pop() {
                break;
            }
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(mut dir) = exe.parent() {
            for _ in 0..8 {
                search_dirs.push(dir.join("config.toml"));
                match dir.parent() {
                    Some(p) => dir = p,
                    None => break,
                }
            }
        }
    }
    search_dirs.into_iter().find(|p| p.exists())
}

fn save_runtime_string_setting(key: &str, value: Option<&str>) -> Result<(), String> {
    ensure_runtime_startup_files()?;
    let config_path =
        find_global_config_path().ok_or_else(|| "could not locate config.toml".to_string())?;
    let content = fs::read_to_string(&config_path)
        .map_err(|e| format!("failed to read {}: {}", config_path.display(), e))?;
    let mut root_table = toml::from_str::<toml::Table>(&content)
        .map_err(|e| format!("invalid TOML in {}: {}", config_path.display(), e))?;

    let runtime_entry = root_table
        .entry("runtime")
        .or_insert_with(|| toml::Value::Table(toml::map::Map::new()));
    if !runtime_entry.is_table() {
        return Err("config runtime section is not a table".to_string());
    }

    let runtime_tbl = runtime_entry
        .as_table_mut()
        .ok_or_else(|| "failed to access runtime table".to_string())?;
    match value.map(str::trim).filter(|v| !v.is_empty()) {
        Some(v) => {
            runtime_tbl.insert(key.to_string(), toml::Value::String(v.to_string()));
        }
        None => {
            runtime_tbl.remove(key);
        }
    }

    let rendered = toml::to_string_pretty(&root_table)
        .map_err(|e| format!("failed to render updated config TOML: {}", e))?;
    fs::write(&config_path, rendered)
        .map_err(|e| format!("failed to write {}: {}", config_path.display(), e))?;
    Ok(())
}

fn ensure_runtime_startup_files() -> Result<(), String> {
    crate::config::ensure_config_and_template_files("config.toml", "config_template.toml")
        .map_err(|e| format!("failed to initialize startup config files: {}", e))
}

fn is_directory_empty(path: &Path) -> Result<bool, String> {
    let mut entries =
        fs::read_dir(path).map_err(|e| format!("failed to read {}: {}", path.display(), e))?;
    Ok(entries.next().is_none())
}

fn save_runtime_albumeasy_cli_path(cli_path: &str) -> Result<(), String> {
    let path = cli_path.trim();
    if path.is_empty() {
        return Err("AlbumEasy CLI path cannot be empty".to_string());
    }

    save_runtime_string_setting("albumeasy_cli_path", Some(path))
}

fn save_runtime_bootstrap_albums_root_path(path: Option<&str>) -> Result<(), String> {
    save_runtime_string_setting("bootstrap_albums_root_path", path)
}

fn start_bootstrap_stampworld_job(req: BootstrapStampWorldRunRequest) -> Result<String, String> {
    let country = req.country.trim().to_string();
    if country.is_empty() {
        return Err("Country is required".to_string());
    }
    if req.start_year > req.end_year {
        return Err("Start year must be less than or equal to end year".to_string());
    }

    let slug = country.replace(' ', "-");
    let albums_root = resolve_bootstrap_albums_root().unwrap_or_else(|| PathBuf::from("Albums"));
    let db_path = albums_root
        .join(&slug)
        .join(format!("{}.db", slug))
        .to_string_lossy()
        .to_string();
    let command_cwd = albums_root
        .parent()
        .map(|p| p.to_path_buf())
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."));

    {
        let mut state = stampworld_job_state()
            .lock()
            .map_err(|_| "Failed to lock StampWorld state".to_string())?;
        if matches!(&*state, StampWorldJobState::Running { .. }) {
            return Err("A StampWorld pull is already running".to_string());
        }
        *state = StampWorldJobState::Running {
            country: country.clone(),
            db_path: db_path.clone(),
        };
    }

    let exe =
        std::env::current_exe().map_err(|e| format!("failed to resolve executable: {}", e))?;
    let args = build_stampworld_args(&req);
    let display_cmd = format!(
        "{} {}",
        exe.display(),
        args.iter()
            .map(|a| quote_for_log(a))
            .collect::<Vec<_>>()
            .join(" ")
    );
    let country_for_thread = country.clone();
    let db_path_for_thread = db_path.clone();
    let command_cwd_for_thread = command_cwd.clone();

    log_runtime_event(&format!("StampWorld: starting command {}", display_cmd));
    log_runtime_event("StampWorld: running. You can watch progress in Runtime Log.");

    thread::spawn(move || {
        let albums_root_env = albums_root.to_string_lossy().to_string();
        let launch_with_cwd = |use_cwd: bool| -> std::io::Result<std::process::Child> {
            let mut cmd = Command::new(&exe);
            if use_cwd {
                cmd.current_dir(&command_cwd_for_thread);
            }
            cmd.env("STAMP_ALBUMS_ROOT", &albums_root_env)
                .args(&args)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());
            cmd.spawn()
        };

        let use_preferred_cwd = command_cwd_for_thread.exists() && command_cwd_for_thread.is_dir();
        if !use_preferred_cwd {
            log_runtime_event(&format!(
                "StampWorld: command cwd is not usable, falling back to process default cwd: {}",
                command_cwd_for_thread.display()
            ));
        }

        let mut child = match launch_with_cwd(use_preferred_cwd) {
            Ok(c) => c,
            Err(first_err) => {
                let should_retry_without_cwd =
                    use_preferred_cwd && cfg!(windows) && first_err.raw_os_error() == Some(123);
                if should_retry_without_cwd {
                    log_runtime_event(&format!(
                        "StampWorld: launch failed with invalid path syntax (os error 123). Retrying without custom cwd (cwd was {}).",
                        command_cwd_for_thread.display()
                    ));
                    match launch_with_cwd(false) {
                        Ok(c) => c,
                        Err(retry_err) => {
                            let msg = format!(
                                "StampWorld: failed to launch command after retry: {} (exe={}, cwd={}, args={})",
                                retry_err,
                                exe.display(),
                                command_cwd_for_thread.display(),
                                args.iter()
                                    .map(|a| quote_for_log(a))
                                    .collect::<Vec<_>>()
                                    .join(" ")
                            );
                            log_runtime_event(&msg);
                            set_stampworld_state(StampWorldJobState::Finished {
                                success: false,
                                country: country_for_thread.clone(),
                                db_path: db_path_for_thread.clone(),
                                message: msg,
                            });
                            return;
                        }
                    }
                } else {
                    let msg = format!(
                        "StampWorld: failed to launch command: {} (exe={}, cwd={}, args={})",
                        first_err,
                        exe.display(),
                        command_cwd_for_thread.display(),
                        args.iter()
                            .map(|a| quote_for_log(a))
                            .collect::<Vec<_>>()
                            .join(" ")
                    );
                    log_runtime_event(&msg);
                    set_stampworld_state(StampWorldJobState::Finished {
                        success: false,
                        country: country_for_thread.clone(),
                        db_path: db_path_for_thread.clone(),
                        message: msg,
                    });
                    return;
                }
            }
        };

        let out_handle = child.stdout.take().map(|stdout| {
            thread::spawn(move || {
                let reader = BufReader::new(stdout);
                for line in reader.lines().map_while(Result::ok) {
                    if !line.trim().is_empty() {
                        log_runtime_event(&format!("StampWorld: {}", line));
                    }
                }
            })
        });

        let err_handle = child.stderr.take().map(|stderr| {
            thread::spawn(move || {
                let reader = BufReader::new(stderr);
                for line in reader.lines().map_while(Result::ok) {
                    if !line.trim().is_empty() {
                        log_runtime_event(&format!("StampWorld [stderr]: {}", line));
                    }
                }
            })
        });

        let wait_result = child.wait();
        if let Some(h) = out_handle {
            let _ = h.join();
        }
        if let Some(h) = err_handle {
            let _ = h.join();
        }

        match wait_result {
            Ok(status) if status.success() => {
                let msg = format!(
                    "StampWorld pull completed for {}. Open Review to continue.",
                    country_for_thread
                );
                log_runtime_event(&msg);
                set_stampworld_state(StampWorldJobState::Finished {
                    success: true,
                    country: country_for_thread.clone(),
                    db_path: db_path_for_thread.clone(),
                    message: msg,
                });
            }
            Ok(status) => {
                let msg = format!(
                    "StampWorld pull failed for {} (exit status {}).",
                    country_for_thread, status
                );
                log_runtime_event(&msg);
                set_stampworld_state(StampWorldJobState::Finished {
                    success: false,
                    country: country_for_thread.clone(),
                    db_path: db_path_for_thread.clone(),
                    message: msg,
                });
            }
            Err(e) => {
                let msg = format!(
                    "StampWorld command wait failed for {}: {}",
                    country_for_thread, e
                );
                log_runtime_event(&msg);
                set_stampworld_state(StampWorldJobState::Finished {
                    success: false,
                    country: country_for_thread.clone(),
                    db_path: db_path_for_thread.clone(),
                    message: msg,
                });
            }
        }
    });

    Ok(format!(
        "StampWorld pull started for {}. Command: {}",
        country, display_cmd
    ))
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
    skipped_details: Vec<String>,
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
    catalog_code: String,
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
    catalog_code: String,
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
    country: Option<String>,
}

#[derive(Debug, Serialize)]
struct ColnectImportResponse {
    ok: bool,
    message: String,
    files_processed: usize,
    stamps_imported: usize,
    stamps_skipped: usize,
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
    AlreadyRunning,
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

    #[cfg(windows)]
    {
        let raw = canonical.to_string_lossy().replace('\\', "/");
        let normalized = raw.strip_prefix("//?/").unwrap_or(&raw);
        let encoded = normalized
            .split('/')
            .enumerate()
            .map(|(idx, segment)| {
                // Keep Windows drive prefix (for example C:) readable in file:/// URLs.
                if idx == 0 && segment.ends_with(':') {
                    segment.to_string()
                } else {
                    urlencoding::encode(segment).into_owned()
                }
            })
            .collect::<Vec<_>>()
            .join("/");
        return Some(format!("file:///{}", encoded));
    }

    #[cfg(not(windows))]
    {
        let encoded = canonical
            .to_string_lossy()
            .split('/')
            .map(|segment| urlencoding::encode(segment).into_owned())
            .collect::<Vec<_>>()
            .join("/");
        Some(format!("file://{}", encoded))
    }
}

fn open_local_path(path: &Path) -> bool {
    let Some(url) = path_to_file_url(path) else {
        log::warn!("Failed to build file URL for {}", path.display());
        return false;
    };

    if let Err(error) = webbrowser::open(&url) {
        log::warn!(
            "Failed to open {} in default viewer: {}",
            path.display(),
            error
        );
        return false;
    }

    true
}

fn try_spawn_browser_command(program: &str, args: &[&str]) -> bool {
    Command::new(program)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .is_ok()
}

fn try_spawn_browser_command_any(programs: &[&str], args: &[&str]) -> bool {
    for program in programs {
        if try_spawn_browser_command(program, args) {
            return true;
        }
    }
    false
}

fn try_open_url_once(url: &str) -> bool {
    if webbrowser::open(url).is_ok() {
        return true;
    }

    if try_spawn_browser_command_any(&["xdg-open", "/usr/bin/xdg-open", "/bin/xdg-open"], &[url]) {
        return true;
    }
    if try_spawn_browser_command_any(&["gio", "/usr/bin/gio", "/bin/gio"], &["open", url]) {
        return true;
    }
    if try_spawn_browser_command_any(&["kde-open5", "/usr/bin/kde-open5"], &[url]) {
        return true;
    }
    if try_spawn_browser_command_any(&["kde-open6", "/usr/bin/kde-open6"], &[url]) {
        return true;
    }
    if try_spawn_browser_command_any(&["kde-open", "/usr/bin/kde-open"], &[url]) {
        return true;
    }
    if try_spawn_browser_command_any(&["sensible-browser", "/usr/bin/sensible-browser"], &[url]) {
        return true;
    }

    false
}

fn open_review_ui_url(url: &str) {
    log_startup(&format!("Review UI: opening browser → {}", url));
    for attempt in 1..=4 {
        if try_open_url_once(url) {
            if attempt > 1 {
                log_runtime_event(&format!(
                    "Review UI: browser auto-open succeeded on retry {}.",
                    attempt
                ));
            }
            return;
        }
        thread::sleep(Duration::from_millis(350));
    }

    log_runtime_event(&format!(
        "Review UI: automatic browser launch failed. Open this URL manually: {}",
        url
    ));
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

    // Debug builds use a different base port so a dev instance and a release
    // instance can run side-by-side without fighting over ports.
    const RELEASE_BASE: u16 = 7878;
    const DEBUG_BASE: u16 = 7879;
    let base_port: u16 = if cfg!(debug_assertions) {
        DEBUG_BASE
    } else {
        RELEASE_BASE
    };
    let build_label = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    log_startup(&format!(
        "Review UI startup: build={}, port range {}-{}",
        build_label,
        base_port,
        base_port + 9,
    ));

    const MAX_TRIES: u16 = 10;
    let mut last_err = String::new();
    for offset in 0..MAX_TRIES {
        let candidate = format!("127.0.0.1:{}", base_port + offset);

        match probe_port(&candidate) {
            PortProbeResult::OurApp(url) if offset == 0 => {
                // Base port is already serving our app — reuse it.
                log_startup(&format!(
                    "Review UI: {} instance already running at {} — reusing",
                    build_label, url
                ));
                return Err(std::io::Error::other(format!("__already_running__{}", url)).into());
            }
            PortProbeResult::OurApp(_) | PortProbeResult::OtherApp => {
                // Another session (ours or foreign) owns this port — skip it.
                log_startup(&format!(
                    "Review UI: port {} in use, trying next",
                    base_port + offset
                ));
                last_err = format!("port {} in use", base_port + offset);
                continue;
            }
            PortProbeResult::Free => {}
        }

        match Server::http(&candidate) {
            Ok(srv) => {
                log_startup(&format!("Review UI: bound to {}", candidate));
                return Ok((srv, candidate));
            }
            Err(e) => {
                last_err = format!("{}", e);
            }
        }
    }

    Err(std::io::Error::other(format!(
        "failed to start review UI server: all ports {}-{} are in use (last error: {}). \
                 Kill the existing process with `fuser -k {}/tcp` or specify a free port with \
                 `--review-ui-bind 127.0.0.1:<PORT>`.",
        base_port,
        base_port + MAX_TRIES - 1,
        last_err,
        base_port,
    ))
    .into())
}

/// If `err` is the sentinel error from `bind_review_server` indicating that
/// our own app is already listening, extract and return the URL.
fn already_running_url(err: &Box<dyn std::error::Error>) -> Option<String> {
    err.to_string()
        .strip_prefix("__already_running__")
        .map(|url| url.to_string())
}

enum PortProbeResult {
    /// Nothing is listening on this port.
    Free,
    /// Our app (same build type: debug or release) is listening.
    OurApp(String),
    /// Something else is listening (foreign app, other build type, old binary).
    OtherApp,
}

/// Probe `addr` with a quick HTTP GET `/api/ping`.
/// Classifies the result as Free / OurApp / OtherApp.
/// Append `msg` to a startup log file in the system temp directory.
/// Useful for diagnosing port selection when the app is launched from a
/// desktop shortcut with no terminal (Terminal=false).
fn log_startup(msg: &str) {
    use std::io::Write;
    let ts = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");

    // Write to every candidate location — Wayland desktop sessions may use a
    // different TMPDIR than the interactive shell, so we cover both.
    let home = std::env::var("HOME")
        .ok()
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"));
    let candidates: &[std::path::PathBuf] = &[
        std::env::temp_dir().join("stamp_album_gen2_startup.log"),
        // Home-dir fallback: reachable regardless of TMPDIR.
        home.join(".local")
            .join("share")
            .join("stamp_album_gen2_startup.log"),
    ];
    for path in candidates {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(mut f) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
        {
            let _ = writeln!(f, "[{}] {}", ts, msg);
        }
    }
    log_runtime_event(msg);
}

fn probe_port(addr: &str) -> PortProbeResult {
    use std::io::{Read, Write};
    use std::net::TcpStream;

    let sock_addr: std::net::SocketAddr = match addr.parse() {
        Ok(a) => a,
        Err(_) => return PortProbeResult::Free,
    };

    let mut stream = match TcpStream::connect_timeout(&sock_addr, Duration::from_millis(150)) {
        Ok(s) => s,
        Err(_) => return PortProbeResult::Free, // nothing listening
    };

    let _ = stream.set_read_timeout(Some(Duration::from_millis(500)));

    let req = format!(
        "GET /api/ping HTTP/1.0\r\nHost: {}\r\nConnection: close\r\n\r\n",
        addr
    );
    if stream.write_all(req.as_bytes()).is_err() {
        return PortProbeResult::OtherApp;
    }

    // Read in a loop: HTTP/1.0 + Connection:close means the server closes
    // after the response, but TCP may deliver headers and body in separate
    // reads, so we accumulate until EOF or timeout.
    let mut buf = vec![0u8; 4096];
    let mut total = 0usize;
    loop {
        match stream.read(&mut buf[total..]) {
            Ok(0) => break,
            Ok(n) => {
                total += n;
                if total >= buf.len() {
                    break;
                }
            }
            Err(_) => break, // read timeout or error — use whatever we got
        }
    }
    let response = std::str::from_utf8(&buf[..total]).unwrap_or("");

    // Grab the body after the HTTP header blank line.
    let body = response.split("\r\n\r\n").nth(1).unwrap_or("").trim();

    let expected_build = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };

    let result = if body.contains("\"stamp_album_gen2\"") && body.contains(expected_build) {
        PortProbeResult::OurApp(format!("http://{}/", addr))
    } else if body.contains("\"stamp_album_gen2\"") {
        // Our app but wrong build type (e.g. debug is here, we're release).
        PortProbeResult::OtherApp
    } else {
        // Either no response body, 404, or a completely foreign service.
        PortProbeResult::OtherApp
    };

    let label = match &result {
        PortProbeResult::Free => "Free".to_string(),
        PortProbeResult::OurApp(u) => format!("OurApp({})", u),
        PortProbeResult::OtherApp => format!("OtherApp (body={:?})", body),
    };
    log_startup(&format!("probe_port({}) => {}", addr, label));

    result
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
    let Some(albums_root) = resolve_bootstrap_albums_root() else {
        return Vec::new();
    };
    let Ok(entries) = fs::read_dir(&albums_root) else {
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
            colnect_dir: colnect_dir.to_string_lossy().to_string(),
        });
    }

    out.sort_by(|a, b| a.country.to_lowercase().cmp(&b.country.to_lowercase()));
    out
}

fn current_bootstrap_albums_root() -> Option<String> {
    resolve_bootstrap_albums_root().map(|p| p.to_string_lossy().to_string())
}

fn pick_bootstrap_albums_root_via_dialog() -> Result<Option<String>, String> {
    let title = "Select Albums Root (Albums folder or its parent)";
    let current = current_bootstrap_albums_root().unwrap_or_else(|| ".".to_string());

    if which::which("zenity").is_ok() {
        let output = Command::new("zenity")
            .arg("--file-selection")
            .arg("--directory")
            .arg("--title")
            .arg(title)
            .arg("--filename")
            .arg(format!("{}/", current.trim_end_matches('/')))
            .output()
            .map_err(|e| format!("failed to launch zenity: {}", e))?;

        if output.status.success() {
            let selected = String::from_utf8_lossy(&output.stdout).trim().to_string();
            return if selected.is_empty() {
                Ok(None)
            } else {
                Ok(Some(selected))
            };
        }

        if output.status.code() == Some(1) {
            return Ok(None);
        }

        return Err(format!(
            "zenity failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    if which::which("kdialog").is_ok() {
        let output = Command::new("kdialog")
            .arg("--getexistingdirectory")
            .arg(&current)
            .arg("--title")
            .arg(title)
            .output()
            .map_err(|e| format!("failed to launch kdialog: {}", e))?;

        if output.status.success() {
            let selected = String::from_utf8_lossy(&output.stdout).trim().to_string();
            return if selected.is_empty() {
                Ok(None)
            } else {
                Ok(Some(selected))
            };
        }

        if output.status.code() == Some(1) {
            return Ok(None);
        }

        return Err(format!(
            "kdialog failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    Err("No supported folder picker found (install zenity or kdialog)".to_string())
}

fn pick_and_copy_colnect_file_via_dialog(country: &str) -> BootstrapPickColnectFileResponse {
    let fail = |msg: String| BootstrapPickColnectFileResponse {
        ok: false,
        picked: false,
        message: msg,
        file_name: None,
        dest_path: None,
        colnect_dir: None,
    };

    let countries = scan_bootstrap_countries();
    let info = match countries.iter().find(|c| c.country == country) {
        Some(i) => i.clone(),
        None => return fail(format!("Country '{}' not found", country)),
    };

    let colnect_dir = PathBuf::from(&info.colnect_dir);
    let colnect_dir_str = colnect_dir.to_string_lossy().to_string();
    let title = format!("Select Colnect CSV for {}", country);
    pick_and_copy_colnect_file_to_dir(&colnect_dir, &title, &colnect_dir_str, fail)
}

fn pick_albumeasy_cli_path_via_dialog(
    current_path: Option<&str>,
) -> Result<Option<String>, String> {
    let title = "Select AlbumEasy CLI executable";
    let start = current_path
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .unwrap_or("/");

    if which::which("zenity").is_ok() {
        let output = Command::new("zenity")
            .arg("--file-selection")
            .arg("--title")
            .arg(title)
            .arg("--filename")
            .arg(start)
            .output()
            .map_err(|e| format!("failed to launch zenity: {}", e))?;

        if output.status.success() {
            let selected = String::from_utf8_lossy(&output.stdout).trim().to_string();
            return Ok(if selected.is_empty() {
                None
            } else {
                Some(selected)
            });
        }

        if output.status.code() == Some(1) {
            return Ok(None);
        }

        return Err(format!(
            "zenity failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    if which::which("kdialog").is_ok() {
        let output = Command::new("kdialog")
            .arg("--getopenfilename")
            .arg(start)
            .arg("*")
            .arg("--title")
            .arg(title)
            .output()
            .map_err(|e| format!("failed to launch kdialog: {}", e))?;

        if output.status.success() {
            let selected = String::from_utf8_lossy(&output.stdout).trim().to_string();
            return Ok(if selected.is_empty() {
                None
            } else {
                Some(selected)
            });
        }

        if output.status.code() == Some(1) {
            return Ok(None);
        }

        return Err(format!(
            "kdialog failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    Err("No supported file picker found (install zenity or kdialog)".to_string())
}

fn pick_and_copy_colnect_file_for_db(db_path: &str) -> BootstrapPickColnectFileResponse {
    let fail = |msg: String| BootstrapPickColnectFileResponse {
        ok: false,
        picked: false,
        message: msg,
        file_name: None,
        dest_path: None,
        colnect_dir: None,
    };

    let colnect_dir = Path::new(db_path)
        .parent()
        .map(|p| p.join("colnect"))
        .unwrap_or_else(|| PathBuf::from("colnect"));
    let colnect_dir_str = colnect_dir.to_string_lossy().to_string();
    let title = "Select Colnect CSV for current country".to_string();
    pick_and_copy_colnect_file_to_dir(&colnect_dir, &title, &colnect_dir_str, fail)
}

fn pick_and_copy_colnect_file_to_dir<F>(
    colnect_dir: &Path,
    title: &str,
    colnect_dir_str: &str,
    fail: F,
) -> BootstrapPickColnectFileResponse
where
    F: Fn(String) -> BootstrapPickColnectFileResponse,
{
    if !colnect_dir.exists() {
        if let Err(e) = fs::create_dir_all(colnect_dir) {
            return fail(format!("Could not create colnect dir: {}", e));
        }
    }
    let starting = format!("{}/", colnect_dir_str.trim_end_matches('/'));

    let pick_result: Result<Option<String>, String> = (|| {
        if which::which("zenity").is_ok() {
            let output = Command::new("zenity")
                .arg("--file-selection")
                .arg("--title")
                .arg(&title)
                .arg("--filename")
                .arg(&starting)
                .arg("--file-filter")
                .arg("Colnect CSV | *.csv")
                .output()
                .map_err(|e| format!("failed to launch zenity: {}", e))?;

            if output.status.success() {
                let selected = String::from_utf8_lossy(&output.stdout).trim().to_string();
                return Ok(if selected.is_empty() {
                    None
                } else {
                    Some(selected)
                });
            }
            if output.status.code() == Some(1) {
                return Ok(None);
            }
            return Err(format!(
                "zenity failed: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }

        if which::which("kdialog").is_ok() {
            let output = Command::new("kdialog")
                .arg("--getopenfilename")
                .arg(&starting)
                .arg("*.csv")
                .arg("--title")
                .arg(&title)
                .output()
                .map_err(|e| format!("failed to launch kdialog: {}", e))?;

            if output.status.success() {
                let selected = String::from_utf8_lossy(&output.stdout).trim().to_string();
                return Ok(if selected.is_empty() {
                    None
                } else {
                    Some(selected)
                });
            }
            if output.status.code() == Some(1) {
                return Ok(None);
            }
            return Err(format!(
                "kdialog failed: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }

        Err("No supported file picker found (install zenity or kdialog)".to_string())
    })();

    match pick_result {
        Err(e) => fail(e),
        Ok(None) => BootstrapPickColnectFileResponse {
            ok: true,
            picked: false,
            message: "Cancelled".to_string(),
            file_name: None,
            dest_path: None,
            colnect_dir: Some(colnect_dir_str.to_string()),
        },
        Ok(Some(src_path)) => {
            let src = PathBuf::from(&src_path);
            let file_name = src
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "file.csv".to_string());
            let dest = colnect_dir.join(&file_name);
            match fs::copy(&src, &dest) {
                Ok(_) => BootstrapPickColnectFileResponse {
                    ok: true,
                    picked: true,
                    message: format!("Copied '{}' to colnect dir", file_name),
                    file_name: Some(file_name),
                    dest_path: Some(dest.to_string_lossy().to_string()),
                    colnect_dir: Some(colnect_dir_str.to_string()),
                },
                Err(e) => fail(format!("Copy failed: {}", e)),
            }
        }
    }
}

fn set_bootstrap_albums_root_override(path: &str) -> Result<Option<String>, String> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        ensure_runtime_startup_files()?;
        save_runtime_bootstrap_albums_root_path(None)?;
        let mut guard = bootstrap_albums_root_override()
            .lock()
            .map_err(|_| "failed to lock albums root override state".to_string())?;
        *guard = None;
        return Ok(current_bootstrap_albums_root());
    }

    let candidate = PathBuf::from(trimmed);
    let resolved = if let Some(path) = normalize_albums_root_candidate(&candidate) {
        path
    } else if candidate.exists() && candidate.is_dir() && is_directory_empty(&candidate)? {
        // For a brand-new empty folder, create an Albums root inside it so
        // startup behavior matches first-run expectations.
        candidate.join("Albums")
    } else {
        return Err(
            "Path must point to an Albums directory, a folder containing Albums, or an empty folder"
                .to_string(),
        );
    };

    if !resolved.exists() {
        fs::create_dir_all(&resolved)
            .map_err(|e| format!("failed to create Albums root {}: {}", resolved.display(), e))?;
    }

    if !resolved.exists() || !resolved.is_dir() {
        return Err(format!(
            "Albums path does not exist or is not a directory: {}",
            resolved.display()
        ));
    }

    ensure_runtime_startup_files()?;
    save_runtime_bootstrap_albums_root_path(Some(&resolved.to_string_lossy()))?;

    let mut guard = bootstrap_albums_root_override()
        .lock()
        .map_err(|_| "failed to lock albums root override state".to_string())?;
    *guard = Some(resolved.clone());
    Ok(Some(resolved.to_string_lossy().to_string()))
}

fn resolve_bootstrap_albums_root() -> Option<PathBuf> {
    if let Ok(guard) = bootstrap_albums_root_override().lock() {
        if let Some(path) = guard.as_ref() {
            if path.exists() && path.is_dir() {
                return Some(path.clone());
            }
        }
    }

    if let Some(persisted) = crate::config::load_config("config.toml")
        .effective_bootstrap_albums_root_path()
        .map(PathBuf::from)
    {
        if let Some(albums_dir) = normalize_albums_root_candidate(&persisted) {
            if let Ok(mut guard) = bootstrap_albums_root_override().lock() {
                *guard = Some(albums_dir.clone());
            }
            return Some(albums_dir);
        }
    }

    if let Ok(mut dir) = std::env::current_dir() {
        for _ in 0..8 {
            if let Some(albums_dir) = normalize_albums_root_candidate(&dir) {
                return Some(albums_dir);
            }
            if !dir.pop() {
                break;
            }
        }
    }

    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(mut dir) = exe_path.parent().map(|p| p.to_path_buf()) {
            for _ in 0..8 {
                if let Some(albums_dir) = normalize_albums_root_candidate(&dir) {
                    return Some(albums_dir);
                }
                if !dir.pop() {
                    break;
                }
            }
        }
    }

    None
}

fn normalize_albums_root_candidate(path: &Path) -> Option<PathBuf> {
    if !path.exists() || !path.is_dir() {
        return None;
    }

    let direct = path.join("Albums");
    if direct.exists() && direct.is_dir() {
        return Some(direct);
    }

    if path
        .file_name()
        .and_then(|n| n.to_str())
        .map(|name| name.eq_ignore_ascii_case("albums"))
        .unwrap_or(false)
    {
        return Some(path.to_path_buf());
    }

    None
}

fn resolve_image_abs(db_path: &str, album_name: &str, rel_image_path: &str) -> String {
    let rel = rel_image_path.trim();
    if rel.is_empty() {
        return String::new();
    }

    let p1 = crate::file_utils::resolve_album_image_path(album_name, rel);
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
    default_sort_mode: &str,
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
    catalog_family_options: &[String],
    show_imageless_left_rows: bool,
) -> Result<MatcherPayload, Box<dyn std::error::Error>> {
    let left = load_left_stamps(conn, db_path, album_name, show_imageless_left_rows)?;
    let right = load_right_candidates(conn, db_path, album_name)?;

    Ok(MatcherPayload {
        left,
        right,
        album_name: album_name.to_string(),
        colnect_dir: album_dir_for_ui(db_path, album_name)
            .join("colnect")
            .to_string_lossy()
            .to_string(),
        colnect_file_count: count_colnect_csv_files_rec(
            &album_dir_for_ui(db_path, album_name).join("colnect"),
        ),
        show_imageless_left_rows,
        default_catalog_code: default_catalog_code.to_string(),
        default_sort_mode: default_sort_mode.to_string(),
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
        catalog_family_options: catalog_family_options.to_vec(),
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

/// Open `path` in the OS file manager. Returns `true` if a command was
/// successfully spawned. Works on Linux (xdg-open / gio / nautilus) and
/// Windows (explorer.exe).
pub(super) fn open_folder_in_file_manager(path: &str) -> bool {
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(path)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .is_ok()
    }
    #[cfg(not(target_os = "windows"))]
    {
        for prog in &[
            "xdg-open",
            "/usr/bin/xdg-open",
            "/bin/xdg-open",
            "gio",
            "/usr/bin/gio",
        ] {
            let args: &[&str] = if prog.ends_with("gio") {
                &["open", path]
            } else {
                &[path]
            };
            let ok = Command::new(prog)
                .args(args)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .is_ok();
            if ok {
                return true;
            }
        }
        false
    }
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
            stamps_skipped: 0,
            errors: vec!["No configured path and no album path available".to_string()],
        };
    }

    // Find CSV files in the first candidate folder that actually contains CSV files.
    let mut csv_files = Vec::new();
    let mut errors = Vec::new();
    let mut candidate_scan_errors = Vec::new();
    let mut checked_dirs: Vec<String> = Vec::new();
    let mut used_inbox_dir: Option<PathBuf> = None;

    for inbox_dir in inbox_candidates {
        checked_dirs.push(inbox_dir.display().to_string());

        if !inbox_dir.exists() {
            candidate_scan_errors.push(format!("Directory not found: {}", inbox_dir.display()));
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
                            candidate_scan_errors.push(format!(
                                "Failed to read directory entry in {}: {}",
                                inbox_dir.display(),
                                e
                            ));
                        }
                    }
                }
            }
            Err(e) => {
                candidate_scan_errors.push(format!(
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
        errors.extend(candidate_scan_errors);
        errors.push(format!("Checked folders: {}", checked_dirs.join(" | ")));
        return ColnectImportResponse {
            ok: true,
            message: "No CSV files found in colnect inbox".to_string(),
            files_processed: 0,
            stamps_imported: 0,
            stamps_skipped: 0,
            errors,
        };
    }

    if let Some(dir) = used_inbox_dir {
        log::info!("Colnect import using inbox folder: {}", dir.display());
    }

    // Parse and ingest CSV files
    let mut total_stamps_imported = 0usize;
    let mut total_stamps_skipped = 0usize;
    let mut files_processed = 0usize;

    for csv_file in &csv_files {
        match crate::colnect::parser::parse_csv(csv_file) {
            Ok(stamps) => {
                let count = stamps.len();
                let filename = csv_file
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| csv_file.display().to_string());
                log_runtime_event(&format!("Parsing {}: {} stamps found", filename, count));
                let before: usize = conn
                    .query_row("SELECT COUNT(*) FROM colnect_stamps", [], |r| {
                        r.get::<_, i64>(0)
                    })
                    .unwrap_or(0) as usize;
                match crate::colnect::ingest::ingest_colnect_data_with_mode(
                    conn, &stamps, "sg",
                    false, // Don't clear existing - we're adding to what's already there
                ) {
                    Ok(_) => {
                        let after: usize = conn
                            .query_row("SELECT COUNT(*) FROM colnect_stamps", [], |r| {
                                r.get::<_, i64>(0)
                            })
                            .unwrap_or(0) as usize;
                        let new_count = after.saturating_sub(before);
                        let dup_count = count.saturating_sub(new_count);
                        files_processed += 1;
                        total_stamps_imported += new_count;
                        total_stamps_skipped += dup_count;
                        let status_msg = if dup_count == 0 {
                            format!("{}: {} new stamps imported", filename, new_count)
                        } else if new_count == 0 {
                            format!("{}: {} assessed — all already in database", filename, count)
                        } else {
                            format!(
                                "{}: {} new imported, {} already in database",
                                filename, new_count, dup_count
                            )
                        };
                        log_runtime_event(&status_msg);
                        log::info!("{}", status_msg);
                    }
                    Err(e) => {
                        errors.push(format!("Failed to ingest {}: {}", csv_file.display(), e));
                        log_runtime_event(&format!(
                            "Failed to ingest {}: {}",
                            csv_file.display(),
                            e
                        ));
                    }
                }
            }
            Err(e) => {
                errors.push(format!("Failed to parse {}: {}", csv_file.display(), e));
                log_runtime_event(&format!("Failed to parse {}: {}", csv_file.display(), e));
            }
        }
    }

    let summary_message = if total_stamps_skipped > 0 && total_stamps_imported == 0 {
        format!(
            "{} assessed across {} file(s) — all already in database",
            total_stamps_skipped, files_processed
        )
    } else if total_stamps_skipped > 0 {
        format!(
            "Imported {} new stamps from {} file(s); {} already in database",
            total_stamps_imported, files_processed, total_stamps_skipped
        )
    } else {
        format!(
            "Imported {} stamps from {} CSV file(s)",
            total_stamps_imported, files_processed
        )
    };

    ColnectImportResponse {
        ok: errors.is_empty(),
        message: summary_message,
        files_processed,
        stamps_imported: total_stamps_imported,
        stamps_skipped: total_stamps_skipped,
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
    log_runtime_event("Review UI: starting startup selector...");

    let (server, bind) = match bind_review_server(bind_addr_opt) {
        Ok(pair) => pair,
        Err(e) => {
            if let Some(url) = already_running_url(&e) {
                log_runtime_event(&format!(
                    "Review UI: instance already running at {} — opening browser",
                    url
                ));
                if open_browser {
                    open_review_ui_url(&url);
                }
                // Nothing more to do; caller treats this as a clean handoff.
                return Ok(ReviewUiBootstrapOutcome::AlreadyRunning);
            }
            return Err(e);
        }
    };

    let html = render_bootstrap_html(default_inbox_path);
    let url = format!("http://{}/", bind);
    log_runtime_event(&format!("Review UI Setup: {}", url));
    if open_browser {
        open_review_ui_url(&url);
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
    default_sort_mode: &str,
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
    catalog_family_options: &[String],
) -> Result<ReviewUiRunOutcome, Box<dyn std::error::Error>> {
    if let Some(albums_root) = Path::new(db_path)
        .parent()
        .and_then(|album_dir| album_dir.parent())
    {
        std::env::set_var("STAMP_ALBUMS_ROOT", albums_root);
        log_runtime_event(&format!(
            "Review UI: using albums root {}",
            albums_root.display()
        ));
    }

    log_runtime_event(&format!("Review UI: preparing data from {}", db_path));
    log_runtime_event("Review UI: opening database...");
    let conn = crate::db_utils::init_db(db_path).map_err(|e| {
        log_runtime_event(&format!(
            "Review UI: failed to open database {}: {}",
            db_path, e
        ));
        std::io::Error::other(format!("failed to open review database {}: {}", db_path, e))
    })?;
    let _ = conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_stamps_review_sg_year ON stamps (sg_number, year)",
        [],
    );
    let _ = conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_colnect_review_queue ON colnect_stamps (is_deleted, is_processed, issued_on, sg_number)",
        [],
    );
    log_runtime_event("Review UI: syncing processed Colnect candidates...");
    sync_processed_colnect_candidates(&conn).map_err(|e| {
        log_runtime_event(&format!(
            "Review UI: failed while syncing processed Colnect candidates: {}",
            e
        ));
        std::io::Error::other(format!(
            "failed to sync processed Colnect candidates: {}",
            e
        ))
    })?;

    let album_name = album_name_opt
        .map(|s| s.to_string())
        .or_else(|| {
            Path::new(db_path)
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
        })
        .unwrap_or_else(|| "Album".to_string());

    let initial_show_imageless_left_rows = false;
    log_runtime_event(&format!(
        "Review UI: loading left StampWorld rows (show image-less: {})...",
        initial_show_imageless_left_rows
    ));
    let payload = build_matcher_payload(
        &conn,
        db_path,
        &album_name,
        default_catalog_code,
        default_sort_mode,
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
        catalog_family_options,
        initial_show_imageless_left_rows,
    )
    .map_err(|e| {
        log_runtime_event(&format!(
            "Review UI: failed to build compare payload for {}: {}",
            db_path, e
        ));
        std::io::Error::other(format!(
            "failed to prepare review compare payload for {}: {}",
            db_path, e
        ))
    })?;
    log_runtime_event(&format!(
        "Review UI: loaded {} left rows and {} right candidates",
        payload.left.len(),
        payload.right.len()
    ));

    let right_map: HashMap<String, RightCandidate> = payload
        .right
        .iter()
        .cloned()
        .map(|c| (c.colnect_id.clone(), c))
        .collect();

    let html = render_html(&payload);

    log_runtime_event("Review UI: starting local web server...");
    let (server, bind) = if let Some(pb) = prebound_server {
        pb
    } else {
        match bind_review_server(bind_addr_opt) {
            Ok(pair) => pair,
            Err(e) => {
                if let Some(url) = already_running_url(&e) {
                    log_runtime_event(&format!(
                        "Review UI: instance already running at {} — opening browser",
                        url
                    ));
                    if open_browser {
                        open_review_ui_url(&url);
                    }
                    return Err(std::io::Error::other(format!(
                        "Review UI already running at {}",
                        url
                    ))
                    .into());
                }
                log_runtime_event(&format!(
                    "Review UI: failed to bind local web server: {}",
                    e
                ));
                return Err(std::io::Error::other(format!(
                    "failed to bind review UI server: {}",
                    e
                ))
                .into());
            }
        }
    };

    let url = format!("http://{}/", bind);
    log_runtime_event(&format!("Review UI: {}", url));
    if open_browser {
        log_runtime_event("Review UI: opening browser...");
        open_review_ui_url(&url);
    } else {
        log_runtime_event("Review UI: waiting for bootstrap tab to load compare view...");
    }

    let review_state = serve_review_requests(
        &server,
        &conn,
        db_path,
        &album_name,
        &html,
        &right_map,
        default_catalog_code,
        default_sort_mode,
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
        catalog_family_options,
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
