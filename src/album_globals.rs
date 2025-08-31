// AlbumEasy output and layout logic
use log::{debug, info, error, warn};
use std::io::{Write, BufWriter};
use std::fs::File;
use std::path::Path;
use crate::db_utils::{init_db, get_all_series, get_stamps_by_series};
use crate::image_utils::{calc_mm_per_pixel, estimate_real_dimensions};
use crate::config::{load_config, Config};

// Define or import Stamp and GenericAttrs
#[derive(Debug, Clone, PartialEq)]
pub struct GenericAttrs {
    attrs: std::collections::HashMap<String, String>,
}
impl GenericAttrs {
    pub fn new() -> Self {
        Self { attrs: std::collections::HashMap::new() }
    }
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
// ...existing use statements...
// ...existing code for helpers, structs, and main function...
/// Writes a new page header and resets counters
pub fn new_page<W: std::io::Write>(writer: &mut W, rows_written: &mut i32, stamps_written: &mut i32, used_width: &mut i32, used_height: &mut i32, max_row_height: &mut i32) {
    let _ = writeln!(writer, "################################################################################");
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
pub fn new_row<W: std::io::Write>(writer: &mut W, year: &str, rows_written: &mut i32, stamps_written: &mut i32, used_width: &mut i32, used_height: &mut i32, max_row_height: &mut i32) {
    *used_height += *max_row_height;
    if *used_height + 43 > 245 {
        new_page(writer, rows_written, stamps_written, used_width, used_height, max_row_height);
        new_year(writer, year);
    }
    let _ = writeln!(writer, "ROW_START_FS (TN 6 0.1 5.0)");
    *rows_written += 1;
    *stamps_written = 0;
    *used_width = 0;
    *used_height += *max_row_height;
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
    let _ = writeln!(writer, "STAMP {} {} {} {} {} {} {} {} {} {} {}", year, field1, field2, field3, color, multi_perf, desc, width, height, catalog_num, field4);
}

// AlbumEasy Output Generation
// ==========================
// This section contains the main entry point and helpers for generating AlbumEasy output.

// Removed duplicate imports

/// Main entry point for AlbumEasy output generation.
/// Queries the database, formats output, and writes to a .txt file for AlbumEasy import.
pub fn generate_album_output<F>(db_path: &str, album_name: &str, mut page_callback: Option<F>)
where
    F: FnMut(usize),{
    // Load config values from config.toml
    let config = load_config("config.toml");
    let stamp_box_height = config.stamp_box_height.unwrap_or(24.0) as f64;
    let row_font_spacing = config.row_font_spacing.unwrap_or(5.0) as f64;
    let header_font_spacing = config.header_font_spacing.unwrap_or(3.0) as f64;
    let reference_width_mm = config.reference_width_mm.unwrap_or(200.0) as f64;
    let reference_height_mm = config.reference_height_mm.unwrap_or(240.0) as f64;
    // Reference image path and measured size (dynamic by country)
    let country_dir = format!("Albums/{}/images", album_name.replace(' ', "-"));
    let reference_image_path = format!("{}/BG1-s.jpg", country_dir);
    let (mm_per_pixel_w, mm_per_pixel_h) = match calc_mm_per_pixel(&reference_image_path, reference_width_mm as f32, reference_height_mm as f32) {
        Some(ratio) => ratio,
        None => {
            warn!("Reference image not found: {}. Using default ratio (1.0, 1.0)", reference_image_path);
            (1.0, 1.0)
        }
    };
    use std::path::Path;
    let conn = init_db(db_path).expect("Failed to open database");
    let db_dir = Path::new(db_path).parent().unwrap_or_else(|| Path::new("."));
    let db_file_stem = Path::new(db_path).file_stem().unwrap_or_default().to_string_lossy();
    let output_path = db_dir.join(format!("{}.txt", db_file_stem));
    let mut output = BufWriter::new(File::create(output_path).expect("Failed to create output file"));

    // Write global album settings
    write_globals(&mut output, album_name);

    // Page size and margins (from AlbumEasy settings)
    let page_height = 279.4; // US Letter height in mm
    let top_margin = 10.0;
    let bottom_margin = 10.0;
    // Border settings (from config or hardcoded)
    let border_outer = 0.5; // ALBUM_PAGES_BORDER outer line thickness
    let border_inner1 = 0.1; // ALBUM_PAGES_BORDER inner1 line thickness
    let border_inner2 = 0.0; // ALBUM_PAGES_BORDER inner2 line thickness
    let border_spacing = 3.0; // ALBUM_PAGES_BORDER spacing between lines
    // Total border thickness: outer + inner1 + inner2 + spacing
    let total_border = border_outer + border_inner1 + border_inner2 + border_spacing;
    let usable_height = (page_height - top_margin - bottom_margin - total_border) as f64;
    // debug!("[PAGE LAYOUT] page_height={:.2}, top_margin={:.2}, bottom_margin={:.2}, total_border={:.2}, usable_height={:.2}",
    //     page_height, top_margin, bottom_margin, total_border, usable_height);
    // Font and spacing from AlbumEasy settings
    let header_font_spacing = 3.0; // PAGE_TEXT_CENTRE (TN 10 ... 3.0)
    let row_font_spacing = 5.0;    // ROW_START_FS (TN 6 0.1 5.0)
    let stamp_box_height = 24.0;   // Estimate: typical stamp box height
    let header_lines = 4;          // Year, name, desc, perforation
    // let header_height = (header_lines as f64) * header_font_spacing;
    // let row_height = stamp_box_height + row_font_spacing;

    let series_list = get_all_series(&conn).expect("Failed to get series");
    let mut page_counter = 1;
    let mut current_height = 0.0;
    let mut first_page = true;
    let mut rows_on_page = 0;
    // Always start the first page before any content
    if let Some(cb) = page_callback.as_mut() {
        cb(page_counter);
    }
    let _ = writeln!(output, "### Page {} ###", page_counter);
    let _ = writeln!(output, "PAGE_START\n################################################################################");
    page_counter += 1;
    first_page = false;
    let series_count = series_list.len();
    for (series_idx, series) in series_list.iter().enumerate() {
            if let Some(cb) = page_callback.as_mut() {
        cb(page_counter);
    }
        // Example: Load stamps as Vec<Stamp> with generic attributes
        let stamps = get_stamps_by_series(&conn, &series.name, &series.year.to_string()).expect("Failed to get stamps");
        let mut stamp_objs: Vec<Stamp> = Vec::new();
        for stamp in stamps {
            let mut attrs = GenericAttrs::new();
            attrs.set("year", &stamp.year.to_string());
            attrs.set("field1", &stamp.field1);
            attrs.set("field2", &stamp.field2);
            attrs.set("field3", &stamp.field3);
            attrs.set("color", &stamp.color);
            attrs.set("desc", &stamp.desc);
            attrs.set("catalog_num", &stamp.catalog_num);
            attrs.set("field4", &stamp.field4);
            attrs.set("watermark", &stamp.watermark);
            attrs.set("design", &stamp.design);
            // Add more fields as needed for other countries
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
            let full_image_path = format!("Albums/{}/{}", album_name.replace(' ', "-"), image_path);
            let (raw_width_mm, _) = estimate_real_dimensions(&full_image_path, mm_per_pixel_w, mm_per_pixel_h)
                .unwrap_or((reference_width_mm as f32, reference_height_mm as f32));
            let est_width_mm = raw_width_mm as f64 / 10.0;
            let margin = 2.0f64; // margin between stamps (mm)
            let page_width = 216.0f64; // US Letter width in mm
            let left_margin = 18.0f64;
            let right_margin = 18.0f64;
            let usable_width = page_width - left_margin - right_margin - (border_outer + border_inner1 + border_inner2 + border_spacing);
            // Accumulate row width as stamps are added
            let mut row_width: f64 = row.iter().map(|s| {
                let img_path = s.image_path.trim();
                let full_img_path = format!("Albums/{}/{}", album_name.replace(' ', "-"), img_path);
                let (w_mm, _) = estimate_real_dimensions(&full_img_path, mm_per_pixel_w, mm_per_pixel_h)
                    .unwrap_or((reference_width_mm as f32, reference_height_mm as f32));
                w_mm as f64 / 10.0 + margin
            }).sum();
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
        let mut row_idx = 0;
        while row_idx < stamp_rows.len() {
            let need_header = !header_written;
            if need_header {
                // Collect all header lines to print
                let mut header_lines_vec = Vec::new();
                let name_line = clean_text(series.name.trim()).replace("\n", " ").replace("  ", " ");
                if !name_line.is_empty() {
                    header_lines_vec.push((format!("PAGE_TEXT_CENTRE (TN 8 \"{}\" 1.0)", name_line), 1.0));
                }
                let perf_trimmed = clean_text(series.perforation.trim()).replace("\n", " ").replace("  ", " ");
                if !perf_trimmed.is_empty() {
                    header_lines_vec.push((format!("PAGE_TEXT_CENTRE (TN 8 \"Perforation {}\" 1.0)", perf_trimmed), 1.0));
                }
                let first_stamp = stamp_rows.get(0).and_then(|row| row.get(0));
                if let Some(stamp) = first_stamp {
                    let watermark = clean_text(stamp.attrs.get("watermark").unwrap_or(&"".to_string()).trim()).replace("\n", " ").replace("  ", " ");
                    if !watermark.is_empty() {
                        header_lines_vec.push((format!("PAGE_TEXT_CENTRE (TN 8 \"Watermark: {}\" 1.0)", watermark), 1.0));
                    }
                    let design_field = stamp.attrs.get("design").unwrap_or(&"".to_string()).replace("\n", " ").replace("  ", " ");
                    let mut design = String::new();
                    let mut engraving = String::new();
                    let mut parts = design_field.split("Engraving:");
                    design = parts.next().unwrap_or("").split("Perforation:").next().unwrap_or("").trim().to_string();
                    if let Some(engraving_part) = parts.next() {
                        engraving = engraving_part.split("Perforation:").next().unwrap_or("").trim().to_string();
                    }
                    design = clean_text(&design);
                    engraving = clean_text(&engraving);
                    if !engraving.is_empty() {
                        header_lines_vec.push((format!("PAGE_TEXT_CENTRE (TN 8 \"Engraving: {}\" 1.0)", engraving), 1.0));
                    }
                    if !design.is_empty() {
                        header_lines_vec.push((format!("PAGE_TEXT_CENTRE (TN 8 \"Design: {}\" 1.0)", design), 1.0));
                    }
                }
                let header_block_height: f64 = header_lines_vec.iter().map(|(_, h)| *h).sum();
                let vspace_before_header = 8.0;
                let vspace_after_header = 12.5;
                // Calculate height of first row
                let stamps_in_row = &stamp_rows[row_idx];
                let mut max_row_height = 0.0f64;
                let font_height = 3.0f64;
                let image_spacing = 2.0f64;
                for stamp in stamps_in_row {
                    let image_path = stamp.image_path.trim();
                    let full_image_path = format!("Albums/{}/{}", album_name.replace(' ', "-"), image_path);
                    let (_, est_height_mm) = estimate_real_dimensions(&full_image_path, mm_per_pixel_w, mm_per_pixel_h)
                        .unwrap_or((reference_width_mm as f32, reference_height_mm as f32));
                    let est_height_mm = est_height_mm as f64 / 10.0;
                    let below_stamp_1 = clean_text(stamp.attrs.get("catalog_num").unwrap_or(&"".to_string()));
                    let below_stamp_2 = clean_text(stamp.attrs.get("field1").unwrap_or(&"".to_string()));
                    let below_stamp_3 = clean_text(stamp.attrs.get("field2").unwrap_or(&"".to_string()));
                    let text_lines = below_stamp_1.matches('\n').count() + 1
                        + below_stamp_2.matches('\n').count() + 1
                        + below_stamp_3.matches('\n').count() + 1;
                    let total_height = est_height_mm + image_spacing + (text_lines as f64) * font_height;
                    if total_height > max_row_height {
                        max_row_height = total_height;
                    }
                }
                let total_header_and_row_height = vspace_before_header + header_block_height + vspace_after_header + max_row_height + row_font_spacing as f64;
                let conservative_buffer = 2.0; // mm buffer to avoid edge cases
                let conservative_needed = total_header_and_row_height + conservative_buffer;
                // debug!("[PAGE FIT CHECK] Header block height: {:.2}, vspace before: {:.2}, vspace after: {:.2}, first row height: {:.2}, total needed: {:.2}, buffer: {:.2}, current: {:.2}, usable: {:.2}, will_fit: {}",
                //     header_block_height, vspace_before_header, vspace_after_header, max_row_height, total_header_and_row_height, conservative_buffer, current_height, usable_height, (current_height + conservative_needed <= usable_height));
                // Peek ahead: if this is a single-row series and next series is also single-row and both fit, combine
                let mut force_new_page = current_height + conservative_needed > usable_height || rows_on_page > 0;
                if !force_new_page && stamp_rows.len() == 1 {
                    // Try to peek at next series
                    // Only if we're at the last row of this series
                    // (This logic is only for the first row, so row_idx == 0)
                    if row_idx == 0 {
                        // Use index to get next series
                        if series_idx + 1 < series_count {
                            let next_series = &series_list[series_idx + 1];
                            let next_stamps = get_stamps_by_series(&conn, &next_series.name, &next_series.year.to_string()).ok();
                            if let Some(next_stamps) = next_stamps {
                                let mut next_stamp_objs: Vec<Stamp> = Vec::new();
                                for stamp in next_stamps {
                                    let mut attrs = GenericAttrs::new();
                                    attrs.set("year", &stamp.year.to_string());
                                    attrs.set("field1", &stamp.field1);
                                    attrs.set("field2", &stamp.field2);
                                    attrs.set("field3", &stamp.field3);
                                    attrs.set("color", &stamp.color);
                                    attrs.set("desc", &stamp.desc);
                                    attrs.set("catalog_num", &stamp.catalog_num);
                                    attrs.set("field4", &stamp.field4);
                                    attrs.set("watermark", &stamp.watermark);
                                    attrs.set("design", &stamp.design);
                                    next_stamp_objs.push(Stamp {
                                        attrs,
                                        image_path: stamp.image_path.clone(),
                                    });
                                }
                                // Group next series stamps into rows of 5
                                let mut next_stamp_rows: Vec<Vec<Stamp>> = Vec::new();
                                let mut next_row: Vec<Stamp> = Vec::new();
                                for stamp in next_stamp_objs {
                                    next_row.push(stamp);
                                    if next_row.len() == 4 {
                                        next_stamp_rows.push(next_row);
                                        next_row = Vec::new();
                                    }
                                }
                                if !next_row.is_empty() {
                                    next_stamp_rows.push(next_row);
                                }
                                // If next series is also single-row
                                if next_stamp_rows.len() == 1 {
                                    // Calculate next header+row height
                                    let mut next_header_lines_vec = Vec::new();
                                    let next_name_line = clean_text(next_series.name.trim()).replace("\n", " ").replace("  ", " ");
                                    if !next_name_line.is_empty() {
                                        next_header_lines_vec.push((format!("PAGE_TEXT_CENTRE (TN 8 \"{}\" 1.0)", next_name_line), 1.0));
                                    }
                                    let next_perf_trimmed = clean_text(next_series.perforation.trim()).replace("\n", " ").replace("  ", " ");
                                    if !next_perf_trimmed.is_empty() {
                                        next_header_lines_vec.push((format!("PAGE_TEXT_CENTRE (TN 8 \"Perforation {}\" 1.0)", next_perf_trimmed), 1.0));
                                    }
                                    let next_first_stamp = next_stamp_rows.get(0).and_then(|row| row.get(0));
                                    if let Some(stamp) = next_first_stamp {
                                        let watermark = clean_text(stamp.attrs.get("watermark").unwrap_or(&"".to_string()).trim()).replace("\n", " ").replace("  ", " ");
                                        if !watermark.is_empty() {
                                            next_header_lines_vec.push((format!("PAGE_TEXT_CENTRE (TN 8 \"Watermark: {}\" 1.0)", watermark), 1.0));
                                        }
                                        let design_field = stamp.attrs.get("design").unwrap_or(&"".to_string()).replace("\n", " ").replace("  ", " ");
                                        let mut design = String::new();
                                        let mut engraving = String::new();
                                        let mut parts = design_field.split("Engraving:");
                                        design = parts.next().unwrap_or("").split("Perforation:").next().unwrap_or("").trim().to_string();
                                        if let Some(engraving_part) = parts.next() {
                                            engraving = engraving_part.split("Perforation:").next().unwrap_or("").trim().to_string();
                                        }
                                        design = clean_text(&design);
                                        engraving = clean_text(&engraving);
                                        if !engraving.is_empty() {
                                            next_header_lines_vec.push((format!("PAGE_TEXT_CENTRE (TN 8 \"Engraving: {}\" 1.0)", engraving), 1.0));
                                        }
                                        if !design.is_empty() {
                                            next_header_lines_vec.push((format!("PAGE_TEXT_CENTRE (TN 8 \"Design: {}\" 1.0)", design), 1.0));
                                        }
                                    }
                                    let next_header_block_height: f64 = next_header_lines_vec.iter().map(|(_, h)| *h).sum();
                                    let next_vspace_before_header = 8.0;
                                    let next_vspace_after_header = 12.5;
                                    // Calculate height of next row
                                    let next_stamps_in_row = &next_stamp_rows[0];
                                    let mut next_max_row_height = 0.0f64;
                                    let font_height = 3.0f64;
                                    let image_spacing = 2.0f64;
                                    for stamp in next_stamps_in_row {
                                        let image_path = stamp.image_path.trim();
                                        let full_image_path = format!("Albums/{}/{}", album_name.replace(' ', "-"), image_path);
                                        let (_, est_height_mm) = estimate_real_dimensions(&full_image_path, mm_per_pixel_w, mm_per_pixel_h)
                                            .unwrap_or((reference_width_mm as f32, reference_height_mm as f32));
                                        let est_height_mm = est_height_mm as f64 / 10.0;
                                        let below_stamp_1 = clean_text(stamp.attrs.get("catalog_num").unwrap_or(&"".to_string()));
                                        let below_stamp_2 = clean_text(stamp.attrs.get("field1").unwrap_or(&"".to_string()));
                                        let below_stamp_3 = clean_text(stamp.attrs.get("field2").unwrap_or(&"".to_string()));
                                        let text_lines = below_stamp_1.matches('\n').count() + 1
                                            + below_stamp_2.matches('\n').count() + 1
                                            + below_stamp_3.matches('\n').count() + 1;
                                        let total_height = est_height_mm + image_spacing + (text_lines as f64) * font_height;
                                        if total_height > next_max_row_height {
                                            next_max_row_height = total_height;
                                        }
                                    }
                                    let next_total_header_and_row_height = next_vspace_before_header + next_header_block_height + next_vspace_after_header + next_max_row_height + row_font_spacing as f64;
                                    let next_conservative_buffer = 2.0;
                                    let next_conservative_needed = next_total_header_and_row_height + next_conservative_buffer;
                                    // If both current and next header+row fit, do not force page break
                                    if current_height + conservative_needed + next_conservative_needed <= usable_height && rows_on_page == 0 {
                                        force_new_page = false;
                                    }
                                }
                            }
                        }
                    }
                }
                if force_new_page {
                    if !first_page {
                        let _ = writeln!(output, "\n################################################################################");
                    }
                    let _ = writeln!(output, "### Page {} ###", page_counter);
                    let _ = writeln!(output, "PAGE_START\n################################################################################");
                    page_counter += 1;
                    current_height = 0.0;
                    first_page = false;
                    rows_on_page = 0;
                    // debug!("Forced page break before header and first row");
                }
                // Output header and first row together
                let _ = writeln!(output, "PAGE_VSPACE (8)");
                current_height += vspace_before_header;
                for (line, h) in header_lines_vec {
                    let _ = writeln!(output, "{}", line);
                    current_height += h;
                }
                let _ = writeln!(output, "PAGE_VSPACE (12.5)");
                current_height += vspace_after_header;
                header_written = true;
            }
            // Calculate row height: image height + spacing + text below stamp
            let stamps_in_row = &stamp_rows[row_idx];
            let mut max_row_height = 0.0f64;
            let font_height = 3.0f64; // TN 10 font spacing (mm)
            let image_spacing = 2.0f64; // space below image (mm)
            for stamp in stamps_in_row {
                let image_path = stamp.image_path.trim();
                let full_image_path = format!("Albums/{}/{}", album_name.replace(' ', "-"), image_path);
                let est_height_mm = if !image_path.is_empty() {
                    if let Some((_, h_mm)) = estimate_real_dimensions(&full_image_path, mm_per_pixel_w, mm_per_pixel_h) {
                        h_mm as f64 / 10.0
                    } else {
                        28.0 // fallback for missing image file
                    }
                } else {
                    10.0 // default for missing image path
                };
                // Compose all text below stamp
                let text_below = [
                    stamp.attrs.get("catalog_num").unwrap_or(&String::new()).clone(),
                    stamp.attrs.get("field1").unwrap_or(&String::new()).clone(),
                    stamp.attrs.get("field2").unwrap_or(&String::new()).clone(),
                ].join("\n");
                let text_lines = text_below.lines().count();
                let text_height = (text_lines as f64) * font_height;
                let total_height = est_height_mm + image_spacing + text_height;
                // debug!("[ROW HEIGHT] Stamp: image_path='{}', est_image_height_mm={:.2}, text_lines={}, text_height_mm={:.2}, total_row_height_mm={:.2}",
                //     image_path, est_height_mm, text_lines, text_height, total_height);
                if total_height > max_row_height {
                    max_row_height = total_height;
                }
            }
            let total_row_height = max_row_height + row_font_spacing as f64;
            // Check if this row fits before adding, and if max rows per page reached
            if current_height + total_row_height > usable_height || rows_on_page >= 4 {
                // Start new page and repeat header
                if !first_page {
                    let _ = writeln!(output, "\n################################################################################");
                }
                let _ = writeln!(output, "### Page {} ###", page_counter);
                let _ = writeln!(output, "PAGE_START\n################################################################################");
                page_counter += 1;
                current_height = 0.0;
                header_written = false;
                rows_on_page = 0;
                continue; // re-check header on next loop
            }
            let _ = writeln!(output, "ROW_START_ES (TN 6 0.1 5.0)");
            for stamp in stamps_in_row {
                let image_path = stamp.image_path.trim();
                let full_image_path = format!("Albums/{}/{}", album_name.replace(' ', "-"), image_path);
                let (raw_width_mm, raw_height_mm) = estimate_real_dimensions(&full_image_path, mm_per_pixel_w, mm_per_pixel_h)
                    .unwrap_or((reference_width_mm as f32, reference_height_mm as f32));
                let est_width_mm = raw_width_mm / 10.0;
                let est_height_mm = raw_height_mm / 10.0;
                // Build multi-line text fields for AlbumEasy using generic attrs
                let sg_number = clean_text(stamp.attrs.get("catalog_num").unwrap_or(&"".to_string()));
                let denomination = clean_text(stamp.attrs.get("field1").unwrap_or(&"".to_string()));
                let color = clean_text(stamp.attrs.get("color").unwrap_or(&"".to_string()));
                let stamps_issued = clean_text(stamp.attrs.get("field2").unwrap_or(&"".to_string()));
                let mint_condition = if stamp.attrs.get("field3").map(|v| v.trim().is_empty()).unwrap_or(true) {
                    "Unused".to_string()
                } else {
                    clean_text(stamp.attrs.get("field3").unwrap_or(&"".to_string()))
                };
                let used = clean_text(stamp.attrs.get("field4").unwrap_or(&"".to_string()));
                let below_stamp_1 = format!("{}", sg_number);
                let below_stamp_2 = format!("{}\n{}\n{}", denomination, color, stamps_issued);
                let below_stamp_3 = format!("{}\n{}", mint_condition, used);
                if !image_path.is_empty() && Path::new(&full_image_path).exists() {
                    let image_path_clean = clean_text(image_path);
                    let line = format!(
                        "STAMP_ADD_IMG ({:.1} {:.1} \"{}\" \"{}\" \"{}\" \"{}\")",
                        est_width_mm,
                        est_height_mm,
                        image_path_clean,
                        below_stamp_1,
                        below_stamp_2,
                        below_stamp_3
                    );
                    let _ = writeln!(output, "{}", line.replace("\r", "").replace("\n", "\\n"));
                } else {
                    let line = format!(
                        "STAMP_ADD ({:.1} {:.1} \"Image\\nNot\\nAvailable\" \"{}\" \"{}\" \"{}\")",
                        est_width_mm,
                        est_height_mm,
                        below_stamp_1,
                        below_stamp_2,
                        below_stamp_3
                    );
                    let _ = writeln!(output, "{}", line.replace("\r", "").replace("\n", "\\n"));
                }
            }
            current_height += total_row_height;
            rows_on_page += 1;
            row_idx += 1;
        }
    }

/// Cleans up text fields for AlbumEasy output
fn clean_text(s: &str) -> String {
    use regex::Regex;
    // Remove 'Â' before fractions/currency
    let re = Regex::new(r"Â([½¼¾⅓⅔⅛⅜⅝⅞£€$¥])").unwrap();
    let mut s = re.replace_all(s, "$1").to_string();
    // Replace stray 'Â' with space
    s = s.replace('Â', " ");
    // Remove known problematic chars
    s = s.replace('â', " ");
    // Replace double quotes
    s = s.replace('"', "'");
    // Replace non-ASCII except allowed symbols with space
    s.chars()
        .map(|c| {
            match c {
                '£' | '€' | '$' | '¥' |
                '½' | '¼' | '¾' | '⅓' | '⅔' | '⅛' | '⅜' | '⅝' | '⅞' => c,
                _ if c.is_ascii() => c,
                _ => ' ',
            }
        })
        .collect()
}

/// Writes global album settings to the provided writer
pub fn write_globals<W: Write>(writer: &mut W, album_name: &str) {
    let lines = [
        "################################################################################",
        &format!("# {}", album_name),
        "################################################################################",
        "",
        "ALBUM_PAGES_SIZE  (216 279) #Set the page to US Letter (8.5 x 11)",
        "#ALBUM_PAGES_SIZE (210.0 297.0) #Setting for A4",
        "",
        "ALBUM_PAGES_MARGINS  (10.0 10.0 10.0 10.0 )",
        "",
        "ALBUM_PAGES_BORDER (0.5 0.1 0.0 3.0)    #two line border",
        "#ALBUM_PAGES_BORDER3  (0.1 0.0 0.0 3.0) #single line border",
        "",
        &format!("ALBUM_PAGES_TITLE (TB 18 \"{}\")", album_name),
        "",
        "ALBUM_PAGES_SPACING (5.0 8.0)",
        "",
        "ALBUM_PAGES_FOOTER_NUMBER (HN 10 C 1 \"Page \" \" of \\$PAGES$\")",
        "",
        "ALBUM_STAMP_IMG_NEW_SETTING (3.0 3.0 true)",
        "",
        "ROW_ALIGN_MIDDLE",
        "",
        "# The stamp boxes in this album are sized approximately",
        "# 4mm larger than the actual stamps giving a 2mm border",
        "# To have the boxes drawn the actual stamp size use",
        "#STAMP_BOXES_SIZE_ADJUST(-4.0)",
        "# To increase the border, e.g by 1mm use STAMP_BOXES_SIZE_ADJUST(2.0)",
        "STAMP_BOXES_SIZE_ADJUST (0)",
    ];
    for line in lines.iter() {
        let _ = writeln!(writer, "{}", line);
    }
}
}

/// Writes global album settings to the provided writer
pub fn write_globals<W: Write>(writer: &mut W, album_name: &str) {
    let lines = [
        "################################################################################",
        &format!("# {}", album_name),
        "################################################################################",
        "",
        "ALBUM_PAGES_SIZE  (215.9 279.4) #Set the page to US Letter (8.5 x 11)",
        "#ALBUM_PAGES_SIZE (210.0 297.0) #Setting for A4",
        "",
        "ALBUM_PAGES_MARGINS  (15.0 10.0 10.0 10.0 )",
        "",
        "ALBUM_PAGES_BORDER (0.5 0.1 0.0 3.0)    #two line border",
        "#ALBUM_PAGES_BORDER3  (0.1 0.0 0.0 3.0) #single line border",
        "",
        &format!("ALBUM_PAGES_TITLE (TB 18 \"{}\")", album_name),
        "",
        "ALBUM_PAGES_SPACING (5.0 3.0)",
        "",
        "#ALBUM_PAGES_NUMBER (TN 10 R)",
        "",
        "ALBUM_STAMP_IMG_NEW_SETTING (6.0 6.0 true)",
        "",
        "ROW_ALIGN_MIDDLE",
        "",
        "# The stamp boxes in this album are sized approximately",
        "# 4mm larger than the actual stamps giving a 2mm border",
        "# To have the boxes drawn the actual stamp size use",
        "#STAMP_BOXES_SIZE_ADJUST(-4.0)",
        "# To increase the border, e.g by 1mm use STAMP_BOXES_SIZE_ADJUST(2.0)",
        "STAMP_BOXES_SIZE_ADJUST (0)",
    ];
    for line in lines.iter() {
        let _ = writeln!(writer, "{}", line);
    }
}

/// Writes a new series header, handles page breaks, and updates counters
pub fn new_series<W: Write>(
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
    month_names: &[&str],
) {
    // Page break logic
    if *rows_written == 4 {
        new_page(writer, rows_written, stamps_written, used_width, used_height, max_row_height);
        new_year(writer, year);
    } else if *used_height + *max_row_height + 43 > 245 {
        new_page(writer, rows_written, stamps_written, used_width, used_height, max_row_height);
        new_year(writer, year);
    } else {
        let rows_needed = (stamps_in_series + 3) / 4;
        if *rows_written + rows_needed > 4 {
            new_page(writer, rows_written, stamps_written, used_width, used_height, max_row_height);
            new_year(writer, year);
        }
    }

    // Escape quotes in series name
    let mut series_display = series_name.to_string();
    if series_display.contains('"') {
        series_display = series_display.replace("\"", "\\\"");
    }

    // Remove leading zero from day
    let mut day_display = day.to_string();
    if day_display.starts_with('0') {
        day_display = day_display.trim_start_matches('0').to_string();
    }

    // Build date string
    let mut date_display = String::new();
    if year != "0000" && month != "00" && day != "00" {
        if let Ok(month_idx) = month.parse::<usize>() {
            if month_idx < month_names.len() {
                date_display = format!("{} {}, {}", month_names[month_idx], day_display, year);
            }
        }
    } else if year != "0000" && month != "00" && day == "00" {
        if let Ok(month_idx) = month.parse::<usize>() {
            if month_idx < month_names.len() {
                date_display = format!("{}, {}", month_names[month_idx], year);
            }
        }
    } else if year != "0000" && month == "00" && day == "00" {
        date_display = year.to_string();
    }

    // Write date if present
    if !date_display.is_empty() {
        for line in date_display.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                let _ = writeln!(writer, "PAGE_TEXT_CENTRE (TN 8 \"{}\" 1.0)", trimmed);
            }
        }
    }

    // Write series name
    for line in series_display.lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            let _ = writeln!(writer, "PAGE_TEXT_CENTRE (TN 8 \"{}\" 1.0)", trimmed);
        }
    }

    // Write perforation info
    if !is_perforated && !perforation.is_empty() {
    let _ = writeln!(writer, "PAGE_TEXT_CENTRE (TN 8 \"Perforation {}\" 2.0)", perforation);
    } else {
    let _ = writeln!(writer, "PAGE_TEXT_CENTRE (TN 8 \"\" 1.5)");
    }

    // Start new row
    new_row(writer, year, rows_written, stamps_written, used_width, used_height, max_row_height);

    *used_height += 13;
    *max_row_height = 0;
}

/// Writes a line to the output, with error handling
pub fn write_line<W: Write>(writer: &mut W, line: &str) {
    if let Err(e) = writeln!(writer, "{}", line) {
        log::error!("Error writing output file: {}", e);
        std::process::exit(1);
    }
}

// Album global state and constants
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref FILE_SEPARATOR: String = std::path::MAIN_SEPARATOR.to_string();
    pub static ref DATA_SOURCE: Mutex<String> = Mutex::new(String::new());
    pub static ref FILE_SOURCE: Mutex<String> = Mutex::new(String::new());
    pub static ref ALBUM_DIRECTORY: Mutex<String> = Mutex::new(String::new());
    pub static ref IMAGES_DIRECTORY: Mutex<String> = Mutex::new(String::new());
    pub static ref ALBUM_NAME: Mutex<String> = Mutex::new(String::new());
    pub static ref ALBUM_FILE_NAME: Mutex<String> = Mutex::new(String::new());
    pub static ref EXTRACT_FILE_NAME: Mutex<String> = Mutex::new(String::new());
    pub static ref EXPORT_FILE_NAME: Mutex<String> = Mutex::new(String::new());
    pub static ref SITE_NAME: Mutex<String> = Mutex::new(String::new());
    pub static ref CATALOG_NAME: Mutex<String> = Mutex::new(String::new());
    pub static ref NO_IMAGES: Mutex<bool> = Mutex::new(false);
    pub static ref FIX_SIZES: Mutex<bool> = Mutex::new(false);
    pub static ref NO_SSL_CHECK: Mutex<bool> = Mutex::new(false);
    pub static ref SEPARATOR: &'static str = "@";
    pub static ref ROWS_WRITTEN: Mutex<i32> = Mutex::new(0);
    pub static ref STAMPS_WRITTEN: Mutex<i32> = Mutex::new(0);
    pub static ref USED_WIDTH: Mutex<i32> = Mutex::new(0);
    pub static ref USED_HEIGHT: Mutex<i32> = Mutex::new(0);
    pub static ref MAX_ROW_HEIGHT: Mutex<i32> = Mutex::new(0);
    pub static ref SEQUENCER: Mutex<i32> = Mutex::new(0);
    pub static ref START_YEAR: Mutex<i32> = Mutex::new(0);
    pub static ref END_YEAR: Mutex<i32> = Mutex::new(9999);
    pub static ref FORMATS: Mutex<Vec<String>> = Mutex::new(Vec::new());
    pub static ref EMISSIONS: Mutex<Vec<String>> = Mutex::new(Vec::new());
    pub static ref CURRENCIES: Mutex<Vec<String>> = Mutex::new(Vec::new());
    pub static ref MONTH_NAMES: [&'static str; 13] = ["?????", "January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    pub static ref VALID_FORMATS: [&'static str; 12] = ["Booklet", "Booklet Pane", "Full Pane", "Gutter Pairs", "Mini Sheet", "Se-tenant", "Souvenir Sheet", "Stamp", "Stamp with Attached Label", "Stamp with Collectible Margin", "Tete-beche", "Unknown"];
    pub static ref VALID_EMISSIONS: [&'static str; 44] = [
    "Agency Issue", "Air Post", "Air Post Official", "Air Post Semi-Postal", "ATM Labels", "Cinderella", "Commemorative", "Cryptostamp", "Definitive", "Erinnophilia", "Forgery", "Franchise Stamps", "Hunting Permit", "Illegal", "Insured Letter", "Military", "Newspaper", "Official", "Official Reprint", "Other", "Parcel Post", "Personal Delivery", "Personalized - Official", "Personalized - Private", "Postage Due", "Postal Fiscal", "Postal Tax", "Precancelled", "Private", "Proof", "Regional", "Registration", "Revenue", "Savings Stamps", "Semi-Postal", "Special Delivery", "Specimen", "Telegraph & Telephone", "Used Abroad", "War Tax", "Charity", "Express", "Local", "Occupation"
    ];
}

/// Struct representing a year, month, and day
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YearMonthDay {
    pub year: String,
    pub month: String,
    pub day: String,
}

impl YearMonthDay {
    pub fn new(year: &str, month: &str, day: &str) -> Self {
        Self {
            year: year.to_string(),
            month: month.to_string(),
            day: day.to_string(),
        }
    }
}

/// Operating system type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OSType {
    Windows,
    MacOS,
    Linux,
    Other,
    Unknown,
}

/// Struct representing series information
#[derive(Debug, Clone, PartialEq)]
pub struct SeriesInfo {
    pub count: i32,
    pub multi_perf: bool,
    pub width: String,
    pub height: String,
}

impl SeriesInfo {
    pub fn new(count: i32, multi_perf: bool, width: &str, height: &str) -> Self {
        Self {
            count,
            multi_perf,
            width: width.to_string(),
            height: height.to_string(),
        }
    }
}

/// Struct representing image status
#[derive(Debug, Clone, PartialEq)]
pub struct ImageStatus {
    pub link: String,
    pub exists: bool,
    pub width: i32,
    pub height: i32,
}

impl ImageStatus {
    pub fn new(link: &str, exists: bool, width: i32, height: i32) -> Self {
        Self {
            link: link.to_string(),
            exists,
            width,
            height,
        }
    }
}

/// Struct representing miscellaneous attributes
#[derive(Debug, Clone, PartialEq)]
pub struct MiscAttrs {
    pub width: String,
    pub height: String,
    pub face_value: String,
    pub currency: String,
    pub month: String,
    pub day: String,
}

impl MiscAttrs {
    pub fn new(width: &str, height: &str, face_value: &str, currency: &str, month: &str, day: &str) -> Self {
        Self {
            width: width.to_string(),
            height: height.to_string(),
            face_value: face_value.to_string(),
            currency: currency.to_string(),
            month: month.to_string(),
            day: day.to_string(),
        }
    }
}
