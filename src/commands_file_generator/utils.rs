//! Utility functions for the commands file generator.
//!
//! This module contains various utility functions used throughout the
//! commands file generator, including validation, formatting, and helper methods.

use crate::commands_file_generator::image_utils::SeriesInfo;

/// Convert month name to number as string
pub fn month_number(month: &str) -> String {
    // Stub: convert month name to number as string
    match month.to_lowercase().as_str() {
        "january" => "1".to_string(),
        "february" => "2".to_string(),
        "march" => "3".to_string(),
        "april" => "4".to_string(),
        "may" => "5".to_string(),
        "june" => "6".to_string(),
        "july" => "7".to_string(),
        "august" => "8".to_string(),
        "september" => "9".to_string(),
        "october" => "10".to_string(),
        "november" => "11".to_string(),
        "december" => "12".to_string(),
        _ => "0".to_string(),
    }
}

/// Check if the given format field is selected
pub fn is_selected_format(formats: &[String], field: &str) -> bool {
    if formats.is_empty() {
        true
    } else {
        formats.iter().any(|f| f.eq_ignore_ascii_case(field))
    }
}

/// Check if the given emission field is selected
pub fn is_selected_emission(emissions: &[String], field: &str) -> bool {
    if emissions.is_empty() {
        true
    } else {
        emissions.iter().any(|e| e.eq_ignore_ascii_case(field))
    }
}

/// Return a catalog number for a record. Simple stub used by the commands generator.
pub fn get_catalog_num(field: &str, _prefer_full: bool) -> String {
    field.to_string()
}

/// Simple series-info extractor stub. Real implementation inspects neighboring lines.
pub fn get_series_info(_data: &[String], _idx: usize, width: &str, height: &str) -> SeriesInfo {
    SeriesInfo {
        count: 1,
        multi_perf: false,
        width: width.to_string(),
        height: height.to_string(),
    }
}

/// Check if a file is a valid JPEG by examining its magic bytes
pub fn is_valid_jpeg(file_path: &str) -> bool {
    use std::fs::File;
    use std::io::Read;
    let mut valid = true;
    match File::open(file_path) {
        Ok(mut file) => {
            let mut buf = [0u8; 4];
            if file.read_exact(&mut buf).is_err() {
                valid = false;
            } else {
                // Java's -2555936 is 0xFFD8FFE0 (JPEG SOI + JFIF marker)
                let magic = i32::from_be_bytes(buf);
                if magic != -2555936 {
                    valid = false;
                }
            }
        }
        Err(_) => {
            valid = false;
        }
    }
    valid
}

/// Fix color string (stub implementation)
pub fn fix_color(color: &str) -> String {
    // Stub: just return the color for now
    color.to_string()
}

/// Sleep for the specified number of milliseconds
pub fn snooze(millis: u64) {
    std::thread::sleep(std::time::Duration::from_millis(millis));
}
