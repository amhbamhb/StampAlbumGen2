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
//! - **Simple implementation**: Direct mapping without complex data structures
//!
//! ## Usage
//!
//! ```rust
//! use stamp_album_gen2::sample_images::get_scaling_for_series;
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
/// let (w_scale, h_scale) = stamp_album_gen2::sample_images::get_scaling_for_series("L-s.jpg");  // (1.25, 2.45)
/// let (w_scale, h_scale) = stamp_album_gen2::sample_images::get_scaling_for_series("M-456");   // (1.15, 1.45)
/// ```
pub fn get_scaling_for_series(series: &str) -> (f64, f64) {
    match series.chars().next() {
        Some('L') | Some('O') => (1.25, 2.45), // L-series: Large registration stamps
        Some('M') => (1.15, 1.45),             // M-series: Medium definitive stamps
        Some('N') => (1.10, 1.40),             // N-series: Small definitive stamps
        Some('R') => (0.97, 1.68),             // R-series: Standard pre-1970 stamps
        Some('S') => (1.05, 1.80),             // S-series: Standard pre-1970 stamps
        Some('U') => (1.04, 1.60),             // U-series: Utility stamps
        Some('T') => (1.03, 1.60),             // T-series: Tax stamps
        _ => (1.17, 1.49),                     // Default: Extra small stamps
    }
}
