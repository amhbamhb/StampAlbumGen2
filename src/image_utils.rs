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
//! use crate::image_utils::calc_mm_per_pixel_embedded;
//!
//! // Calculate mm-per-pixel ratio from embedded reference
//! if let Some((mm_per_px_w, mm_per_px_h)) = calc_mm_per_pixel_embedded(200.0, 240.0) {
//!     println!("Reference ratio: {:.3} x {:.3} mm/px", mm_per_px_w, mm_per_px_h);
//! }
//! ```
//!
//! ### File-based Image Processing
//! ```rust
//! use crate::image_utils::{get_image_dimensions, estimate_real_dimensions};
//!
//! if let Some((width_px, height_px)) = get_image_dimensions("stamp.jpg") {
//!     if let Some((width_mm, height_mm)) = estimate_real_dimensions("stamp.jpg", 0.208, 0.107) {
//!         println!("Stamp size: {:.1}mm x {:.1}mm", width_mm, height_mm);
//!     }
//! }
//! ```

use image::{DynamicImage, io::Reader as ImageReader, GenericImageView};
use std::path::Path;

// Embedded reference image data - compiled into the binary at build time
static EMBEDDED_REFERENCE_IMAGE: &[u8] = include_bytes!("B4-s.jpg");

/// Calculates mm-per-pixel ratio from reference image file and real-world size
///
/// This function loads a reference image from disk and calculates the mm-per-pixel
/// ratio based on the known real-world dimensions of that reference.
///
/// # Arguments
///
/// * `reference_image_path` - Path to the reference image file
/// * `real_width_mm` - Real-world width of the reference in millimeters
/// * `real_height_mm` - Real-world height of the reference in millimeters
///
/// # Returns
///
/// Option containing (mm_per_pixel_width, mm_per_pixel_height) or None if image loading fails
///
/// # Examples
///
/// ```rust
/// let ratio = calc_mm_per_pixel("reference.jpg", 200.0, 240.0);
/// ```
pub fn calc_mm_per_pixel(reference_image_path: &str, real_width_mm: f32, real_height_mm: f32) -> Option<(f32, f32)> {
    if let Some((px_width, px_height)) = get_image_dimensions(reference_image_path) {
        let mm_per_pixel_w = real_width_mm / px_width as f32;
        let mm_per_pixel_h = real_height_mm / px_height as f32;
        Some((mm_per_pixel_w, mm_per_pixel_h))
    } else {
        None
    }
}

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
/// let ratio = calc_mm_per_pixel_embedded(200.0, 240.0);
/// // Result: Some((0.208, 0.107)) for B4-s.jpg
/// ```
pub fn calc_mm_per_pixel_embedded(real_width_mm: f32, real_height_mm: f32) -> Option<(f32, f32)> {
    // Use embedded reference image
    if let Some((px_width, px_height)) = get_embedded_reference_dimensions() {
        let mm_per_pixel_w = real_width_mm / px_width as f32;
        let mm_per_pixel_h = real_height_mm / px_height as f32;
        Some((mm_per_pixel_w, mm_per_pixel_h))
    } else {
        None
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
pub fn estimate_real_dimensions(image_path: &str, mm_per_pixel_w: f32, mm_per_pixel_h: f32) -> Option<(f32, f32)> {
    if let Some((px_width, px_height)) = get_image_dimensions(image_path) {
        Some((px_width as f32 * mm_per_pixel_w, px_height as f32 * mm_per_pixel_h))
    } else {
        None
    }
}

/// Loads an image from the specified file path
///
/// # Arguments
///
/// * `path` - Path to the image file
///
/// # Returns
///
/// Result containing the loaded DynamicImage or an image error
pub fn load_image<P: AsRef<Path>>(path: P) -> image::ImageResult<DynamicImage> {
    ImageReader::open(path)?.decode()
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
    match load_image(&path) {
        Ok(img) => Some(img.dimensions()),
        Err(_) => None,
    }
}

/// Returns (width, height) in pixels for the embedded reference image
///
/// This function loads the embedded B4-s.jpg reference image from memory
/// and returns its pixel dimensions. The embedded image ensures consistent
/// reference dimensions across all installations.
///
/// # Returns
///
/// Option containing (width, height) in pixels or None if loading fails
///
/// # Examples
///
/// ```rust
/// if let Some((width, height)) = get_embedded_reference_dimensions() {
///     println!("Reference image: {}x{} pixels", width, height);
/// }
/// ```
pub fn get_embedded_reference_dimensions() -> Option<(u32, u32)> {
    match image::load_from_memory(EMBEDDED_REFERENCE_IMAGE) {
        Ok(img) => Some(img.dimensions()),
        Err(_) => None,
    }
}

/// Loads the embedded reference image
///
/// Returns the embedded B4-s.jpg reference image as a DynamicImage.
/// This function is used internally for reference dimension calculations
/// and can also be used for debugging or validation purposes.
///
/// # Returns
///
/// Option containing the loaded DynamicImage or None if loading fails
pub fn load_embedded_reference_image() -> Option<DynamicImage> {
    match image::load_from_memory(EMBEDDED_REFERENCE_IMAGE) {
        Ok(img) => Some(img),
        Err(_) => None,
    }
}
