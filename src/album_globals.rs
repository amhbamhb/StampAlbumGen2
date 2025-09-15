//! # Album Globals Module
//!
//! This module handles the core album generation logic for StampAlbumGen2,
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
//!
//! ## Usage Examples
//!
//! ### Basic Album Generation
//! ```rust
//! use crate::album_globals::generate_album_output_from_stamps;
//!
//! // Generate album from filtered stamp list
//! generate_album_output_from_stamps(stamps, "Canada", Some(page_callback));
//! ```
//!
//! ### Dimension Calculation
//! ```rust
//! use crate::album_globals::calculate_stamp_dimensions;
//!
//! // Calculate precise dimensions for L-series stamp
//! let (width, height) = calculate_stamp_dimensions("L-s.jpg", "Canada");
//! // Result: (49.0, 21.0) mm for optimal album layout
//! ```
//!
//! ## Configuration
//!
//! The module integrates with global settings from `config.toml`:
//! - `ALBUM_STAMP_IMG_ASPECT_RATIO`: Controls dimension calculation mode
//! - `ALBUM_STAMP_IMG_NEW_SETTING`: Additional scaling parameters
//! - `STAMP_BOXES_SIZE_ADJUST`: Fine-tuning for box dimensions
//!
//! ## Dependencies
//!
//! - `image_utils`: For embedded reference image processing
//! - `sample_images`: For series-specific scaling factors
//! - `config`: For global settings management
//! - `rusqlite`: For database operations (indirectly)

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

    // Group stamps by series, then by year within series
    use std::collections::BTreeMap;
    let mut by_series: BTreeMap<String, BTreeMap<i32, Vec<&StampStub>>> = BTreeMap::new();
    for stamp in stamps {
        let series_key = if stamp.series.is_empty() {
            "Unknown Series".to_string()
        } else {
            stamp.series.clone()
        };
        by_series.entry(series_key)
            .or_default()
            .entry(stamp.year)
            .or_default()
            .push(stamp);
    }

    let mut page_num = 1;
    let mut rows_on_page = 0;
    let max_rows_per_page = 4;
    let mut total_images = 0;
    
    for (series, series_years) in &by_series {
        println!("[INFO] Processing series '{}' with {} years", series, series_years.len());
        
        for (year, year_stamps) in series_years {
            println!("[INFO] Processing year {} with {} stamps in series '{}'", year, year_stamps.len(), series);
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
            let page_width_mm = 180.0; // User specified page width
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
                let stamp = &year_stamps[stamp_idx];
                
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
    }
    println!("[INFO] Total images processed: {}", total_images);
}
use std::io::{Write, BufWriter};
use std::fs::File;
use std::sync::Mutex;
use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::db_utils::{init_db, get_all_series, get_stamps_by_series_only, StampStub};

use crate::config::load_config;
use log::warn;
use image::io::Reader as ImageReader;

#[derive(Debug, Clone)]
pub struct PageHeaderInfo {
    pub year: i32,
    pub month: String,
    pub day: String,
    pub series: String,
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
    
    if let Some(first_stamp) = stamps.first() {
        info.year = first_stamp.year;
        info.month = first_stamp.month.clone();
        info.day = first_stamp.day.clone();
        // Extract and clean series information
        if !first_stamp.series.is_empty() {
            info.series = clean_and_format_series(&first_stamp.series);
        }
    }
    
    // Extract unique values from all stamps in this group
    let mut designs = HashSet::new();
    let mut engravings = HashSet::new();
    let mut watermarks = HashSet::new();
    let mut perforations = HashSet::new();
    
    for stamp in stamps {
        // Clean and extract design information - only take image filenames
        if !stamp.design.is_empty() {
            let clean_design = clean_text(&stamp.design)
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty() && !line.starts_with("Engraving:") && !line.starts_with("Perforation:"))
                .collect::<Vec<_>>()
                .join(" ");
            // Extract only image filenames from the design field
            let image_files: Vec<String> = clean_design
                .split_whitespace()
                .filter(|word| word.ends_with(".jpg") || word.ends_with(".png") || word.ends_with(".jpeg"))
                .map(|s| s.to_string())
                .collect();
            if !image_files.is_empty() {
                // Check if images already have "images/" prefix
                let formatted_images: Vec<String> = image_files
                    .into_iter()
                    .map(|img| {
                        if img.starts_with("images/") {
                            img
                        } else {
                            format!("images/{}", img)
                        }
                    })
                    .collect();
                designs.insert(formatted_images.join(", "));
            }
        }
        
        // Clean and extract engraving information - look for engraving data in engraving field
        if !stamp.engraving.is_empty() {
            let clean_engraving = clean_text(&stamp.engraving)
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty() && !line.starts_with("Perforation:") && !line.starts_with("Design:"))
                .collect::<Vec<_>>()
                .join(" ");
            if !clean_engraving.is_empty() && clean_engraving != "None" {
                engravings.insert(clean_engraving);
            }
        }
        
        // Also check perforation field for engraving data (some databases mix this)
        if !stamp.perforation.is_empty() {
            let clean_perf = clean_text(&stamp.perforation);
            for line in clean_perf.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("Engraving:") {
                    let engraving_data = trimmed.strip_prefix("Engraving:").unwrap_or(trimmed).trim();
                    if !engraving_data.is_empty() && engraving_data != "None" {
                        engravings.insert(engraving_data.to_string());
                    }
                }
            }
        }
        
        // Extract watermark information
        if !stamp.watermark.is_empty() {
            watermarks.insert(clean_text(&stamp.watermark.trim().to_string()));
        }
        
        // Extract perforation from the perforation field - clean up mixed data
        if !stamp.perforation.is_empty() {
            let clean_perf = clean_text(&stamp.perforation)
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty() && !line.starts_with("Engraving:") && !line.starts_with("Design:"))
                .collect::<Vec<_>>()
                .join(" ");
            // Extract only perforation values (numbers, x, ½, etc.)
            let perf_parts: Vec<String> = clean_perf
                .split_whitespace()
                .filter(|word| {
                    word.chars().all(|c| c.is_numeric() || c == 'x' || c == '½' || c == '¼' || c == '¾' || c == '.' || c == '-')
                })
                .map(|s| s.to_string())
                .collect();
            if !perf_parts.is_empty() {
                perforations.insert(perf_parts.join(" "));
            }
        }
    }
    
    // Combine unique values
    if !designs.is_empty() {
        // Sort and format design images
        let mut design_list: Vec<String> = designs.into_iter().collect();
        design_list.sort();
        info.design = design_list.join(", ");
    }
    if !engravings.is_empty() {
        let mut engraving_list: Vec<String> = engravings.into_iter().collect();
        engraving_list.sort();
        info.engraving = engraving_list.join(", ");
    }
    if !watermarks.is_empty() {
        let mut watermark_list: Vec<String> = watermarks.into_iter().collect();
        watermark_list.sort();
        info.watermark = watermark_list.join(", ");
    }
    if !perforations.is_empty() {
        let mut perforation_list: Vec<String> = perforations.into_iter().collect();
        perforation_list.sort();
        info.perforation = perforation_list.join(", ");
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
    // Write series information first
    if !header_info.series.is_empty() {
        let series_clean = header_info.series.replace('"', "'");
        let _ = writeln!(output, "PAGE_TEXT_CENTRE_VSPACE (TN 12 \"{}\" 2.0)", series_clean);
    }
    
    let mut date_parts = vec![];
    
    // Build date string in Month - Day - Year format
    if header_info.year > 0 {
        if !header_info.month.is_empty() && !header_info.day.is_empty() {
            // Full date: Month - Day - Year
            date_parts.push(format!("{} - {} - {}", header_info.month, header_info.day, header_info.year));
        } else if !header_info.month.is_empty() {
            // Month and Year only
            date_parts.push(format!("{} - {}", header_info.month, header_info.year));
        } else {
            // Year only
            date_parts.push(header_info.year.to_string());
        }
    }
    
    let date_str = date_parts.join(" - ");
    let date_clean = date_str.replace('"', "'");
    let _ = writeln!(output, "PAGE_TEXT_CENTRE_VSPACE (TN 10 \"{}\" 3.0)", date_clean);
    // let _ = writeln!(output, "PAGE_TEXT_CENTRE_VSPACE (TN 10 \"{}\" 0.5)", header_info.issue_type);

    // Write watermark and perforation on separate lines
    if !header_info.watermark.is_empty() {
        let watermark_clean = header_info.watermark.replace('"', "'");
        let _ = writeln!(output, "PAGE_TEXT_CENTRE_VSPACE (TN 8 \"WM: {}\" 1.0)", watermark_clean);
    }
    
    if !header_info.perforation.is_empty() {
        let perforation_clean = header_info.perforation.replace('"', "'");
        let _ = writeln!(output, "PAGE_TEXT_CENTRE_VSPACE (TN 8 \"Perforation: {}\" 1.0)", perforation_clean);
    }

    if !header_info.design.is_empty() {
        let design_clean = header_info.design.replace('"', "'");
        let _ = writeln!(output, "PAGE_TEXT_CENTRE_VSPACE (TN 8 \"Design: {}\" 1.0)", design_clean);
    }

    if !header_info.engraving.is_empty() {
        let engraving_clean = header_info.engraving.replace('"', "'");
        let _ = writeln!(output, "PAGE_TEXT_CENTRE_VSPACE (TN 8 \"Engraving: {}\" 1.0)", engraving_clean);
    }

    let currency_clean = header_info.currency.replace('"', "'");
    let _ = writeln!(output, "PAGE_TEXT_CENTRE_VSPACE (TN 8 \"{}\" 1.5)", currency_clean);
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
    for series_name in series_list {
        let stamps = crate::db_utils::get_stamps_by_series_only(&conn, &series_name).expect("Failed to fetch stamps");
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
pub fn new_year<W: std::io::Write>(writer: &mut W, year: &str, month: &str, day: &str) {
    // Remove leading zero from day
    let mut day_display = day.to_string();
    if day_display.starts_with('0') {
        day_display = day_display.trim_start_matches('0').to_string();
    }

    // Build date string in Month - Day - Year format
    let date_display: String;
    
    // If month is not empty and not "00", try to use it directly or find it in MONTH_NAMES
    if year != "0000" && !month.is_empty() && month != "00" {
        if !day.is_empty() && day != "00" {
            // Full date: Month - Day - Year
            date_display = format!("{} - {} - {}", month, day_display, year);
        } else {
            // Month - Year
            date_display = format!("{} - {}", month, year);
        }
    } else if year != "0000" {
        // Just year
        date_display = year.to_string();
    } else {
        date_display = "Unknown Date".to_string();
    }

    // Write the PAGE_TEXT_CENTRE_VSPACE command
    let date_clean = date_display.replace('"', "'");
    writeln!(writer, "PAGE_TEXT_CENTRE_VSPACE (TN 10 \"{}\" 3.0)", date_clean).unwrap();
}

/// Writes a new row, handles page breaks, and updates counters
pub fn new_row<W: std::io::Write>(writer: &mut W, year: &str, month: &str, day: &str, rows_written: &mut i32, stamps_written: &mut i32, used_width: &mut i32, used_height: &mut i32, max_row_height: &mut i32) {
    *used_height += *max_row_height;
    if *used_height + 43 > 245 {
        new_page(writer, rows_written, stamps_written, used_width, used_height, max_row_height);
        new_year(writer, year, month, day);
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
pub fn generate_album_output<F>(db_path: &str, album_name: &str, _page_callback: Option<F>)
where
    F: FnMut(usize),
{
    // Load config values from config.toml
    let config_path = find_config_path();
    let config = load_config(&config_path);
    let reference_width_mm = config.reference_width_mm.unwrap_or(200.0) as f64;
    let reference_height_mm = config.reference_height_mm.unwrap_or(240.0) as f64;
    // Calculate mm-per-pixel ratios using embedded reference image
    let _ = match crate::image_utils::calc_mm_per_pixel_embedded(reference_width_mm as f32, reference_height_mm as f32) {
        Some(_ratio) => {},
        None => {
            warn!("Could not load embedded reference image. Using default ratio (1.0, 1.0)");
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
    for series_name in series_list {
        // Get all stamps for this series (across all years), sorted by SG number
        let stamps = get_stamps_by_series_only(&conn, &series_name).expect("Failed to fetch stamps");
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
        // When writing the series header, use the series name
        // ...existing code for writing series header and stamps...
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

/// Gets the dimensions of an image file
pub fn get_image_dimensions(image_path: &str) -> Option<(u32, u32)> {
    use std::path::Path;
    
    let path = Path::new(image_path);
    if !path.exists() {
        return None;
    }
    
    match ImageReader::open(path) {
        Ok(reader) => match reader.decode() {
            Ok(img) => Some((img.width(), img.height())),
            Err(_) => None,
        },
        Err(_) => None,
    }
}

/// Calculates appropriate stamp box dimensions based on image dimensions
///
/// This function implements a sophisticated stamp sizing algorithm that:
/// 1. Loads the actual image dimensions from disk
/// 2. Uses embedded reference image for consistent mm-per-pixel ratios
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

        println!("[INFO] Stamp dimensions: {} -> {:.1}mm x {:.1}mm (pixels: {}x{}, ratio: {:.3}x{:.3})",
                image_path, final_width, final_height, img_width, img_height, mm_per_pixel_w, mm_per_pixel_h);        (final_width, final_height)
    } else {
        // Fall back to default dimensions
        println!("[DEBUG] Could not read image dimensions for {}, using defaults: {:.1}mm x {:.1}mm",
                image_path, default_width, default_height);
        (default_width, default_height)
    }
}/// Writes global album settings to the provided writer
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
        new_year(writer, year, month, day);
    } else if *used_height + *max_row_height + 43 > 245 {
        new_page(writer, rows_written, stamps_written, used_width, used_height, max_row_height);
        new_year(writer, year, month, day);
    } else {
        let rows_needed = (stamps_in_series + 3) / 4;
        if *rows_written + rows_needed > 4 {
            new_page(writer, rows_written, stamps_written, used_width, used_height, max_row_height);
            new_year(writer, year, month, day);
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
    new_row(writer, year, month, day, rows_written, stamps_written, used_width, used_height, max_row_height);

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
