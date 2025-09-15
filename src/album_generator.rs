//! # Album Generator Module
//!
//! This module handles the core album generation logic for StampAlbumGen2,
//! including stamp layout, page management, and AlbumEasy output generation.

use std::collections::BTreeMap;
use std::io::{BufWriter, Write};
use std::fs::File;

/// Page header information structure
#[derive(Debug, Clone)]
pub struct PageHeaderInfo {
    pub year: String,
    pub month: String,
    pub day: String,
    pub series: String,
    pub description: String,
}

/// Generates AlbumEasy output from a filtered list of stamps
pub fn generate_album_output_from_stamps<F>(
    stamps: &[crate::db_utils::StampStub],
    album_name: &str,
    mut page_callback: Option<F>
) where
    F: FnMut(usize),
{
    use std::path::Path;

    // Load config values
    let config_path = crate::file_utils::find_config_path();
    let config = crate::file_utils::load_config(&config_path);
    let reference_width_mm = config.reference_width_mm.unwrap_or(200.0) as f64;
    let reference_height_mm = config.reference_height_mm.unwrap_or(240.0) as f64;

    // Calculate mm-per-pixel ratios
    let _ = crate::stamp_dimensions::calc_mm_per_pixel_embedded(
        reference_width_mm as f32,
        reference_height_mm as f32
    );

    let output_path = if !crate::ALBUM_FILE_NAME.lock().unwrap().is_empty() {
        Path::new(&*crate::ALBUM_FILE_NAME.lock().unwrap()).to_path_buf()
    } else {
        Path::new("Albums")
            .join(album_name.replace(' ', "-"))
            .join(format!("{}.txt", album_name.replace(' ', "-")))
    };

    let mut output = BufWriter::new(File::create(&output_path)
        .expect("Failed to create output file"));

    // Write global album settings
    write_globals(&mut output, album_name);

    // Group stamps by series, then by year within series
    let mut by_series: BTreeMap<String, BTreeMap<i32, Vec<&crate::db_utils::StampStub>>> = BTreeMap::new();
    for stamp in stamps {
        let series = stamp.series.clone();
        let year = stamp.year;
        by_series.entry(series)
            .or_insert_with(BTreeMap::new)
            .entry(year)
            .or_insert_with(Vec::new)
            .push(stamp);
    }

    // Process each series
    for (_series_name, years) in by_series {
        // Process each year in the series
        for (year, year_stamps) in years {
            process_year(&mut output, year, &year_stamps, album_name);

            if let Some(ref mut callback) = page_callback {
                callback(1); // Simplified callback
            }
        }
    }
}

/// Extracts header information from stamps
pub fn extract_header_info(stamps: &[&crate::db_utils::StampStub]) -> PageHeaderInfo {
    if stamps.is_empty() {
        return PageHeaderInfo {
            year: "Unknown".to_string(),
            month: "Unknown".to_string(),
            day: "Unknown".to_string(),
            series: "Unknown".to_string(),
            description: "No stamps".to_string(),
        };
    }

    let first_stamp = stamps[0];
    PageHeaderInfo {
        year: first_stamp.year.to_string(),
        month: first_stamp.month.clone(),
        day: first_stamp.day.clone(),
        series: first_stamp.series.clone(),
        description: first_stamp.desc.clone(),
    }
}

/// Writes page header to output
pub fn write_page_header(output: &mut impl Write, header_info: &PageHeaderInfo) {
    writeln!(output, "PAGE {}", header_info.year).unwrap();
    writeln!(output, "YEAR {}", header_info.year).unwrap();
    if !header_info.month.is_empty() && header_info.month != "0" {
        writeln!(output, "MONTH {}", header_info.month).unwrap();
    }
    if !header_info.day.is_empty() && header_info.day != "0" {
        writeln!(output, "DAY {}", header_info.day).unwrap();
    }
}

/// Loads stamps from database
pub fn load_stamps_from_db(db_path: &str) -> Vec<crate::db_utils::StampStub> {
    use rusqlite::Connection;

    let conn = Connection::open(db_path)
        .expect("Failed to open database");

    let mut stmt = conn.prepare(
        "SELECT id, country, series, year, month, month_num, day, sg_number, type, denomination, color, description, stamps_issued, mint_condition, unused, used, letter_fdc, currency, image_path, remote_image_url, watermark, design, engraving, perforation, context FROM stamps ORDER BY year, month_num, day, sg_number"
    ).expect("Failed to prepare statement");

    let stamps = stmt.query_map([], |row| {
        Ok(crate::db_utils::StampStub {
            year: row.get(3)?,
            month: row.get(4)?,
            day: row.get(6)?,
            field1: row.get(1)?, // country
            field2: row.get(2)?, // series
            field3: row.get(9)?, // denomination
            color: row.get(10)?,
            multi_perf: false, // We'll need to determine this from perforation
            desc: row.get(11)?, // description
            catalog_num: row.get(7)?, // sg_number
            field4: row.get(8)?, // type/stamp_type
            image_path: row.get(18)?,
            watermark: row.get(20)?,
            design: row.get(21)?,
            engraving: row.get(22)?,
            perforation: row.get(23)?,
            series: row.get(2)?, // series
        })
    }).expect("Failed to query stamps");

    stamps.filter_map(|r| r.ok()).collect()
}

/// Generates album output from database
pub fn generate_album_output<F>(db_path: &str, album_name: &str, _page_callback: Option<F>)
where
    F: FnMut(usize),
{
    let stamps = load_stamps_from_db(db_path);
    generate_album_output_from_stamps(&stamps, album_name, _page_callback);
}

/// Writes global album settings in proper AlbumEasy format
pub fn write_globals<W: Write>(writer: &mut W, album_name: &str) {
    writeln!(writer, "################################################################################").unwrap();
    writeln!(writer, "# {}", album_name).unwrap();
    writeln!(writer, "################################################################################").unwrap();
    writeln!(writer).unwrap();
    writeln!(writer, "ALBUM_PAGES_SIZE  (215.9 279.4) #Set the page to US Letter (8.5 x 11)").unwrap();
    writeln!(writer, "#ALBUM_PAGES_SIZE (210.0 297.0) #Setting for A4").unwrap();
    writeln!(writer).unwrap();
    writeln!(writer, "ALBUM_PAGES_MARGINS  (15.0 10.0 10.0 10.0 )").unwrap();
    writeln!(writer).unwrap();
    writeln!(writer, "ALBUM_PAGES_BORDER (0.5 0.1 0.0 3.0)    #two line border").unwrap();
    writeln!(writer, "#ALBUM_PAGES_BORDER3  (0.1 0.0 0.0 3.0) #single line border").unwrap();
    writeln!(writer).unwrap();
    writeln!(writer, "ALBUM_PAGES_TITLE (TB 18 \"{}\")", album_name).unwrap();
    writeln!(writer).unwrap();
    writeln!(writer, "ALBUM_PAGES_SPACING (5.0 3.0)").unwrap();
    writeln!(writer).unwrap();
    writeln!(writer, "#ALBUM_PAGES_NUMBER (TN 10 R)").unwrap();
    writeln!(writer).unwrap();
    writeln!(writer, "ALBUM_STAMP_NEW_IMG_SETTING (6.0 6.0 true)").unwrap();
    writeln!(writer).unwrap();
    writeln!(writer, "ROW_ALIGN_MIDDLE").unwrap();
    writeln!(writer).unwrap();
    writeln!(writer, "# The stamp boxes in this album are sized approximately").unwrap();
    writeln!(writer, "# 4mm larger than the actual stamps giving a 2mm border").unwrap();
    writeln!(writer, "# To have the boxes drawn the actual stamp size use").unwrap();
    writeln!(writer, "#STAMP_BOXES_SIZE_ADJUST(-4.0)").unwrap();
    writeln!(writer, "# To increase the border, e.g by 1mm use STAMP_BOXES_SIZE_ADJUST(2.0)").unwrap();
    writeln!(writer, "STAMP_BOXES_SIZE_ADJUST (0)").unwrap();
    writeln!(writer, "################################################################################").unwrap();
    writeln!(writer).unwrap();
}

/// Writes series header (not used in new AlbumEasy format)
pub fn write_series_header<W: Write>(_writer: &mut W, _series_name: &str) {
    // Series headers are handled within the page structure in the new format
}

/// Helper function to convert month number to month name
fn get_month_name(month: &str) -> String {
    match month {
        "1" => "January".to_string(),
        "2" => "February".to_string(),
        "3" => "March".to_string(),
        "4" => "April".to_string(),
        "5" => "May".to_string(),
        "6" => "June".to_string(),
        "7" => "July".to_string(),
        "8" => "August".to_string(),
        "9" => "September".to_string(),
        "10" => "October".to_string(),
        "11" => "November".to_string(),
        "12" => "December".to_string(),
        _ => month.to_string(),
    }
}

/// Processes stamps for a specific year in proper AlbumEasy format
pub fn process_year<W: Write>(writer: &mut W, year: i32, stamps: &[&crate::db_utils::StampStub], album_name: &str) {
    if stamps.is_empty() {
        return;
    }

    // Start a new page for this year
    writeln!(writer, "################################################################################").unwrap();
    writeln!(writer, "PAGE_START").unwrap();

    // Add year header
    writeln!(writer, "PAGE_TEXT_CENTRE_VSPACE (TN 10 \"{}\" 3.0)", year).unwrap();

    // Try to extract date information from the first stamp
    let first_stamp = stamps[0];
    if !first_stamp.month.is_empty() && first_stamp.month != "0" {
        let month_name = get_month_name(&first_stamp.month);
        if !first_stamp.day.is_empty() && first_stamp.day != "0" {
            writeln!(writer, "PAGE_TEXT_CENTRE_VSPACE (TN 10 \"{} {}, {}\" 0.5)", month_name, first_stamp.day, year).unwrap();
        } else {
            writeln!(writer, "PAGE_TEXT_CENTRE_VSPACE (TN 10 \"{}, {}\" 0.5)", month_name, year).unwrap();
        }
    }

    // Add description if available
    if !first_stamp.desc.is_empty() && first_stamp.desc != " " {
        writeln!(writer, "PAGE_TEXT_CENTRE_VSPACE (TN 10 \"{}\" 0.5)", first_stamp.desc).unwrap();
    }

    // Add perforation info if available
    if !first_stamp.perforation.is_empty() {
        writeln!(writer, "PAGE_TEXT_CENTRE_VSPACE (TN 10 \"Perforation {}\" 2.0)", first_stamp.perforation).unwrap();
    }

    writeln!(writer, "ROW_START_FS (TN 6 0.1 5.0)").unwrap();

    // Process stamps in rows of 4, with page breaks
    let mut stamp_count = 0;
    let mut row_count = 0;
    let max_rows_per_page = 4;
    let stamps_per_row = 4;

    for stamp in stamps {
        // Check if we need to start a new page
        if row_count >= max_rows_per_page {
            // Start a new page
            writeln!(writer, "################################################################################").unwrap();
            writeln!(writer, "PAGE_START").unwrap();
            writeln!(writer, "PAGE_TEXT_CENTRE_VSPACE (TN 10 \"{} (continued)\" 3.0)", year).unwrap();

            // Reset counters for new page
            row_count = 0;
            stamp_count = 0;
        }

        // Start a new row every 4 stamps
        if stamp_count % stamps_per_row == 0 {
            writeln!(writer, "ROW_START_FS (TN 6 0.1 5.0)").unwrap();
            row_count += 1;
        }

        // Use the actual image path from database, or generate from catalog number if empty
        let image_path = if !stamp.image_path.is_empty() && stamp.image_path != "images/.jpg" {
            // Check if the image path has a valid extension
            let path_lower = stamp.image_path.to_lowercase();
            if path_lower.ends_with(".jpg") || path_lower.ends_with(".jpeg") || path_lower.ends_with(".png") {
                stamp.image_path.clone()
            } else {
                // Invalid image format, use placeholder
                crate::placeholder_image::get_placeholder_image_path()
            }
        } else {
            // No image path in database, use placeholder
            crate::placeholder_image::get_placeholder_image_path()
        };

        // Calculate actual stamp dimensions instead of using hardcoded values
        let (stamp_width, stamp_height) = if image_path != crate::placeholder_image::get_placeholder_image_path() {
            crate::stamp_dimensions::calculate_stamp_dimensions(&image_path, album_name)
        } else {
            (30.0, 30.0) // Fallback for placeholder
        };

        // Format stamp description
        let mut description = String::new();
        if !stamp.field3.is_empty() { // denomination
            description.push_str(&stamp.field3);
        }
        if !stamp.color.is_empty() {
            if !description.is_empty() {
                description.push_str(" ");
            }
            description.push_str(&stamp.color);
        }
        if description.is_empty() {
            description = stamp.desc.clone();
        }

        // Escape quotes in description
        let escaped_desc = description.replace("\"", "\\\"");

        // Write stamp entry with calculated dimensions
        writeln!(writer, "STAMP_ADD_IMG ({:.1} {:.1} \"{}\" \"\" \"{}\" \"\")", stamp_width, stamp_height, image_path, escaped_desc).unwrap();

        stamp_count += 1;
    }

    writeln!(writer).unwrap();
}

/// Writes a line to the output
pub fn write_line<W: Write>(writer: &mut W, line: &str) {
    writeln!(writer, "{}", line).unwrap();
}