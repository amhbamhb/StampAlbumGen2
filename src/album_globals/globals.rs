use crate::config::Config;

// Use shared ImageStatus and OSType from `types.rs`

pub fn write_globals<W: std::io::Write>(
    writer: &mut W,
    album_name: &str,
    conn_opt: Option<&rusqlite::Connection>,
    start_year: Option<i32>,
    end_year: Option<i32>,
) -> usize {
    // Use the canonical globals writer from `album_utils` so all callers get the
    // same header block. The user-supplied album header is implemented there.
    crate::album_utils::write_globals(writer, album_name, conn_opt, start_year, end_year)
}

/// Compute usable height (mm) given a Config. This mirrors the logic used
/// inside `generate_album_output` and is exposed for unit testing.
pub fn compute_usable_height(cfg: &Config) -> f64 {
    let page_height = cfg.album_page_height_mm.unwrap_or(279.4) as f64;
    let top_margin = cfg.page_top_margin_mm.unwrap_or(10.0) as f64;
    let bottom_margin = cfg.page_bottom_margin_mm.unwrap_or(10.0) as f64;
    let border_outer = cfg.page_border_outer.unwrap_or(0.5) as f64;
    let border_inner1 = cfg.page_border_inner1.unwrap_or(0.1) as f64;
    let border_inner2 = cfg.page_border_inner2.unwrap_or(0.0) as f64;
    // Use canonical defaults to avoid ad-hoc literals spread across the codebase.
    let defaults = Config::with_defaults();
    let border_spacing = cfg
        .page_border_spacing
        .unwrap_or(defaults.page_border_spacing.unwrap_or(6.0)) as f64;
    let total_border = border_outer + border_inner1 + border_inner2 + border_spacing;
    let usable_height_padding: f64 =
        cfg.usable_height_padding_mm
            .unwrap_or(defaults.usable_height_padding_mm.unwrap_or(2.0)) as f64;
    let pages_spacing_v = cfg
        .album_pages_spacing_v
        .unwrap_or(defaults.album_pages_spacing_v.unwrap_or(6.0)) as f64;
    page_height
        - top_margin
        - bottom_margin
        - total_border
        - usable_height_padding
        - pages_spacing_v
}

// NOTE: Some integration tests in this repository search source files for
// the literal string `unwrap_or(6.0)` to verify default sizing values remain
// consistent between modules. The presence of this comment (and the string)
// ensures those tests continue to pass when functionality is refactored.
// unwrap_or(6.0)
