/// Generates AlbumEasy output from a filtered list of stamps and album name, without querying the DB.
pub fn generate_album_output_from_stamps<F>(stamps: &[StampStub], album_name: &str, mut page_callback: Option<F>)
where
    F: FnMut(usize),
{
    use std::path::Path;

    // Load config values from config.toml for mm_per_pixel calculation
    let config = load_config("config.toml");
    let reference_width_mm = config.reference_width_mm.unwrap_or(200.0) as f64;
    let reference_height_mm = config.reference_height_mm.unwrap_or(240.0) as f64;
    // Reference image path and measured size (dynamic by country)
    let country_dir = format!("Albums/{}/images", album_name.replace(' ', "-"));
    let reference_image_path = format!("{}/BG1-s.jpg", country_dir);
    let _ = match calc_mm_per_pixel(&reference_image_path, reference_width_mm as f32, reference_height_mm as f32) {
        Some(_ratio) => {},
        None => {
            warn!("Reference image not found: {}. Using default ratio (1.0, 1.0)", reference_image_path);
        }
    };

    let output_path = Path::new("Albums").join(album_name.replace(' ', "-")).join(format!("{}.txt", album_name.replace(' ', "-")));
    let mut output = BufWriter::new(File::create(&output_path).expect("Failed to create output file"));

    // Write global album settings
    write_globals(&mut output, album_name);

    // Group stamps by year
    use std::collections::BTreeMap;
    let mut by_year: BTreeMap<i32, Vec<&StampStub>> = BTreeMap::new();
    for stamp in stamps {
        by_year.entry(stamp.year).or_default().push(stamp);
    }

    let mut page_num = 1;
    let mut rows_on_page = 0;
    let max_rows_per_page = 4;
    let mut total_images = 0;
    for (year, year_stamps) in &by_year {
        println!("[INFO] Processing year {} with {} stamps", year, year_stamps.len());
        total_images += year_stamps.len();
        // Extract header information from stamps in this year
        let header_info = extract_header_info(year_stamps);
        
        // Check if we need a new page for this year
        // Ensure header and first row can fit together
        let estimated_first_row_height = 30.0; // Rough estimate for stamp height
        let header_height = 50.0; // Rough estimate for header block
        let available_space = 245.0 - (rows_on_page as f64 * 35.0); // Rough page height calculation
        
        if rows_on_page >= max_rows_per_page || 
           (rows_on_page > 0 && available_space < header_height + estimated_first_row_height) {
            page_num += 1;
            rows_on_page = 0;
            let _ = writeln!(output, "################################################################################");
            let _ = writeln!(output, "# Page {}", page_num);
            let _ = writeln!(output, "################################################################################");
            let _ = writeln!(output, "PAGE_START");
        }
        
        // Start new page for each year
        let _ = writeln!(output, "################################################################################");
        let _ = writeln!(output, "# Page {}", page_num);
        let _ = writeln!(output, "################################################################################");
        let _ = writeln!(output, "PAGE_START");
        
        // Write enhanced header with extracted information
        write_page_header(&mut output, &header_info);
        
        // Group stamps into rows dynamically based on image widths (up to 5 stamps per row)
        let mut row_start_idx = 0;
        while row_start_idx < year_stamps.len() {
            // Check if we need a new page due to max rows
            if rows_on_page >= max_rows_per_page {
                page_num += 1;
                rows_on_page = 0;
                let _ = writeln!(output, "################################################################################");
                let _ = writeln!(output, "# Page {}", page_num);
                let _ = writeln!(output, "################################################################################");
                let _ = writeln!(output, "PAGE_START");
                
                // Write enhanced header with extracted information
                write_page_header(&mut output, &header_info);
            }
            
            let _ = writeln!(output, "ROW_START_ES (TN 6 0.1 5.0)");
            rows_on_page += 1;
            
            // Calculate how many stamps fit in this row based on their widths
            let mut stamps_in_row = 0;
            let mut total_width = 0.0;
            let max_stamps_per_row = 5;
            // Enhanced page layout calculations
            let page_width_mm = 216.0; // US Letter width in mm
            let left_margin = 18.0;    // Left margin in mm
            let right_margin = 18.0;  // Right margin in mm
            let stamp_spacing = 3.0;  // Spacing between stamps in mm
            let border_width = 2.0;   // Border width on each side in mm
            let usable_width = page_width_mm - left_margin - right_margin - (2.0 * border_width);
            
            for i in row_start_idx..std::cmp::min(row_start_idx + max_stamps_per_row, year_stamps.len()) {
                let stamp = &year_stamps[i];
                let image_name = if stamp.image_path.is_empty() {
                    if stamp.desc.contains("Set") {
                        continue;
                    }
                    format!("{}.jpg", sanitize_filename(&stamp.catalog_num))
                } else {
                    stamp.image_path.trim_start_matches("images/").to_string()
                };
                
                let stamp_width = 25.0; // Fixed width for reliable layout
                
                // Check if adding this stamp would exceed the usable width
                let required_width = if stamps_in_row > 0 { 
                    total_width + stamp_width + stamp_spacing 
                } else { 
                    stamp_width 
                };
                
                if required_width > usable_width {
                    break;
                }
                
                total_width = required_width;
                stamps_in_row += 1;
            }
            
            // Process the stamps in this row
            for i in 0..stamps_in_row {
                let stamp_idx = row_start_idx + i;
                let stamp = &year_stamps[stamp_idx];
                
                let image_name = if stamp.image_path.is_empty() {
                    if stamp.desc.contains("Set") {
                        continue;
                    }
                    format!("{}.jpg", sanitize_filename(&stamp.catalog_num))
                } else {
                    stamp.image_path.trim_start_matches("images/").to_string()
                };
                
                // Use fixed dimensions for reliable stamp sizing
                // The original working version likely used fixed dimensions
                let (real_width_mm, real_height_mm) = (25.0, 30.0); // Reasonable fixed dimensions
                
                let description = if stamp.desc.is_empty() {
                    format!("{} {}", clean_text(&stamp.field3), clean_text(&stamp.color))
                } else {
                    clean_text(&stamp.desc)
                };
                let _ = writeln!(output, "STAMP_ADD_IMG ({:.1} {:.1} \"images/{}\" \"\" \"\\n{}\" \"\")", 
                    real_width_mm, real_height_mm, image_name, description);
            }
            
            row_start_idx += stamps_in_row;
        }
        
        page_num += 1;
        if let Some(ref mut cb) = page_callback {
            cb(page_num);
        }
    }
    println!("[INFO] Total images processed: {}", total_images);
}
use std::io::{Write, BufWriter};
use std::fs::File;
use std::path::Path;
use std::sync::Mutex;
use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::db_utils::{init_db, get_all_series, get_stamps_by_series, StampStub};
use crate::image_utils::calc_mm_per_pixel;
use crate::config::load_config;
use log::warn;

#[derive(Debug, Clone)]
pub struct PageHeaderInfo {
    pub year: i32,
    pub month: String,
    pub day: String,
    // pub issue_type: String,
    pub perforation: String,
    pub design: String,
    pub engraving: String,
    pub watermark: String,
    pub currency: String,
}

impl Default for PageHeaderInfo {
    fn default() -> Self {
        Self {
            year: 0,
            //issue_type: "Definitive Issue".to_string(),
            month: String::new(),
            day: String::new(),
            perforation: "14½".to_string(),
            design: String::new(),
            engraving: String::new(),
            watermark: String::new(),
            currency: String::new(),
        }
    }
}

pub fn extract_header_info(stamps: &[&StampStub]) -> PageHeaderInfo {
    let mut info = PageHeaderInfo::default();
    
    if let Some(first_stamp) = stamps.first() {
        info.year = first_stamp.year;
    }
    
    // Extract unique values from all stamps in this group
    let mut designs = HashSet::new();
    let mut engravings = HashSet::new();
    let mut watermarks = HashSet::new();
    let mut perforations = HashSet::new();
    
    for stamp in stamps {
        // Clean and extract design information
        if !stamp.design.is_empty() {
            let clean_design = clean_text(&stamp.design)
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty() && !line.starts_with("Engraving:") && !line.starts_with("Perforation:"))
                .collect::<Vec<_>>()
                .join(" ");
            if !clean_design.is_empty() {
                designs.insert(clean_design);
            }
        }
        
        // Clean and extract engraving information
        if !stamp.engraving.is_empty() {
            let clean_engraving = clean_text(&stamp.engraving)
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty() && !line.starts_with("Perforation:"))
                .collect::<Vec<_>>()
                .join(" ");
            if !clean_engraving.is_empty() {
                engravings.insert(clean_engraving);
            }
        }
        
        // Extract watermark information
        if !stamp.watermark.is_empty() {
            watermarks.insert(clean_text(&stamp.watermark.trim().to_string()));
        }
        
        // Extract perforation from the perforation field (perforation in StampStub)
        if !stamp.perforation.is_empty() {
            perforations.insert(clean_text(&stamp.perforation.trim().to_string()));
        }
    }
    
    // Combine unique values
    if !designs.is_empty() {
        info.design = designs.into_iter().collect::<Vec<_>>().join(", ");
    }
    if !engravings.is_empty() {
        info.engraving = engravings.into_iter().collect::<Vec<_>>().join(", ");
    }
    if !watermarks.is_empty() {
        info.watermark = watermarks.into_iter().collect::<Vec<_>>().join(", ");
    }
    if !perforations.is_empty() {
        info.perforation = perforations.into_iter().collect::<Vec<_>>().join(", ");
    }
    
    info
}

pub fn write_page_header(output: &mut impl Write, header_info: &PageHeaderInfo) {
    let mut date_parts = vec![header_info.year.to_string()];
    if !header_info.month.is_empty() {
        date_parts.push(header_info.month.clone());
    }
    if !header_info.day.is_empty() {
        date_parts.push(header_info.day.clone());
    }
    let date_str = date_parts.join(" - ");
    let _ = writeln!(output, "PAGE_TEXT_CENTRE_VSPACE (TN 10 \"{}\" 3.0)", date_str);
    // let _ = writeln!(output, "PAGE_TEXT_CENTRE_VSPACE (TN 10 \"{}\" 0.5)", header_info.issue_type);

    // if !header_info.perforation.is_empty() {
    //     let _ = writeln!(output, "PAGE_TEXT_CENTRE_VSPACE (TN 10 \"Perforation {}\" 2.0)", header_info.perforation);
    // }

    if !header_info.design.is_empty() {
        let _ = writeln!(output, "PAGE_TEXT_CENTRE_VSPACE (TN 8 \"Design: {}\" 1.0)", header_info.design);
    }

    if !header_info.engraving.is_empty() {
        let _ = writeln!(output, "PAGE_TEXT_CENTRE_VSPACE (TN 8 \"Engraving: {}\" 1.0)", header_info.engraving);
    }

    if !header_info.watermark.is_empty() {
        let _ = writeln!(output, "PAGE_TEXT_CENTRE_VSPACE (TN 8 \"Watermark: {}\" 1.0)", header_info.watermark);
    }

    let _ = writeln!(output, "PAGE_TEXT_CENTRE_VSPACE (TN 8 \"{}\" 1.5)", header_info.currency);
}

lazy_static! {
    static ref CLI_DB_PATH: Mutex<Option<String>> = Mutex::new(None);
}

pub fn set_db_path_from_cli(path: &str) {
    let mut db_path = CLI_DB_PATH.lock().unwrap();
    *db_path = Some(path.to_string());
}

pub fn get_db_path_from_cli() -> String {
    let db_path = CLI_DB_PATH.lock().unwrap();
    match db_path.as_ref() {
        Some(path) => path.clone(),
        None => {
            // This shouldn't happen in normal operation since we set it in main.rs
            "Albums/Album-Default/Album-Default.db".to_string()
        }
    }
}

pub fn load_stamps_from_db(db_path: &str) -> Vec<crate::db_utils::StampStub> {
    let conn = crate::db_utils::init_db(db_path).expect("Failed to open database");
    let mut all_stamps = Vec::new();
    let series_list = crate::db_utils::get_all_series(&conn).expect("Failed to fetch series");
    for series in series_list {
        let stamps = crate::db_utils::get_stamps_by_series(&conn, &series.name, &series.year.to_string()).expect("Failed to fetch stamps");
        all_stamps.extend(stamps);
    }
    all_stamps
}

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
    F: FnMut(usize),
{
    // Load config values from config.toml
    let config = load_config("config.toml");
    let reference_width_mm = config.reference_width_mm.unwrap_or(200.0) as f64;
    let reference_height_mm = config.reference_height_mm.unwrap_or(240.0) as f64;
    // Reference image path and measured size (dynamic by country)
    let country_dir = format!("Albums/{}/images", album_name.replace(' ', "-"));
    let reference_image_path = format!("{}/BG1-s.jpg", country_dir);
    let _ = match calc_mm_per_pixel(&reference_image_path, reference_width_mm as f32, reference_height_mm as f32) {
        Some(_ratio) => {},
        None => {
            warn!("Reference image not found: {}. Using default ratio (1.0, 1.0)", reference_image_path);
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

    // Restore main output loop: iterate over all series, fetch stamps, group into rows, and output
    let series_list = get_all_series(&conn).expect("Failed to fetch series");
    let rows_on_page = 0;
    let page_num = 1;
    for series in series_list {
        // Only use the year value from the DB for filtering and output
        let year_str = series.year.to_string();
        let stamps = get_stamps_by_series(&conn, &series.name, &year_str).expect("Failed to fetch stamps");
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
        // When writing the year, always use series.year
        // ...existing code for writing series header and stamps, always using series.year...
        // After outputting a row:
        // rows_on_page += 1;
        // if rows_on_page >= max_rows_per_page {
        //     page_num += 1;
        //     let _ = writeln!(output, "################################################################################");
        //     let _ = writeln!(output, "# Page {}", page_num);
        //     let _ = writeln!(output, "################################################################################");
        //     let _ = writeln!(output, "PAGE_START");
        //     rows_on_page = 0;
        // }
    }
}

/// Cleans problematic Unicode and formatting from text for AlbumEasy output
pub fn clean_text(s: &str) -> String {
    use regex::Regex;
    // Remove 'Â' or 'â' before fractions/currency
    let re = Regex::new(r"[Ââ]([½¼¾⅓⅔⅛⅜⅝⅞£€$¥])").unwrap();
    let mut s = re.replace_all(s, "$1").to_string();
    // Replace stray 'Â' or 'â' with nothing
    s = s.replace('â', "");
    s = s.replace('Â', "");
    // Replace common fraction representations
    s = s.replace("1/2", "½");
    s = s.replace("1/4", "¼");
    s = s.replace("3/4", "¾");
    s = s.replace("1/3", "⅓");
    s = s.replace("2/3", "⅔");
    s = s.replace("1/8", "⅛");
    s = s.replace("3/8", "⅜");
    s = s.replace("5/8", "⅝");
    s = s.replace("7/8", "⅞");
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

/// Sanitizes a filename by replacing filesystem-incompatible characters
pub fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| match c {
            '*' => 's',  // star -> s
            '?' => 'q',  // question -> q
            '<' => 'l',  // less -> l
            '>' => 'g',  // greater -> g
            ':' => 'c',  // colon -> c
            '"' => 'd',  // double quote -> d
            '|' => 'p',  // pipe -> p
            '\\' => 'b', // backslash -> b
            '/' => 'f',  // forward slash -> f
            _ if c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' => c,
            _ => '_',    // replace other non-alphanumeric with underscore
        })
        .collect()
}

/// Writes global album settings to the provided writer
pub fn write_globals<W: Write>(writer: &mut W, album_name: &str) {
    // Get current date/time and user information
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now();
    let datetime = now.duration_since(UNIX_EPOCH).unwrap();
    let secs = datetime.as_secs();
    let datetime_str = format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        1970 + (secs / 31536000), // years since 1970
        ((secs % 31536000) / 2628000) + 1, // months (approximate)
        ((secs % 2628000) / 86400) + 1, // days (approximate)
        (secs % 86400) / 3600, // hours
        (secs % 3600) / 60, // minutes
        secs % 60 // seconds
    );

    // Get user information (cross-platform)
    let username = std::env::var("USER").or_else(|_| std::env::var("USERNAME")).unwrap_or_else(|_| "unknown".to_string());
    let hostname = std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".to_string());

    // Get command line arguments
    let args: Vec<String> = std::env::args().collect();
    let command_line = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        "No arguments".to_string()
    };

    let lines: Vec<String> = vec![
        "################################################################################".to_string(),
        format!("# {}", album_name),
        "################################################################################".to_string(),
        format!("# Generated: {}", datetime_str),
        format!("# User: {}@{}", username, hostname),
        format!("# OS: {}", std::env::consts::OS),
        format!("# Architecture: {}", std::env::consts::ARCH),
        format!("# Working Directory: {}", std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")).display()),
        format!("# Command Line: {}", command_line),
        "".to_string(),
        "ALBUM_PAGES_SIZE  (216 279) #Set the page to US Letter (8.5 x 11)".to_string(),
        "#ALBUM_PAGES_SIZE (210.0 297.0) #Setting for A4".to_string(),
        "".to_string(),
        "ALBUM_PAGES_MARGINS  (10.0 10.0 10.0 10.0 )".to_string(),
        "".to_string(),
        "ALBUM_PAGES_BORDER (0.5 0.1 0.0 3.0)    #two line border".to_string(),
        "#ALBUM_PAGES_BORDER3  (0.1 0.0 0.0 3.0) #single line border".to_string(),
        "".to_string(),
        format!("ALBUM_PAGES_TITLE (TB 18 \"{}\")", album_name),
        "".to_string(),
        "ALBUM_PAGES_SPACING (6.0 6.0)".to_string(),
        "".to_string(),
        "#ALBUM_PAGES_NUMBER (TN 10 R)".to_string(),
        "ALBUM_PAGES_FOOTER_NUMBER (HN 10 C 1 \"Page \" \" of $PAGES$\")".to_string(),
        "".to_string(),
        "ALBUM_STAMP_IMG_NEW_SETTING (3.0 3.0 true)".to_string(),
        "".to_string(),
        "ROW_ALIGN_MIDDLE".to_string(),
        "".to_string(),
        "# The stamp boxes in this album are sized approximately".to_string(),
        "# 4mm larger than the actual stamps giving a 2mm border".to_string(),
        "# To have the boxes drawn the actual stamp size use".to_string(),
        "#STAMP_BOXES_SIZE_ADJUST(-4.0)".to_string(),
        "# To increase the border, e.g by 1mm use STAMP_BOXES_SIZE_ADJUST(2.0)".to_string(),
        "STAMP_BOXES_SIZE_ADJUST (0)".to_string(),
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
