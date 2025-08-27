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

use std::io::Write;
use crate::db_utils::{init_db, get_all_series, get_stamps_by_series};
use crate::image_utils::{calc_mm_per_pixel, estimate_real_dimensions};
use std::fs::File;
use std::io::BufWriter;

/// Main entry point for AlbumEasy output generation.
/// Queries the database, formats output, and writes to a .txt file for AlbumEasy import.
pub fn generate_album_output(db_path: &str, album_name: &str) {
    let reference_image = "Albums/Great-Britain/images/BG1-s.jpg";
    let reference_width_mm = 200.0;
    let reference_height_mm = 240.0;
    let (mm_per_pixel_w, mm_per_pixel_h) = match calc_mm_per_pixel(reference_image, reference_width_mm, reference_height_mm) {
        Some(ratio) => ratio,
        None => {
            eprintln!("[WARN] Reference image not found: {}. Using default ratio (1.0, 1.0)", reference_image);
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
    let usable_height = page_height - top_margin - bottom_margin;
    // Font and spacing from AlbumEasy settings
    let header_font_spacing = 3.0; // PAGE_TEXT_CENTRE (TN 10 ... 3.0)
    let row_font_spacing = 5.0;    // ROW_START_FS (TN 6 0.1 5.0)
    let stamp_box_height = 24.0;   // Estimate: typical stamp box height
    let header_lines = 4;          // Year, name, desc, perforation
    let header_height = (header_lines as f64) * header_font_spacing;
    let row_height = stamp_box_height + row_font_spacing;

    let series_list = get_all_series(&conn).expect("Failed to get series");
    let mut page_counter = 1;
    let mut current_height = 0.0;
    let mut first_page = true;
    // Always start the first page before any content
    let _ = writeln!(output, "### Page {} ###", page_counter);
    let _ = writeln!(output, "PAGE_START\n################################################################################");
    page_counter += 1;
    first_page = false;
    for series in series_list {
        let stamps = get_stamps_by_series(&conn, &series.name, &series.year.to_string()).expect("Failed to get stamps");
        let mut stamp_rows: Vec<Vec<_>> = Vec::new();
        let mut row: Vec<_> = Vec::new();
        for (i, stamp) in stamps.into_iter().enumerate() {
            row.push(stamp);
            if row.len() == 5 {
                stamp_rows.push(row);
                row = Vec::new();
            }
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
                // Use only series.name for the header line
                let name_line = clean_text(series.name.trim()).replace("\n", " ").replace("  ", " ");
                if !name_line.is_empty() {
                    header_lines_vec.push((format!("PAGE_TEXT_CENTRE (TN 8 \"{}\" 1.0)", name_line), 1.0));
                }
                // Perforation
                let perf_trimmed = clean_text(series.perforation.trim()).replace("\n", " ").replace("  ", " ");
                if !perf_trimmed.is_empty() {
                    header_lines_vec.push((format!("PAGE_TEXT_CENTRE (TN 8 \"Perforation {}\" 1.0)", perf_trimmed), 1.0));
                }
                // Add Watermark, Engraving, Design from first stamp in the series (if available)
                let first_stamp = stamp_rows.get(0).and_then(|row| row.get(0));
                if let Some(stamp) = first_stamp {
                    let watermark = clean_text(stamp.watermark.trim()).replace("\n", " ").replace("  ", " ");
                    if !watermark.is_empty() {
                        header_lines_vec.push((format!("PAGE_TEXT_CENTRE (TN 8 \"WM = {}\" 1.0)", watermark), 1.0));
                    }
                    // Sweep design field: extract first unlabeled part as Design, and value after 'Engraving:' as Engraving
                    let design_field = stamp.design.replace("\n", " ").replace("  ", " ");
                    let mut design = String::new();
                    let mut engraving = String::new();
                    let mut parts = design_field.split("Engraving:");
                    // First part before 'Engraving:' is Design
                    design = parts.next().unwrap_or("").split("Perforation:").next().unwrap_or("").trim().to_string();
                    // Second part after 'Engraving:' is Engraving
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
                // Calculate total height of header block plus vspace before/after header
                let header_block_height: f64 = header_lines_vec.iter().map(|(_, h)| *h).sum();
                let vspace_before_header = 8.0;
                let vspace_after_header = 12.5;
                if current_height + vspace_before_header + header_block_height + vspace_after_header > usable_height {
                    if !first_page {
                        let _ = writeln!(output, "\n################################################################################");
                    }
                    let _ = writeln!(output, "### Page {} ###", page_counter);
                    let _ = writeln!(output, "PAGE_START\n################################################################################");
                    page_counter += 1;
                    current_height = 0.0;
                    first_page = false;
                }
                // Insert PAGE_VSPACE (8) before header
                let _ = writeln!(output, "PAGE_VSPACE (8)");
                current_height += vspace_before_header;
                for (line, h) in header_lines_vec {
                    let _ = writeln!(output, "{}", line);
                    current_height += h;
                }
                // Insert PAGE_VSPACE (12.5) after header
                let _ = writeln!(output, "PAGE_VSPACE (12.5)");
                current_height += vspace_after_header;
                header_written = true;
            }
            // Calculate row height including extra text lines for all stamps in the row
            let stamps_in_row = &stamp_rows[row_idx];
            let mut max_stamp_height = stamp_box_height;
            let text_line_height = 3.0; // TN 10 font spacing for text under stamp
            for stamp in stamps_in_row {
                let left_text = clean_text(&format!("{} {}", stamp.year, stamp.field1));
                let centre_text = clean_text(&format!("{} {} {} {}", stamp.field2, stamp.field3, stamp.color, stamp.desc));
                let right_text = clean_text(&format!("{} {}", stamp.catalog_num, stamp.field4));
                let mut lines = 0;
                lines += left_text.matches('\n').count();
                lines += centre_text.matches('\n').count();
                lines += right_text.matches('\n').count();
                // Add 1 for each field (at least one line per field)
                lines += 3;
                let stamp_height = stamp_box_height + (lines as f64) * text_line_height;
                if stamp_height > max_stamp_height {
                    max_stamp_height = stamp_height;
                }
            }
            let total_row_height = max_stamp_height + row_font_spacing;
            // Check if this row fits before adding
            if current_height + total_row_height > usable_height {
                // Start new page and repeat header
                if !first_page {
                    let _ = writeln!(output, "\n################################################################################");
                }
                let _ = writeln!(output, "### Page {} ###", page_counter);
                let _ = writeln!(output, "PAGE_START\n################################################################################");
                page_counter += 1;
                current_height = 0.0;
                header_written = false;
                continue; // re-check header on next loop
            }
            let _ = writeln!(output, "ROW_START_FS (TN 6 0.1 5.0)");
            for stamp in stamps_in_row {
                let image_path = stamp.image_path.trim();
                let full_image_path = format!("Albums/Great-Britain/{}", image_path);
                let (raw_width_mm, raw_height_mm) = estimate_real_dimensions(&full_image_path, mm_per_pixel_w, mm_per_pixel_h)
                    .unwrap_or((reference_width_mm, reference_height_mm));
                let est_width_mm = raw_width_mm / 10.0;
                let est_height_mm = raw_height_mm / 10.0;
                // Build multi-line text fields for AlbumEasy
                let sg_number = clean_text(&stamp.catalog_num);
                let denomination = clean_text(&stamp.field1);
                let color = clean_text(&stamp.color);
                let stamps_issued = clean_text(&stamp.field2);
                let mint_condition = if stamp.field3.trim().is_empty() { "Unused".to_string() } else { clean_text(&stamp.field3) };
                let used = clean_text(&stamp.field4);
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
        "ALBUM_PAGES_SIZE  (215.9 279.4) #Set the page to US Letter (8.5 x 11)",
        "#ALBUM_PAGES_SIZE (210.0 297.0) #Setting for A4",
        "",
        "ALBUM_PAGES_MARGINS  (10.0 10.0 10.0 10.0 )",
        "",
        "ALBUM_PAGES_BORDER (0.5 0.1 0.0 3.0)    #two line border",
        "#ALBUM_PAGES_BORDER3  (0.1 0.0 0.0 3.0) #single line border",
        "",
        &format!("ALBUM_PAGES_TITLE (TB 18 \"{}\")", album_name),
        "",
        "ALBUM_PAGES_SPACING (5.0 3.0)",
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
