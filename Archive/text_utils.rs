//! # Text Utilities Module
//!
//! This module provides text processing utilities for the StampAlbumGen2 application,
//! including text cleaning, sanitization, and formatting functions.

/// Cleans text by removing extra whitespace and normalizing line breaks
pub fn clean_text(s: &str) -> String {
    s.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>()
        .join(" ")
}

/// Sanitizes filenames by removing or replacing invalid characters
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

/// Converts month name to number string
pub fn month_number(month: &str) -> String {
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