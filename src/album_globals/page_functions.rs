use super::generator_new::layout;
use super::helpers::build_page_text_template;
use crate::config::Config;

/// Series-level context for page emission helpers.
pub struct SeriesContext<'a> {
    pub series_name: &'a str,
    pub year: &'a str,
    pub month: &'a str,
    pub day: &'a str,
    pub stamps_in_series: i32,
    pub is_perforated: bool,
    pub perforation: &'a str,
}

pub fn new_series<W: std::io::Write>(
    writer: &mut W,
    ctx: &SeriesContext<'_>,
    rows_written: &mut i32,
    stamps_written: &mut i32,
    used_width: &mut i32,
    used_height: &mut i32,
    max_row_height: &mut i32,
) {
    let _ = writeln!(
        writer,
        "SERIES {} {} {} {} {} {} {}",
        ctx.series_name,
        ctx.year,
        ctx.month,
        ctx.day,
        ctx.stamps_in_series,
        ctx.is_perforated,
        ctx.perforation
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
    let _ = layout::write_page_start(writer);
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
    // Load configuration and compute the real usable height so page breaks
    // reflect current settings (margins, borders, padding, etc.). Fall back
    // to conservative defaults when config keys are missing.
    let cfg = crate::config::load_effective_config("config.toml");
    let usable_height = crate::album_globals::globals::compute_usable_height(&cfg);
    // Conservative buffer (mm) reserved at bottom of page to avoid footer bleed
    let conservative_buffer = cfg.page_fit_conservative_buffer_mm.unwrap_or(2.0) as f64;

    let used_h = (*used_height + *max_row_height) as f64;
    if used_h + conservative_buffer > usable_height {
        new_page(
            writer,
            rows_written,
            stamps_written,
            used_width,
            used_height,
            max_row_height,
        );
        // Emit the year header using the loaded config so templates/spacing are accurate
        new_year(writer, year, Some(&cfg));
    }

    // Write ROW_START_ES using the loaded config so the emitted vspace and
    // spacing match the current configuration.
    let _ = crate::album_globals::generator_new::layout::emit_row_start_es(writer, &cfg);

    *rows_written += 1;
    *stamps_written = 0;
    *used_width = 0;
    // Do NOT add max_row_height again here; it was already added above
    *max_row_height = 0;
}

/// Writes a stamp entry (stub implementation)
/// Stamp-level context for a single emitted stamp row.
pub struct StampContext<'a> {
    pub year: &'a str,
    pub field1: &'a str,
    pub field2: &'a str,
    pub field3: &'a str,
    pub color: &'a str,
    pub multi_perf: bool,
    pub desc: &'a str,
    pub width: &'a str,
    pub height: &'a str,
    pub catalog_num: &'a str,
    pub field4: &'a str,
}

pub fn add_stamp<W: std::io::Write>(writer: &mut W, ctx: &StampContext<'_>) {
    let _ = writeln!(
        writer,
        "STAMP {} {} {} {} {} {} {} {} {} {} {}",
        ctx.year,
        ctx.field1,
        ctx.field2,
        ctx.field3,
        ctx.color,
        ctx.multi_perf,
        ctx.desc,
        ctx.width,
        ctx.height,
        ctx.catalog_num,
        ctx.field4
    );
}
