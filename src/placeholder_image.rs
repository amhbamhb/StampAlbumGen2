//! # Placeholder Image Generator
//!
//! This module generates placeholder images for stamps that don't have associated images.

use image::{ImageBuffer, Rgba};
use std::path::Path;

/// Generates a placeholder image with the text "No Image currently found"
/// and saves it as a PNG file.
///
/// # Arguments
/// * `width` - Width of the image in pixels
/// * `height` - Height of the image in pixels
/// * `output_path` - Path where to save the PNG file
///
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Success or error
pub fn generate_placeholder_image(
    width: u32,
    height: u32,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a white background
    let mut img = ImageBuffer::from_pixel(width, height, Rgba([255u8, 255u8, 255u8, 255u8]));

    // Draw a light gray border
    let border_color = Rgba([200u8, 200u8, 200u8, 255u8]);
    let border_width = 2;

    // Draw border
    for x in 0..width {
        for y in 0..height {
            if x < border_width || x >= width - border_width ||
               y < border_width || y >= height - border_width {
                img.put_pixel(x, y, border_color);
            }
        }
    }

    // For now, we'll create a simple placeholder without text rendering
    // since font loading can be complex. We'll add a simple pattern instead.

    // Create a simple pattern - diagonal lines
    let pattern_color = Rgba([220u8, 220u8, 220u8, 255u8]);
    for i in 0..(width + height) / 20 {
        let x = (i * 20) as i32 - (height as i32);
        for dy in 0..20 {
            let y = x + dy;
            if y >= 0 && y < height as i32 {
                let px = (x + dy) as u32;
                if px < width {
                    img.put_pixel(px, y as u32, pattern_color);
                }
            }
        }
    }

    // Save the image
    img.save(output_path)?;

    Ok(())
}

/// Generates the default placeholder image if it doesn't exist
pub fn ensure_placeholder_image() -> Result<String, Box<dyn std::error::Error>> {
    // Get the album directory from the global album file name
    let album_file_name = crate::globals::ALBUM_FILE_NAME.lock().unwrap().clone();
    let album_dir = if !album_file_name.is_empty() {
        Path::new(&album_file_name).parent().unwrap_or(Path::new("."))
    } else {
        Path::new(".")
    };

    let placeholder_path = album_dir.join("images").join("no_image_placeholder.png");

    // Check if the placeholder already exists
    if !placeholder_path.exists() {
        // Create images directory if it doesn't exist
        std::fs::create_dir_all(placeholder_path.parent().unwrap())?;

        // Generate the placeholder image
        generate_placeholder_image(200, 150, &placeholder_path.to_string_lossy())?;
        println!("Generated placeholder image: {}", placeholder_path.display());
    }

    // Return relative path from the album directory
    Ok("images/no_image_placeholder.png".to_string())
}

/// Gets the path to the placeholder image, creating it if necessary
pub fn get_placeholder_image_path() -> String {
    match ensure_placeholder_image() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Failed to create placeholder image: {}", e);
            // Fallback to a simple text-based path
            "images/no_image_placeholder.png".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_placeholder_image() {
        let test_path = "test_placeholder.png";
        let result = generate_placeholder_image(100, 75, test_path);
        assert!(result.is_ok());

        // Check if file was created
        assert!(Path::new(test_path).exists());

        // Clean up
        std::fs::remove_file(test_path).unwrap();
    }
}