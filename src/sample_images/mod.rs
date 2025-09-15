//! # Sample Images Module
//!
//! This module provides reference dimensions and scaling factors for different stamp series
//! to ensure consistent and accurate stamp sizing in album generation.
//!
//! The module implements a series-specific scaling system that eliminates the need for
//! external reference files by using embedded assets and predefined scaling factors.
//!
//! ## Features
//!
//! - **Series-specific scaling**: Different scaling factors for L/M/N stamp series
//! - **Embedded reference system**: No external file dependencies
//! - **Precise dimension control**: Exact target dimensions for optimal album layout
//! - **Extensible design**: Easy to add new stamp series and scaling factors
//!
//! ## Usage
//!
//! ```rust
//! use crate::sample_images::get_scaling_for_series;
//!
//! // Get scaling factors for L-series stamps
//! let (scale_w, scale_h) = get_scaling_for_series("L-s.jpg");
//! ```
//!
//! ## Scaling Factors
//!
//! | Series | Target Dimensions | Scaling Factors | Purpose |
//! |--------|------------------|-----------------|---------|
//! | L-series | 49.0mm x 21.0mm | (1.25, 2.45) | Large registration stamps |
//! | M-series | 31.1mm x 24.1mm | (1.15, 1.45) | Medium definitive stamps |
//! | N-series | 26.1mm x 21.0mm | (1.10, 1.40) | Small definitive stamps |
//! | R-series | 38.0mm x 25.0mm | (0.97, 1.68) | Standard pre-1970 stamps |
//! | S-series | 38.0mm x 25.0mm | (1.05, 1.80) | Standard pre-1970 stamps |
//! | T-series | 25.0mm x 24.0mm | (1.03, 1.60) | Tax stamps |
//! | U-series | 25.0mm x 24.0mm | (1.04, 1.60) | Utility stamps |
//! | Small | 22.0mm x 18.0mm | (1.05, 1.35) | Extra small stamps |

use std::collections::HashMap;

/// Reference data structure for stamp series configuration
///
/// Contains the target dimensions and scaling factors needed to achieve
/// precise stamp sizing for album generation.
#[derive(Debug, Clone)]
pub struct StampReference {
    /// Human-readable name for the stamp series
    pub name: String,
    /// Target width in millimeters for ALBUM_STAMP_IMG_ASPECT_RATIO (false)
    pub expected_width_mm: f64,
    /// Target height in millimeters
    pub expected_height_mm: f64,
    /// Width scaling factor applied to base dimensions
    pub scaling_width: f64,
    /// Height scaling factor applied to base dimensions
    pub scaling_height: f64,
}

impl StampReference {
    /// Creates a new StampReference with the specified parameters
    ///
    /// # Arguments
    ///
    /// * `name` - Human-readable series name
    /// * `width` - Target width in millimeters
    /// * `height` - Target height in millimeters
    /// * `scale_w` - Width scaling factor
    /// * `scale_h` - Height scaling factor
    ///
    /// # Returns
    ///
    /// A new StampReference instance
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

/// Returns a complete map of stamp series references
///
/// This function defines the reference dimensions and scaling factors for all
/// supported stamp series. The values are carefully calibrated to achieve
/// optimal album layout and visual consistency.
///
/// # Returns
///
/// HashMap containing StampReference entries for each series
pub fn get_stamp_references() -> HashMap<String, StampReference> {
    let mut references = HashMap::new();

    // Large stamps (L-series, O-series) - Registration and large definitive stamps
    references.insert("L".to_string(), StampReference::new(
        "Large Registration",
        49.0, 21.0,  // Target dimensions for ALBUM_STAMP_IMG_ASPECT_RATIO (false)
        1.25, 2.45   // Fine-tuned scaling factors for exact 49mm x 21mm result
    ));

    // Medium stamps (M-series) - Standard definitive stamps
    references.insert("M".to_string(), StampReference::new(
        "Medium Definitive",
        28.0, 24.0,  // Expected actual dimensions
        1.15, 1.45   // Scaling factors for 31.1mm x 24.1mm result
    ));

    // Standard stamps (R-series, S-series) - Pre-1970 definitive stamps
    references.insert("R".to_string(), StampReference::new(
        "Standard Definitive",
        38.0, 25.0,  // Target dimensions for pre-1970 stamps
        0.97, 1.68   // Adjusted scaling factors for 38mm x 25mm result
    ));

    references.insert("S".to_string(), StampReference::new(
        "Standard Definitive",
        38.0, 25.0,  // Target dimensions for pre-1970 stamps
        1.05, 1.80   // Adjusted scaling factors for 38mm x 25mm result
    ));

    // Utility stamps (U-series) - Small utility stamps
    references.insert("U".to_string(), StampReference::new(
        "Utility Stamp",
        25.0, 24.0,  // Target dimensions: 25mm wide x 24mm high (with 2 spaces)
        1.04, 1.60   // Scaling factors for 25mm x 24mm result
    ));

    // Tax stamps (T-series) - Tax/utility stamps
    references.insert("T".to_string(), StampReference::new(
        "Tax Stamp",
        25.0, 24.0,  // Target dimensions: 25mm wide x 24mm high (with 2 spaces)
        1.03, 1.60   // Scaling factors for 25mm x 24mm result
    ));

    // Small stamps (N-series) - Small definitive stamps
    references.insert("N".to_string(), StampReference::new(
        "Small Definitive",
        26.0, 21.0,  // Expected actual dimensions
        1.10, 1.40   // Scaling factors for 26.1mm x 21.0mm result
    ));

    // Extra small stamps - For specialty or commemorative stamps
    references.insert("small".to_string(), StampReference::new(
        "Extra Small",
        22.0, 18.0,  // Expected actual dimensions
        1.05, 1.35   // Conservative scaling factors
    ));

    references
}

/// Determines scaling factors based on stamp series
///
/// Extracts the series identifier from a stamp filename and returns the
/// appropriate scaling factors for dimension calculation.
///
/// # Arguments
///
/// * `series` - Stamp filename or series identifier (e.g., "L-s.jpg", "M-123")
///
/// # Returns
///
/// Tuple of (width_scaling, height_scaling) factors
///
/// # Examples
///
/// ```rust
/// let (w_scale, h_scale) = get_scaling_for_series("L-s.jpg");  // (1.25, 2.45)
/// let (w_scale, h_scale) = get_scaling_for_series("M-456");   // (1.15, 1.45)
/// ```
pub fn get_scaling_for_series(series: &str) -> (f64, f64) {
    let references = get_stamp_references();

    // Extract series letter from filename (e.g., "L-s.jpg" -> "L")
    let series_key = if series.starts_with('L') || series.starts_with('O') {
        "L"
    } else if series.starts_with('M') {
        "M"
    } else if series.starts_with('N') {
        "N"
    } else if series.starts_with('R') {
        "R"
    } else if series.starts_with('S') {
        "S"
    } else if series.starts_with('U') {
        "U"
    } else if series.starts_with('T') {
        "T"
    } else {
        "small"
    };

    if let Some(ref reference) = references.get(series_key) {
        (reference.scaling_width, reference.scaling_height)
    } else {
        // Default scaling for unrecognized series
        (1.17, 1.49)
    }
}
