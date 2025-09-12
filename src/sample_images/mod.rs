/// Sample image configurations for different stamp sizes
/// This module defines reference dimensions and scaling factors for various stamp types

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct StampReference {
    pub name: String,
    pub expected_width_mm: f64,
    pub expected_height_mm: f64,
    pub scaling_width: f64,
    pub scaling_height: f64,
}

impl StampReference {
    pub fn new(name: &str, width: f64, height: f64, scale_w: f64, scale_h: f64) -> Self {
        Self {
            name: name.to_string(),
            expected_width_mm: width,
            expected_height_mm: height,
            scaling_width: scale_w,
            scaling_height: scale_h,
        }
    }
}

pub fn get_stamp_references() -> HashMap<String, StampReference> {
    let mut references = HashMap::new();

    // Large stamps (L-series, O-series)
    references.insert("L".to_string(), StampReference::new(
        "Large Registration",
        49.0, 21.0,  // Target dimensions for ALBUM_STAMP_IMG_ASPECT_RATIO (false)
        1.25, 2.45   // Fine-tuned scaling factors
    ));

    // Medium stamps (M-series)
    references.insert("M".to_string(), StampReference::new(
        "Medium Definitive",
        28.0, 24.0,  // Expected actual dimensions
        1.15, 1.45   // Scaling factors
    ));

    // Small stamps (N-series)
    references.insert("N".to_string(), StampReference::new(
        "Small Definitive",
        26.0, 21.0,  // Expected actual dimensions
        1.10, 1.40   // Scaling factors
    ));

    // Extra small stamps
    references.insert("small".to_string(), StampReference::new(
        "Extra Small",
        22.0, 18.0,  // Expected actual dimensions
        1.05, 1.35   // Scaling factors
    ));

    references
}

/// Get scaling factors based on stamp series
pub fn get_scaling_for_series(series: &str) -> (f64, f64) {
    let references = get_stamp_references();

    // Extract series letter from filename (e.g., "L-s.jpg" -> "L")
    let series_key = if series.starts_with('L') || series.starts_with('O') {
        "L"
    } else if series.starts_with('M') {
        "M"
    } else if series.starts_with('N') {
        "N"
    } else {
        "small"
    };

    if let Some(ref reference) = references.get(series_key) {
        (reference.scaling_width, reference.scaling_height)
    } else {
        // Default scaling
        (1.17, 1.49)
    }
}
