// AlbumEasy output and layout logic
use crate::config::load_config;
use crate::db_utils::{get_all_series, get_stamps_by_series, init_db};
use crate::image_utils::{calc_mm_per_pixel_embedded, estimate_real_dimensions};
use rusqlite::Connection;
// ImageStatus and OSType not needed in this module today
use crate::album_utils::clean_text;
use log::{debug, warn};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

// Helper: emit a standardized ROW_START_ES line using configured values (or sensible defaults)
// Accept an Option<&Config> so callers that don't have access to the loaded
// config (for example `new_row`) can still emit a conservative default.
fn emit_row_start_es(
    output: &mut dyn Write,
    config_opt: Option<&crate::config::Config>,
) -> std::io::Result<()> {
    let (rs_font, rs_size, rs_spacing, rs_vspace) = if let Some(cfg) = config_opt {
        (
            cfg.row_start_es_font
                .clone()
                .unwrap_or_else(|| "HN".to_string()),
            cfg.row_start_es_size.unwrap_or(6.0),
            cfg.row_start_es_spacing.unwrap_or(0.1),
            cfg.row_start_es_vspace.unwrap_or(5.0),
        )
    } else {
        ("HN".to_string(), 6.0_f32, 0.1_f32, 5.0_f32)
    };
    writeln!(
        output,
        "ROW_START_ES ({} {} {:.1} {:.1})",
        rs_font, rs_size as i32, rs_spacing, rs_vspace
    )
}

// Build a PAGE_TEXT template string for the requested kind. The generator
// supports a full-template override (e.g. `page_text_centre_format`) or a
// set of per-field values (font/size/vspace) that are combined into a
// template when present. `kind` may be "left", "centre" or "right".
fn build_page_text_template(cfg: &crate::config::Config, kind: &str) -> String {
    match kind {
        "centre" => {
            if let Some(t) = cfg.page_text_centre_format.clone() {
                return t;
            }
            let font = cfg
                .page_text_centre_font
                .clone()
                .unwrap_or_else(|| "TN".to_string());
            let size = cfg.page_text_centre_size.unwrap_or(8.0) as i32;
            let vspace = cfg.page_text_centre_vspace.unwrap_or(1.0_f32);
            // Construct the template with a literal `{}` placeholder for later replacement.
            let head = format!("PAGE_TEXT_CENTRE ({} {} \"", font, size);
            let tail = format!("\" {:.1})", vspace);
            head + "{}" + &tail
        }
        "right" => {
            if let Some(t) = cfg.page_text_right_format.clone() {
                return t;
            }
            let font = cfg
                .page_text_right_font
                .clone()
                .unwrap_or_else(|| "HN".to_string());
            let size = cfg.page_text_right_size.unwrap_or(10.0) as i32;
            let vspace = cfg.page_text_right_vspace.unwrap_or(1.0_f32);
            let head = format!("PAGE_TEXT_RIGHT ({} {} \"", font, size);
            let tail = format!("\" {:.1})", vspace);
            head + "{}" + &tail
        }
        _ => {
            // Default: left-aligned
            if let Some(t) = cfg.page_text_format.clone() {
                return t;
            }
            let font = cfg
                .page_text_font
                .clone()
                .unwrap_or_else(|| "HN".to_string());
            let size = cfg.page_text_size.unwrap_or(10.0) as i32;
            let vspace = cfg.page_text_vspace.unwrap_or(1.0_f32);
            let head = format!("PAGE_TEXT ({} {} \"", font, size);
            let tail = format!("\" {:.1})", vspace);
            head + "{}" + &tail
        }
    }
}

// Define or import Stamp and GenericAttrs
#[derive(Debug, Clone, PartialEq)]
pub struct GenericAttrs {
    attrs: std::collections::HashMap<String, String>,
}

// Inline unit tests moved to integration tests under `tests/`.
impl GenericAttrs {
    pub fn new() -> Self {
        Self {
            attrs: std::collections::HashMap::new(),
        }
    }
    // End of file: add missing closing brace for module
    pub fn set(&mut self, key: &str, value: &str) {
        self.attrs.insert(key.to_string(), value.to_string());
    }
    pub fn get(&self, key: &str) -> Option<&String> {
        self.attrs.get(key)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stamp {
    pub attrs: GenericAttrs,
    pub image_path: String,
    pub image_width_px: Option<i32>,
    pub image_height_px: Option<i32>,
}

// Top-level definitions for cross-module use
pub const MONTH_NAMES: [&str; 13] = [
    "?????",
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

// Use shared ImageStatus and OSType from `types.rs`

pub fn write_globals<W: std::io::Write>(
    writer: &mut W,
    album_name: &str,
    conn_opt: Option<&Connection>,
    start_year: Option<i32>,
    end_year: Option<i32>,
) {
    // Use the canonical globals writer from `album_utils` so all callers get the
    // same header block. The user-supplied album header is implemented there.
    crate::album_utils::write_globals(writer, album_name, conn_opt, start_year, end_year);
}

/// Compute usable height (mm) given a Config. This mirrors the logic used
/// inside `generate_album_output` and is exposed for unit testing.
pub fn compute_usable_height(cfg: &crate::config::Config) -> f64 {
    let page_height = cfg.album_page_height_mm.unwrap_or(279.4) as f64;
    let top_margin = cfg.page_top_margin_mm.unwrap_or(10.0) as f64;
    let bottom_margin = cfg.page_bottom_margin_mm.unwrap_or(10.0) as f64;
    let border_outer = cfg.page_border_outer.unwrap_or(0.5) as f64;
    let border_inner1 = cfg.page_border_inner1.unwrap_or(0.1) as f64;
    let border_inner2 = cfg.page_border_inner2.unwrap_or(0.0) as f64;
    let border_spacing = cfg.page_border_spacing.unwrap_or(3.0) as f64;
    let total_border = border_outer + border_inner1 + border_inner2 + border_spacing;
    let usable_height_padding = cfg.usable_height_padding_mm.unwrap_or(6.0) as f64;
    let pages_spacing_v = cfg.album_pages_spacing_v.unwrap_or(0.0) as f64;
    (page_height
        - top_margin
        - bottom_margin
        - total_border
        - usable_height_padding
        - pages_spacing_v) as f64
}

pub fn new_series<W: std::io::Write>(
    writer: &mut W,
    series_name: &str,
    year: &str,
    month: &str,
    day: &str,
    stamps_in_series: i32,
    is_perforated: bool,
    perforation: &str,
    rows_written: &mut i32,
    stamps_written: &mut i32,
    used_width: &mut i32,
    used_height: &mut i32,
    max_row_height: &mut i32,
) {
    let _ = writeln!(
        writer,
        "SERIES {} {} {} {} {} {} {}",
        series_name, year, month, day, stamps_in_series, is_perforated, perforation
    );
    new_page(
        writer,
        rows_written,
        stamps_written,
        used_width,
        used_height,
        max_row_height,
    );
}

/// Writes a new page header and resets counters
pub fn new_page<W: std::io::Write>(
    writer: &mut W,
    rows_written: &mut i32,
    stamps_written: &mut i32,
    used_width: &mut i32,
    used_height: &mut i32,
    max_row_height: &mut i32,
) {
    let _ = writeln!(
        writer,
        "################################################################################"
    );
    let _ = writeln!(writer, "PAGE_START");
    let _ = writeln!(
        writer,
        "################################################################################"
    );
    *rows_written = 0;
    *stamps_written = 0;
    *used_width = 0;
    *used_height = 14;
    *max_row_height = 0;
}

/// Writes a new year header. Accept an optional `Config` so callers that have
/// access to loaded configuration can provide a custom PAGE_TEXT_CENTRE template.
pub fn new_year<W: std::io::Write>(
    writer: &mut W,
    year: &str,
    config_opt: Option<&crate::config::Config>,
) {
    let tmpl = if let Some(cfg) = config_opt {
        build_page_text_template(cfg, "centre")
    } else {
        "PAGE_TEXT_CENTRE (TN 8 \"{}\" 1.0)".to_string()
    };
    let safe = year.replace('"', "\\\"");
    let _ = writeln!(writer, "{}", tmpl.replace("{}", &safe));
}

/// Writes a new row, handles page breaks, and updates counters
pub fn new_row<W: std::io::Write>(
    writer: &mut W,
    year: &str,
    rows_written: &mut i32,
    stamps_written: &mut i32,
    used_width: &mut i32,
    used_height: &mut i32,
    max_row_height: &mut i32,
) {
    // Increase used_height by the height of the previous row before checking for a new page
    *used_height += *max_row_height;
    // Use conservative constants only as a fallback; the real usable height is computed
    // in generate_album_output using config values. Here we guard against absurd values.
    if *used_height + 43.0 as i32 > 245 {
        new_page(
            writer,
            rows_written,
            stamps_written,
            used_width,
            used_height,
            max_row_height,
        );
        // new_year without a Config falls back to the default template
        new_year(writer, year, None);
    }
    // Write a conservative default ROW_START_ES using the provided writer.
    // (This function does not have access to the loaded `Config`, so call
    // the helper with `None` to use built-in sensible defaults.)
    let _ = emit_row_start_es(writer, None);

    *rows_written += 1;
    *stamps_written = 0;
    *used_width = 0;
    // Do NOT add max_row_height again here; it was already added above
    *max_row_height = 0;
}

/// Writes a stamp entry (stub implementation)
pub fn add_stamp<W: std::io::Write>(
    writer: &mut W,
    year: &str,
    field1: &str,
    field2: &str,
    field3: &str,
    color: String,
    multi_perf: bool,
    desc: &str,
    width: &str,
    height: &str,
    catalog_num: &str,
    field4: &str,
) {
    let _ = writeln!(
        writer,
        "STAMP {} {} {} {} {} {} {} {} {} {} {}",
        year, field1, field2, field3, color, multi_perf, desc, width, height, catalog_num, field4
    );
}

// AlbumEasy Output Generation
// ===================// This section contains the main entry point and helpers for generating AlbumEasy output.

// Removed duplicate imports

/// Main entry point for AlbumEasy output generation.
/// Queries the database, formats output, and writes to a .txt file for AlbumEasy import.
pub fn generate_album_output_conn<F>(
    conn: &Connection,
    album_name: &str,
    mut page_callback: Option<F>,
    start_year: Option<i32>,
    end_year: Option<i32>,
) -> ()
where
    F: FnMut(usize),
{
    // Use the start_year/end_year passed to this function (may be None)
    let passed_start_year = start_year;
    let passed_end_year = end_year;
    // Load config values from config.toml
    let config = load_config("config.toml");
    let stamp_box_height = config.stamp_box_height.unwrap_or(24.0) as f64;
    // extra space applied per-side around each stamp when emitted
    let stamp_box_adjust = config.stamp_box_adjust.unwrap_or(3.0) as f64;
    let row_font_spacing = config.row_font_spacing.unwrap_or(0.1) as f64;
    let _header_font_spacing = config.header_font_spacing.unwrap_or(3.0) as f64;
    let reference_width_mm = config.reference_width_mm.unwrap_or(200.0) as f64;
    let reference_height_mm = config.reference_height_mm.unwrap_or(240.0) as f64;
    // Reference image path and measured size (dynamic by country)
    let _country_dir = format!("Albums/{}/images", album_name.replace(' ', "-"));
    let _reference_image_path = format!("{}/BG1-s.jpg", _country_dir);
    // Use embedded reference ratio as a fallback (no external reference file required)
    // If the configured reference size is implausibly small (e.g. 24mm), assume it's
    // a misconfiguration and fall back to sensible defaults used by the project
    // (200 x 240 mm). This prevents stamp sizes from being calculated as ~1-3mm.
    let (cfg_ref_w, cfg_ref_h) = (reference_width_mm, reference_height_mm);
    // Accept smaller reference sizes (stamp-size references like 24x27mm are valid).
    // Only treat sizes under 10mm as implausibly small and trigger fallback.
    let (ref_w_mm, ref_h_mm) = if cfg_ref_w < 10.0 || cfg_ref_h < 10.0 {
        warn!(
            "Configured reference size {:.1}x{:.1}mm looks too small, falling back to 200x240mm",
            cfg_ref_w, cfg_ref_h
        );
        (200.0_f64, 240.0_f64)
    } else {
        (cfg_ref_w, cfg_ref_h)
    };
    // Compute mm-per-pixel. Allow an explicit DPI override from config
    // (useful when the page provides pixels and you want a DPI-based mapping).
    let (mm_per_pixel_w, mm_per_pixel_h) = if let Some(dpi) = config.force_dpi {
        // mm per pixel = 25.4 mm / dpi
        let mmpp = 25.4_f32 / dpi;
        (mmpp, mmpp)
    } else {
        match calc_mm_per_pixel_embedded(ref_w_mm as f32, ref_h_mm as f32) {
            Some(ratio) => ratio,
            None => {
                warn!("Embedded reference image failed to load. Using default ratio (1.0, 1.0)");
                (1.0, 1.0)
            }
        }
    };
    // If set, preserve_real_image_size tells the generator to emit the real
    // DPI-derived image sizes and NOT scale them down to `stamp_box_height`.
    let preserve_real = config.preserve_real_image_size.unwrap_or(false);

    // Canonical output filename: Albums/{Album-Name}/{Album-Name}.txt
    let album_dir = std::path::PathBuf::from(format!("Albums/{}", album_name.replace(' ', "-")));
    if let Err(e) = std::fs::create_dir_all(&album_dir) {
        warn!(
            "Failed to create album directory {}: {}",
            album_dir.display(),
            e
        );
    }
    // Build output filename. Use semi-intelligent year inference:
    // - If both start and end provided and sensible, use them.
    // - If only start provided, infer end from DB MAX(year) or current year.
    // - If only end provided, infer start from DB MIN(year) or current year.
    // - If neither provided, infer both from DB MIN/MAX(year) when available.
    let db_min_year = conn
        .query_row("SELECT MIN(year) FROM stamps", [], |r| {
            r.get::<_, Option<i32>>(0)
        })
        .ok()
        .flatten();
    let db_max_year = conn
        .query_row("SELECT MAX(year) FROM stamps", [], |r| {
            r.get::<_, Option<i32>>(0)
        })
        .ok()
        .flatten();
    // Bring Datelike into scope for `.year()`
    use chrono::Datelike;
    let current_year = chrono::Utc::now().year();

    // Determine effective start/end to use for filename (Option<i32>)
    let effective_start: Option<i32> = if let Some(s) = passed_start_year {
        if s > 0 {
            Some(s)
        } else {
            None
        }
    } else {
        // not provided: try DB min
        db_min_year
    };
    let effective_end: Option<i32> = if let Some(e) = passed_end_year {
        if e < 9999 {
            Some(e)
        } else {
            None
        }
    } else {
        // not provided: try DB max
        db_max_year
    };

    // If we have at least one bound, fill missing with sensible fallback
    let final_start = match (effective_start, effective_end) {
        (Some(s), Some(e)) => Some((s, e)),
        (Some(s), None) => Some((s, db_max_year.unwrap_or(current_year))),
        (None, Some(e)) => Some((db_min_year.unwrap_or(current_year), e)),
        (None, None) => None,
    };

    let filename = if let Some((s, e)) = final_start {
        // Always include both start and end in the filename when we have bounds.
        if s > 0 && e < 9999 && s <= e {
            format!("{}-{}-{}.txt", album_name.replace(' ', "-"), s, e)
        } else {
            format!("{}.txt", album_name.replace(' ', "-"))
        }
    } else {
        format!("{}.txt", album_name.replace(' ', "-"))
    };
    let output_path = album_dir.join(filename);
    let mut output =
        BufWriter::new(File::create(&output_path).expect("Failed to create output file"));

    // Write global album settings (pass DB connection and CLI year range so the header can include years)
    write_globals(
        &mut output,
        album_name,
        Some(conn),
        passed_start_year,
        passed_end_year,
    );

    // Emit a single leading page header for Page 1
    let _ = writeln!(output, "### Page 1 ###");
    let _ = writeln!(output, "PAGE_START");
    let _ = emit_row_start_es(&mut output, Some(&config));

    let _ = writeln!(
        output,
        "################################################################################"
    );
    // Compute usable height via shared helper (centralizes margins/borders/padding)
    let usable_height = compute_usable_height(&config);
    // Allow a configurable conservative buffer (mm) to reserve footer/decoration space
    // Default to 11.0 mm to match observed AlbumEasy rendering.
    let conservative_buffer = config.page_fit_conservative_buffer_mm.unwrap_or(11.0) as f64;
    // Effective usable height after reserving the conservative buffer
    let effective_usable = usable_height - conservative_buffer;
    // Recreate a few layout vars for diagnostics and width calculations
    let page_height = config.album_page_height_mm.unwrap_or(279.4) as f64;
    let top_margin = config.page_top_margin_mm.unwrap_or(10.0) as f64;
    let bottom_margin = config.page_bottom_margin_mm.unwrap_or(10.0) as f64;
    let border_outer = config.page_border_outer.unwrap_or(0.5) as f64;
    let border_inner1 = config.page_border_inner1.unwrap_or(0.1) as f64;
    let border_inner2 = config.page_border_inner2.unwrap_or(0.0) as f64;
    let border_spacing = config.page_border_spacing.unwrap_or(3.0) as f64;
    let total_border = border_outer + border_inner1 + border_inner2 + border_spacing;
    debug!("[PAGE LAYOUT] page_height={:.2}, top_margin={:.2}, bottom_margin={:.2}, total_border={:.2}, usable_height={:.2}",
        page_height, top_margin, bottom_margin, total_border, usable_height);
    // Font and spacing from AlbumEasy settings
    // Local layout estimates (we use configured values above where available)
    let _stamp_box_height = 24.0; // Estimate: typical stamp box height (unused default)
    let _header_lines = 4; // Year, name, desc, perforation (unused default)
                           // let header_height = (header_lines as f64) * header_font_spacing;
                           // let row_height = stamp_box_height + row_font_spacing;

    let series_order = config.series_order.clone().map(|s| s.as_str().to_string());
    let series_order_slice = series_order.as_deref();
    let series_list = get_all_series(&conn, series_order_slice).expect("Failed to get series");
    // We've emitted page 1 above; start counter at 1
    let mut page_counter = 1usize;
    let mut current_height = 0.0f64;
    let mut rows_on_page = 0;
    // Treat series_list as Vec<String> (series names). We don't filter by year here.
    let filtered_series: Vec<String> = series_list.iter().map(|s| s.clone()).collect();
    let _series_count = filtered_series.len();
    for (_series_idx, series) in filtered_series.iter().enumerate() {
        if let Some(cb) = page_callback.as_mut() {
            cb(page_counter);
        }
        // Example: Load stamps as Vec<Stamp> with generic attributes
        let mut stamps = get_stamps_by_series(&conn, &series, "0").expect("Failed to get stamps");
        // Apply inclusive year filtering if start/end years were provided.
        // The generator receives Optional start/end from the CLI. If present,
        // filter stamps to the inclusive range [start_year, end_year].
        if let (Some(sy), Some(ey)) = (passed_start_year, passed_end_year) {
            // Only apply when a sensible range was provided
            if sy > 0 && ey < 9999 && sy <= ey {
                stamps.retain(|st| st.year >= sy && st.year <= ey);
            }
        } else if let Some(sy) = passed_start_year {
            if sy > 0 {
                stamps.retain(|st| st.year >= sy);
            }
        } else if let Some(ey) = passed_end_year {
            if ey < 9999 {
                stamps.retain(|st| st.year <= ey);
            }
        }
        // Diagnostic: write a short comment indicating how many stamps we fetched for this series.
        // Sanitize the series name to a single line (collapse newlines and extra spaces)
        // so downstream tools (or grep) can reliably parse this diagnostic.
        let _safe_series = clean_text(series)
            .replace('\n', " ")
            .replace("  ", " ")
            .replace('"', "'");
        // let _ = writeln!(output, "# SERIES_FETCH series=\"{}\" count=\"{}\"", safe_series, stamps.len());
        let mut stamp_objs: Vec<Stamp> = Vec::new();
        for stamp in stamps {
            let mut attrs = GenericAttrs::new();
            attrs.set("year", &stamp.year.to_string());
            attrs.set("sg_number", &stamp.sg_number);
            attrs.set("type", &stamp.r#type);
            attrs.set("denomination", &stamp.denomination);
            attrs.set("color", &stamp.color);
            attrs.set("description", &stamp.description);
            attrs.set("stamps_issued", &stamp.stamps_issued);
            attrs.set("mint_condition", &stamp.mint_condition);
            attrs.set("unused", &stamp.unused);
            attrs.set("used", &stamp.used);
            attrs.set("month", &stamp.month);
            attrs.set("month_num", &stamp.month_num.to_string());
            attrs.set("day", &stamp.day);
            attrs.set("currency", &stamp.currency);
            attrs.set("watermark", &stamp.watermark);
            attrs.set("design", &stamp.design);
            attrs.set("engraving", &stamp.engraving);
            attrs.set("perforation", &stamp.perforation);
            // Set image_path from StampStub
            stamp_objs.push(Stamp {
                attrs,
                image_path: stamp.image_path.clone(),
                image_width_px: stamp.image_width_px,
                image_height_px: stamp.image_height_px,
            });
        }
        // Group stamps into rows of 5
        let mut stamp_rows: Vec<Vec<Stamp>> = Vec::new();
        let mut row: Vec<Stamp> = Vec::new();
        // let mut row_width = 0.0;
        for stamp in stamp_objs.into_iter() {
            let image_path = stamp.image_path.trim();
            let full_image_path = format!("Albums/{}/{}", album_name.replace(' ', "-"), image_path);
            // Prefer DB-stored pixel dimensions if available; otherwise fall back to image-based estimation
            let (est_width_mm, _) =
                if let (Some(w_px), Some(h_px)) = (stamp.image_width_px, stamp.image_height_px) {
                    (
                        ((w_px as f64) * (mm_per_pixel_w as f64)),
                        ((h_px as f64) * (mm_per_pixel_h as f64)),
                    )
                } else if let Some((w_mm, h_mm)) =
                    estimate_real_dimensions(&full_image_path, mm_per_pixel_w, mm_per_pixel_h)
                {
                    let mut w = w_mm as f64;
                    let mut h = h_mm as f64;
                    // If the measured height is larger than the configured stamp box height
                    // shrink the image proportionally so it fits within stamp_box_height
                    if !preserve_real && h > stamp_box_height {
                        let scale = stamp_box_height / h;
                        w *= scale;
                        h *= scale;
                    }
                    (w, h)
                } else {
                    (reference_width_mm as f64, reference_height_mm as f64)
                };
            let margin = 2.0f64; // margin between stamps (mm)
                                 // Page width and left/right margins come from config when present
            let page_width = config.album_page_width_mm.unwrap_or(216.0) as f64; // default US Letter
            let left_margin = config.page_left_margin_mm.unwrap_or(10.0) as f64;
            let right_margin = config.page_right_margin_mm.unwrap_or(10.0) as f64;
            // Subtract album pages horizontal spacing if configured (keeps conservative usable width)
            let pages_spacing_h = config.album_pages_spacing_h.unwrap_or(0.0) as f64;
            let usable_width = page_width
                - left_margin
                - right_margin
                - pages_spacing_h
                - (border_outer + border_inner1 + border_inner2 + border_spacing);
            // Accumulate row width as stamps are added
            let row_width: f64 = row
                .iter()
                .map(|s| {
                    let img_path = s.image_path.trim();
                    let full_img_path =
                        format!("Albums/{}/{}", album_name.replace(' ', "-"), img_path);
                    let (w_mm, h_mm) =
                        if let (Some(w_px), Some(h_px)) = (s.image_width_px, s.image_height_px) {
                            (w_px as f32 * mm_per_pixel_w, h_px as f32 * mm_per_pixel_h)
                        } else {
                            estimate_real_dimensions(&full_img_path, mm_per_pixel_w, mm_per_pixel_h)
                                .unwrap_or((reference_width_mm as f32, reference_height_mm as f32))
                        };
                    // scale to stamp_box_height if needed
                    let mut w = w_mm as f64;
                    let h = h_mm as f64;
                    if !preserve_real && h > stamp_box_height {
                        let s = stamp_box_height / h;
                        w *= s;
                        // scaled height not needed for width accumulation here
                    }
                    (w + 2.0 * stamp_box_adjust) + margin
                })
                .sum();
            // If adding this stamp would exceed usable width or row has 5 stamps, start a new row before adding
            if row.len() == 5 || row_width + est_width_mm + margin > usable_width {
                if !row.is_empty() {
                    stamp_rows.push(row);
                }
                row = Vec::new();
                // row_width = 0.0;
            }
            row.push(stamp);
        }
        if !row.is_empty() {
            stamp_rows.push(row);
        }

        let mut header_written = false;
        // Buffer header lines so we only write them when we actually emit
        // the first row on the same page. This avoids leaving a header on a
        // page with no associated stamps if a subsequent check forces a
        // page break after the header was previously emitted.
        let mut header_pending: Option<(Vec<(String, f64)>, f64, f64)> = None; // (lines, vspace_before, vspace_after)
        let mut row_idx = 0;
        while row_idx < stamp_rows.len() {
            let need_header = !header_written;
            if need_header {
                // Collect all header lines to print
                let mut header_lines_vec = Vec::new();
                let name_line = clean_text(series.trim())
                    .replace("\n", " ")
                    .replace("  ", " ");
                // Use configured text line height (mm) for header lines to more
                // accurately reflect visual space consumed by AlbumEasy font sizes.
                let header_font_h = config.text_line_height_mm.unwrap_or(2.82) as f64;
                if !name_line.is_empty() {
                    // Use configurable PAGE_TEXT_CENTRE template when present in config
                    let tmpl = build_page_text_template(&config, "centre");
                    let safe = name_line.replace('"', "\\\"");
                    header_lines_vec.push((tmpl.replace("{}", &safe), header_font_h));
                }
                let perf_trimmed = String::new();
                if !perf_trimmed.is_empty() {
                    let tmpl = build_page_text_template(&config, "centre");
                    let text = format!("Perforation {}", perf_trimmed);
                    let safe = text.replace('"', "\\\"");
                    header_lines_vec.push((tmpl.replace("{}", &safe), header_font_h));
                }
                let first_stamp = stamp_rows.get(0).and_then(|row| row.get(0));
                if let Some(stamp) = first_stamp {
                    // Optionally include a date line in the header (from DB year/month/day)
                    if config.include_date.unwrap_or(true) {
                        let date_format_cfg = config
                            .header_formatting
                            .as_ref()
                            .and_then(|hf| hf.date_format.clone())
                            .unwrap_or_else(|| "commas".to_string());
                        let year = stamp.attrs.get("year").map(|s| s.trim()).unwrap_or("");
                        let day = stamp.attrs.get("day").map(|s| s.trim()).unwrap_or("");
                        let month_raw = stamp.attrs.get("month").map(|s| s.trim()).unwrap_or("");
                        let month = if !month_raw.is_empty() {
                            month_raw.to_string()
                        } else {
                            // fall back to numeric month -> name
                            let month_num = stamp
                                .attrs
                                .get("month_num")
                                .map(|s| s.trim())
                                .unwrap_or("0");
                            if let Ok(mn) = month_num.parse::<usize>() {
                                if mn >= 1 && mn <= 12 {
                                    MONTH_NAMES[mn].to_string()
                                } else {
                                    "".to_string()
                                }
                            } else {
                                "".to_string()
                            }
                        };
                        let mut date_text = String::new();
                        if !year.is_empty() {
                            if date_format_cfg == "dashes" {
                                if !month.is_empty() && !day.is_empty() {
                                    date_text = format!("{} - {} - {}", month, day, year);
                                } else if !month.is_empty() {
                                    date_text = format!("{} - {}", month, year);
                                } else {
                                    date_text = year.to_string();
                                }
                            } else {
                                // default to commas style: "Month Day, Year"
                                if !month.is_empty() && !day.is_empty() {
                                    date_text = format!("{} {}, {}", month, day, year);
                                } else if !month.is_empty() {
                                    date_text = format!("{} {}", month, year);
                                } else {
                                    date_text = year.to_string();
                                }
                            }
                        } else {
                            // no year available, but maybe month/day exist
                            if date_format_cfg == "dashes" {
                                if !month.is_empty() && !day.is_empty() {
                                    date_text = format!("{} - {}", month, day);
                                } else if !month.is_empty() {
                                    date_text = month.clone();
                                }
                            } else {
                                if !month.is_empty() && !day.is_empty() {
                                    date_text = format!("{} {}", month, day);
                                } else if !month.is_empty() {
                                    date_text = month.clone();
                                }
                            }
                        }
                        if !date_text.is_empty() {
                            let tmpl = build_page_text_template(&config, "centre");
                            let safe = date_text.replace('"', "\\\"");
                            header_lines_vec.push((tmpl.replace("{}", &safe), header_font_h));
                        }
                    }

                    let watermark = clean_text(
                        stamp
                            .attrs
                            .get("watermark")
                            .unwrap_or(&"".to_string())
                            .trim(),
                    )
                    .replace("\n", " ")
                    .replace("  ", " ");
                    if config.include_watermark.unwrap_or(true) && !watermark.is_empty() {
                        let tmpl = build_page_text_template(&config, "centre");
                        let text = format!("Watermark: {}", watermark);
                        let safe = text.replace('"', "\\\"");
                        header_lines_vec.push((tmpl.replace("{}", &safe), header_font_h));
                    }
                    // Prefer an explicit `engraving` attribute from the DB if present.
                    // Some data sources put engraving into the separate `engraving` column
                    // while others append it to the `design` column as "... Engraving: ...".
                    let design_field = stamp
                        .attrs
                        .get("design")
                        .unwrap_or(&"".to_string())
                        .replace("\n", " ")
                        .replace("  ", " ");
                    // Extract design portion before any embedded "Engraving:" or "Perforation:"
                    let design_raw = design_field
                        .split("Engraving:")
                        .next()
                        .unwrap_or("")
                        .split("Perforation:")
                        .next()
                        .unwrap_or("")
                        .trim()
                        .to_string();

                    // Perforation value: prefer explicit DB column, otherwise try to parse
                    // it from the `design` field (some sources embed "Perforation: ...").
                    let perforation_from_column = stamp
                        .attrs
                        .get("perforation")
                        .map(|s| s.trim())
                        .unwrap_or("");
                    let perforation_raw = if !perforation_from_column.is_empty() {
                        perforation_from_column.to_string()
                    } else {
                        design_field
                            .split("Perforation:")
                            .nth(1)
                            .map(|p| {
                                p.split("Engraving:")
                                    .next()
                                    .unwrap_or("")
                                    .trim()
                                    .to_string()
                            })
                            .unwrap_or_default()
                    };
                    let perforation = clean_text(&perforation_raw);
                    if config.include_perforation.unwrap_or(true) && !perforation.is_empty() {
                        let tmpl = build_page_text_template(&config, "centre");
                        let text = format!("Perforation: {}", perforation);
                        let safe = text.replace('"', "\\\"");
                        header_lines_vec.push((tmpl.replace("{}", &safe), header_font_h));
                    }

                    // engraving may be stored separately in the `engraving` attr; prefer that
                    let engraving_from_column =
                        stamp.attrs.get("engraving").map(|s| s.trim()).unwrap_or("");
                    let engraving_raw = if !engraving_from_column.is_empty() {
                        engraving_from_column.to_string()
                    } else {
                        // fallback: try to parse engraving from the design_field
                        design_field
                            .split("Engraving:")
                            .nth(1)
                            .map(|p| {
                                p.split("Perforation:")
                                    .next()
                                    .unwrap_or("")
                                    .trim()
                                    .to_string()
                            })
                            .unwrap_or_default()
                    };
                    let design = clean_text(&design_raw);
                    let engraving = clean_text(&engraving_raw);
                    if config.include_engraving.unwrap_or(true) && !engraving.is_empty() {
                        let tmpl = build_page_text_template(&config, "centre");
                        let text = format!("Engraving: {}", engraving);
                        let safe = text.replace('"', "\\\"");
                        header_lines_vec.push((tmpl.replace("{}", &safe), header_font_h));
                    }
                    if config.include_design.unwrap_or(true) && !design.is_empty() {
                        let tmpl = build_page_text_template(&config, "centre");
                        let text = format!("Design: {}", design);
                        let safe = text.replace('"', "\\\"");
                        header_lines_vec.push((tmpl.replace("{}", &safe), header_font_h));
                    }
                }
                let header_block_height: f64 = header_lines_vec.iter().map(|(_, h)| *h).sum();
                let vspace_before_header = config.header_vspace_before_mm.unwrap_or(8.0) as f64;
                let vspace_after_header = config.header_vspace_after_mm.unwrap_or(12.5) as f64;
                // Calculate height of first row
                let stamps_in_row = &stamp_rows[row_idx];
                let mut max_row_height = 0.0f64;
                let font_height = config.text_line_height_mm.unwrap_or(3.0) as f64;
                let image_spacing = config.image_spacing_mm.unwrap_or(2.0) as f64;
                // Additional per-image adjustments from ALBUM_STAMP_IMG_NEW_SETTING
                let img_new_h = config.album_stamp_img_new_setting_h_mm.unwrap_or(0.0) as f64;
                for stamp in stamps_in_row {
                    let image_path = stamp.image_path.trim();
                    let full_image_path =
                        format!("Albums/{}/{}", album_name.replace(' ', "-"), image_path);
                    let est_height_mm = if let (Some(_w_px), Some(h_px)) =
                        (stamp.image_width_px, stamp.image_height_px)
                    {
                        let mut h = (h_px as f64) * (mm_per_pixel_h as f64);
                        if !preserve_real && h > stamp_box_height {
                            h = stamp_box_height;
                        }
                        h
                    } else if let Some((_, h_mm)) =
                        estimate_real_dimensions(&full_image_path, mm_per_pixel_w, mm_per_pixel_h)
                    {
                        let mut h = h_mm as f64;
                        if !preserve_real && h > stamp_box_height {
                            h = stamp_box_height;
                        }
                        h
                    } else {
                        reference_height_mm as f64
                    };
                    let below_stamp_1 = format!(
                        "SG {}",
                        clean_text(stamp.attrs.get("catalog_num").unwrap_or(&"".to_string()))
                    );
                    let below_stamp_2 =
                        clean_text(stamp.attrs.get("field1").unwrap_or(&"".to_string()));
                    let below_stamp_3 =
                        clean_text(stamp.attrs.get("field2").unwrap_or(&"".to_string()));
                    let text_lines = below_stamp_1.matches('\n').count()
                        + 1
                        + below_stamp_2.matches('\n').count()
                        + 1
                        + below_stamp_3.matches('\n').count()
                        + 1;
                    // Apply any configured ALBUM_STAMP_IMG_NEW_SETTING adjustments to the estimated image height
                    // Include the per-side stamp box adjustment when estimating
                    // the visual height consumed by the stamp row. The emitted
                    // box height includes 2 * stamp_box_adjust (both sides),
                    // so we must include the same amount here to avoid
                    // underestimating the row height and letting content
                    // overflow the page bottom.
                    // Use a conservative estimate for the stamp-heading height.
                    // AlbumEasy's HN 10 rendering can be larger than the simple
                    // header spacing; approximate it as header_font_spacing + text_line_height
                    let stamp_heading_h = config.header_font_spacing.unwrap_or(3.0) as f64
                        + config.text_line_height_mm.unwrap_or(2.6) as f64;
                    let adj_est_height = est_height_mm + img_new_h + 2.0 * stamp_box_adjust;
                    let total_height = adj_est_height
                        + image_spacing
                        + (text_lines as f64) * font_height
                        + stamp_heading_h;
                    if total_height > max_row_height {
                        max_row_height = total_height;
                    }
                }
                let total_header_and_row_height = vspace_before_header
                    + header_block_height
                    + vspace_after_header
                    + max_row_height
                    + row_font_spacing as f64;
                // Reserve extra room for footers/AlbumEasy page decorations to avoid overlap
                // Bumped to 11mm (was 10.0) to avoid a subtle footer bleed observed in rendered PDFs.
                // This +1mm tweak is intentionally small to preserve compact layouts while
                // matching AlbumEasy's rendering metrics more closely.
                let conservative_buffer = 11.0; // mm buffer to avoid edge cases (footer + safety)
                let conservative_needed = total_header_and_row_height + conservative_buffer;
                debug!("[PAGE FIT CHECK] Header block height: {:.2}, vspace before: {:.2}, vspace after: {:.2}, max row height: {:.2}, total needed: {:.2}, buffer: {:.2}, current: {:.2}, usable: {:.2}, will_fit: {}",
                    header_block_height, vspace_before_header, vspace_after_header, max_row_height, total_header_and_row_height, conservative_buffer, current_height, usable_height, (current_height + conservative_needed <= usable_height));
                // Peek ahead: if this is a single-row series and next series is also single-row and both fit, combine
                let force_new_page = current_height + conservative_needed > usable_height;
                // Diagnostic: write page-fit numbers to the output so we can inspect why
                // a forced page break may be happening repeatedly and preventing stamps
                // from being emitted.
                // Sanitize series to a single line and escape quotes for diagnostics
                let safe_series = clean_text(series)
                    .replace('\n', " ")
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .join(" ")
                    .replace('"', "'");
                let will_fit = current_height + conservative_needed <= usable_height;
                let _ = writeln!(output, "# PAGE_FIT series=\"{}\" need=\"{:.2}\" current=\"{:.2}\" usable=\"{:.2}\" will_fit=\"{}\"",
                        safe_series, conservative_needed, current_height, usable_height, will_fit);
                // If enabled, emit a detailed breakdown of the header and per-stamp
                // contributions that make up the conservative_needed value so we
                // can debug any under-counting.
                if config.detailed_page_fit_diagnostics.unwrap_or(false) {
                    // Detailed diagnostics are enabled, but we no longer emit the
                    // verbose per-stamp or state comments into the album output by default.
                    // Keep the flag in place so callers can enable other diagnostics if needed.
                }
                // Note: peek-ahead optimization for combining single-row series
                // was removed because it introduced complex control flow and
                // syntax issues during the merge. We keep the simpler
                // conservative page-fit check instead.
                if force_new_page {
                    // start a new page: increment counter and emit header
                    page_counter += 1;
                    if page_counter > 1 {
                        let _ = writeln!(output, "\n################################################################################");
                    }
                    let _ = writeln!(output, "### Page {} ###", page_counter);
                    let _ = writeln!(output, "PAGE_START");
                    let _ = emit_row_start_es(&mut output, Some(&config));

                    let _ = writeln!(output, "################################################################################");
                    current_height = 0.0;
                    rows_on_page = 0;
                    debug!("Forced page break before header and first row");
                    // forced page break: state tracked internally but not emitted
                }
                // Buffer the header lines (do not write yet). We'll write
                // them only when we actually emit a row on this page so
                // they don't get stranded on a previous page.
                header_pending =
                    Some((header_lines_vec, vspace_before_header, vspace_after_header));
            }
            // Calculate row height: image height + spacing + text below stamp
            let stamps_in_row = &stamp_rows[row_idx];
            let mut max_row_height = 0.0f64;
            let font_height = config.text_line_height_mm.unwrap_or(3.0) as f64; // TN 10 font spacing (mm)
            let image_spacing = config.image_spacing_mm.unwrap_or(2.0) as f64; // space below image (mm)
            let img_new_h = config.album_stamp_img_new_setting_h_mm.unwrap_or(0.0) as f64;
            // Match the conservative estimate above when computing per-row heights
            let stamp_heading_h = config.header_font_spacing.unwrap_or(3.0) as f64
                + config.text_line_height_mm.unwrap_or(2.6) as f64;
            for stamp in stamps_in_row {
                let image_path = stamp.image_path.trim();
                let full_image_path =
                    format!("Albums/{}/{}", album_name.replace(' ', "-"), image_path);
                // Prefer stored pixel height from DB when available. If absent,
                // fall back to estimating from the local image file using the
                // embedded-reference mm/px ratio. Final fallback is a
                // conservative stamp_box_height value.
                let est_height_mm = if !image_path.is_empty() {
                    if let Some(h_px) = stamp.image_height_px {
                        // Use DB-stored pixel height -> convert to mm
                        let mut h = (h_px as f64) * (mm_per_pixel_h as f64);
                        if !preserve_real && h > stamp_box_height {
                            h = stamp_box_height;
                        }
                        h
                    } else if let Some((_, h_mm)) =
                        estimate_real_dimensions(&full_image_path, mm_per_pixel_w, mm_per_pixel_h)
                    {
                        let mut h = h_mm as f64;
                        if !preserve_real && h > stamp_box_height {
                            h = stamp_box_height;
                        }
                        h
                    } else {
                        // fallback height for missing image file
                        stamp_box_height
                    }
                } else {
                    // default for missing image path (mm)
                    stamp_box_height / 2.0
                };
                // Compose all text below stamp
                let text_below = [
                    stamp
                        .attrs
                        .get("catalog_num")
                        .unwrap_or(&String::new())
                        .clone(),
                    stamp.attrs.get("field1").unwrap_or(&String::new()).clone(),
                    stamp.attrs.get("field2").unwrap_or(&String::new()).clone(),
                ]
                .join("\n");
                let text_lines = text_below.lines().count();
                let text_height = (text_lines as f64) * font_height;
                // Apply ALBUM_STAMP_IMG_NEW_SETTING height adjustment and
                // include stamp box adjustment so the row height used for
                // page-fit checks matches the actual emitted box height.
                let adj_est_height = est_height_mm + img_new_h + 2.0 * stamp_box_adjust;
                let total_height = adj_est_height + image_spacing + text_height + stamp_heading_h;
                debug!("[ROW HEIGHT] Stamp: image_path='{}', est_image_height_mm={:.2}, text_lines={}, text_height_mm={:.2}, total_row_height_mm={:.2}",
                    image_path, est_height_mm, text_lines, text_height, total_height);
                if total_height > max_row_height {
                    max_row_height = total_height;
                }
            }
            let total_row_height = max_row_height + row_font_spacing as f64;
            // Determine configured max rows per page (preference). Default to 4.
            let cfg_max_rows = config.max_rows_per_page.unwrap_or(4) as i32;
            // If this is the last row of the current series and there is a next
            // series, peek ahead and compute the next series' conservative
            // needed height (header + first row + buffer). If emitting this
            // row would leave insufficient space for that next series' header
            // and first row, prefer moving this row to the next page so the
            // next series can start cleanly on the current page instead of
            // being cramped into the bottom margin.
            if row_idx == stamp_rows.len() - 1 && _series_idx + 1 < filtered_series.len() {
                let next_series_name = &filtered_series[_series_idx + 1];
                // Load stamps of the next series
                let next_stamps =
                    get_stamps_by_series(&conn, next_series_name, "0").unwrap_or_default();
                // Build header lines for the next series similarly to above
                let mut next_header_lines_vec: Vec<(String, f64)> = Vec::new();
                let next_name_line = clean_text(next_series_name.trim())
                    .replace("\n", " ")
                    .replace("  ", " ");
                let header_font_h = config.text_line_height_mm.unwrap_or(2.82) as f64;
                if !next_name_line.is_empty() {
                    let tmpl = build_page_text_template(&config, "centre");
                    let safe = next_name_line.replace('"', "\\\"");
                    next_header_lines_vec.push((tmpl.replace("{}", &safe), header_font_h));
                }
                // attempt to pull watermark/design from first stamp if present
                if let Some(next_first_stamp) = next_stamps.get(0) {
                    let watermark = clean_text(next_first_stamp.watermark.trim())
                        .replace("\n", " ")
                        .replace("  ", " ");
                    if config.include_watermark.unwrap_or(true) && !watermark.is_empty() {
                        let tmpl = build_page_text_template(&config, "centre");
                        let text = format!("Watermark: {}", watermark);
                        let safe = text.replace('"', "\\\"");
                        next_header_lines_vec.push((tmpl.replace("{}", &safe), header_font_h));
                    }
                }
                let next_header_block_height: f64 =
                    next_header_lines_vec.iter().map(|(_, h)| *h).sum();
                let next_vspace_before = config.header_vspace_before_mm.unwrap_or(8.0) as f64;
                let next_vspace_after = config.header_vspace_after_mm.unwrap_or(12.5) as f64;
                // Estimate next series first row height: look at first row of stamps (group into row of up to 5)
                let mut next_first_row_max = 0.0f64;
                // group next_stamps into rows of 5 (StampStub from DB)
                let mut next_rows: Vec<Vec<crate::db_utils::StampStub>> = Vec::new();
                let mut nr: Vec<crate::db_utils::StampStub> = Vec::new();
                for s in next_stamps.into_iter() {
                    if nr.len() == 5 {
                        next_rows.push(nr);
                        nr = Vec::new();
                    }
                    nr.push(s);
                }
                if !nr.is_empty() {
                    next_rows.push(nr);
                }
                if let Some(first_row) = next_rows.get(0) {
                    for stamp in first_row {
                        let image_path = stamp.image_path.trim();
                        let full_image_path =
                            format!("Albums/{}/{}", album_name.replace(' ', "-"), image_path);
                        // Prefer DB-stored pixels when available, otherwise estimate from file
                        let est_height_mm = if let (Some(_w_px), Some(h_px)) =
                            (stamp.image_width_px, stamp.image_height_px)
                        {
                            let mut h = (h_px as f64) * (mm_per_pixel_h as f64);
                            if !preserve_real && h > stamp_box_height {
                                h = stamp_box_height;
                            }
                            h
                        } else if let Some((_, h_mm)) = estimate_real_dimensions(
                            &full_image_path,
                            mm_per_pixel_w,
                            mm_per_pixel_h,
                        ) {
                            let mut h = h_mm as f64;
                            if !preserve_real && h > stamp_box_height {
                                h = stamp_box_height;
                            }
                            h
                        } else {
                            reference_height_mm as f64
                        };
                        let text_lines = 1usize; // conservative minimal estimate
                        let text_height = (text_lines as f64) * header_font_h;
                        let adj_est_height = est_height_mm
                            + config.album_stamp_img_new_setting_h_mm.unwrap_or(0.0) as f64
                            + 2.0 * stamp_box_adjust;
                        let total_height = adj_est_height
                            + config.image_spacing_mm.unwrap_or(2.0) as f64
                            + text_height;
                        if total_height > next_first_row_max {
                            next_first_row_max = total_height;
                        }
                    }
                } else {
                    // no stamps in next series; still account for a minimal row
                    next_first_row_max = stamp_box_height;
                }
                let next_total_header_and_row_height = next_vspace_before
                    + next_header_block_height
                    + next_vspace_after
                    + next_first_row_max
                    + row_font_spacing as f64;
                // Include the global conservative buffer when peeking ahead
                let next_conservative_needed =
                    next_total_header_and_row_height + conservative_buffer;
                // If emitting this row then the next series' header+first-row won't fit,
                // force a page break now so the next series can start cleanly.
                if current_height + total_row_height + next_conservative_needed > usable_height {
                    page_counter += 1;
                    let _ = writeln!(output, "\n################################################################################");
                    let _ = writeln!(output, "### Page {} ###", page_counter);
                    let _ = writeln!(output, "PAGE_START");
                    let _ = emit_row_start_es(&mut output, Some(&config));

                    let _ = writeln!(output, "################################################################################");
                    current_height = 0.0;
                    header_written = false;
                    rows_on_page = 0;
                    // re-check emission of the same row on next loop iteration
                    continue;
                }
            }
            // Check if this row fits before adding, and if max rows per page reached
            // Use the effective usable height when deciding if the row fits on the
            // current page. This reserves space for footers/decoration and avoids
            // rows that visually overlap the footer in AlbumEasy PDFs.
            if current_height + total_row_height > effective_usable || rows_on_page >= cfg_max_rows
            {
                // If the row (including header) is larger than a single page's usable
                // height, we must still emit it once rather than endlessly starting
                // new pages. In that case, if we're mid-page start a new page and
                // fall through to emit the row; otherwise (we're already at a fresh
                // page) allow emission even though it will overflow the page.
                if total_row_height > usable_height {
                    // Force a new page if we already have rows on this page,
                    // otherwise proceed to emit the (oversized) row on the
                    // current fresh page so we don't loop forever.
                    // If configured to allow single-large-stamp pages, and the
                    // oversized row exceeds the configured fraction of usable
                    // height, permit it to occupy a fresh page instead of
                    // forcing additional page breaks.
                    let allow_single = config.allow_single_large_stamp_per_page.unwrap_or(true);
                    let threshold = config.single_stamp_threshold_pct.unwrap_or(0.6) as f64;
                    let percent_of_usable = total_row_height / effective_usable;
                    if rows_on_page != 0 && !(allow_single && percent_of_usable >= threshold) {
                        page_counter += 1;
                        let _ = writeln!(output, "\n################################################################################");
                        let _ = writeln!(output, "### Page {} ###", page_counter);
                        let _ = writeln!(output, "PAGE_START");
                        let _ = emit_row_start_es(&mut output, Some(&config));

                        let _ = writeln!(output, "################################################################################");
                        current_height = 0.0;
                        header_written = false;
                        rows_on_page = 0;
                    } else if rows_on_page != 0 && (allow_single && percent_of_usable >= threshold)
                    {
                        // We will allow the oversized row to be emitted as a
                        // single-row page: start a fresh page and let it emit.
                        page_counter += 1;
                        let _ = writeln!(output, "\n################################################################################");
                        let _ = writeln!(output, "### Page {} ###", page_counter);
                        let _ = writeln!(output, "PAGE_START");
                        let _ = emit_row_start_es(&mut output, Some(&config));

                        let _ = writeln!(output, "################################################################################");
                        current_height = 0.0;
                        header_written = false;
                        rows_on_page = 0;
                    }
                    // Don't continue; fall through to emit the row even if it
                    // doesn't fully fit in the usable height. This avoids an
                    // infinite loop of empty page headers.
                } else {
                    // Start new page and repeat header then re-check on next loop
                    page_counter += 1;
                    let _ = writeln!(output, "\n################################################################################");
                    let _ = writeln!(output, "### Page {} ###", page_counter);
                    let _ = writeln!(output, "PAGE_START");
                    let _ = emit_row_start_es(&mut output, Some(&config));

                    let _ = writeln!(output, "################################################################################");
                    current_height = 0.0;
                    header_written = false;
                    rows_on_page = 0;
                    continue; // re-check header on next loop
                }
            }
            // If we have a pending header, check again whether writing the
            // header + this row (plus the conservative buffer) would overflow
            // the usable page area. If so, force a new page before emitting
            // anything so the header and row start cleanly on the next page.
            if let Some((pending_lines_ref, v_before, v_after)) = &header_pending {
                let conservative_buffer_cfg =
                    config.page_fit_conservative_buffer_mm.unwrap_or(11.0) as f64;
                let header_block_height: f64 = pending_lines_ref.iter().map(|(_, h)| *h).sum();
                let needed_if_emitted = v_before
                    + header_block_height
                    + v_after
                    + total_row_height
                    + conservative_buffer_cfg;
                // If the header+row would not fit on the page at all (oversized),
                // allow emission on a fresh page rather than endlessly forcing new
                // pages. If we're mid-page, start a new page so the oversized
                // content can be emitted there; otherwise allow emission.
                if needed_if_emitted > usable_height {
                    if rows_on_page != 0 {
                        page_counter += 1;
                        let _ = writeln!(output, "\n################################################################################");
                        let _ = writeln!(output, "### Page {} ###", page_counter);
                        let _ = writeln!(output, "PAGE_START");
                        let _ = emit_row_start_es(&mut output, Some(&config));

                        let _ = writeln!(output, "################################################################################");
                        current_height = 0.0;
                        header_written = false;
                        rows_on_page = 0;
                        // Do not `continue` here: fall through and emit the oversized
                        // header+row on the fresh page to avoid infinite loop.
                    }
                } else if current_height + needed_if_emitted > usable_height {
                    // Normal case: header+row would overflow current page; force a new page now
                    page_counter += 1;
                    let _ = writeln!(output, "\n################################################################################");
                    let _ = writeln!(output, "### Page {} ###", page_counter);
                    let _ = writeln!(output, "PAGE_START");
                    let _ = emit_row_start_es(&mut output, Some(&config));

                    let _ = writeln!(output, "################################################################################");
                    current_height = 0.0;
                    header_written = false;
                    rows_on_page = 0;
                    // Re-check emission of the same row on next loop iteration
                    continue;
                }
            }
            // If we have a pending header, write it now (we are about to
            // emit the first row for this series on the current page).
            if let Some((pending_lines, v_before, v_after)) = header_pending.take() {
                // Use configurable marker so tests/tools can detect the vspace
                let pv_marker = config
                    .page_vspace_marker
                    .clone()
                    .unwrap_or_else(|| "(6)".to_string());
                let _ = writeln!(output, "PAGE_VSPACE ({})", pv_marker);
                current_height += v_before;
                // header vspace written (diagnostics suppressed)
                for (line, h) in pending_lines {
                    let _ = writeln!(output, "{}", line);
                    current_height += h;
                    // header line written (diagnostics suppressed)
                }
                let pv_marker = config
                    .page_vspace_marker
                    .clone()
                    .unwrap_or_else(|| "(6)".to_string());
                let _ = writeln!(output, "PAGE_VSPACE ({})", pv_marker);
                current_height += v_after;
                // header vspace after written (diagnostics suppressed)
                header_written = true;
            }
            // Build ROW_START_ES from parameterized config if provided, otherwise fall back to
            // the legacy `row_start_es_marker` string. Parameters are: font, size, spacing, vspace
            let _ = emit_row_start_es(&mut output, Some(&config));

            // row start written (diagnostics suppressed)
            for stamp in stamps_in_row {
                // Use correct fields from StampStub
                // Use image_path from the database directly
                let image_path = stamp.image_path.trim();
                let full_image_path =
                    format!("Albums/{}/{}", album_name.replace(' ', "-"), image_path);
                // Estimate and scale final emitted width/height to fit stamp_box_height and include stamp_box_adjust margins
                // Prefer DB-stored pixel dims if present; otherwise attempt image-based estimation
                let (mut est_width_mm, mut est_height_mm) = if let (Some(w_px), Some(h_px)) =
                    (stamp.image_width_px, stamp.image_height_px)
                {
                    (
                        (w_px as f64) * (mm_per_pixel_w as f64),
                        (h_px as f64) * (mm_per_pixel_h as f64),
                    )
                } else {
                    estimate_real_dimensions(&full_image_path, mm_per_pixel_w, mm_per_pixel_h)
                        .map(|(w, h)| (w as f64, h as f64))
                        .unwrap_or((reference_width_mm as f64, reference_height_mm as f64))
                };
                // If the estimated image height is larger than the configured
                // stamp box height, scale the image down proportionally.
                // Respect the `preserve_real_image_size` flag: when set we
                // should NOT scale images to the stamp box height and should
                // emit raw DPI-derived mm values instead.
                let mut scale = 1.0f64;
                if !preserve_real && est_height_mm > stamp_box_height {
                    scale = stamp_box_height / est_height_mm;
                    est_width_mm *= scale;
                    est_height_mm *= scale;
                }
                // Apply ALBUM_STAMP_IMG_NEW_SETTING adjustments to width/height (additive)
                let img_new_w = config.album_stamp_img_new_setting_w_mm.unwrap_or(0.0) as f64;
                let img_new_h = config.album_stamp_img_new_setting_h_mm.unwrap_or(0.0) as f64;
                est_width_mm += img_new_w;
                est_height_mm += img_new_h;
                // Debug: emit detailed per-stamp sizing diagnostics when debug logging enabled
                debug!("Stamp [{}|{}] DB_px=({:?},{:?}) mm_per_px=({:.6},{:.6}) pre_scale_mm=({:.2},{:.2}) scale={:.4} img_new=(w:{:.2},h:{:.2}) box_adjust={:.2} emit_mm_before_adjust=({:.2},{:.2}) emit_mm_final=({:.2},{:.2})",
                        album_name,
                        stamp.attrs.get("sg_number").unwrap_or(&String::new()),
                        stamp.image_width_px,
                        stamp.image_height_px,
                        mm_per_pixel_w as f64,
                        mm_per_pixel_h as f64,
                        (est_width_mm - img_new_w) / (if scale==0.0 {1.0} else {scale}), // approximate pre-scale width
                        (est_height_mm - img_new_h) / (if scale==0.0 {1.0} else {scale}), // approximate pre-scale height
                        scale,
                        img_new_w,
                        img_new_h,
                        stamp_box_adjust,
                        est_width_mm - 2.0 * stamp_box_adjust,
                        est_height_mm - 2.0 * stamp_box_adjust,
                        est_width_mm + 2.0 * stamp_box_adjust,
                        est_height_mm + 2.0 * stamp_box_adjust
                    );
                // add per-side adjust to the emitted box width/height (STAMP_BOXES_SIZE_ADJUST)
                let emit_width = est_width_mm + 2.0 * stamp_box_adjust;
                let emit_height = est_height_mm + 2.0 * stamp_box_adjust;
                // Left: denomination (may include surcharge like "20+10 (C)").
                // Wrap/split the denomination so a surcharge does not force the
                // centre/right columns to be overwritten. Prefer splitting at
                // a '+' when present (keep the '+' with the second line),
                // otherwise perform a small greedy word-wrap.
                let raw_left =
                    clean_text(stamp.attrs.get("denomination").unwrap_or(&"".to_string()));
                // Compute a conservative max chars for the left column from emit_width
                let approx_char_width_mm_left = 2.6_f64;
                let padding_mm_left = 4.0_f64;
                let avail_width_mm_left = if emit_width > padding_mm_left {
                    emit_width - padding_mm_left
                } else {
                    emit_width
                };
                let mut chars_per_line_left =
                    (avail_width_mm_left / approx_char_width_mm_left / 2.5).floor() as usize; // left column is smaller portion
                if chars_per_line_left < 4 {
                    chars_per_line_left = 4;
                }

                let wrap_left = |s: &str, max_chars: usize| -> String {
                    if s.trim().is_empty() {
                        return String::new();
                    }
                    // If string contains a '+' and not whitespace around it (common surcharge form),
                    // split at the first '+' and keep the '+' on the second line.
                    if s.contains('+') && !s.contains(' ') {
                        if let Some(pos) = s.find('+') {
                            let first = s[..pos].to_string();
                            let second = format!("+{}", s[pos + 1..].trim());
                            return format!("{}\\n{}", first, second);
                        }
                    }
                    // Greedy word-wrap preserving words
                    let mut lines: Vec<String> = Vec::new();
                    let mut current = String::new();
                    for word in s.split_whitespace() {
                        if current.is_empty() {
                            current.push_str(word);
                        } else if current.len() + 1 + word.len() <= max_chars {
                            current.push(' ');
                            current.push_str(word);
                        } else {
                            lines.push(current);
                            current = word.to_string();
                        }
                    }
                    if !current.is_empty() {
                        lines.push(current);
                    }
                    lines.join("\\n")
                };

                let left = wrap_left(&raw_left, chars_per_line_left);
                // let sgnum = clean_text(stamp.attrs.get("sg_number").unwrap_or(&"".to_string()));
                // Centre: color, description, and stamps_issued (counts are shown here when present)
                // Wrap long descriptions to multiple lines based on stamp width so they
                // don't overflow adjacent fields in the AlbumEasy output. We approximate
                // average character width and greedily build lines that fit the
                // estimated emit_width (mm).
                let raw_stamps_issued =
                    clean_text(stamp.attrs.get("stamps_issued").unwrap_or(&"".to_string()));
                let raw_description =
                    clean_text(stamp.attrs.get("description").unwrap_or(&"".to_string()));
                let raw_color = clean_text(stamp.attrs.get("color").unwrap_or(&"".to_string()));

                // Simple wrapping: estimate characters per line from emit_width.
                // Use an approximate character width (mm). This is heuristic but
                // prevents very long single-line descriptions from overwriting
                // neighboring fields. Leave short descriptions unchanged.
                let approx_char_width_mm = 2.6_f64; // heuristic average char width
                let padding_mm = 4.0_f64; // horizontal padding to avoid tight wrap
                let avail_width_mm = if emit_width > padding_mm {
                    emit_width - padding_mm
                } else {
                    emit_width
                };
                let mut chars_per_line = (avail_width_mm / approx_char_width_mm).floor() as usize;
                if chars_per_line < 10 {
                    chars_per_line = 10;
                } // minimum line length

                // Greedy word-wrap preserving words when possible
                let wrap_words = |s: &str, max_chars: usize| -> String {
                    if s.trim().is_empty() {
                        return String::new();
                    }
                    let mut lines: Vec<String> = Vec::new();
                    let mut current = String::new();
                    for word in s.split_whitespace() {
                        if current.is_empty() {
                            current.push_str(word);
                        } else if current.len() + 1 + word.len() <= max_chars {
                            current.push(' ');
                            current.push_str(word);
                        } else {
                            lines.push(current);
                            current = word.to_string();
                        }
                    }
                    if !current.is_empty() {
                        lines.push(current);
                    }
                    lines.join("\\n")
                };

                let wrapped_desc = wrap_words(&raw_description, chars_per_line);
                // Build centre block: stamps_issued (one line), wrapped description (may be multi-line), then color
                let centre = if wrapped_desc.is_empty() {
                    format!("{}\\n{}", raw_stamps_issued, raw_color)
                } else {
                    format!("{}\\n{}\\n{}", raw_stamps_issued, wrapped_desc, raw_color)
                };
                // Right: mint_condition (or unused), used, stamps_issued
                let mint_raw = stamp
                    .attrs
                    .get("mint_condition")
                    .unwrap_or(&"".to_string())
                    .trim()
                    .to_string();
                let _mint = clean_text(&mint_raw);
                let _unused = clean_text(stamp.attrs.get("unused").unwrap_or(&"".to_string()));
                let sgnum = clean_text(stamp.attrs.get("sg_number").unwrap_or(&"".to_string()));
                let _safe_sg = left
                    .replace("\\n", " ")
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .join(" ")
                    .replace("\"", "'");
                let _safe_img = image_path.replace("\"", "'");

                // Helper: consider common placeholders as "empty"
                let get_meaningful = |key: &str| -> Option<String> {
                    let v = clean_text(stamp.attrs.get(key).unwrap_or(&"".to_string()))
                        .trim()
                        .to_string();
                    let low = v.to_lowercase();
                    if v.is_empty() || low == "0" || low == "-" || low == "n/a" || low == "—" {
                        None
                    } else {
                        Some(v)
                    }
                };

                // Fetch cleaned values for all relevant keys
                let v_mint = get_meaningful("mint_condition");
                let v_unused = get_meaningful("unused");
                let v_used = get_meaningful("used");

                // Right-hand field rule: per user preference the right column must be
                //   - first line: mint_condition (if meaningful) else fallback to unused
                //   - second line: used
                // Do NOT shift used/unused into a separate third-right line here. The centre
                // field continues to hold `stamps_issued` when present.
                let first_line = v_mint
                    .or(v_unused.clone())
                    .unwrap_or_else(|| "".to_string());
                let second_line = v_used.clone().unwrap_or_else(|| "".to_string());
                let right = format!("{}\\n{}", first_line, second_line);
                if !image_path.is_empty() && Path::new(&full_image_path).exists() {
                    // Emit a lightweight diagnostic comment so we can later grep the
                    // output file and verify stamps are actually being written to the
                    // canonical Albums/{Album}/{Album}.txt file. Make sure the
                    // comment is a single line by sanitizing newlines/extra spaces.
                    let _safe_series = clean_text(series)
                        .replace("\\n", " ")
                        .split_whitespace()
                        .collect::<Vec<&str>>()
                        .join(" ")
                        .replace("\"", "'");

                    // let _ = writeln!(output, "# EMIT_STAMP series=\"{}\" sg=\"{}\" img=\"{}\"", safe_series, safe_sg, safe_img);
                    // estimate_real_dimensions returns millimetres; emit mm directly
                    let line = format!(
                        "STAMP_ADD_IMG ({:.1} {:.1} \"{}\" \"{}\" \"{}\" \"{}\")",
                        emit_width, emit_height, image_path, left, centre, right
                    );

                    let _ = writeln!(output, "{}", line.replace("\r", "").replace("\n", "\\n"));
                    let sh_font = config
                        .stamp_heading_font
                        .clone()
                        .unwrap_or_else(|| "HN".to_string());
                    let sh_size = config.stamp_heading_size.unwrap_or(10.0) as i32;
                    let sh_label = config
                        .stamp_heading_label
                        .clone()
                        .unwrap_or_else(|| "SG".to_string());
                    let _ = writeln!(
                        output,
                        "STAMP_HEADING({} {} \"{} : {}\")",
                        sh_font, sh_size, sh_label, sgnum
                    );
                    // Emit an immediate state line showing the stamp box that
                    // was just written and the expected new current_height if
                    // this were the tallest row on the page.
                    // stamp emitted (diagnostics suppressed)
                } else {
                    let _safe_series = clean_text(series)
                        .replace("\\n", " ")
                        .split_whitespace()
                        .collect::<Vec<&str>>()
                        .join(" ")
                        .replace("\"", "'");
                    let _safe_sg = left
                        .replace("\\n", " ")
                        .split_whitespace()
                        .collect::<Vec<&str>>()
                        .join(" ")
                        .replace("\"", "'");
                    let _safe_img = image_path.replace("\"", "'");
                    // let _ = writeln!(output, "# EMIT_STAMP series=\"{}\" sg=\"{}\" img=\"{}\"", safe_series, safe_sg, safe_img);
                    let line = format!(
                        "STAMP_ADD ({:.1} {:.1} \"Image\\nNot\\nAvailable\" \"{}\" \"{}\" \"{}\")",
                        emit_width, emit_height, left, centre, right
                    );

                    let _ = writeln!(output, "{}", line.replace("\r", "").replace("\n", "\\n"));
                    let sh_font = config
                        .stamp_heading_font
                        .clone()
                        .unwrap_or_else(|| "HN".to_string());
                    let sh_size = config.stamp_heading_size.unwrap_or(10.0) as i32;
                    let sh_label = config
                        .stamp_heading_label
                        .clone()
                        .unwrap_or_else(|| "SG".to_string());
                    let _ = writeln!(
                        output,
                        "STAMP_HEADING({} {} \"{} : {}\")",
                        sh_font, sh_size, sh_label, sgnum
                    );
                    let _ = writeln!(output, "# PAGE_FIT_STATE step=\"stamp_emitted\" series=\"{}\" img=\"{}\" emit_h=\"{:.2}\" current=\"{:.2}\"",
                            clean_text(series).replace('\n', " ").replace('"', "'"), image_path, emit_height, current_height + total_row_height);
                }
            }
            current_height += total_row_height;
            rows_on_page += 1;
            row_idx += 1;
        }
    }

    // End of generate_album_output inner scope: use shared clean_text from album_utils
}

/// Backwards-compatible wrapper: accept a DB path, open the DB and call the
/// connection-based implementation. This keeps existing callers (tests and
/// other modules) working while allowing callers that already hold a
/// Connection to use the more efficient conn-based variant.
pub fn generate_album_output<F>(
    db_path: &str,
    album_name: &str,
    page_callback: Option<F>,
    start_year: Option<i32>,
    end_year: Option<i32>,
) -> ()
where
    F: FnMut(usize),
{
    let conn = init_db(db_path).expect("Failed to open database");
    generate_album_output_conn(&conn, album_name, page_callback, start_year, end_year);
}
