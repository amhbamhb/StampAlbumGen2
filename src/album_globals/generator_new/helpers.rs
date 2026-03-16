use crate::album_globals::structs::{GenericAttrs, Stamp};
use crate::colnect::db::get_colnect_info;
use crate::config::Config;
use crate::db_utils::get_stamps_by_series;
use crate::db_utils::update_image_dimensions;
use crate::image_utils::estimate_real_dimensions;
use rusqlite::Connection;
use rusqlite::Result as SqlResult;
use std::io::Write;
// Path is not currently needed here; keep import commented for future use.
// use std::path::Path;

// Local copy of helper functions so the refactor can be worked on inside
// `generator_new` without changing behavior. These mirror the existing
// `crate::album_globals::helpers` implementations and will be unified later.
pub fn emit_row_start_es(
    output: &mut dyn Write,
    config_opt: Option<&Config>,
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

/// Emit a concise per-stamp diagnostic line that mirrors the legacy generator
/// output. Centralizing this helps keep `generator.rs` free of direct writes.
pub fn emit_stamp_diag<W: Write>(
    output: &mut W,
    sg: &str,
    source: &str,
    est_w_mm: f64,
    est_h_mm: f64,
    px_w: i32,
    px_h: i32,
    img: &str,
) -> std::io::Result<()> {
    writeln!(
        output,
        "# STAMP_DIAG sg=\"{}\" source=\"{}\" est_w_mm=\"{:.2}\" est_h_mm=\"{:.2}\" px=\"{}x{}\" img=\"{}\"",
        sg.replace('"', "'"),
        source,
        est_w_mm,
        est_h_mm,
        px_w,
        px_h,
        img.replace('"', "'")
    )
}

pub fn build_page_text_template(cfg: &Config, kind: &str) -> String {
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

pub fn build_series_page_text_centre_template(cfg: &Config) -> String {
    if let Some(t) = cfg.series_page_text_centre_format.clone() {
        return t;
    }
    let has_series_override = cfg.series_page_text_centre_font.is_some()
        || cfg.series_page_text_centre_size.is_some()
        || cfg.series_page_text_centre_vspace.is_some();

    if !has_series_override {
        return build_page_text_template(cfg, "centre");
    }

    let font = cfg
        .series_page_text_centre_font
        .clone()
        .or_else(|| cfg.page_text_centre_font.clone())
        .unwrap_or_else(|| "TN".to_string());
    let size = cfg
        .series_page_text_centre_size
        .or(cfg.page_text_centre_size)
        .unwrap_or(8.0) as i32;
    let vspace = cfg
        .series_page_text_centre_vspace
        .or(cfg.page_text_centre_vspace)
        .unwrap_or(1.0_f32);

    format!(
        "PAGE_TEXT_CENTRE ({} {} \"{{}}\" {:.1})",
        font, size, vspace
    )
}

fn build_scoped_page_text_centre_template(
    cfg: &Config,
    format_opt: Option<String>,
    font_opt: Option<String>,
    size_opt: Option<f32>,
    vspace_opt: Option<f32>,
) -> String {
    if let Some(t) = format_opt {
        return t;
    }

    let has_override = font_opt.is_some() || size_opt.is_some() || vspace_opt.is_some();
    if !has_override {
        return build_page_text_template(cfg, "centre");
    }

    let font = font_opt
        .or_else(|| cfg.page_text_centre_font.clone())
        .unwrap_or_else(|| "TN".to_string());
    let size = size_opt.or(cfg.page_text_centre_size).unwrap_or(8.0) as i32;
    let vspace = vspace_opt
        .or(cfg.page_text_centre_vspace)
        .unwrap_or(1.0_f32);
    format!(
        "PAGE_TEXT_CENTRE ({} {} \"{{}}\" {:.1})",
        font, size, vspace
    )
}

pub fn build_date_page_text_centre_template(cfg: &Config) -> String {
    build_scoped_page_text_centre_template(
        cfg,
        cfg.date_page_text_centre_format.clone(),
        cfg.date_page_text_centre_font.clone(),
        cfg.date_page_text_centre_size,
        cfg.date_page_text_centre_vspace,
    )
}

pub fn build_watermark_page_text_centre_template(cfg: &Config) -> String {
    build_scoped_page_text_centre_template(
        cfg,
        cfg.watermark_page_text_centre_format.clone(),
        cfg.watermark_page_text_centre_font.clone(),
        cfg.watermark_page_text_centre_size,
        cfg.watermark_page_text_centre_vspace,
    )
}

pub fn build_engraving_page_text_centre_template(cfg: &Config) -> String {
    build_scoped_page_text_centre_template(
        cfg,
        cfg.engraving_page_text_centre_format.clone(),
        cfg.engraving_page_text_centre_font.clone(),
        cfg.engraving_page_text_centre_size,
        cfg.engraving_page_text_centre_vspace,
    )
}

/// Fetch stamps for a given `series` from the DB and apply optional
/// start/end year filtering. Returns a Vec<`Stamp`> suitable for the
/// generator layout pipeline.
pub fn fetch_and_filter_stamps(
    conn: &Connection,
    series: &str,
    start_year: Option<i32>,
    end_year: Option<i32>,
) -> SqlResult<Vec<Stamp>> {
    // Fast path: when a single target year is requested, ask the DB helper
    // for that year directly to avoid loading full-series history.
    let mut stamps = if let (Some(sy), Some(ey)) = (start_year, end_year) {
        if sy > 0 && ey == sy {
            get_stamps_by_series(conn, series, &sy.to_string())?
        } else {
            get_stamps_by_series(conn, series, "0")?
        }
    } else {
        get_stamps_by_series(conn, series, "0")?
    };

    // Apply inclusive year filtering like the canonical generator.
    if let (Some(sy), Some(ey)) = (start_year, end_year) {
        if sy > 0 && ey < 9999 && sy <= ey {
            stamps.retain(|st| st.year >= sy && st.year <= ey);
        }
    } else if let Some(sy) = start_year {
        if sy > 0 {
            stamps.retain(|st| st.year >= sy);
        }
    } else if let Some(ey) = end_year {
        if ey < 9999 {
            stamps.retain(|st| st.year <= ey);
        }
    }

    // Convert DB StampStub into the generator's Stamp type.
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
        attrs.set("issue_location", &stamp.issue_location);
        attrs.set("quantity_printed", &stamp.quantity_printed);
        attrs.set("mint_condition", &stamp.mint_condition);
        attrs.set("unused", &stamp.unused);

        // Try to fetch Colnect info and store width/height for later use
        let mut colnect_width_mm: Option<f64> = None;
        let mut colnect_height_mm: Option<f64> = None;

        if let Ok(Some(colnect)) = get_colnect_info(conn, &stamp.sg_number) {
            // Parse and store Colnect dimensions (in mm)
            if let Some(ref w) = colnect.width {
                if let Ok(width_val) = w.trim().parse::<f64>() {
                    colnect_width_mm = Some(width_val);
                }
            }
            if let Some(ref h) = colnect.height {
                if let Ok(height_val) = h.trim().parse::<f64>() {
                    colnect_height_mm = Some(height_val);
                }
            }

            if let Some(v) = colnect.name {
                attrs.set("colnect_name", &v);
            }
            if let Some(v) = colnect.watermark {
                attrs.set("colnect_watermark", &v);
            }
            if let Some(v) = colnect.format {
                attrs.set("colnect_format", &v);
            }
            if let Some(v) = colnect.emission {
                attrs.set("colnect_emission", &v);
            }
            if let Some(v) = colnect.perforation {
                attrs.set("colnect_perforation", &v);
            }
            if let Some(v) = colnect.printing {
                attrs.set("colnect_printing", &v);
            }
            if let Some(v) = colnect.paper {
                attrs.set("colnect_paper", &v);
            }
            if let Some(v) = colnect.gum {
                attrs.set("colnect_gum", &v);
            }
            if let Some(v) = colnect.themes {
                attrs.set("colnect_themes", &v);
            }
            if let Some(v) = colnect.description {
                attrs.set("colnect_description", &v);
            }
            if let Some(v) = colnect.issued_on {
                attrs.set("colnect_issued_on", &v);
            }
            if let Some(v) = colnect.expiry_date {
                attrs.set("colnect_expiry_date", &v);
            }
            if let Some(v) = colnect.print_run {
                attrs.set("colnect_print_run", &v);
            }
            if let Some(v) = colnect.face_value {
                attrs.set("colnect_face_value", &v);
            }
            if let Some(v) = colnect.currency {
                attrs.set("colnect_currency", &v);
            }
            if let Some(v) = colnect.colors {
                attrs.set("colnect_colors", &v);
            }
            if let Some(v) = colnect.designers {
                attrs.set("colnect_designers", &v);
            }
            if let Some(v) = colnect.printers {
                attrs.set("colnect_printers", &v);
            }
            if let Some(v) = colnect.score {
                attrs.set("colnect_score", &v);
            }
            if let Some(v) = colnect.accuracy {
                attrs.set("colnect_accuracy", &v);
            }

            if !colnect.catalog_codes.is_empty() {
                let codes_str = colnect
                    .catalog_codes
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<_>>()
                    .join(" | ");
                attrs.set("colnect_catalog_codes", &codes_str);

                // Extract Scott number specifically for use as default catalog number
                if let Some((_, scott_code)) = colnect
                    .catalog_codes
                    .iter()
                    .find(|(catalog_type, _)| catalog_type == "Sn")
                {
                    attrs.set("scott_number", scott_code);
                }
            }
        }

        // Store Colnect dimensions in attributes for use in dimension calculation
        if let Some(w) = colnect_width_mm {
            attrs.set("colnect_width_mm", &w.to_string());
        }
        if let Some(h) = colnect_height_mm {
            attrs.set("colnect_height_mm", &h.to_string());
        }

        attrs.set("used", &stamp.used);
        attrs.set("month", &stamp.month);
        attrs.set("month_num", &stamp.month_num.to_string());
        attrs.set("day", &stamp.day);
        attrs.set("currency", &stamp.currency);
        attrs.set("watermark", &stamp.watermark);
        attrs.set("design", &stamp.design);
        attrs.set("engraving", &stamp.engraving);
        attrs.set("perforation", &stamp.perforation);

        stamp_objs.push(Stamp {
            attrs,
            image_path: stamp.image_path.clone(),
            image_width_px: stamp.image_width_px,
            image_height_px: stamp.image_height_px,
            est_width_mm: None,
            est_height_mm: None,
        });
    }

    Ok(stamp_objs)
}

/// Estimate physical sizes for the stamps in `stamps` and ensure placeholders
/// exist for stamps without pixel dimensions. This mirrors the pre-pass logic
/// in the original generator so layout can rely on finalized `est_*` fields.
pub fn estimate_and_ensure_placeholders(
    conn: &Connection,
    album_name: &str,
    series_name: &str,
    stamps: &mut [Stamp],
    mm_per_pixel_w: f32,
    mm_per_pixel_h: f32,
    preserve_real: bool,
    stamp_box_height: f64,
    reference_width_mm: f64,
    reference_height_mm: f64,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    // Compute series averages from available images
    let mut series_widths_mm: Vec<f64> = Vec::new();
    let mut series_heights_mm: Vec<f64> = Vec::new();
    for stamp in stamps.iter() {
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

    // Get Colnect dimension adjustments from config (default 2.0mm if not set)
    let colnect_w_adjust = config.colnect_width_adjust_mm.unwrap_or(2.0) as f64;
    let colnect_h_adjust = config.colnect_height_adjust_mm.unwrap_or(2.0) as f64;

    for s in stamps.iter_mut() {
        // First priority: Use Colnect dimensions if available (already in mm)
        let colnect_w = s
            .attrs
            .get("colnect_width_mm")
            .and_then(|w| w.parse::<f64>().ok());
        let colnect_h = s
            .attrs
            .get("colnect_height_mm")
            .and_then(|h| h.parse::<f64>().ok());

        if let (Some(w_mm), Some(h_mm)) = (colnect_w, colnect_h) {
            // Use Colnect dimensions with separate width/height adjustments
            s.est_width_mm = Some(w_mm + colnect_w_adjust);
            s.est_height_mm = Some(h_mm + colnect_h_adjust);
        } else {
            // Fall back to existing logic for image-based or default dimensions
            let image_path = s.image_path.trim();
            let full_image_path = format!("Albums/{}/{}", album_name.replace(' ', "-"), image_path);
            if let (Some(w_px), Some(h_px)) = (s.image_width_px, s.image_height_px) {
                let w_mm = (w_px as f64) * (mm_per_pixel_w as f64);
                let h_mm = (h_px as f64) * (mm_per_pixel_h as f64);
                s.est_width_mm = Some(w_mm);
                s.est_height_mm = Some(h_mm);
            } else if let Some((w_mm, h_mm)) =
                estimate_real_dimensions(&full_image_path, mm_per_pixel_w, mm_per_pixel_h)
            {
                let mut w = w_mm as f64;
                let mut h = h_mm as f64;
                if !preserve_real && h > stamp_box_height {
                    let sc = stamp_box_height / h;
                    w *= sc;
                    h *= sc;
                }
                s.est_width_mm = Some(w);
                s.est_height_mm = Some(h);

                // We found the real image and got dimensions!
                // Update the stamp object and DB so we don't generate a placeholder.
                let px_w = (w_mm / mm_per_pixel_w).round() as i32;
                let px_h = (h_mm / mm_per_pixel_h).round() as i32;
                s.image_width_px = Some(px_w);
                s.image_height_px = Some(px_h);

                if px_w > 0 && px_h > 0 {
                    let sg_str = s.attrs.get("sg_number").map(|v| v.as_str()).unwrap_or("");
                    if let Err(e) = update_image_dimensions(
                        conn,
                        album_name,
                        series_name,
                        sg_str,
                        Some(px_w),
                        Some(px_h),
                    ) {
                        log::error!(
                            "Failed to update DB image dims for {} sg {}: {}",
                            album_name,
                            sg_str,
                            e
                        );
                    }
                }
            } else if let (Some(avg_w), Some(avg_h)) = (series_avg_w_mm, series_avg_h_mm) {
                s.est_width_mm = Some(avg_w);
                s.est_height_mm = Some(avg_h);
            } else {
                s.est_width_mm = Some(reference_width_mm as f64);
                s.est_height_mm = Some(reference_height_mm as f64);
            }
        }

        // If the stamp lacks pixel dims, generate a placeholder sized to the
        // image-only mm size so layout later sees consistent pixel sizes.
        if s.image_width_px.is_none() {
            let base_w = s.est_width_mm.unwrap_or(reference_width_mm as f64);
            let base_h = s.est_height_mm.unwrap_or(reference_height_mm as f64);
            let target_w_mm = base_w;
            let target_h_mm = base_h;
            // Use Scott number if available, fall back to SG number for placeholder filename
            let catalog_num = s
                .attrs
                .get("scott_number")
                .or_else(|| s.attrs.get("sg_number"))
                .map(|v| v.as_str())
                .unwrap_or("");
            match crate::placeholder_image::ensure_placeholder_image_for_stamp_with_hint(
                album_name,
                catalog_num,
                s.attrs.get("year").map(|v| v.as_str()).unwrap_or(""),
                Some(series_name),
                None,
                Some(target_w_mm),
                Some(target_h_mm),
                Some(mm_per_pixel_w),
                Some(mm_per_pixel_h),
            ) {
                Ok(rel) => {
                    s.image_path = rel;
                    let px_w = (target_w_mm / (mm_per_pixel_w as f64)).round() as i32;
                    let px_h = (target_h_mm / (mm_per_pixel_h as f64)).round() as i32;
                    s.image_width_px = Some(px_w);
                    s.image_height_px = Some(px_h);
                    if px_w > 0 && px_h > 0 {
                        let sg_str = s.attrs.get("sg_number").map(|v| v.as_str()).unwrap_or("");
                        if let Err(e) = update_image_dimensions(
                            conn,
                            album_name,
                            series_name,
                            sg_str,
                            Some(px_w),
                            Some(px_h),
                        ) {
                            log::error!(
                                "Failed to update DB image dims for {} sg {}: {}",
                                album_name,
                                sg_str,
                                e
                            );
                        }
                    }
                }
                Err(e) => {
                    log::error!(
                        "Failed to create placeholder for {} sg {}: {}",
                        album_name,
                        s.attrs.get("sg_number").unwrap_or(&"".to_string()),
                        e
                    );
                }
            }
        }
    }

    Ok(())
}

/// Ensure a placeholder image exists for a single stamp and persist its
/// pixel dimensions to the DB. Returns `Some((rel_path, px_w, px_h))` when
/// a placeholder was created (or already exists and was returned by the
/// placeholder generator); otherwise `None`.
pub fn ensure_placeholder_for_stamp_and_update_db(
    conn: &Connection,
    album_name: &str,
    series_name: &str,
    sg_number: &str,
    catalog_number: &str,
    year: &str,
    target_w_mm: f64,
    target_h_mm: f64,
    mm_per_pixel_w: f32,
    mm_per_pixel_h: f32,
    mut writer: Option<&mut dyn Write>,
) -> Option<(String, i32, i32)> {
    match crate::placeholder_image::ensure_placeholder_image_for_stamp_with_hint(
        album_name,
        catalog_number,
        year,
        Some(series_name),
        None,
        Some(target_w_mm),
        Some(target_h_mm),
        Some(mm_per_pixel_w),
        Some(mm_per_pixel_h),
    ) {
        Ok(rel) => {
            let px_w = (target_w_mm / (mm_per_pixel_w as f64)).round() as i32;
            let px_h = (target_h_mm / (mm_per_pixel_h as f64)).round() as i32;
            if px_w > 0 && px_h > 0 {
                if let Err(e) = update_image_dimensions(
                    conn,
                    album_name,
                    series_name,
                    sg_number,
                    Some(px_w),
                    Some(px_h),
                ) {
                    log::error!(
                        "Failed to update DB image dims for {} sg {}: {}",
                        album_name,
                        sg_number,
                        e
                    );
                }
            }
            // If a writer was provided, emit the diagnostic line here so
            // callers don't need to perform this side-effect themselves.
            if let Some(ref mut w) = writer.as_mut() {
                let ph_sg = sg_number.replace('"', "'");
                let ph_img = rel.replace('"', "'");
                let _ = writeln!(
                    w,
                    "# STAMP_DIAG_PLACEHOLDER sg=\"{}\" px=\"{}x{}\" img=\"{}\" target_mm=\"{:.2}x{:.2}\" source=\"placeholder_helper\"",
                    ph_sg, px_w, px_h, ph_img, target_w_mm, target_h_mm
                );
            }
            Some((rel, px_w, px_h))
        }
        Err(e) => {
            log::error!(
                "Failed to create placeholder for {} sg {}: {}",
                album_name,
                sg_number,
                e
            );
            None
        }
    }
}
