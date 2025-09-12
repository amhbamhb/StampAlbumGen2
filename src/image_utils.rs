/// Calculates mm-per-pixel ratio from reference image and real-world size
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
pub fn estimate_real_dimensions(image_path: &str, mm_per_pixel_w: f32, mm_per_pixel_h: f32) -> Option<(f32, f32)> {
    if let Some((px_width, px_height)) = get_image_dimensions(image_path) {
        Some((px_width as f32 * mm_per_pixel_w, px_height as f32 * mm_per_pixel_h))
    } else {
        None
    }
}
// Image utilities for StampAlbumGen2
use image::{DynamicImage, io::Reader as ImageReader, GenericImageView};
use std::path::Path;

// Embedded reference image data
static EMBEDDED_REFERENCE_IMAGE: &[u8] = include_bytes!("B4-s.jpg");

pub fn load_image<P: AsRef<Path>>(path: P) -> image::ImageResult<DynamicImage> {
    ImageReader::open(path)?.decode()
}

/// Returns (width, height) in pixels for the given image path
pub fn get_image_dimensions<P: AsRef<Path>>(path: P) -> Option<(u32, u32)> {
    match load_image(&path) {
        Ok(img) => Some(img.dimensions()),
        Err(_) => None,
    }
}

/// Returns (width, height) in pixels for the embedded reference image
pub fn get_embedded_reference_dimensions() -> Option<(u32, u32)> {
    match image::load_from_memory(EMBEDDED_REFERENCE_IMAGE) {
        Ok(img) => Some(img.dimensions()),
        Err(_) => None,
    }
}

/// Loads the embedded reference image
pub fn load_embedded_reference_image() -> Option<DynamicImage> {
    match image::load_from_memory(EMBEDDED_REFERENCE_IMAGE) {
        Ok(img) => Some(img),
        Err(_) => None,
    }
}
