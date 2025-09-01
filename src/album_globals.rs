// AlbumEasy output and layout logic
use log::{debug, info, error, warn};
use std::io::{Write, BufWriter};
use std::fs::File;
use std::path::Path;
use crate::db_utils::{init_db, get_all_series, get_stamps_by_series, StampStub};
use crate::image_utils::{calc_mm_per_pixel, estimate_real_dimensions};
use crate::config::{load_config, Config};
// Album global state and constants
use std::sync::Mutex;
use lazy_static::lazy_static;

#[derive(Debug, Clone, PartialEq)]
pub struct GenericAttrs {
    pub attrs: std::collections::HashMap<String, String>,
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
    let _ = writeln!(writer, "ROW_START_ES (TN 6 0.1 5.0)");
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
    let row_font_spacing = 5.0;    // ROW_START_JS (TN 6 0.1 5.0)
    // Declare and initialize missing variables for output logic
    let mut current_height: f64 = 0.0;
    let mut rows_on_page: usize = 0;
    let mut row_idx: usize = 0;
    let total_row_height: f64 = 0.0;
    let stamps_in_row: Vec<GenericAttrs> = Vec::new();
    // Restore main output loop: iterate over all series, fetch stamps, group into rows, and output
    let series_list = get_all_series(&conn).expect("Failed to fetch series");
    let max_rows_per_page = 4;
    let mut rows_on_page = 0;
    let mut page_num = 1;
    for series in series_list {
        let stamps = get_stamps_by_series(&conn, &series.name, &series.year.to_string()).expect("Failed to fetch stamps");
        if stamps.is_empty() {
            continue;
        }
        // Start a new page if needed (only when there are stamps to output)
        if rows_on_page == 0 {
            let _ = writeln!(output, "################################################################################");
            let _ = writeln!(output, "# Page {}", page_num);
            let _ = writeln!(output, "################################################################################");
            let _ = writeln!(output, "PAGE_START");
        }
        // Write series header and handle page breaks
        // Format year with month and day if available, single line
        let mut date_display = String::new();
        if series.year != 0 && series.month != "00" && series.day != "00" {
            date_display = format!("{} {} {}", series.day, series.month, series.year);
        } else if series.year != 0 && series.month != "00" {
            date_display = format!("{} {}", series.month, series.year);
        } else if series.year != 0 {
            date_display = format!("{}", series.year);
        }
        // Helper to collapse whitespace, remove newlines, and replace double quotes with single quotes
        fn single_line(s: &str) -> String {
            s.replace('"', "'").replace('\n', " ").split_whitespace().collect::<Vec<_>>().join(" ")
        }
        // Write series header with design and engraving, all single-line
        let _ = writeln!(output, "PAGE_TEXT_CENTRE (TN 8 \"{}\" 1.0)", single_line(&date_display));
        let _ = writeln!(output, "PAGE_TEXT_CENTRE (TN 8 \"{}\" 1.0)", single_line(&series.name));
        if !series.perforation.is_empty() {
            let _ = writeln!(output, "PAGE_TEXT_CENTRE (TN 8 \"Perforation {}\" 2.0)", single_line(&series.perforation));
        }
        if let Some(first_stamp) = stamps.first() {
            if !first_stamp.design.is_empty() {
                let _ = writeln!(output, "PAGE_TEXT_CENTRE (TN 8 \"Design: {}\" 1.0)", single_line(&clean_text(&first_stamp.design)));
            }
            if !first_stamp.engraving.is_empty() {
                let _ = writeln!(output, "PAGE_TEXT_CENTRE (TN 8 \"Engraving: {}\" 1.0)", single_line(&clean_text(&first_stamp.engraving)));
            }
        }

        let mut row: Vec<&StampStub> = Vec::new();
        let max_stamps_per_row = 5;
        for (i, stamp) in stamps.iter().enumerate() {
            row.push(stamp);
            if row.len() == max_stamps_per_row || i == stamps.len() - 1 {
                // Start a new row before outputting stamps
                let _ = writeln!(output, "ROW_START_JS (TN 6 0.1 5.0)");
                // Output each stamp in the row
                for stamp in &row {
                    let image_path = stamp.image_path.trim();
                    let full_image_path = format!("Albums/{}/{}", album_name.replace(' ', "-"), image_path);
                    let (raw_width_mm, raw_height_mm) = estimate_real_dimensions(&full_image_path, mm_per_pixel_w, mm_per_pixel_h)
                        .unwrap_or((reference_width_mm as f32, reference_height_mm as f32));
                    let est_width_mm = raw_width_mm / 10.0;
                    let est_height_mm = raw_height_mm / 10.0;
                    // SG number heading
                    let sg_heading = format!("SG {}", clean_text(&stamp.catalog_num));
                    // Centre: denomination, color, description, currency
                    let centre = format!(
                        "{}\n{}\n{}\n{}",
                        clean_text(&stamp.field3),
                        clean_text(&stamp.color),
                        clean_text(&stamp.desc),
                        ""
                    );
                    // Right: watermark only
                    let right = clean_text(&stamp.watermark);
                    if !image_path.is_empty() && Path::new(&full_image_path).exists() {
                        let line = format!(
                            "STAMP_ADD_IMG ({:.1} {:.1} \"{}\" \"{}\" \"{}\" \"{}\")",
                            est_width_mm, est_height_mm, image_path, "", centre, right
                        );
                        let _ = writeln!(output, "{}", line.replace("\r", "").replace("\n", "\\n"));
                    } else {
                        let line = format!(
                            "STAMP_ADD ({:.1} {:.1} \"Image\nNot\nAvailable\" \"{}\" \"{}\" \"{}\")",
                            est_width_mm,
                            est_height_mm,
                            "",
                            centre,
                            right
                        );
                        let _ = writeln!(output, "{}", line.replace("\r", "").replace("\n", "\\n"));
                    }
                    // Add SG heading above each stamp
                    let _ = writeln!(output, "STAMP_HEADING(HN 6 \"{}\")", sg_heading);
                }
                row.clear();
                rows_on_page += 1;
                // Page break if max rows per page reached
                if rows_on_page >= max_rows_per_page {
                    page_num += 1;
                    let _ = writeln!(output, "################################################################################");
                    let _ = writeln!(output, "# Page {}", page_num);
                    let _ = writeln!(output, "################################################################################");
                    let _ = writeln!(output, "PAGE_START");
                    rows_on_page = 0;
                }
            }
        }
    }
        }

    

/// Cleans problematic Unicode and formatting from text for AlbumEasy output
pub fn clean_text(s: &str) -> String {
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
        "ALBUM_PAGES_SPACING (6.0 6.0)",
        "",
        "#ALBUM_PAGES_NUMBER (TN 10 R)",
        "ALBUM_PAGES_FOOTER_NUMBER (HN 10 C 1 \"Page \" \" of $PAGES$\")",
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
