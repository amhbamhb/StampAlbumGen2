use super::helpers::build_page_text_template;
use super::helpers::emit_row_start_es;
use crate::config::Config;
use std::io::Write;

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
pub fn new_year<W: std::io::Write>(writer: &mut W, year: &str, config_opt: Option<&Config>) {
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
