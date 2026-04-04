#![allow(dead_code)]
//! # Placeholder Image Generator
//!
//! This module generates placeholder images for stamps that don't have associated images.

use crate::image_utils::calc_mm_per_pixel_embedded;
use ab_glyph::{point, Font, FontArc, PxScale, ScaleFont};
use image::{GenericImageView, ImageBuffer, Rgba};
use log::{error, info};
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Compute a simple perceptual luminance from an sRGB color (0..255 each).
/// Returns a linear luminance in range 0.0..1.0 approximated by ITU-R BT.709 weights.
pub fn luminance(color: Rgba<u8>) -> f32 {
    let r = color[0] as f32 / 255.0;
    let g = color[1] as f32 / 255.0;
    let b = color[2] as f32 / 255.0;
    // Use gamma-corrected linear approximation: convert sRGB -> linear
    let lin = |c: f32| {
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    let lr = lin(r);
    let lg = lin(g);
    let lb = lin(b);
    // Rec. 709 luminance weights
    0.2126 * lr + 0.7152 * lg + 0.0722 * lb
}

/// Return true if the color is considered "dark" and would benefit from a light
/// background and dark text for contrast.
pub fn is_color_dark(color: Rgba<u8>) -> bool {
    // Luminance threshold chosen so that mid-gray (approx 0.5 sRGB) is considered light.
    luminance(color) < 0.45
}

/// Return a contrasting opaque color (Rgba<u8>) for the provided color.
/// For dark inputs this returns white, for light inputs black. This is intentionally
/// simple and deterministic; callers can refine it later to use more subtle contrasts.
pub fn contrasting_color(color: Rgba<u8>) -> Rgba<u8> {
    if is_color_dark(color) {
        Rgba([255, 255, 255, 255])
    } else {
        Rgba([0, 0, 0, 255])
    }
}

/// Compute a placeholder pixel size for a given series identifier.
/// Uses `sample_images::get_scaling_for_series` to scale from a base pixel size.
pub fn compute_placeholder_size_for_series(series: &str) -> (u32, u32) {
    // Prefer physical mm->px sizing using embedded reference if available.
    // Map series identifier to an expected physical stamp size (mm).
    let (target_mm_w, target_mm_h) = match series.chars().next() {
        Some('L') | Some('O') => (49.0_f64, 21.0_f64),
        Some('M') => (31.1_f64, 24.1_f64),
        Some('N') => (26.1_f64, 21.0_f64),
        Some('R') => (38.0_f64, 25.0_f64),
        Some('S') => (38.0_f64, 25.0_f64),
        Some('T') | Some('U') => (25.0_f64, 24.0_f64),
        _ => (22.0_f64, 18.0_f64),
    };

    // Ask image_utils for the mm-per-pixel ratio using the embedded reference.
    // Try to use the same reference size as the album generator's config so the
    // mm->px mapping matches the values used when emitting stamp boxes.
    let cfg = crate::config::load_config("config.toml");
    let ref_w_mm = cfg.reference_width_mm.unwrap_or(25.0_f32);
    let ref_h_mm = cfg.reference_height_mm.unwrap_or(30.0_f32);
    if let Some((mm_per_px_w, mm_per_px_h)) = calc_mm_per_pixel_embedded(ref_w_mm, ref_h_mm) {
        // mm_per_px_* are f32; convert and compute pixel dims
        let px_w = (target_mm_w as f32 / mm_per_px_w).round() as u32;
        let px_h = (target_mm_h as f32 / mm_per_px_h).round() as u32;
        log::debug!("compute_placeholder_size_for_series: series='{}' target_mm=({:.2},{:.2}) mm_per_px=({:.6},{:.6}) -> px=({},{})", series, target_mm_w, target_mm_h, mm_per_px_w, mm_per_px_h, px_w, px_h);
        // Keep sensible minimums
        return (px_w.max(50), px_h.max(40));
    }

    // Fallback: legacy pixel-scaling from base pixels
    let base_w = 300.0_f64;
    let base_h = 220.0_f64;
    let (w_scale, h_scale) = crate::sample_images::get_scaling_for_series(series);
    let w = (base_w * w_scale).round() as u32;
    let h = (base_h * h_scale).round() as u32;
    (w.max(50), h.max(40))
}

/// High-level helper: generate a per-stamp placeholder image using the provided stamp background
/// color (the color of the source stamp). The generator will choose an image background that
/// contrasts with the stamp background (so dark stamps get a light placeholder and vice versa),
/// then choose the text color to contrast with the image background.
///
/// `series_hint` may be provided to size the placeholder more like a real stamp from that series.
pub fn generate_placeholder_for_stamp_with_bg<P: AsRef<std::path::Path>>(
    _album_name: &str,
    sg_number: &str,
    year: &str,
    stamp_bg_color: Option<Rgba<u8>>,
    series_hint: Option<&str>,
    out_path: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let out = out_path.as_ref();

    // Determine size
    let (w, h) = if let Some(s) = series_hint {
        compute_placeholder_size_for_series(s)
    } else {
        (300u32, 220u32)
    };

    // Determine image background and text colors based on stamp background.
    let stamp_bg = stamp_bg_color.unwrap_or(Rgba([255, 255, 255, 255]));
    let image_bg = contrasting_color(stamp_bg); // white for dark stamp, black for light stamp
    let text_color = contrasting_color(image_bg);

    // Create image and fill with image_bg
    let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(w, h);
    for pixel in img.pixels_mut() {
        *pixel = image_bg;
    }

    // Border: choose a border color that slightly contrasts with background
    let border_color = if is_color_dark(image_bg) {
        Rgba([200, 200, 200, 255])
    } else {
        Rgba([80, 80, 80, 255])
    };
    for x in 0..w {
        for y in 0..h {
            if x < 2 || x >= w - 2 || y < 2 || y >= h - 2 {
                img.put_pixel(x, y, border_color);
            }
        }
    }

    // Draw title and subtitle centered
    let title = format!("SG {}", sg_number);
    let subtitle = if year.is_empty() {
        "".to_string()
    } else {
        year.to_string()
    };

    draw_text(
        &mut img,
        &title,
        (w / 2) as i32,
        (h / 2 - 20) as i32,
        28.0,
        text_color,
        None,
    );
    // Draw separator
    // let sep_y = (h / 2) as i32 + 2;
    // for x in 40..(w.saturating_sub(40)) {
    //     if sep_y >= 0 && (sep_y as u32) < h {
    //         img.put_pixel(x, sep_y as u32, Rgba([220, 220, 220, 255]));
    //     }
    // }
    if !subtitle.is_empty() {
        draw_text(
            &mut img,
            &subtitle,
            (w / 2) as i32,
            (h / 2 + 18) as i32,
            14.0,
            text_color,
            None,
        );
    }

    // Ensure directory exists
    if let Some(parent) = out.parent() {
        std::fs::create_dir_all(parent)?;
    }

    img.save(out)?;
    Ok(())
}

/// Generate a fully-transparent spacer PNG of the requested pixel size and
/// write it into the album images folder. Returns the relative path
/// (e.g. `images/row_spacer_p1_r2.png`) on success.
pub fn generate_transparent_spacer(
    album_name: &str,
    page: usize,
    row: usize,
    px_w: u32,
    px_h: u32,
) -> Result<String, Box<dyn std::error::Error>> {
    let rel = format!("images/row_spacer_p{}_r{}.png", page, row);
    let out_path = format!("Albums/{}/{}", album_name.replace(' ', "-"), rel);
    let out = std::path::Path::new(&out_path);
    if let Some(parent) = out.parent() {
        std::fs::create_dir_all(parent)?;
    }
    // Ensure at least 1x1 pixels
    let w = px_w.max(1);
    let h = px_h.max(1);
    let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(w, h);
    // Fill with fully transparent pixels
    let clear = Rgba([0u8, 0u8, 0u8, 0u8]);
    for pixel in img.pixels_mut() {
        *pixel = clear;
    }
    img.save(out)?;
    Ok(rel)
}

/// Try to load a TTF font from a list of likely locations (system fonts or project fonts).
/// Returns a loaded `ab_glyph::FontArc` on success, or `None` if no font could be found.
fn load_fallback_font(font_override: Option<&str>) -> Option<FontArc> {
    // If caller provided override path, try it first
    if let Some(p) = font_override {
        if Path::new(p).exists() {
            if let Ok(mut f) = File::open(p) {
                let mut buf = Vec::new();
                if f.read_to_end(&mut buf).is_ok() {
                    if let Some(font) = FontArc::try_from_vec(buf).ok() {
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
                        if let Some(font) = FontArc::try_from_vec(buf).ok() {
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
        "C:/Windows/Fonts/arial.ttf", // Windows accepts forward slashes
    ];

    for path in candidates.iter() {
        if Path::new(path).exists() {
            if let Ok(mut f) = File::open(path) {
                let mut buf = Vec::new();
                if f.read_to_end(&mut buf).is_ok() {
                    if let Some(font) = FontArc::try_from_vec(buf).ok() {
                        return Some(font);
                    }
                }
            }
        }
    }

    None
}

/// Draw anti-aliased text centered at (center_x, center_y) using ab_glyph.
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

    let scale = PxScale::from(font_size);
    let scaled_font = font.as_scaled(scale);

    // Measure total advance width for centering
    let total_width: f32 = text
        .chars()
        .map(|c| scaled_font.h_advance(scaled_font.glyph_id(c)))
        .sum();
    let ascent = scaled_font.ascent();
    let descent = scaled_font.descent();

    // Starting point
    let start_x = center_x as f32 - total_width / 2.0;
    let baseline = center_y as f32 + (ascent - descent) / 2.0;

    // Layout glyphs at final render position
    let mut cursor_x = start_x;
    let glyphs: Vec<_> = text
        .chars()
        .map(|c| {
            let glyph_id = scaled_font.glyph_id(c);
            let advance = scaled_font.h_advance(glyph_id);
            let g = glyph_id.with_scale_and_position(scale, point(cursor_x, baseline));
            cursor_x += advance;
            g
        })
        .collect();

    // Draw each glyph into the image buffer
    for g in glyphs {
        if let Some(outlined) = font.outline_glyph(g) {
            let bb = outlined.px_bounds();
            outlined.draw(|gx, gy, gv| {
                let x = bb.min.x as i32 + gx as i32;
                let y = bb.min.y as i32 + gy as i32;
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
        info!(
            "Generated placeholder image: {}",
            placeholder_path.display()
        );
    }

    // Return relative path from the album directory
    Ok("images/no_image_placeholder.png".to_string())
}

/// Generates a per-stamp placeholder image (includes catalog number and year) inside the album images folder.
/// Returns the relative path (from album dir) to the generated image, e.g. "images/no_image_AU_619a_1975.png".
pub fn ensure_placeholder_image_for_stamp(
    album_name: &str,
    catalog_number: &str,
    year: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Normalize album directory name to match other modules (use hyphens for spaces)
    let album_dir = Path::new("Albums").join(album_name.replace(' ', "-"));
    // sanitize catalog_number for filename (replace slashes etc.)
    let safe_catalog = catalog_number.replace(|c: char| !c.is_alphanumeric(), "_");
    let safe_year = if year.is_empty() { "unknown" } else { year };
    let filename = format!("no_image_{}_{}.png", safe_catalog, safe_year);
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

        // Title: catalog number (e.g., "AU 619a")
        let title = catalog_number.to_string();
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
        // let sep_y = (height / 2) as i32 + 2;
        // for x in 40..(width - 40) {
        //     if sep_y >= 0 && (sep_y as u32) < height {
        //         img.put_pixel(x, sep_y as u32, Rgba([220, 220, 220, 255]));
        //     }
        // }
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
        info!(
            "Generated per-stamp placeholder image: {}",
            placeholder_path.display()
        );
    }

    // Return relative path from album dir
    Ok(format!("images/{}", filename))
}

/// New: ensure placeholder for a stamp but accept a series hint and optional
/// stamp background color so the generated placeholder can match the series
/// sizing and contrast. Backwards-compatible callers can continue to call
/// `ensure_placeholder_image_for_stamp` which delegates to this with None hints.
pub fn ensure_placeholder_image_for_stamp_with_hint(
    album_name: &str,
    catalog_number: &str,
    year: &str,
    series_hint: Option<&str>,
    stamp_bg_color: Option<Rgba<u8>>,
    // If provided, target_mm_* and mm_per_px_* will be used to compute an exact
    // pixel size so the generated placeholder, when converted by the album
    // mm-per-pixel ratio, matches the emitted mm dimensions. If omitted we
    // fall back to series-based pixel sizing.
    target_mm_w: Option<f64>,
    target_mm_h: Option<f64>,
    mm_per_px_w: Option<f32>,
    mm_per_px_h: Option<f32>,
) -> Result<String, Box<dyn std::error::Error>> {
    // Normalize album directory name to match other modules (use hyphens for spaces)
    let album_dir = Path::new("Albums").join(album_name.replace(' ', "-"));
    // sanitize catalog_number for filename (replace slashes etc.)
    let safe_catalog = catalog_number.replace(|c: char| !c.is_alphanumeric(), "_");
    let safe_year = if year.is_empty() { "unknown" } else { year };
    let filename = format!("no_image_{}_{}.png", safe_catalog, safe_year);
    let placeholder_path = album_dir.join("images").join(&filename);

    // Ensure parent dir exists even if file already exists (we may overwrite)
    std::fs::create_dir_all(placeholder_path.parent().unwrap())?;

    // Determine desired pixel dimensions. Prefer explicit target mm -> px mapping
    // using the provided mm_per_px values so the placeholder's reported
    // mm size matches the emitted box exactly. Otherwise fall back to
    // series-aware sizing.
    let explicit_mapping = target_mm_w.is_some()
        && target_mm_h.is_some()
        && mm_per_px_w.is_some()
        && mm_per_px_h.is_some();

    let (desired_px_w, desired_px_h) = if explicit_mapping {
        let tw = target_mm_w.unwrap();
        let th = target_mm_h.unwrap();
        let mmw = mm_per_px_w.unwrap();
        let mmh = mm_per_px_h.unwrap();
        let w = (tw as f32 / mmw).round() as u32;
        let h = (th as f32 / mmh).round() as u32;
        (w.max(50), h.max(40))
    } else if let Some(s) = series_hint {
        compute_placeholder_size_for_series(s)
    } else {
        (300u32, 220u32)
    };

    // Load config to see if the user wants placeholders to always be
    // overwritten when explicit mm/mm_per_px mappings are provided.
    let cfg = crate::config::load_config("config.toml");
    let _placeholder_overwrite = cfg.placeholder_always_overwrite.unwrap_or(true);

    // If the placeholder already exists but its pixel dimensions do not match
    // the desired dimensions (e.g., created by an earlier run), overwrite it so
    // the placeholder pixel size corresponds to the emitted mm box. Additionally,
    // when an explicit mm->px mapping was provided, always regenerate the
    // placeholder to ensure it exactly matches the emitted box semantics and
    // any textual or styling updates.
    // If explicit mm->px mapping was provided we MUST produce a placeholder
    // with the requested pixel dimensions so layout and diagnostics remain
    // consistent. Always recreate when explicit mapping is present. Otherwise
    // respect existing files unless their dimensions differ from desired.
    let mut need_create = true;
    if placeholder_path.exists() {
        if explicit_mapping {
            // Always recreate to guarantee exact pixel dimensions for the
            // explicit mapping case (user requested it in generator).
            log::info!(
                "Placeholder {} will be recreated to match explicit mm->px mapping {}x{} px",
                placeholder_path.display(),
                desired_px_w,
                desired_px_h
            );
            need_create = true;
        } else {
            // Try to read actual dimensions; if reading fails we'll recreate.
            if let Ok(img_existing) = image::open(&placeholder_path) {
                let (aw, ah) = img_existing.dimensions();
                if aw == desired_px_w && ah == desired_px_h {
                    need_create = false;
                } else {
                    // log the mismatch and proceed to recreate
                    log::info!("Placeholder {} exists with size {}x{} px but desired is {}x{} px; recreating", placeholder_path.display(), aw, ah, desired_px_w, desired_px_h);
                }
            }
        }
    }

    if need_create {
        // Determine bundled font path if available
        let bundled = if Path::new("assets/fonts/DejaVuSans.ttf").exists() {
            Some("assets/fonts/DejaVuSans.ttf")
        } else {
            None
        };

        let px_w = desired_px_w;
        let px_h = desired_px_h;

        let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(px_w, px_h);
        for pixel in img.pixels_mut() {
            *pixel = stamp_bg_color
                .map(|c| contrasting_color(c))
                .unwrap_or(Rgba([255, 255, 255, 255]));
        }

        // Draw border
        let border_color = if is_color_dark(img.get_pixel(0, 0).to_owned()) {
            Rgba([200, 200, 200, 255])
        } else {
            Rgba([80, 80, 80, 255])
        };
        for x in 0..px_w {
            for y in 0..px_h {
                if x < 2 || x >= px_w - 2 || y < 2 || y >= px_h - 2 {
                    img.put_pixel(x, y, border_color);
                }
            }
        }

        // Title/subtitle
        let title = catalog_number.to_string();
        let subtitle = if year.is_empty() {
            "".to_string()
        } else {
            year.to_string()
        };
        draw_text(
            &mut img,
            &title,
            (px_w / 2) as i32,
            (px_h / 2 - 20) as i32,
            28.0,
            Rgba([90, 90, 90, 255]),
            bundled,
        );
        // let sep_y = (px_h / 2) as i32 + 2;
        // for x in 40..(px_w.saturating_sub(40)) {
        //     if sep_y >= 0 && (sep_y as u32) < px_h {
        //         img.put_pixel(x, sep_y as u32, Rgba([220, 220, 220, 255]));
        //     }
        // }
        if !subtitle.is_empty() {
            draw_text(
                &mut img,
                &subtitle,
                (px_w / 2) as i32,
                (px_h / 2 + 18) as i32,
                14.0,
                Rgba([90, 90, 90, 255]),
                bundled,
            );
        }

        img.save(&placeholder_path)?;
        // Verify saved image dimensions match the requested pixel size. If
        // they do not, attempt one more time and then return an error so
        // callers are aware.
        match image::open(&placeholder_path) {
            Ok(saved) => {
                let (sw, sh) = saved.dimensions();
                if sw != px_w || sh != px_h {
                    // Try a second write using the higher-level generator to be safe
                    log::warn!("Saved placeholder {} has size {}x{} px but expected {}x{} px — attempting rewrite", placeholder_path.display(), sw, sh, px_w, px_h);
                    // Re-generate explicitly via the options API which is known
                    // to produce exact buffer sizes.
                    let _ = generate_placeholder_image_with_options(
                        px_w,
                        px_h,
                        &placeholder_path,
                        None,
                    )?;
                    // Re-open and check again
                    if let Ok(reopen) = image::open(&placeholder_path) {
                        let (rw, rh) = reopen.dimensions();
                        if rw != px_w || rh != px_h {
                            return Err(format!("Failed to write placeholder {} at requested size {}x{} px (wrote {}x{} px)", placeholder_path.display(), px_w, px_h, rw, rh).into());
                        }
                    } else {
                        return Err(format!(
                            "Failed to verify placeholder image after rewrite: {}",
                            placeholder_path.display()
                        )
                        .into());
                    }
                }
            }
            Err(e) => {
                return Err(format!(
                    "Failed to read back placeholder image {}: {}",
                    placeholder_path.display(),
                    e
                )
                .into());
            }
        }
        info!(
            "Generated per-stamp placeholder image: {} ({}x{} px)",
            placeholder_path.display(),
            px_w,
            px_h
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
            error!("Failed to create placeholder image: {}", e);
            // Fallback to a simple text-based path
            "images/no_image_placeholder.png".to_string()
        }
    }
}

// Inline tests moved to `tests/placeholder_image.rs`.
