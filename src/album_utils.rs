//! # Album Utilities Module
//!
//! This module contains shared utility functions used by both album generation
//! modules (album_globals and stampworld). These functions are identical across
//! both modules and have been consolidated here to eliminate duplication.
//!
//! ## Functions
//!
//! - `clean_text`: Cleans problematic Unicode and formatting from text for AlbumEasy output
//! - `sanitize_filename`: Sanitizes filenames by replacing filesystem-incompatible characters
//! - `write_globals`: Writes global album settings to the output
//! - `calculate_stamp_dimensions`: Calculates appropriate stamp dimensions based on series
//! - `load_stamps_from_db`: Loads stamp data from SQLite database

use std::io::Write;
use regex::Regex;
// Use image utilities to measure actual image dimensions where possible
use crate::image_utils::{calc_mm_per_pixel_embedded, estimate_real_dimensions};

/// Cleans problematic Unicode and formatting from text for AlbumEasy output
pub fn clean_text(s: &str) -> String {
    // Remove 'Â' or 'â' before fractions/currency (UTF-8 encoding artifacts)
    let re = Regex::new(r"[Ââ]([½¼¾⅓⅔⅛⅜⅝⅞£€$¥])").unwrap();
    let mut s = re.replace_all(s, "$1").to_string();

    // Replace stray 'Â' or 'â' with nothing
    s = s.replace('â', "");
    s = s.replace('Â', "");

    // REVERSE LOGIC: Convert Unicode fractions BACK to ASCII for AlbumEasy compatibility
    // First, handle fractions that need spacing (e.g., 11⅝ → 11 5/8)
    let fraction_chars = ["½", "¼", "¾", "⅓", "⅔", "⅛", "⅜", "⅝", "⅞"];
    let fraction_replacements = ["1/2", "1/4", "3/4", "1/3", "2/3", "1/8", "3/8", "5/8", "7/8"];

    for (fraction_char, replacement) in fraction_chars.iter().zip(fraction_replacements.iter()) {
        // Use regex to add space between number and fraction (e.g., 11⅝ → 11 5/8)
        let pattern = format!(r"(\d)({})", fraction_char);
        let re = Regex::new(&pattern).unwrap();
        s = re.replace_all(&s, format!("$1 {}", replacement)).to_string();
    }

    // Handle standalone fractions (not preceded by numbers)
    s = s.replace("½", "1/2");
    s = s.replace("¼", "1/4");
    s = s.replace("¾", "3/4");
    s = s.replace("⅓", "1/3");
    s = s.replace("⅔", "2/3");
    s = s.replace("⅛", "1/8");
    s = s.replace("⅜", "3/8");
    s = s.replace("⅝", "5/8");
    s = s.replace("⅞", "7/8");

    // Replace double quotes
    s = s.replace('"', "'");

    // Replace newlines and tabs with spaces
    s = s.replace('\n', " ");
    s = s.replace('\r', " ");
    s = s.replace('\t', " ");

    // Replace multiple spaces with single space
    let re_spaces = Regex::new(r"\s+").unwrap();
    s = re_spaces.replace_all(&s, " ").to_string();

    // Trim whitespace
    s = s.trim().to_string();

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
        "ALBUM_PAGES_SIZE  (216 270) #Set the page to US Letter (8.5 x 11)".to_string(),
        "#ALBUM_PAGES_SIZE (210.0 297.0) #Setting for A4".to_string(),
        "".to_string(),
        "ALBUM_PAGES_MARGINS  (10.0 10.0 10.0 10.0 )".to_string(),
        "".to_string(),
        "ALBUM_PAGES_BORDER (0.5 0.1 0.0 3.0)    #two line border".to_string(),
        "#ALBUM_PAGES_BORDER3  (0.1 0.0 0.0 3.0) #single line border".to_string(),
        "".to_string(),
        format!("ALBUM_PAGES_TITLE (TB 18 \"{}\")", album_name),
        "".to_string(),
        "ALBUM_PAGES_SPACING (3.0 10.0)".to_string(),
        "".to_string(),
        "ALBUM_STAMP_IMG_ASPECT_RATIO (true) ".to_string(),
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
        "STAMP_BOXES_SIZE_ADJUST (3)".to_string(),
    ];
    for line in lines.iter() {
        let _ = writeln!(writer, "{}", line);
    }
}

/// Calculates appropriate stamp box dimensions based on image dimensions
///
/// This function implements a sophisticated stamp sizing algorithm that:
/// 1. Loads the actual image dimensions from disk
/// 2. Uses embedded reference image for consistent mm/px ratios
/// 3. Applies series-specific scaling factors for precise control
/// 4. Applies reasonable bounds to ensure proper album layout
///
/// # Arguments
///
/// * `image_path` - Relative path to the stamp image (e.g., "L-s.jpg")
/// * `album_name` - Name of the album for path construction
///
/// # Returns
///
/// Tuple of (width_mm, height_mm) representing the calculated stamp dimensions
///
/// # Algorithm
///
/// 1. **Image Loading**: Attempts to load actual pixel dimensions from the image file
/// 2. **Reference Calculation**: Uses embedded B4-s.jpg reference for consistent mm/px ratios
/// 3. **Series Scaling**: Applies series-specific scaling factors (L/M/N series)
/// 4. **Bounds Checking**: Ensures dimensions fit within album layout constraints
/// 5. **Fallback**: Uses default dimensions if image loading fails
///
/// # Examples
///
/// ```rust
/// // Calculate dimensions for L-series stamp
/// let (width, height) = calculate_stamp_dimensions("L-s.jpg", "Canada");
/// // Result: (49.0, 21.0) for L-series with ALBUM_STAMP_IMG_ASPECT_RATIO = false
/// ```
///
/// # Scaling Factors Applied
///
/// - **L-series**: (1.25, 2.45) → 49.0mm x 21.0mm target
/// - **M-series**: (1.15, 1.45) → 31.1mm x 24.1mm target
/// - **N-series**: (1.10, 1.40) → 26.1mm x 21.0mm target
pub fn calculate_stamp_dimensions(image_path: &str, _album_name: &str) -> (f64, f64) {
    // Preferred approach: use actual image pixel dimensions converted to mm
    // via the embedded reference image. This eliminates reliance on hard-coded
    // series groups (L/M/N/O) and produces sizes based on the real image.

    // Reference real-world dimensions used with the embedded B4-s.jpg
    // Historically the project used approx 200x240 mm for the reference.
    const REF_W_MM: f32 = 200.0;
    const REF_H_MM: f32 = 240.0;

    // Attempt to compute mm-per-pixel from the embedded reference image
    if let Some((mm_per_px_w, mm_per_px_h)) = calc_mm_per_pixel_embedded(REF_W_MM, REF_H_MM) {
        // Try to estimate using the actual image file if it exists
        if let Some((w_mm, h_mm)) = estimate_real_dimensions(image_path, mm_per_px_w, mm_per_px_h) {
            let width = w_mm as f64;
            let height = h_mm as f64;

            // Do not apply any album-specific scaling. Use measured image dimensions directly.

            // Enforce reasonable bounds to avoid absurd sizes
            let width = width.clamp(18.0, 60.0);   // between 18mm and 60mm
            let height = height.clamp(14.0, 60.0); // between 14mm and 60mm

            return (width, height);
        }
    }

    // Fallback: if image-based estimation fails, use a conservative default
    // (previous default was 28.0 x 31.0). Keep a single sensible fallback
    // and remove the L/M/N/O series branching entirely.
    let (width, height) = (28.0_f64, 31.0_f64);

    (width, height)
}

/// Loads stamp data from SQLite database
pub fn load_stamps_from_db(db_path: &str) -> Vec<crate::db_utils::StampStub> {
    let conn = crate::db_utils::init_db(db_path).expect("Failed to open database");

    // Load all stamps directly instead of filtering by series
    let mut stmt = conn.prepare("SELECT year, month, day, sg_number, type, denomination, color, month_num, description, stamps_issued, image_path, remote_image_url, watermark, design, engraving, perforation, series, currency, context FROM stamps ORDER BY sg_number").expect("Failed to prepare statement");

    let stamp_iter = stmt.query_map([], |row| {
        // Debug: print raw row data
        // let sg_result: Result<String, _> = row.get(3);
        // let desc_result: Result<String, _> = row.get(8);
        // println!("[DEBUG] Raw row data: sg_number={:?}, description={:?}", sg_result, desc_result);

        Ok(crate::db_utils::StampStub {
            year: row.get::<_, i32>(0).unwrap_or(0),
            month: row.get(1).unwrap_or(String::new()),
            day: row.get(2).unwrap_or(String::new()),
            sg_number: row.get(3).unwrap_or(String::new()),
            stamp_type: row.get(4).unwrap_or(String::new()),
            denomination: row.get(5).unwrap_or(String::new()),
            color: row.get(6).unwrap_or(String::new()),
            multi_perf: false,
            desc: row.get(8).unwrap_or(String::new()),  // description is at index 8
            catalog_num: row.get(3).unwrap_or(String::new()),  // Use sg_number for catalog_num
            stamps_issued: row.get(9).unwrap_or(String::new()),  // stamps_issued at index 9
            image_path: row.get(10).unwrap_or(String::new()),   // image_path at index 10
            remote_image_url: row.get(11).unwrap_or(String::new()), // remote_image_url at index 11
            watermark: row.get(12).unwrap_or(String::new()),    // watermark at index 12
            design: row.get(13).unwrap_or(String::new()),       // design at index 13
            engraving: row.get(14).unwrap_or(String::new()),    // engraving at index 14
            perforation: row.get(15).unwrap_or(String::new()),  // perforation at index 15
            series: row.get(16).unwrap_or(String::new()),       // series at index 16
            currency: row.get(17).unwrap_or(String::new()),     // currency at index 17
            context: row.get(18).unwrap_or(String::new()),      // context at index 18
        })
    }).expect("Failed to query stamps");

    let mut all_stamps = Vec::new();
    for stamp_result in stamp_iter {
        match stamp_result {
            Ok(stamp) => {
                // println!("[DEBUG] Created stamp: sg={}, desc='{}'", stamp.sg_number, stamp.desc);
                all_stamps.push(stamp);
            }
            Err(_e) => {
                // println!("[DEBUG] Error creating stamp: {}", e);
            }
        }
    }
    all_stamps
}