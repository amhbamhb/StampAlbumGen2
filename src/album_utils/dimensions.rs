use std::process::Command;

/// Calculates appropriate stamp box dimensions based on image dimensions
///
/// This function implements a sophisticated stamp sizing algorithm that:
/// 1. Loads the actual image dimensions from disk
/// 2. Uses embedded reference image for consistent mm/px ratios
/// 3. Applies series-specific scaling factors for precise control
/// 4. Applies reasonable bounds to ensure proper album layout
///
/// # Arguments
///
/// * `image_path` - Relative path to the stamp image (e.g., "L-s.jpg")
/// * `album_name` - Name of the album for path construction
///
/// # Returns
///
/// Tuple of (width_mm, height_mm) representing the calculated stamp dimensions
///
/// # Algorithm
///
/// 1. **Image Loading**: Attempts to load actual pixel dimensions from the image file
/// 2. **Reference Calculation**: Uses embedded B4-s.jpg reference for consistent mm/px ratios
/// 3. **Series Scaling**: Applies series-specific scaling factors (L/M/N series)
/// 4. **Bounds Checking**: Ensures dimensions fit within album layout constraints
/// 5. **Fallback**: Uses default dimensions if image loading fails
///
/// # Examples
///
/// ```rust
/// // Calculate dimensions for L-series stamp
/// let cfg = stamp_album_gen2::Config::default();
/// let (width, height) = stamp_album_gen2::album_utils::calculate_stamp_dimensions("L-s.jpg", "Canada", &cfg);
/// // Result: (49.0, 21.0) for L-series with ALBUM_STAMP_IMG_ASPECT_RATIO = false
/// ```
///
/// # Scaling Factors Applied
///
/// - **L-series**: (1.25, 2.45) → 49.0mm x 21.0mm target
/// - **M-series**: (1.15, 1.45) → 31.1mm x 24.1mm target
/// - **N-series**: (1.10, 1.40) → 26.1mm x 21.0mm target
pub fn calculate_stamp_dimensions(
    image_path: &str,
    album_name: &str,
    cfg: &crate::config::Config,
) -> (f64, f64) {
    // Preferred approach: use actual image pixel dimensions converted to mm
    // via the embedded reference image. This eliminates reliance on hard-coded
    // series groups (L/M/N/O) and produces sizes based on the real image.

    // Reference real-world dimensions used with the embedded B4-s.jpg
    // Updated via calibration (see scripts/calibrate_mm_per_px.sh)
    // Suggested values computed from user-supplied measurements on 2025-09-21
    const REF_W_MM: f32 = 23.42;
    const REF_H_MM: f32 = 25.93;

    // Resolve candidate filesystem paths for the image. The DB may contain "images/<file>"
    // or just a filename; images are stored under Albums/<Album-Name>/images/<file>.
    // We'll prefer a local path first (image_path as-is), then try the album images dir.
    let mut resolved_image_path = image_path.to_string();
    let album_images_candidate = format!("Albums/{}/{}", album_name.replace(' ', "-"), image_path);
    // If the straight image_path does not exist, use the album images candidate for measurements.
    if !std::path::Path::new(&resolved_image_path).exists()
        && std::path::Path::new(&album_images_candidate).exists()
    {
        resolved_image_path = album_images_candidate;
    }

    // If a calibration CSV is provided, load it and check for a basename match.
    // CSV format: filename,mm_per_px_w,mm_per_px_h
    fn load_calibration_map(path: &str) -> std::collections::HashMap<String, (f32, f32)> {
        let mut m = std::collections::HashMap::new();
        if let Ok(content) = std::fs::read_to_string(path) {
            for (i, line) in content.lines().enumerate() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
                if parts.len() >= 3 {
                    if let (Ok(w), Ok(h)) = (parts[1].parse::<f32>(), parts[2].parse::<f32>()) {
                        m.insert(parts[0].to_string(), (w, h));
                    }
                } else {
                    log::warn!("calibration CSV: bad line {}: {}", i + 1, line);
                }
            }
        }
        m
    }

    // Try to find a calibration match by filename (basename)
    if let Some(cal_csv) = cfg.calibration_csv.as_ref() {
        let calib_map = load_calibration_map(cal_csv);
        if let Some(basename) = std::path::Path::new(&resolved_image_path)
            .file_name()
            .and_then(|s| s.to_str())
        {
            if let Some((mm_per_px_w, mm_per_px_h)) = calib_map.get(basename) {
                if let Some((w_mm, h_mm)) = crate::image_utils::estimate_real_dimensions(
                    &resolved_image_path,
                    *mm_per_px_w,
                    *mm_per_px_h,
                ) {
                    let mut width = w_mm as f64;
                    let mut height = h_mm as f64;
                    if let Some(scale) = cfg.global_scale {
                        width *= scale as f64;
                        height *= scale as f64;
                    }
                    width = width.clamp(10.0, 120.0);
                    height = height.clamp(10.0, 120.0);
                    return (width, height);
                }
            }
        }
    }

    // Preferred approach: use embedded-reference sizing first (more consistent)
    if let Some((mm_per_px_w, mm_per_px_h)) =
        crate::image_utils::calc_mm_per_pixel_embedded(REF_W_MM, REF_H_MM)
    {
        if let Some((w_mm, h_mm)) = crate::image_utils::estimate_real_dimensions(
            &resolved_image_path,
            mm_per_px_w,
            mm_per_px_h,
        ) {
            let mut width = w_mm as f64;
            let mut height = h_mm as f64;

            // Enforce reasonable bounds to avoid absurd sizes from mis-detected images
            if width > 120.0 || height > 120.0 {
                log::warn!(
                    "Embedded-reference measured size is unexpectedly large ({:.1} x {:.1} mm) for image '{}'.",
                    width,
                    height,
                    resolved_image_path
                );
            } else {
                // Apply global_scale from provided config if present
                if let Some(scale) = cfg.global_scale {
                    width *= scale as f64;
                    height *= scale as f64;
                }

                // Clamp to expected practical range (older stamps are often ~22-37mm)
                width = width.clamp(10.0, 120.0);
                height = height.clamp(10.0, 120.0);
                return (width, height);
            }
        }
    }

    // If embedded-reference approach failed, try DPI-based sizing
    if let Ok(output) = Command::new("identify")
        .args(["-format", "%x %y %U", &resolved_image_path])
        .output()
    {
        if output.status.success() {
            if let Ok(s) = String::from_utf8(output.stdout) {
                let parts: Vec<&str> = s.split_whitespace().collect();
                // Handle possible formats from ImageMagick identify:
                let mut dpi_x_opt: Option<f32> = None;
                let mut dpi_y_opt: Option<f32> = None;
                let mut unit_opt: Option<&str> = None;

                if parts.len() >= 3 {
                    if let (Ok(dx), Ok(dy)) = (parts[0].parse::<f32>(), parts[1].parse::<f32>()) {
                        dpi_x_opt = Some(dx);
                        dpi_y_opt = Some(dy);
                        unit_opt = Some(parts[2]);
                    } else if parts[0].contains('x') {
                        if let Some((dxs, dys)) = parts[0].split_once('x') {
                            if let (Ok(dx), Ok(dy)) = (dxs.parse::<f32>(), dys.parse::<f32>()) {
                                dpi_x_opt = Some(dx);
                                dpi_y_opt = Some(dy);
                                unit_opt = Some(parts[1]);
                            }
                        }
                    }
                } else if parts.len() >= 2 && parts[0].contains('x') {
                    if let Some((dxs, dys)) = parts[0].split_once('x') {
                        if let (Ok(dx), Ok(dy)) = (dxs.parse::<f32>(), dys.parse::<f32>()) {
                            dpi_x_opt = Some(dx);
                            dpi_y_opt = Some(dy);
                            unit_opt = Some(parts[1]);
                        }
                    }
                }

                if let (Some(dpi_x), Some(dpi_y), Some(unit)) = (dpi_x_opt, dpi_y_opt, unit_opt) {
                    let (mm_per_px_w_dpi, mm_per_px_h_dpi) = if unit.to_lowercase().contains("inch")
                        || unit.to_lowercase().contains("pixelsperinch")
                    {
                        (25.4_f32 / dpi_x, 25.4_f32 / dpi_y)
                    } else if unit.to_lowercase().contains("centimeter")
                        || unit.to_lowercase().contains("pixelspercentimeter")
                    {
                        (10.0_f32 / dpi_x, 10.0_f32 / dpi_y)
                    } else {
                        (0.0_f32, 0.0_f32)
                    };

                    if mm_per_px_w_dpi > 0.0 && mm_per_px_h_dpi > 0.0 {
                        if let Some((wpx, hpx)) =
                            crate::image_utils::get_image_dimensions(&resolved_image_path)
                        {
                            let mut w_mm = (wpx as f32) * mm_per_px_w_dpi;
                            let mut h_mm = (hpx as f32) * mm_per_px_h_dpi;
                            // Apply global_scale if present
                            if let Some(scale) = cfg.global_scale {
                                w_mm *= scale;
                                h_mm *= scale;
                            }
                            let w_mm = w_mm as f64;
                            let h_mm = h_mm as f64;
                            // Clamp and return
                            return (w_mm.clamp(10.0, 120.0), h_mm.clamp(10.0, 120.0));
                        }
                    }
                }
            }
        }
    }

    // If DPI-based approach failed or unknown, attempt to compute mm-per-pixel from the embedded reference image
    if let Some((mm_per_px_w, mm_per_px_h)) =
        crate::image_utils::calc_mm_per_pixel_embedded(REF_W_MM, REF_H_MM)
    {
        // Try to estimate using the actual image file if it exists
        if let Some((w_mm, h_mm)) = crate::image_utils::estimate_real_dimensions(
            &resolved_image_path,
            mm_per_px_w,
            mm_per_px_h,
        ) {
            let width = w_mm as f64;
            let height = h_mm as f64;

            // If measurements are absurdly large (likely a full-page scan or wrong reference),
            // attempt a DPI-based fallback (already tried), otherwise fall back to conservative default.
            if width > 120.0 || height > 120.0 {
                log::warn!(
                    "Embedded-reference measured size is unexpectedly large ({:.1} x {:.1} mm) for image '{}'. Falling back to default.",
                    width,
                    height,
                    resolved_image_path
                );
                return (28.0_f64, 31.0_f64);
            }

            // Enforce reasonable bounds to avoid absurd sizes
            let width = width.clamp(18.0, 60.0); // between 18mm and 60mm
            let height = height.clamp(14.0, 60.0); // between 14mm and 60mm

            return (width, height);
        }
    }

    // Fallback: if image-based estimation fails, use a conservative default
    // (previous default was 28.0 x 31.0). Keep a single sensible fallback
    // and remove the L/M/N/O series branching entirely.
    let (width, height) = (28.0_f64, 31.0_f64);

    (width, height)
}
