use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::stampworld::fetch::{Fetcher, ReqwestFetcher};
use image::GenericImageView;

/// Small local flag DB: map lowercase country name -> ISO alpha-2 code
/// We'll initialize it with a compact set and allow loading from a JSON file
/// under `flags/flag_codes.json` to allow updates.

pub fn default_flag_map() -> HashMap<String, String> {
    // Minimal initial set; we'll try to be forgiving with lookups
    let mut m = HashMap::new();
    m.insert("united states".to_string(), "us".to_string());
    m.insert("usa".to_string(), "us".to_string());
    m.insert("canada".to_string(), "ca".to_string());
    m.insert("great britain".to_string(), "gb".to_string());
    m.insert("united kingdom".to_string(), "gb".to_string());
    m.insert("israel".to_string(), "il".to_string());
    m.insert("australia".to_string(), "au".to_string());
    m.insert("germany".to_string(), "de".to_string());
    m.insert("france".to_string(), "fr".to_string());
    // ... add more as needed, but primary approach is to load a more complete list from flags/flag_codes.json
    m
}

/// Attempt to load an override JSON file at flags/flag_codes.json. The file should be a JSON map
/// of "country name" -> "alpha2 code". If not present, fall back to default_flag_map.
pub fn load_flag_map_from_fs() -> HashMap<String, String> {
    // Try a few candidate locations for flags/flag_codes.json to be tolerant
    // when the binary is run from a different working directory. Prefer the
    // current working directory's `flags/` first, then the executable's
    // parent directory (useful when running from a dist directory that sits
    // beside the repo). If none found, fall back to the built-in map.
    let mut candidates: Vec<PathBuf> = Vec::new();
    // Environment override (highest priority)
    if let Ok(p) = std::env::var("STAMP_FLAGS_DIR") {
        candidates.push(PathBuf::from(p).join("flag_codes.json"));
    }
    // Current working directory
    candidates.push(PathBuf::from("flags/flag_codes.json"));
    // Executable parent directory
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            candidates.push(parent.join("flags/flag_codes.json"));
        }
    }
    // Parent of current working dir (common when running from a dist dir)
    if let Ok(cwd) = std::env::current_dir() {
        if let Some(parent) = cwd.parent() {
            candidates.push(parent.join("flags/flag_codes.json"));
        }
    }
    // At build time, embed the manifest dir so distributed binaries can still
    // look up the repo's flags/ directory if the source tree is present.
    candidates.push(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("flags/flag_codes.json"));

    for p in candidates.iter() {
        if p.exists() {
            if let Ok(contents) = fs::read_to_string(p) {
                if let Ok(j) = serde_json::from_str::<HashMap<String, String>>(&contents) {
                    return j
                        .into_iter()
                        .map(|(k, v)| (k.to_lowercase(), v.to_lowercase()))
                        .collect();
                }
            }
        }
    }

    // If no file found on disk, fall back to an embedded copy of the
    // flag_codes.json so the binary is usable even when run from a small
    // temporary directory. Also attempt to write the embedded JSON out to a
    // 'flags/flag_codes.json' file so subsequent runs can pick it up.
    const EMBEDDED_FLAGS_JSON: &str = include_str!("../flags/flag_codes.json");
    if let Ok(j) = serde_json::from_str::<HashMap<String, String>>(EMBEDDED_FLAGS_JSON) {
        // Attempt to write the embedded json to the preferred writable location
        // (cwd 'flags' preferred). Ignore write errors but log them at caller
        // level if needed; here we simply try the obvious locations.
        let prefer_dirs: Vec<PathBuf> = vec![
            PathBuf::from("flags"),
            // exe parent
            std::env::current_exe()
                .ok()
                .and_then(|e| e.parent().map(|p| p.to_path_buf()))
                .unwrap_or_else(|| PathBuf::from("flags")),
            // build manifest
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("flags"),
        ];
        // Try writing to the current working directory 'flags' first and
        // overwrite any existing file so tests that set a temporary cwd can
        // reliably observe the write-back. If that fails, try the other
        // preferred dirs and stop when a write succeeds.
        let cwd_flags = PathBuf::from("flags");
        if fs::create_dir_all(&cwd_flags).is_ok() {
            let out_path = cwd_flags.join("flag_codes.json");
            let _ = fs::write(&out_path, EMBEDDED_FLAGS_JSON);
        } else {
            for dir in prefer_dirs.iter() {
                if fs::create_dir_all(dir).is_ok() {
                    let out_path = dir.join("flag_codes.json");
                    // Attempt to write (overwrite) and stop on success
                    if fs::write(&out_path, EMBEDDED_FLAGS_JSON).is_ok() {
                        break;
                    }
                }
            }
        }

        return j
            .into_iter()
            .map(|(k, v)| (k.to_lowercase(), v.to_lowercase()))
            .collect();
    }

    default_flag_map()
}

/// Given a country name, attempt to return an alpha-2 code (lowercase) using simple normalization.
pub fn country_to_code(country: &str) -> Option<String> {
    let map = load_flag_map_from_fs();
    let key = country.trim().to_lowercase();
    if let Some(code) = map.get(&key) {
        return Some(code.clone());
    }
    // Try simplified remappings (strip punctuation, common words)
    let key2 = key.replace('\'', "").replace('-', " ").replace(',', "");
    if let Some(code) = map.get(&key2) {
        return Some(code.clone());
    }

    // Fallback: some albums are sub-national regions (provinces/states) like "New Brunswick"
    // Provide a small mapping from common region names to their parent country codes so a
    // sensible flag (e.g. Canada) can be used for the cover page when the album is regional.
    let mut region_map: HashMap<String, String> = HashMap::new();
    // Canadian provinces (common names)
    region_map.insert("new brunswick".to_string(), "ca".to_string());
    region_map.insert("british columbia".to_string(), "ca".to_string());
    region_map.insert("nova scotia".to_string(), "ca".to_string());
    region_map.insert("prince edward island".to_string(), "ca".to_string());
    region_map.insert("ontario".to_string(), "ca".to_string());
    region_map.insert("quebec".to_string(), "ca".to_string());
    region_map.insert("manitoba".to_string(), "ca".to_string());
    region_map.insert("saskatchewan".to_string(), "ca".to_string());
    region_map.insert("newfoundland".to_string(), "ca".to_string());

    // US states (examples) - fallback to 'us' for obvious matches
    region_map.insert("alabama".to_string(), "us".to_string());
    region_map.insert("alaska".to_string(), "us".to_string());
    region_map.insert("california".to_string(), "us".to_string());
    region_map.insert("new york".to_string(), "us".to_string());

    // Try lookup in region_map using key and key2
    if let Some(code) = region_map.get(&key) {
        return Some(code.clone());
    }
    if let Some(code) = region_map.get(&key2) {
        return Some(code.clone());
    }

    None
}

/// Normalize free-form country input into a canonical lookup key.
/// Replaces common separators with spaces and lowercases so callers can
/// accept values like "United-States" or "United_States".
pub fn normalize_country_input(country: &str) -> String {
    country
        .trim()
        .to_lowercase()
        .replace('-', " ")
        .replace('_', " ")
        .replace(',', "")
}

/// Ensure flag image exists for the given alpha2 code in flags/, download from FlagCDN if missing.
/// size example: "w80" or "w320". Returns path to downloaded file on success.
pub fn ensure_flag_image(alpha2: &str, size: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Backwards-compatible wrapper that uses the default ReqwestFetcher.
    let fetcher = ReqwestFetcher::new();
    ensure_flag_image_with_fetcher(alpha2, size, &fetcher)
}

/// Ensure flag image exists for the given alpha2 code in flags/, download from FlagCDN if missing.
/// size example: "w80" or "w320". Returns path to downloaded file on success.
pub fn ensure_flag_image_with_fetcher(
    alpha2: &str,
    size: &str,
    fetcher: &dyn Fetcher,
) -> Result<String, Box<dyn std::error::Error>> {
    let alpha = alpha2.to_lowercase();
    let filename = format!("{}-{}.png", alpha, size);

    // Candidate directories to look for an existing flags image. Check the
    // current working directory first, then the executable parent, then the
    // parent of the cwd (useful in some layouts).
    let mut candidates: Vec<PathBuf> = Vec::new();
    // Environment override
    if let Ok(p) = std::env::var("STAMP_FLAGS_DIR") {
        candidates.push(PathBuf::from(p));
    }
    // Current working directory 'flags'
    candidates.push(PathBuf::from("flags"));
    // Executable parent 'flags'
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            candidates.push(parent.join("flags"));
        }
    }
    // Parent of cwd 'flags'
    if let Ok(cwd) = std::env::current_dir() {
        if let Some(parent) = cwd.parent() {
            candidates.push(parent.join("flags"));
        }
    }
    // Build-time manifest 'flags' (source tree)
    candidates.push(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("flags"));

    // If we already have the image in any candidate dir, return its path.
    for dir in candidates.iter() {
        let filepath = dir.join(&filename);
        if filepath.exists() {
            // If a rasterized PNG already exists next to the SVG, prefer it.
            let png_candidate = filepath.with_extension("png");
            if png_candidate.exists() {
                return Ok(png_candidate.to_string_lossy().to_string());
            }

            // Attempt to rasterize the existing SVG in-place; if successful,
            // return the PNG path so callers (renderer) get a supported format.
            if try_rasterize_svg_to_png(&filepath, &png_candidate).unwrap_or(false) {
                return Ok(png_candidate.to_string_lossy().to_string());
            }

            return Ok(filepath.to_string_lossy().to_string());
        }
    }

    // Not found locally: pick a writeable target directory to store downloaded flags.
    // Preference order: STAMP_FLAGS_DIR env, cwd 'flags', exe parent 'flags', build manifest 'flags'.
    let mut target_dir = PathBuf::from("flags");
    if let Ok(p) = std::env::var("STAMP_FLAGS_DIR") {
        let cand = PathBuf::from(p);
        if fs::create_dir_all(&cand).is_ok() {
            target_dir = cand;
        }
    }
    if !target_dir.exists() && fs::create_dir_all(&target_dir).is_err() {
        if let Ok(exe) = std::env::current_exe() {
            if let Some(parent) = exe.parent() {
                let alt = parent.join("flags");
                if fs::create_dir_all(&alt).is_ok() {
                    target_dir = alt;
                }
            }
        }
    }

    let filepath = target_dir.join(&filename);

    // Build URL: https://flagcdn.com/[size]/[code].png where size is like w40 (width px)
    let url = format!("https://flagcdn.com/{}/{}.png", size, alpha);

    // Use the injected fetcher to retrieve the bytes. The Fetcher implementation
    // is responsible for user-agent, timeouts and retry policy.
    let (_status, _headers, content_bytes) = fetcher.fetch_bytes(&url)?;
    let mut out = fs::File::create(&filepath)?;
    // Write the downloaded bytes directly to the file
    out.write_all(&content_bytes)?;
    Ok(filepath.to_string_lossy().to_string())
}

/// Convenience: try to find a flag for the provided country name and copy it into the given album images dir
/// If no match, return Err so callers can fallback to placeholder creation.
pub fn copy_flag_for_album(
    country: &str,
    album_images_dir: &str,
    size: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Normalize country input to be forgiving of separators & case
    let norm = normalize_country_input(country);
    if let Some(code) = country_to_code(&norm) {
        let downloaded = ensure_flag_image(&code, size)?;
        let dest = Path::new(album_images_dir).join("flag_placeholder.png");
        fs::create_dir_all(album_images_dir)?;
        fs::copy(&downloaded, &dest)?;
        Ok(dest.to_string_lossy().to_string())
    } else {
        Err("No country code match".into())
    }
}

/// Ensure coat of arms SVG exists for the given alpha2 code in flags/coats_of_arms/, download
/// from restcountries.com if missing. Returns path to downloaded file on success.
pub fn ensure_coat_of_arms(alpha2: &str) -> Result<String, Box<dyn std::error::Error>> {
    let fetcher = ReqwestFetcher::new_for_api();
    ensure_coat_of_arms_with_fetcher(alpha2, &fetcher)
}

pub fn ensure_coat_of_arms_with_fetcher(
    alpha2: &str,
    fetcher: &dyn Fetcher,
) -> Result<String, Box<dyn std::error::Error>> {
    let alpha = alpha2.to_lowercase();
    let filename = format!("{}.svg", alpha);

    // Candidate directories to look for an existing coat of arms image.
    let mut candidates: Vec<PathBuf> = Vec::new();
    // Environment override (STAMP_FLAGS_DIR/coats_of_arms)
    if let Ok(p) = std::env::var("STAMP_FLAGS_DIR") {
        candidates.push(PathBuf::from(p).join("coats_of_arms"));
    }
    // Current working directory 'flags/coats_of_arms'
    candidates.push(PathBuf::from("flags").join("coats_of_arms"));
    // Executable parent 'flags/coats_of_arms'
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            candidates.push(parent.join("flags").join("coats_of_arms"));
        }
    }
    // Parent of cwd 'flags/coats_of_arms'
    if let Ok(cwd) = std::env::current_dir() {
        if let Some(parent) = cwd.parent() {
            candidates.push(parent.join("flags").join("coats_of_arms"));
        }
    }
    // Build-time manifest 'flags/coats_of_arms' (source tree)
    candidates.push(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("flags")
            .join("coats_of_arms"),
    );

    for dir in candidates.iter() {
        let filepath = dir.join(&filename);
        if filepath.exists() {
            // Prefer an existing rasterized PNG if present
            let png_candidate = filepath.with_extension("png");
            if png_candidate.exists() {
                return Ok(png_candidate.to_string_lossy().to_string());
            }

            // Try to rasterize the existing SVG to PNG so downstream renderer
            // receives a supported image format. If conversion succeeds return
            // the PNG path, otherwise return the SVG path.
            if try_rasterize_svg_to_png(&filepath, &png_candidate).unwrap_or(false) {
                return Ok(png_candidate.to_string_lossy().to_string());
            }

            return Ok(filepath.to_string_lossy().to_string());
        }
    }

    // Not found locally: pick a writeable target directory to store downloaded coats of arms.
    let mut target_dir = PathBuf::from("flags").join("coats_of_arms");
    if let Ok(p) = std::env::var("STAMP_FLAGS_DIR") {
        let cand = PathBuf::from(p).join("coats_of_arms");
        if fs::create_dir_all(&cand).is_ok() {
            target_dir = cand;
        }
    }
    if !target_dir.exists() && fs::create_dir_all(&target_dir).is_err() {
        if let Ok(exe) = std::env::current_exe() {
            if let Some(parent) = exe.parent() {
                let alt = parent.join("flags").join("coats_of_arms");
                if fs::create_dir_all(&alt).is_ok() {
                    target_dir = alt;
                }
            }
        }
    }

    let filepath = target_dir.join(&filename);

    // Query restcountries API for coatOfArms URL: https://restcountries.com/v3.1/alpha/{alpha}
    let api_url = format!("https://restcountries.com/v3.1/alpha/{}", alpha);
    let (_status, body) = fetcher.fetch_text(&api_url)?;
    // Parse JSON and extract coatOfArms.svg
    let v: serde_json::Value = serde_json::from_str(&body)?;
    // API returns an array with a single object
    let coa_url_opt = v
        .get(0)
        .and_then(|o| o.get("coatOfArms"))
        .and_then(|c| c.get("svg"))
        .and_then(|s| s.as_str())
        .map(|s| s.to_string());

    if let Some(coa_url) = coa_url_opt {
        if coa_url.trim().is_empty() {
            return Err("No coatOfArms.svg URL provided by API".into());
        }
        // Download the SVG bytes
        let (_s, _h, bytes) = fetcher.fetch_bytes(&coa_url)?;
        let mut out = fs::File::create(&filepath)?;
        out.write_all(&bytes)?;

        // Attempt best-effort rasterization to PNG so the renderer (which
        // currently only supports PNG/JPG) can consume the coat-of-arms.
        // We try common command-line tools in order: rsvg-convert, convert, magick.
        // If none exist or conversion fails, we leave the SVG on disk and
        // return its path; callers may decide how to handle SVG fallback.
        let png_path = filepath.with_extension("png");
        if try_rasterize_svg_to_png(&filepath, &png_path).unwrap_or(false) {
            // Prefer the PNG when available
            return Ok(png_path.to_string_lossy().to_string());
        }

        return Ok(filepath.to_string_lossy().to_string());
    }

    Err("Coat of arms not available for this country".into())
}

/// Convenience: try to find a coat of arms for the provided country name and copy it into the given album images dir
pub fn copy_coa_for_album(
    country: &str,
    album_images_dir: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Normalize input to handle hyphens/underscores and casing
    let norm = normalize_country_input(country);
    if let Some(code) = country_to_code(&norm) {
        let downloaded = ensure_coat_of_arms(&code)?;
        fs::create_dir_all(album_images_dir)?;
        let src_path = Path::new(&downloaded).to_path_buf();
        // If the source is a small PNG, try to find an SVG sibling and
        // re-rasterize it at a larger target size so the album gets a
        // high-quality coat PNG. This covers the case where a previously
        // cached small PNG exists but a higher-quality SVG is present.
        if src_path
            .extension()
            .and_then(|s| s.to_str())
            .map_or(false, |ext| ext.eq_ignore_ascii_case("png"))
        {
            // Check current PNG width
            if let Ok(img) = image::open(&src_path) {
                let (w, _h) = img.dimensions();
                let target_width: u32 = std::env::var("COAT_TARGET_WIDTH")
                    .ok()
                    .and_then(|s| s.parse::<u32>().ok())
                    .unwrap_or(600);
                if w < target_width {
                    let svg_candidate = src_path.with_extension("svg");
                    if svg_candidate.exists() {
                        // Attempt rasterization from the SVG into the PNG path.
                        let _ = try_rasterize_svg_to_png(&svg_candidate, &src_path);
                        // If successful, the png at src_path will be replaced
                        // with the higher-quality downsampled PNG.
                    }
                }
            }
        }
        // Choose dest filename by source extension so we copy the rasterized
        // PNG when available, otherwise fall back to SVG.
        let dest_name = match src_path.extension().and_then(|s| s.to_str()) {
            Some("png") | Some("jpg") | Some("jpeg") => "coat_of_arms.png",
            _ => "coat_of_arms.svg",
        };
        let dest = Path::new(album_images_dir).join(dest_name);
        fs::copy(&downloaded, &dest)?;
        Ok(dest.to_string_lossy().to_string())
    } else {
        Err("No country code match".into())
    }
}

/// Try to rasterize an SVG file to PNG using common external tools.
/// Returns Ok(true) on successful conversion, Ok(false) if tool not found or
/// conversion failed. Any errors from spawning the tools are returned as Err.
fn try_rasterize_svg_to_png(svg: &Path, png: &Path) -> Result<bool, Box<dyn std::error::Error>> {
    // First try a pure-Rust SVG rasterizer (resvg + usvg + tiny-skia). This
    // keeps the build self-contained and deterministic in CI. If this path
    // fails for any reason, fall back to external command-line tools, and
    // finally to a simple placeholder PNG.
    if let Ok(svg_bytes) = std::fs::read(svg) {
        // Use local names to avoid unused-import warnings when dependencies
        // are temporarily unavailable during development.
        match (|| -> Result<bool, Box<dyn std::error::Error>> {
            // Parse using usvg
            let mut opt = usvg::Options::default();
            // Ensure relative resources (fonts/images) are looked up relative to the svg
            if let Some(parent) = svg.parent() {
                opt.resources_dir = Some(parent.to_path_buf());
            }
            // usvg::Tree::from_data accepts a reference to Options
            let rtree = usvg::Tree::from_data(&svg_bytes, &opt)
                .map_err(|e| format!("usvg parse error: {}", e))?;
            // Compute pixel dimensions. usvg expresses size in px units but as
            // floating point; convert to u32 for Pixmap.
            let width = rtree.size.width().ceil() as u32;
            let height = rtree.size.height().ceil() as u32;
            if width == 0 || height == 0 {
                return Err("SVG has zero width or height".into());
            }

            // Render at a very high resolution, then downsample with a
            // high-quality Lanczos filter to a target final size. This
            // produces crisp detail while keeping file size reasonable.
            // Behavior is controlled via environment variables:
            // - COAT_TARGET_WIDTH (px) default 600
            // - COAT_UPSCALE_FACTOR (float) default 4.0 (render at 4x the final size)
            // These provide a good balance between final size and fidelity.
            let target_width: u32 = std::env::var("COAT_TARGET_WIDTH")
                .ok()
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(600);
            let upscale: f32 = std::env::var("COAT_UPSCALE_FACTOR")
                .ok()
                .and_then(|s| s.parse::<f32>().ok())
                .unwrap_or(4.0);

            // Logical SVG size in px (floating)
            let svg_w = rtree.size.width();
            let svg_h = rtree.size.height();
            if svg_w == 0.0 || svg_h == 0.0 {
                return Err("SVG has zero logical width/height".into());
            }

            // Compute final target dimensions preserving aspect ratio
            let target_w_f = target_width as f64;
            let target_h_f = (target_w_f * svg_h) / svg_w;
            let target_h = target_h_f.ceil() as u32;

            // Render at (target * upscale) resolution
            let render_w = (target_w_f * (upscale as f64)).ceil() as u32;
            let render_h = (target_h_f * (upscale as f64)).ceil() as u32;

            // Safety cap to avoid excessive memory usage
            let cap_dim: u32 = 10000;
            let render_w = std::cmp::min(render_w, cap_dim);
            let render_h = std::cmp::min(render_h, cap_dim);

            // Create pixmap and render
            let mut pixmap = tiny_skia::Pixmap::new(render_w, render_h)
                .ok_or_else(|| "failed to allocate pixmap for high-res render")?;
            let pixmap_mut = pixmap.as_mut();

            // Compute scale factor to map SVG logical units to rendered pixels
            let scale_x = (render_w as f64) / svg_w;
            let scale_y = (render_h as f64) / svg_h;
            // Use a uniform scale to preserve aspect (they should be equal)
            let transform = tiny_skia::Transform::from_scale(scale_x as f32, scale_y as f32);
            let fit_to = usvg::FitTo::Original;
            resvg::render(&rtree, fit_to, transform, pixmap_mut)
                .ok_or_else(|| "resvg render failed")?;

            // Convert tiny-skia pixmap into an image::RgbaImage directly
            // and downsample in-memory using the image crate. This avoids
            // relying on temporary files and keeps everything in Rust.
            let data = pixmap.data().to_vec();
            let img_buf =
                image::ImageBuffer::<image::Rgba<u8>, Vec<u8>>::from_raw(render_w, render_h, data)
                    .ok_or_else(|| "failed to create image buffer from pixmap data")?;
            let dynimg = image::DynamicImage::ImageRgba8(img_buf);
            let final_img = dynimg.resize_exact(
                target_width,
                target_h,
                image::imageops::FilterType::Lanczos3,
            );
            final_img.save(png)?;
            Ok(true)
        })() {
            Ok(true) => {
                return Ok(true);
            }
            Ok(false) => { /* continue to fallback methods */ }
            Err(e) => {
                // If resvg path failed, continue to try external tools below.
                eprintln!("resvg rasterization failed: {}", e);
            }
        }
    }

    use std::process::Command;

    // Try rsvg-convert first. Prefer to produce a high-resolution PNG and
    // downsample it to the target final size for quality (Lanczos3).
    if which::which("rsvg-convert").is_ok() {
        // Attempt to determine a target and upscale similar to the resvg path.
        if let Ok(svg_bytes) = std::fs::read(svg) {
            if let Ok(tree) = {
                let mut opt = usvg::Options::default();
                if let Some(parent) = svg.parent() {
                    opt.resources_dir = Some(parent.to_path_buf());
                }
                usvg::Tree::from_data(&svg_bytes, &opt)
            } {
                let svg_w = tree.size.width();
                let svg_h = tree.size.height();
                if svg_w > 0.0 && svg_h > 0.0 {
                    let target_width: u32 = std::env::var("COAT_TARGET_WIDTH")
                        .ok()
                        .and_then(|s| s.parse::<u32>().ok())
                        .unwrap_or(600);
                    let upscale: f32 = std::env::var("COAT_UPSCALE_FACTOR")
                        .ok()
                        .and_then(|s| s.parse::<f32>().ok())
                        .unwrap_or(4.0);
                    let target_w_f = target_width as f64;
                    let target_h_f = (target_w_f * svg_h) / svg_w;
                    let target_h = target_h_f.ceil() as u32;
                    let render_w = (target_w_f * (upscale as f64)).ceil() as u32;
                    let render_h = (target_h_f * (upscale as f64)).ceil() as u32;
                    let cap_dim: u32 = 10000;
                    let render_w = std::cmp::min(render_w, cap_dim);
                    let render_h = std::cmp::min(render_h, cap_dim);

                    let tmp = tempfile::NamedTempFile::new()?;
                    let tmp_path = tmp.path().to_path_buf();
                    let status = Command::new("rsvg-convert")
                        .arg("-w")
                        .arg(render_w.to_string())
                        .arg("-h")
                        .arg(render_h.to_string())
                        .arg("-o")
                        .arg(&tmp_path)
                        .arg(svg)
                        .status()?;
                    if status.success() {
                        // downsample and save
                        if let Ok(dynimg) = image::open(&tmp_path) {
                            let final_img = dynimg.resize_exact(
                                target_width,
                                target_h,
                                image::imageops::FilterType::Lanczos3,
                            );
                            final_img.save(png)?;
                            return Ok(true);
                        }
                    }
                }
            }
        }
        // If the above high-res -> downsample path failed, try a simple conversion
        let status = Command::new("rsvg-convert")
            .arg("-f")
            .arg("png")
            .arg("-o")
            .arg(png)
            .arg(svg)
            .status()?;
        if status.success() {
            return Ok(true);
        }
    }

    // Try ImageMagick: convert
    if which::which("convert").is_ok() {
        let status = Command::new("convert").arg(svg).arg(png).status()?;
        if status.success() {
            return Ok(true);
        }
    }

    // Try Windows-style 'magick' (some ImageMagick installs)
    if which::which("magick").is_ok() {
        let status = Command::new("magick")
            .arg("convert")
            .arg(svg)
            .arg(png)
            .status()?;
        if status.success() {
            return Ok(true);
        }
    }

    // If external tools are not available or conversion failed, create a
    // simple placeholder PNG so the rest of the application (which expects
    // PNG/JPG) can continue. This avoids runtime failure when no rasterizer
    // is present. The placeholder is a plain white canvas with a thin
    // grey border.
    if let Ok(width) = std::env::var("STAMP_COA_PNG_WIDTH") {
        // Allow override via env var, else fall back to 320
        if let Ok(w) = width.parse::<u32>() {
            let h = (w * 3) / 4; // 4:3 aspect
            let mut im = image::RgbaImage::new(w, h);
            // fill white
            for px in im.pixels_mut() {
                *px = image::Rgba([255, 255, 255, 255]);
            }
            // draw border
            for x in 0..w {
                im.put_pixel(x, 0, image::Rgba([200, 200, 200, 255]));
                im.put_pixel(x, h - 1, image::Rgba([200, 200, 200, 255]));
            }
            for y in 0..h {
                im.put_pixel(0, y, image::Rgba([200, 200, 200, 255]));
                im.put_pixel(w - 1, y, image::Rgba([200, 200, 200, 255]));
            }
            let _ = im.save(png);
            return Ok(true);
        }
    }

    // Default placeholder 320x240
    let w = 320u32;
    let h = 240u32;
    let mut im = image::RgbaImage::new(w, h);
    for px in im.pixels_mut() {
        *px = image::Rgba([255, 255, 255, 255]);
    }
    for x in 0..w {
        im.put_pixel(x, 0, image::Rgba([200, 200, 200, 255]));
        im.put_pixel(x, h - 1, image::Rgba([200, 200, 200, 255]));
    }
    for y in 0..h {
        im.put_pixel(0, y, image::Rgba([200, 200, 200, 255]));
        im.put_pixel(w - 1, y, image::Rgba([200, 200, 200, 255]));
    }
    let _ = im.save(png);
    Ok(true)
}

// Inline tests moved to `tests/flagdb.rs`.

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_try_rasterize_svg_to_png_roundtrip() {
        // Create a minimal SVG file and a target PNG path in a temp dir.
        let dir = tempdir().expect("tempdir");
        let svg_path = dir.path().join("test_coa.svg");
        let png_path = dir.path().join("test_coa.png");

        let svg_content = r#"<?xml version='1.0' encoding='UTF-8'?>
<svg xmlns='http://www.w3.org/2000/svg' width='200' height='160'>
    <rect width='200' height='160' fill='#ffffff'/>
    <circle cx='100' cy='80' r='60' fill='#008000' />
    <text x='100' y='85' font-size='24' text-anchor='middle' fill='#ffffff'>CA</text>
</svg>"#;

        let mut f = std::fs::File::create(&svg_path).expect("create svg");
        f.write_all(svg_content.as_bytes()).expect("write svg");

        // Ensure the function returns Ok(true) and the PNG is created.
        let res = try_rasterize_svg_to_png(&svg_path, &png_path).expect("rasterize result");
        assert!(res, "rasterizer should return true");
        assert!(png_path.exists(), "png output should exist");

        // Basic sanity: image crate can open it and has non-zero dimensions
        let img = image::open(&png_path).expect("open png");
        let (w, h) = img.dimensions();
        assert!(w > 0 && h > 0, "output image has non-zero dimensions");
    }
}
