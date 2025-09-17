//! # StampWorld Album Generator Module
//!
//! This module contains the core album generation logic for StampWorld data,
//! including stamp dimension calculations, AlbumEasy output generation, and
//! global album settings management.
//!
//! ## Overview
//!
//! The album generation process involves:
//! 1. **Data Processing**: Loading and filtering stamp data from SQLite databases
//! 2. **Dimension Calculation**: Computing precise stamp dimensions using embedded references
//! 3. **Layout Optimization**: Arranging stamps within 180mm page width constraints
//! 4. **Output Generation**: Creating AlbumEasy-compatible output files
//!
//! ## Key Components
//!
//! ### Stamp Dimension System
//! - **Embedded Reference**: Uses compiled-in B4-s.jpg for consistent mm/px ratios
//! - **Series-Specific Scaling**: Different scaling factors for L/M/N stamp series
//! - **Bounds Checking**: Ensures dimensions fit album layout requirements
//! - **Fallback Handling**: Graceful degradation when images are unavailable
//!
//! ### AlbumEasy Output Format
//! - **Global Settings**: Album metadata and configuration
//! - **Page Layout**: Optimized stamp arrangement within page constraints
//! - **Header Information**: Year-based grouping and section headers
//! - **Stamp Positioning**: Precise coordinate calculations for each stamp

use std::io::{Write, BufWriter};
use std::fs::File;
use std::sync::Mutex;
use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::db_utils::StampStub;
use crate::config::load_config;
use log::warn;
use crate::image_utils::get_image_dimensions;
use crate::stampworld::state::*;

/// Generates AlbumEasy output from a filtered list of stamps and album name, without querying the database.
/// This function takes a pre-filtered list of stamps and generates the complete AlbumEasy output file.
pub fn generate_album_output_from_stamps<F>(stamps: &[StampStub], album_name: &str, mut page_callback: Option<F>)
where
    F: FnMut(usize),
{
    use std::path::Path;

    // Load config values from config.toml for mm_per_pixel calculation
    let config_path = find_config_path();
    let config = load_config(&config_path);
    let reference_width_mm = config.reference_width_mm.unwrap_or(200.0) as f64;
    let reference_height_mm = config.reference_height_mm.unwrap_or(240.0) as f64;

    // Get the correct source directory from the database path
    let _db_path = get_db_path_from_cli();

    // Calculate mm-per-pixel ratios using embedded reference image
    let _ = match crate::image_utils::calc_mm_per_pixel_embedded(reference_width_mm as f32, reference_height_mm as f32) {
        Some(_ratio) => {},
        None => {
            warn!("Could not load embedded reference image. Using default ratio (1.0, 1.0)");
        }
    };

    let output_path = if !ALBUM_FILE_NAME.lock().unwrap().is_empty() {
        Path::new(&*ALBUM_FILE_NAME.lock().unwrap()).to_path_buf()
    } else {
        Path::new("Albums").join(album_name.replace(' ', "-")).join(format!("{}.txt", album_name.replace(' ', "-")))
    };
    let mut output = BufWriter::new(File::create(&output_path).expect("Failed to create output file"));

    // Write global album settings
    write_globals(&mut output, album_name);

    // Group stamps by series only (not by year within series)
    use std::collections::BTreeMap;
    let mut by_series: BTreeMap<String, Vec<&StampStub>> = BTreeMap::new();
    for stamp in stamps {
        let series_key = if stamp.series.is_empty() {
            "Unknown Series".to_string()
        } else {
            stamp.series.clone()
        };
        by_series.entry(series_key)
            .or_default()
            .push(stamp);
    }

    let mut page_num = 1;
    let mut rows_on_page = 0;
    let max_rows_per_page = 4;
    let mut total_images = 0;

    for (series, series_stamps) in &by_series {
        println!("[INFO] Processing series '{}' with {} stamps", series, series_stamps.len());
        total_images += series_stamps.len();
        
        // Extract header information from all stamps in this series
        let header_info = extract_header_info(series_stamps);

        // Check if we need a new page for this series
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

        // Start new page for each series
        let _ = writeln!(output, "################################################################################");
        let _ = writeln!(output, "# Page {}", page_num);
        let _ = writeln!(output, "################################################################################");
        let _ = writeln!(output, "PAGE_START");

        // Write enhanced header with extracted information
        write_page_header(&mut output, &header_info);

        // Group stamps into rows dynamically based on image widths (up to 5 stamps per row)
        let mut row_start_idx = 0;
        while row_start_idx < series_stamps.len() {
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
            let page_width_mm = 180.0; // User specified page width
            let left_margin = 18.0;    // Left margin in mm
            let right_margin = 18.0;  // Right margin in mm
            let stamp_spacing = 3.0;  // Spacing between stamps in mm
            let border_width = 2.0;   // Border width on each side in mm
            let usable_width = page_width_mm - left_margin - right_margin - (2.0 * border_width);

            for i in row_start_idx..std::cmp::min(row_start_idx + max_stamps_per_row, series_stamps.len()) {
                let stamp = &series_stamps[i];
                let image_name = if stamp.image_path.is_empty() {
                    if stamp.desc.contains("Set") {
                        continue;
                    }
                    format!("{}.jpg", sanitize_filename(&stamp.catalog_num))
                } else {
                    stamp.image_path.trim_start_matches("images/").to_string()
                };

                let (stamp_width, _) = calculate_stamp_dimensions(&image_name, album_name);

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
                let stamp = &series_stamps[stamp_idx];

                let image_name = if stamp.image_path.is_empty() {
                    if stamp.desc.contains("Set") {
                        continue;
                    }
                    format!("{}.jpg", sanitize_filename(&stamp.catalog_num))
                } else {
                    stamp.image_path.trim_start_matches("images/").to_string()
                };

                // Calculate stamp dimensions based on actual image size
                let (real_width_mm, real_height_mm) = calculate_stamp_dimensions(&image_name, album_name);

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

        rows_on_page += 1;
        page_num += 1;
        if let Some(ref mut cb) = page_callback {
            cb(page_num);
        }
    }
    println!("[INFO] Total images processed: {}", total_images);
}

#[derive(Debug, Clone)]
pub struct PageHeaderInfo {
    pub year: i32,
    pub month: String,
    pub day: String,
    pub series: String,
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
            month: String::new(),
            day: String::new(),
            series: String::new(),
            perforation: "14½".to_string(),
            design: String::new(),
            engraving: String::new(),
            watermark: String::new(),
            currency: String::new(),
        }
    }
}

fn find_config_path() -> String {
    use std::env;
    use std::path::Path;

    // First try current directory
    if Path::new("config.toml").exists() {
        return "config.toml".to_string();
    }

    // Try to find it relative to the executable
    if let Ok(exe_path) = env::current_exe() {
        if let Some(parent) = exe_path.parent() {
            // Go up two levels from target/release to the workspace root
            if let Some(workspace_root) = parent.parent().and_then(|p| p.parent()) {
                let config_path = workspace_root.join("config.toml");
                if config_path.exists() {
                    return config_path.to_string_lossy().to_string();
                }
            }
        }
    }

    // Fallback to current directory
    "config.toml".to_string()
}

pub fn extract_header_info(stamps: &[&StampStub]) -> PageHeaderInfo {
    let mut info = PageHeaderInfo::default();
    
    if stamps.is_empty() {
        return info;
    }
    
    // Use the first stamp for series information
    info.series = stamps[0].series.clone();
    
    // Collect all unique values for each field across all stamps in the series
    let mut engravings = HashSet::new();
    let mut designs = HashSet::new();
    let mut perforations = HashSet::new();
    let mut watermarks = HashSet::new();
    let mut currencies = HashSet::new();
    let mut years = HashSet::new();
    let mut months = HashSet::new();
    let mut days = HashSet::new();
    
    for stamp in stamps {
        if !stamp.engraving.is_empty() {
            engravings.insert(stamp.engraving.clone());
        }
        if !stamp.design.is_empty() {
            designs.insert(stamp.design.clone());
        }
        if !stamp.perforation.is_empty() {
            perforations.insert(stamp.perforation.clone());
        }
        if !stamp.watermark.is_empty() {
            watermarks.insert(stamp.watermark.clone());
        }
        if !stamp.currency.is_empty() {
            currencies.insert(stamp.currency.clone());
        }
        if stamp.year > 0 {
            years.insert(stamp.year);
        }
        if !stamp.month.is_empty() {
            months.insert(stamp.month.clone());
        }
        if !stamp.day.is_empty() {
            days.insert(stamp.day.clone());
        }
        
        // Enhanced parsing: extract design and engraving from context field if database fields are empty
        if stamp.design.is_empty() || stamp.engraving.is_empty() {
            if !stamp.context.is_empty() {
                let context_lower = stamp.context.to_lowercase();
                
                // Parse design information from context
                if stamp.design.is_empty() {
                    if context_lower.contains("design:") {
                        let design_start = context_lower.find("design:").unwrap();
                        let design_part = &stamp.context[design_start + 7..];
                        let design_value = design_part
                            .split(|c: char| !c.is_alphanumeric() && c != ' ' && c != ',' && c != '-' && c != '.')
                            .next()
                            .unwrap_or("")
                            .trim();
                        if !design_value.is_empty() {
                            designs.insert(design_value.to_string());
                        }
                    }
                }
                
                // Parse engraving information from context
                if stamp.engraving.is_empty() {
                    if context_lower.contains("engraving:") {
                        let engraving_start = context_lower.find("engraving:").unwrap();
                        let engraving_part = &stamp.context[engraving_start + 10..];
                        let engraving_value = engraving_part
                            .split(|c: char| !c.is_alphanumeric() && c != ' ' && c != ',' && c != '-' && c != '.')
                            .next()
                            .unwrap_or("")
                            .trim();
                        if !engraving_value.is_empty() {
                            engravings.insert(engraving_value.to_string());
                        }
                    }
                }
            }
        }
    }
    
    // Join unique values with commas
    info.engraving = engravings.into_iter().collect::<Vec<_>>().join(", ");
    info.design = designs.into_iter().collect::<Vec<_>>().join(", ");
    info.perforation = perforations.into_iter().collect::<Vec<_>>().join(", ");
    info.watermark = watermarks.into_iter().collect::<Vec<_>>().join(", ");
    info.currency = currencies.into_iter().collect::<Vec<_>>().join(", ");
    
    // Set date information - use the earliest year if multiple, and collect unique months/days
    if !years.is_empty() {
        info.year = *years.iter().min().unwrap();
    }
    if !months.is_empty() {
        info.month = months.into_iter().collect::<Vec<_>>().join(", ");
    }
    if !days.is_empty() {
        info.day = days.into_iter().collect::<Vec<_>>().join(", ");
    }
    
    info
}

/// Clean and format series text to fit on 1-2 lines
fn clean_and_format_series(series: &str) -> String {
    let cleaned = clean_text(series);
    let lines: Vec<&str> = cleaned.lines().collect();

    if lines.is_empty() {
        return "Unknown Series".to_string();
    }

    // If single line and not too long, return as is
    if lines.len() == 1 && lines[0].len() <= 60 {
        return lines[0].to_string();
    }

    // Combine multiple lines intelligently, using \n for AlbumEasy compatibility
    let mut result = String::new();
    let mut current_line = String::new();

    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // If adding this line would make the current line too long, start a new line
        if !current_line.is_empty() && current_line.len() + trimmed.len() + 2 > 60 {
            if !result.is_empty() {
                result.push_str("\\n");
            }
            result.push_str(&current_line);
            current_line = trimmed.to_string();
        } else if !current_line.is_empty() {
            current_line.push_str(" - ");
            current_line.push_str(trimmed);
        } else {
            current_line = trimmed.to_string();
        }
    }

    if !current_line.is_empty() {
        if !result.is_empty() {
            result.push_str("\\n");
        }
        result.push_str(&current_line);
    }

    if result.is_empty() {
        "Unknown Series".to_string()
    } else {
        result
    }
}

pub fn write_page_header(output: &mut impl Write, header_info: &PageHeaderInfo) {
    // Write series information first (bold)
    if !header_info.series.is_empty() {
        let series_clean = header_info.series.replace('"', "'");
        let _ = writeln!(output, "PAGE_TEXT_CENTRE (TB 14 \"{}\" 2.0)", series_clean);
    }
    
    // Write date information
    let mut date_parts = vec![];
    
    // Build date string in Month Day, Year format
    if header_info.year > 0 {
        if !header_info.month.is_empty() && !header_info.day.is_empty() {
            // Full date: Month Day, Year
            date_parts.push(format!("{} {}, {}", header_info.month, header_info.day, header_info.year));
        } else if !header_info.month.is_empty() {
            // Month and Year only
            date_parts.push(format!("{} {}", header_info.month, header_info.year));
        } else {
            // Year only
            date_parts.push(header_info.year.to_string());
        }
    }
    
    if !date_parts.is_empty() {
        let date_str = date_parts.join(" - ");
        let date_clean = date_str.replace('"', "'");
        let _ = writeln!(output, "PAGE_TEXT_CENTRE (TN 10 \"Date: {}\" 1.5)", date_clean);
    }
    
    // Write perforation
    if !header_info.perforation.is_empty() {
        let perforation_clean = header_info.perforation.replace('"', "'");
        let _ = writeln!(output, "PAGE_TEXT_CENTRE (TN 8 \"Perforation: {}\" 1.0)", perforation_clean);
    }
    
    // Write watermark (use "Watermark" instead of "WM")
    if !header_info.watermark.is_empty() {
        let watermark_clean = header_info.watermark.replace('"', "'");
        let _ = writeln!(output, "PAGE_TEXT_CENTRE (TN 8 \"Watermark: {}\" 1.0)", watermark_clean);
    }
    
    // Write design
    if !header_info.design.is_empty() {
        let design_clean = header_info.design.replace('"', "'");
        let _ = writeln!(output, "PAGE_TEXT_CENTRE (TN 8 \"Design: {}\" 1.0)", design_clean);
    }
    
    // Write engraving
    if !header_info.engraving.is_empty() {
        let engraving_clean = header_info.engraving.replace('"', "'");
        let _ = writeln!(output, "PAGE_TEXT_CENTRE (TN 8 \"Engraving: {}\" 1.0)", engraving_clean);
    }
    
    // Write currency last
    if !header_info.currency.is_empty() {
        let currency_clean = header_info.currency.replace('"', "'");
        let _ = writeln!(output, "PAGE_TEXT_CENTRE (TN 8 \"Currency: {}\" 1.5)", currency_clean);
    }
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

/// Calculates appropriate stamp box dimensions based on image dimensions
pub fn calculate_stamp_dimensions(image_path: &str, album_name: &str) -> (f64, f64) {
    // Default dimensions if image can't be read
    let default_width = 25.0;
    let default_height = 30.0;

    // Try to get actual image dimensions
    let image_full_path = format!("Albums/{}/images/{}", album_name.replace(' ', "-"), image_path);

    if let Some((img_width, img_height)) = get_image_dimensions(&image_full_path) {
        // Use reference-based sizing approach from older code

        // Use reference dimensions - these should be configurable
        let reference_width_mm = 25.0;  // Reduced from 50.0 to decrease mm-per-pixel ratio
        let reference_height_mm = 15.0; // Reduced from 25.0 to decrease mm-per-pixel ratio

        // Calculate mm-per-pixel ratios using embedded reference image
        let (mm_per_pixel_w, mm_per_pixel_h) = match crate::image_utils::calc_mm_per_pixel_embedded(reference_width_mm as f32, reference_height_mm as f32) {
            Some(ratio) => ratio,
            None => {
                println!("[DEBUG] Could not load embedded reference image. Using default ratio (1.0, 1.0)");
                (1.0, 1.0)
            }
        };

        // Estimate real dimensions using the mm-per-pixel ratios
        let (raw_width_mm, raw_height_mm) = crate::image_utils::estimate_real_dimensions(&image_full_path, mm_per_pixel_w, mm_per_pixel_h)
            .unwrap_or((reference_width_mm as f32, reference_height_mm as f32));

        // Apply series-specific scaling factor based on stamp type
        let (width_scale, height_scale) = crate::sample_images::get_scaling_for_series(image_path);
        let est_width_mm = raw_width_mm as f64 * width_scale;
        let est_height_mm = raw_height_mm as f64 * height_scale;

        // Apply reasonable bounds
        let min_size = 15.0;
        let max_width = 50.0;  // Max width for page layout
        let max_height = 30.0; // Max height for page layout

        let final_width = est_width_mm.max(min_size).min(max_width);
        let final_height = est_height_mm.max(min_size).min(max_height);

        // println!("[INFO] Stamp dimensions: {} -> {:.1}mm x {:.1}mm (pixels: {}x{}, ratio: {:.3}x{:.3})",
                // image_path, final_width, final_height, img_width, img_height, mm_per_pixel_w, mm_per_pixel_h);
        (final_width, final_height)
    } else {
        // Fall back to default dimensions
        println!("[DEBUG] Could not read image dimensions for {}, using defaults: {:.1}mm x {:.1}mm",
                image_path, default_width, default_height);
        (default_width, default_height)
    }
}

pub fn load_stamps_from_db(db_path: &str) -> Vec<crate::db_utils::StampStub> {
    let conn = crate::db_utils::init_db(db_path).expect("Failed to open database");
    let mut all_stamps = Vec::new();
    let series_list = crate::db_utils::get_all_series(&conn).expect("Failed to fetch series");
    for series_name in series_list {
        let stamps = crate::db_utils::get_stamps_by_series_only(&conn, &series_name).expect("Failed to fetch stamps");
        all_stamps.extend(stamps);
    }
    all_stamps
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
        "ALBUM_PAGES_SPACING (3.0 14.0)".to_string(),
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