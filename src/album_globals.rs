// AlbumEasy output and layout logic
use crate::config::load_config;
use crate::db_utils::{get_all_series, get_stamps_by_series, init_db};
use rusqlite::Connection;
use crate::image_utils::{calc_mm_per_pixel_embedded, estimate_real_dimensions};
// ImageStatus and OSType not needed in this module today
use crate::album_utils::clean_text;
use log::{debug, warn};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

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
}

// Top-level definitions for cross-module use
pub const MONTH_NAMES: [&str; 13] = [
    "?????", "January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"
];

// Use shared ImageStatus and OSType from `types.rs`

pub fn write_globals<W: std::io::Write>(writer: &mut W, album_name: &str) {
    // Use the canonical globals writer from `album_utils` so all callers get the
    // same header block. The user-supplied album header is implemented there.
    crate::album_utils::write_globals(writer, album_name);
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
    (page_height - top_margin - bottom_margin - total_border - usable_height_padding - pages_spacing_v) as f64
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
    let _ = writeln!(writer, "SERIES {} {} {} {} {} {} {}", series_name, year, month, day, stamps_in_series, is_perforated, perforation);
    new_page(writer, rows_written, stamps_written, used_width, used_height, max_row_height);
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
    *rows_written = 0;
    *stamps_written = 0;
    *used_width = 0;
    *used_height = 14;
    *max_row_height = 0;
}

/// Writes a new year header
pub fn new_year<W: std::io::Write>(writer: &mut W, year: &str) {
    let _ = writeln!(writer, "PAGE_TEXT_CENTRE (TN 8 \"{}\" 1.0)", year);
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
        new_year(writer, year);
    }
    let _ = writeln!(writer, "ROW_START_ES (TN 6 0.1 5.0)");
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
    let row_font_spacing = config.row_font_spacing.unwrap_or(5.0) as f64;
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
        warn!("Configured reference size {:.1}x{:.1}mm looks too small, falling back to 200x240mm", cfg_ref_w, cfg_ref_h);
        (200.0_f64, 240.0_f64)
    } else {
        (cfg_ref_w, cfg_ref_h)
    };
    let (mm_per_pixel_w, mm_per_pixel_h) = match calc_mm_per_pixel_embedded(
        ref_w_mm as f32,
        ref_h_mm as f32,
    ) {
        Some(ratio) => ratio,
        None => {
            warn!("Embedded reference image failed to load. Using default ratio (1.0, 1.0)");
            (1.0, 1.0)
        }
    };

    // Canonical output filename: Albums/{Album-Name}/{Album-Name}.txt
    let album_dir = std::path::PathBuf::from(format!("Albums/{}", album_name.replace(' ', "-")));
    if let Err(e) = std::fs::create_dir_all(&album_dir) {
        warn!("Failed to create album directory {}: {}", album_dir.display(), e);
    }
    // Build output filename. If a sensible start/end year range is provided
    // (start > 0 and end < 9999 and start <= end), include it as
    // AlbumName-START-END.txt. Otherwise fall back to AlbumName.txt
    let filename = if let (Some(s), Some(e)) = (passed_start_year, passed_end_year) {
        if s > 0 && e < 9999 && s <= e {
            format!("{}-{}-{}.txt", album_name.replace(' ', "-"), s, e)
        } else {
            format!("{}.txt", album_name.replace(' ', "-"))
        }
    } else {
        format!("{}.txt", album_name.replace(' ', "-"))
    };
    let output_path = album_dir.join(filename);
    let mut output = BufWriter::new(File::create(&output_path).expect("Failed to create output file"));

    // Write global album settings
    write_globals(&mut output, album_name);

    // Emit a single leading page header for Page 1
    let _ = writeln!(output, "### Page 1 ###");
    let _ = writeln!(output, "PAGE_START");
    let _ = writeln!(output, "ROW_START_ES (TN 6 0.1 5.0)");
    let _ = writeln!(output, "################################################################################");
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

        let series_list = get_all_series(&conn).expect("Failed to get series");
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
            let stamps = get_stamps_by_series(&conn, &series, "0")
                .expect("Failed to get stamps");
            // Diagnostic: write a short comment indicating how many stamps we fetched for this series.
            // Sanitize the series name to a single line (collapse newlines and extra spaces)
            // so downstream tools (or grep) can reliably parse this diagnostic.
            let safe_series = clean_text(series).replace('\n', " ").replace("  ", " ").replace('"', "'");
            let _ = writeln!(output, "# SERIES_FETCH series=\"{}\" count=\"{}\"", safe_series, stamps.len());
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
                attrs.set("perforation", &stamp.perforation);
                // Set image_path from StampStub
                stamp_objs.push(Stamp {
                    attrs,
                    image_path: stamp.image_path.clone(),
                });
            }
            // Group stamps into rows of 5
            let mut stamp_rows: Vec<Vec<Stamp>> = Vec::new();
            let mut row: Vec<Stamp> = Vec::new();
            // let mut row_width = 0.0;
            for stamp in stamp_objs.into_iter() {
                let image_path = stamp.image_path.trim();
                let full_image_path =
                    format!("Albums/{}/{}", album_name.replace(' ', "-"), image_path);
                // Estimate image physical size (mm) and scale to fit the configured stamp_box_height
                let (est_width_mm, _) = if let Some((w_mm, h_mm)) = estimate_real_dimensions(&full_image_path, mm_per_pixel_w, mm_per_pixel_h) {
                    let mut w = w_mm as f64;
                    let mut h = h_mm as f64;
                    // If the measured height is larger than the configured stamp box height
                    // shrink the image proportionally so it fits within stamp_box_height
                    if h > stamp_box_height {
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
                        let (w_mm, h_mm) = estimate_real_dimensions(
                            &full_img_path,
                            mm_per_pixel_w,
                            mm_per_pixel_h,
                        )
                        .unwrap_or((reference_width_mm as f32, reference_height_mm as f32));
                        // scale to stamp_box_height if needed
                        let mut w = w_mm as f64;
                        let h = h_mm as f64;
                        if h > stamp_box_height {
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
                        header_lines_vec.push((
                            format!("PAGE_TEXT_CENTRE (TN 8 \"{}\" 1.0)", name_line),
                            header_font_h,
                        ));
                    }
                    let perf_trimmed = String::new();
                    if !perf_trimmed.is_empty() {
                        header_lines_vec.push((
                            format!(
                                "PAGE_TEXT_CENTRE (TN 8 \"Perforation {}\" 1.0)",
                                perf_trimmed
                            ),
                            header_font_h,
                        ));
                    }
                    let first_stamp = stamp_rows.get(0).and_then(|row| row.get(0));
                    if let Some(stamp) = first_stamp {
                        let watermark = clean_text(
                            stamp
                                .attrs
                                .get("watermark")
                                .unwrap_or(&"".to_string())
                                .trim(),
                        )
                        .replace("\n", " ")
                        .replace("  ", " ");
                        if !watermark.is_empty() {
                            header_lines_vec.push((
                                format!("PAGE_TEXT_CENTRE (TN 8 \"Watermark: {}\" 1.0)", watermark),
                                header_font_h,
                            ));
                        }
                        let design_field = stamp
                            .attrs
                            .get("design")
                            .unwrap_or(&"".to_string())
                            .replace("\n", " ")
                            .replace("  ", " ");
                        let mut parts = design_field.split("Engraving:");
                        let design_raw = parts
                            .next()
                            .unwrap_or("")
                            .split("Perforation:")
                            .next()
                            .unwrap_or("")
                            .trim()
                            .to_string();
                        let engraving_raw = parts
                            .next()
                            .map(|p| p.split("Perforation:").next().unwrap_or("").trim().to_string())
                            .unwrap_or_default();
                        let design = clean_text(&design_raw);
                        let engraving = clean_text(&engraving_raw);
                        if !engraving.is_empty() {
                            header_lines_vec.push((
                                format!("PAGE_TEXT_CENTRE (TN 8 \"Engraving: {}\" 1.0)", engraving),
                                header_font_h,
                            ));
                        }
                        if !design.is_empty() {
                            header_lines_vec.push((
                                format!("PAGE_TEXT_CENTRE (TN 8 \"Design: {}\" 1.0)", design),
                                header_font_h,
                            ));
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
                        let est_height_mm = if let Some((_, h_mm)) = estimate_real_dimensions(&full_image_path, mm_per_pixel_w, mm_per_pixel_h) {
                            let mut h = h_mm as f64;
                            if h > stamp_box_height {
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
                            let total_height = adj_est_height + image_spacing + (text_lines as f64) * font_height + stamp_heading_h;
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
                        let _ = writeln!(output, "ROW_START_ES (TN 6 0.1 5.0)");
                        let _ = writeln!(output, "################################################################################");
                        current_height = 0.0;
                        rows_on_page = 0;
                        debug!("Forced page break before header and first row");
                        // forced page break: state tracked internally but not emitted
                    }
                    // Buffer the header lines (do not write yet). We'll write
                    // them only when we actually emit a row on this page so
                    // they don't get stranded on a previous page.
                    header_pending = Some((header_lines_vec, vspace_before_header, vspace_after_header));
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
                        let est_height_mm = if !image_path.is_empty() {
                            if let Some((_, h_mm)) = estimate_real_dimensions(
                                &full_image_path,
                                mm_per_pixel_w,
                                mm_per_pixel_h,
                            ) {
                                let mut h = h_mm as f64;
                                if h > stamp_box_height {
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
                    let next_stamps = get_stamps_by_series(&conn, next_series_name, "0").unwrap_or_default();
                    // Build header lines for the next series similarly to above
                    let mut next_header_lines_vec: Vec<(String, f64)> = Vec::new();
                    let next_name_line = clean_text(next_series_name.trim()).replace("\n", " ").replace("  ", " ");
                    let header_font_h = config.text_line_height_mm.unwrap_or(2.82) as f64;
                    if !next_name_line.is_empty() {
                        next_header_lines_vec.push((format!("PAGE_TEXT_CENTRE (TN 8 \"{}\" 1.0)", next_name_line), header_font_h));
                    }
                    // attempt to pull watermark/design from first stamp if present
                    if let Some(next_first_stamp) = next_stamps.get(0) {
                        let watermark = clean_text(next_first_stamp.watermark.trim()).replace("\n", " ").replace("  ", " ");
                        if !watermark.is_empty() {
                            next_header_lines_vec.push((format!("PAGE_TEXT_CENTRE (TN 8 \"Watermark: {}\" 1.0)", watermark), header_font_h));
                        }
                    }
                    let next_header_block_height: f64 = next_header_lines_vec.iter().map(|(_, h)| *h).sum();
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
                    if !nr.is_empty() { next_rows.push(nr); }
                    if let Some(first_row) = next_rows.get(0) {
                        for stamp in first_row {
                            let image_path = stamp.image_path.trim();
                            let full_image_path = format!("Albums/{}/{}", album_name.replace(' ', "-"), image_path);
                            let est_height_mm = if let Some((_, h_mm)) = estimate_real_dimensions(&full_image_path, mm_per_pixel_w, mm_per_pixel_h) {
                                let mut h = h_mm as f64;
                                if h > stamp_box_height { h = stamp_box_height; }
                                h
                            } else { reference_height_mm as f64 };
                            let text_lines = 1usize; // conservative minimal estimate
                            let text_height = (text_lines as f64) * header_font_h;
                            let adj_est_height = est_height_mm + config.album_stamp_img_new_setting_h_mm.unwrap_or(0.0) as f64 + 2.0 * stamp_box_adjust;
                            let total_height = adj_est_height + config.image_spacing_mm.unwrap_or(2.0) as f64 + text_height;
                            if total_height > next_first_row_max { next_first_row_max = total_height; }
                        }
                    } else {
                        // no stamps in next series; still account for a minimal row
                        next_first_row_max = stamp_box_height;
                    }
                    let next_total_header_and_row_height = next_vspace_before + next_header_block_height + next_vspace_after + next_first_row_max + row_font_spacing as f64;
                    // Include the global conservative buffer when peeking ahead
                    let next_conservative_needed = next_total_header_and_row_height + conservative_buffer;
                    // If emitting this row then the next series' header+first-row won't fit,
                    // force a page break now so the next series can start cleanly.
                    if current_height + total_row_height + next_conservative_needed > usable_height {
                        page_counter += 1;
                        let _ = writeln!(output, "\n################################################################################");
                        let _ = writeln!(output, "### Page {} ###", page_counter);
                        let _ = writeln!(output, "PAGE_START");
                        let _ = writeln!(output, "ROW_START_ES (TN 6 0.1 5.0)");
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
                if current_height + total_row_height > effective_usable || rows_on_page >= cfg_max_rows {
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
                            let _ = writeln!(output, "ROW_START_ES (TN 6 0.1 5.0)");
                            let _ = writeln!(output, "################################################################################");
                            current_height = 0.0;
                            header_written = false;
                            rows_on_page = 0;
                        } else if rows_on_page != 0 && (allow_single && percent_of_usable >= threshold) {
                            // We will allow the oversized row to be emitted as a
                            // single-row page: start a fresh page and let it emit.
                            page_counter += 1;
                            let _ = writeln!(output, "\n################################################################################");
                            let _ = writeln!(output, "### Page {} ###", page_counter);
                            let _ = writeln!(output, "PAGE_START");
                            let _ = writeln!(output, "ROW_START_ES (TN 6 0.1 5.0)");
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
                        let _ = writeln!(output, "ROW_START_ES (TN 6 0.1 5.0)");
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
                    let conservative_buffer_cfg = config.page_fit_conservative_buffer_mm.unwrap_or(11.0) as f64;
                    let header_block_height: f64 = pending_lines_ref.iter().map(|(_, h)| *h).sum();
                    let needed_if_emitted = v_before + header_block_height + v_after + total_row_height + conservative_buffer_cfg;
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
                            let _ = writeln!(output, "ROW_START_ES (TN 6 0.1 5.0)");
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
                        let _ = writeln!(output, "ROW_START_ES (TN 6 0.1 5.0)");
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
                    let _ = writeln!(output, "PAGE_VSPACE (6)");
                    current_height += v_before;
                    // header vspace written (diagnostics suppressed)
                    for (line, h) in pending_lines {
                        let _ = writeln!(output, "{}", line);
                        current_height += h;
                        // header line written (diagnostics suppressed)
                    }
                    let _ = writeln!(output, "PAGE_VSPACE (6)");
                    current_height += v_after;
                    // header vspace after written (diagnostics suppressed)
                    header_written = true;
                }
                let _ = writeln!(output, "ROW_START_ES (TN 6 0.1 5.0)");
                // row start written (diagnostics suppressed)
                for stamp in stamps_in_row {
                    // Use correct fields from StampStub
                    // Use image_path from the database directly
                    let image_path = stamp.image_path.trim();
                    let full_image_path =
                        format!("Albums/{}/{}", album_name.replace(' ', "-"), image_path);
                    // Estimate and scale final emitted width/height to fit stamp_box_height and include stamp_box_adjust margins
                    let (mut est_width_mm, mut est_height_mm) = estimate_real_dimensions(&full_image_path, mm_per_pixel_w, mm_per_pixel_h)
                        .map(|(w,h)| (w as f64, h as f64))
                        .unwrap_or((reference_width_mm as f64, reference_height_mm as f64));
                    if est_height_mm > stamp_box_height {
                        let s = stamp_box_height / est_height_mm;
                        est_width_mm *= s;
                        est_height_mm *= s;
                    }
                    // Apply ALBUM_STAMP_IMG_NEW_SETTING adjustments to width/height (additive)
                    let img_new_w = config.album_stamp_img_new_setting_w_mm.unwrap_or(0.0) as f64;
                    let img_new_h = config.album_stamp_img_new_setting_h_mm.unwrap_or(0.0) as f64;
                    est_width_mm += img_new_w;
                    est_height_mm += img_new_h;
                    // add per-side adjust to the emitted box width/height (STAMP_BOXES_SIZE_ADJUST)
                    let emit_width = est_width_mm + 2.0 * stamp_box_adjust;
                    let emit_height = est_height_mm + 2.0 * stamp_box_adjust;
                    // Left: sg_number
                    let left =  clean_text(stamp.attrs.get("denomination").unwrap_or(&"".to_string()));
                    // let sgnum = clean_text(stamp.attrs.get("sg_number").unwrap_or(&"".to_string()));
                    // Centre: color, description, and stamps_issued (counts are shown here when present)
                    let centre = format!(
                        "{}\\n{}\\n{}",
                        clean_text(stamp.attrs.get("color").unwrap_or(&"".to_string())),
                        clean_text(stamp.attrs.get("description").unwrap_or(&"".to_string())),
                        clean_text(stamp.attrs.get("stamps_issued").unwrap_or(&"".to_string()))
                    );
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
                        let v = clean_text(stamp.attrs.get(key).unwrap_or(&"".to_string())).trim().to_string();
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
                    let first_line = v_mint.or(v_unused.clone()).unwrap_or_else(|| "".to_string());
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
                        let _ = writeln!(output, "STAMP_HEADING(HN 10 \"SG : {}\")", sgnum);
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
                            emit_width,
                            emit_height,
                            left,
                            centre,
                            right
                        );
                        
                        let _ = writeln!(output, "{}", line.replace("\r", "").replace("\n", "\\n"));
                        let _ = writeln!(output, "STAMP_HEADING(HN 10 \"SG : {}\")", sgnum);
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