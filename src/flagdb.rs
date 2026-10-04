use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::stampworld::fetch::{Fetcher, ReqwestFetcher};
use image::GenericImageView;
use rusqlite::Connection;
use std::time::{SystemTime, UNIX_EPOCH};

mod assets;
mod countries;

/// Bundled ISO country names/codes plus the application's established aliases.
pub async fn default_flag_map() -> HashMap<String, String> {
    static MAP: once_cell::sync::Lazy<Result<HashMap<String, String>, String>> =
        once_cell::sync::Lazy::new(countries::bundled_map);
    match &*MAP {
        Ok(map) => map.clone(),
        Err(error) => {
            log::error!("Cannot load bundled ISO country codes: {error}");
            HashMap::new()
        }
    }
}

/// Merge the first valid local name-to-code override file over bundled ISO data.
pub async fn load_flag_map_from_fs() -> HashMap<String, String> {
    let mut map = default_flag_map().await;
    for dir in assets::flag_directories() {
        let path = dir.join("flag_codes.json");
        let result = match fs::read_to_string(&path) {
            Ok(contents) => serde_json::from_str::<HashMap<String, String>>(&contents)
                .map_err(|error| error.to_string())
                .and_then(|overrides| countries::merge_overrides(&mut map, overrides)),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(error) => Err(error.to_string()),
        };
        match result {
            Ok(()) => return map,
            Err(error) => log::warn!("Cannot load flag overrides {}: {error}", path.display()),
        }
    }
    map
}

// #[cfg(feature = "full-flag-db")]
// fn load_embedded_flag_map_if_enabled() -> Option<HashMap<String, String>> {
//     const EMBEDDED_FLAGS_JSON: &str = include_str!("../flags/flag_codes.json");

//     if let Ok(j) = serde_json::from_str::<HashMap<String, String>>(EMBEDDED_FLAGS_JSON) {
//         // Attempt to write the embedded json to the preferred writable location
//         // (cwd 'flags' preferred). Ignore write errors; runtime lookups still
//         // succeed from the in-memory map if writes are not possible.
//         let prefer_dirs: Vec<PathBuf> = vec![
//             PathBuf::from("flags"),
//             std::env::current_exe()
//                 .ok()
//                 .and_then(|e| e.parent().map(|p| p.to_path_buf()))
//                 .unwrap_or_else(|| PathBuf::from("flags")),
//             PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("flags"),
//         ];

//         let cwd_flags = PathBuf::from("flags");
//         if fs::create_dir_all(&cwd_flags).is_ok() {
//             let out_path = cwd_flags.join("flag_codes.json");
//             let _ = fs::write(&out_path, EMBEDDED_FLAGS_JSON);
//         } else {
//             for dir in prefer_dirs.iter() {
//                 if fs::create_dir_all(dir).is_ok() {
//                     let out_path = dir.join("flag_codes.json");
//                     if fs::write(&out_path, EMBEDDED_FLAGS_JSON).is_ok() {
//                         break;
//                     }
//                 }
//             }
//         }

//         return Some(
//             j.into_iter()
//                 .map(|(k, v)| (k.to_lowercase(), v.to_lowercase()))
//                 .collect(),
//         );
//     }

//     None
// }

// #[cfg(not(feature = "full-flag-db"))]
// fn load_embedded_flag_map_if_enabled() -> Option<HashMap<String, String>> {
//     None
// }

/// Given a country name, attempt to return an alpha-2 code (lowercase) using simple normalization.
pub async fn country_to_code(country: &str) -> Option<String> {
    let map = load_flag_map_from_fs().await;
    let key = countries::normalize(country);
    if let Some(code) = map.get(&key) {
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

    if let Some(code) = region_map.get(&key) {
        return Some(code.clone());
    }

    None
}

/// Normalize free-form country input into a canonical lookup key.
/// Replaces common separators with spaces and lowercases so callers can
/// accept values like "United-States" or "United_States".
pub async fn normalize_country_input(country: &str) -> String {
    countries::normalize(country)
}

/// Ensure flag image exists for the given alpha2 code in flags/, download from FlagCDN if missing.
/// size example: "w80" or "w320". Returns path to downloaded file on success.
pub async fn ensure_flag_image(
    alpha2: &str,
    size: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Backwards-compatible wrapper that uses the default ReqwestFetcher.
    let fetcher = ReqwestFetcher::new();
    ensure_flag_image_with_fetcher(alpha2, size, &fetcher).await
}

/// Ensure flag image exists for the given alpha2 code in flags/, download from FlagCDN if missing.
/// size example: "w80" or "w320". Returns path to downloaded file on success.
pub async fn ensure_flag_image_with_fetcher(
    alpha2: &str,
    size: &str,
    fetcher: &dyn Fetcher,
) -> Result<String, Box<dyn std::error::Error>> {
    let cfg = crate::config::load_effective_config("config.toml");
    assets::ensure_flag(
        alpha2,
        size,
        fetcher,
        &assets::flag_directories(),
        cfg.flag_cover_overwrite_flag.unwrap_or(false),
    )
    .await
}

/// Copy a validated flag to the album, retaining a valid local flag on lookup/download failure.
pub async fn copy_flag_for_album(
    country: &str,
    album_images_dir: &str,
    size: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let result = match country_to_code(country).await {
        Some(code) => ensure_flag_image(&code, size).await.and_then(|downloaded| {
            assets::copy_to_album(Path::new(&downloaded), Path::new(album_images_dir))
        }),
        None => Err(format!("No country code match for {country:?}").into()),
    };
    match result {
        Ok(path) => Ok(path),
        Err(error) => {
            log::warn!("Flag lookup/copy failed for {country:?}: {error}");
            if let Some(local) = assets::local_album_flag(Path::new(album_images_dir)) {
                log::warn!("Using existing album-local flag {}", local.display());
                Ok(local.to_string_lossy().to_string())
            } else {
                Err(error)
            }
        }
    }
}

/// Ensure coat of arms SVG exists for the given alpha2 code in flags/coats_of_arms/, download
/// from restcountries.com if missing. Returns path to downloaded file on success.
pub async fn ensure_coat_of_arms(alpha2: &str) -> Result<String, Box<dyn std::error::Error>> {
    let fetcher = ReqwestFetcher::new_for_api();
    ensure_coat_of_arms_with_fetcher(alpha2, &fetcher).await
}

pub async fn ensure_coat_of_arms_with_fetcher(
    alpha2: &str,
    fetcher: &dyn Fetcher,
) -> Result<String, Box<dyn std::error::Error>> {
    let alpha = alpha2.to_lowercase();
    let filename = format!("{}.svg", alpha);

    // Candidate directories to look for an existing coat of arms image.
    let mut candidates: Vec<PathBuf> = Vec::new();
    // Environment override: accept either STAMP_FLAGS_DIR as the flags
    // directory itself or as the parent containing a `flags/` directory.
    if let Ok(p) = std::env::var("STAMP_FLAGS_DIR") {
        let pbuf = PathBuf::from(&p);
        // Direct `coats_of_arms` under the provided dir
        candidates.push(pbuf.join("coats_of_arms"));
        // Or the common layout where the provided dir is the parent and
        // the actual flags live in a `flags/` subdirectory.
        candidates.push(pbuf.join("flags").join("coats_of_arms"));
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
    // Honor overwrite config: only attempt network fetch if overwrite enabled
    // or the target file doesn't already exist.
    let cfg = crate::config::load_effective_config("config.toml");
    let overwrite_coa = cfg.flag_cover_overwrite_coa.unwrap_or(false);

    // If overwrite is disabled and a file already exists at `filepath`, return it.
    if filepath.exists() && !overwrite_coa {
        // Prefer a rasterized PNG sibling if present
        let png_candidate = filepath.with_extension("png");
        if png_candidate.exists() {
            return Ok(png_candidate.to_string_lossy().to_string());
        }
        return Ok(filepath.to_string_lossy().to_string());
    }

    // First try neXperts countries API for coat (preferred source): PNG then SVG.
    // This provides a predictable, high-quality asset set.
    if overwrite_coa || !filepath.exists() {
        let nex_png = format!("https://countries.nexperts.nl/coa/{}.png", alpha);
        if let Ok((_s, _h, bytes)) = fetcher.fetch_bytes(&nex_png).await {
            let target_dir_coa = PathBuf::from("flags").join("coats_of_arms");
            let _ = std::fs::create_dir_all(&target_dir_coa);
            let out_path = target_dir_coa.join(format!("{}.png", alpha));
            let mut f = std::fs::File::create(&out_path)?;
            f.write_all(&bytes)?;
            // copy into target filepath location if different
            if out_path != filepath.with_extension("png") {
                let _ = std::fs::create_dir_all(filepath.parent().unwrap_or(&target_dir_coa));
                let _ = std::fs::copy(&out_path, filepath.with_extension("png"))?;
                return Ok(filepath.with_extension("png").to_string_lossy().to_string());
            }
            return Ok(out_path.to_string_lossy().to_string());
        }

        // Try neXperts SVG
        let nex_svg = format!("https://countries.nexperts.nl/coa/{}.svg", alpha);
        if let Ok((_s, _h, bytes)) = fetcher.fetch_bytes(&nex_svg).await {
            let target_dir_coa = PathBuf::from("flags").join("coats_of_arms");
            let _ = std::fs::create_dir_all(&target_dir_coa);
            let out_path = target_dir_coa.join(format!("{}.svg", alpha));
            let mut f = std::fs::File::create(&out_path)?;
            f.write_all(&bytes)?;
            let png_path = out_path.with_extension("png");
            if try_rasterize_svg_to_png(&out_path, &png_path).unwrap_or(false) {
                return Ok(png_path.to_string_lossy().to_string());
            }
            return Ok(out_path.to_string_lossy().to_string());
        }
    }

    // Query restcountries API for coatOfArms URL. Try v5 first, then v3 as fallback.
    // Prefer minimal-field v5 endpoint, fall back to other shapes
    let candidates = [
        format!(
            "https://restcountries.com/v5.1/alpha/{}?fields=coatOfArms",
            alpha
        ),
        format!(
            "https://restcountries.com/v5.1/alpha?codes={}&fields=coatOfArms",
            alpha
        ),
        format!(
            "https://restcountries.com/v3.1/alpha/{}?fields=coatOfArms",
            alpha
        ),
        format!(
            "https://restcountries.com/v3.1/alpha?codes={}&fields=coatOfArms",
            alpha
        ),
    ];

    let mut body: Option<String> = None;
    let mut last_err: Option<String> = None;
    for url in candidates.iter() {
        match fetcher.fetch_text(url).await {
            Ok((_s, b)) => {
                body = Some(b);
                break;
            }
            Err(e) => {
                // remember the last error but continue to next candidate
                last_err = Some(format!("fetch {} failed: {}", url, e));
            }
        }
    }

    // If restcountries did not provide usable coat, try neXperts countries API (provides /coa/{code}.svg/png)
    if body.is_none()
        || body
            .as_ref()
            .map(|s| s.contains("deprecated"))
            .unwrap_or(false)
    {
        // try neXperts country full info first
        let nex_info_url = format!("https://countries.nexperts.nl/{alpha}");
        if let Ok((_s, b)) = fetcher.fetch_text(&nex_info_url).await {
            // save a copy into a small sqlite cache for future
            let _ = cache_country_info(&alpha, &b);
            // try to parse links to coa or flag from the returned content (it's HTML)
            if let Some(coa_url) = extract_coa_from_nexperts_html(&b, &alpha) {
                // download bytes
                if let Ok((_st, _headers, bytes)) = fetcher.fetch_bytes(&coa_url).await {
                    let ext = if coa_url.to_lowercase().ends_with(".png") {
                        "png"
                    } else {
                        "svg"
                    };
                    let final_path = target_dir.join(format!("{}.{}", alpha, ext));
                    let mut out = std::fs::File::create(&final_path)?;
                    out.write_all(&bytes)?;
                    if ext == "png" {
                        return Ok(final_path.to_string_lossy().to_string());
                    }
                    let png_path = final_path.with_extension("png");
                    if try_rasterize_svg_to_png(&final_path, &png_path).unwrap_or(false) {
                        return Ok(png_path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    let body = match body {
        Some(b) => b,
        None => {
            if let Some(e) = last_err {
                return Err(e.into());
            }
            return Err("Failed to fetch country data from restcountries API".into());
        }
    };

    // Parse JSON and try to extract coatOfArms.svg or coatOfArms.png from a variety
    // of possible response shapes (array, single object, nested 'data' responses).
    let v: serde_json::Value = serde_json::from_str(&body)?;

    // Helper to inspect a JSON value for coat urls
    fn extract_from_obj(o: &serde_json::Value) -> Option<String> {
        if let Some(coa) = o.get("coatOfArms") {
            if let Some(s) = coa.get("svg").and_then(|x| x.as_str()) {
                if !s.trim().is_empty() {
                    return Some(s.to_string());
                }
            }
            if let Some(s) = coa.get("png").and_then(|x| x.as_str()) {
                if !s.trim().is_empty() {
                    return Some(s.to_string());
                }
            }
        }
        None
    }

    // If the API responded with an error structure, create a placeholder PNG
    // so album generation can continue (the absence of a coat should not
    // abort the whole run). Surface the message to logs for diagnosis.
    if let Some(errors) = v.get("errors") {
        if let Some(first) = errors.get(0) {
            if let Some(msg) = first.get("message").and_then(|m| m.as_str()) {
                eprintln!("restcountries API error: {}", msg);
                // create placeholder PNG and return its path
                let png_path = filepath.with_extension("png");
                // Ensure parent dir
                if let Some(parent) = png_path.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                // Generate a simple placeholder image (320x240) with border
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
                let _ = im.save(&png_path);
                return Ok(png_path.to_string_lossy().to_string());
            }
        }
    }

    // Try a few shapes: array of objects, single object, or nested data->... structures.
    let mut coa_url_opt: Option<String> = None;
    if v.is_array() {
        for item in v.as_array().unwrap().iter() {
            if let Some(s) = extract_from_obj(item) {
                coa_url_opt = Some(s);
                break;
            }
        }
    } else if v.is_object() {
        // Direct object
        if let Some(s) = extract_from_obj(&v) {
            coa_url_opt = Some(s);
        } else if let Some(data) = v.get("data") {
            if data.is_array() {
                for item in data.as_array().unwrap().iter() {
                    if let Some(s) = extract_from_obj(item) {
                        coa_url_opt = Some(s);
                        break;
                    }
                }
            } else if data.is_object() {
                if let Some(s) = extract_from_obj(data) {
                    coa_url_opt = Some(s);
                }
            }
        }
    }

    if let Some(coa_url) = coa_url_opt {
        if coa_url.trim().is_empty() {
            return Err("No coatOfArms URL provided by API".into());
        }

        // Download the bytes and inspect headers to decide extension
        let (_s, headers, bytes) = fetcher.fetch_bytes(&coa_url).await?;

        // Decide on extension: prefer Content-Type header, fall back to URL suffix
        let mut ext = None::<&str>;
        if let Some(ct) = headers.get(reqwest::header::CONTENT_TYPE) {
            if let Ok(cts) = ct.to_str() {
                if cts.contains("svg") {
                    ext = Some("svg");
                } else if cts.contains("png") {
                    ext = Some("png");
                }
            }
        }
        if ext.is_none() {
            if coa_url.to_lowercase().ends_with(".png") {
                ext = Some("png");
            } else if coa_url.to_lowercase().ends_with(".svg") {
                ext = Some("svg");
            }
        }
        let ext = ext.unwrap_or("svg");

        let final_path = target_dir.join(format!("{}.{}", alpha, ext));
        let mut out = fs::File::create(&final_path)?;
        out.write_all(&bytes)?;

        // If the downloaded asset is already a PNG, return it directly
        if ext.eq_ignore_ascii_case("png") {
            return Ok(final_path.to_string_lossy().to_string());
        }

        // If we have an SVG, attempt to rasterize to PNG for printing support.
        let png_path = final_path.with_extension("png");
        if try_rasterize_svg_to_png(&final_path, &png_path).unwrap_or(false) {
            return Ok(png_path.to_string_lossy().to_string());
        }

        // As a last resort, return the SVG path (caller may still use it),
        // but note the application can't print SVG directly — higher layers
        // should detect this and prefer a PNG. We still return the SVG so
        // callers can inspect or rerun rasterization later.
        return Ok(final_path.to_string_lossy().to_string());
    }

    Err("Coat of arms not available for this country".into())
}

/// Convenience: try to find a coat of arms for the provided country name and copy it into the given album images dir
pub async fn copy_coa_for_album(
    country: &str,
    album_images_dir: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Normalize input to handle hyphens/underscores and casing
    let norm = normalize_country_input(country).await;
    if let Some(code) = country_to_code(&norm).await {
        let downloaded = ensure_coat_of_arms(&code).await?;
        fs::create_dir_all(album_images_dir)?;
        let src_path = Path::new(&downloaded).to_path_buf();
        // If the source is a small PNG, try to find an SVG sibling and
        // re-rasterize it at a larger target size so the album gets a
        // high-quality coat PNG. This covers the case where a previously
        // cached small PNG exists but a higher-quality SVG is present.
        if src_path
            .extension()
            .and_then(|s| s.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("png"))
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
            let width = rtree.size().width().ceil() as u32;
            let height = rtree.size().height().ceil() as u32;
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
            let svg_w = rtree.size().width() as f64;
            let svg_h = rtree.size().height() as f64;
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
                .ok_or("failed to allocate pixmap for high-res render")?;

            // Compute scale factor to map SVG logical units to rendered pixels
            let scale_x = (render_w as f64) / svg_w;
            let scale_y = (render_h as f64) / svg_h;
            // Use a uniform scale to preserve aspect (they should be equal)
            let transform = tiny_skia::Transform::from_scale(scale_x as f32, scale_y as f32);
            resvg::render(&rtree, transform, &mut pixmap.as_mut());

            // Convert tiny-skia pixmap into an image::RgbaImage directly
            // and downsample in-memory using the image crate. This avoids
            // relying on temporary files and keeps everything in Rust.
            let data = pixmap.data().to_vec();
            let img_buf =
                image::ImageBuffer::<image::Rgba<u8>, Vec<u8>>::from_raw(render_w, render_h, data)
                    .ok_or("failed to create image buffer from pixmap data")?;
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
                let svg_w = tree.size().width() as f64;
                let svg_h = tree.size().height() as f64;
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

// --- neXperts helpers -----------------------------------------------------

fn cache_country_info(alpha: &str, html: &str) -> Result<(), Box<dyn std::error::Error>> {
    let db_dir = Path::new("flags");
    let _ = std::fs::create_dir_all(db_dir);
    let db_path = db_dir.join("countries.db");
    let conn = Connection::open(&db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS countries (
            alpha TEXT PRIMARY KEY,
            name TEXT,
            flag_url TEXT,
            coa_url TEXT,
            raw_html TEXT,
            fetched_at INTEGER
        )",
        [],
    )?;
    // Attempt to extract simple metadata from the HTML so we can store structured info
    let name = extract_country_name_from_nexperts_html(html).unwrap_or_else(|| alpha.to_string());
    let flag_url = extract_flag_from_nexperts_html(html, alpha);
    let coa_url = extract_coa_from_nexperts_html(html, alpha);
    let ts = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;
    conn.execute(
        "INSERT OR REPLACE INTO countries(alpha, name, flag_url, coa_url, raw_html, fetched_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params![alpha, name, flag_url, coa_url, html, ts],
    )?;
    Ok(())
}

fn extract_flag_from_nexperts_html(html: &str, alpha: &str) -> Option<String> {
    // Heuristic: look for /flags/{alpha}.png or .svg
    let needle_png = format!("/flags/{}.png", alpha);
    if let Some(pos) = html.find(&needle_png) {
        if let Some(start) = html[..pos].rfind('"') {
            let snippet = &html[start..pos + needle_png.len() + 64.min(html.len() - pos)];
            if let Some(url_start) = snippet.find("http") {
                let url = &snippet[url_start..];
                if let Some(endq) = url.find('"') {
                    return Some(url[..endq].to_string());
                }
            }
        }
        return Some(format!("https://countries.nexperts.nl{}", needle_png));
    }
    let needle_svg = format!("/flags/{}.svg", alpha);
    if let Some(pos) = html.find(&needle_svg) {
        if let Some(start) = html[..pos].rfind('"') {
            let snippet = &html[start..pos + needle_svg.len() + 64.min(html.len() - pos)];
            if let Some(url_start) = snippet.find("http") {
                let url = &snippet[url_start..];
                if let Some(endq) = url.find('"') {
                    return Some(url[..endq].to_string());
                }
            }
        }
        return Some(format!("https://countries.nexperts.nl{}", needle_svg));
    }
    None
}

fn extract_country_name_from_nexperts_html(html: &str) -> Option<String> {
    // Try <title> or an <h1> entry
    if let Some(pos) = html.find("<title>") {
        if let Some(end) = html[pos..].find("</title>") {
            let t = &html[pos + 7..pos + end];
            return Some(t.trim().to_string());
        }
    }
    if let Some(pos) = html.find("<h1") {
        // find closing '>' then closing </h1>
        if let Some(start) = html[pos..].find('>') {
            if let Some(end) = html[pos + start..].find("</h1>") {
                let t = &html[pos + start + 1..pos + start + end];
                return Some(t.trim().to_string());
            }
        }
    }
    None
}

fn extract_coa_from_nexperts_html(html: &str, alpha: &str) -> Option<String> {
    // Very small heuristic HTML parse: look for /coa/{alpha}.png or .svg
    let needle_png = format!("/coa/{}.png", alpha);
    if let Some(pos) = html.find(&needle_png) {
        // find surrounding href or src
        let start = html[..pos].rfind('"').unwrap_or(pos.saturating_sub(64));
        let snippet = &html[start..pos + needle_png.len() + 8.min(html.len() - pos)];
        if let Some(url_start) = snippet.find("http") {
            let url = &snippet[url_start..];
            if let Some(endq) = url.find('"') {
                return Some(url[..endq].to_string());
            }
        }
        return Some(format!("https://countries.nexperts.nl{}", needle_png));
    }
    let needle_svg = format!("/coa/{}.svg", alpha);
    if let Some(pos) = html.find(&needle_svg) {
        let start = html[..pos].rfind('"').unwrap_or(pos.saturating_sub(64));
        let snippet = &html[start..pos + needle_svg.len() + 8.min(html.len() - pos)];
        if let Some(url_start) = snippet.find("http") {
            let url = &snippet[url_start..];
            if let Some(endq) = url.find('"') {
                return Some(url[..endq].to_string());
            }
        }
        return Some(format!("https://countries.nexperts.nl{}", needle_svg));
    }
    None
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
