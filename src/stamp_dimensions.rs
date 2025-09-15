//! # Stamp Dimensions Module
//!
//! This module handles stamp dimension calculations and image processing
//! for the StampAlbumGen2 application.

/// Calculates stamp dimensions based on image path and album name
pub fn calculate_stamp_dimensions(image_path: &str, album_name: &str) -> (f64, f64) {
    use std::path::Path;

    // Extract series from image path (e.g., "A-s.jpg" -> "A")
    let path = Path::new(image_path);
    let filename = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    let series = if filename.contains("-s.jpg") {
        filename.split("-s.jpg").next().unwrap_or("")
    } else {
        ""
    };

    // Get base dimensions for the series
    let (base_width, base_height) = get_series_base_dimensions(series);

    // Apply album-specific scaling
    let scale_factor = get_album_scale_factor(album_name);

    (
        base_width * scale_factor,
        base_height * scale_factor
    )
}

/// Gets base dimensions for a stamp series
fn get_series_base_dimensions(series: &str) -> (f64, f64) {
    match series {
        "A" => (25.8, 20.2),
        "B" => (26.2, 20.2),
        "C" => (25.8, 20.2),
        "D" => (26.5, 20.2),
        "E" => (26.2, 20.2),
        "F" => (25.8, 20.2),
        "G" => (26.7, 20.2),
        "H" => (26.7, 20.2),
        "I" => (26.2, 20.2),
        "J" => (25.4, 20.2),
        "K" => (26.7, 20.2),
        "L" => (47.5, 20.2), // Large format
        "M" => (26.2, 20.2),
        _ => (25.0, 20.0), // Default
    }
}

/// Gets scaling factor for album-specific adjustments
fn get_album_scale_factor(album_name: &str) -> f64 {
    match album_name.to_lowercase().as_str() {
        "canada" => 1.0,
        "great-britain" => 0.95,
        _ => 1.0,
    }
}

/// Calculates mm per pixel ratio using embedded reference image
pub fn calc_mm_per_pixel_embedded(reference_width_mm: f32, reference_height_mm: f32) -> Option<(f64, f64)> {
    // This would use an embedded reference image
    // For now, return default ratios
    Some((reference_width_mm as f64 / 200.0, reference_height_mm as f64 / 240.0))
}