#![allow(dead_code)]
//! # Placeholder Image Generator
//!
//! This module generates placeholder images for stamps that don't have associated images.

use image::{ImageBuffer, Rgba};
use rusttype::{point, Font, Scale};
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Try to load a TTF font from a list of likely locations (system fonts or project fonts).
/// Returns a loaded `rusttype::Font` on success, or `None` if no font could be found.
fn load_fallback_font(font_override: Option<&str>) -> Option<Font<'static>> {
    // If caller provided override path, try it first
    if let Some(p) = font_override {
        if Path::new(p).exists() {
            if let Ok(mut f) = File::open(p) {
                let mut buf = Vec::new();
                if f.read_to_end(&mut buf).is_ok() {
                    if let Some(font) = Font::try_from_vec(buf) {
                        return Some(font);
                    }
                }
            }
        }
    }
    // Allow user to override font via environment variable if no explicit override provided
    if font_override.is_none() {
        if let Ok(env_path) = std::env::var("STAMP_ALBUM_PLACEHOLDER_FONT") {
            if Path::new(&env_path).exists() {
                if let Ok(mut f) = File::open(&env_path) {
                    let mut buf = Vec::new();
                    if f.read_to_end(&mut buf).is_ok() {
                        if let Some(font) = Font::try_from_vec(buf) {
                            return Some(font);
                        }
                    }
                }
            }
        }
    }
    // Candidate font paths (common Linux, macOS, Windows locations). The project may bundle a font later.
    let candidates = [
        "assets/fonts/DejaVuSans.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
        "/usr/share/fonts/truetype/freefont/FreeSans.ttf",
        "/Library/Fonts/Arial.ttf",
        "C:\\Windows\\Fonts\\arial.ttf",
    ];

    for path in candidates.iter() {
        if Path::new(path).exists() {
            if let Ok(mut f) = File::open(path) {
                let mut buf = Vec::new();
                if f.read_to_end(&mut buf).is_ok() {
                    if let Some(font) = Font::try_from_vec(buf) {
                        return Some(font);
                    }
                }
            }
        }
    }

    None
}

/// Draw anti-aliased text centered at (center_x, center_y) using rusttype.
fn draw_text(
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    text: &str,
    center_x: i32,
    center_y: i32,
    font_size: f32,
    color: Rgba<u8>,
    font_override: Option<&str>,
) {
    // Load font (cached per-call). If missing, fall back to simple rectangle drawing per character.
    let font = match load_fallback_font(font_override) {
        Some(f) => f,
        None => {
            // Fallback: draw a simple rectangle border for each glyph area
            let char_width = (font_size / 2.0) as i32;
            let char_height = font_size as i32;
            let total_width = (text.len() as i32) * char_width;
            let start_x = center_x - total_width / 2;
            let start_y = center_y - char_height / 2;
            for i in 0..text.len() {
                let x = start_x + (i as i32) * char_width;
                for yy in 0..char_height {
                    for xx in 0..char_width {
                        if yy == 0 || yy == char_height - 1 || xx == 0 || xx == char_width - 1 {
                            let px = x + xx;
                            let py = start_y + yy;
                            if px >= 0
                                && py >= 0
                                && (px as u32) < img.width()
                                && (py as u32) < img.height()
                            {
                                img.put_pixel(px as u32, py as u32, color);
                            }
                        }
                    }
                }
            }
            return;
        }
    };

    let scale = Scale::uniform(font_size);

    // Measure text width
    let v_metrics = font.v_metrics(scale);
    let glyphs: Vec<_> = font
        .layout(text, scale, point(0.0, 0.0 + v_metrics.ascent))
        .collect();
    let glyph_width = glyphs
        .iter()
        .rev()
        .filter_map(|g| g.pixel_bounding_box().map(|b| b.max.x as i32))
        .next()
        .unwrap_or(0);
    let total_width = glyph_width;

    // Starting point
    let start_x = center_x as f32 - (total_width as f32) / 2.0;
    let baseline = center_y as f32 + (v_metrics.ascent - v_metrics.descent) / 2.0;

    // Draw each glyph into the image buffer
    for g in glyphs {
        if let Some(bb) = g.pixel_bounding_box() {
            g.draw(|gx, gy, gv| {
                let x = start_x as i32 + bb.min.x + gx as i32;
                let y = baseline as i32 + bb.min.y + gy as i32;
                if x >= 0 && y >= 0 && (x as u32) < img.width() && (y as u32) < img.height() {
                    // Blend gray glyph value into RGBA (simple alpha compositing)
                    let alpha = (gv * 255.0) as u8;
                    let existing = img.get_pixel(x as u32, y as u32);
                    // Foreground color premultiplied blending
                    let fg_r = color[0] as u32;
                    let fg_g = color[1] as u32;
                    let fg_b = color[2] as u32;
                    let fg_a = alpha as u32;

                    let bg_r = existing[0] as u32;
                    let bg_g = existing[1] as u32;
                    let bg_b = existing[2] as u32;

                    // Simple over operator (assuming bg alpha 255)
                    let out_a = 255u32; // keep opaque
                    let out_r = ((fg_r * fg_a + bg_r * (255 - fg_a)) / 255) as u8;
                    let out_g = ((fg_g * fg_a + bg_g * (255 - fg_a)) / 255) as u8;
                    let out_b = ((fg_b * fg_a + bg_b * (255 - fg_a)) / 255) as u8;

                    img.put_pixel(x as u32, y as u32, Rgba([out_r, out_g, out_b, out_a as u8]));
                }
            });
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
// ...existing code...

/// Options to customize placeholder image rendering
pub struct PlaceholderOptions<'a> {
    pub font_path: Option<&'a str>,
    pub title_size: f32,
    pub subtitle_size: f32,
    pub color: Rgba<u8>,
    pub title: Option<&'a str>,
    pub subtitle: Option<&'a str>,
}

/// New API: generate with options. Keeps previous API behavior by calling this with None.
pub fn generate_placeholder_image_with_options(
    width: u32,
    height: u32,
    output_path: &Path,
    options: Option<PlaceholderOptions>,
) -> Result<(), Box<dyn std::error::Error>> {
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

    // Get options or defaults
    let opts = options.unwrap_or(PlaceholderOptions {
        font_path: None,
        title_size: 28.0,
        subtitle_size: 16.0,
        color: Rgba([80, 80, 80, 255]),
        title: None,
        subtitle: None,
    });

    // Draw centered, clean text
    draw_text(
        &mut img,
        "NO IMAGE",
        (width / 2) as i32,
        (height / 2 - 14) as i32,
        opts.title_size,
        opts.color,
        opts.font_path,
    );
    draw_text(
        &mut img,
        "AVAILABLE",
        (width / 2) as i32,
        (height / 2 + 18) as i32,
        opts.subtitle_size,
        opts.color,
        opts.font_path,
    );

    // Save the image
    img.save(output_path)?;

    Ok(())
}

// Backwards-compatible wrapper
pub fn generate_placeholder_image(
    width: u32,
    height: u32,
    output_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    generate_placeholder_image_with_options(width, height, output_path, None)
}

/// Generates the default placeholder image if it doesn't exist
pub fn ensure_placeholder_image_for_album(
    album_name: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Normalize album directory name to match other modules (use hyphens for spaces)
    let album_dir = Path::new("Albums").join(album_name.replace(' ', "-"));
    let placeholder_path = album_dir.join("images").join("no_image_placeholder.png");

    // Check if the placeholder already exists
    if !placeholder_path.exists() {
        // Create images directory if it doesn't exist
        std::fs::create_dir_all(placeholder_path.parent().unwrap())?;

        // Generate the placeholder image using bundled font if available and album name as subtitle
        let bundled = if Path::new("assets/fonts/DejaVuSans.ttf").exists() {
            Some("assets/fonts/DejaVuSans.ttf")
        } else {
            None
        };
        let opts = PlaceholderOptions {
            font_path: bundled,
            title_size: 28.0,
            subtitle_size: 12.0,
            color: Rgba([80, 80, 80, 255]),
            title: Some("NO IMAGE"),
            subtitle: Some(album_name),
        };
        generate_placeholder_image_with_options(200, 150, &placeholder_path, Some(opts))?;
        println!(
            "[INFO] Generated placeholder image: {}",
            placeholder_path.display()
        );
    }

    // Return relative path from the album directory
    Ok("images/no_image_placeholder.png".to_string())
}

/// Generates a per-stamp placeholder image (includes SG number and year) inside the album images folder.
/// Returns the relative path (from album dir) to the generated image, e.g. "images/no_image_SG1234_1967.png".
pub fn ensure_placeholder_image_for_stamp(
    album_name: &str,
    sg_number: &str,
    year: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Normalize album directory name to match other modules (use hyphens for spaces)
    let album_dir = Path::new("Albums").join(album_name.replace(' ', "-"));
    // sanitize sg_number for filename (replace slashes etc.)
    let safe_sg = sg_number.replace(|c: char| !c.is_alphanumeric(), "_");
    let safe_year = if year.is_empty() { "unknown" } else { year };
    let filename = format!("no_image_{}_{}.png", safe_sg, safe_year);
    let placeholder_path = album_dir.join("images").join(&filename);

    if !placeholder_path.exists() {
        std::fs::create_dir_all(placeholder_path.parent().unwrap())?;

        // Choose bundled font if available
        let bundled = if Path::new("assets/fonts/DejaVuSans.ttf").exists() {
            Some("assets/fonts/DejaVuSans.ttf")
        } else {
            None
        };

        // Create a new image buffer with white background
        let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(300, 220);
        for pixel in img.pixels_mut() {
            *pixel = Rgba([255, 255, 255, 255]);
        }

        // Draw a subtle border
        let border_color = Rgba([200, 200, 200, 255]);
        let (width, height) = (img.width(), img.height());
        for x in 0..width {
            for y in 0..height {
                if x < 2 || x >= width - 2 || y < 2 || y >= height - 2 {
                    img.put_pixel(x, y, border_color);
                }
            }
        }

        // Title: SG {sg_number}
        let title = format!("SG {}", sg_number);
        let subtitle = if year.is_empty() {
            "".to_string()
        } else {
            year.to_string()
        };

        // Draw texts centered (use subtle gray similar to samples/placeholder_b.png)
        draw_text(
            &mut img,
            &title,
            (width / 2) as i32,
            (height / 2 - 20) as i32,
            28.0,
            Rgba([90, 90, 90, 255]),
            bundled,
        );
        // Draw a thin separator line to add breathing room between title and subtitle
        let sep_y = (height / 2) as i32 + 2;
        for x in 40..(width - 40) {
            if sep_y >= 0 && (sep_y as u32) < height {
                img.put_pixel(x, sep_y as u32, Rgba([220, 220, 220, 255]));
            }
        }
        if !subtitle.is_empty() {
            draw_text(
                &mut img,
                &subtitle,
                (width / 2) as i32,
                (height / 2 + 18) as i32,
                14.0,
                Rgba([90, 90, 90, 255]),
                bundled,
            );
        }

        // Save
        img.save(&placeholder_path)?;
        println!(
            "[INFO] Generated per-stamp placeholder image: {}",
            placeholder_path.display()
        );
    }

    // Return relative path from album dir
    Ok(format!("images/{}", filename))
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

// Inline tests moved to `tests/placeholder_image.rs`.
