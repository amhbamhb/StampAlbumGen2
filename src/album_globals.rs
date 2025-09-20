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
    stamp_width: f64,
    stamp_height: f64,
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
            stamp_width: 28.0,
            stamp_height: 31.0,
            max_stamps_per_row: 5,
            max_rows_per_page: 4,
        }
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

    /// Calculate space required for a series header
    fn calculate_header_space(&self, header_info: &PageHeaderInfo) -> f64 {
        let mut space = 0.0;

        // Series name (TN 12, 2.0 spacing)
        if !header_info.series.is_empty() {
            space += 12.0 + 2.0;
        }

        // Date (TN 10, 3.0 spacing)
        if header_info.year > 0 {
            space += 10.0 + 3.0;
        }

        // Watermark (TN 8, 1.0 spacing)
        if !header_info.watermark.is_empty() {
            space += 8.0 + 1.0;
        }

        // Perforation (TN 8, 1.0 spacing)
        if !header_info.perforation.is_empty() {
            space += 8.0 + 1.0;
        }

        // Design (TN 8, 1.0 spacing)
        if !header_info.design.is_empty() {
            space += 8.0 + 1.0;
        }

        // Engraving (TN 8, 1.0 spacing)
        if !header_info.engraving.is_empty() {
            space += 8.0 + 1.0;
        }

        // Currency (TN 8, 1.5 spacing)
        if !header_info.currency.is_empty() {
            space += 8.0 + 1.5;
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
                      used_space: f64) -> bool {
        let header_space = self.calculate_header_space(header_info);
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
) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::path::Path;

    // Get output path
    let output_path = if !ALBUM_FILE_NAME.lock().unwrap().is_empty() {
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

    // Write global album settings
    write_globals(&mut output, album_name)?;

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

    // Initialize page layout calculator
    let layout = PageLayout::new();

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
            let header_space = layout.calculate_header_space(&header_info);

            // Check if we need a new page for this series/year
            let can_fit = layout.can_fit_on_page(&header_info, year_stamps, current_page_used_space);

            if !can_fit || rows_on_current_page >= layout.max_rows_per_page {
                // Start new page
                page_num += 1;
                current_page_used_space = 0.0;
                rows_on_current_page = 0;
                write_page_header_consistent(&mut output, page_num)?;
                println!("[INFO] Started new page {} for series '{}'", page_num, series);
            }

            // Write the detailed header for this series/year
            write_page_header(&mut output, &header_info)?;
            current_page_used_space += header_space;

            // Process stamps in rows
            let mut row_start_idx = 0;
            while row_start_idx < year_stamps.len() {
                // Check if we need a new page for this row
                let stamps_in_row = std::cmp::min(layout.max_stamps_per_row, year_stamps.len() - row_start_idx);
                let row_space = layout.calculate_row_space(stamps_in_row);

                if current_page_used_space + row_space > layout.available_content_height() ||
                   rows_on_current_page >= layout.max_rows_per_page {
                    // Start new page
                    page_num += 1;
                    current_page_used_space = 0.0;
                    rows_on_current_page = 0;
                    write_page_header_consistent(&mut output, page_num)?;
                    write_page_header(&mut output, &header_info)?;
                    current_page_used_space += header_space;
                    println!("[INFO] Started new page {} for continuing series '{}'", page_num, series);
                }

                // Write the row
                writeln!(output, "ROW_START_ES (TN 6 0.1 5.0)")?;
                current_page_used_space += row_space;
                rows_on_current_page += 1;

                // Calculate stamps per row
                let stamps_in_row = std::cmp::min(5, year_stamps.len() - row_start_idx);

                for i in 0..stamps_in_row {
                    let stamp = &year_stamps[row_start_idx + i];
                    let image_name = if stamp.image_path.is_empty() {
                        format!("{}.jpg", stamp.catalog_num)
                    } else {
                        stamp.image_path.trim_start_matches("images/").to_string()
                    };

                    write!(output, "STAMP_ADD_IMG (28.0 31.0 \"images/{}\" \"\" \"\" \"\")\n", image_name)?;
                    write!(output, "STAMP_HEADING (TN 8 \"SG : {}\" 1.0)\n", stamp.catalog_num)?;
                }

                row_start_idx += stamps_in_row;
            }
        }
    }

    println!("[INFO] AlbumEasy output complete. Output file: {:?}", output_path);
    Ok(())
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
pub fn write_globals<W: Write>(output: &mut W, album_name: &str) -> Result<(), io::Error> {
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
    writeln!(output, "STAMP_BOXES_SIZE_ADJUST (3)")?;
    writeln!(output, "################################################################################")?;
    writeln!(output)?;

    Ok(())
}

/// Write page header
pub fn write_page_header<W: Write>(output: &mut W, header_info: &PageHeaderInfo) -> Result<(), io::Error> {
    // Write series information
    if !header_info.series.is_empty() {
        let series_clean = clean_and_format_series(&header_info.series);
        writeln!(output, "PAGE_TEXT_CENTRE (TN 12 \"{}\" 2.0)", series_clean)?;
    }

    // Build date string
    let mut date_parts = vec![];
    if header_info.year > 0 {
        if !header_info.month.is_empty() && !header_info.day.is_empty() {
            date_parts.push(format!("{} - {} - {}", header_info.month, header_info.day, header_info.year));
        } else if !header_info.month.is_empty() {
            date_parts.push(format!("{} - {}", header_info.month, header_info.year));
        } else {
            date_parts.push(header_info.year.to_string());
        }
    }

    let date_str = date_parts.join(" - ");
    let date_clean = date_str.replace('"', "'");
    writeln!(output, "PAGE_TEXT_CENTRE (TN 10 \"{}\" 3.0)", date_clean)?;

    // Write watermark and perforation
    if !header_info.watermark.is_empty() {
        let watermark_clean = header_info.watermark.replace('"', "'");
        writeln!(output, "PAGE_TEXT_CENTRE (TN 8 \"WM: {}\" 1.0)", watermark_clean)?;
    }

    if !header_info.perforation.is_empty() {
        let perforation_clean = header_info.perforation.replace('"', "'");
        writeln!(output, "PAGE_TEXT_CENTRE (TN 8 \"Perforation: {}\" 1.0)", perforation_clean)?;
    }

    if !header_info.design.is_empty() {
        let design_clean = header_info.design.replace('"', "'");
        writeln!(output, "PAGE_TEXT_CENTRE (TN 8 \"Design: {}\" 1.0)", design_clean)?;
    }

    if !header_info.engraving.is_empty() {
        let engraving_clean = header_info.engraving.replace('"', "'");
        writeln!(output, "PAGE_TEXT_CENTRE (TN 8 \"Engraving: {}\" 1.0)", engraving_clean)?;
    }

    let currency_clean = header_info.currency.replace('"', "'");
    writeln!(output, "PAGE_TEXT_CENTRE (TN 8 \"{}\" 1.5)", currency_clean)?;

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

/// Calculate stamp dimensions
pub fn calculate_stamp_dimensions(_image_name: &str, _album_name: &str) -> (f64, f64) {
    // Default dimensions - in a real implementation this would analyze the actual image
    (28.0, 31.0)
}