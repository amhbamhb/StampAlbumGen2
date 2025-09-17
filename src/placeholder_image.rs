//! # Placeholder Image Generator
//!
//! This module generates placeholder images for stamps that don't have associated images.

use image::{ImageBuffer, Rgba};
use std::path::Path;

/// Simple function to draw basic text using block characters
fn draw_simple_text(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, text: &str, center_x: i32, center_y: i32, font_size: i32, color: Rgba<u8>) {
    let char_width = font_size / 2;
    let char_height = font_size;
    let total_width = (text.len() as i32) * char_width;
    let start_x = center_x - total_width / 2;
    let start_y = center_y - char_height / 2;

    for (i, ch) in text.chars().enumerate() {
        let char_x = start_x + (i as i32) * char_width;
        draw_simple_char(img, ch, char_x, start_y, char_width, char_height, color);
    }
}

/// Draw a simple character using basic shapes
fn draw_simple_char(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, ch: char, x: i32, y: i32, width: i32, height: i32, color: Rgba<u8>) {
    let width_u32 = img.width() as i32;
    let height_u32 = img.height() as i32;

    match ch.to_ascii_uppercase() {
        'N' => {
            // Draw N shape
            for i in 0..height {
                // Left vertical line
                if x >= 0 && x < width_u32 && y + i >= 0 && y + i < height_u32 {
                    img.put_pixel(x as u32, (y + i) as u32, color);
                }
                // Right vertical line
                if x + width - 1 >= 0 && x + width - 1 < width_u32 && y + i >= 0 && y + i < height_u32 {
                    img.put_pixel((x + width - 1) as u32, (y + i) as u32, color);
                }
            }
            // Diagonal line
            for i in 0..height {
                let diag_x = x + (i * width / height);
                if diag_x >= 0 && diag_x < width_u32 && y + i >= 0 && y + i < height_u32 {
                    img.put_pixel(diag_x as u32, (y + i) as u32, color);
                }
            }
        }
        'O' => {
            // Draw O shape (circle approximation)
            for i in 0..height {
                for j in 0..width {
                    let center_x = x + width / 2;
                    let center_y = y + height / 2;
                    let dx = (j - width / 2) as f32;
                    let dy = (i - height / 2) as f32;
                    let distance = (dx * dx + dy * dy).sqrt();
                    if distance >= (width as f32 / 2.0) - 1.0 && distance <= (width as f32 / 2.0) + 1.0 {
                        let px = center_x + j - width / 2;
                        let py = center_y + i - height / 2;
                        if px >= 0 && px < width_u32 && py >= 0 && py < height_u32 {
                            img.put_pixel(px as u32, py as u32, color);
                        }
                    }
                }
            }
        }
        'I' => {
            // Draw I shape
            for i in 0..height {
                let px = x + width / 2;
                if px >= 0 && px < width_u32 && y + i >= 0 && y + i < height_u32 {
                    img.put_pixel(px as u32, (y + i) as u32, color);
                }
            }
        }
        'M' => {
            // Draw M shape
            for i in 0..height {
                // Left vertical line
                if x >= 0 && x < width_u32 && y + i >= 0 && y + i < height_u32 {
                    img.put_pixel(x as u32, (y + i) as u32, color);
                }
                // Right vertical line
                if x + width - 1 >= 0 && x + width - 1 < width_u32 && y + i >= 0 && y + i < height_u32 {
                    img.put_pixel((x + width - 1) as u32, (y + i) as u32, color);
                }
                // Left diagonal
                let left_diag_x = x + (i * width / (2 * height));
                if left_diag_x >= 0 && left_diag_x < width_u32 && y + i >= 0 && y + i < height_u32 {
                    img.put_pixel(left_diag_x as u32, (y + i) as u32, color);
                }
                // Right diagonal
                let right_diag_x = x + width - 1 - (i * width / (2 * height));
                if right_diag_x >= 0 && right_diag_x < width_u32 && y + i >= 0 && y + i < height_u32 {
                    img.put_pixel(right_diag_x as u32, (y + i) as u32, color);
                }
            }
        }
        'A' => {
            // Draw A shape
            for i in 0..height {
                // Left diagonal
                let left_x = x + (i * width / height);
                if left_x >= 0 && left_x < width_u32 && y + i >= 0 && y + i < height_u32 {
                    img.put_pixel(left_x as u32, (y + i) as u32, color);
                }
                // Right diagonal
                let right_x = x + width - 1 - (i * width / height);
                if right_x >= 0 && right_x < width_u32 && y + i >= 0 && y + i < height_u32 {
                    img.put_pixel(right_x as u32, (y + i) as u32, color);
                }
            }
            // Horizontal bar
            let bar_y = y + height * 3 / 4;
            for j in 0..width {
                let px = x + j;
                if px >= 0 && px < width_u32 && bar_y >= 0 && bar_y < height_u32 {
                    img.put_pixel(px as u32, bar_y as u32, color);
                }
            }
        }
        'G' => {
            // Draw G shape
            for i in 0..height {
                // Top horizontal
                if i == 0 {
                    for j in width/4..width {
                        let px = x + j;
                        if px >= 0 && px < width_u32 && y + i >= 0 && y + i < height_u32 {
                            img.put_pixel(px as u32, (y + i) as u32, color);
                        }
                    }
                }
                // Bottom horizontal
                if i == height - 1 {
                    for j in 0..width {
                        let px = x + j;
                        if px >= 0 && px < width_u32 && y + i >= 0 && y + i < height_u32 {
                            img.put_pixel(px as u32, (y + i) as u32, color);
                        }
                    }
                }
                // Left vertical
                if x >= 0 && x < width_u32 && y + i >= 0 && y + i < height_u32 {
                    img.put_pixel(x as u32, (y + i) as u32, color);
                }
                // Right vertical (top half)
                if i < height / 2 {
                    let px = x + width - 1;
                    if px >= 0 && px < width_u32 && y + i >= 0 && y + i < height_u32 {
                        img.put_pixel(px as u32, (y + i) as u32, color);
                    }
                }
                // Middle horizontal bar
                if i == height / 2 {
                    for j in width/2..width {
                        let px = x + j;
                        if px >= 0 && px < width_u32 && y + i >= 0 && y + i < height_u32 {
                            img.put_pixel(px as u32, (y + i) as u32, color);
                        }
                    }
                }
            }
        }
        'E' => {
            // Draw E shape
            for i in 0..height {
                // Left vertical
                if x >= 0 && x < width_u32 && y + i >= 0 && y + i < height_u32 {
                    img.put_pixel(x as u32, (y + i) as u32, color);
                }
                // Top horizontal
                if i == 0 {
                    for j in 0..width {
                        let px = x + j;
                        if px >= 0 && px < width_u32 && y + i >= 0 && y + i < height_u32 {
                            img.put_pixel(px as u32, (y + i) as u32, color);
                        }
                    }
                }
                // Middle horizontal
                if i == height / 2 {
                    for j in 0..width * 3 / 4 {
                        let px = x + j;
                        if px >= 0 && px < width_u32 && y + i >= 0 && y + i < height_u32 {
                            img.put_pixel(px as u32, (y + i) as u32, color);
                        }
                    }
                }
                // Bottom horizontal
                if i == height - 1 {
                    for j in 0..width {
                        let px = x + j;
                        if px >= 0 && px < width_u32 && y + i >= 0 && y + i < height_u32 {
                            img.put_pixel(px as u32, (y + i) as u32, color);
                        }
                    }
                }
            }
        }
        'V' => {
            // Draw V shape
            for i in 0..height {
                // Left diagonal
                let left_x = x + (i * width / height);
                if left_x >= 0 && left_x < width_u32 && y + i >= 0 && y + i < height_u32 {
                    img.put_pixel(left_x as u32, (y + i) as u32, color);
                }
                // Right diagonal
                let right_x = x + width - 1 - (i * width / height);
                if right_x >= 0 && right_x < width_u32 && y + i >= 0 && y + i < height_u32 {
                    img.put_pixel(right_x as u32, (y + i) as u32, color);
                }
            }
        }
        'L' => {
            // Draw L shape
            for i in 0..height {
                // Left vertical
                if x >= 0 && x < width_u32 && y + i >= 0 && y + i < height_u32 {
                    img.put_pixel(x as u32, (y + i) as u32, color);
                }
                // Bottom horizontal
                if i == height - 1 {
                    for j in 0..width {
                        let px = x + j;
                        if px >= 0 && px < width_u32 && y + i >= 0 && y + i < height_u32 {
                            img.put_pixel(px as u32, (y + i) as u32, color);
                        }
                    }
                }
            }
        }
        'B' => {
            // Draw B shape
            for i in 0..height {
                // Left vertical
                if x >= 0 && x < width_u32 && y + i >= 0 && y + i < height_u32 {
                    img.put_pixel(x as u32, (y + i) as u32, color);
                }
                // Top horizontal
                if i == 0 {
                    for j in 0..width * 3 / 4 {
                        let px = x + j;
                        if px >= 0 && px < width_u32 && y + i >= 0 && y + i < height_u32 {
                            img.put_pixel(px as u32, (y + i) as u32, color);
                        }
                    }
                }
                // Middle horizontal
                if i == height / 2 {
                    for j in 0..width * 3 / 4 {
                        let px = x + j;
                        if px >= 0 && px < width_u32 && y + i >= 0 && y + i < height_u32 {
                            img.put_pixel(px as u32, (y + i) as u32, color);
                        }
                    }
                }
                // Bottom horizontal
                if i == height - 1 {
                    for j in 0..width * 3 / 4 {
                        let px = x + j;
                        if px >= 0 && px < width_u32 && y + i >= 0 && y + i < height_u32 {
                            img.put_pixel(px as u32, (y + i) as u32, color);
                        }
                    }
                }
                // Right curves
                if i > 0 && i < height / 2 {
                    let px = x + width - 1;
                    if px >= 0 && px < width_u32 && y + i >= 0 && y + i < height_u32 {
                        img.put_pixel(px as u32, (y + i) as u32, color);
                    }
                }
                if i > height / 2 && i < height - 1 {
                    let px = x + width - 1;
                    if px >= 0 && px < width_u32 && y + i >= 0 && y + i < height_u32 {
                        img.put_pixel(px as u32, (y + i) as u32, color);
                    }
                }
            }
        }
        ' ' => {
            // Space - do nothing
        }
        _ => {
            // For unknown characters, draw a simple rectangle
            for i in 0..height {
                for j in 0..width {
                    if i == 0 || i == height - 1 || j == 0 || j == width - 1 {
                        let px = x + j;
                        let py = y + i;
                        if px >= 0 && px < width_u32 && py >= 0 && py < height_u32 {
                            img.put_pixel(px as u32, py as u32, color);
                        }
                    }
                }
            }
        }
    }
}

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
pub fn generate_placeholder_image(width: u32, height: u32, output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Create a new image buffer with white background
    let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    // Fill with white background
    for pixel in img.pixels_mut() {
        *pixel = Rgba([255, 255, 255, 255]);
    }

    // Draw a border
    let border_color = Rgba([200, 200, 200, 255]);
    for x in 0..width {
        for y in 0..height {
            if x < 2 || x >= width - 2 || y < 2 || y >= height - 2 {
                img.put_pixel(x, y, border_color);
            }
        }
    }

    // Draw "NO IMAGE" text
    let text_color = Rgba([100, 100, 100, 255]);
    draw_simple_text(&mut img, "NO IMAGE", (width / 2) as i32, (height / 2 - 20) as i32, 24, text_color);

    // Draw "AVAILABLE" text below
    draw_simple_text(&mut img, "AVAILABLE", (width / 2) as i32, (height / 2 + 20) as i32, 16, text_color);

    // Save the image
    img.save(output_path)?;

    Ok(())
}

/// Generates the default placeholder image if it doesn't exist
pub fn ensure_placeholder_image_for_album(album_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let album_dir = Path::new("Albums").join(album_name);
    let placeholder_path = album_dir.join("images").join("no_image_placeholder.png");

    // Check if the placeholder already exists
    if !placeholder_path.exists() {
        // Create images directory if it doesn't exist
        std::fs::create_dir_all(placeholder_path.parent().unwrap())?;

        // Generate the placeholder image
        generate_placeholder_image(200, 150, &placeholder_path)?;
        println!("[INFO] Generated placeholder image: {}", placeholder_path.display());
    }

    // Return relative path from the album directory
    Ok("images/no_image_placeholder.png".to_string())
}

/// Generates the default placeholder image if it doesn't exist
pub fn ensure_placeholder_image() -> Result<String, Box<dyn std::error::Error>> {
    // For backward compatibility, use a default path
    ensure_placeholder_image_for_album("")
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
        let test_path = Path::new("test_placeholder.png");
        let result = generate_placeholder_image(100, 75, test_path);
        assert!(result.is_ok());

        // Check if file was created
        assert!(test_path.exists());

        // Clean up
        std::fs::remove_file(test_path).unwrap();
    }
}