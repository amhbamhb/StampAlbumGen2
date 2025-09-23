//! Album Globals Module

use std::collections::BTreeMap;
use std::io::{self, Write};
use std::sync::Mutex;
use lazy_static::lazy_static;

// Global state for album generation
lazy_static! {
    pub static ref START_YEAR: Mutex<i32> = Mutex::new(0);
    pub static ref END_YEAR: Mutex<i32> = Mutex::new(9999);
    pub static ref ALBUM_FILE_NAME: Mutex<String> = Mutex::new(String::new());
    pub static ref ALBUM_DB_PATH: Mutex<String> = Mutex::new(String::new());
}

use crate::types::PageHeaderInfo;

/// Page layout calculator for AlbumEasy format
#[derive(Debug, Clone)]
struct PageLayout {
    page_width: f64,
    page_height: f64,
    margin_left: f64,
    margin_right: f64,
    margin_top: f64,
    margin_bottom: f64,
    border_top: f64,
    border_bottom: f64,
    border_left: f64,
    border_right: f64,
    title_height: f64,
    footer_height: f64,
    row_spacing: f64,
    horizontal_spacing: f64,
    stamp_width: f64,
    stamp_height: f64,
    stamp_box_adjust: f64,
    max_stamps_per_row: usize,
    max_rows_per_page: usize,
}

impl PageLayout {
    fn new() -> Self {
        // US Letter page size: 8.5" x 11" = 216mm x 279.4mm (but AlbumEasy uses 216 x 270)
        Self {
            page_width: 216.0,
            page_height: 270.0,
            margin_left: 10.0,
            margin_right: 10.0,
            margin_top: 10.0,
            margin_bottom: 10.0,
            border_top: 0.5,
            border_bottom: 3.0,
            border_left: 0.5,
            border_right: 0.1,
            title_height: 25.0,  // Estimated space for title
            footer_height: 15.0, // Estimated space for footer
            row_spacing: 3.0,    // Vertical spacing between rows
            horizontal_spacing: 3.0, // Horizontal spacing between stamps
            stamp_width: 28.0,
            stamp_height: 31.0,
            stamp_box_adjust: 3.0, // matches STAMP_BOXES_SIZE_ADJUST (adds mm border)
            max_stamps_per_row: 5,
            max_rows_per_page: 4,
        }
    }

    /// Construct a PageLayout using values from the provided Config (if any)
    fn from_config(cfg: &crate::config::Config) -> Self {
        let mut layout = PageLayout::new();

        if let Some(w) = cfg.reference_width_mm {
            layout.stamp_width = w as f64;
        }
        if let Some(h) = cfg.reference_height_mm {
            layout.stamp_height = h as f64;
        }
        if let Some(sb) = cfg.stamp_box_height {
            layout.stamp_height = sb as f64;
        }
        if let Some(adjust) = cfg.stamp_box_adjust {
            layout.stamp_box_adjust = adjust as f64;
        }
        if let Some(rs) = cfg.row_font_spacing {
            layout.row_spacing = rs as f64;
        }

        // Allow override of max rows per page when provided in config
        if let Some(mrp) = cfg.max_rows_per_page {
            layout.max_rows_per_page = mrp;
        } else {
            // If the user chooses to include additional header fields, reduce rows
            let mut extra_lines = 0;
            if cfg.include_design.unwrap_or(false) { extra_lines += 1; }
            if cfg.include_engraving.unwrap_or(false) { extra_lines += 1; }
            if cfg.include_watermark.unwrap_or(false) { extra_lines += 1; }
            // For each extra header line, conservatively reduce rows per page by 1
            layout.max_rows_per_page = std::cmp::max(1, layout.max_rows_per_page.saturating_sub(extra_lines));
        }

        layout
    }

    /// Calculate available content width on a page (for stamps horizontally)
    fn available_content_width(&self) -> f64 {
        self.page_width - self.margin_left - self.margin_right - self.border_left - self.border_right
    }

    /// Calculate available content height on a page
    fn available_content_height(&self) -> f64 {
        self.page_height
            - self.margin_top
            - self.margin_bottom
            - self.border_top
            - self.border_bottom
            - self.title_height
            - self.footer_height
    }

    /// Calculate space required for a series header, honoring config toggles
    fn calculate_header_space(&self, header_info: &PageHeaderInfo, cfg: &crate::config::Config) -> f64 {
        let mut space = 0.0;

        // Series name (TN 12, 2.0 spacing) - configurable
        if cfg.include_series.unwrap_or(true) && !header_info.series.is_empty() {
            space += 12.0 + 2.0;
        }

        // Date (TN 10, 3.0 spacing)
        if cfg.include_date.unwrap_or(true) && header_info.year > 0 {
            space += 10.0 + 3.0;
        }

        // Watermark (TN 8, 1.0 spacing)
        if cfg.include_watermark.unwrap_or(true) && !header_info.watermark.is_empty() {
            space += 8.0 + 1.0;
        }

        // Perforation (TN 8, 1.0 spacing)
        if cfg.include_perforation.unwrap_or(true) && !header_info.perforation.is_empty() {
            space += 8.0 + 1.0;
        }

        // Design (TN 8, 1.0 spacing)
        if cfg.include_design.unwrap_or(false) && !header_info.design.is_empty() {
            space += 8.0 + 1.0;
        }

        // Engraving (TN 8, 1.0 spacing)
        if cfg.include_engraving.unwrap_or(false) && !header_info.engraving.is_empty() {
            space += 8.0 + 1.0;
        }

        // Currency (TN 8, 1.5 spacing) - controlled by header_formatting.include_currency
        let include_currency = cfg
            .header_formatting
            .as_ref()
            .and_then(|hf| hf.include_currency)
            .unwrap_or(true);
        if include_currency && !header_info.currency.is_empty() {
            space += 8.0 + 1.5;
        }

        // Country line (TN 8, 1.0 spacing) - optional, header_info may not include this
        if cfg.include_country.unwrap_or(false) {
            // If header_info has country in currency or context, conservatively add space
            // (no dedicated country field is present in PageHeaderInfo currently)
            space += 0.0; // placeholder; only count when a country field exists
        }

        space
    }

    /// Calculate space required for a stamp row
    fn calculate_row_space(&self, _stamps_in_row: usize) -> f64 {
        // Row header (TN 6, 0.1 spacing above, 5.0 below)
        let row_header_space = 6.0 + 0.1 + 5.0;

        // Stamp space (height + spacing) - using max stamps per row for estimation
        let stamp_space = self.stamp_height + self.row_spacing;

        row_header_space + stamp_space
    }

    /// Check if a series/year combination can fit on current page
    fn can_fit_on_page(&self, header_info: &PageHeaderInfo, stamps: &[&crate::db_utils::StampStub],
                      used_space: f64, cfg: &crate::config::Config) -> bool {
        let header_space = self.calculate_header_space(header_info, cfg);
        let available_space = self.available_content_height() - used_space;

        // If header alone doesn't fit, need new page
        if header_space >= available_space {
            return false;
        }

        // Calculate rows needed for stamps
        let mut remaining_stamps = stamps.len();
        let mut rows_needed = 0;
        let mut space_for_stamps = available_space - header_space;

        while remaining_stamps > 0 && rows_needed < self.max_rows_per_page {
            let stamps_in_this_row = std::cmp::min(self.max_stamps_per_row, remaining_stamps);
            let row_space = self.calculate_row_space(stamps_in_this_row);

            if row_space > space_for_stamps {
                break; // This row won't fit
            }

            space_for_stamps -= row_space;
            remaining_stamps -= stamps_in_this_row;
            rows_needed += 1;
        }

        // If we couldn't fit all stamps, we need a new page
        remaining_stamps == 0
    }
}

/// Generates AlbumEasy output from a filtered list of stamps and album name
pub fn generate_album_output_from_stamps(
    stamps: &[crate::db_utils::StampStub],
    album_name: &str
) -> Result<usize, Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::path::Path;

    // Get output path: prefer the global filename from `types::ALBUM_FILE_NAME` if set,
    // then fall back to the local ALBUM_FILE_NAME, then to START/END year defaults.
    let output_path = if !crate::types::ALBUM_FILE_NAME.lock().unwrap().is_empty() {
        Path::new(&*crate::types::ALBUM_FILE_NAME.lock().unwrap()).to_path_buf()
    } else if !ALBUM_FILE_NAME.lock().unwrap().is_empty() {
        Path::new(&*ALBUM_FILE_NAME.lock().unwrap()).to_path_buf()
    } else {
        Path::new("Albums").join(album_name.replace(' ', "-")).join(
            format!("{}-{}-{}.txt",
                album_name.replace(' ', "-"),
                *START_YEAR.lock().unwrap(),
                *END_YEAR.lock().unwrap()
            )
        )
    };

    let mut output = std::io::BufWriter::new(File::create(&output_path)?);

    // Load configuration (if present) and initialize page layout calculator
    let cfg = crate::config::load_config("config.toml");
    // Write global album settings (use loaded config to avoid repeated loads)
    write_globals(&mut output, album_name, &cfg)?;

    // Group stamps by series, then by year within series
    let mut by_series: BTreeMap<String, BTreeMap<i32, Vec<&crate::db_utils::StampStub>>> = BTreeMap::new();

    for stamp in stamps {
        let series_key = if stamp.series.is_empty() {
            "Unknown Series".to_string()
        } else {
            // Clean the series name for consistent grouping
            clean_and_format_series(&stamp.series)
        };
        by_series.entry(series_key)
            .or_default()
            .entry(stamp.year)
            .or_default()
            .push(stamp);
    }

    let layout = PageLayout::from_config(&cfg);

    let mut page_num = 1;
    let mut current_page_used_space = 0.0;
    let mut rows_on_current_page = 0;

    // Start first page
    write_page_header_consistent(&mut output, page_num)?;

    for (series, series_years) in &by_series {
        println!("[INFO] Processing series '{}' with {} years", series, series_years.len());

        for (year, year_stamps) in series_years {
            println!("[INFO] Processing year {} with {} stamps in series '{}'", year, year_stamps.len(), series);

            // Extract header information from stamps in this year
            let header_info = extract_header_info(year_stamps);

            // Calculate space requirements
            let header_space = layout.calculate_header_space(&header_info, &cfg);
            // Compute per-stamp measured widths for this year to derive stamps-per-row
            let available_w = layout.available_content_width();
            let mut measured_widths: Vec<f64> = Vec::new();
                for stamp in year_stamps.iter() {
                let (w_mm, _h_mm) = crate::album_utils::calculate_stamp_dimensions(&stamp.image_path, album_name, &cfg);
                // apply stamp_box_adjust as per-side padding (both left and right)
                measured_widths.push(w_mm + 2.0 * layout.stamp_box_adjust);
            }
            // If we have measurements, compute median to avoid outliers
            let stamps_per_row_limit = if !measured_widths.is_empty() {
                measured_widths.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let mid = measured_widths.len() / 2;
                let median = if measured_widths.len() % 2 == 0 {
                    (measured_widths[mid - 1] + measured_widths[mid]) / 2.0
                } else {
                    measured_widths[mid]
                };
                // Compute how many median-width boxes fit (include horizontal spacing)
                let candidate = ((available_w + layout.horizontal_spacing) / (median + layout.horizontal_spacing)).floor() as usize;
                let candidate = std::cmp::max(1, candidate);
                // Per-series heuristic: large median widths should reduce stamps-per-row
                let mut capped = candidate;
                // Read configurable thresholds from cfg if present, otherwise use sensible defaults
                let cap3_thresh = cfg.median_width_cap_3.map(|v| v as f64).unwrap_or(36.0);
                let cap4_thresh = cfg.median_width_cap_4.map(|v| v as f64).unwrap_or(30.0);
                if median >= cap3_thresh {
                    capped = std::cmp::min(capped, 3);
                } else if median >= cap4_thresh {
                    capped = std::cmp::min(capped, 4);
                }
                std::cmp::min(layout.max_stamps_per_row, capped)
            } else {
                layout.max_stamps_per_row
            };

            // Check if we need a new page for this series/year
            let can_fit = layout.can_fit_on_page(&header_info, year_stamps, current_page_used_space, &cfg);

            if !can_fit || rows_on_current_page >= layout.max_rows_per_page {
                // Start new page
                page_num += 1;
                current_page_used_space = 0.0;
                rows_on_current_page = 0;
                write_page_header_consistent(&mut output, page_num)?;
                println!("[INFO] Started new page {} for series '{}'", page_num, series);
            }

            // Write the detailed header for this series/year
            write_page_header(&mut output, &header_info, &cfg)?;
            current_page_used_space += header_space;

            // Process stamps in rows (dynamic packing based on measured widths)
            let mut row_start_idx = 0;
            while row_start_idx < year_stamps.len() {
                // Build a row by packing as many stamps as fit horizontally
                let mut used_width = 0.0_f64;
                let mut stamps_in_row = 0_usize;
                let available_w = layout.available_content_width();

                for i in row_start_idx..year_stamps.len() {
                    let stamp = &year_stamps[i];
                    // Measure the stamp size using album_utils::calculate_stamp_dimensions
                    let (w_mm, h_mm) = crate::album_utils::calculate_stamp_dimensions(&stamp.image_path, album_name, &cfg);
                    // Include stamp_box_adjust which adds border to box size
                    // treat stamp_box_adjust as per-side padding (left+right, top+bottom)
                    let box_w = w_mm + 2.0 * layout.stamp_box_adjust;
                    let stamp_total_w = if stamps_in_row > 0 { layout.horizontal_spacing } else { 0.0 } + box_w;

                    if used_width + stamp_total_w > available_w || stamps_in_row >= stamps_per_row_limit {
                        break;
                    }

                    used_width += stamp_total_w;
                    stamps_in_row += 1;
                }

                if stamps_in_row == 0 {
                    // Force at least one stamp to avoid infinite loop
                    stamps_in_row = 1;
                }

                // Check vertical space for the row
                let row_space = layout.calculate_row_space(stamps_in_row);
                if current_page_used_space + row_space > layout.available_content_height() ||
                   rows_on_current_page >= layout.max_rows_per_page {
                    // Start new page
                    page_num += 1;
                    current_page_used_space = 0.0;
                    rows_on_current_page = 0;
                    write_page_header_consistent(&mut output, page_num)?;
                    write_page_header(&mut output, &header_info, &cfg)?;
                    current_page_used_space += header_space;
                    println!("[INFO] Started new page {} for continuing series '{}'", page_num, series);
                }

                // Write the row
                writeln!(output, "ROW_START_ES (TN 6 0.1 5.0)")?;
                current_page_used_space += row_space;
                rows_on_current_page += 1;

                // Emit stamps for this row with their measured dimensions
                for i in 0..stamps_in_row {
                    let stamp = &year_stamps[row_start_idx + i];
                    // Ensure the image path emitted points to the album's images directory
                    let image_name = if stamp.image_path.is_empty() {
                        format!("{}.jpg", stamp.catalog_num)
                    } else {
                        // Use only the filename portion so AlbumEasy receives relative "images/<file>"
                        match std::path::Path::new(&stamp.image_path).file_name() {
                            Some(fname) => fname.to_string_lossy().into_owned(),
                            None => stamp.image_path.trim_start_matches("images/").to_string(),
                        }
                    };

                    let (w_mm, h_mm) = crate::album_utils::calculate_stamp_dimensions(&stamp.image_path, album_name, &cfg);
                    // STAMP_ADD_IMG expects (width height "path" "..." "..." "") - include box adjust
                    // Emit raw measured stamp sizes. AlbumEasy uses the global
                    // STAMP_BOXES_SIZE_ADJUST to draw the visible border; avoid
                    // adding the adjust twice (previously we added 2*adjust here
                    // and also wrote a global STAMP_BOXES_SIZE_ADJUST). Emit the
                    // actual stamp dimensions and let AlbumEasy apply the border.
                    let out_w = w_mm;
                    let out_h = h_mm;
                    // Emit image path relative to the album file so AlbumEasy can locate it
                    write!(output, "STAMP_ADD_IMG ({:.1} {:.1} \"images/{}\" \"\" \"\" \"\")\n", out_w, out_h, image_name)?;
                    write!(output, "STAMP_HEADING (TN 8 \"SG : {}\" 1.0)\n", stamp.catalog_num)?;
                }

                row_start_idx += stamps_in_row;
            }
        }
    }

    println!("[INFO] AlbumEasy output complete. Output file: {:?}", output_path);
    // Return the number of pages generated
    Ok(page_num)
}

/// Extract header information from stamps
pub fn extract_header_info(stamps: &[&crate::db_utils::StampStub]) -> PageHeaderInfo {
    let mut info = PageHeaderInfo::default();

    if let Some(first_stamp) = stamps.first() {
        info.year = first_stamp.year;
        info.month = first_stamp.month.clone();
        info.day = first_stamp.day.clone();

        // Extract and clean series information
        if !first_stamp.series.is_empty() {
            info.series = clean_and_format_series(&first_stamp.series);
        }
    }

    // Extract unique values from all stamps
    use std::collections::HashSet;
    let mut watermarks = HashSet::new();
    let mut perforations = HashSet::new();

    for stamp in stamps {
        if !stamp.watermark.is_empty() {
            watermarks.insert(stamp.watermark.clone());
        }
        if !stamp.perforation.is_empty() {
            perforations.insert(stamp.perforation.clone());
        }
    }

    info.watermark = watermarks.into_iter().collect::<Vec<_>>().join(", ");
    info.perforation = perforations.into_iter().collect::<Vec<_>>().join(", ");

    info
}

/// Clean and format series text
pub fn clean_and_format_series(series: &str) -> String {
    let mut cleaned = series
        .replace('"', "'")
        .replace('\n', " ")
        .replace('\r', " ")
        .replace('\t', " ");

    // Remove type information
    if let Some(type_pos) = cleaned.find(", type") {
        cleaned = cleaned[..type_pos].to_string();
    }

    // Remove brackets
    if cleaned.starts_with('[') && cleaned.ends_with(']') {
        cleaned = cleaned[1..cleaned.len()-1].to_string();
    }

    // Remove common stamp-specific suffixes
    let suffixes_to_remove = ["type P", "type S", "type L"];
    for suffix in &suffixes_to_remove {
        if cleaned.ends_with(suffix) {
            cleaned = cleaned[..cleaned.len() - suffix.len()].trim().to_string();
            break;
        }
    }

    // Clean up extra whitespace
    cleaned.split_whitespace().collect::<Vec<&str>>().join(" ")
}

/// Write global album settings
pub fn write_globals<W: Write>(output: &mut W, album_name: &str, cfg: &crate::config::Config) -> Result<(), io::Error> {
    writeln!(output, "################################################################################")?;
    writeln!(output, "# {}", album_name)?;
    writeln!(output, "################################################################################")?;
    writeln!(output, "# Generated: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"))?;
    writeln!(output, "# User: {}@{}", whoami::username(), whoami::fallible::hostname().unwrap_or_else(|_| "unknown".to_string()))?;
    writeln!(output, "# OS: {}", std::env::consts::OS)?;
    writeln!(output, "# Architecture: {}", std::env::consts::ARCH)?;
    writeln!(output, "# Working Directory: {}", std::env::current_dir()?.display())?;
    writeln!(output)?;

    writeln!(output, "ALBUM_PAGES_SIZE  (216 270) #Set the page to US Letter (8.5 x 11)")?;
    writeln!(output, "#ALBUM_PAGES_SIZE (210.0 297.0) #Setting for A4")?;
    writeln!(output)?;
    writeln!(output, "ALBUM_PAGES_MARGINS  (10.0 10.0 10.0 10.0 )")?;
    writeln!(output)?;
    writeln!(output, "ALBUM_PAGES_BORDER (0.5 0.1 0.0 3.0)    #two line border")?;
    writeln!(output, "#ALBUM_PAGES_BORDER3  (0.1 0.0 0.0 3.0) #single line border")?;
    writeln!(output)?;
    writeln!(output, "ALBUM_PAGES_TITLE (TB 18 \"{}\")", album_name)?;
    writeln!(output)?;
    writeln!(output, "ALBUM_PAGES_SPACING (3.0 10.0)")?;
    writeln!(output)?;
    writeln!(output, "ALBUM_STAMP_IMG_ASPECT_RATIO (true) ")?;
    writeln!(output)?;
    writeln!(output, "#ALBUM_PAGES_NUMBER (TN 10 R)")?;
    writeln!(output, "ALBUM_PAGES_FOOTER_NUMBER (HN 10 C 1 \"Page \" \" of $PAGES$\")")?;
    writeln!(output)?;
    writeln!(output, "ALBUM_STAMP_IMG_NEW_SETTING (3.0 3.0 true)")?;
    writeln!(output)?;
    writeln!(output, "ROW_ALIGN_MIDDLE")?;
    writeln!(output)?;
    writeln!(output, "# The stamp boxes in this album are sized approximately")?;
    writeln!(output, "# 4mm larger than the actual stamps giving a 2mm border")?;
    writeln!(output, "# To have the boxes drawn the actual stamp size use")?;
    writeln!(output, "#STAMP_BOXES_SIZE_ADJUST(-4.0)")?;
    writeln!(output, "# To increase the border, e.g by 1mm use STAMP_BOXES_SIZE_ADJUST(2.0)")?;
    // Use stamp_box_adjust from provided config so emitted global matches
    // what the generator expects. Default to 3.0 mm for backward compatibility.
    let sb = cfg.stamp_box_adjust.unwrap_or(3.0_f32);
    writeln!(output, "STAMP_BOXES_SIZE_ADJUST ({:.1})", sb)?;
    writeln!(output, "################################################################################")?;
    writeln!(output)?;

    Ok(())
}

/// Write page header
pub fn write_page_header<W: Write>(output: &mut W, header_info: &PageHeaderInfo, cfg: &crate::config::Config) -> Result<(), io::Error> {
    // Write series information (configurable)
    if cfg.include_series.unwrap_or(true) && !header_info.series.is_empty() {
        let series_clean = clean_and_format_series(&header_info.series);
        writeln!(output, "PAGE_TEXT_CENTRE (TN 12 \"{}\" 2.0)", series_clean)?;
    }

    // Build date string. Prefer user-configured format if present.
    let date_format = cfg
        .header_formatting
        .as_ref()
        .and_then(|hf| hf.date_format.as_ref())
        .map(|s| s.as_str())
        .unwrap_or("dashes");

    let mut date_parts = vec![];
    if header_info.year > 0 {
        if !header_info.month.is_empty() && !header_info.day.is_empty() {
            if date_format == "commas" {
                // e.g. "November 15, 1897"
                date_parts.push(format!("{} {}, {}", header_info.month, header_info.day, header_info.year));
            } else {
                // default: dashes: "November - 15 - 1897"
                date_parts.push(format!("{} - {} - {}", header_info.month, header_info.day, header_info.year));
            }
        } else if !header_info.month.is_empty() {
            if date_format == "commas" {
                // e.g. "November 1897"
                date_parts.push(format!("{} {}", header_info.month, header_info.year));
            } else {
                date_parts.push(format!("{} - {}", header_info.month, header_info.year));
            }
        } else {
            date_parts.push(header_info.year.to_string());
        }
    }

    let date_str = date_parts.join(" - ");
    let date_clean = date_str.replace('"', "'");
    if cfg.include_date.unwrap_or(true) {
        writeln!(output, "PAGE_TEXT_CENTRE (TN 10 \"{}\" 3.0)", date_clean)?;
    }

    // Write watermark and perforation
    if cfg.include_watermark.unwrap_or(true) && !header_info.watermark.is_empty() {
        let watermark_clean = header_info.watermark.replace('"', "'");
        // Respect label_style if provided (abbreviated vs full)
        let wm_label = cfg
            .header_formatting
            .as_ref()
            .and_then(|hf| hf.label_style.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("abbreviated");
        if wm_label == "full" {
            writeln!(output, "PAGE_TEXT_CENTRE (TN 8 \"Watermark: {}\" 1.0)", watermark_clean)?;
        } else {
            writeln!(output, "PAGE_TEXT_CENTRE (TN 8 \"WM: {}\" 1.0)", watermark_clean)?;
        }
    }

    // Perforation is optional and controlled by config.include_perforation
    if cfg.include_perforation.unwrap_or(true) && !header_info.perforation.is_empty() {
        let perforation_clean = header_info.perforation.replace('"', "'");
        // Respect label_style for perforation label
        let perf_label = cfg
            .header_formatting
            .as_ref()
            .and_then(|hf| hf.label_style.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("abbreviated");
        if perf_label == "full" {
            writeln!(output, "PAGE_TEXT_CENTRE (TN 8 \"Perforation: {}\" 1.0)", perforation_clean)?;
        } else {
            writeln!(output, "PAGE_TEXT_CENTRE (TN 8 \"Perf.: {}\" 1.0)", perforation_clean)?;
        }
    }

    if cfg.include_design.unwrap_or(false) && !header_info.design.is_empty() {
        let design_clean = header_info.design.replace('"', "'");
        writeln!(output, "PAGE_TEXT_CENTRE (TN 8 \"Design: {}\" 1.0)", design_clean)?;
    }

    if cfg.include_engraving.unwrap_or(false) && !header_info.engraving.is_empty() {
        let engraving_clean = header_info.engraving.replace('"', "'");
        writeln!(output, "PAGE_TEXT_CENTRE (TN 8 \"Engraving: {}\" 1.0)", engraving_clean)?;
    }

    let currency_clean = header_info.currency.replace('"', "'");
    // Allow toggling inclusion of the currency field via header_formatting.include_currency
    let include_currency = cfg
        .header_formatting
        .as_ref()
        .and_then(|hf| hf.include_currency)
        .unwrap_or(true);
    if include_currency {
        writeln!(output, "PAGE_TEXT_CENTRE (TN 8 \"{}\" 1.5)", currency_clean)?;
    }

    Ok(())
}

/// Write consistent page header
pub fn write_page_header_consistent<W: Write>(output: &mut W, page_num: usize) -> Result<(), io::Error> {
    writeln!(output, "################################################################################")?;
    writeln!(output, "# Page {}", page_num)?;
    writeln!(output, "################################################################################")?;
    writeln!(output, "PAGE_START")?;
    Ok(())
}

/// Create new page
pub fn new_page_main<W: Write>(output: &mut W, page_num: &mut usize, rows_on_page: &mut usize) -> Result<(), io::Error> {
    *page_num += 1;
    *rows_on_page = 0;
    write_page_header_consistent(output, *page_num)?;
    Ok(())
}

/// Sanitize filename for filesystem safety
pub fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect()
}

/// Clean text for processing
pub fn clean_text(text: &str) -> String {
    text
        .replace('"', "'")
        .replace('\n', " ")
        .replace('\r', " ")
        .replace('\t', " ")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}

/// Load stamps from database
pub fn load_stamps_from_db(_db_path: &str, _album_name: &str) -> Result<Vec<crate::db_utils::StampStub>, Box<dyn std::error::Error>> {
    use rusqlite::Connection;

    let conn = Connection::open(_db_path)?;
    let mut stmt = conn.prepare(
        "SELECT year, month, day, sg_number, type, denomination, color, month_num, description, stamps_issued, image_path, remote_image_url, watermark, design, engraving, perforation, series, currency, context
         FROM stamps
         ORDER BY year, sg_number"
    )?;

    let stamp_iter = stmt.query_map([], |row| {
        Ok(crate::db_utils::StampStub {
            year: row.get(0)?,
            month: row.get(1)?,
            day: row.get(2)?,
            sg_number: row.get(3)?,
            stamp_type: row.get(4)?,
            denomination: row.get(5)?,
            color: row.get(6)?,
            multi_perf: false,
            desc: row.get(8)?,
            catalog_num: row.get(3)?,  // Use sg_number for catalog_num
            stamps_issued: row.get(9)?,
            image_path: row.get(10)?,
            remote_image_url: row.get(11)?,
            watermark: row.get(12)?,
            design: row.get(13)?,
            engraving: row.get(14)?,
            perforation: row.get(15)?,
            series: row.get(16)?,
            currency: row.get(17)?,
            context: row.get(18)?,
        })
    })?;

    let mut stamps = Vec::new();
    for stamp in stamp_iter {
        stamps.push(stamp?);
    }

    Ok(stamps)
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, HeaderFormatting};
    use crate::types::PageHeaderInfo;

    // Helper to build a Config with the provided toggles and label style
    fn make_config(
        include_design: bool,
        include_engraving: bool,
        include_watermark: bool,
        include_perforation: bool,
        include_catalog_numbers: bool,
        include_series: bool,
        include_country: bool,
        include_date: bool,
        label_style: &str,
        include_currency: bool,
    ) -> Config {
        let mut cfg = Config::default();
        cfg.include_design = Some(include_design);
        cfg.include_engraving = Some(include_engraving);
        cfg.include_watermark = Some(include_watermark);
        cfg.include_perforation = Some(include_perforation);
        cfg.include_catalog_numbers = Some(include_catalog_numbers);
        cfg.include_series = Some(include_series);
        cfg.include_country = Some(include_country);
        cfg.include_date = Some(include_date);
        cfg.header_formatting = Some(HeaderFormatting {
            date_format: None,
            label_style: Some(label_style.to_string()),
            series_style: None,
            include_currency: Some(include_currency),
            header_spacing: None,
        });
        cfg
    }

    #[test]
    fn test_header_toggles_exhaustive() {
        // Create a PageHeaderInfo with all fields non-empty so we can test toggles
        let mut header = PageHeaderInfo::default();
        header.year = 1897;
        header.month = "November".to_string();
        header.day = "15".to_string();
        header.series = "1897 -1898 Queen Victoria - Maple Leaf in Corners".to_string();
        header.perforation = "12".to_string();
        header.design = "Maple Leaf".to_string();
        header.engraving = "Line Engraving".to_string();
        header.watermark = "None".to_string();
        header.currency = "Canada".to_string();

        // iterate all boolean combinations for the 8 toggles (256 combinations)
        for bits in 0u16..(1u16 << 8) {
            let include_design = (bits & (1 << 0)) != 0;
            let include_engraving = (bits & (1 << 1)) != 0;
            let include_watermark = (bits & (1 << 2)) != 0;
            let include_perforation = (bits & (1 << 3)) != 0;
            let include_catalog_numbers = (bits & (1 << 4)) != 0;
            let include_series = (bits & (1 << 5)) != 0;
            let include_country = (bits & (1 << 6)) != 0;
            let include_date = (bits & (1 << 7)) != 0;

            for &label_style in &["abbreviated", "full"] {
                // include_currency true/false (2x)
                for &include_currency in &[true, false] {
                    let cfg = make_config(
                        include_design,
                        include_engraving,
                        include_watermark,
                        include_perforation,
                        include_catalog_numbers,
                        include_series,
                        include_country,
                        include_date,
                        label_style,
                        include_currency,
                    );

                    // Capture header output
                    let mut out: Vec<u8> = Vec::new();
                    write_page_header(&mut out, &header, &cfg).expect("write_page_header failed");
                    let s = String::from_utf8(out).expect("invalid utf8");

                    // series line
                    if include_series {
                        assert!(s.contains("TN 12"), "expected series line when include_series=true; cfg={:?}", cfg);
                    } else {
                        assert!(!s.contains("TN 12"), "did not expect series line when include_series=false; cfg={:?}", cfg);
                    }

                    // date line
                    if include_date {
                        assert!(s.contains("TN 10"), "expected date line when include_date=true; cfg={:?}", cfg);
                    } else {
                        assert!(!s.contains("TN 10"), "did not expect date line when include_date=false; cfg={:?}", cfg);
                    }

                    // watermark
                    if include_watermark {
                        if label_style == "full" {
                            assert!(s.contains("Watermark:"), "expected full watermark label; cfg={:?}", cfg);
                        } else {
                            assert!(s.contains("WM:"), "expected abbreviated watermark label; cfg={:?}", cfg);
                        }
                    } else {
                        assert!(!s.contains("WM:") && !s.contains("Watermark:"), "did not expect watermark when include_watermark=false; cfg={:?}", cfg);
                    }

                    // perforation
                    if include_perforation {
                        if label_style == "full" {
                            assert!(s.contains("Perforation:"), "expected full perforation label; cfg={:?}", cfg);
                        } else {
                            assert!(s.contains("Perf."), "expected abbreviated perforation label; cfg={:?}", cfg);
                        }
                    } else {
                        assert!(!s.contains("Perforation:") && !s.contains("Perf."), "did not expect perforation when include_perforation=false; cfg={:?}", cfg);
                    }

                    // design
                    if include_design {
                        assert!(s.contains("Design:"), "expected Design line when include_design=true; cfg={:?}", cfg);
                    } else {
                        assert!(!s.contains("Design:"), "did not expect Design when include_design=false; cfg={:?}", cfg);
                    }

                    // engraving
                    if include_engraving {
                        assert!(s.contains("Engraving:"), "expected Engraving line when include_engraving=true; cfg={:?}", cfg);
                    } else {
                        assert!(!s.contains("Engraving:"), "did not expect Engraving when include_engraving=false; cfg={:?}", cfg);
                    }

                    // currency
                    if include_currency {
                        assert!(s.contains(&header.currency), "expected currency line when include_currency=true; cfg={:?}", cfg);
                    } else {
                        assert!(!s.contains(&header.currency), "did not expect currency when include_currency=false; cfg={:?}", cfg);
                    }
                }
            }
        }
    }
}