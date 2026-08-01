use crate::album_globals::structs::{GenericAttrs, Stamp};
use crate::colnect::db::{get_colnect_info, get_colnect_info_by_id};
use crate::config::Config;
use crate::db_utils::get_stamps_by_series;
use crate::db_utils::update_image_dimensions;
use crate::image_utils::estimate_real_dimensions;
use rusqlite::Connection;
use rusqlite::OptionalExtension;
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

#[derive(Debug, Default)]
struct StoredColnectMeta {
    colnect_id: String,
    catalog_code: String,
    colnect_name: String,
    colnect_catalog_codes: String,
    colnect_width: String,
    colnect_height: String,
    colnect_format: String,
    colnect_emission: String,
    colnect_printing: String,
    colnect_paper: String,
    colnect_gum: String,
    colnect_expiry_date: String,
    colnect_print_run: String,
    colnect_face_value: String,
    colnect_currency: String,
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
    colnect_ord_sg: String,
}

fn load_stored_colnect_meta(
    conn: &Connection,
    stamp: &crate::db_utils::StampStub,
) -> SqlResult<Option<StoredColnectMeta>> {
    conn.query_row(
        "SELECT COALESCE(colnect_id, ''),
            COALESCE(catalog_code, ''),
                COALESCE(colnect_name, ''),
                COALESCE(colnect_catalog_codes, ''),
                COALESCE(colnect_width, ''),
                COALESCE(colnect_height, ''),
                COALESCE(colnect_format, ''),
                COALESCE(colnect_emission, ''),
                COALESCE(colnect_printing, ''),
                COALESCE(colnect_paper, ''),
                COALESCE(colnect_gum, ''),
                COALESCE(colnect_expiry_date, ''),
                COALESCE(colnect_print_run, ''),
                COALESCE(colnect_face_value, ''),
                COALESCE(colnect_currency, ''),
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
                COALESCE(colnect_private_note, ''),
                COALESCE(colnect_ord_sg, '')
         FROM stamps
         WHERE series = ?1
           AND year = ?2
           AND sg_number = ?3
         ORDER BY CASE WHEN image_path = ?4 THEN 0 ELSE 1 END, rowid DESC
         LIMIT 1",
        [
            &stamp.series,
            &stamp.year.to_string(),
            &stamp.sg_number,
            &stamp.image_path,
        ],
        |row| {
            Ok(StoredColnectMeta {
                colnect_id: row.get(0)?,
                catalog_code: row.get(1)?,
                colnect_name: row.get(2)?,
                colnect_catalog_codes: row.get(3)?,
                colnect_width: row.get(4)?,
                colnect_height: row.get(5)?,
                colnect_format: row.get(6)?,
                colnect_emission: row.get(7)?,
                colnect_printing: row.get(8)?,
                colnect_paper: row.get(9)?,
                colnect_gum: row.get(10)?,
                colnect_expiry_date: row.get(11)?,
                colnect_print_run: row.get(12)?,
                colnect_face_value: row.get(13)?,
                colnect_currency: row.get(14)?,
                colnect_colors: row.get(15)?,
                colnect_parent_item: row.get(16)?,
                colnect_variant: row.get(17)?,
                colnect_overprint_variant: row.get(18)?,
                colnect_score: row.get(19)?,
                colnect_accuracy: row.get(20)?,
                colnect_list: row.get(21)?,
                colnect_quantity: row.get(22)?,
                colnect_condition: row.get(23)?,
                colnect_public_note: row.get(24)?,
                colnect_private_note: row.get(25)?,
                colnect_ord_sg: row.get(26)?,
            })
        },
    )
    .optional()
}

fn set_attr_if_non_empty(attrs: &mut GenericAttrs, key: &str, value: &str) {
    if !value.trim().is_empty() {
        attrs.set(key, value);
    }
}

fn set_attr_if_missing(attrs: &mut GenericAttrs, key: &str, value: Option<String>) {
    let missing = attrs.get(key).map_or(true, |v| v.trim().is_empty());
    if missing {
        if let Some(v) = value {
            if !v.trim().is_empty() {
                attrs.set(key, &v);
            }
        }
    }
}

fn leading_year_from_series(series: &str) -> Option<i32> {
    let t = series.trim_start();
    if t.len() < 4 {
        return None;
    }
    let prefix: String = t.chars().take(4).collect();
    if prefix.chars().all(|c| c.is_ascii_digit()) {
        prefix.parse::<i32>().ok()
    } else {
        None
    }
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
    let default_catalog_code =
        crate::config::load_effective_config("config.toml").effective_default_catalog_code();
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

    // Guard against mixed/legacy records where the series title carries an
    // explicit year (e.g. "1970 ...") that conflicts with row.year.
    // In ranged runs, trust the explicit series year for inclusion checks.
    if let (Some(sy), Some(ey)) = (start_year, end_year) {
        if sy > 0 && ey < 9999 && sy <= ey {
            stamps.retain(|st| {
                if let Some(series_year) = leading_year_from_series(&st.series) {
                    series_year >= sy && series_year <= ey
                } else {
                    true
                }
            });
        }
    }

    // Convert DB StampStub into the generator's Stamp type.
    let mut stamp_objs: Vec<Stamp> = Vec::new();
    for stamp in stamps {
        let mut attrs = GenericAttrs::new();
        attrs.set("year", &stamp.year.to_string());
        attrs.set("sg_number", &stamp.sg_number);
        attrs.set("type", &stamp.r#type);
        // Pin catalog_number to the stamps-table sg_number before any Colnect
        // enrichment runs.  This ensures user edits made in the Review UI are
        // the source of truth for the STAMP_HEADING catalog number. Colnect
        // catalog codes are stored separately in colnect_catalog_codes and are
        // only used as a last resort when the stamps table has no number at all.
        if !stamp.sg_number.trim().is_empty() {
            attrs.set("catalog_number", &stamp.sg_number);
        }
        attrs.set("denomination", &stamp.denomination);
        attrs.set("color", &stamp.color);
        attrs.set("description", &stamp.description);
        attrs.set("stamps_issued", &stamp.stamps_issued);
        attrs.set("issue_location", &stamp.issue_location);
        attrs.set("quantity_printed", &stamp.quantity_printed);
        attrs.set("mint_condition", &stamp.mint_condition);
        attrs.set("unused", &stamp.unused);

        // Prefer Colnect values already merged into `stamps` rows via Review UI.
        // This keeps output aligned with user-selected metadata even when SG mapping
        // does not match Colnect sg_number exactly.
        let stored_colnect = load_stored_colnect_meta(conn, &stamp).ok().flatten();
        let lookup_colnect_id = stored_colnect.as_ref().map(|m| m.colnect_id.clone());
        if let Some(meta) = stored_colnect.as_ref() {
            set_attr_if_non_empty(&mut attrs, "catalog_code", &meta.catalog_code);
            set_attr_if_non_empty(&mut attrs, "colnect_name", &meta.colnect_name);
            set_attr_if_non_empty(
                &mut attrs,
                "colnect_catalog_codes",
                &meta.colnect_catalog_codes,
            );
            set_attr_if_non_empty(&mut attrs, "colnect_width_mm", &meta.colnect_width);
            set_attr_if_non_empty(&mut attrs, "colnect_height_mm", &meta.colnect_height);
            set_attr_if_non_empty(&mut attrs, "colnect_format", &meta.colnect_format);
            set_attr_if_non_empty(&mut attrs, "colnect_emission", &meta.colnect_emission);
            set_attr_if_non_empty(&mut attrs, "colnect_printing", &meta.colnect_printing);
            set_attr_if_non_empty(&mut attrs, "colnect_paper", &meta.colnect_paper);
            set_attr_if_non_empty(&mut attrs, "colnect_gum", &meta.colnect_gum);
            set_attr_if_non_empty(&mut attrs, "colnect_expiry_date", &meta.colnect_expiry_date);
            set_attr_if_non_empty(&mut attrs, "colnect_print_run", &meta.colnect_print_run);
            set_attr_if_non_empty(&mut attrs, "colnect_face_value", &meta.colnect_face_value);
            set_attr_if_non_empty(&mut attrs, "colnect_currency", &meta.colnect_currency);
            set_attr_if_non_empty(&mut attrs, "colnect_colors", &meta.colnect_colors);
            set_attr_if_non_empty(&mut attrs, "colnect_parent_item", &meta.colnect_parent_item);
            set_attr_if_non_empty(&mut attrs, "colnect_variant", &meta.colnect_variant);
            set_attr_if_non_empty(
                &mut attrs,
                "colnect_overprint_variant",
                &meta.colnect_overprint_variant,
            );
            set_attr_if_non_empty(&mut attrs, "colnect_score", &meta.colnect_score);
            set_attr_if_non_empty(&mut attrs, "colnect_accuracy", &meta.colnect_accuracy);
            set_attr_if_non_empty(&mut attrs, "colnect_list", &meta.colnect_list);
            set_attr_if_non_empty(&mut attrs, "colnect_quantity", &meta.colnect_quantity);
            set_attr_if_non_empty(&mut attrs, "colnect_condition", &meta.colnect_condition);
            set_attr_if_non_empty(&mut attrs, "colnect_public_note", &meta.colnect_public_note);
            set_attr_if_non_empty(
                &mut attrs,
                "colnect_private_note",
                &meta.colnect_private_note,
            );
            set_attr_if_non_empty(&mut attrs, "colnect_ord_sg", &meta.colnect_ord_sg);
        }

        // Enrich missing attributes from Colnect reference tables.
        let colnect_lookup = if let Some(cid) = lookup_colnect_id.filter(|id| !id.trim().is_empty())
        {
            get_colnect_info_by_id(conn, &cid).ok().flatten()
        } else {
            get_colnect_info(conn, &stamp.sg_number).ok().flatten()
        };

        if let Some(colnect) = colnect_lookup {
            set_attr_if_missing(&mut attrs, "colnect_country", colnect.country);
            set_attr_if_missing(&mut attrs, "colnect_page_url", colnect.colnect_link);
            set_attr_if_missing(&mut attrs, "colnect_name", colnect.name);
            set_attr_if_missing(&mut attrs, "colnect_watermark", colnect.watermark);
            set_attr_if_missing(&mut attrs, "colnect_format", colnect.format);
            set_attr_if_missing(&mut attrs, "colnect_emission", colnect.emission);
            set_attr_if_missing(&mut attrs, "colnect_perforation", colnect.perforation);
            set_attr_if_missing(&mut attrs, "colnect_printing", colnect.printing);
            set_attr_if_missing(&mut attrs, "colnect_paper", colnect.paper);
            set_attr_if_missing(&mut attrs, "colnect_gum", colnect.gum);
            set_attr_if_missing(&mut attrs, "colnect_themes", colnect.themes);
            set_attr_if_missing(&mut attrs, "colnect_description", colnect.description);
            set_attr_if_missing(&mut attrs, "colnect_issued_on", colnect.issued_on);
            set_attr_if_missing(&mut attrs, "colnect_expiry_date", colnect.expiry_date);
            set_attr_if_missing(&mut attrs, "colnect_print_run", colnect.print_run);
            set_attr_if_missing(&mut attrs, "colnect_face_value", colnect.face_value);
            set_attr_if_missing(&mut attrs, "colnect_currency", colnect.currency);
            set_attr_if_missing(&mut attrs, "colnect_colors", colnect.colors);
            set_attr_if_missing(&mut attrs, "colnect_designers", colnect.designers);
            set_attr_if_missing(&mut attrs, "colnect_printers", colnect.printers);
            set_attr_if_missing(&mut attrs, "colnect_score", colnect.score);
            set_attr_if_missing(&mut attrs, "colnect_accuracy", colnect.accuracy);
            set_attr_if_missing(&mut attrs, "colnect_width_mm", colnect.width);
            set_attr_if_missing(&mut attrs, "colnect_height_mm", colnect.height);
            set_attr_if_missing(&mut attrs, "colnect_ord_mi", colnect.ord_mi);
            set_attr_if_missing(&mut attrs, "colnect_ord_sn", colnect.ord_sn);
            set_attr_if_missing(&mut attrs, "colnect_ord_yt", colnect.ord_yt);
            set_attr_if_missing(&mut attrs, "colnect_ord_sg", colnect.ord_sg);
            set_attr_if_missing(&mut attrs, "colnect_parent_item", colnect.parent_item);
            set_attr_if_missing(&mut attrs, "colnect_variant", colnect.variant);
            set_attr_if_missing(
                &mut attrs,
                "colnect_overprint_variant",
                colnect.overprint_variant,
            );
            set_attr_if_missing(&mut attrs, "colnect_list", colnect.list);
            set_attr_if_missing(&mut attrs, "colnect_quantity", colnect.quantity);
            set_attr_if_missing(&mut attrs, "colnect_condition", colnect.condition);
            set_attr_if_missing(&mut attrs, "colnect_public_note", colnect.public_note);
            set_attr_if_missing(&mut attrs, "colnect_private_note", colnect.private_note);

            if attrs
                .get("colnect_catalog_codes")
                .map_or(true, |v| v.trim().is_empty())
                && !colnect.catalog_codes.is_empty()
            {
                let codes_str = colnect
                    .catalog_codes
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<_>>()
                    .join(" | ");
                attrs.set("colnect_catalog_codes", &codes_str);
            }

            if attrs
                .get("catalog_number")
                .map_or(true, |v| v.trim().is_empty())
            {
                let raw_codes = colnect
                    .catalog_codes
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<_>>()
                    .join(" | ");
                if let Some((catalog_type, catalog_number)) =
                    crate::catalog::preferred_catalog_entry(&raw_codes, &default_catalog_code)
                {
                    attrs.set(
                        "catalog_number",
                        &crate::catalog::normalize_catalog_number(&catalog_number),
                    );
                    attrs.set("catalog_code", &catalog_type);
                    if catalog_type == "sn"
                        && attrs
                            .get("scott_number")
                            .map_or(true, |v| v.trim().is_empty())
                    {
                        attrs.set(
                            "scott_number",
                            &crate::catalog::normalize_catalog_number(&catalog_number),
                        );
                    }
                }
            }

            // Keep the output heading aligned with an explicitly chosen
            // catalog family from the Review UI. If the family was saved on the
            // stamp row, prefer the matching number from Colnect codes; if the
            // family is missing, infer it from the currently displayed number
            // before falling back to the album default.
            let explicit_catalog_code = attrs
                .get("catalog_code")
                .map(|v| crate::catalog::normalize_catalog_type(v))
                .filter(|v| !v.is_empty());
            let raw_catalog_codes = attrs
                .get("colnect_catalog_codes")
                .cloned()
                .unwrap_or_default();
            let current_catalog_number = attrs.get("catalog_number").cloned().unwrap_or_default();

            if !raw_catalog_codes.trim().is_empty() {
                if let Some(explicit_code) = explicit_catalog_code.as_deref() {
                    if let Some((_, preferred_number)) =
                        crate::catalog::preferred_catalog_entry(&raw_catalog_codes, explicit_code)
                    {
                        attrs.set(
                            "catalog_number",
                            &crate::catalog::normalize_catalog_number(&preferred_number),
                        );
                        attrs.set("catalog_code", explicit_code);
                    }
                } else if !current_catalog_number.trim().is_empty() {
                    let inferred_catalog_code = crate::catalog::infer_catalog_type_for_number(
                        &raw_catalog_codes,
                        &current_catalog_number,
                        &default_catalog_code,
                    );
                    if !inferred_catalog_code.trim().is_empty() {
                        attrs.set("catalog_code", &inferred_catalog_code);
                    }
                }
            }

            if attrs
                .get("scott_number")
                .map_or(true, |v| v.trim().is_empty())
            {
                if let Some((_, scott_code)) = colnect
                    .catalog_codes
                    .iter()
                    .find(|(catalog_type, _)| catalog_type == "Sn")
                {
                    attrs.set("scott_number", scott_code);
                }
            }
        }

        if attrs
            .get("catalog_number")
            .map_or(true, |v| v.trim().is_empty())
        {
            if let Some(sg_number) = attrs.get("sg_number").cloned() {
                attrs.set("catalog_number", &sg_number);
            }
        }

        if attrs
            .get("catalog_code")
            .map_or(true, |v| v.trim().is_empty())
        {
            attrs.set("catalog_code", &default_catalog_code);
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
    fit_box: bool,
    target_stamp_height_mm: f64,
    reference_width_mm: f64,
    reference_height_mm: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    // Compute series averages from available images
    let mut series_widths_mm: Vec<f64> = Vec::new();
    let mut series_heights_mm: Vec<f64> = Vec::new();
    for stamp in stamps.iter() {
        let image_path = stamp.image_path.trim();
        let full_image_path = crate::file_utils::resolve_album_image_path(album_name, image_path)
            .to_string_lossy()
            .to_string();
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
    // (No per-source adjustments; extra_width_mm/extra_height_mm are applied at emission)

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
            // Use Colnect real-world dimensions directly (no per-source adjustments)
            s.est_width_mm = Some(w_mm);
            s.est_height_mm = Some(h_mm);
        } else {
            // Fall back to existing logic for image-based or default dimensions
            let image_path = s.image_path.trim();
            let full_image_path =
                crate::file_utils::resolve_album_image_path(album_name, image_path)
                    .to_string_lossy()
                    .to_string();
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
                if fit_box && h > target_stamp_height_mm {
                    let sc = target_stamp_height_mm / h;
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
                .get("catalog_number")
                .or_else(|| s.attrs.get("scott_number"))
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
