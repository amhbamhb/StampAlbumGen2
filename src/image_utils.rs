//! # Image Utilities Module
//!
//! This module provides image processing utilities for the Stamp Album Generator,
//! including dimension calculation, embedded reference image handling, and
//! mm-per-pixel ratio calculations.
//!
//! ## Features
//!
//! - **Embedded Reference System**: Uses compiled-in reference images to eliminate external dependencies
//! - **Dimension Calculations**: Accurate mm-per-pixel ratio calculations for precise sizing
//! - **Flexible Image Loading**: Support for both file-based and embedded image sources
//! - **Error Handling**: Robust error handling for missing or corrupted images
//!
//! ## Usage
//!
//! ### Using Embedded Reference
//! ```rust
//! use stamp_album_gen2::image_utils::calc_mm_per_pixel_embedded;
//!
//! // Calculate mm-per-pixel ratio from embedded reference
//! if let Some((mm_per_px_w, mm_per_px_h)) = calc_mm_per_pixel_embedded(200.0, 240.0) {
//!     println!("Reference ratio: {:.3} x {:.3} mm/px", mm_per_px_w, mm_per_px_h);
//! }
//! ```
//!
//! ### File-based Image Processing
//! ```rust
//! use stamp_album_gen2::image_utils::{get_image_dimensions, estimate_real_dimensions};
//!
//! if let Some((width_px, height_px)) = get_image_dimensions("stamp.jpg") {
//!     if let Some((width_mm, height_mm)) = estimate_real_dimensions("stamp.jpg", 0.208, 0.107) {
//!         println!("Stamp size: {:.1}mm x {:.1}mm", width_mm, height_mm);
//!     }
//! }
//! ```

use image::{io::Reader as ImageReader, GenericImageView};
use std::path::Path;

// Embedded reference image data - compiled into the binary at build time
static EMBEDDED_REFERENCE_IMAGE: &[u8] = include_bytes!("B4-s.jpg");

/// Calculates mm-per-pixel ratio from embedded reference image
///
/// This function uses the embedded B4-s.jpg reference image (compiled into the binary)
/// to calculate consistent mm-per-pixel ratios across all installations, eliminating
/// the need for external reference files.
///
/// # Arguments
///
/// * `real_width_mm` - Real-world width of the reference in millimeters
/// * `real_height_mm` - Real-world height of the reference in millimeters
///
/// # Returns
///
/// Option containing (mm_per_pixel_width, mm_per_pixel_height) or None if embedded image loading fails
///
/// # Examples
///
/// ```rust
/// // Calculate ratio for 200mm x 240mm reference
/// let ratio = stamp_album_gen2::image_utils::calc_mm_per_pixel_embedded(200.0, 240.0);
/// // Result: Some((0.208, 0.107)) for B4-s.jpg
/// ```
pub fn calc_mm_per_pixel_embedded(real_width_mm: f32, real_height_mm: f32) -> Option<(f32, f32)> {
    // Use embedded reference image directly
    match image::load_from_memory(EMBEDDED_REFERENCE_IMAGE) {
        Ok(img) => {
            let (px_width, px_height) = img.dimensions();
            let mm_per_pixel_w = real_width_mm / px_width as f32;
            let mm_per_pixel_h = real_height_mm / px_height as f32;
            Some((mm_per_pixel_w, mm_per_pixel_h))
        }
        Err(_) => None,
    }
}

/// Estimates real-world dimensions (mm) for an image using mm-per-pixel ratio
///
/// Converts pixel dimensions to real-world millimeters using the provided
/// mm-per-pixel conversion ratios.
///
/// # Arguments
///
/// * `image_path` - Path to the image file
/// * `mm_per_pixel_w` - Millimeters per pixel for width
/// * `mm_per_pixel_h` - Millimeters per pixel for height
///
/// # Returns
///
/// Option containing (width_mm, height_mm) or None if image loading fails
pub fn estimate_real_dimensions(
    image_path: &str,
    mm_per_pixel_w: f32,
    mm_per_pixel_h: f32,
) -> Option<(f32, f32)> {
    if let Some((px_width, px_height)) = get_image_dimensions(image_path) {
        Some((
            px_width as f32 * mm_per_pixel_w,
            px_height as f32 * mm_per_pixel_h,
        ))
    } else {
        None
    }
}

/// Returns (width, height) in pixels for the given image path
///
/// # Arguments
///
/// * `path` - Path to the image file
///
/// # Returns
///
/// Option containing (width, height) in pixels or None if loading fails
pub fn get_image_dimensions<P: AsRef<Path>>(path: P) -> Option<(u32, u32)> {
    match ImageReader::open(path) {
        Ok(reader) => match reader.decode() {
            Ok(img) => Some(img.dimensions()),
            Err(_) => None,
        },
        Err(_) => None,
    }
}
