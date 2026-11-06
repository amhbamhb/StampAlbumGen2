use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fs;
use std::io::IsTerminal;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
// removed OpenOptions and SystemTime/UNIX_EPOCH now that persistent debug file is unused
use std::env;
#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::process::Command;

// `rfd` is optional; if the `rfd` feature is enabled we'll use a GUI
// folder picker and a Yes/No confirmation dialog, otherwise fall back
// to a console prompt (only when a terminal is present).
#[cfg(feature = "rfd")]
use rfd::{FileDialog, MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};

// Native-only layout settings are validated by native_pdf::config, not legacy generation.
// Keep their per-key template comments: the config editor displays them as field help.
const EMBEDDED_CONFIG_TEMPLATE: &str = include_str!("../config_template.toml");
const EMBEDDED_PDF_HELP: &[u8] = include_bytes!("../StampAlbumGenerator2HelpManual.pdf");

thread_local! {
    static SCOPED_ALBUM_CONFIG_DIR: RefCell<Option<PathBuf>> = const { RefCell::new(None) };
}

pub fn embedded_config_template() -> &'static str {
    EMBEDDED_CONFIG_TEMPLATE
}
pub fn embedded_pdf_help() -> &'static [u8] {
    EMBEDDED_PDF_HELP
}
pub fn load_pdf_content_with_fallback(
    pdf_path: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    match fs::read(pdf_path) {
        Ok(content) => Ok(content),
        Err(_) => Ok(EMBEDDED_PDF_HELP.to_vec()),
    }
}

pub fn load_template_content_with_fallback(
    template_path: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    match fs::read_to_string(template_path) {
        Ok(content) => Ok(content),
        Err(_) => Ok(EMBEDDED_CONFIG_TEMPLATE.to_string()),
    }
}

pub fn ensure_config_and_template_files(
    config_path: &str,
    template_path: &str,
    pdf_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Debug: indicate whether the optional GUI folder-picker feature is compiled in
    // debug!(
    //     "ensure_config_and_template_files: rfd feature compiled = {}",
    //     cfg!(feature = "rfd")
    // );
    // Use structured logging (debug) instead of a persistent debug file.
    // debug!("rfd_compiled={}", cfg!(feature = "rfd"));
    // Determine working directory and allow the user to move/copy the
    // running binary to a different folder on first-run when this
    // directory appears otherwise empty (no config/template present).
    // Prefer the executable's parent directory (so double-click launches
    // evaluate first-run relative to the exe location) but fall back to
    // the process current directory when the exe path cannot be found.
    let cwd = match std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|pp| pp.to_path_buf()))
    {
        Some(exe_parent) => {
            debug!(
                "Using executable parent dir for first-run check: {}",
                exe_parent.display()
            );
            exe_parent
        }
        None => {
            let d = std::env::current_dir()?;
            debug!(
                "Falling back to current_dir for first-run check: {}",
                d.display()
            );
            d
        }
    };

    // Allow forcing the first-run interactive flow via env var. This is
    // useful when testing GUI double-click flows (set to "1" to force).
    let force_first_run = match std::env::var("STAMP_ALBUM_FORCE_FIRST_RUN") {
        Ok(v) => !(v == "0" || v.to_lowercase() == "false"),
        Err(_) => false,
    };
    if force_first_run {
        debug!("force_first_run enabled via env");
    }

    // If there is no config yet and the current folder appears empty
    // except for the running binary, offer the user an interactive
    // folder picker so they may move/copy the binary and continue there.
    if !Path::new(config_path).exists() || force_first_run {
        // Count visible (non-dot) entries excluding the running executable
        if let Ok(entries) = fs::read_dir(&cwd) {
            let exe_name = std::env::current_exe()
                .ok()
                .and_then(|p| p.file_name().map(|s| s.to_os_string()));

            let mut visible_count = 0usize;
            for e in entries.flatten() {
                let name = e.file_name();
                // Log each visible entry for debugging first-run detection
                debug!("dir-entry: {}", name.to_string_lossy());
                if let Some(ref exe_n) = exe_name {
                    if name == *exe_n {
                        continue;
                    }
                }
                // Ignore startup probe file(s) created earlier in main.rs
                if name.to_string_lossy().starts_with("startup_trace") {
                    debug!("ignoring probe file: {}", name.to_string_lossy());
                    continue;
                }
                if name.to_string_lossy().starts_with('.') {
                    continue;
                }
                visible_count += 1;
            }

            debug!("computed visible_count={}", visible_count);

            if visible_count == 0 {
                debug!("visible_count==0: attempting interactive first-run flow");
                // Try a GUI folder picker when enabled, otherwise ask for a
                // typed path on the console. If the user cancels/skips we
                // continue in the current directory.
                // Prefer GUI folder-picker when available, but fall back to
                // a console prompt if the GUI was not selected or returned
                // no result (user canceled or failure). Emit a diagnostic
                // so we can observe which path was used.
                let mut picked_opt: Option<PathBuf> = None;

                // Detect whether we have an interactive terminal available
                // so we never block on stdin when the user launched the
                // binary from a GUI (double-click/file-manager).
                let has_tty = io::stdin().is_terminal();

                let gui_initiated = cfg!(feature = "rfd");
                #[cfg(feature = "rfd")]
                {
                    let proceed = MessageDialogResult::Ok;
                    if proceed == MessageDialogResult::Ok {
                        match FileDialog::new()
                            .set_title(
                                "Select directory to move/copy binary to (Cancel to keep current)",
                            )
                            .pick_folder()
                        {
                            Some(p) => {
                                println!("GUI picker selected: {}", p.display());
                                debug!("GUI picker selected: {}", p.display());
                                picked_opt = Some(p);
                            }
                            None => {
                                println!("GUI picker returned no selection; skipping interactive move/copy");
                                debug!("GUI picker returned no selection");
                                // keep picked_opt as None; since GUI was used,
                                // avoid falling back to console prompt below.
                            }
                        }
                    } else {
                        debug!("User cancelled folder selection");
                    }
                }
                debug!(
                    "has_tty={} picked_opt.is_some={} gui_initiated={}",
                    has_tty,
                    picked_opt.is_some(),
                    gui_initiated
                );
                // Only fall back to a console prompt when no GUI was invoked
                // (so GUI cancel/close won't unexpectedly block on stdin).
                if picked_opt.is_none() && has_tty && !gui_initiated {
                    print!("Target directory (absolute or relative, blank to skip): ");
                    io::stdout().flush().ok();
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).ok();
                    let input = input.trim();
                    if !input.is_empty() {
                        picked_opt = Some(PathBuf::from(input));
                    }
                } else if picked_opt.is_none() && !has_tty {
                    println!("No terminal available; skipping console prompt");
                    debug!("No terminal available; skipping console prompt");
                }

                if let Some(target) = picked_opt {
                    let target = if target.is_absolute() {
                        target
                    } else {
                        env::current_dir()?.join(target)
                    };
                    println!("Selected target directory: {}", target.display());
                    debug!("Selected target directory: {}", target.display());

                    // Ask whether to move or copy. Prefer GUI dialogs when
                    // compiled with `rfd`; otherwise fall back to the console
                    // prompt. Keep the console loop as a fallback for TTY
                    // sessions so scripts and headless runs continue to work.
                    let exe_path = std::env::current_exe()?;
                    let exe_file_name = exe_path
                        .file_name()
                        .and_then(|s| s.to_str())
                        .ok_or("Failed to determine executable name")?;
                    let dest = target.join(exe_file_name);

                    #[cfg(feature = "rfd")]
                    {
                        // Use a two-step dialog to offer Move / Copy / Skip.
                        // First ask whether to Move; if No, ask whether to Copy.
                        let do_move = MessageDialog::new()
                            .set_title("Move binary to target?")
                            .set_description(format!(
                                "Move the running binary to {}?\n\nChoose Yes to move, No to consider copying, Cancel to skip.",
                                target.display()
                            ))
                            .set_buttons(MessageButtons::YesNo)
                            .set_level(MessageLevel::Warning)
                            .show();

                        debug!("GUI move dialog result: {}", do_move);
                        if do_move == MessageDialogResult::Yes {
                            match fs::rename(&exe_path, &dest) {
                                Ok(()) => info!("Moved executable to {}", dest.display()),
                                Err(e) => {
                                    fs::copy(&exe_path, &dest)?;
                                    info!(
                                        "Rename failed ({}); copied executable to {}",
                                        e,
                                        dest.display()
                                    );
                                }
                            }
                            // Ensure template/config exist in the target to avoid
                            // re-triggering first-run after exec in an empty dir.
                            let target_template = target.join(template_path);
                            let target_config = target.join(config_path);
                            let target_pdf = target.join(pdf_path);
                            if !target_template.exists()
                                || std::fs::read_to_string(&target_template).ok().as_deref()
                                    != Some(EMBEDDED_CONFIG_TEMPLATE)
                            {
                                fs::write(&target_template, EMBEDDED_CONFIG_TEMPLATE)?;
                            }
                            if !target_config.exists() {
                                fs::copy(&target_template, &target_config)?;
                            }
                            if !target_pdf.exists() {
                                fs::write(&target_pdf, EMBEDDED_PDF_HELP)?;
                            }
                            env::set_current_dir(&target)?;
                            #[cfg(unix)]
                            {
                                let err = Command::new(&dest).args(env::args_os().skip(1)).exec();
                                return Err(Box::new(err));
                            }
                        } else {
                            // User selected No in the first dialog — ask about
                            // copying. If they decline again, treat as Skip.
                            let do_copy = MessageDialog::new()
                                .set_title("Copy binary to target?")
                                .set_description(format!(
                                    "Copy the running binary to {}?\n\nChoose Yes to copy, No/Cancel to skip.",
                                    target.display()
                                ))
                                .set_buttons(MessageButtons::YesNo)
                                .set_level(MessageLevel::Info)
                                .show();
                            debug!("GUI copy dialog result: {}", do_copy);
                            if do_copy == MessageDialogResult::Yes {
                                fs::copy(&exe_path, &dest)?;
                                info!("Copied executable to {}", dest.display());
                                // Write template/config into the target to avoid
                                // the new process re-prompting on empty dir.
                                let target_template = target.join(template_path);
                                let target_config = target.join(config_path);
                                let target_pdf: PathBuf = target.join(pdf_path);

                                if !target_template.exists()
                                    || std::fs::read_to_string(&target_template).ok().as_deref()
                                        != Some(EMBEDDED_CONFIG_TEMPLATE)
                                {
                                    fs::write(&target_template, EMBEDDED_CONFIG_TEMPLATE)?;
                                }
                                if !target_config.exists() {
                                    fs::copy(&target_template, &target_config)?;
                                }
                                if !target_pdf.exists() {
                                    fs::write(&target_pdf, EMBEDDED_PDF_HELP)?;
                                }

                                env::set_current_dir(&target)?;
                                #[cfg(unix)]
                                {
                                    let err =
                                        Command::new(&dest).args(env::args_os().skip(1)).exec();
                                    return Err(Box::new(err));
                                }
                            } else {
                                debug!("User chose to skip move/copy via GUI dialogs");
                            }
                        }
                    }

                    #[cfg(not(feature = "rfd"))]
                    {
                        // Console fallback for headful TTY sessions.
                        loop {
                            print!("Move binary to target (m), Copy (c), or Skip (s)? [m/c/s]: ");
                            io::stdout().flush().ok();
                            let mut choice = String::new();
                            if io::stdin().read_line(&mut choice)? == 0 {
                                break;
                            }
                            let choice = choice.trim().to_lowercase();

                            if choice == "m" {
                                match fs::rename(&exe_path, &dest) {
                                    Ok(()) => info!("Moved executable to {}", dest.display()),
                                    Err(e) => {
                                        fs::copy(&exe_path, &dest)?;
                                        info!(
                                            "Rename failed ({}); copied executable to {}",
                                            e,
                                            dest.display()
                                        );
                                    }
                                }
                                ensure_default_files(
                                    &target.join(config_path),
                                    &target.join(template_path),
                                    &target.join(pdf_path),
                                )?;
                                env::set_current_dir(&target)?;
                                #[cfg(unix)]
                                {
                                    let err =
                                        Command::new(&dest).args(env::args_os().skip(1)).exec();
                                    return Err(Box::new(err));
                                }
                                #[cfg(not(unix))]
                                {
                                    break;
                                }
                            } else if choice == "c" {
                                fs::copy(&exe_path, &dest)?;
                                info!("Copied executable to {}", dest.display());
                                ensure_default_files(
                                    &target.join(config_path),
                                    &target.join(template_path),
                                    &target.join(pdf_path),
                                )?;
                                env::set_current_dir(&target)?;
                                #[cfg(unix)]
                                {
                                    let err =
                                        Command::new(&dest).args(env::args_os().skip(1)).exec();
                                    return Err(Box::new(err));
                                }
                                #[cfg(not(unix))]
                                {
                                    break;
                                }
                            } else if choice == "s" {
                                break;
                            } else {
                                println!("Please enter 'm', 'c' or 's'.");
                                continue;
                            }
                        }
                    }
                }
            }
        }
    }
    // Compute absolute paths relative to the current working directory
    // (which may have been changed by a move/copy above). This avoids
    // relying on the earlier `cwd` variable which represented the
    // executable parent dir at process start.
    let current_dir = env::current_dir()?;
    let config_abs: PathBuf = if Path::new(config_path).is_absolute() {
        PathBuf::from(config_path)
    } else {
        current_dir.join(config_path)
    };
    let template_abs: PathBuf = if Path::new(template_path).is_absolute() {
        PathBuf::from(template_path)
    } else {
        current_dir.join(template_path)
    };
    let pdf_abs: PathBuf = if Path::new(pdf_path).is_absolute() {
        PathBuf::from(pdf_path)
    } else {
        current_dir.join(pdf_path)
    };

    ensure_default_files(&config_abs, &template_abs, &pdf_abs)
}

fn ensure_default_files(
    config_abs: &Path,
    template_abs: &Path,
    pdf_abs: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Always keep config_template.toml in sync with the embedded defaults so
    // that users can see newly added options after upgrading. This file is
    // purely a reference; it is never user-edited.
    if !template_abs.exists()
        || std::fs::read_to_string(template_abs).ok().as_deref() != Some(EMBEDDED_CONFIG_TEMPLATE)
    {
        fs::write(template_abs, EMBEDDED_CONFIG_TEMPLATE)?;
        log::info!(
            "Updated config template from embedded defaults at {}",
            template_abs.display()
        );
    }

    if !config_abs.exists() {
        if template_abs.exists() {
            fs::copy(template_abs, config_abs)?;
            log::info!(
                "Created missing config file from template at {}",
                config_abs.display()
            );
        } else {
            fs::write(config_abs, EMBEDDED_CONFIG_TEMPLATE)?;
            log::info!(
                "Created missing config file from embedded template at {}",
                config_abs.display()
            );
        }
    }

    // Keep the installed manual synchronized with the version embedded in the
    // executable so missing or outdated manuals are restored on startup.
    if fs::read(pdf_abs).ok().as_deref() != Some(EMBEDDED_PDF_HELP) {
        fs::write(pdf_abs, EMBEDDED_PDF_HELP)?;
        log::info!(
            "Updated manual PDF from embedded version at {}",
            pdf_abs.display()
        );
    }

    Ok(())
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HeaderFormatting {
    pub date_format: Option<String>,
    /// Optional date pattern for stamp-group headers. Supported tokens are
    /// `{year}`, `{month}`, `{month_num}`, and `{day}`. When set, this takes
    /// precedence over the legacy `date_format` style selector.
    pub date_pattern: Option<String>,
    pub label_style: Option<String>,
    pub series_style: Option<String>,
    pub include_currency: Option<bool>,
    // Optional toggles that may be placed inside the [header_formatting]
    // table in older or alternate config layouts. We accept these here
    // and propagate them up into the top-level `Config` so either
    // placement works.
    pub include_design: Option<bool>,
    pub include_engraving: Option<bool>,
    pub include_watermark: Option<bool>,
    pub include_perforation: Option<bool>,
    pub include_catalog_numbers: Option<bool>,
    pub include_series: Option<bool>,
    pub include_country: Option<bool>,
    pub include_date: Option<bool>,
    // Stamp-row side/center value visibility toggles
    pub include_denomination: Option<bool>,
    pub include_stamps_issued: Option<bool>,
    pub include_description: Option<bool>,
    pub include_color: Option<bool>,
    pub include_mint_unused: Option<bool>,
    pub include_used: Option<bool>,
    pub print_flag_cover: Option<bool>,
    pub header_spacing: Option<f32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Config {
    pub reference_width_mm: Option<f32>,
    pub reference_height_mm: Option<f32>,
    /// Maximum stamp height (mm) used when sizing_mode is "fit_box".
    pub target_stamp_height_mm: Option<f32>,
    pub stamp_box_adjust: Option<f32>,
    // Page layout overrides (mm)
    pub album_page_width_mm: Option<f32>,
    pub album_page_height_mm: Option<f32>,
    // Album title/header font and size (AlbumEasy font codes)
    pub album_pages_title_font: Option<String>,
    pub album_pages_title_size: Option<f32>,
    // Header position: L, R, C, LR, RL
    pub album_pages_header_position: Option<String>,
    pub page_top_margin_mm: Option<f32>,
    pub page_bottom_margin_mm: Option<f32>,
    pub page_left_margin_mm: Option<f32>,
    pub page_right_margin_mm: Option<f32>,
    pub page_border_outer: Option<f32>,
    pub page_border_inner1: Option<f32>,
    pub page_border_inner2: Option<f32>,
    pub page_border_spacing: Option<f32>,
    pub album_pages_spacing_h: Option<f32>,
    pub album_pages_spacing_v: Option<f32>,
    pub median_width_cap_3: Option<f32>,
    pub median_width_cap_4: Option<f32>,
    pub row_font_spacing: Option<f32>,
    pub global_scale: Option<f32>,
    pub header_font_spacing: Option<f32>,
    pub header_formatting: Option<HeaderFormatting>,
    // Footer/page number controls
    pub album_pages_footer_number_enabled: Option<bool>,
    pub album_pages_footer_number_font: Option<String>,
    pub album_pages_footer_number_size: Option<f32>,
    pub album_pages_footer_number_position: Option<String>,
    pub album_pages_footer_number_start: Option<i32>,
    pub album_pages_footer_number_before: Option<String>,
    pub album_pages_footer_number_after: Option<String>,
    // When true, emit detailed per-stamp PAGE_FIT diagnostics into generated
    // AlbumEasy text output to help debug page-fit math (disabled by default).
    pub detailed_page_fit_diagnostics: Option<bool>,
    // When set to Some("placeholders"), grouped-range tokens like "1067-1068"
    // encountered on StampWorld pages will be expanded into individual
    // placeholder DB rows (fast). When set to Some("detail_fetch") the
    // collector may attempt to visit individual stamp detail pages to
    // populate full metadata (slower, not implemented here yet).
    pub expand_grouped_ranges: Option<String>,
    // Conservative buffer reserved at bottom of page (mm) to avoid footer bleed
    pub page_fit_conservative_buffer_mm: Option<f32>,
    // Optional path to an offline calibration CSV file. CSV format:
    // filename,mm_per_px_w,mm_per_px_h
    pub calibration_csv: Option<String>,
    // Optional toggles to include certain header fields when true
    pub include_design: Option<bool>,
    pub include_engraving: Option<bool>,
    pub include_watermark: Option<bool>,
    pub include_perforation: Option<bool>,
    pub include_catalog_numbers: Option<bool>,
    pub include_series: Option<bool>,
    pub include_country: Option<bool>,
    pub include_date: Option<bool>,
    // Stamp-row side/center value visibility toggles
    pub include_denomination: Option<bool>,
    pub include_stamps_issued: Option<bool>,
    pub include_description: Option<bool>,
    pub include_color: Option<bool>,
    pub include_mint_unused: Option<bool>,
    pub include_used: Option<bool>,
    // Stamp detail layout mode: "inline" (default), "separate_rows", "minimal"
    // - inline: All details inside stamp boxes (current behavior, can be cluttered)
    // - separate_rows: Clean stamps on first row, organized details below each stamp
    // - minimal: Only essential info (face value) in stamps, no detail rows
    pub stamp_detail_layout: Option<String>,
    // Which Colnect fields to include in detail output (comma-separated)
    // Options: print_run, themes, printing, paper, gum, designers, printers, colors, description
    // Default: "print_run,themes,designers" for concise output
    pub colnect_detail_fields: Option<String>,
    // Dedicated Colnect detail-output controls. If present, this section
    // takes precedence over legacy top-level Colnect detail keys.
    pub colnect: Option<ColnectConfig>,
    // Dedicated StampWorld scraper and auth controls.
    pub stampworld: Option<StampworldConfig>,
    // Review UI visual controls.
    pub review_ui: Option<ReviewUiConfig>,
    // Font size for stamp details in separate_rows mode (e.g., 6, 7, 8)
    // Default: 8
    pub stamp_detail_font_size: Option<u8>,
    // Vertical space before and after stamp details in separate_rows mode
    // Default: 2.0
    pub stamp_detail_vspace: Option<f64>,
    // Allow overriding maximum rows per page when header is larger
    pub max_rows_per_page: Option<usize>,
    // Maximum PAGE_VSPACE (mm) the centering postprocessor is allowed to emit.
    // Helps avoid excessive whitespace on sparse pages.
    pub max_page_vspace_mm: Option<f32>,
    // When false, don't emit ROW_START_ES automatically after PAGE_START
    // Default: true
    pub row_start_es_enabled: Option<bool>,
    // Configurable ROW_START_ES parameters
    pub row_start_es_font: Option<String>,
    pub row_start_es_size: Option<f32>,
    pub row_start_es_spacing: Option<f32>,
    pub row_start_es_vspace: Option<f32>,
    // Configurable ROW_START_ES parameters (font/size/spacing/vspace)
    // Allow a single very large stamp (or row) to occupy a page if enabled
    // and it exceeds the fraction `single_stamp_threshold_pct` of usable height.
    pub allow_single_large_stamp_per_page: Option<bool>,
    // Fraction (0.0..1.0) above which a single-row page is permitted.
    pub single_stamp_threshold_pct: Option<f32>,
    // Extra padding subtracted from usable height (mm) to avoid bleed at page bottom
    pub usable_height_padding_mm: Option<f32>,
    pub header_vspace_before_mm: Option<f32>,
    pub header_vspace_after_mm: Option<f32>,
    pub text_line_height_mm: Option<f32>,
    pub image_spacing_mm: Option<f32>,
    // Optional per-series inter-row gap (mm). When set, this value will be
    // applied between rows inside a series before distributing any leftover
    // vertical space to top/between-series/bottom gaps. If None, no explicit
    // per-row gap is applied (legacy behavior).
    pub series_row_gap_mm: Option<f32>,
    pub max_stamps_per_row: Option<u32>,
    /// Extra mm added to every stamp box width (applied uniformly, regardless of image source).
    pub extra_width_mm: Option<f32>,
    /// Extra mm added to every stamp box height (applied uniformly, regardless of image source).
    pub extra_height_mm: Option<f32>,
    pub album_stamp_img_new_setting_preserve_aspect: Option<bool>,
    // If false, the generator will skip emitting the large flag/title cover page
    // Default: true (when None or not present)
    pub print_flag_cover: Option<bool>,
    // When true, overwrite existing downloaded flag PNGs from remote sources
    // Default: false
    pub flag_cover_overwrite_flag: Option<bool>,
    // When true, overwrite existing downloaded coat-of-arms images from remote sources
    // Default: false
    pub flag_cover_overwrite_coa: Option<bool>,
    // If true, fetch a short Wikipedia summary for the album/site and save
    // it under the album directory (wiki_summary.txt). Default: false
    pub fetch_wikipedia: Option<bool>,
    // Legacy / alternate name: include_wikipedia_content
    pub include_wikipedia_content: Option<bool>,
    // Controls whether Wikipedia fallback ingest inserts rows into album DB
    // when scraping returns no stamps.
    pub wikipedia_ingest_enable: Option<bool>,
    // Maximum number of placeholder rows inserted by fallback ingest.
    pub wikipedia_ingest_max_entries: Option<usize>,
    // Prefix used for fallback ingest series/context labels.
    pub wikipedia_ingest_series_prefix: Option<String>,
    // Keep-alive alias for preserve aspect ratio flag used in some configs
    pub preserve_aspect: Option<bool>,
    // Alternative alias for aspect ratio preservation (used in some config files)
    pub album_stamp_img_aspect_ratio: Option<bool>,

    // Emit extra diagnostics related to placeholder expansion
    pub emit_placeholder_diagnostics: Option<bool>,

    // When true (default), placeholders generated with explicit mm/mm_per_px
    // mappings will be recreated/overwritten to ensure pixel dimensions match
    // emitted mm box sizes. When false, existing placeholder files will be
    // preserved if their pixel dimensions match the desired size.
    pub placeholder_always_overwrite: Option<bool>,

    // Backwards-compatible alias for detailed_page_fit_diagnostics
    pub detailed_page_fit: Option<bool>,

    // Legacy compatibility flag. Visible headings are always measured for safe pagination.
    pub stamp_heading_in_row_height: Option<bool>,
    // Legacy compatibility flag. Explicit vertical spacing is always measured.
    pub pages_vspace_in_row_height: Option<bool>,

    // Packing safety margins (mm) - used to add small horizontal/vertical
    // safety paddings during packing to avoid visual overlap.
    pub packing_horizontal_safety_mm: Option<f32>,
    pub packing_vertical_safety_mm: Option<f32>,
    // When true, emit a PAGE_HSPACE line before rows to indicate an intended
    // horizontal left offset to center the row. This is a prototype helper
    // for renderers that can honor per-row horizontal offsets. Default: false
    pub enable_row_centering: Option<bool>,
    // When true, emit a compact ROW_META diagnostic comment before each row
    // showing the number of stamps, computed row width and available usable
    // width. This helps tune packing/centering without changing rendering.
    pub show_stamps_per_row: Option<bool>,

    // Color customization keys (string names understood by the generator)
    pub color_album_border: Option<String>,
    pub color_album_footer: Option<String>,
    pub color_album_header: Option<String>,
    pub color_album_title: Option<String>,
    pub color_album_margin_txt: Option<String>,
    pub color_stamp_border: Option<String>,
    pub color_stamp_heading: Option<String>,
    pub color_stamp_text: Option<String>,
    pub color_stamp_background: Option<String>,
    // Flag cover specific settings (unique names prefixed with `flag_cover_`)
    pub flag_cover_img_w_mm: Option<f32>,
    pub flag_cover_img_h_mm: Option<f32>,
    pub flag_cover_page_vspace_top: Option<f32>,
    pub flag_cover_image_vspace_after: Option<f32>,
    // Order and optional elements for the flag cover page. Example in config.toml:
    // flag_cover_elements = ["coat", "flag", "name"]
    // If omitted, the default order is ["coat","flag","name"].
    pub flag_cover_elements: Option<Vec<String>>,
    // Optional size for the coat of arms image on the cover (mm)
    pub flag_cover_coat_img_w_mm: Option<f32>,
    pub flag_cover_coat_img_h_mm: Option<f32>,
    pub flag_cover_image_title_font: Option<String>,
    pub flag_cover_image_title_size: Option<f32>,
    pub flag_cover_image_title_scale: Option<f32>,
    pub flag_cover_text_only_vspace_top: Option<f32>,
    pub flag_cover_text_only_title_font: Option<String>,
    pub flag_cover_text_only_title_size: Option<f32>,
    pub flag_cover_text_only_title_scale: Option<f32>,
    pub flag_cover_text_only_vspace_after: Option<f32>,
    pub flag_cover_background_color: Option<String>,
    // How series are ordered in generated albums. Options:
    // - Some("first_date"): order series by the date of their first stamp (default behavior)
    // - Some("first_sg"): order series by the first stamp's SG number
    // - Some("alpha"): alphabetical series order
    pub series_order: Option<String>,
    // Custom string markers emitted around headers and row starts so tests
    // and downstream tools can reliably parse and identify sections.
    // Default values are chosen to reproduce historic output.
    pub page_vspace_marker: Option<String>,
    pub row_start_es_marker: Option<String>,
    // Optional templates for PAGE_TEXT output. Each template must include
    // a single `{}` placeholder which will be replaced with the text to
    // emit. If omitted the generator falls back to historical literals
    // such as `PAGE_TEXT_CENTRE (TN 8 "{}" 1.0)`.
    pub page_text_format: Option<String>,
    pub page_text_centre_format: Option<String>,
    pub page_text_right_format: Option<String>,
    // Per-field overrides for PAGE_TEXT templates. If present these fields
    // are used to synthesize a template string and provide convenient tuning
    // for font, size and optional vspace without having to provide a full
    // template string. Each of these are optional and defaults are used
    // when omitted.
    pub page_text_font: Option<String>,
    pub page_text_size: Option<f32>,
    pub page_text_vspace: Option<f32>,
    pub page_text_centre_font: Option<String>,
    pub page_text_centre_size: Option<f32>,
    pub page_text_centre_vspace: Option<f32>,
    // Optional per-header-line PAGE_TEXT_CENTRE templates and overrides.
    // Each `*_format` should contain a single `{}` placeholder for text.
    #[serde(default)]
    pub series_page_text_centre_format_enabled: bool,
    pub series_page_text_centre_format: Option<String>,
    // Optional series-header-specific PAGE_TEXT_CENTRE overrides. When any of
    // these are set, series name lines use these values instead of the generic
    // centered text template.
    pub series_page_text_centre_font: Option<String>,
    pub series_page_text_centre_size: Option<f32>,
    pub series_page_text_centre_vspace: Option<f32>,
    #[serde(default)]
    pub date_page_text_centre_format_enabled: bool,
    pub date_page_text_centre_format: Option<String>,
    pub date_page_text_centre_font: Option<String>,
    pub date_page_text_centre_size: Option<f32>,
    pub date_page_text_centre_vspace: Option<f32>,
    #[serde(default)]
    pub watermark_page_text_centre_format_enabled: bool,
    pub watermark_page_text_centre_format: Option<String>,
    pub watermark_page_text_centre_font: Option<String>,
    pub watermark_page_text_centre_size: Option<f32>,
    pub watermark_page_text_centre_vspace: Option<f32>,
    #[serde(default)]
    pub engraving_page_text_centre_format_enabled: bool,
    pub engraving_page_text_centre_format: Option<String>,
    pub engraving_page_text_centre_font: Option<String>,
    pub engraving_page_text_centre_size: Option<f32>,
    pub engraving_page_text_centre_vspace: Option<f32>,
    pub page_text_right_font: Option<String>,
    pub page_text_right_size: Option<f32>,
    pub page_text_right_vspace: Option<f32>,
    // Stamp heading formatting: font, size and label used in STAMP_HEADING
    pub stamp_heading_font: Option<String>,
    pub stamp_heading_size: Option<f32>,
    pub stamp_heading_label: Option<String>,
    pub default_catalog_code: Option<String>,
    // Optional override: force a DPI to be used when converting pixels -> mm.
    // If present the generator will compute mm_per_pixel = 25.4 / dpi and
    // use that instead of the embedded-reference ratio when converting
    // stored pixel dimensions to millimetres.
    pub force_dpi: Option<f32>,
    // Controls how stamp image heights are treated during emission.
    // "fit_box": scale images down to target_stamp_height_mm when taller.
    // "real_world": emit the actual DPI-derived/physical dimensions as-is.
    pub sizing_mode: Option<String>,

    // Progress indicator settings
    // When true, show spinning progress indicator with current operation status
    pub show_progress_indicator: Option<bool>,
    // Default Colnect inbox directory used when `--colnect-inbox` is not
    // provided on the CLI for `--source colnectexport` runs.
    pub colnect_inbox_path: Option<String>,
    // Optional AlbumEasy CLI path for PDF generation from generated text files.
    // Example: "/path/to/AlbumEasy/AlbumEasy.sh"
    pub albumeasy_cli_path: Option<String>,
    // Optional Albums root path override used by Review UI bootstrap selector.
    // Supports either the Albums directory itself or a parent containing Albums.
    pub bootstrap_albums_root_path: Option<String>,
    // When true, run AlbumEasy in console mode (-c) after generating the
    // album text file so the PDF is produced automatically.
    pub albumeasy_auto_generate_pdf: Option<bool>,
    // Seconds to wait for a fresh PDF after the AlbumEasy launcher exits.
    pub albumeasy_pdf_timeout_secs: Option<std::num::NonZeroU32>,
    // When true, emit additional PDF generation diagnostics to logs.
    // This is intended as a support/DX switch for shipped binaries.
    pub pdf_dx_logging: Option<bool>,

    // Wikipedia content configuration
    // Path to Wikipedia cache database
    pub wikipedia_cache_db_path: Option<String>,
    // Wikipedia content font settings
    pub content_wiki_font: Option<String>,
    pub content_wiki_font_size: Option<u32>,
    pub content_wiki_max_lines_per_page: Option<u32>,
    pub content_wiki_estimated_chars_per_line: Option<usize>,
    // Wikipedia paragraph formatting
    pub wikipedia_paragraph_font: Option<String>,
    pub wikipedia_paragraph_font_size: Option<u32>,
    pub wikipedia_paragraph_alignment: Option<String>,
    pub wikipedia_paragraph_padding: Option<f32>,
    pub wikipedia_paragraph_vspace: Option<f32>,
    pub wikipedia_paragraph_right_padding: Option<f32>,
    // Pagination strategy for Wikipedia content: "balanced" | "conservative" | "no_balance"
    pub wikipedia_pagination_strategy: Option<String>,
    // If true, distribute leftover vertical space as top padding to center content on page
    pub wikipedia_center_content: Option<bool>,
    // Conservative pagination fraction (used by conservative strategy). e.g., 0.75
    pub wikipedia_pagination_conservative_fraction: Option<f32>,
    // Wikipedia section title formatting
    pub wikipedia_section_title_font: Option<String>,
    pub wikipedia_section_title_font_size: Option<u32>,
    pub wikipedia_section_title_spacing: Option<f64>,
    pub wikipedia_use_math_spacing: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct StampworldConfig {
    pub username: Option<String>,
    pub password: Option<String>,
    pub auto_login: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ColnectConfig {
    pub username: Option<String>,
    pub password: Option<String>,
    pub auto_login: Option<bool>,
    // Master toggle for including Colnect detail metadata in output.
    // Defaults to true when omitted.
    pub include_details: Option<bool>,
    // Optional comma-separated list of Colnect detail fields. When omitted,
    // falls back to legacy `colnect_detail_fields` or built-in defaults.
    pub detail_fields: Option<String>,
    // Optional per-field boolean toggles. When at least one toggle is
    // explicitly set (true or false), this map takes precedence over
    // `detail_fields`.
    pub fields: Option<ColnectFieldToggles>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ColnectFieldToggles {
    pub name: Option<bool>,
    pub catalog_codes: Option<bool>,
    pub country: Option<bool>,
    pub colnect_link: Option<bool>,
    pub issued_on: Option<bool>,
    pub expiry_date: Option<bool>,
    pub width: Option<bool>,
    pub height: Option<bool>,
    pub size: Option<bool>,
    pub themes: Option<bool>,
    pub printing: Option<bool>,
    pub printing_type: Option<bool>,
    pub paper: Option<bool>,
    pub gum: Option<bool>,
    pub designers: Option<bool>,
    pub printers: Option<bool>,
    pub colors: Option<bool>,
    pub watermark: Option<bool>,
    pub format: Option<bool>,
    pub emission: Option<bool>,
    pub perforation: Option<bool>,
    pub face_value: Option<bool>,
    pub currency: Option<bool>,
    pub print_run: Option<bool>,
    pub score: Option<bool>,
    pub accuracy: Option<bool>,
    pub ord_mi: Option<bool>,
    pub ord_sn: Option<bool>,
    pub ord_yt: Option<bool>,
    pub ord_sg: Option<bool>,
    pub parent_item: Option<bool>,
    pub variant: Option<bool>,
    pub overprint_variant: Option<bool>,
    pub list: Option<bool>,
    pub quantity: Option<bool>,
    pub condition: Option<bool>,
    pub public_note: Option<bool>,
    pub private_note: Option<bool>,
    pub description: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ReviewUiConfig {
    pub odd_row_color: Option<String>,
    pub even_row_color: Option<String>,
    pub first_column_color: Option<String>,
    pub second_column_color: Option<String>,
    pub default_sort_mode: Option<String>,
    pub default_series_checked: Option<bool>,
    pub default_description_checked: Option<bool>,
    pub default_watermark_checked: Option<bool>,
    pub default_design_checked: Option<bool>,
    pub default_engraving_checked: Option<bool>,
    pub default_perforation_checked: Option<bool>,
    pub default_context_checked: Option<bool>,
    pub default_colnect_links_checked: Option<bool>,
    pub default_colnect_image_url_checked: Option<bool>,
    pub default_colnect_metadata_checked: Option<bool>,
    pub default_replace_sg_number_checked: Option<bool>,
    pub default_right_unmatched_only_checked: Option<bool>,
    // Optional catalog family list for Review UI selectors.
    // Format per entry: "code|label|count" (count optional)
    // Example: "sn|Stamp Number|814224"
    pub catalog_family_options: Option<Vec<String>>,
}

impl Config {
    pub fn effective_stampworld_username(&self) -> Option<String> {
        std::env::var("STAMPWORLD_USER")
            .or_else(|_| std::env::var("STAMPWORLD_USERNAME"))
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .or_else(|| {
                self.stampworld
                    .as_ref()
                    .and_then(|s| s.username.clone())
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
            })
    }

    pub fn effective_stampworld_password(&self) -> Option<String> {
        std::env::var("STAMPWORLD_PASS")
            .or_else(|_| std::env::var("STAMPWORLD_PASSWORD"))
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .or_else(|| {
                self.stampworld
                    .as_ref()
                    .and_then(|s| s.password.clone())
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
            })
    }

    pub fn effective_colnect_username(&self) -> Option<String> {
        std::env::var("COLNECT_USER")
            .or_else(|_| std::env::var("COLNECT_USERNAME"))
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .or_else(|| {
                self.colnect
                    .as_ref()
                    .and_then(|s| s.username.clone())
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
            })
    }

    pub fn effective_colnect_password(&self) -> Option<String> {
        std::env::var("COLNECT_PASS")
            .or_else(|_| std::env::var("COLNECT_PASSWORD"))
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .or_else(|| {
                self.colnect
                    .as_ref()
                    .and_then(|s| s.password.clone())
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
            })
    }

    /// Returns a Config with all default values set (used for auto-fixing missing config fields)
    pub fn with_defaults() -> Self {
        Config {
            reference_width_mm: Some(19.0),
            reference_height_mm: Some(25.0),
            target_stamp_height_mm: Some(25.0),
            stamp_box_adjust: Some(0.0),
            album_page_width_mm: Some(216.0),
            album_page_height_mm: Some(279.4),
            album_pages_title_font: Some("HN".to_string()),
            album_pages_title_size: Some(10.0),
            album_pages_header_position: Some("C".to_string()),
            page_top_margin_mm: Some(10.0),
            page_bottom_margin_mm: Some(10.0),
            page_left_margin_mm: Some(10.0),
            page_right_margin_mm: Some(10.0),
            page_border_outer: Some(0.5),
            page_border_inner1: Some(0.1),
            page_border_inner2: Some(0.0),
            page_border_spacing: Some(3.0),
            album_pages_spacing_h: Some(3.0),
            album_pages_spacing_v: Some(0.0),
            row_font_spacing: Some(0.1),
            header_font_spacing: Some(5.0),
            album_pages_footer_number_enabled: Some(true),
            album_pages_footer_number_font: Some("HN".to_string()),
            album_pages_footer_number_size: Some(8.0),
            album_pages_footer_number_position: Some("C".to_string()),
            album_pages_footer_number_start: Some(1),
            album_pages_footer_number_before: Some("Page ".to_string()),
            album_pages_footer_number_after: Some(" of $PAGES$".to_string()),
            detailed_page_fit_diagnostics: Some(false),
            expand_grouped_ranges: Some("placeholders".to_string()),
            page_fit_conservative_buffer_mm: Some(2.0),
            include_design: Some(true),
            include_engraving: Some(true),
            include_watermark: Some(true),
            include_perforation: Some(true),
            include_catalog_numbers: Some(true),
            include_series: Some(true),
            include_country: Some(true),
            include_date: Some(true),
            include_denomination: Some(true),
            include_stamps_issued: Some(true),
            include_description: Some(true),
            include_color: Some(true),
            include_mint_unused: Some(true),
            include_used: Some(true),
            stamp_detail_layout: Some("separate_rows".to_string()),
            colnect_detail_fields: Some("print_run,themes,designers,printing_type".to_string()),
            review_ui: Some(ReviewUiConfig {
                odd_row_color: Some("#ffffff".to_string()),
                even_row_color: Some("#e6eef4".to_string()),
                first_column_color: Some("#f2f6fb".to_string()),
                second_column_color: Some("#ffffff".to_string()),
                default_sort_mode: Some("year_series_date_catalog".to_string()),
                default_series_checked: Some(true),
                default_description_checked: Some(true),
                default_watermark_checked: Some(false),
                default_design_checked: Some(false),
                default_engraving_checked: Some(false),
                default_perforation_checked: Some(false),
                default_context_checked: Some(false),
                default_colnect_links_checked: Some(true),
                default_colnect_image_url_checked: Some(true),
                default_colnect_metadata_checked: Some(true),
                default_replace_sg_number_checked: Some(false),
                default_right_unmatched_only_checked: Some(false),
                catalog_family_options: None,
            }),
            max_rows_per_page: Some(5),
            max_page_vspace_mm: Some(10.0),
            row_start_es_enabled: Some(true),
            row_start_es_font: Some("HN".to_string()),
            row_start_es_size: Some(6.0),
            row_start_es_spacing: Some(0.1),
            row_start_es_vspace: Some(5.0),
            allow_single_large_stamp_per_page: Some(true),
            single_stamp_threshold_pct: Some(0.7),
            usable_height_padding_mm: Some(6.0),
            header_vspace_before_mm: Some(10.0),
            header_vspace_after_mm: Some(1.0),
            text_line_height_mm: Some(2.6),
            image_spacing_mm: Some(5.0),
            series_row_gap_mm: None,
            max_stamps_per_row: Some(3),
            extra_width_mm: Some(6.0),
            extra_height_mm: Some(6.0),
            album_stamp_img_new_setting_preserve_aspect: Some(true),
            print_flag_cover: Some(true),
            include_wikipedia_content: Some(true),
            wikipedia_ingest_enable: Some(true),
            wikipedia_ingest_max_entries: Some(12),
            wikipedia_ingest_series_prefix: Some("Wikipedia import".to_string()),
            album_stamp_img_aspect_ratio: Some(true),
            packing_horizontal_safety_mm: Some(5.0),
            packing_vertical_safety_mm: Some(2.0),
            enable_row_centering: Some(false),
            show_stamps_per_row: Some(false),
            color_album_border: Some("darkblue".to_string()),
            color_album_footer: Some("darkblue".to_string()),
            color_album_header: Some("darkblue".to_string()),
            color_album_title: Some("darkblue".to_string()),
            color_album_margin_txt: Some("darkblue".to_string()),
            color_stamp_border: Some("black".to_string()),
            color_stamp_heading: Some("black".to_string()),
            color_stamp_text: Some("black".to_string()),
            color_stamp_background: Some("black".to_string()),
            flag_cover_img_w_mm: Some(160.0),
            flag_cover_img_h_mm: Some(80.0),
            flag_cover_page_vspace_top: Some(40.0),
            flag_cover_image_vspace_after: Some(12.0),
            flag_cover_image_title_font: Some("HN".to_string()),
            flag_cover_image_title_size: Some(32.0),
            flag_cover_image_title_scale: Some(1.0),
            flag_cover_text_only_vspace_top: Some(24.0),
            flag_cover_text_only_title_font: Some("HN".to_string()),
            flag_cover_text_only_title_size: Some(48.0),
            flag_cover_text_only_title_scale: Some(1.0),
            flag_cover_text_only_vspace_after: Some(18.0),
            series_order: Some("first_date".to_string()),
            page_vspace_marker: Some("6".to_string()),
            row_start_es_marker: Some("ROW_START_ES".to_string()),
            page_text_format: Some(r#"PAGE_TEXT (HN 10 "{}" 1.0)"#.to_string()),
            page_text_centre_format: Some(r#"PAGE_TEXT_CENTRE (TN 8 "{}" 1.0)"#.to_string()),
            page_text_right_format: Some(r#"PAGE_TEXT_RIGHT (HN 10 "{}" 1.0)"#.to_string()),
            page_text_font: Some("HN".to_string()),
            page_text_size: Some(10.0),
            page_text_vspace: Some(1.0),
            page_text_centre_font: Some("HN".to_string()),
            page_text_centre_size: Some(8.0),
            page_text_centre_vspace: Some(1.0),
            series_page_text_centre_format_enabled: false,
            series_page_text_centre_format: None,
            series_page_text_centre_font: None,
            series_page_text_centre_size: None,
            series_page_text_centre_vspace: None,
            date_page_text_centre_format_enabled: false,
            date_page_text_centre_format: None,
            date_page_text_centre_font: None,
            date_page_text_centre_size: None,
            date_page_text_centre_vspace: None,
            watermark_page_text_centre_format_enabled: false,
            watermark_page_text_centre_format: None,
            watermark_page_text_centre_font: None,
            watermark_page_text_centre_size: None,
            watermark_page_text_centre_vspace: None,
            engraving_page_text_centre_format_enabled: false,
            engraving_page_text_centre_format: None,
            engraving_page_text_centre_font: None,
            engraving_page_text_centre_size: None,
            engraving_page_text_centre_vspace: Some(5.0),
            page_text_right_font: Some("HN".to_string()),
            page_text_right_size: Some(10.0),
            page_text_right_vspace: Some(1.0),
            stamp_heading_font: Some("HN".to_string()),
            stamp_heading_size: Some(6.0),
            stamp_heading_label: Some("auto".to_string()),
            default_catalog_code: Some("sg".to_string()),
            force_dpi: Some(96.0),
            sizing_mode: Some("fit_box".to_string()),
            show_progress_indicator: Some(true),
            colnect_inbox_path: Some("samples/colnect_inbox".to_string()),
            albumeasy_cli_path: None,
            bootstrap_albums_root_path: None,
            albumeasy_auto_generate_pdf: Some(false),
            albumeasy_pdf_timeout_secs: std::num::NonZeroU32::new(300),
            pdf_dx_logging: Some(false),
            wikipedia_cache_db_path: Some("wikipedia_cache.db".to_string()),
            content_wiki_font: Some("HN".to_string()),
            content_wiki_font_size: Some(12),
            // Prefer automatic estimation by default; allow users to override
            // with explicit caps only when necessary.
            content_wiki_max_lines_per_page: None,
            content_wiki_estimated_chars_per_line: None,
            wikipedia_paragraph_font: Some("HN".to_string()),
            wikipedia_paragraph_font_size: Some(12),
            wikipedia_paragraph_alignment: Some("JUSTIFIED".to_string()),
            wikipedia_paragraph_padding: Some(10.0),
            wikipedia_paragraph_vspace: Some(2.0),
            wikipedia_paragraph_right_padding: Some(10.0),
            wikipedia_pagination_strategy: Some("balanced".to_string()),
            wikipedia_center_content: Some(true),
            wikipedia_pagination_conservative_fraction: Some(0.75),
            wikipedia_section_title_font: Some("HB".to_string()),
            wikipedia_section_title_font_size: Some(14),
            wikipedia_section_title_spacing: Some(2.0),
            wikipedia_use_math_spacing: Some(true),
            // All other fields remain None (not set in defaults)
            ..Default::default()
        }
    }

    /// Effective master toggle for Colnect detail metadata output.
    /// New `[colnect] include_details` takes precedence when present.
    pub fn colnect_details_enabled(&self) -> bool {
        self.colnect
            .as_ref()
            .and_then(|c| c.include_details)
            .unwrap_or(true)
    }

    /// Effective comma-separated Colnect detail fields list.
    /// New `[colnect] detail_fields` takes precedence over legacy
    /// top-level `colnect_detail_fields`.
    pub fn effective_colnect_detail_fields(&self) -> &str {
        self.colnect
            .as_ref()
            .and_then(|c| c.detail_fields.as_deref())
            .or(self.colnect_detail_fields.as_deref())
            .unwrap_or("print_run,themes,designers,printing_type")
    }

    /// Effective enabled Colnect detail fields.
    /// Precedence:
    /// 1) `[colnect.fields]` booleans when at least one key is explicitly set
    ///    (true or false)
    /// 2) `[colnect].detail_fields` comma-separated list
    /// 3) legacy top-level `colnect_detail_fields`
    /// 4) built-in default list
    pub fn effective_colnect_detail_field_set(&self) -> std::collections::HashSet<String> {
        if let Some(fields) = self.colnect.as_ref().and_then(|c| c.fields.as_ref()) {
            let any_set = [
                fields.name,
                fields.catalog_codes,
                fields.country,
                fields.colnect_link,
                fields.issued_on,
                fields.expiry_date,
                fields.width,
                fields.height,
                fields.size,
                fields.themes,
                fields.printing,
                fields.printing_type,
                fields.paper,
                fields.gum,
                fields.designers,
                fields.printers,
                fields.colors,
                fields.watermark,
                fields.format,
                fields.emission,
                fields.perforation,
                fields.face_value,
                fields.currency,
                fields.print_run,
                fields.score,
                fields.accuracy,
                fields.ord_mi,
                fields.ord_sn,
                fields.ord_yt,
                fields.ord_sg,
                fields.parent_item,
                fields.variant,
                fields.overprint_variant,
                fields.list,
                fields.quantity,
                fields.condition,
                fields.public_note,
                fields.private_note,
                fields.description,
            ]
            .iter()
            .any(|v| v.is_some());

            if any_set {
                let mut set = std::collections::HashSet::new();
                if fields.name.unwrap_or(false) {
                    set.insert("name".to_string());
                }
                if fields.catalog_codes.unwrap_or(false) {
                    set.insert("catalog_codes".to_string());
                }
                if fields.country.unwrap_or(false) {
                    set.insert("country".to_string());
                }
                if fields.colnect_link.unwrap_or(false) {
                    set.insert("colnect_link".to_string());
                }
                if fields.issued_on.unwrap_or(false) {
                    set.insert("issued_on".to_string());
                }
                if fields.expiry_date.unwrap_or(false) {
                    set.insert("expiry_date".to_string());
                }
                if fields.width.unwrap_or(false) {
                    set.insert("width".to_string());
                }
                if fields.height.unwrap_or(false) {
                    set.insert("height".to_string());
                }
                if fields.size.unwrap_or(false) {
                    set.insert("size".to_string());
                }
                if fields.themes.unwrap_or(false) {
                    set.insert("themes".to_string());
                }
                if fields.printing.unwrap_or(false) {
                    set.insert("printing".to_string());
                }
                if fields.printing_type.unwrap_or(false) {
                    set.insert("printing_type".to_string());
                }
                if fields.paper.unwrap_or(false) {
                    set.insert("paper".to_string());
                }
                if fields.gum.unwrap_or(false) {
                    set.insert("gum".to_string());
                }
                if fields.designers.unwrap_or(false) {
                    set.insert("designers".to_string());
                }
                if fields.printers.unwrap_or(false) {
                    set.insert("printers".to_string());
                }
                if fields.colors.unwrap_or(false) {
                    set.insert("colors".to_string());
                }
                if fields.watermark.unwrap_or(false) {
                    set.insert("watermark".to_string());
                }
                if fields.format.unwrap_or(false) {
                    set.insert("format".to_string());
                }
                if fields.emission.unwrap_or(false) {
                    set.insert("emission".to_string());
                }
                if fields.perforation.unwrap_or(false) {
                    set.insert("perforation".to_string());
                }
                if fields.face_value.unwrap_or(false) {
                    set.insert("face_value".to_string());
                }
                if fields.currency.unwrap_or(false) {
                    set.insert("currency".to_string());
                }
                if fields.print_run.unwrap_or(false) {
                    set.insert("print_run".to_string());
                }
                if fields.score.unwrap_or(false) {
                    set.insert("score".to_string());
                }
                if fields.accuracy.unwrap_or(false) {
                    set.insert("accuracy".to_string());
                }
                if fields.ord_mi.unwrap_or(false) {
                    set.insert("ord_mi".to_string());
                }
                if fields.ord_sn.unwrap_or(false) {
                    set.insert("ord_sn".to_string());
                }
                if fields.ord_yt.unwrap_or(false) {
                    set.insert("ord_yt".to_string());
                }
                if fields.ord_sg.unwrap_or(false) {
                    set.insert("ord_sg".to_string());
                }
                if fields.parent_item.unwrap_or(false) {
                    set.insert("parent_item".to_string());
                }
                if fields.variant.unwrap_or(false) {
                    set.insert("variant".to_string());
                }
                if fields.overprint_variant.unwrap_or(false) {
                    set.insert("overprint_variant".to_string());
                }
                if fields.list.unwrap_or(false) {
                    set.insert("list".to_string());
                }
                if fields.quantity.unwrap_or(false) {
                    set.insert("quantity".to_string());
                }
                if fields.condition.unwrap_or(false) {
                    set.insert("condition".to_string());
                }
                if fields.public_note.unwrap_or(false) {
                    set.insert("public_note".to_string());
                }
                if fields.private_note.unwrap_or(false) {
                    set.insert("private_note".to_string());
                }
                if fields.description.unwrap_or(false) {
                    set.insert("description".to_string());
                }
                return set;
            }
        }

        self.effective_colnect_detail_fields()
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    }

    pub fn review_ui_odd_row_color(&self) -> &str {
        self.review_ui
            .as_ref()
            .and_then(|r| r.odd_row_color.as_deref())
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .unwrap_or("#ffffff")
    }

    pub fn review_ui_even_row_color(&self) -> &str {
        self.review_ui
            .as_ref()
            .and_then(|r| r.even_row_color.as_deref())
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .unwrap_or("#70c1ff")
    }

    pub fn review_ui_first_column_color(&self) -> &str {
        self.review_ui
            .as_ref()
            .and_then(|r| r.first_column_color.as_deref())
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .unwrap_or("#f2f6fb")
    }

    pub fn review_ui_second_column_color(&self) -> &str {
        self.review_ui
            .as_ref()
            .and_then(|r| r.second_column_color.as_deref())
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .unwrap_or("#ffffff")
    }

    pub fn review_ui_default_sort_mode(&self) -> String {
        let mode = self
            .review_ui
            .as_ref()
            .and_then(|r| r.default_sort_mode.as_deref())
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .unwrap_or("year_series_date_catalog")
            .to_ascii_lowercase();

        match mode.as_str() {
            "year_series_date_catalog" => "year_series_date_catalog".to_string(),
            "catalog" => "catalog".to_string(),
            "year_catalog" => "year_catalog".to_string(),
            "series_catalog" => "series_catalog".to_string(),
            "series_date_catalog" => "series_date_catalog".to_string(),
            "sg" => "catalog".to_string(),
            "year_sg" => "year_catalog".to_string(),
            "series_sg" => "series_catalog".to_string(),
            "series_date_sg" => "series_date_catalog".to_string(),
            "rowid" | "colnect_id" => mode,
            _ => "year_series_date_catalog".to_string(),
        }
    }

    pub fn review_ui_series_default_checked(&self) -> bool {
        self.review_ui
            .as_ref()
            .and_then(|r| r.default_series_checked)
            .unwrap_or(true)
    }

    pub fn review_ui_description_default_checked(&self) -> bool {
        self.review_ui
            .as_ref()
            .and_then(|r| r.default_description_checked)
            .unwrap_or(true)
    }

    pub fn review_ui_watermark_default_checked(&self) -> bool {
        self.review_ui
            .as_ref()
            .and_then(|r| r.default_watermark_checked)
            .unwrap_or(false)
    }

    pub fn review_ui_design_default_checked(&self) -> bool {
        self.review_ui
            .as_ref()
            .and_then(|r| r.default_design_checked)
            .unwrap_or(false)
    }

    pub fn review_ui_engraving_default_checked(&self) -> bool {
        self.review_ui
            .as_ref()
            .and_then(|r| r.default_engraving_checked)
            .unwrap_or(false)
    }

    pub fn review_ui_perforation_default_checked(&self) -> bool {
        self.review_ui
            .as_ref()
            .and_then(|r| r.default_perforation_checked)
            .unwrap_or(false)
    }

    pub fn review_ui_context_default_checked(&self) -> bool {
        self.review_ui
            .as_ref()
            .and_then(|r| r.default_context_checked)
            .unwrap_or(false)
    }

    pub fn review_ui_colnect_links_default_checked(&self) -> bool {
        self.review_ui
            .as_ref()
            .and_then(|r| r.default_colnect_links_checked)
            .unwrap_or(true)
    }

    pub fn review_ui_colnect_image_url_default_checked(&self) -> bool {
        self.review_ui
            .as_ref()
            .and_then(|r| r.default_colnect_image_url_checked)
            .unwrap_or(true)
    }

    pub fn review_ui_colnect_metadata_default_checked(&self) -> bool {
        self.review_ui
            .as_ref()
            .and_then(|r| r.default_colnect_metadata_checked)
            .unwrap_or(true)
    }

    pub fn review_ui_replace_sg_number_default_checked(&self) -> bool {
        self.review_ui
            .as_ref()
            .and_then(|r| r.default_replace_sg_number_checked)
            .unwrap_or(false)
    }

    pub fn review_ui_right_unmatched_only_default_checked(&self) -> bool {
        self.review_ui
            .as_ref()
            .and_then(|r| r.default_right_unmatched_only_checked)
            .unwrap_or(false)
    }

    pub fn review_ui_catalog_family_options(&self) -> Vec<String> {
        self.review_ui
            .as_ref()
            .and_then(|r| r.catalog_family_options.clone())
            .unwrap_or_default()
    }

    pub fn effective_default_catalog_code(&self) -> String {
        self.default_catalog_code
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(crate::catalog::normalize_catalog_type)
            .unwrap_or_else(|| "sg".to_string())
    }

    pub fn wikipedia_ingest_enabled(&self) -> bool {
        self.wikipedia_ingest_enable.unwrap_or(true)
    }

    pub fn effective_wikipedia_ingest_max_entries(&self) -> usize {
        self.wikipedia_ingest_max_entries.unwrap_or(12)
    }

    pub fn effective_wikipedia_ingest_series_prefix(&self) -> &str {
        self.wikipedia_ingest_series_prefix
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .unwrap_or("Wikipedia import")
    }

    pub fn effective_stamp_heading_label(&self) -> String {
        match self
            .stamp_heading_label
            .as_deref()
            .map(str::trim)
            .unwrap_or("auto")
        {
            "" => String::new(),
            label if label.eq_ignore_ascii_case("auto") => {
                crate::catalog::catalog_display_name(&self.effective_default_catalog_code())
                    .to_string()
            }
            label => label.to_string(),
        }
    }

    /// Effective default Colnect inbox path from configuration.
    pub fn effective_colnect_inbox_path(&self) -> Option<&str> {
        self.colnect_inbox_path
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
    }

    /// Effective AlbumEasy CLI path for post-generation PDF rendering.
    pub fn effective_albumeasy_cli_path(&self) -> Option<&str> {
        self.albumeasy_cli_path
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
    }

    /// Optional persisted Albums root path for Review UI bootstrap.
    pub fn effective_bootstrap_albums_root_path(&self) -> Option<&str> {
        self.bootstrap_albums_root_path
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
    }

    /// True when generated album text should be passed to AlbumEasy CLI (-c)
    /// to produce PDF output automatically.
    pub fn albumeasy_auto_pdf_enabled(&self) -> bool {
        self.albumeasy_auto_generate_pdf.unwrap_or(false)
    }

    /// Maximum PDF discovery wait after the AlbumEasy launcher exits.
    pub fn albumeasy_pdf_timeout_secs(&self) -> u32 {
        self.albumeasy_pdf_timeout_secs
            .map_or(300, |value| value.get())
    }

    /// True when additional PDF generation diagnostics should be emitted.
    pub fn pdf_dx_logging_enabled(&self) -> bool {
        self.pdf_dx_logging.unwrap_or(false)
    }
}

/// Compare the keys in the user's config file against the embedded template.
/// Returns a list of key names that appear in the template but are absent from
/// the user config, indicating options added since the user last initialised
/// their config.
///
/// Only top-level TOML keys are compared (section headers and comments are
/// ignored).  The check is purely additive — it never reports keys that exist
/// in the user config but not in the template.
pub fn check_config_staleness(user_config_content: &str) -> Vec<String> {
    fn extract_keys(content: &str) -> std::collections::HashSet<String> {
        let re = regex::Regex::new(r"(?m)^([a-z_][a-z_0-9]*)\s*=").unwrap();
        re.captures_iter(content)
            .map(|c| c[1].to_string())
            .collect()
    }

    let template_keys = extract_keys(EMBEDDED_CONFIG_TEMPLATE);
    let user_keys = extract_keys(user_config_content);

    let mut missing: Vec<String> = template_keys.difference(&user_keys).cloned().collect();
    missing.sort();
    missing
}

/// Create or update the canonical config file from the template or defaults.
/// If `force` is false and the target file exists, this function will return
/// Ok(()) without modifying the file. If `force` is true and the file exists,
/// the existing file will be renamed to a timestamped backup before writing.
pub fn create_config_from_template(
    path: &str,
    force: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let cwd = std::env::current_dir()?;
    let target = cwd.join(path);
    let template = cwd.join("config_template.toml");

    if target.exists() && !force {
        // Do not overwrite by default
        println!("Config file '{}' already exists; not overwriting (use --force-init-config to overwrite).", target.display());
        return Ok(());
    }

    // If target exists and force is true, back it up first
    if target.exists() && force {
        let ts = chrono::Utc::now().format("%Y%m%dT%H%M%SZ").to_string();
        let backup_name = format!("{}.bak.{}", target.display(), ts);
        fs::copy(&target, &backup_name)?;
        println!("Backed up existing config to {}", backup_name);
    }

    if template.exists() {
        fs::copy(&template, &target)?;
        println!(
            "Copied template {} -> {}",
            template.display(),
            target.display()
        );
    } else {
        fs::write(&template, EMBEDDED_CONFIG_TEMPLATE)?;
        fs::copy(&template, &target)?;
        println!(
            "Template was missing. Wrote embedded template {} and copied to {}",
            template.display(),
            target.display()
        );
    }

    Ok(())
}

/// Detects duplicate keys in TOML content by parsing line by line
/// Returns a vector of duplicate key names if any are found
fn detect_duplicate_keys(content: &str) -> Option<Vec<String>> {
    use std::collections::HashMap;

    let mut key_counts: HashMap<String, usize> = HashMap::new();
    let mut duplicates: Vec<String> = Vec::new();
    let mut current_table_path = String::new();

    for line in content.lines() {
        let trimmed_no_comment = line.split('#').next().unwrap_or_default().trim();

        // Skip comments and empty lines
        if trimmed_no_comment.is_empty() {
            continue;
        }

        // Track table scope so keys with the same name in different
        // sections (for example `description` in multiple tables) are not
        // reported as duplicates.
        if trimmed_no_comment.starts_with("[[") && trimmed_no_comment.ends_with("]]") {
            current_table_path = trimmed_no_comment
                .trim_start_matches("[[")
                .trim_end_matches("]]")
                .trim()
                .to_string();
            continue;
        }
        if trimmed_no_comment.starts_with('[') && trimmed_no_comment.ends_with(']') {
            current_table_path = trimmed_no_comment
                .trim_start_matches('[')
                .trim_end_matches(']')
                .trim()
                .to_string();
            continue;
        }

        // Look for key = value patterns (basic detection)
        if let Some(key_end) = trimmed_no_comment.find('=') {
            let key = trimmed_no_comment[..key_end].trim();

            // Skip if key contains brackets (array/table syntax)
            if key.contains('[') || key.contains(']') {
                continue;
            }

            // Remove quotes if present
            let clean_key = key.trim_matches('"').trim_matches('\'');
            let scoped_key = if current_table_path.is_empty() {
                clean_key.to_string()
            } else {
                format!("{}.{}", current_table_path, clean_key)
            };

            *key_counts.entry(scoped_key).or_insert(0) += 1;
        }
    }

    // Collect keys that appear more than once
    for (key, count) in key_counts {
        if count > 1 {
            duplicates.push(key);
        }
    }

    if duplicates.is_empty() {
        None
    } else {
        duplicates.sort();
        Some(duplicates)
    }
}

pub fn load_config(path: &str) -> Config {
    // If the caller provided an absolute path, only try that exact file.
    // This is useful for tests which create a temporary config file and want
    // to parse it without changing the process-wide current working directory.
    let try_paths = if std::path::Path::new(path).is_absolute() {
        vec![std::path::PathBuf::from(path)]
    } else {
        // Try to find the config file starting at the current working directory
        // and searching upward through parent directories. This ensures that
        // running the binary from a subdirectory (e.g. Albums/Canada) will still
        // pick up the project's top-level `config.toml`.
        let mut paths = Vec::new();

        // First, try paths relative to current working directory
        if let Ok(mut dir) = std::env::current_dir() {
            for _ in 0..8 {
                paths.push(dir.join(path));
                if !dir.pop() {
                    break;
                }
            }
        }

        // Also try paths relative to the executable's location
        // This helps when running the executable from a different directory
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(mut exe_dir) = exe_path.parent() {
                for _ in 0..8 {
                    paths.push(exe_dir.join(path));
                    if let Some(parent) = exe_dir.parent() {
                        exe_dir = parent;
                    } else {
                        break;
                    }
                }
            }
        }

        // Also try the literal path as a last resort
        paths.push(std::path::PathBuf::from(path));
        paths
    };

    for candidate in try_paths.iter() {
        debug!("Trying config path: {}", candidate.display());
        if let Ok(content) = fs::read_to_string(candidate) {
            info!("Found config file at: {}", candidate.display());
            debug!(
                "Config file content (first 200 chars): {}",
                content.chars().take(200).collect::<String>()
            );

            // Check for duplicate keys in the TOML content
            if let Some(duplicates) = detect_duplicate_keys(&content) {
                eprintln!(
                    "WARNING: Config file '{}' contains duplicate keys:",
                    candidate.display()
                );
                for key in duplicates {
                    eprintln!("  - '{}' appears multiple times", key);
                }
                eprintln!("The last value for each duplicate key will be used.");
                eprintln!("Consider removing the duplicate entries to clean up your config file.");
            }

            // First parse into a generic toml::Value to inspect raw mapping
            if let Ok(_value) = toml::from_str::<toml::Value>(&content) {
                // Successfully parsed into toml::Value; suppress noisy stdout debug prints
                // and use the log crate for structured logging instead.
                debug!("Successfully parsed as TOML");
            } else {
                // Provide user-friendly error message for TOML parsing failures
                eprintln!(
                    "ERROR: Config file '{}' contains invalid TOML syntax.",
                    candidate.display()
                );
                eprintln!("Common issues:");
                eprintln!("- Duplicate keys (same setting appears multiple times)");
                eprintln!("- Invalid value format (e.g., missing quotes around strings)");
                eprintln!("- Syntax errors (unmatched brackets, invalid characters)");
                eprintln!("Please check the file and fix any syntax errors.");
                eprintln!("File content preview (first 10 lines):");
                for (i, line) in content.lines().enumerate().take(10) {
                    eprintln!("  {}: {}", i + 1, line);
                }
                continue;
            }

            // Parse the content into a toml::Value so we can detect keys that
            // may have been placed inside `[header_formatting]` by mistake.
            match toml::from_str::<toml::Value>(&content) {
                Ok(mut value) => {
                    debug!(
                        "Successfully parsed TOML structure from {}",
                        candidate.display()
                    );
                    // If keys are present inside the header_formatting table but
                    // not at top-level, promote them to top-level so deserialization
                    // into `Config` picks them up. This handles malformed or
                    // legacy configs that put page/layout keys inside
                    // `[header_formatting]`.
                    if let Some(hf_table) = value
                        .get("header_formatting")
                        .and_then(|v| v.as_table())
                        .cloned()
                    {
                        // Only promote keys that are missing from top-level. We cloned
                        // the header_formatting table above so we don't hold an immutable
                        // borrow on `value` while mutating it.
                        if let Some(tbl) = value.as_table_mut() {
                            for (k, v) in hf_table.iter() {
                                if tbl.get(k).is_none() {
                                    tbl.insert(k.clone(), v.clone());
                                }
                            }
                        }
                    }

                    // Support sectioned config layouts by promoting known
                    // top-level section tables into the root TOML table.
                    // This keeps backward compatibility with the existing
                    // flat Config struct while allowing cleaner config files.
                    let promotable_sections = [
                        "page",
                        "stamps",
                        "layout",
                        "header",
                        "footer",
                        "colors",
                        "flag_cover",
                        "data_sources",
                        "wikipedia",
                        "images",
                        "text_templates",
                        "runtime",
                    ];
                    for section in promotable_sections.iter() {
                        if let Some(sec_table) =
                            value.get(*section).and_then(|v| v.as_table()).cloned()
                        {
                            if let Some(tbl) = value.as_table_mut() {
                                for (k, v) in sec_table.iter() {
                                    // Sectioned keys should win over stale top-level values
                                    // so explicit [stamps]/[layout]/... edits are honored.
                                    tbl.insert(k.clone(), v.clone());
                                }
                            }
                        }
                    }

                    // Now serialize the possibly-modified `Value` back to TOML
                    // and deserialize into the strongly-typed `Config`.
                    match toml::to_string(&value) {
                        Ok(s) => match toml::from_str::<Config>(&s) {
                            Ok(mut cfg) => {
                                // Auto-fix missing fields by adding defaults
                                let defaults = Config::with_defaults();
                                let mut has_missing = false;

                                // Check and set missing fields
                                if cfg.reference_width_mm.is_none() {
                                    cfg.reference_width_mm = defaults.reference_width_mm;
                                    has_missing = true;
                                }
                                if cfg.reference_height_mm.is_none() {
                                    cfg.reference_height_mm = defaults.reference_height_mm;
                                    has_missing = true;
                                }
                                if cfg.target_stamp_height_mm.is_none() {
                                    cfg.target_stamp_height_mm = defaults.target_stamp_height_mm;
                                    has_missing = true;
                                }
                                if cfg.album_page_width_mm.is_none() {
                                    cfg.album_page_width_mm = defaults.album_page_width_mm;
                                    has_missing = true;
                                }
                                if cfg.album_page_height_mm.is_none() {
                                    cfg.album_page_height_mm = defaults.album_page_height_mm;
                                    has_missing = true;
                                }
                                if cfg.album_pages_title_font.is_none() {
                                    cfg.album_pages_title_font = defaults.album_pages_title_font;
                                    has_missing = true;
                                }
                                if cfg.album_pages_title_size.is_none() {
                                    cfg.album_pages_title_size = defaults.album_pages_title_size;
                                    has_missing = true;
                                }
                                if cfg.album_pages_header_position.is_none() {
                                    cfg.album_pages_header_position =
                                        defaults.album_pages_header_position;
                                    has_missing = true;
                                }
                                if cfg.page_top_margin_mm.is_none() {
                                    cfg.page_top_margin_mm = defaults.page_top_margin_mm;
                                    has_missing = true;
                                }
                                if cfg.page_bottom_margin_mm.is_none() {
                                    cfg.page_bottom_margin_mm = defaults.page_bottom_margin_mm;
                                    has_missing = true;
                                }
                                if cfg.page_left_margin_mm.is_none() {
                                    cfg.page_left_margin_mm = defaults.page_left_margin_mm;
                                    has_missing = true;
                                }
                                if cfg.page_right_margin_mm.is_none() {
                                    cfg.page_right_margin_mm = defaults.page_right_margin_mm;
                                    has_missing = true;
                                }
                                if cfg.page_border_outer.is_none() {
                                    cfg.page_border_outer = defaults.page_border_outer;
                                    has_missing = true;
                                }
                                if cfg.page_border_inner1.is_none() {
                                    cfg.page_border_inner1 = defaults.page_border_inner1;
                                    has_missing = true;
                                }
                                if cfg.page_border_inner2.is_none() {
                                    cfg.page_border_inner2 = defaults.page_border_inner2;
                                    has_missing = true;
                                }
                                if cfg.page_border_spacing.is_none() {
                                    cfg.page_border_spacing = defaults.page_border_spacing;
                                    has_missing = true;
                                }
                                if cfg.album_pages_spacing_h.is_none() {
                                    cfg.album_pages_spacing_h = defaults.album_pages_spacing_h;
                                    has_missing = true;
                                }
                                if cfg.album_pages_spacing_v.is_none() {
                                    cfg.album_pages_spacing_v = defaults.album_pages_spacing_v;
                                    has_missing = true;
                                }
                                if cfg.row_font_spacing.is_none() {
                                    cfg.row_font_spacing = defaults.row_font_spacing;
                                    has_missing = true;
                                }
                                if cfg.header_font_spacing.is_none() {
                                    cfg.header_font_spacing = defaults.header_font_spacing;
                                    has_missing = true;
                                }
                                if cfg.album_pages_footer_number_enabled.is_none() {
                                    cfg.album_pages_footer_number_enabled =
                                        defaults.album_pages_footer_number_enabled;
                                    has_missing = true;
                                }
                                if cfg.album_pages_footer_number_font.is_none() {
                                    cfg.album_pages_footer_number_font =
                                        defaults.album_pages_footer_number_font;
                                    has_missing = true;
                                }
                                if cfg.album_pages_footer_number_size.is_none() {
                                    cfg.album_pages_footer_number_size =
                                        defaults.album_pages_footer_number_size;
                                    has_missing = true;
                                }
                                if cfg.album_pages_footer_number_position.is_none() {
                                    cfg.album_pages_footer_number_position =
                                        defaults.album_pages_footer_number_position;
                                    has_missing = true;
                                }
                                if cfg.album_pages_footer_number_start.is_none() {
                                    cfg.album_pages_footer_number_start =
                                        defaults.album_pages_footer_number_start;
                                    has_missing = true;
                                }
                                if cfg.album_pages_footer_number_before.is_none() {
                                    cfg.album_pages_footer_number_before =
                                        defaults.album_pages_footer_number_before;
                                    has_missing = true;
                                }
                                if cfg.album_pages_footer_number_after.is_none() {
                                    cfg.album_pages_footer_number_after =
                                        defaults.album_pages_footer_number_after;
                                    has_missing = true;
                                }
                                if cfg.include_design.is_none() {
                                    cfg.include_design = defaults.include_design;
                                    has_missing = true;
                                }
                                if cfg.include_engraving.is_none() {
                                    cfg.include_engraving = defaults.include_engraving;
                                    has_missing = true;
                                }
                                if cfg.include_watermark.is_none() {
                                    cfg.include_watermark = defaults.include_watermark;
                                    has_missing = true;
                                }
                                if cfg.include_perforation.is_none() {
                                    cfg.include_perforation = defaults.include_perforation;
                                    has_missing = true;
                                }
                                if cfg.include_catalog_numbers.is_none() {
                                    cfg.include_catalog_numbers = defaults.include_catalog_numbers;
                                    has_missing = true;
                                }
                                if cfg.include_series.is_none() {
                                    cfg.include_series = defaults.include_series;
                                    has_missing = true;
                                }
                                if cfg.include_country.is_none() {
                                    cfg.include_country = defaults.include_country;
                                    has_missing = true;
                                }
                                if cfg.include_date.is_none() {
                                    cfg.include_date = defaults.include_date;
                                    has_missing = true;
                                }
                                if cfg.include_denomination.is_none() {
                                    cfg.include_denomination = defaults.include_denomination;
                                    has_missing = true;
                                }
                                if cfg.include_stamps_issued.is_none() {
                                    cfg.include_stamps_issued = defaults.include_stamps_issued;
                                    has_missing = true;
                                }
                                if cfg.include_description.is_none() {
                                    cfg.include_description = defaults.include_description;
                                    has_missing = true;
                                }
                                if cfg.include_color.is_none() {
                                    cfg.include_color = defaults.include_color;
                                    has_missing = true;
                                }
                                if cfg.include_mint_unused.is_none() {
                                    cfg.include_mint_unused = defaults.include_mint_unused;
                                    has_missing = true;
                                }
                                if cfg.include_used.is_none() {
                                    cfg.include_used = defaults.include_used;
                                    has_missing = true;
                                }
                                if cfg.max_rows_per_page.is_none() {
                                    cfg.max_rows_per_page = defaults.max_rows_per_page;
                                    has_missing = true;
                                }
                                if cfg.max_page_vspace_mm.is_none() {
                                    cfg.max_page_vspace_mm = defaults.max_page_vspace_mm;
                                    has_missing = true;
                                }
                                if cfg.row_start_es_enabled.is_none() {
                                    cfg.row_start_es_enabled = defaults.row_start_es_enabled;
                                    has_missing = true;
                                }
                                if cfg.row_start_es_font.is_none() {
                                    cfg.row_start_es_font = defaults.row_start_es_font;
                                    has_missing = true;
                                }
                                if cfg.row_start_es_size.is_none() {
                                    cfg.row_start_es_size = defaults.row_start_es_size;
                                    has_missing = true;
                                }
                                if cfg.row_start_es_spacing.is_none() {
                                    cfg.row_start_es_spacing = defaults.row_start_es_spacing;
                                    has_missing = true;
                                }
                                if cfg.row_start_es_vspace.is_none() {
                                    cfg.row_start_es_vspace = defaults.row_start_es_vspace;
                                    has_missing = true;
                                }
                                if cfg.print_flag_cover.is_none() {
                                    cfg.print_flag_cover = defaults.print_flag_cover;
                                    has_missing = true;
                                }
                                if cfg.include_wikipedia_content.is_none() {
                                    cfg.include_wikipedia_content =
                                        defaults.include_wikipedia_content;
                                    has_missing = true;
                                }
                                if cfg.wikipedia_ingest_enable.is_none() {
                                    cfg.wikipedia_ingest_enable = defaults.wikipedia_ingest_enable;
                                    has_missing = true;
                                }
                                if cfg.wikipedia_ingest_max_entries.is_none() {
                                    cfg.wikipedia_ingest_max_entries =
                                        defaults.wikipedia_ingest_max_entries;
                                    has_missing = true;
                                }
                                if cfg.wikipedia_ingest_series_prefix.is_none() {
                                    cfg.wikipedia_ingest_series_prefix =
                                        defaults.wikipedia_ingest_series_prefix;
                                    has_missing = true;
                                }
                                if cfg.album_stamp_img_aspect_ratio.is_none() {
                                    cfg.album_stamp_img_aspect_ratio =
                                        defaults.album_stamp_img_aspect_ratio;
                                    has_missing = true;
                                }
                                if cfg.color_album_border.is_none() {
                                    cfg.color_album_border = defaults.color_album_border;
                                    has_missing = true;
                                }
                                if cfg.color_album_footer.is_none() {
                                    cfg.color_album_footer = defaults.color_album_footer;
                                    has_missing = true;
                                }
                                if cfg.color_album_header.is_none() {
                                    cfg.color_album_header = defaults.color_album_header;
                                    has_missing = true;
                                }
                                if cfg.color_album_title.is_none() {
                                    cfg.color_album_title = defaults.color_album_title;
                                    has_missing = true;
                                }
                                if cfg.color_album_margin_txt.is_none() {
                                    cfg.color_album_margin_txt = defaults.color_album_margin_txt;
                                    has_missing = true;
                                }
                                if cfg.color_stamp_border.is_none() {
                                    cfg.color_stamp_border = defaults.color_stamp_border;
                                    has_missing = true;
                                }
                                if cfg.color_stamp_heading.is_none() {
                                    cfg.color_stamp_heading = defaults.color_stamp_heading;
                                    has_missing = true;
                                }
                                if cfg.color_stamp_text.is_none() {
                                    cfg.color_stamp_text = defaults.color_stamp_text;
                                    has_missing = true;
                                }
                                if cfg.color_stamp_background.is_none() {
                                    cfg.color_stamp_background = defaults.color_stamp_background;
                                    has_missing = true;
                                }
                                if cfg.flag_cover_img_w_mm.is_none() {
                                    cfg.flag_cover_img_w_mm = defaults.flag_cover_img_w_mm;
                                    has_missing = true;
                                }
                                if cfg.flag_cover_img_h_mm.is_none() {
                                    cfg.flag_cover_img_h_mm = defaults.flag_cover_img_h_mm;
                                    has_missing = true;
                                }
                                if cfg.flag_cover_page_vspace_top.is_none() {
                                    cfg.flag_cover_page_vspace_top =
                                        defaults.flag_cover_page_vspace_top;
                                    has_missing = true;
                                }
                                if cfg.flag_cover_image_vspace_after.is_none() {
                                    cfg.flag_cover_image_vspace_after =
                                        defaults.flag_cover_image_vspace_after;
                                    has_missing = true;
                                }
                                if cfg.flag_cover_image_title_font.is_none() {
                                    cfg.flag_cover_image_title_font =
                                        defaults.flag_cover_image_title_font;
                                    has_missing = true;
                                }
                                if cfg.flag_cover_image_title_size.is_none() {
                                    cfg.flag_cover_image_title_size =
                                        defaults.flag_cover_image_title_size;
                                    has_missing = true;
                                }
                                if cfg.flag_cover_image_title_scale.is_none() {
                                    cfg.flag_cover_image_title_scale =
                                        defaults.flag_cover_image_title_scale;
                                    has_missing = true;
                                }
                                if cfg.flag_cover_text_only_vspace_top.is_none() {
                                    cfg.flag_cover_text_only_vspace_top =
                                        defaults.flag_cover_text_only_vspace_top;
                                    has_missing = true;
                                }
                                if cfg.flag_cover_text_only_title_font.is_none() {
                                    cfg.flag_cover_text_only_title_font =
                                        defaults.flag_cover_text_only_title_font;
                                    has_missing = true;
                                }
                                if cfg.flag_cover_text_only_title_size.is_none() {
                                    cfg.flag_cover_text_only_title_size =
                                        defaults.flag_cover_text_only_title_size;
                                    has_missing = true;
                                }
                                if cfg.flag_cover_text_only_title_scale.is_none() {
                                    cfg.flag_cover_text_only_title_scale =
                                        defaults.flag_cover_text_only_title_scale;
                                    has_missing = true;
                                }
                                if cfg.flag_cover_text_only_vspace_after.is_none() {
                                    cfg.flag_cover_text_only_vspace_after =
                                        defaults.flag_cover_text_only_vspace_after;
                                    has_missing = true;
                                }
                                if cfg.series_order.is_none() {
                                    cfg.series_order = defaults.series_order;
                                    has_missing = true;
                                }
                                if cfg.page_text_format.is_none() {
                                    cfg.page_text_format = defaults.page_text_format;
                                    has_missing = true;
                                }
                                if cfg.page_text_centre_format.is_none() {
                                    cfg.page_text_centre_format = defaults.page_text_centre_format;
                                    has_missing = true;
                                }
                                if cfg.page_text_right_format.is_none() {
                                    cfg.page_text_right_format = defaults.page_text_right_format;
                                    has_missing = true;
                                }
                                if cfg.page_text_font.is_none() {
                                    cfg.page_text_font = defaults.page_text_font;
                                    has_missing = true;
                                }
                                if cfg.page_text_size.is_none() {
                                    cfg.page_text_size = defaults.page_text_size;
                                    has_missing = true;
                                }
                                if cfg.page_text_vspace.is_none() {
                                    cfg.page_text_vspace = defaults.page_text_vspace;
                                    has_missing = true;
                                }
                                if cfg.page_text_centre_font.is_none() {
                                    cfg.page_text_centre_font = defaults.page_text_centre_font;
                                    has_missing = true;
                                }
                                if cfg.page_text_centre_size.is_none() {
                                    cfg.page_text_centre_size = defaults.page_text_centre_size;
                                    has_missing = true;
                                }
                                if cfg.page_text_centre_vspace.is_none() {
                                    cfg.page_text_centre_vspace = defaults.page_text_centre_vspace;
                                    has_missing = true;
                                }
                                // Keep series-specific centered text overrides optional.
                                if cfg.page_text_right_font.is_none() {
                                    cfg.page_text_right_font = defaults.page_text_right_font;
                                    has_missing = true;
                                }
                                if cfg.page_text_right_size.is_none() {
                                    cfg.page_text_right_size = defaults.page_text_right_size;
                                    has_missing = true;
                                }
                                if cfg.page_text_right_vspace.is_none() {
                                    cfg.page_text_right_vspace = defaults.page_text_right_vspace;
                                    has_missing = true;
                                }
                                if cfg.stamp_heading_font.is_none() {
                                    cfg.stamp_heading_font = defaults.stamp_heading_font;
                                    has_missing = true;
                                }
                                if cfg.stamp_heading_size.is_none() {
                                    cfg.stamp_heading_size = defaults.stamp_heading_size;
                                    has_missing = true;
                                }
                                if cfg.stamp_heading_label.is_none() {
                                    cfg.stamp_heading_label = defaults.stamp_heading_label;
                                    has_missing = true;
                                }
                                if cfg.default_catalog_code.is_none() {
                                    cfg.default_catalog_code = defaults.default_catalog_code;
                                    has_missing = true;
                                }
                                if cfg.force_dpi.is_none() {
                                    cfg.force_dpi = defaults.force_dpi;
                                    has_missing = true;
                                }
                                if cfg.sizing_mode.is_none() {
                                    cfg.sizing_mode = defaults.sizing_mode;
                                    has_missing = true;
                                }
                                if cfg.show_progress_indicator.is_none() {
                                    cfg.show_progress_indicator = defaults.show_progress_indicator;
                                    has_missing = true;
                                }
                                if cfg.colnect_inbox_path.is_none() {
                                    cfg.colnect_inbox_path = defaults.colnect_inbox_path;
                                    has_missing = true;
                                }
                                if cfg.albumeasy_cli_path.is_none() {
                                    cfg.albumeasy_cli_path = defaults.albumeasy_cli_path;
                                    has_missing = true;
                                }
                                if cfg.bootstrap_albums_root_path.is_none() {
                                    cfg.bootstrap_albums_root_path =
                                        defaults.bootstrap_albums_root_path;
                                    has_missing = true;
                                }
                                if cfg.albumeasy_auto_generate_pdf.is_none() {
                                    cfg.albumeasy_auto_generate_pdf =
                                        defaults.albumeasy_auto_generate_pdf;
                                    has_missing = true;
                                }
                                if cfg.albumeasy_pdf_timeout_secs.is_none() {
                                    cfg.albumeasy_pdf_timeout_secs =
                                        defaults.albumeasy_pdf_timeout_secs;
                                    has_missing = true;
                                }
                                if cfg.pdf_dx_logging.is_none() {
                                    cfg.pdf_dx_logging = defaults.pdf_dx_logging;
                                    has_missing = true;
                                }
                                if cfg.wikipedia_cache_db_path.is_none() {
                                    cfg.wikipedia_cache_db_path = defaults.wikipedia_cache_db_path;
                                    has_missing = true;
                                }
                                if cfg.content_wiki_font.is_none() {
                                    cfg.content_wiki_font = defaults.content_wiki_font;
                                    has_missing = true;
                                }
                                if cfg.content_wiki_font_size.is_none() {
                                    cfg.content_wiki_font_size = defaults.content_wiki_font_size;
                                    has_missing = true;
                                }
                                if cfg.content_wiki_max_lines_per_page.is_none() {
                                    cfg.content_wiki_max_lines_per_page =
                                        defaults.content_wiki_max_lines_per_page;
                                    has_missing = true;
                                }
                                if cfg.content_wiki_estimated_chars_per_line.is_none() {
                                    cfg.content_wiki_estimated_chars_per_line =
                                        defaults.content_wiki_estimated_chars_per_line;
                                    has_missing = true;
                                }
                                if cfg.wikipedia_paragraph_font.is_none() {
                                    cfg.wikipedia_paragraph_font =
                                        defaults.wikipedia_paragraph_font;
                                    has_missing = true;
                                }
                                if cfg.wikipedia_paragraph_font_size.is_none() {
                                    cfg.wikipedia_paragraph_font_size =
                                        defaults.wikipedia_paragraph_font_size;
                                    has_missing = true;
                                }
                                if cfg.wikipedia_paragraph_alignment.is_none() {
                                    cfg.wikipedia_paragraph_alignment =
                                        defaults.wikipedia_paragraph_alignment;
                                    has_missing = true;
                                }
                                if cfg.wikipedia_paragraph_padding.is_none() {
                                    cfg.wikipedia_paragraph_padding =
                                        defaults.wikipedia_paragraph_padding;
                                    has_missing = true;
                                }
                                if cfg.wikipedia_paragraph_vspace.is_none() {
                                    cfg.wikipedia_paragraph_vspace =
                                        defaults.wikipedia_paragraph_vspace;
                                    has_missing = true;
                                }
                                if cfg.wikipedia_paragraph_right_padding.is_none() {
                                    cfg.wikipedia_paragraph_right_padding =
                                        defaults.wikipedia_paragraph_right_padding;
                                    has_missing = true;
                                }
                                if cfg.wikipedia_section_title_font.is_none() {
                                    cfg.wikipedia_section_title_font =
                                        defaults.wikipedia_section_title_font;
                                    has_missing = true;
                                }
                                if cfg.wikipedia_section_title_font_size.is_none() {
                                    cfg.wikipedia_section_title_font_size =
                                        defaults.wikipedia_section_title_font_size;
                                    has_missing = true;
                                }
                                if cfg.wikipedia_section_title_spacing.is_none() {
                                    cfg.wikipedia_section_title_spacing =
                                        defaults.wikipedia_section_title_spacing;
                                    has_missing = true;
                                }
                                if cfg.wikipedia_use_math_spacing.is_none() {
                                    cfg.wikipedia_use_math_spacing =
                                        defaults.wikipedia_use_math_spacing;
                                    has_missing = true;
                                }

                                if has_missing {
                                    // Do NOT modify the user's existing config file automatically.
                                    // Previously the code would write missing default fields back
                                    // into the file which could overwrite comments or user edits.
                                    // Instead, log an informational message so the user can
                                    // choose to update the file manually or run a dedicated
                                    // initialization/upgrade command if desired.
                                    log::info!(
                                        "Config file {} is missing some fields. Defaults will be used at runtime but the file will not be modified.",
                                        candidate.display()
                                    );
                                    log::debug!(
                                        "Missing fields were detected and would have been added automatically in older versions. To write defaults back into the file, run the configuration initializer or edit the file manually."
                                    );
                                }

                                // If the file places the include_* toggles inside the
                                // `[header_formatting]` table (common), ensure those
                                // values are propagated into the top-level `Config` fields
                                // when the top-level ones are not present.
                                if let Some(hf) = cfg.header_formatting.clone() {
                                    cfg.include_design = cfg.include_design.or(hf.include_design);
                                    cfg.include_engraving =
                                        cfg.include_engraving.or(hf.include_engraving);
                                    cfg.include_watermark =
                                        cfg.include_watermark.or(hf.include_watermark);
                                    cfg.include_perforation =
                                        cfg.include_perforation.or(hf.include_perforation);
                                    cfg.include_catalog_numbers =
                                        cfg.include_catalog_numbers.or(hf.include_catalog_numbers);
                                    cfg.include_series = cfg.include_series.or(hf.include_series);
                                    cfg.include_country =
                                        cfg.include_country.or(hf.include_country);
                                    cfg.include_date = cfg.include_date.or(hf.include_date);
                                    cfg.print_flag_cover =
                                        cfg.print_flag_cover.or(hf.print_flag_cover);
                                    // header_formatting currently does not include the detailed
                                    // diagnostics toggle; leave it as-is when present at top-level.
                                }

                                info!("Loaded config from {}", candidate.display());
                                // Avoid dumping the entire `Config` struct into the logs
                                // (it can be large). Emit a small summary instead.
                                // Compute a stable 'include_wikipedia' summary from the
                                // possibly-optional fields to avoid referencing a
                                // non-existent field directly.
                                let cfg_include_wikipedia = cfg
                                    .include_wikipedia_content
                                    .unwrap_or(cfg.fetch_wikipedia.unwrap_or(false));
                                debug!(
                                    "Parsed Config (summary): page_size={:?}x{:?} include_wikipedia={:?}",
                                    cfg.album_page_width_mm,
                                    cfg.album_page_height_mm,
                                    cfg_include_wikipedia
                                );

                                // Debug log a few targeted color configuration values to help troubleshoot
                                debug!(
                                    "Color config values: album_border={:?} stamp_background={:?}",
                                    cfg.color_album_border, cfg.color_stamp_background
                                );

                                return cfg;
                            }
                            Err(e) => {
                                eprintln!(
                                    "ERROR: Failed to deserialize config from '{}': {}",
                                    candidate.display(),
                                    e
                                );
                                eprintln!("This usually means there are unknown or invalid configuration keys.");
                                eprintln!("Please check that all setting names are correct and values are in the right format.");
                                break;
                            }
                        },
                        Err(e) => {
                            eprintln!(
                                "ERROR: Failed to process config file '{}': {}",
                                candidate.display(),
                                e
                            );
                            eprintln!(
                                "This is an internal error while processing the configuration."
                            );
                            break;
                        }
                    }
                }
                Err(e) => {
                    eprintln!(
                        "ERROR: Config file '{}' has invalid TOML structure: {}",
                        candidate.display(),
                        e
                    );
                    eprintln!("File content preview (first 5 lines):");
                    for line in content.lines().take(5) {
                        eprintln!("  {}", line);
                    }
                    continue;
                }
            }
        } else {
            debug!("Could not read file: {}", candidate.display());
        }
    }

    // Log all the paths that were tried to help with debugging
    debug!("Config file '{}' not found. Searched paths:", path);
    for p in try_paths.iter() {
        debug!("  - {}", p.display());
    }

    // If no config file was found, create one with the canonical template
    // Preference order when creating a base `config.toml`:
    // 1. If a `config_template.toml` exists in the current working dir, copy it verbatim
    //    (preserves comments and human-friendly documentation).
    // 2. Otherwise fall back to serializing `Config::with_defaults()` (machine-friendly).
    //
    // IMPORTANT MAINTAINER NOTE / COPILOT: When you add/remove configuration
    // keys you MUST keep these three things in sync:
    //  - the `Config` struct and its `with_defaults()` implementation,
    //  - the `config_template.toml` human-readable template, and
    //  - this default-creation logic (so created files contain the template where possible).
    //
    // This comment serves as the canonical reminder for automated assistants and
    // maintainers to update all three places whenever configuration changes.
    if let Err(e) = ensure_config_and_template_files(
        path,
        "config_template.toml",
        "StampAlbumGenerator2HelpManual.pdf",
    ) {
        log::warn!(
            "Failed to ensure config/template files for '{}': {}",
            path,
            e
        );
    }

    log::info!("config.toml not found in parent dirs or failed to parse, using default values");
    Config::default()
}

pub fn with_album_config_override<R>(album_dir: &Path, f: impl FnOnce() -> R) -> R {
    SCOPED_ALBUM_CONFIG_DIR.with(|slot| {
        let previous = slot.replace(Some(album_dir.to_path_buf()));
        let result = f();
        slot.replace(previous);
        result
    })
}

/// Async sibling of [`with_album_config_override`]. Unlike the sync version, this
/// actually awaits the future produced by `f` while the thread-local override is
/// set, so the override stays in effect for the whole async operation instead of
/// being reset before the future's body actually runs.
pub async fn with_album_config_override_async<R, Fut>(
    album_dir: &Path,
    f: impl FnOnce() -> Fut,
) -> R
where
    Fut: std::future::Future<Output = R>,
{
    let previous = SCOPED_ALBUM_CONFIG_DIR.with(|slot| slot.replace(Some(album_dir.to_path_buf())));
    let result = f().await;
    SCOPED_ALBUM_CONFIG_DIR.with(|slot| slot.replace(previous));
    result
}

pub fn load_effective_config(path: &str) -> Config {
    let album_dir = SCOPED_ALBUM_CONFIG_DIR.with(|slot| slot.borrow().clone());
    match album_dir {
        Some(album_dir) => load_config_with_override(path, &album_dir),
        None => load_config(path),
    }
}

use std::collections::HashMap;

// use crate::album_globals::pdf_dx_logging_enabled;

/// Recursively merge two `toml::Value` trees.  When both sides are tables the
/// overlay's keys win (scalars/arrays are replaced; inner tables are merged
/// recurively).  For any non-table type the overlay always wins.
fn deep_merge_toml_values(base: toml::Value, overlay: toml::Value) -> toml::Value {
    match (base, overlay) {
        (toml::Value::Table(mut base_table), toml::Value::Table(overlay_table)) => {
            for (key, overlay_val) in overlay_table {
                if let Some(base_val) = base_table.get(&key).cloned() {
                    base_table.insert(key, deep_merge_toml_values(base_val, overlay_val));
                } else {
                    base_table.insert(key, overlay_val);
                }
            }
            toml::Value::Table(base_table)
        }
        // For non-table types the overlay wins unconditionally.
        (_, overlay) => overlay,
    }
}

/// Load the global `config.toml` and, if a `config.toml` also exists inside
/// `album_dir`, deep-merge it on top so that per-album settings override the
/// global defaults.
///
/// The album-local file does **not** need to be a complete config.  Any key
/// present in the album file overrides the corresponding global value; missing
/// keys simply keep their global value.  The file supports the same section
/// groupings as the global config (`[stamps]`, `[runtime]`, `[colnect]`, …).
///
/// Examples – create `Albums/Great-Britain/config.toml` containing only:
/// ```toml
/// default_catalog_code = "sg"
/// stamp_heading_label = "Stanley Gibbons"
/// ```
/// and run generation for Great Britain.  All other settings still come from
/// the global `config.toml`.
pub fn load_config_with_override(global_path: &str, album_dir: &std::path::Path) -> Config {
    // Always load the global config first (handles search-up-path, defaults, etc.)
    let base = load_config(global_path);

    // Look for an album-local override file.
    let override_path = album_dir.join("config.toml");
    if !override_path.exists() {
        return base;
    }

    let override_content = match fs::read_to_string(&override_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "WARNING: Cannot read album config override '{}': {}. Using global config.",
                override_path.display(),
                e
            );
            return base;
        }
    };

    let mut override_val: toml::Value = match toml::from_str(&override_content) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "WARNING: Album config override '{}' has invalid TOML: {}. Using global config.",
                override_path.display(),
                e
            );
            return base;
        }
    };

    // Apply the same section-promotion logic as load_config so the album
    // override can use convenience groupings like [stamps] or [runtime].
    let promotable_sections = [
        "page",
        "stamps",
        "layout",
        "header",
        "footer",
        "colors",
        "flag_cover",
        "data_sources",
        "wikipedia",
        "images",
        "text_templates",
        "runtime",
    ];
    if let Some(hf_table) = override_val
        .get("header_formatting")
        .and_then(|v| v.as_table())
        .cloned()
    {
        if let Some(tbl) = override_val.as_table_mut() {
            for (k, v) in hf_table.iter() {
                if tbl.get(k).is_none() {
                    tbl.insert(k.clone(), v.clone());
                }
            }
        }
    }
    for section in promotable_sections.iter() {
        if let Some(sec_table) = override_val
            .get(*section)
            .and_then(|v| v.as_table())
            .cloned()
        {
            if let Some(tbl) = override_val.as_table_mut() {
                for (k, v) in sec_table.iter() {
                    // In album overrides, sectioned keys should override root keys
                    // to avoid old preserved values masking current UI edits.
                    tbl.insert(k.clone(), v.clone());
                }
            }
        }
    }

    // Serialize the (already-defaulted) base Config to toml::Value so we can
    // deep-merge the override on top of it.
    let base_str = match toml::to_string(&base) {
        Ok(s) => s,
        Err(e) => {
            log::warn!(
                "Failed to serialize base config for album override merge: {}. Using global config.",
                e
            );
            return base;
        }
    };
    let mut base_val: toml::Value = match toml::from_str(&base_str) {
        Ok(v) => v,
        Err(e) => {
            log::warn!(
                "Failed to re-parse base config as toml::Value for merge: {}. Using global config.",
                e
            );
            return base;
        }
    };

    // Overlay: album keys win over global keys.
    base_val = deep_merge_toml_values(base_val, override_val);

    // Deserialize the merged value into a final Config.
    match toml::to_string(&base_val)
        .map_err(|e| e.to_string())
        .and_then(|s| toml::from_str::<Config>(&s).map_err(|e| e.to_string()))
    {
        Ok(merged) => {
            info!(
                "Applied album config override from '{}'",
                override_path.display()
            );
            merged
        }
        Err(e) => {
            eprintln!(
                "WARNING: Failed to apply album config override '{}': {}. Using global config.",
                override_path.display(),
                e
            );
            base
        }
    }
}

/// Load a simple mapping file that allows users to override site_name -> canonical_slug
/// Example TOML:
/// [slug_overrides]
/// "British-ColumbiaIsland" = "British-Columbia!Vancouver-Island"
pub fn load_slug_overrides(path: &str) -> HashMap<String, String> {
    match try_load_slug_overrides(path) {
        Ok(overrides) => overrides,
        Err(error) => {
            log::error!("Failed to load StampWorld country mappings: {error}");
            HashMap::new()
        }
    }
}

pub(crate) fn slug_override_path(path: &str) -> Result<std::path::PathBuf, String> {
    let try_paths = {
        let mut paths = Vec::new();

        // First, try paths relative to current working directory
        if let Ok(mut dir) = std::env::current_dir() {
            for _ in 0..8 {
                paths.push(dir.join(path));
                if !dir.pop() {
                    break;
                }
            }
        }

        // Also try paths relative to the executable's location
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(mut exe_dir) = exe_path.parent() {
                for _ in 0..8 {
                    paths.push(exe_dir.join(path));
                    if let Some(parent) = exe_dir.parent() {
                        exe_dir = parent;
                    } else {
                        break;
                    }
                }
            }
        }

        paths.push(std::path::PathBuf::from(path));
        paths
    };

    for candidate in try_paths {
        match fs::metadata(&candidate) {
            Ok(_) => return Ok(candidate),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("Cannot access {}: {error}", candidate.display())),
        }
    }
    std::env::current_dir()
        .map(|dir| dir.join(path))
        .map_err(|error| format!("Cannot resolve country mapping location: {error}"))
}

pub(crate) fn try_load_slug_overrides(path: &str) -> Result<HashMap<String, String>, String> {
    let candidate = slug_override_path(path)?;
    let content = match fs::read_to_string(&candidate) {
        Ok(content) => content,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(HashMap::new()),
        Err(error) => return Err(format!("Cannot read {}: {error}", candidate.display())),
    };
    let value: toml::Value = toml::from_str(&content)
        .map_err(|error| format!("Invalid {}: {error}", candidate.display()))?;
    let Some(table) = value.get("slug_overrides") else {
        return Ok(HashMap::new());
    };
    let table = table
        .as_table()
        .ok_or_else(|| "slug_overrides must be a TOML table".to_string())?;
    table
        .iter()
        .map(|(country, slug)| {
            slug.as_str()
                .map(|slug| (country.clone(), slug.to_string()))
                .ok_or_else(|| format!("StampWorld mapping for {country} must be text"))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{detect_duplicate_keys, Config};

    #[test]
    fn albumeasy_pdf_timeout_defaults_and_positive_overrides() {
        assert_eq!(Config::with_defaults().albumeasy_pdf_timeout_secs(), 300);
        let legacy: Config = toml::from_str("").expect("legacy config");
        assert_eq!(legacy.albumeasy_pdf_timeout_secs(), 300);
        let custom: Config =
            toml::from_str("albumeasy_pdf_timeout_secs = 600").expect("custom PDF timeout");
        assert_eq!(custom.albumeasy_pdf_timeout_secs(), 600);
        let roundtrip: Config =
            toml::from_str(&toml::to_string(&custom).expect("serialize timeout"))
                .expect("deserialize timeout");
        assert_eq!(roundtrip.albumeasy_pdf_timeout_secs(), 600);
        for invalid in ["0", "-1", "1.5", "\"600\""] {
            assert!(
                toml::from_str::<Config>(&format!("albumeasy_pdf_timeout_secs = {invalid}"))
                    .is_err(),
                "invalid timeout accepted: {invalid}"
            );
        }
    }

    #[test]
    fn albumeasy_pdf_timeout_album_runtime_override_wins() {
        let dir = tempfile::tempdir().expect("album config directory");
        std::fs::write(
            dir.path().join("config.toml"),
            "[runtime]\nalbumeasy_pdf_timeout_secs = 600\n",
        )
        .expect("write album config");
        let global = dir.path().join("global.toml");
        std::fs::write(
            &global,
            toml::to_string(&Config::with_defaults()).expect("serialize global config"),
        )
        .expect("write global config");
        let merged =
            super::load_config_with_override(global.to_str().expect("global path"), dir.path());
        assert_eq!(merged.albumeasy_pdf_timeout_secs(), 600);
    }

    #[test]
    fn colnect_fields_boolean_toggles_are_parsed() {
        let cfg_toml = r#"
[colnect]
include_details = true
detail_fields = "catalog_codes,themes,printing,perforation"

[colnect.fields]
catalog_codes = true
themes = true
perforation = true
issued_on = true
face_value = true
print_run = true
"#;

        let cfg: Config = toml::from_str(cfg_toml).expect("config should parse");
        let fields = cfg.effective_colnect_detail_field_set();

        assert!(fields.contains("catalog_codes"));
        assert!(fields.contains("themes"));
        assert!(fields.contains("perforation"));
        assert!(fields.contains("issued_on"));
        assert!(fields.contains("face_value"));
        assert!(fields.contains("print_run"));
        assert!(!fields.contains("printing"));
    }

    #[test]
    fn colnect_printing_type_alias_is_supported() {
        let cfg_toml = r#"
[colnect]
detail_fields = "themes"

[colnect.fields]
printing_type = true
"#;

        let cfg: Config = toml::from_str(cfg_toml).expect("config should parse");
        let fields = cfg.effective_colnect_detail_field_set();
        assert!(fields.contains("printing_type"));
    }

    #[test]
    fn colnect_all_false_toggles_override_detail_fields_to_empty() {
        let cfg_toml = r#"
[colnect]
detail_fields = "print_run,printing"

[colnect.fields]
print_run = false
printing = false
"#;

        let cfg: Config = toml::from_str(cfg_toml).expect("config should parse");
        let fields = cfg.effective_colnect_detail_field_set();
        assert!(fields.is_empty());
    }

    #[test]
    fn colnect_false_override_disables_global_true_field() {
        let base_toml = r#"
[colnect]
detail_fields = "print_run,printing"

[colnect.fields]
catalog_codes = true
issued_on = true
expiry_date = true
face_value = true
print_run = true
score = true
description = true
"#;

        let override_toml = r#"
[colnect.fields]
expiry_date = false
face_value = false
score = false
"#;

        let base_val: toml::Value = toml::from_str(base_toml).expect("base parse should succeed");
        let override_val: toml::Value =
            toml::from_str(override_toml).expect("override parse should succeed");
        let merged = super::deep_merge_toml_values(base_val, override_val);
        let cfg: Config =
            toml::from_str(&toml::to_string(&merged).expect("serialize merged config"))
                .expect("merged config should parse");

        let fields = cfg.effective_colnect_detail_field_set();
        assert!(fields.contains("catalog_codes"));
        assert!(fields.contains("issued_on"));
        assert!(fields.contains("print_run"));
        assert!(fields.contains("description"));
        assert!(!fields.contains("expiry_date"));
        assert!(!fields.contains("face_value"));
        assert!(!fields.contains("score"));
    }

    #[test]
    fn duplicate_detection_ignores_same_key_in_different_sections() {
        let cfg_toml = r#"
[stamps]
description = "A"

[review_ui]
description = "B"
"#;

        let duplicates = detect_duplicate_keys(cfg_toml);
        assert!(
            duplicates.is_none(),
            "same key name in different sections must not be treated as duplicate"
        );
    }

    #[test]
    fn duplicate_detection_finds_duplicates_in_same_section() {
        let cfg_toml = r#"
[runtime]
albumeasy_auto_generate_pdf = false
albumeasy_auto_generate_pdf = true
"#;

        let duplicates = detect_duplicate_keys(cfg_toml).unwrap_or_default();
        assert!(duplicates.contains(&"runtime.albumeasy_auto_generate_pdf".to_string()));
    }

    #[test]
    fn album_override_section_value_wins_over_root_for_stamp_visibility_keys() {
        use std::fs;
        use std::time::{SystemTime, UNIX_EPOCH};

        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock before unix epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("stamp_cfg_override_test_{}", nonce));
        let album_dir = root.join("Albums").join("Testland");
        fs::create_dir_all(&album_dir).expect("create temp album dir");

        let global_path = root.join("config.toml");
        fs::write(
            &global_path,
            r#"
include_denomination = true
[stamps]
include_denomination = true
"#,
        )
        .expect("write global config");

        fs::write(
            album_dir.join("config.toml"),
            r#"
include_denomination = true
[stamps]
include_denomination = false
"#,
        )
        .expect("write album override config");

        let cfg = super::load_config_with_override(
            global_path.to_str().expect("global path utf8"),
            &album_dir,
        );
        assert_eq!(cfg.include_denomination, Some(false));

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn scoped_album_override_applies_stamp_visibility_keys_in_effective_config() {
        use std::fs;
        use std::time::{SystemTime, UNIX_EPOCH};

        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock before unix epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("stamp_cfg_scope_test_{}", nonce));
        let album_dir = root.join("Albums").join("Testland");
        fs::create_dir_all(&album_dir).expect("create temp album dir");

        let global_path = root.join("config.toml");
        fs::write(
            &global_path,
            r#"
[stamps]
include_denomination = true
include_stamps_issued = true
include_description = true
include_color = true
include_mint_unused = true
include_used = true
"#,
        )
        .expect("write global config");

        fs::write(
            album_dir.join("config.toml"),
            r#"
[stamps]
include_denomination = false
include_stamps_issued = false
include_description = false
include_color = false
include_mint_unused = false
include_used = false
"#,
        )
        .expect("write album override config");

        let previous_dir = std::env::current_dir().expect("current dir");
        std::env::set_current_dir(&root).expect("set current dir to temp root");

        let cfg = super::with_album_config_override(&album_dir, || {
            super::load_effective_config("config.toml")
        });

        std::env::set_current_dir(previous_dir).expect("restore current dir");
        let _ = fs::remove_dir_all(&root);

        assert_eq!(cfg.include_denomination, Some(false));
        assert_eq!(cfg.include_stamps_issued, Some(false));
        assert_eq!(cfg.include_description, Some(false));
        assert_eq!(cfg.include_color, Some(false));
        assert_eq!(cfg.include_mint_unused, Some(false));
        assert_eq!(cfg.include_used, Some(false));
    }
}
