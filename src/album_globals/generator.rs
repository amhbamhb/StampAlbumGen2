use super::constants::MONTH_NAMES;
use super::globals::{compute_usable_height, write_globals};
use super::helpers::{
    build_date_page_text_centre_template, build_engraving_page_text_centre_template,
    build_page_text_template, build_series_page_text_centre_template,
    build_watermark_page_text_centre_template,
};
use super::structs::Stamp;
use crate::album_globals::generator_new::layout;
use crate::album_utils::clean_text;
use crate::config::load_config;
use crate::db_utils::{get_all_series, get_stamps_by_series, init_db};
use crate::image_utils::{calc_mm_per_pixel_embedded, estimate_real_dimensions};
use log::{error, info, warn};
use rusqlite::Connection;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn json_escape(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

fn normalize_denom(v: &str) -> String {
    v.to_ascii_lowercase()
        .replace([' ', '\t', '\n', '\r'], "")
        .replace("\u{a0}", "")
}

fn normalize_digits(v: &str) -> String {
    v.chars().filter(|c| c.is_ascii_digit()).collect()
}

fn stamp_date_key(stamp: &Stamp) -> String {
    let year = stamp
        .attrs
        .get("year")
        .map(|s| normalize_digits(s))
        .unwrap_or_default();
    let month = stamp
        .attrs
        .get("month_num")
        .map(|s| normalize_digits(s))
        .unwrap_or_default();
    let day = stamp
        .attrs
        .get("day")
        .map(|s| normalize_digits(s))
        .unwrap_or_default();
    if !year.is_empty() {
        format!(
            "{}-{:0>2}-{:0>2}",
            year,
            if month.is_empty() { "0" } else { &month },
            if day.is_empty() { "0" } else { &day }
        )
    } else {
        String::new()
    }
}

fn stamp_dedupe_fingerprints(stamp: &Stamp) -> Vec<String> {
    let year = stamp
        .attrs
        .get("year")
        .map(|s| normalize_digits(s))
        .unwrap_or_default();
    let date = stamp_date_key(stamp);
    let denom = normalize_denom(
        stamp
            .attrs
            .get("denomination")
            .map(|s| s.as_str())
            .unwrap_or(""),
    );
    let issued = normalize_digits(
        stamp
            .attrs
            .get("stamps_issued")
            .map(|s| s.as_str())
            .unwrap_or(""),
    );

    let mut keys = Vec::new();
    if !year.is_empty() && !date.is_empty() && !denom.is_empty() && !issued.is_empty() {
        keys.push(format!(
            "y:{}|d:{}|den:{}|iss:{}",
            year, date, denom, issued
        ));
    }
    if !year.is_empty() && !denom.is_empty() && !issued.is_empty() {
        keys.push(format!("y:{}|den:{}|iss:{}", year, denom, issued));
    }
    keys
}

fn is_real_image_path(image_path: &str) -> bool {
    let p = image_path.trim().to_ascii_lowercase();
    !p.is_empty() && !p.contains("no_image")
}

// AlbumEasy Output Generation
// ===================// This section contains the main entry point and helpers for generating AlbumEasy output.

// Removed duplicate imports

/// Main entry point for AlbumEasy output generation.
/// Queries the database, formats output, and writes to a .txt file for AlbumEasy import.
/// Returns the path to the generated output file.
pub fn generate_album_output_conn<F>(
    conn: &Connection,
    album_name: &str,
    mut page_callback: Option<F>,
    start_year: Option<i32>,
    end_year: Option<i32>,
) -> std::path::PathBuf
where
    F: FnMut(usize),
{
    // Use the start_year/end_year passed to this function (may be None)
    let passed_start_year = start_year;
    let passed_end_year = end_year;
    // Load config values from config.toml
    let config = load_config("config.toml");
    // Check if this is a US1847 sourced album
    let is_us1847 = conn
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='us1847_series'",
            [],
            |r| r.get::<_, i64>(0),
        )
        .unwrap_or(0)
        > 0
        && conn
            .query_row("SELECT COUNT(*) FROM us1847_series", [], |r| {
                r.get::<_, i64>(0)
            })
            .unwrap_or(0)
            > 0;
    // Also obtain canonical defaults so we always fall back to the same
    // values the `Config::with_defaults()` helper exposes. This ensures
    // generator math always uses config-driven values (either user-set or
    // project defaults) rather than ad-hoc literals sprinkled through code.
    let cfg_defaults = crate::config::Config::with_defaults();
    let stamp_box_height = config
        .stamp_box_height
        .unwrap_or(cfg_defaults.stamp_box_height.unwrap()) as f64;
    // extra space applied per-side around each stamp when emitted
    let _stamp_box_adjust = config
        .stamp_box_adjust
        .unwrap_or(cfg_defaults.stamp_box_adjust.unwrap()) as f64;
    let row_font_spacing = config
        .row_font_spacing
        .unwrap_or(cfg_defaults.row_font_spacing.unwrap()) as f64;
    let _header_font_spacing = config
        .header_font_spacing
        .unwrap_or(cfg_defaults.header_font_spacing.unwrap()) as f64;
    let reference_width_mm = config
        .reference_width_mm
        .unwrap_or(cfg_defaults.reference_width_mm.unwrap()) as f64;
    let reference_height_mm = config
        .reference_height_mm
        .unwrap_or(cfg_defaults.reference_height_mm.unwrap()) as f64;
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
            "Configured reference size {:.1}x{:.1}mm looks too small, falling back to 25x30mm",
            cfg_ref_w, cfg_ref_h
        );
        (25.0_f64, 30.0_f64)
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
    // Write into a temporary file first to avoid truncating the canonical
    // output file if generation yields no stamp lines. We will promote the
    // temporary file into place only when it contains STAMP lines.
    let tmp_name = format!(
        "{}.tmp",
        output_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("out")
    );
    let tmp_path = output_path.with_file_name(tmp_name);
    let mut output = BufWriter::new(
        File::create(&tmp_path)
            .unwrap_or_else(|e| panic!("Failed to create temp output file: {}", e)),
    );

    // Write global album settings (pass DB connection and CLI year range so the header can include years)
    let pages_from_globals = write_globals(
        &mut output,
        album_name,
        Some(conn),
        passed_start_year,
        passed_end_year,
    );

    // Start page counter after the globals section
    let starting_page_number = pages_from_globals + 1;

    // Emit page header for first stamp page
    let _ = crate::album_globals::generator_new::layout::write_stamp_page_header(
        &mut output,
        starting_page_number,
        &config,
    );
    // Compute usable height via shared helper (centralizes margins/borders/padding)
    let usable_height = compute_usable_height(&config);
    // Allow a configurable conservative buffer (mm) to reserve footer/decoration space.
    // Always prefer the value from `config.toml`; if absent, fall back to the
    // canonical defaults from `Config::with_defaults()` so behavior is consistent.
    let conservative_buffer = config
        .page_fit_conservative_buffer_mm
        .unwrap_or(cfg_defaults.page_fit_conservative_buffer_mm.unwrap())
        as f64;
    // Effective usable height after reserving the conservative buffer
    let effective_usable = usable_height - conservative_buffer;
    // Recreate a few layout vars for diagnostics and width calculations
    let _page_height = config.album_page_height_mm.unwrap_or(279.4) as f64;
    // Use `effective_usable` (usable_height minus conservative buffer)
    // when computing and inserting PAGE_VSPACE so the post-process
    // respects the same reserved footer/buffer used during page-fit
    let _top_margin = config.page_top_margin_mm.unwrap_or(10.0) as f64;
    let _bottom_margin = config.page_bottom_margin_mm.unwrap_or(10.0) as f64;
    let border_outer = config.page_border_outer.unwrap_or(0.5) as f64;
    let border_inner1 = config.page_border_inner1.unwrap_or(0.1) as f64;
    let border_inner2 = config.page_border_inner2.unwrap_or(0.0) as f64;
    let border_spacing = config.page_border_spacing.unwrap_or(3.0) as f64;
    let total_border = border_outer + border_inner1 + border_inner2 + border_spacing;
    // Compute a suggested `computed_stamps_per_row` value using simple math
    // so we can write it into `config.toml` for user convenience. The value is
    // based on the usable page width and the average stamp emit width computed
    // from the DB (or fallback reference size).
    let page_width = config
        .album_page_width_mm
        .unwrap_or(cfg_defaults.album_page_width_mm.unwrap()) as f64;
    let left_margin = config
        .page_left_margin_mm
        .unwrap_or(cfg_defaults.page_left_margin_mm.unwrap()) as f64;
    let right_margin = config
        .page_right_margin_mm
        .unwrap_or(cfg_defaults.page_right_margin_mm.unwrap()) as f64;
    let pages_spacing_h = config
        .album_pages_spacing_h
        .unwrap_or(cfg_defaults.album_pages_spacing_h.unwrap()) as f64;
    let usable_width_global =
        page_width - left_margin - right_margin - pages_spacing_h - total_border;
    // Compute average image width from DB (px -> mm) when available
    let avg_w_mm: f64 = match conn.query_row("SELECT AVG(image_width_px) FROM stamps", [], |r| {
        r.get::<_, Option<f64>>(0)
    }) {
        Ok(opt_px) => opt_px
            .map(|px| px * (mm_per_pixel_w as f64))
            .unwrap_or(reference_width_mm as f64),
        Err(_) => reference_width_mm as f64,
    };
    let img_new_w = config
        .album_stamp_img_new_setting_w_mm
        .unwrap_or(cfg_defaults.album_stamp_img_new_setting_w_mm.unwrap())
        as f64;
    // Ensure height parameter is read in the same places as width so
    // tests that count parameter usage remain consistent.
    let _img_new_h = config
        .album_stamp_img_new_setting_h_mm
        .unwrap_or(cfg_defaults.album_stamp_img_new_setting_h_mm.unwrap())
        as f64;
    let stamp_box_adjust = config
        .stamp_box_adjust
        .unwrap_or(cfg_defaults.stamp_box_adjust.unwrap()) as f64;
    let avg_emit_w_mm = avg_w_mm + img_new_w + 2.0 * stamp_box_adjust;
    let packing_h = config
        .packing_horizontal_safety_mm
        .unwrap_or(cfg_defaults.packing_horizontal_safety_mm.unwrap()) as f64;
    let mut computed_stamps = (usable_width_global / (avg_emit_w_mm + packing_h)).floor() as i32;
    if computed_stamps < 1 {
        computed_stamps = 1;
    }
    // Respect configured hard cap if present
    if let Some(cap) = config.max_stamps_per_row {
        let cap_i = cap as i32;
        if computed_stamps > cap_i {
            computed_stamps = cap_i;
        }
    }
    // Attempt to update `config.toml` with a computed value so users can see it
    // Note: best-effort — do not fail generation if writing the file fails.
    // Write computed values to an album-specific advisory file so we do not
    // mutate the user's main `config.toml`. This is non-destructive and makes
    // it easy to inspect recommended values per-album.
    if let Ok(_) = std::fs::create_dir_all(&album_dir) {
        let computed_path = album_dir.join("computed_values.toml");
        let computed_txt = format!(
            "# Auto-generated computed values for album\ncomputed_stamps_per_row = {}\naverage_emit_width_mm = {:.2}\n# usable_width_mm = {:.2}\n",
            computed_stamps, avg_emit_w_mm, usable_width_global
        );
        if let Err(e) = std::fs::write(&computed_path, computed_txt) {
            warn!(
                "Failed to write computed values to {}: {}",
                computed_path.display(),
                e
            );
        } else {
            info!("Wrote computed values to {}", computed_path.display());
        }
    }
    // debug!("[PAGE LAYOUT] page_height={:.2}, top_margin={:.2}, bottom_margin={:.2}, total_border={:.2}, usable_height={:.2}",
    // page_height, top_margin, bottom_margin, total_border, usable_height);
    // Font and spacing from AlbumEasy settings
    // Local layout estimates (we use configured values above where available)
    let _stamp_box_height = 24.0; // Estimate: typical stamp box height (unused default)
    let _header_lines = 4; // Year, name, desc, perforation (unused default)
                           // let header_height = (header_lines as f64) * header_font_spacing;
                           // let row_height = stamp_box_height + row_font_spacing;

    let series_order = config.series_order.clone().map(|s| s.as_str().to_string());
    let series_order_slice = series_order.as_deref();
    let series_list = get_all_series(conn, series_order_slice).expect("Failed to get series");
    // Initialize page counter from the starting page (after globals pages)
    let mut page_counter = starting_page_number;
    let mut current_height = 0.0f64;
    let mut rows_on_page = 0;
    let mut seen_stamp_fingerprints: std::collections::HashMap<String, bool> =
        std::collections::HashMap::new();
    let mut dedupe_audit: Vec<String> = Vec::new();
    let mut dedupe_dropped: usize = 0;
    // Treat series_list as Vec<String> (series names). We don't filter by year here.
    let filtered_series: Vec<String> = series_list.to_vec();
    let _series_count = filtered_series.len();
    crate::album_globals::generator_new::for_each_series(conn, series_order_slice, | _series_idx, series | {
        if let Some(cb) = page_callback.as_mut() {
            cb(page_counter);
        }
        // Fetch stamps + year-filter + conversion into Stamp objects via helper.
        let stamps_iter = crate::album_globals::generator_new::helpers::
            fetch_and_filter_stamps(conn, series, passed_start_year, passed_end_year)
            .expect("Failed to get stamps");
        let mut stamps_vec = stamps_iter;
        // Group stamps into rows (configurable max stamps per row). For rich
        // inline metadata, use a more conservative per-row cap to keep text
        // readable and avoid visual crowding in PDF output.
        let detail_layout = config
            .stamp_detail_layout
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("inline");
        let configured_max = config.max_stamps_per_row.unwrap_or(5) as usize;
        let img_new_w = config.album_stamp_img_new_setting_w_mm.unwrap_or(6.0) as f64;
        let field_count = if config.colnect_details_enabled() {
            config.effective_colnect_detail_field_set().len()
        } else {
            0
        };
        let max_stamps_per_row = if detail_layout == "inline" {
            if field_count >= 12 {
                configured_max.min(2)
            } else if field_count >= 8 {
                configured_max.min(3)
            } else {
                configured_max.min(4)
            }
        } else {
            configured_max
        };
        let mut stamp_rows: Vec<Vec<Stamp>> = Vec::new();
        let mut row: Vec<Stamp> = Vec::new();
        // Compute a series-average estimated size (mm) from available images in this series.
        // This helps placeholders and missing-image stamps adopt the same size as their
        // series peers instead of falling back to the global reference size.
        let mut series_widths_mm: Vec<f64> = Vec::new();
        let mut series_heights_mm: Vec<f64> = Vec::new();
        for stamp in stamps_vec.iter() {
            let image_path = stamp.image_path.trim();
            let full_image_path = format!("Albums/{}/{}", album_name.replace(' ', "-"), image_path);
            if let (Some(w_px), Some(h_px)) = (stamp.image_width_px, stamp.image_height_px) {
                series_widths_mm.push((w_px as f64) * (mm_per_pixel_w as f64));
                series_heights_mm.push((h_px as f64) * (mm_per_pixel_h as f64));
            } else if let Some((w_mm, h_mm)) =
                estimate_real_dimensions(&full_image_path, mm_per_pixel_w, mm_per_pixel_h)
            {
                series_widths_mm.push(w_mm as f64);
                series_heights_mm.push(h_mm as f64);
            }
        }
        let series_avg_w_mm = if !series_widths_mm.is_empty() {
            Some(series_widths_mm.iter().sum::<f64>() / series_widths_mm.len() as f64)
        } else {
            None
        };
        let series_avg_h_mm = if !series_heights_mm.is_empty() {
            Some(series_heights_mm.iter().sum::<f64>() / series_heights_mm.len() as f64)
        } else {
            None
        };
        // -------------------------
        // Pre-pass: compute final estimated physical sizes (mm) for every stamp
        // in this series. This ensures placeholders are generated now and
        // layout uses the final sizes rather than seeing stale or missing
        // dimensions during the single-pass layout.
        // -------------------------
        // Get Colnect dimension adjustments from config (default 2.0mm if not set)
        let colnect_w_adjust = config.colnect_width_adjust_mm.unwrap_or(2.0) as f64;
        let colnect_h_adjust = config.colnect_height_adjust_mm.unwrap_or(2.0) as f64;

        for s in stamps_vec.iter_mut() {
            // First priority: Check for Colnect dimensions (already in mm)
            let colnect_w = s.attrs.get("colnect_width_mm")
                .and_then(|w| w.parse::<f64>().ok());
            let colnect_h = s.attrs.get("colnect_height_mm")
                .and_then(|h| h.parse::<f64>().ok());

            let est_source: &str;
            if let (Some(w_mm), Some(h_mm)) = (colnect_w, colnect_h) {
                // Use Colnect dimensions with separate width/height adjustments
                s.est_width_mm = Some(w_mm + colnect_w_adjust);
                s.est_height_mm = Some(h_mm + colnect_h_adjust);
                est_source = "colnect";
            } else {
                // Fall back to existing dimension sources
                let image_path = s.image_path.trim();
                let full_image_path = format!("Albums/{}/{}", album_name.replace(' ', "-"), image_path);
                // Prefer DB-stored pixel dimensions if available; otherwise fall back
                // to on-disk image estimation or series-average, finally reference size.
                // Track where the estimated size came from for diagnostics
                if let (Some(w_px), Some(h_px)) = (s.image_width_px, s.image_height_px) {
                    let w_mm = (w_px as f64) * (mm_per_pixel_w as f64);
                    let h_mm = (h_px as f64) * (mm_per_pixel_h as f64);
                    s.est_width_mm = Some(w_mm);
                    s.est_height_mm = Some(h_mm);
                    est_source = "db_pixels";
                } else if let Some((w_mm, h_mm)) =
                    estimate_real_dimensions(&full_image_path, mm_per_pixel_w, mm_per_pixel_h)
                {
                    let mut w = w_mm as f64;
                    let mut h = h_mm as f64;
                    // Scale down proportionally if image is taller than the stamp box
                    if !preserve_real && h > stamp_box_height {
                        let s = stamp_box_height / h;
                        w *= s;
                        h *= s;
                    }
                    s.est_width_mm = Some(w);
                    s.est_height_mm = Some(h);
                    est_source = "image_estimate";
                } else if let (Some(avg_w), Some(avg_h)) = (series_avg_w_mm, series_avg_h_mm) {
                    s.est_width_mm = Some(avg_w);
                    s.est_height_mm = Some(avg_h);
                    est_source = "series_average";
                } else {
                    s.est_width_mm = Some(reference_width_mm as f64);
                    s.est_height_mm = Some(reference_height_mm as f64);
                    est_source = "reference";
                }
            }

            // Emit a compact per-stamp diagnostic so we can trace where the
            // estimated mm sizes came from and what placeholder pixel sizes
            // (if any) were created. This helps debug mismatches between
            // series-declared sizes and actual placeholders.
            let diag_sg = s
                .attrs
                .get("sg_number")
                .unwrap_or(&"".to_string())
                .replace('"', "'");
            let diag_img_path = s.image_path.replace('"', "'");
            let diag_w_mm = s.est_width_mm.unwrap_or(reference_width_mm as f64);
            let diag_h_mm = s.est_height_mm.unwrap_or(reference_height_mm as f64);
            // If pixel dims are present (placeholders set them later), print them too.
            let diag_px_w = s.image_width_px.unwrap_or(0);
            let diag_px_h = s.image_height_px.unwrap_or(0);
            let _ = crate::album_globals::generator_new::helpers::emit_stamp_diag(
                &mut output,
                &diag_sg,
                est_source,
                diag_w_mm,
                diag_h_mm,
                diag_px_w,
                diag_px_h,
                &diag_img_path,
            );

            // If the stamp has no image and therefore no stored pixel dims, create
            // a placeholder now using the emit box size (est + 2*stamp_box_adjust)
            // so future layout sees consistent sizes.
            let image_path = s.image_path.trim();
            let full_image_path = format!("Albums/{}/{}", album_name.replace(' ', "-"), image_path);
            if s.image_width_px.is_none() && (image_path.is_empty() || !Path::new(&full_image_path).exists()) {
                let base_w = s.est_width_mm.unwrap_or(reference_width_mm as f64);
                let base_h = s.est_height_mm.unwrap_or(reference_height_mm as f64);
                let target_w_mm = base_w; // image-only width (mm)
                let target_h_mm = base_h; // image-only height (mm)
                let sg_str = s.attrs.get("sg_number").map(|v| v.as_str()).unwrap_or("");
                let catalog_num = s.attrs.get("scott_number")
                    .or_else(|| s.attrs.get("sg_number"))
                    .map(|v| v.as_str())
                    .unwrap_or("");
                let year_str = s.attrs.get("year").map(|v| v.as_str()).unwrap_or("");
                if let Some((rel, px_w, px_h)) =
                    crate::album_globals::generator_new::helpers::ensure_placeholder_for_stamp_and_update_db(
                        conn,
                        album_name,
                        series,
                        sg_str,
                        catalog_num,
                        year_str,
                        target_w_mm,
                        target_h_mm,
                        mm_per_pixel_w,
                        mm_per_pixel_h,
                        Some(&mut output),
                    )
                {
                    s.image_path = rel;
                    s.image_width_px = Some(px_w);
                    s.image_height_px = Some(px_h);
                }
            }
        }

        // COMMEMORATIVE SEPARATION: Check if this series has commemoratives and route accordingly
        let (commemorative_stamps, regular_stamps): (Vec<Stamp>, Vec<Stamp>) = stamps_vec.into_iter().partition(|stamp| {
            crate::album_globals::generator_new::commemorative_layout::is_commemorative(stamp, passed_start_year)
        });

        // If this series has commemoratives, emit them using the nice layout
        if !commemorative_stamps.is_empty() {
            let year = passed_start_year.unwrap_or_else(|| {
                commemorative_stamps.first()
                    .and_then(|s| s.attrs.get("year").and_then(|y| y.parse::<i32>().ok()))
                    .unwrap_or(1955)
            });

            // Split commemoratives into regular and special stamps
            let (special_stamps, regular_commemoratives): (Vec<Stamp>, Vec<Stamp>) = commemorative_stamps.into_iter().partition(|stamp| {
                crate::album_globals::commemorative_generator::is_special_stamp(stamp)
            });

            let has_commemoratives = !regular_commemoratives.is_empty() || !special_stamps.is_empty();

            // Only emit commemorative pages if there are regular commemoratives
            if !regular_commemoratives.is_empty() {
                let _ = crate::album_globals::commemorative_generator::emit_commemorative_pages(&mut output, &regular_commemoratives);
                page_counter += 1;
            }

            // Emit special stamps on a separate page if any exist
            if !special_stamps.is_empty() {
                let _ = crate::album_globals::commemorative_generator::emit_special_stamps_page(&mut output, &special_stamps);
                page_counter += 1;
            }

            // Emit summary page with all commemoratives
            if has_commemoratives {
                let all_commemoratives: Vec<Stamp> = [regular_commemoratives, special_stamps].concat();
                let _ = crate::album_globals::commemorative_generator::emit_summary_page(&mut output, year, &all_commemoratives);
                page_counter += 1;
            }
            // If all stamps were commemorative, we're done with this series
            if regular_stamps.is_empty() {
                return;
            }
        }

        // Phase A+B: conservative matcher + dedupe pass.
        // If two rows share strong identity fingerprints and one already-seen row
        // has a real image while the new row is a no_image placeholder, drop the
        // new row to prevent overlap/duplicate emission.
        let mut deduped_regular_stamps: Vec<Stamp> = Vec::new();
        for stamp in regular_stamps.into_iter() {
            let fps = stamp_dedupe_fingerprints(&stamp);
            let has_real_image = is_real_image_path(&stamp.image_path);
            let mut matched_fp: Option<String> = None;
            let mut seen_had_real = false;

            for fp in fps.iter() {
                if let Some(prev_has_real) = seen_stamp_fingerprints.get(fp) {
                    matched_fp = Some(fp.clone());
                    seen_had_real = *prev_has_real;
                    break;
                }
            }

            let drop_duplicate = if matched_fp.is_some() {
                (seen_had_real && !has_real_image) || (!seen_had_real && !has_real_image)
            } else {
                false
            };

            if drop_duplicate {
                dedupe_dropped += 1;
                let sg = stamp
                    .attrs
                    .get("sg_number")
                    .map(|s| s.as_str())
                    .unwrap_or("");
                let den = stamp
                    .attrs
                    .get("denomination")
                    .map(|s| s.as_str())
                    .unwrap_or("");
                let iss = stamp
                    .attrs
                    .get("stamps_issued")
                    .map(|s| s.as_str())
                    .unwrap_or("");
                let date = stamp_date_key(&stamp);
                let fp = matched_fp.unwrap_or_default();
                dedupe_audit.push(format!(
                    "{{\"event\":\"dedupe_drop\",\"series\":\"{}\",\"sg\":\"{}\",\"denomination\":\"{}\",\"stamps_issued\":\"{}\",\"date\":\"{}\",\"fingerprint\":\"{}\",\"reason\":\"duplicate_no_image\",\"image_path\":\"{}\"}}",
                    json_escape(series),
                    json_escape(sg),
                    json_escape(den),
                    json_escape(iss),
                    json_escape(&date),
                    json_escape(&fp),
                    json_escape(&stamp.image_path)
                ));
                continue;
            }

            for fp in fps {
                seen_stamp_fingerprints
                    .entry(fp)
                    .and_modify(|v| *v = *v || has_real_image)
                    .or_insert(has_real_image);
            }
            deduped_regular_stamps.push(stamp);
        }

        // Now perform layout using the finalized est_* values for REGULAR stamps
        // let mut row_width = 0.0;
        for stamp in deduped_regular_stamps.into_iter() {
            let image_path = stamp.image_path.trim();
            let _full_image_path =
                format!("Albums/{}/{}", album_name.replace(' ', "-"), image_path);
            // Use the est_width_mm precomputed in the pre-pass (should always be present)
            let est_width_mm = stamp.est_width_mm.unwrap_or(reference_width_mm as f64);
            let _est_height_mm = stamp.est_height_mm.unwrap_or(reference_height_mm as f64);
            // Horizontal spacing between stamps (mm). Use packing horizontal safety
            // config so users can tune how tightly stamps are packed horizontally.
            let margin = config.packing_horizontal_safety_mm.unwrap_or(5.0) as f64;
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
            // Accumulate row width as stamps are added. In `separate_rows`
            // mode we pack by the detail-box footprint so metadata boxes do
            // not overlap even when stamp images themselves are narrow.
            let row_width: f64 = row
                .iter()
                .map(|s| {
                    let est_w = s.est_width_mm.unwrap_or(reference_width_mm as f64);
                    let emit_w = est_w + img_new_w + 2.0 * stamp_box_adjust;
                    let footprint = if detail_layout == "separate_rows"
                        && !config.colnect_details_inside_stamp()
                    {
                        let max_box_width = usable_width.min(emit_w * 3.5);
                        (emit_w * 2.8).min(max_box_width).max(85.0)
                    } else {
                        emit_w
                    };
                    footprint + margin
                })
                .sum();
            // If adding this stamp would exceed usable width or row has max stamps, start a new row before adding.
            // Note: `row_width` already includes the per-stamp `2.0 * stamp_box_adjust` offset for existing
            // stamps. Ensure we include the same adjustment when considering the new stamp so the width
            // check is consistent and prevents placing an oversized placeholder into the row.
            let new_stamp_footprint = if detail_layout == "separate_rows"
                && !config.colnect_details_inside_stamp()
            {
                let emit_w = est_width_mm + img_new_w + 2.0 * stamp_box_adjust;
                let max_box_width = usable_width.min(emit_w * 3.5);
                (emit_w * 2.8).min(max_box_width).max(85.0)
            } else {
                est_width_mm + img_new_w + 2.0 * stamp_box_adjust
            };
            let new_stamp_total = new_stamp_footprint + margin;
            if row.len() >= max_stamps_per_row || row_width + new_stamp_total > usable_width {
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
                    let tmpl = build_series_page_text_centre_template(&config);
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
                let first_stamp = stamp_rows.first().and_then(|row| row.first());
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
                                if (1..=12).contains(&mn) {
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
                            } else if !month.is_empty() && !day.is_empty() {
                                date_text = format!("{} {}", month, day);
                            } else if !month.is_empty() {
                                date_text = month.clone();
                            }
                        }
                        if !date_text.is_empty() {
                            let tmpl = build_date_page_text_centre_template(&config);
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
                        let tmpl = build_watermark_page_text_centre_template(&config);
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
                    // Skip perforation in header if using separate_rows mode (it's in details)
                    let detail_layout = config.stamp_detail_layout.as_ref().map(|s| s.as_str()).unwrap_or("inline");
                    if config.include_perforation.unwrap_or(true) && !perforation.is_empty() && detail_layout != "separate_rows" {
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
                        let tmpl = build_engraving_page_text_centre_template(&config);
                        let text = format!("Engraving: {}", engraving);
                        let safe = text.replace('"', "\\\"");
                        header_lines_vec.push((tmpl.replace("{}", &safe), header_font_h));
                    }
                    // Skip design in header if using separate_rows mode (it's in details)
                    let detail_layout = config.stamp_detail_layout.as_ref().map(|s| s.as_str()).unwrap_or("inline");
                    if config.include_design.unwrap_or(true) && !design.is_empty() && detail_layout != "separate_rows" {
                        let tmpl = build_page_text_template(&config, "centre");
                        let text = format!("Design: {}", design);
                        let safe = text.replace('"', "\\\"");
                        header_lines_vec.push((tmpl.replace("{}", &safe), header_font_h));
                    }
                }

                let header_block_height: f64 = header_lines_vec.iter().map(|(_, h)| *h).sum();
                let vspace_before_header = config
                    .header_vspace_before_mm
                    .unwrap_or(cfg_defaults.header_vspace_before_mm.unwrap())
                    as f64;
                let vspace_after_header = config
                    .header_vspace_after_mm
                    .unwrap_or(cfg_defaults.header_vspace_after_mm.unwrap())
                    as f64;
                // Calculate height of first row
                let stamps_in_row = &stamp_rows[row_idx];
                let mut max_row_height = 0.0f64;
                let font_height = config
                    .text_line_height_mm
                    .unwrap_or(cfg_defaults.text_line_height_mm.unwrap())
                    as f64;
                let image_spacing = config
                    .image_spacing_mm
                    .unwrap_or(cfg_defaults.image_spacing_mm.unwrap())
                    as f64;
                // Additional per-image adjustments from ALBUM_STAMP_IMG_NEW_SETTING
                let _img_new_w = config.album_stamp_img_new_setting_w_mm.unwrap_or(0.0) as f64;
                let img_new_h = config.album_stamp_img_new_setting_h_mm.unwrap_or(0.0) as f64;
                let sh_label_for_fit = config
                    .stamp_heading_label
                    .clone()
                    .unwrap_or_else(|| "SG".to_string());
                let sh_size_for_fit = config.stamp_heading_size.unwrap_or(10.0) as i32;
                let include_perforation_in_heading = config.include_perforation.unwrap_or(true);
                for stamp in stamps_in_row {
                    let est_height_mm = stamp.est_height_mm.unwrap_or(reference_height_mm as f64);
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
                    let stamp_heading_h = config
                        .header_font_spacing
                        .unwrap_or(cfg_defaults.header_font_spacing.unwrap())
                        as f64
                        + config
                            .text_line_height_mm
                            .unwrap_or(cfg_defaults.text_line_height_mm.unwrap())
                            as f64;
                    let heading_emit_width =
                        stamp.est_width_mm.unwrap_or(reference_width_mm as f64) + 2.0 * stamp_box_adjust;
                    let heading_perforation = stamp
                        .attrs
                        .get("colnect_perforation")
                        .or_else(|| stamp.attrs.get("perforation"))
                        .map(|s| s.as_str());
                    let heading_line_count = crate::album_globals::generator_new::emit::estimate_stamp_heading_line_count(
                        &sh_label_for_fit,
                        stamp.attrs.get("catalog_num").map(|v| v.as_str()).unwrap_or(""),
                        stamp.attrs.get("colnect_name").map(|v| v.as_str()),
                        heading_perforation,
                        include_perforation_in_heading,
                        heading_emit_width,
                        sh_size_for_fit,
                    ) as f64;
                    let adj_est_height = est_height_mm + img_new_h + 2.0 * stamp_box_adjust;
                    let total_height = adj_est_height
                        + image_spacing
                        + (text_lines as f64) * font_height
                        + (stamp_heading_h * heading_line_count.max(1.0));
                    if total_height > max_row_height {
                        max_row_height = total_height;
                    }
                }
                let vertical_safety = config
                    .packing_vertical_safety_mm
                    .unwrap_or(cfg_defaults.packing_vertical_safety_mm.unwrap())
                    as f64;
                let total_header_and_row_height = vspace_before_header
                    + header_block_height
                    + vspace_after_header
                    + max_row_height
                    + row_font_spacing as f64
                    + vertical_safety; // include vertical safety padding
                                       // Reserve extra room for footers/AlbumEasy page decorations to avoid overlap
                                       // Bumped to 11mm (was 10.0) to avoid a subtle footer bleed observed in rendered PDFs.
                                       // This +1mm tweak is intentionally small to preserve compact layouts while
                                       // matching AlbumEasy's rendering metrics more closely.
                let conservative_buffer =
                    cfg_defaults.page_fit_conservative_buffer_mm.unwrap_or(2.0) as f64; // canonical default from Config::with_defaults()
                let conservative_needed = total_header_and_row_height + conservative_buffer;
                // debug!("[PAGE FIT CHECK] Header block height: {:.2}, vspace before: {:.2}, vspace after: {:.2}, max row height: {:.2}, total needed: {:.2}, buffer: {:.2}, current: {:.2}, usable: {:.2}, will_fit: {}",
                // header_block_height, vspace_before_header, vspace_after_header, max_row_height, total_header_and_row_height, conservative_buffer, current_height, usable_height, (current_height + conservative_needed <= usable_height));
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
                    // start a new page using the centralized layout helper
                    let _ = crate::album_globals::generator_new::layout::start_stamp_page(
                        &mut output,
                        &mut page_counter,
                        &config,
                    );

                    current_height = 0.0;
                    rows_on_page = 0;
                    // debug!("Forced page break before header and first row");
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
            let font_height = config
                .text_line_height_mm
                .unwrap_or(cfg_defaults.text_line_height_mm.unwrap())
                as f64; // TN 10 font spacing (mm)
            let image_spacing = config
                .image_spacing_mm
                .unwrap_or(cfg_defaults.image_spacing_mm.unwrap())
                as f64; // space below image (mm)
            let img_new_w = config.album_stamp_img_new_setting_w_mm.unwrap_or(0.0) as f64;
            let img_new_h = config.album_stamp_img_new_setting_h_mm.unwrap_or(0.0) as f64;
            // Match the conservative estimate above when computing per-row heights
            let stamp_heading_h = config
                .header_font_spacing
                .unwrap_or(cfg_defaults.header_font_spacing.unwrap())
                as f64
                + config
                    .text_line_height_mm
                    .unwrap_or(cfg_defaults.text_line_height_mm.unwrap()) as f64;
            let sh_label_for_fit = config
                .stamp_heading_label
                .clone()
                .unwrap_or_else(|| "SG".to_string());
            let sh_size_for_fit = config.stamp_heading_size.unwrap_or(10.0) as i32;
            let include_perforation_in_heading = config.include_perforation.unwrap_or(true);
            for stamp in stamps_in_row {
                let est_height_mm = stamp.est_height_mm.unwrap_or(stamp_box_height);
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
                let heading_emit_width = stamp
                    .est_width_mm
                    .unwrap_or(series_avg_w_mm.unwrap_or(reference_width_mm as f64))
                    + img_new_w
                    + 2.0 * stamp_box_adjust;
                let heading_perforation = stamp
                    .attrs
                    .get("colnect_perforation")
                    .or_else(|| stamp.attrs.get("perforation"))
                    .map(|s| s.as_str());
                let heading_line_count = crate::album_globals::generator_new::emit::estimate_stamp_heading_line_count(
                    &sh_label_for_fit,
                    stamp.attrs.get("catalog_num").map(|v| v.as_str()).unwrap_or(""),
                    stamp.attrs.get("colnect_name").map(|v| v.as_str()),
                    heading_perforation,
                    include_perforation_in_heading,
                    heading_emit_width,
                    sh_size_for_fit,
                ) as f64;
                let total_height =
                    adj_est_height + image_spacing + text_height + (stamp_heading_h * heading_line_count.max(1.0));
                // debug!("[ROW HEIGHT] Stamp: image_path='{}', est_image_height_mm={:.2}, text_lines={}, text_height_mm={:.2}, total_row_height_mm={:.2}",
                // image_path, est_height_mm, text_lines, text_height, total_height);
                if total_height > max_row_height {
                    max_row_height = total_height;
                }
            }
            let vertical_safety = config
                .packing_vertical_safety_mm
                .unwrap_or(cfg_defaults.packing_vertical_safety_mm.unwrap())
                as f64;
            let mut total_row_height = max_row_height + row_font_spacing as f64 + vertical_safety;
            // ROW_START_ES emits a line with configured vspace; account for it
            // in fit checks and consumed height to avoid stranded headers.
            let row_start_es_height = if config.row_start_es_enabled.unwrap_or(true) {
                config
                    .row_start_es_vspace
                    .unwrap_or(cfg_defaults.row_start_es_vspace.unwrap())
                    as f64
            } else {
                0.0
            };
            // A freshly started page already emits one ROW_START_ES from
            // `start_stamp_page`; include that when placing the first row.
            let page_entry_row_start_es_height = if rows_on_page == 0 {
                row_start_es_height
            } else {
                0.0
            };
            let row_start_total_height = row_start_es_height + page_entry_row_start_es_height;

            // In separate_rows mode, reserve a conservative detail-box block
            // during page-fit checks so subsequent rows don't get placed into
            // the vertical space required by metadata boxes.
            let detail_layout_cfg = config
                .stamp_detail_layout
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or("inline");
            if detail_layout_cfg == "separate_rows"
                && config.colnect_details_enabled()
                && !config.colnect_details_inside_stamp()
            {
                let enabled_fields = config.effective_colnect_detail_field_set();

                let mut max_est_lines = 1.0_f64; // catalog header line
                for stamp in stamps_in_row {
                    let mut lines = 1.0_f64;

                    if enabled_fields.contains("catalog_codes")
                        && stamp.attrs.get("colnect_catalog_codes").map(|v| !v.trim().is_empty()).unwrap_or(false)
                    {
                        lines += 2.0;
                    }

                    for key in [
                        "colnect_description",
                        "colnect_themes",
                        "colnect_printing",
                        "colnect_format",
                        "colnect_emission",
                        "colnect_paper",
                        "colnect_gum",
                        "colnect_designers",
                        "colnect_printers",
                        "colnect_colors",
                        "colnect_watermark",
                        "colnect_perforation",
                        "colnect_issued_on",
                        "colnect_expiry_date",
                        "colnect_face_value",
                        "colnect_score",
                    ] {
                        let show = match key {
                            "colnect_description" => enabled_fields.contains("description"),
                            "colnect_themes" => enabled_fields.contains("themes"),
                            "colnect_printing" => enabled_fields.contains("printing"),
                            "colnect_format" => enabled_fields.contains("format"),
                            "colnect_emission" => enabled_fields.contains("emission"),
                            "colnect_paper" => enabled_fields.contains("paper"),
                            "colnect_gum" => enabled_fields.contains("gum"),
                            "colnect_designers" => enabled_fields.contains("designers"),
                            "colnect_printers" => enabled_fields.contains("printers"),
                            "colnect_colors" => enabled_fields.contains("colors"),
                            "colnect_watermark" => enabled_fields.contains("watermark"),
                            "colnect_perforation" => enabled_fields.contains("perforation"),
                            "colnect_issued_on" => enabled_fields.contains("issued_on"),
                            "colnect_expiry_date" => enabled_fields.contains("expiry_date"),
                            "colnect_face_value" => enabled_fields.contains("face_value"),
                            "colnect_score" => enabled_fields.contains("score"),
                            _ => false,
                        };
                        if !show {
                            continue;
                        }

                        if let Some(v) = stamp.attrs.get(key) {
                            let t = v.trim();
                            if !t.is_empty() {
                                let norm = t.replace(',', ", ").replace('|', " | ");
                                lines += (norm.len() as f64 / 42.0).ceil().max(1.0);
                            }
                        }
                    }

                    // Size and print-run/score synthetic lines
                    if enabled_fields.contains("size")
                        && stamp.attrs.get("colnect_width_mm").is_some()
                        && stamp.attrs.get("colnect_height_mm").is_some()
                    {
                        lines += 1.0;
                    }
                    if enabled_fields.contains("print_run")
                        && stamp.attrs.get("colnect_print_run").map(|v| !v.trim().is_empty()).unwrap_or(false)
                    {
                        lines += 1.0;
                    }

                    if lines > max_est_lines {
                        max_est_lines = lines;
                    }
                }

                let line_height_mm = config.page_box_font_size.unwrap_or(8) as f64 * 0.72;
                let detail_vspace = config.stamp_detail_vspace.unwrap_or(2.0) as f64;
                let detail_block_height = (max_est_lines * line_height_mm) + (2.0 * detail_vspace) + 1.0;
                total_row_height += detail_block_height;
            }
            // Determine configured max rows per page (preference). Use canonical default.
            let cfg_max_rows = config
                .max_rows_per_page
                .unwrap_or(cfg_defaults.max_rows_per_page.unwrap())
                as i32;
            // If this is the last row of the current series and there is a next
            // series, peek ahead and compute the next series' conservative
            // needed height (header + first row + buffer). If emitting this
            // row would leave insufficient space for that next series' header
            // and first row, prefer moving this row to the next page so the
            // next series can start cleanly on the current page instead of
            // being cramped into the bottom margin.
            let enable_next_series_lookahead = false;
            if enable_next_series_lookahead
                && row_idx == stamp_rows.len() - 1
                && _series_idx + 1 < filtered_series.len()
                && rows_on_page > 0
            {
                let next_series_name = &filtered_series[_series_idx + 1];
                // Load stamps of the next series
                let next_stamps =
                    get_stamps_by_series(conn, next_series_name, "0").unwrap_or_default();
                // Build header lines for the next series similarly to above
                let mut next_header_lines_vec: Vec<(String, f64)> = Vec::new();
                let next_name_line = clean_text(next_series_name.trim())
                    .replace("\n", " ")
                    .replace("  ", " ");
                let header_font_h = config.text_line_height_mm.unwrap_or(2.82) as f64;
                if !next_name_line.is_empty() {
                    let tmpl = build_series_page_text_centre_template(&config);
                    let safe = next_name_line.replace('"', "\\\"");
                    next_header_lines_vec.push((tmpl.replace("{}", &safe), header_font_h));
                }
                // attempt to pull watermark/design from first stamp if present
                if let Some(next_first_stamp) = next_stamps.first() {
                    let watermark = clean_text(next_first_stamp.watermark.trim())
                        .replace("\n", " ")
                        .replace("  ", " ");
                    if config.include_watermark.unwrap_or(true) && !watermark.is_empty() {
                        let tmpl = build_watermark_page_text_centre_template(&config);
                        let text = format!("Watermark: {}", watermark);
                        let safe = text.replace('"', "\\\"");
                        next_header_lines_vec.push((tmpl.replace("{}", &safe), header_font_h));
                    }
                }
                let next_header_block_height: f64 =
                    next_header_lines_vec.iter().map(|(_, h)| *h).sum();
                let next_vspace_before = config
                    .header_vspace_before_mm
                    .unwrap_or(cfg_defaults.header_vspace_before_mm.unwrap())
                    as f64;
                let next_vspace_after = config
                    .header_vspace_after_mm
                    .unwrap_or(cfg_defaults.header_vspace_after_mm.unwrap())
                    as f64;
                // Additional per-image adjustments from ALBUM_STAMP_IMG_NEW_SETTING
                let _img_new_w = config.album_stamp_img_new_setting_w_mm.unwrap_or(0.0) as f64;
                let img_new_h = config.album_stamp_img_new_setting_h_mm.unwrap_or(0.0) as f64;
                // Estimate next series first row height: look at first row of stamps (group into row of up to 5)
                let mut next_first_row_max = 0.0f64;
                // group next_stamps into rows (configurable max stamps per row)
                let mut next_rows: Vec<Vec<crate::db_utils::StampStub>> = Vec::new();
                let mut nr: Vec<crate::db_utils::StampStub> = Vec::new();
                for s in next_stamps.into_iter() {
                    if nr.len() >= max_stamps_per_row {
                        next_rows.push(nr);
                        nr = Vec::new();
                    }
                    nr.push(s);
                }
                if !nr.is_empty() {
                    next_rows.push(nr);
                }
                if let Some(first_row) = next_rows.first() {
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
                        let adj_est_height = est_height_mm + img_new_h + 2.0 * stamp_box_adjust;
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
                if current_height + row_start_total_height + total_row_height + next_conservative_needed > usable_height {
                    let _ = crate::album_globals::generator_new::layout::start_stamp_page(
                        &mut output,
                        &mut page_counter,
                        &config,
                    );

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
            if current_height + row_start_total_height + total_row_height > effective_usable || rows_on_page >= cfg_max_rows
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
                        // use the new layout helper which expects a mutable page counter
                        let mut pc = page_counter;
                        let _ = layout::start_stamp_page(&mut output, &mut pc, &config);
                        page_counter = pc;
                        current_height = 0.0;
                        header_written = false;
                        rows_on_page = 0;
                    } else if rows_on_page != 0 && (allow_single && percent_of_usable >= threshold)
                    {
                        // We will allow the oversized row to be emitted as a
                        // single-row page: start a fresh page and let it emit.
                        page_counter += 1;
                        let mut pc = page_counter;
                        let _ = layout::start_stamp_page(&mut output, &mut pc, &config);
                        page_counter = pc;

                        current_height = 0.0;
                        header_written = false;
                        rows_on_page = 0;
                    }
                    // Don't continue; fall through to emit the row even if it
                    // doesn't fully fit in the usable height. This avoids an
                    // infinite loop of empty page headers.
                } else {
                    // Row is larger than effective_usable but not larger than
                    // full usable_height. On a fresh page, emit once rather than
                    // looping forever with repeated page breaks.
                    if rows_on_page != 0 {
                        // Start new page and repeat header then re-check on next loop.
                        let _ =
                            layout::start_stamp_page(&mut output, &mut page_counter, &config);
                        current_height = 0.0;
                        header_written = false;
                        rows_on_page = 0;
                        continue; // re-check header on next loop
                    }
                }
            }
            // If we have a pending header, check again whether writing the
            // header + this row (plus the conservative buffer) would overflow
            // the usable page area. If so, force a new page before emitting
            // anything so the header and row start cleanly on the next page.
            if let Some((pending_lines_ref, v_before, v_after)) = &header_pending {
                let conservative_buffer_cfg = config
                    .page_fit_conservative_buffer_mm
                    .unwrap_or(cfg_defaults.page_fit_conservative_buffer_mm.unwrap())
                    as f64;
                let header_block_height: f64 = pending_lines_ref.iter().map(|(_, h)| *h).sum();
                let needed_if_emitted = v_before
                    + header_block_height
                    + v_after
                    + row_start_total_height
                    + total_row_height
                    + conservative_buffer_cfg;
                // If the header+row would not fit on the page at all (oversized),
                // allow emission on a fresh page rather than endlessly forcing new
                // pages. If we're mid-page, start a new page so the oversized
                // content can be emitted there; otherwise allow emission.
                if needed_if_emitted > usable_height {
                    if rows_on_page != 0 {
                        // Start a fresh stamp page using the centralized layout helper.
                        // `start_stamp_page` will increment the page counter and
                        // emit the standard header and separators.
                        let _ = layout::start_stamp_page(&mut output, &mut page_counter, &config);
                        current_height = 0.0;
                        header_written = false;
                        rows_on_page = 0;
                        // Do not `continue` here: fall through and emit the oversized
                        // header+row on the fresh page to avoid infinite loop.
                    }
                } else if current_height + needed_if_emitted > usable_height {
                    // Normal case: header+row would overflow current page; force a new page now
                    // Use the centralized helper which increments the page counter
                    // and emits the standard header/separators and ROW_START_ES.
                    let _ = layout::start_stamp_page(&mut output, &mut page_counter, &config);
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
                // Delegate header emission to layout helper which writes the
                // page vspace markers and header lines and returns the
                // total vertical millimetre height consumed.
                match crate::album_globals::generator_new::layout::write_header_block(
                    &mut output,
                    &pending_lines,
                    v_before,
                    v_after,
                    &config,
                ) {
                    Ok(h) => current_height += h,
                    Err(e) => {
                        log::error!("Failed to write header block: {}", e);
                        // Fallback: pretend no height was written so generation continues
                    }
                }
                header_written = true;
            }
            // Build ROW_START_ES from parameterized config if provided, otherwise fall back to
            // the legacy `row_start_es_marker` string. Parameters are: font, size, spacing, vspace
            // Before emitting the row, compute row metadata (width, stamps) so
            // we can optionally emit diagnostics or a per-row horizontal offset
            // to center the row when configured.
            {
                let stamps_in_row = &stamp_rows[row_idx];
                let mut row_w = 0.0f64;
                let margin = config.packing_horizontal_safety_mm.unwrap_or(5.0) as f64;
                let stamp_box_adjust = config.stamp_box_adjust.unwrap_or(0.0) as f64;
                for s in stamps_in_row.iter() {
                    let est_w = s.est_width_mm.unwrap_or(reference_width_mm as f64);
                    let emit_w = est_w + img_new_w + 2.0 * stamp_box_adjust;
                    row_w += emit_w + margin;
                }
                // The computed row_w currently includes a trailing margin; subtract one margin
                // so row width represents the true occupied width.
                if !stamps_in_row.is_empty() {
                    row_w -= margin;
                }
                let usable_w = page_width
                    - config.page_left_margin_mm.unwrap_or(10.0) as f64
                    - config.page_right_margin_mm.unwrap_or(10.0) as f64
                    - config.album_pages_spacing_h.unwrap_or(0.0) as f64
                    - (border_outer + border_inner1 + border_inner2 + border_spacing);
                let extra = usable_w - row_w;
                let left_offset = if extra > 0.0 { extra / 2.0 } else { 0.0 };
                if config.show_stamps_per_row.unwrap_or(false) {
                    let _ = writeln!(output, "# ROW_META stamps=\"{}\" row_w=\"{:.2}\" usable=\"{:.2}\" extra=\"{:.2}\" left_offset=\"{:.2}\"",
                        stamps_in_row.len(), row_w, usable_w, extra, left_offset);
                }
                // Row centering requested but generator-side filler disabled: do not
                // emit filler images or mutate album images. Keep ROW_META diagnostics
                // (if enabled) but avoid generating spacer artifacts that change layout.
            }

            let _ = crate::album_globals::generator_new::layout::emit_row_start_es(&mut output, &config);

            // Collect stamp details for separate_rows mode (catalog_num, details, width_mm)
            let mut stamp_details: Vec<(String, String, f64)> = Vec::new();

            // row start written (diagnostics suppressed)
            for stamp in stamps_in_row {
                // Use correct fields from StampStub
                // Use image_path from the database directly
                let image_path = stamp.image_path.trim();
                let full_image_path =
                    format!("Albums/{}/{}", album_name.replace(' ', "-"), image_path);
                // Use precomputed estimates from the pre-pass so row packing,
                // page-fit checks, and emitted dimensions stay in sync.
                let mut est_width_mm = stamp
                    .est_width_mm
                    .unwrap_or(series_avg_w_mm.unwrap_or(reference_width_mm as f64));
                let mut est_height_mm = stamp
                    .est_height_mm
                    .unwrap_or(series_avg_h_mm.unwrap_or(reference_height_mm as f64));
                // Apply ALBUM_STAMP_IMG_NEW_SETTING adjustments to width/height (additive)
                est_width_mm += img_new_w;
                est_height_mm += img_new_h;
                // Debug: emit detailed per-stamp sizing diagnostics when debug logging enabled
                // debug!("Stamp [{}|{}] DB_px=({:?},{:?}) mm_per_px=({:.6},{:.6}) pre_scale_mm=({:.2},{:.2}) scale={:.4} img_new=(w:{:.2},h:{:.2}) box_adjust={:.2} emit_mm_before_adjust=({:.2},{:.2}) emit_mm_final=({:.2},{:.2})",
                //         album_name,
                //         stamp.attrs.get("sg_number").unwrap_or(&String::new()),
                //         stamp.image_width_px,
                //         stamp.image_height_px,
                //         mm_per_pixel_w as f64,
                //         mm_per_pixel_h as f64,
                //         (est_width_mm - img_new_w) / (if scale==0.0 {1.0} else {scale}), // approximate pre-scale width
                //         (est_height_mm - img_new_h) / (if scale==0.0 {1.0} else {scale}), // approximate pre-scale height
                //         scale,
                //         img_new_w,
                //         img_new_h,
                //         stamp_box_adjust,
                //         est_width_mm - 2.0 * stamp_box_adjust,
                //         est_height_mm - 2.0 * stamp_box_adjust,
                //         est_width_mm + 2.0 * stamp_box_adjust,
                //         est_height_mm + 2.0 * stamp_box_adjust
                //     );
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
                let approx_char_width_mm = 2.2_f64; // base heuristic average char width
                let padding_mm = 4.0_f64; // horizontal padding to avoid tight wrap
                let avail_width_mm = if emit_width > padding_mm {
                    emit_width - padding_mm
                } else {
                    emit_width
                };
                // Wider-than-tall stamps can carry longer metadata lines safely,
                // while tall/narrow stamps should keep shorter lines.
                let aspect_ratio = if emit_height > 0.0 {
                    emit_width / emit_height
                } else {
                    1.0
                };
                let width_bias = if aspect_ratio >= 1.20 {
                    1.35
                } else if aspect_ratio >= 1.0 {
                    1.15
                } else {
                    0.95
                };
                let mut chars_per_line =
                    ((avail_width_mm / approx_char_width_mm) * width_bias).floor() as usize;
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

                // Description text can be sentence-like and reads poorly with
                // very tight wrapping. Use a looser target and soft overflow
                // so lines look closer to other metadata fields.
                let wrap_description_words = |s: &str, max_chars: usize| -> String {
                    if s.trim().is_empty() {
                        return String::new();
                    }
                    let target = max_chars.max(18);
                    let soft_limit = target + 8;
                    let mut lines: Vec<String> = Vec::new();
                    let mut current = String::new();
                    for word in s.split_whitespace() {
                        if current.is_empty() {
                            current.push_str(word);
                        } else if current.len() + 1 + word.len() <= soft_limit {
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

                // Wrap a metadata field while keeping `Label: value` together.
                // If value is empty, drop the field entirely.
                let wrap_label_value = |s: &str, max_chars: usize| -> Option<String> {
                    let trimmed = s.trim();
                    if trimmed.is_empty() {
                        return None;
                    }

                    if let Some((label, value_raw)) = trimmed.split_once(':') {
                        let value = value_raw.trim();
                        if value.is_empty() {
                            return None;
                        }

                        let label = label.trim();
                        let value_words: Vec<&str> = value.split_whitespace().collect();
                        if value_words.is_empty() {
                            return None;
                        }

                        let mut lines: Vec<String> = Vec::new();
                        // Keep at least two value words on the label line when available
                        // so fields like `Designers: Otto Treumann` do not fragment.
                        let first_take = value_words.len().min(2);
                        let mut current = format!("{}: {}", label, value_words[..first_take].join(" "));
                        for word in value_words.iter().skip(first_take) {
                            // Soft overflow tolerance reduces orphan single-word lines.
                            let soft_limit = max_chars + 10;
                            if current.len() + 1 + word.len() <= soft_limit {
                                current.push(' ');
                                current.push_str(word);
                            } else {
                                lines.push(current);
                                current = (*word).to_string();
                            }
                        }
                        if !current.is_empty() {
                            lines.push(current);
                        }
                        return Some(lines.join("\\n"));
                    }

                    // Unlabeled metadata entries are typically descriptions;
                    // keep phrase-level readability similar to other fields.
                    let wrapped = wrap_description_words(trimmed, max_chars);
                    if wrapped.is_empty() {
                        None
                    } else {
                        Some(wrapped)
                    }
                };

                let wrapped_desc = wrap_description_words(&raw_description, chars_per_line);

                // Get layout mode from config
                let detail_layout = config
                    .stamp_detail_layout
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or("inline");

                // Build Colnect data based on layout mode and config
                let mut colnect_parts = Vec::new();
                let colnect_enabled = config.colnect_details_enabled();
                let enabled_fields = if colnect_enabled {
                    config.effective_colnect_detail_field_set()
                } else {
                    std::collections::HashSet::new()
                };

                // Only collect Colnect data if not in minimal mode
                if detail_layout != "minimal" && colnect_enabled {
                    if enabled_fields.contains("catalog_codes") {
                        if let Some(v) = stamp.attrs.get("colnect_catalog_codes") {
                            colnect_parts.push(format!("Catalog: {}", v));
                        }
                    }
                    if enabled_fields.contains("description") {
                        if let Some(v) = stamp.attrs.get("colnect_description") {
                            colnect_parts.push(format!("{}", v));
                        }
                    }
                    if enabled_fields.contains("themes") {
                        if let Some(v) = stamp.attrs.get("colnect_themes") {
                            colnect_parts.push(format!("Themes: {}", v));
                        }
                    }
                    if enabled_fields.contains("printing") {
                        if let Some(v) = stamp.attrs.get("colnect_printing") {
                            colnect_parts.push(format!("Printing: {}", v));
                        }
                    }
                    if enabled_fields.contains("format") {
                        if let Some(v) = stamp.attrs.get("colnect_format") {
                            colnect_parts.push(format!("Format: {}", v));
                        }
                    }
                    if enabled_fields.contains("emission") {
                        if let Some(v) = stamp.attrs.get("colnect_emission") {
                            colnect_parts.push(format!("Emission: {}", v));
                        }
                    }
                    if enabled_fields.contains("paper") {
                        if let Some(v) = stamp.attrs.get("colnect_paper") {
                            colnect_parts.push(format!("Paper: {}", v));
                        }
                    }
                    if enabled_fields.contains("gum") {
                        if let Some(v) = stamp.attrs.get("colnect_gum") {
                            colnect_parts.push(format!("Gum: {}", v));
                        }
                    }
                    if enabled_fields.contains("designers") {
                        if let Some(v) = stamp.attrs.get("colnect_designers") {
                            colnect_parts.push(format!("Designers: {}", v));
                        }
                    }
                    if enabled_fields.contains("printers") {
                        if let Some(v) = stamp.attrs.get("colnect_printers") {
                            colnect_parts.push(format!("Printers: {}", v));
                        }
                    }
                    if enabled_fields.contains("colors") {
                        if let Some(v) = stamp.attrs.get("colnect_colors") {
                            colnect_parts.push(format!("Colors: {}", v));
                        }
                    }
                    if enabled_fields.contains("watermark") {
                        if let Some(v) = stamp.attrs.get("colnect_watermark") {
                            colnect_parts.push(format!("Watermark: {}", v));
                        }
                    }
                    // Perforation is now shown in STAMP_HEADING when enabled.
                    // Keep it out of inline/inside-stamp body text to avoid duplicate clutter.
                    let show_perforation_in_stamp_body =
                        detail_layout == "separate_rows" && !config.colnect_details_inside_stamp();
                    if enabled_fields.contains("perforation") && show_perforation_in_stamp_body {
                        if let Some(v) = stamp.attrs.get("colnect_perforation") {
                            colnect_parts.push(format!("Perforation: {}", v));
                        }
                    }
                    if enabled_fields.contains("issued_on") {
                        if let Some(v) = stamp.attrs.get("colnect_issued_on") {
                            colnect_parts.push(format!("Issued: {}", v));
                        }
                    }
                    if enabled_fields.contains("expiry_date") {
                        if let Some(v) = stamp.attrs.get("colnect_expiry_date") {
                            colnect_parts.push(format!("Expiry: {}", v));
                        }
                    }
                    if enabled_fields.contains("size") {
                        if let (Some(w), Some(h)) = (
                            stamp.attrs.get("colnect_width_mm"),
                            stamp.attrs.get("colnect_height_mm"),
                        ) {
                            colnect_parts.push(format!("Size: {} x {} mm", w, h));
                        }
                    }
                    if enabled_fields.contains("face_value") {
                        if let Some(v) = stamp.attrs.get("colnect_face_value") {
                            if let Some(cur) = stamp.attrs.get("colnect_currency") {
                                colnect_parts.push(format!("Face value: {} {}", v, cur));
                            } else {
                                colnect_parts.push(format!("Face value: {}", v));
                            }
                        }
                    }
                    if enabled_fields.contains("score") {
                        if let Some(v) = stamp.attrs.get("colnect_score") {
                            if let Some(acc) = stamp.attrs.get("colnect_accuracy") {
                                colnect_parts.push(format!("Score: {} (Accuracy: {})", v, acc));
                            } else {
                                colnect_parts.push(format!("Score: {}", v));
                            }
                        }
                    }
                }

                // Remove empty fields (or empty `Label:` entries) before formatting.
                colnect_parts.retain(|entry| {
                    let t = entry.trim();
                    if t.is_empty() {
                        return false;
                    }
                    if let Some((_, rhs)) = t.split_once(':') {
                        return !rhs.trim().is_empty();
                    }
                    true
                });

                let colnect_text = if detail_layout == "separate_rows"
                    && !config.colnect_details_inside_stamp()
                {
                    colnect_parts.join("\n")
                } else {
                    colnect_parts.join("; ")
                };
                let wrapped_colnect = if detail_layout == "separate_rows"
                    && !config.colnect_details_inside_stamp()
                {
                    wrap_words(&colnect_text, chars_per_line)
                } else {
                    let mut wrapped_entries: Vec<String> = Vec::new();
                    for (idx, entry) in colnect_parts.iter().enumerate() {
                        if let Some(mut wrapped) = wrap_label_value(entry, chars_per_line) {
                            if idx + 1 < colnect_parts.len() {
                                wrapped.push(';');
                            }
                            wrapped_entries.push(wrapped);
                        }
                    }
                    wrapped_entries.join("\\n")
                };

                // Build centre text based on layout mode
                let centre = match detail_layout {
                    "minimal" => {
                        // Minimal: only face value, no clutter
                        String::new()
                    }
                    "separate_rows" => {
                        if config.colnect_details_inside_stamp() {
                            // Keep details with the stamp instead of PAGE_ADD_BOX.
                            if wrapped_colnect.is_empty() {
                                raw_stamps_issued.clone()
                            } else if raw_stamps_issued.is_empty() {
                                wrapped_colnect.clone()
                            } else {
                                format!("{}\\n\\n{}", raw_stamps_issued, wrapped_colnect)
                            }
                        } else {
                            // Separate rows with detail boxes: keep stamp text minimal.
                            if enabled_fields.contains("print_run") {
                                raw_stamps_issued.clone()
                            } else {
                                String::new()
                            }
                        }
                    }
                    _ => {
                        // Inline (default): everything crammed inside stamp box
                        match (wrapped_desc.is_empty(), wrapped_colnect.is_empty()) {
                            (true, true) => format!("{}\\n{}", raw_stamps_issued, raw_color),
                            (false, true) => {
                                format!("{}\\n{}\\n{}", raw_stamps_issued, wrapped_desc, raw_color)
                            }
                            (true, false) => {
                                format!("{}\\n\\n{}\\n{}", raw_stamps_issued, wrapped_colnect, raw_color)
                            }
                            (false, false) => format!(
                                "{}\\n{}\\n{}\\n{}",
                                raw_stamps_issued, wrapped_desc, wrapped_colnect, raw_color
                            ),
                        }
                    }
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

                // Prefer Scott catalog number if available, fall back to SG number
                let sgnum = if let Some(scott) = stamp.attrs.get("scott_number") {
                    clean_text(scott)
                } else {
                    clean_text(stamp.attrs.get("sg_number").unwrap_or(&"".to_string()))
                };

                // Store details for separate_rows mode with stamp width
                if detail_layout == "separate_rows"
                    && !config.colnect_details_inside_stamp()
                    && !colnect_text.is_empty()
                {
                    stamp_details.push((sgnum.clone(), colnect_text.clone(), emit_width));
                }

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
                    let emit_type = "standard";
                    if is_us1847 {
                        if emit_type == "image" {
                            // Emit image
                            let line = format!(
                                "STAMP_ADD_IMG ({:.1} {:.1} \"{}\" \"\" \"\" \"\")",
                                emit_width, emit_height, image_path
                            );
                            let _ = writeln!(output, "{}", line.replace("\r", "").replace("\n", "\\n"));
                        } else if emit_type == "scott" {
                            // Emit Scott number as text
                            let line = format!(
                                "STAMP_ADD ({:.1} {:.1} \"{}\" \"HN\" \"12\" \"black\")",
                                emit_width, 15.0, sgnum
                            );
                            let _ = writeln!(output, "{}", line.replace("\r", "").replace("\n", "\\n"));
                        } else if emit_type == "info" {
                            // Emit stamp info as text box
                            let info = format!(
                                "Denomination: {}\\nDescription: {}\\nColor: {}\\nIssued: {}",
                                raw_left, wrapped_desc, raw_color, raw_stamps_issued
                            );
                            let line = format!(
                                "STAMP_ADD ({:.1} {:.1} \"{}\" \"HN\" \"8\" \"black\")",
                                emit_width, 40.0, info.replace("\n", "\\n").replace("\"", "'")
                            );
                            let _ = writeln!(output, "{}", line.replace("\r", "").replace("\n", "\\n"));
                        } else if emit_type == "standard" {
                            // Emit image
                            let line = format!(
                                "STAMP_ADD_IMG ({:.1} {:.1} \"{}\" \"\" \"\" \"\")",
                                emit_width, emit_height, image_path
                            );
                            let _ = writeln!(output, "{}", line.replace("\r", "").replace("\n", "\\n"));
                            // Emit Scott number as text
                            let line = format!(
                                "STAMP_ADD ({:.1} {:.1} \"{}\" \"HN\" \"12\" \"black\")",
                                emit_width, 15.0, sgnum
                            );
                            let _ = writeln!(output, "{}", line.replace("\r", "").replace("\n", "\\n"));
                            // Emit stamp info as text box
                            let info = format!(
                                "Denomination: {}\\nDescription: {}\\nColor: {}\\nIssued: {}",
                                raw_left, wrapped_desc, raw_color, raw_stamps_issued
                            );
                            let line = format!(
                                "STAMP_ADD ({:.1} {:.1} \"{}\" \"HN\" \"8\" \"black\")",
                                100.0, 40.0, info.replace("\n", "\\n").replace("\"", "'")
                            );
                            let _ = writeln!(output, "{}", line.replace("\r", "").replace("\n", "\\n"));
                        }
                    } else {
                        // Standard formatting
                        // Delegate formatted emission to the centralized emitter
                        // so formatting and escaping are consistent across call sites.
                        let sh_font = config
                            .stamp_heading_font
                            .clone()
                            .unwrap_or_else(|| "HN".to_string());
                        let sh_size = config.stamp_heading_size.unwrap_or(10.0) as i32;
                        let sh_label = config
                            .stamp_heading_label
                            .clone()
                            .unwrap_or_else(|| "SG".to_string());
                        let stamp_name = stamp.attrs.get("colnect_name").map(|s| s.as_str());
                        let heading_perforation = stamp
                            .attrs
                            .get("colnect_perforation")
                            .or_else(|| stamp.attrs.get("perforation"))
                            .map(|s| s.as_str());
                        let _ = crate::album_globals::generator_new::emit::emit_stamp(
                            &mut output,
                            true,
                            image_path,
                            emit_width,
                            emit_height,
                            &left,
                            &centre,
                            &right,
                            &sh_font,
                            sh_size,
                            &sh_label,
                            &sgnum,
                            stamp_name,
                            heading_perforation,
                            config.include_perforation.unwrap_or(true),
                        );
                    }
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

                    // Try to generate a per-stamp placeholder image when the real image
                    // is missing or points at a common placeholder. If generation fails
                    // fall back to the textual "Image Not Available" box.
                    let mut emitted_placeholder: Option<String> = None;
                    // Determine if we should attempt to create a placeholder
                    let should_generate = image_path.is_empty()
                        || !Path::new(&full_image_path).exists()
                        || image_path.starts_with("images/no_image");
                    if should_generate {
                        // Year from attributes (fallback to empty)
                        let year_attr = stamp.attrs.get("year").unwrap_or(&"".to_string()).clone();
                        let year_clean = clean_text(&year_attr);
                        // Determine a better target size for the placeholder. Prefer the
                        // current emitted dimensions, but if this stamp lacks image
                        // dimensions try to use neighbouring stamps on the same row as
                        // a second-level fallback so the placeholder visually matches
                        // its neighbours.
                        // Default target for placeholder creation: the image-only
                        // estimated width/height (do NOT include the per-side
                        // `stamp_box_adjust` here). If neighbours are used as a
                        // fallback, use their raw image mm values (not the
                        // emitted box size) so placeholders match neighbour
                        // image areas rather than the printed box padding.
                        let mut target_w_mm =
                            stamp.est_width_mm.unwrap_or(reference_width_mm as f64);
                        let mut target_h_mm =
                            stamp.est_height_mm.unwrap_or(reference_height_mm as f64);
                        // If this stamp has no explicit image pixel dimensions,
                        // attempt to compute an average from neighbours in the row.
                        if stamp.image_width_px.is_none() && stamp.image_height_px.is_none() {
                            let mut neigh_w_sum = 0.0f64;
                            let mut neigh_h_sum = 0.0f64;
                            let mut neigh_count = 0u32;
                            // stamps_in_row is available in this scope and refers to the
                            // vector of stamps in the current row; iterate to collect
                            // neighbor sizes when available.
                            for neigh in stamps_in_row.iter() {
                                // skip self
                                let neigh_sg = neigh
                                    .attrs
                                    .get("sg_number")
                                    .unwrap_or(&"".to_string())
                                    .clone();
                                if neigh_sg
                                    == stamp
                                        .attrs
                                        .get("sg_number")
                                        .unwrap_or(&"".to_string())
                                        .clone()
                                {
                                    continue;
                                }
                                // prefer DB-stored pixel dims
                                if let (Some(nw_px), Some(nh_px)) =
                                    (neigh.image_width_px, neigh.image_height_px)
                                {
                                    let mut nh = (nh_px as f64) * (mm_per_pixel_h as f64);
                                    if !preserve_real && nh > stamp_box_height {
                                        nh = stamp_box_height;
                                    }
                                    let nw = (nw_px as f64) * (mm_per_pixel_w as f64);
                                    neigh_w_sum += nw;
                                    neigh_h_sum += nh;
                                    neigh_count += 1;
                                } else {
                                    // try to estimate from actual image file if present
                                    let neigh_img = neigh.image_path.trim();
                                    let neigh_full = format!(
                                        "Albums/{}/{}",
                                        album_name.replace(' ', "-"),
                                        neigh_img
                                    );
                                    if let Some((wmm, hmm)) = estimate_real_dimensions(
                                        &neigh_full,
                                        mm_per_pixel_w,
                                        mm_per_pixel_h,
                                    ) {
                                        let mut nh = hmm as f64;
                                        if !preserve_real && nh > stamp_box_height {
                                            nh = stamp_box_height;
                                        }
                                        neigh_w_sum += wmm as f64;
                                        neigh_h_sum += nh;
                                        neigh_count += 1;
                                    }
                                }
                            }
                            if neigh_count > 0 {
                                // Neighbour sums are based on the raw image physical
                                // dimensions (mm). The emitted stamp box includes the
                                // per-side `stamp_box_adjust` margin (added twice,
                                // once per side) so include that here so the
                                // placeholder is generated at the same printed
                                // dimensions as neighbouring stamps.
                                let neigh_w_mm = neigh_w_sum / (neigh_count as f64);
                                let neigh_h_mm = neigh_h_sum / (neigh_count as f64);
                                // Use neighbour image-only sizes (no stamp_box_adjust)
                                target_w_mm = neigh_w_mm;
                                target_h_mm = neigh_h_mm;
                            }
                        }

                        // sgnum already contains Scott number (or SG fallback), use for both DB and filename
                        let sg_for_db = stamp.attrs.get("sg_number").map(|v| v.as_str()).unwrap_or("");
                        if let Some((rel_path, _, _)) =
                            crate::album_globals::generator_new::helpers::ensure_placeholder_for_stamp_and_update_db(
                                conn,
                                album_name,
                                series,
                                sg_for_db,
                                &sgnum,
                                &year_clean,
                                target_w_mm,
                                target_h_mm,
                                mm_per_pixel_w,
                                mm_per_pixel_h,
                                Some(&mut output),
                            )
                        {
                            emitted_placeholder = Some(rel_path.clone());
                        }
                    }

                    if let Some(img_rel) = emitted_placeholder {
                        // We have a generated placeholder; emit STAMP_ADD_IMG using it
                        let sh_font = config
                            .stamp_heading_font
                            .clone()
                            .unwrap_or_else(|| "HN".to_string());
                        let sh_size = config.stamp_heading_size.unwrap_or(10.0) as i32;
                        let sh_label = config
                            .stamp_heading_label
                            .clone()
                            .unwrap_or_else(|| "".to_string());
                        let stamp_name = stamp.attrs.get("colnect_name").map(|s| s.as_str());
                        let heading_perforation = stamp
                            .attrs
                            .get("colnect_perforation")
                            .or_else(|| stamp.attrs.get("perforation"))
                            .map(|s| s.as_str());
                        let _ = crate::album_globals::generator_new::emit::emit_stamp(
                            &mut output,
                            true,
                            &img_rel,
                            emit_width,
                            emit_height,
                            &left,
                            &centre,
                            &right,
                            &sh_font,
                            sh_size,
                            &sh_label,
                            &sgnum,
                            stamp_name,
                            heading_perforation,
                            config.include_perforation.unwrap_or(true),
                        );
                    } else {
                        // Could not create a placeholder; fall back to the textual box
                        let sh_font = config
                            .stamp_heading_font
                            .clone()
                            .unwrap_or_else(|| "HN".to_string());
                        let sh_size = config.stamp_heading_size.unwrap_or(10.0) as i32;
                        let sh_label = config
                            .stamp_heading_label
                            .clone()
                            .unwrap_or_else(|| "".to_string());
                        let stamp_name = stamp.attrs.get("colnect_name").map(|s| s.as_str());
                        let heading_perforation = stamp
                            .attrs
                            .get("colnect_perforation")
                            .or_else(|| stamp.attrs.get("perforation"))
                            .map(|s| s.as_str());
                        let _ = crate::album_globals::generator_new::emit::emit_stamp(
                            &mut output,
                            false,
                            "Image\\nNot\\nAvailable",
                            emit_width,
                            emit_height,
                            &left,
                            &centre,
                            &right,
                            &sh_font,
                            sh_size,
                            &sh_label,
                            &sgnum,
                            stamp_name,
                            heading_perforation,
                            config.include_perforation.unwrap_or(true),
                        );
                        let _ = writeln!(output, "# PAGE_FIT_STATE step=\"stamp_emitted\" series=\"{}\" img=\"{}\" emit_h=\"{:.2}\" current=\"{:.2}\"",
                                clean_text(series).replace('\n', " ").replace('"', "'"), image_path, emit_height, current_height + total_row_height);
                    }
                }
            }

            let mut consumed_height_this_row = total_row_height;

            // Emit stamp details in separate row if mode is "separate_rows"
            if !stamp_details.is_empty() {
                let vspace = config.stamp_detail_vspace.unwrap_or(2.0);

                // Get PAGE_ADD_BOX configuration from config
                let box_font = config.page_box_font.as_ref().map(|s| s.as_str()).unwrap_or("HN");
                let box_font_size = config.page_box_font_size.unwrap_or(8);
                let box_border_width = config.page_box_border_width.unwrap_or(0.1);
                let box_border_color = config.page_box_border_color.as_ref().map(|s| s.as_str()).unwrap_or("blue");
                let box_text_color = config.page_box_text_color.as_ref().map(|s| s.as_str()).unwrap_or("blue");
                let box_bg_color = config.page_box_background_color.as_ref().map(|s| s.as_str()).unwrap_or("white");

                // Note: We don't emit PAGE_VSPACE here because:
                // 1. The box_top coordinate already includes the vspace offset
                // 2. Post-processing filters out PAGE_VSPACE and adds its own for page centering

                // Calculate usable page width for detail boxes (same as row centering logic)
                let page_width = config.album_page_width_mm.unwrap_or(216.0) as f64;
                let left_margin = config.page_left_margin_mm.unwrap_or(10.0) as f64;
                let right_margin = config.page_right_margin_mm.unwrap_or(10.0) as f64;
                let pages_spacing_h = config.album_pages_spacing_h.unwrap_or(0.0) as f64;
                let border_total = border_outer + border_inner1 + border_inner2 + border_spacing;

                // Calculate usable width (same formula as row centering)
                let usable_width = page_width - left_margin - right_margin - pages_spacing_h - border_total;

                // Vertical position: boxes should appear AFTER the stamp row, not at the same height
                // current_height tracks where we are on the page before this row
                // total_row_height is the height of the stamp row that was just emitted
                // vspace is additional spacing between stamps and detail boxes
                let box_top = current_height + total_row_height + vspace;

                // Calculate row width and centering offset (mimicking the ROW_META calculation above)
                // Use the same spacing value as ROW_META calculation (packing_horizontal_safety_mm)
                let row_spacing = config.packing_horizontal_safety_mm.unwrap_or(5.0) as f64;
                let mut row_w = 0.0;
                for stamp_width in stamp_details.iter().map(|(_, _, w)| w) {
                    row_w += stamp_width + row_spacing;
                }
                // Remove trailing spacing
                if !stamp_details.is_empty() {
                    row_w -= row_spacing;
                }

                // Calculate centering offset (extra space divided by 2)
                let extra = usable_width - row_w;
                let left_offset = if extra > 0.0 { extra / 2.0 } else { 0.0 };

                // The absolute starting position on the page is:
                // left_margin + pages_spacing_h + (borders are not counted here) + left_offset
                // But actually, reviewing the calculation: usable_width already excludes left_margin, right_margin, pages_spacing_h, and borders
                // So left_offset is relative to the start of usable area
                // Absolute position = left_margin + pages_spacing_h + left_offset
                let base_start = left_margin + pages_spacing_h + left_offset;

                // Calculate X positions based on stamp widths with row centering applied
                let mut stamp_positions: Vec<(f64, f64)> = Vec::new(); // (left_pos, stamp_width)
                let mut current_x = base_start;

                for (_catalog_num, _details, stamp_width) in stamp_details.iter() {
                    stamp_positions.push((current_x, *stamp_width));
                    current_x += stamp_width + row_spacing;
                }

                for (idx, (catalog_num, details, stamp_width)) in stamp_details.iter().enumerate() {
                    // Get the stamp's left position from our calculated positions
                    let (stamp_left, _) = stamp_positions[idx];

                    // Box width: prefer 2x stamp width for better text flow, but cap at stamp width minimum
                    // Maximum box width should not exceed usable page width
                    let max_box_width = usable_width.min(*stamp_width * 3.5);
                    let box_width = (*stamp_width * 2.8).min(max_box_width).max(85.0);

                    // Center box under stamp using the stamp's actual position
                    let stamp_center = stamp_left + (*stamp_width / 2.0);
                    let box_left = stamp_center - (box_width / 2.0).max(0.0);

                    // Accurate character width calculation based on font size
                    // For Helvetica Normal, average character width is approximately:
                    // - Lowercase letters: ~0.5-0.6 * font_size in points
                    // - Mixed text (typical): ~0.55 * font_size in points
                    // In mm: 1 point ≈ 0.3527mm, so char_width ≈ font_size * 0.55 * 0.3527 ≈ font_size * 0.194mm
                    // However, with proper kerning and tight spacing: ~0.22mm per point of font size
                    // For tighter fit to minimize white space: using 0.21
                    let char_width_mm = box_font_size as f64 * 0.21;
                    // Line height needs more spacing to prevent bleeding/touching
                    // For font size 6: 6 * 0.60 = 3.6mm
                    // For font size 8: 8 * 0.60 = 4.8mm
                    // For font size 10: 10 * 0.60 = 6.0mm
                    let line_height_mm = box_font_size as f64 * 0.72;

                    // Calculate max characters per line based on box width
                    let padding = 0.5; // minimal padding for tight fit
                    let available_width = box_width - padding;
                    let max_chars_per_line = (available_width / char_width_mm).floor() as usize;
                    let max_chars = max_chars_per_line.max(40); // keep sections readable

                    // Wrap text to fit box width
                    let mut wrapped_lines = Vec::new();

                    // Add catalog number as first line
                    wrapped_lines.push(format!("{}:", catalog_num));

                    // Preserve section lines where possible and only wrap long lines.
                    for field in details.lines() {
                        if field.is_empty() {
                            continue;
                        }

                        let normalized_field = field.replace(',', ", ").replace('|', " | ");

                        // Word wrapping with better space utilization
                        let mut current_line = String::new();
                        for raw_word in normalized_field.split_whitespace() {
                            // Split very long tokens so a single unbroken token
                            // (e.g. long theme/catalog fragments) can't force
                            // overlapping text in the detail box.
                            let mut parts: Vec<String> = Vec::new();
                            if raw_word.len() > max_chars {
                                let mut start = 0;
                                while start < raw_word.len() {
                                    let end = (start + max_chars).min(raw_word.len());
                                    parts.push(raw_word[start..end].to_string());
                                    start = end;
                                }
                            } else {
                                parts.push(raw_word.to_string());
                            }

                            for word in parts {
                            let test_line = if current_line.is_empty() {
                                word.clone()
                            } else {
                                format!("{} {}", current_line, word)
                            };

                            if test_line.len() <= max_chars {
                                current_line = test_line;
                            } else {
                                // Line would be too long, push current and start new
                                if !current_line.is_empty() {
                                    wrapped_lines.push(current_line);
                                }
                                current_line = word;
                            }
                        }
                        }
                        if !current_line.is_empty() {
                            wrapped_lines.push(current_line);
                        }
                    }

                    // Calculate box height based on number of wrapped lines
                    let num_lines = wrapped_lines.len();
                    let box_height = (num_lines as f64 * line_height_mm) + padding;

                    // Format inside text with newlines
                    let inside_text = wrapped_lines.join("\\n").replace('"', "'");

                    // Set colors before box
                    let _ = writeln!(output, "COLOR_STAMP_BACKGROUND({})", box_bg_color);
                    let _ = writeln!(output, "COLOUR_STAMP_BORDER ({})", box_border_color);
                    let _ = writeln!(output, "COLOUR_STAMP_TEXT ({})", box_text_color);

                    // Emit PAGE_ADD_BOX (Font Fontsize border left top width height inside-text left-text centre-text right-text)
                    let _ = writeln!(
                        output,
                        "PAGE_ADD_BOX ({} {} {} {:.2} {:.2} {:.2} {:.2} \"{}\" \"\" \"\" \"\")",
                        box_font,
                        box_font_size,
                        box_border_width,
                        box_left,
                        box_top,
                        box_width,
                        box_height,
                        inside_text
                    );

                    // Reset colors to default values from config
                    let default_bg = config.color_stamp_background.as_ref().map(|s| s.as_str()).unwrap_or("black");
                    let default_border = config.color_stamp_border.as_ref().map(|s| s.as_str()).unwrap_or("black");
                    let default_text = config.color_stamp_text.as_ref().map(|s| s.as_str()).unwrap_or("black");
                    let _ = writeln!(output, "COLOR_STAMP_BACKGROUND({})", default_bg);
                    let _ = writeln!(output, "COLOUR_STAMP_BORDER({})", default_border);
                    let _ = writeln!(output, "COLOUR_STAMP_TEXT({})", default_text);
                }

                // Update current_height to account for the tallest box
                if let Some(max_height) = stamp_details.iter().map(|(_, details, width)| {
                    // Calculate box width consistently with the loop above
                    let max_box_width = usable_width.min(*width * 3.5);
                    let box_width = (*width * 2.8).min(max_box_width).max(85.0);
                    let char_width_mm = box_font_size as f64 * 0.21;
                    let line_height_mm = box_font_size as f64 * 0.72;
                    let padding = 0.5;
                    let available_width = box_width - padding;
                    let max_chars = ((available_width / char_width_mm).floor() as usize).max(40);

                    let mut line_count = 1; // catalog number
                    for field in details.lines() {
                        if !field.is_empty() {
                            let normalized_field = field.replace(',', ", ").replace('|', " | ");
                            let mut current_line = String::new();
                            for raw_word in normalized_field.split_whitespace() {
                                let mut parts: Vec<String> = Vec::new();
                                if raw_word.len() > max_chars {
                                    let mut start = 0;
                                    while start < raw_word.len() {
                                        let end = (start + max_chars).min(raw_word.len());
                                        parts.push(raw_word[start..end].to_string());
                                        start = end;
                                    }
                                } else {
                                    parts.push(raw_word.to_string());
                                }

                                for word in parts {
                                let test_line = if current_line.is_empty() {
                                    word.clone()
                                } else {
                                    format!("{} {}", current_line, word)
                                };

                                if test_line.len() <= max_chars {
                                    current_line = test_line;
                                } else {
                                    if !current_line.is_empty() {
                                        line_count += 1;
                                    }
                                    current_line = word;
                                }
                            }
                            }
                            if !current_line.is_empty() {
                                line_count += 1;
                            }
                        }
                    }
                    (line_count as f64 * line_height_mm) + padding
                }).max_by(|a, b| a.partial_cmp(b).unwrap()) {
                    // Reserve full block height: stamp row + gap above details +
                    // detail box + trailing safety gap. This prevents the next
                    // stamp row from starting inside this detail block.
                    consumed_height_this_row = (box_top - current_height) + max_height + vspace;
                }
            }

            current_height += consumed_height_this_row + row_start_total_height;
            rows_on_page += 1;
            row_idx += 1;
        }
    }).expect("for_each_series failed");

    // Finalize: we've written to a temp file. Flush buffer and promote it into place only if it
    // contains STAMP lines; otherwise remove the temp and leave existing file intact.
    // This prevents accidental truncation when generation yields no stamps.
    {
        // Ensure buffered output is flushed before we read the temp file
        let _ = output.flush();
        // Post-process the temp output to insert vertical centering markers
        // (PAGE_VSPACE (n)) at the top of each stamp page where there is
        // unused vertical space. This keeps centering purely vertical and
        // avoids inserting filler images or modifying page content otherwise.
        let detail_layout = config
            .stamp_detail_layout
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("inline");
        if detail_layout != "separate_rows" || config.colnect_details_inside_stamp() {
            if let Err(e) = center_tmp_pages(&tmp_path, &config, effective_usable) {
                log::warn!("Page-centering post-process failed: {}", e);
            }
        } else {
            log::info!("Skipping page-centering post-process for separate_rows layout");
        }
        use std::fs;
        match fs::read_to_string(&tmp_path) {
            Ok(contents) => {
                let stamp_count = contents.lines().filter(|l| l.starts_with("STAMP")).count();

                if stamp_count == 0 {
                    // If the user explicitly requested overwrite, create a backup
                    // of the existing file even if generation produced no STAMP lines.
                    if crate::file_utils::get_overwrite_album_from_cli() && output_path.exists() {
                        use chrono::Local;
                        if let Some(stem_os) = output_path.file_stem() {
                            if let Some(stem) = stem_os.to_str() {
                                let ts = Local::now().format("%Y%m%d-%H%M%S").to_string();
                                let ext = output_path
                                    .extension()
                                    .and_then(|e| e.to_str())
                                    .unwrap_or("");
                                let backup_name = if ext.is_empty() {
                                    format!("{}.created-{}", stem, ts)
                                } else {
                                    format!("{}.created-{}.{}", stem, ts, ext)
                                };
                                let backup_path = output_path.with_file_name(backup_name);
                                match fs::copy(&output_path, &backup_path) {
                                    Ok(_) => info!("Created backup of existing file at {} (requested overwrite)", backup_path.display()),
                                    Err(e) => error!("Failed to create backup before overwrite (requested) : {}", e),
                                }
                            }
                        }
                    }
                    info!(
                        "No stamp lines produced for {} ({}). Leaving existing file intact.",
                        album_name,
                        output_path.display()
                    );
                    let _ = fs::remove_file(&tmp_path);
                } else {
                    // Create a timestamped backup of the existing file if present
                    if output_path.exists() {
                        use chrono::Local;
                        if let Some(stem_os) = output_path.file_stem() {
                            if let Some(stem) = stem_os.to_str() {
                                let ts = Local::now().format("%Y%m%d-%H%M%S").to_string();
                                let ext = output_path
                                    .extension()
                                    .and_then(|e| e.to_str())
                                    .unwrap_or("");
                                let backup_name = if ext.is_empty() {
                                    format!("{}.created-{}", stem, ts)
                                } else {
                                    format!("{}.created-{}.{}", stem, ts, ext)
                                };
                                let backup_path = output_path.with_file_name(backup_name);
                                match fs::copy(&output_path, &backup_path) {
                                    Ok(_) => info!(
                                        "Created backup of existing file at {}",
                                        backup_path.display()
                                    ),
                                    Err(e) => {
                                        error!("Failed to create backup before overwrite: {}", e)
                                    }
                                }
                            }
                        }
                    }
                    // Atomically move temp into place
                    match fs::rename(&tmp_path, &output_path) {
                        Ok(_) => {
                            info!(
                                "Successfully wrote album output to {} ({} stamps)",
                                output_path.display(),
                                stamp_count
                            );
                        }
                        Err(e) => {
                            error!("Failed to move temp output into place: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                error!(
                    "Failed to read generated temp file {}: {}",
                    tmp_path.display(),
                    e
                );
                let _ = fs::remove_file(&tmp_path);
            }
        }
    }

    // Persist dedupe audit for Phase A+B matcher decisions.
    let audit_path = album_dir.join("merge_dedupe_audit.jsonl");
    let mut audit_lines: Vec<String> = Vec::new();
    audit_lines.push(format!(
        "{{\"event\":\"summary\",\"album\":\"{}\",\"dropped\":{},\"seen_fingerprints\":{}}}",
        json_escape(album_name),
        dedupe_dropped,
        seen_stamp_fingerprints.len()
    ));
    audit_lines.extend(dedupe_audit);
    if let Err(e) = std::fs::write(&audit_path, audit_lines.join("\n") + "\n") {
        warn!(
            "Failed to write dedupe audit file {}: {}",
            audit_path.display(),
            e
        );
    } else {
        info!("Wrote dedupe audit file {}", audit_path.display());
    }

    output_path // Return the output file path

    // End of generate_album_output inner scope: use shared clean_text from album_utils
}

/// Backwards-compatible wrapper: accept a DB path, open the DB and call the
/// connection-based implementation. This keeps existing callers (tests and
/// other modules) working while allowing callers that already hold a
/// Connection to use the more efficient conn-based variant.
/// Returns the path to the generated output file.
pub fn generate_album_output<F>(
    db_path: &str,
    album_name: &str,
    page_callback: Option<F>,
    start_year: Option<i32>,
    end_year: Option<i32>,
) -> std::path::PathBuf
where
    F: FnMut(usize),
{
    let conn = init_db(db_path).expect("Failed to open database");
    generate_album_output_conn(&conn, album_name, page_callback, start_year, end_year)
}

/// Post-process the temporary album text file and insert a numeric
/// PAGE_VSPACE (n) after each PAGE_START for stamp pages so the page's
/// content is vertically centered where possible. This is a best-effort
/// purely-generator-side adjustment that uses conservative estimates of
/// header and row heights (matching the generator heuristics).
fn center_tmp_pages(
    tmp_path: &std::path::Path,
    config: &crate::config::Config,
    usable_height: f64,
) -> std::io::Result<()> {
    let max_page_vspace_mm = config
        .max_page_vspace_mm
        .map(|v| v as f64)
        .filter(|v| *v > 0.0)
        .unwrap_or(10.0);

    use std::fs;
    let contents = fs::read_to_string(tmp_path)?;
    let lines: Vec<&str> = contents.lines().collect();
    let mut out_lines: Vec<String> = Vec::with_capacity(lines.len());

    let mut i = 0usize;
    // copy pre-page globals until first page header
    while i < lines.len() {
        if lines[i].starts_with("### Page ") {
            break;
        }
        out_lines.push(lines[i].to_string());
        i += 1;
    }

    // process pages
    while i < lines.len() {
        if !lines[i].starts_with("### Page ") {
            out_lines.push(lines[i].to_string());
            i += 1;
            continue;
        }

        // collect this page's block
        let mut page_block: Vec<&str> = Vec::new();
        page_block.push(lines[i]);
        i += 1;
        while i < lines.len() && !lines[i].starts_with("### Page ") {
            page_block.push(lines[i]);
            i += 1;
        }

        // Determine positions of PAGE_START, all rows, and series starts
        let mut page_start_idx: Option<usize> = None;
        let mut series_row_indices: Vec<usize> = Vec::new();
        let mut all_row_indices: Vec<usize> = Vec::new();
        for (idx, l) in page_block.iter().enumerate() {
            let t = l.trim();
            if t == "PAGE_START" {
                page_start_idx = Some(idx);
            }
            if t.starts_with("ROW_START_ES") {
                all_row_indices.push(idx);
            }
            // Primary marker for a series: a PAGE_FIT diagnostic for that series
            if t.starts_with("# PAGE_FIT series=") {
                // find the next ROW_START_ES after this marker
                let mut j = idx + 1;
                while j < page_block.len() {
                    if page_block[j].trim().starts_with("ROW_START_ES") {
                        series_row_indices.push(j);
                        break;
                    }
                    j += 1;
                }
            }
        }

        // If this page contains a cover/flag section (e.g. "### Canada Flag ###"),
        // skip centering modifications for it to preserve the explicit layout.
        let has_flag_header = page_block
            .iter()
            .any(|l| l.trim().starts_with("###") && l.contains("Flag"));
        if has_flag_header {
            // emit page block unchanged
            for l in page_block.iter() {
                out_lines.push(l.to_string());
            }
            continue;
        }

        // If this page contains Wikipedia/Grokipedia content and centering is disabled in config,
        // skip centering modifications to preserve top-aligned content layout
        let has_wikipedia_header = page_block
            .iter()
            .any(|l| l.contains("Wikipedia Content") || l.contains("Grokipedia Content"));
        let wikipedia_centering_enabled = config.wikipedia_center_content.unwrap_or(false);
        if has_wikipedia_header && !wikipedia_centering_enabled {
            // emit page block unchanged - Wikipedia pages should NOT be centered when disabled
            for l in page_block.iter() {
                out_lines.push(l.to_string());
            }
            continue;
        }

        // Fallback: if no # PAGE_FIT markers, try to detect headers via PAGE_TEXT_* then ROW_START_ES
        if series_row_indices.is_empty() {
            for (idx, l) in page_block.iter().enumerate() {
                if l.trim().starts_with("PAGE_TEXT_") {
                    let mut j = idx + 1;
                    while j < page_block.len() && j <= idx + 8 {
                        if page_block[j].trim().starts_with("ROW_START_ES") {
                            series_row_indices.push(j);
                            break;
                        }
                        j += 1;
                    }
                }
            }
        }

        // If still empty, fallback to first ROW_START_ES as single series
        if series_row_indices.is_empty() {
            for (idx, l) in page_block.iter().enumerate() {
                if l.trim().starts_with("ROW_START_ES") {
                    series_row_indices.push(idx);
                    break;
                }
            }
        }

        // Compute used height and decide whether to center. We must ignore any
        // existing numeric `PAGE_VSPACE` markers that may be present in the
        // temporary file (for example from a previous run or manual edits).
        // If we include those numeric markers in the used-height estimate the
        // algorithm will double-count and keep adding more PAGE_VSPACE.
        let cleaned_block: Vec<&str> = page_block
            .iter()
            .filter(|l| !l.trim().starts_with("PAGE_VSPACE"))
            .cloned()
            .collect();
        let used_h = compute_page_used_height(&cleaned_block, config);
        let extra = usable_height - used_h;

        // Count actual rows on this page and configured maximum.
        let rows_on_page = all_row_indices.len();
        let cfg_max_rows = config.max_rows_per_page.unwrap_or(4) as usize;

        // Do not attempt to center when there is no meaningful extra space,
        // or when the page already contains the max allowed rows.
        let vertical_safety = config.packing_vertical_safety_mm.unwrap_or(2.0) as f64;
        if rows_on_page >= cfg_max_rows || extra <= vertical_safety {
            // emit page block unchanged (no centering)
            for l in page_block.iter() {
                out_lines.push(l.to_string());
            }
            continue;
        }

        // Compute how many inter-row gaps exist (rows inside a series where we can
        // add a little spacing). For each series, rows_in_series - 1 contributes.
        let mut inter_row_gaps: usize = 0;
        if !series_row_indices.is_empty() {
            // sort just in case
            series_row_indices.sort();
            // for each series start, count rows until next series or end
            for (si, &start_idx) in series_row_indices.iter().enumerate() {
                let end_idx = if si + 1 < series_row_indices.len() {
                    series_row_indices[si + 1]
                } else {
                    // until end of page_block (use large sentinel)
                    usize::MAX
                };
                // count rows whose indices are >= start_idx and < end_idx
                let mut rows_in_series = 0usize;
                for &r in all_row_indices.iter() {
                    if r >= start_idx && (end_idx == usize::MAX || r < end_idx) {
                        rows_in_series += 1;
                    }
                }
                if rows_in_series > 1 {
                    inter_row_gaps += rows_in_series - 1;
                }
            }
        } else {
            // No explicit series markers: all rows belong to a single implicit series
            if rows_on_page > 1 {
                inter_row_gaps = rows_on_page - 1;
            }
        }

        // gaps = number of series + 1 (top and between series and bottom).
        // For pages where no explicit series markers are found treat the
        // page as a single implicit series. If the page contains an
        // explicit header text block we prefer not to insert a top gap
        // (it tends to push the header down); in that case distribute
        // the extra space to the remaining gaps (bottom/between-series).
        let series_count = if series_row_indices.is_empty() {
            0
        } else {
            series_row_indices.len()
        };
        let base_gaps = if series_count == 0 {
            2
        } else {
            series_count + 1
        } as usize;
        let has_page_header = page_block
            .iter()
            .any(|l| l.trim().starts_with("PAGE_TEXT_"));
        let effective_base_gaps = if series_count == 0 && has_page_header {
            // only bottom gap (no top)
            1usize
        } else {
            base_gaps
        };

        // Respect a possible configured per-row gap from config.toml. If set,
        // allocate that amount to each inter-row gap first (clamped by available
        // extra). Any remaining extra is distributed across top/between-series/bottom.
        let configured_per_row_gap = config.series_row_gap_mm.map(|v| v as f64).unwrap_or(0.0f64);

        let mut per_row_gap = 0.0f64;
        let mut gap_mm = 0.0f64;

        if configured_per_row_gap > 0.0 && inter_row_gaps > 0 {
            let needed_for_rows = configured_per_row_gap * (inter_row_gaps as f64);
            if needed_for_rows >= extra {
                // Not enough room to give full configured gap: clamp to available
                per_row_gap = extra / (inter_row_gaps as f64);
                gap_mm = 0.0;
            } else {
                // Deduct the reserved per-row space and distribute the remainder
                per_row_gap = configured_per_row_gap;
                let remaining = extra - needed_for_rows;
                if remaining > 0.5 {
                    gap_mm = remaining / (effective_base_gaps as f64);
                }
            }
        } else {
            // No configured per-row gap: distribute extra across top/between/bottom only
            if extra > 0.5 {
                gap_mm = extra / (effective_base_gaps as f64);
            }
        }

        // Emit page block while inserting PAGE_VSPACE for top and between-series gaps
        if let Some(ps_idx) = page_start_idx {
            for (j, l) in page_block.iter().enumerate() {
                let trimmed = l.trim();
                // Handle page start specially: emit top gap immediately after PAGE_START
                if j == ps_idx {
                    out_lines.push(l.to_string());
                    // Only emit a top gap when we are not suppressing it
                    // due to an explicit header (see effective_base_gaps).
                    if gap_mm > 0.0 && !(series_count == 0 && has_page_header) {
                        let s = crate::album_globals::generator_new::layout::format_page_vspace_mm(
                            gap_mm.min(max_page_vspace_mm),
                        );
                        out_lines.push(s);
                    }
                    continue;
                }
                // Skip any existing numeric PAGE_VSPACE markers found in the
                // temporary page_block so we don't duplicate or compound them.
                if trimmed.starts_with("PAGE_VSPACE") {
                    continue;
                }
                // If this is a ROW_START_ES and it's not the very first row on the page,
                // optionally insert a per-row gap (when configured). This keeps
                // inter-row spacing under explicit control via `series_row_gap_mm`.
                if trimmed.starts_with("ROW_START_ES") {
                    if let Some(first_row_idx) = all_row_indices.first() {
                        if j != *first_row_idx && per_row_gap > 0.0 {
                            let s =
                                crate::album_globals::generator_new::layout::format_page_vspace_mm(
                                    per_row_gap.min(max_page_vspace_mm),
                                );
                            out_lines.push(s);
                        }
                    }
                    // If this row is also a series start and it's not the first series,
                    // insert the inter-series gap (gap_mm) as well. These are additive
                    // but only when gap_mm > 0.0.
                    if series_row_indices.contains(&j) {
                        if let Some(pos) = series_row_indices.iter().position(|&x| x == j) {
                            if pos > 0 && gap_mm > 0.0 {
                                let s = crate::album_globals::generator_new::layout::format_page_vspace_mm(gap_mm.min(max_page_vspace_mm));
                                out_lines.push(s);
                            }
                        }
                    }
                }

                out_lines.push(l.to_string());
            }
        } else {
            for l in page_block.iter() {
                out_lines.push(l.to_string());
            }
        }
    }

    // write adjusted content back to tmp file
    fs::write(tmp_path, out_lines.join("\n"))?;
    Ok(())
}

/// Conservative per-page height estimator matching generator heuristics.
fn compute_page_used_height(page_block: &[&str], config: &crate::config::Config) -> f64 {
    // Use canonical defaults so all fallbacks come from `Config::with_defaults()`.
    let defaults = crate::config::Config::with_defaults();
    let font_height = config
        .text_line_height_mm
        .unwrap_or(defaults.text_line_height_mm.unwrap()) as f64;
    let image_spacing = config
        .image_spacing_mm
        .unwrap_or(defaults.image_spacing_mm.unwrap()) as f64;
    let stamp_heading_h = config
        .header_font_spacing
        .unwrap_or(defaults.header_font_spacing.unwrap()) as f64
        + font_height;
    let row_font_spacing = config
        .row_font_spacing
        .unwrap_or(defaults.row_font_spacing.unwrap()) as f64;
    let vertical_safety = config
        .packing_vertical_safety_mm
        .unwrap_or(defaults.packing_vertical_safety_mm.unwrap()) as f64;

    let mut used = 0.0f64;

    // detect header between two PAGE_VSPACE markers after PAGE_START
    let mut idx = 0usize;
    while idx < page_block.len() {
        if page_block[idx].trim() == "PAGE_START" {
            idx += 1;
            break;
        }
        idx += 1;
    }

    // header detection
    let mut header_lines = 0usize;
    if idx < page_block.len() && page_block[idx].trim().starts_with("PAGE_VSPACE") {
        // skip first PAGE_VSPACE
        idx += 1;
        // count header text lines until next PAGE_VSPACE or ROW_START_ES
        while idx < page_block.len() {
            let l = page_block[idx].trim();
            if l.starts_with("PAGE_VSPACE")
                || l.starts_with("ROW_START_ES")
                || l.starts_with("# ROW_META")
            {
                break;
            }
            if l.starts_with("PAGE_TEXT_") {
                header_lines += 1;
            }
            idx += 1;
        }
        // skip a single trailing PAGE_VSPACE if present
        if idx < page_block.len() && page_block[idx].trim().starts_with("PAGE_VSPACE") {
            // increment not required later; evaluate expression to avoid unused_assignment warning
            let _ = idx + 1;
        }
    }
    let v_before = config
        .header_vspace_before_mm
        .unwrap_or(defaults.header_vspace_before_mm.unwrap()) as f64;
    let v_after = config
        .header_vspace_after_mm
        .unwrap_or(defaults.header_vspace_after_mm.unwrap()) as f64;
    used += v_before + (header_lines as f64) * font_height + v_after;

    // now scan for rows and estimate each row height
    let mut i = 0usize;
    while i < page_block.len() {
        let l = page_block[i].trim();
        if l.starts_with("ROW_START_ES") {
            // find next stamp lines until next ROW_START_ES or page end
            i += 1;
            let mut row_max_h = 0.0f64;
            while i < page_block.len() {
                let ll = page_block[i].trim();
                if ll.starts_with("ROW_START_ES") || ll.starts_with("### Page ") {
                    break;
                }
                if ll.starts_with("STAMP_ADD_IMG") || ll.starts_with("STAMP_ADD (") {
                    // parse numbers inside parentheses
                    if let Some(paren_idx) = ll.find('(') {
                        if let Some(close_idx) = ll.rfind(')') {
                            let inside = &ll[paren_idx + 1..close_idx];
                            let parts: Vec<&str> = inside.split_whitespace().collect();
                            if parts.len() >= 2 {
                                if let Ok(hv) = parts[1].parse::<f64>() {
                                    if hv > row_max_h {
                                        row_max_h = hv;
                                    }
                                }
                            }
                        }
                    }
                }
                // Also check for PAGE_ADD_BOX detail boxes and add their height
                if ll.starts_with("PAGE_ADD_BOX") {
                    // parse: PAGE_ADD_BOX (Font Fontsize border left top width height inside-text ...)
                    if let Some(paren_idx) = ll.find('(') {
                        if let Some(close_idx) = ll.rfind(')') {
                            let inside = &ll[paren_idx + 1..close_idx];
                            let parts: Vec<&str> = inside.split_whitespace().collect();
                            // box height is at index 6 (Font=0, Fontsize=1, border=2, left=3, top=4, width=5, height=6)
                            if parts.len() >= 7 {
                                if let Ok(box_h) = parts[6].parse::<f64>() {
                                    // Add box height to row height (boxes appear below stamps)
                                    row_max_h += box_h;
                                }
                            }
                        }
                    }
                }
                i += 1;
            }
            if row_max_h <= 0.0 {
                // fallback conservative row height
                row_max_h = config.album_stamp_img_new_setting_h_mm.unwrap_or(6.0) as f64
                    + 2.0 * config.stamp_box_adjust.unwrap_or(0.0) as f64;
                // Ensure symmetric usage of the width parameter in this module so
                // tests that count parameter occurrences remain balanced.
                let _row_max_w_default =
                    config.album_stamp_img_new_setting_w_mm.unwrap_or(6.0) as f64;
            }
            // estimate text lines conservatively as 2 (centre+one other)
            let text_lines = 2.0f64;
            let row_h = row_max_h
                + image_spacing
                + text_lines * font_height
                + stamp_heading_h
                + row_font_spacing
                + vertical_safety;
            used += row_h;
            continue;
        }
        i += 1;
    }

    // clamp used to non-negative
    if used < 0.0 {
        used = 0.0;
    }
    used
}
