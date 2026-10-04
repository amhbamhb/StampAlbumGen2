//! # Image Downloader Module
//!
//! This module provides functionality to download stamp images from remote URLs
//! when they are missing from the local filesystem.

use log::{debug, info, warn};
use reqwest;
use std::fs;
use std::path::Path;

/// Downloads an image from a remote URL to the local filesystem
///
/// # Arguments
/// * `url` - The remote URL of the image
/// * `local_path` - The local path where to save the image
///
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Success or error
pub async fn download_image(
    url: &str,
    local_path: &Path,
    sg: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    debug!(
        "[DOWNLOAD_ATTEMPT] sg={} url={} -> {}",
        sg,
        url,
        local_path.display()
    );

    // Create the directory if it doesn't exist
    if let Some(parent) = local_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Build a small client that looks like a browser to reduce anti-bot responses
    // Use the centralized fetch image client and fetch_bytes helper so tests
    // can inject a fake fetcher if needed.
    let client = crate::stampworld::fetch::build_image_client();
    let (status, headers, bytes) = match crate::stampworld::fetch::fetch_bytes(&client, url).await {
        Ok((s, h, b)) => (s, h, b),
        Err(e) => {
            warn!(
                "[DOWNLOAD_FAIL] sg={} url={} error=failed_request:{}",
                sg, url, e
            );
            return Err(Box::new(e));
        }
    };

    if !status.is_success() {
        warn!(
            "[DOWNLOAD_FAIL] sg={} url={} error=status:{}",
            sg,
            url,
            status.as_u16()
        );
        return Err(format!("HTTP status {}", status.as_u16()).into());
    }

    // Check content-type header to avoid saving HTML error pages
    if let Some(ct) = headers.get(reqwest::header::CONTENT_TYPE) {
        if let Ok(ct_str) = ct.to_str() {
            if !ct_str.starts_with("image/") {
                warn!(
                    "[DOWNLOAD_FAIL] sg={} url={} error=unexpected_content_type:{}",
                    sg, url, ct_str
                );
                return Err(format!("unexpected content-type: {}", ct_str).into());
            }
        }
    }

    // Validate bytes decode as an image before writing to disk
    if image::load_from_memory(&bytes).is_err() {
        warn!(
            "[DOWNLOAD_FAIL] sg={} url={} error=invalid_image_bytes size={}",
            sg,
            url,
            bytes.len()
        );
        return Err("downloaded bytes did not decode as an image".into());
    }

    // Save to file
    if let Err(e) = fs::write(local_path, &bytes) {
        warn!(
            "[DOWNLOAD_FAIL] sg={} url={} error=write_file:{}",
            sg, url, e
        );
        return Err(Box::new(e));
    }

    info!(
        "[DOWNLOAD_OK] sg={} url={} status={} saved={} size={}",
        sg,
        url,
        status.as_u16(),
        local_path.display(),
        bytes.len()
    );
    Ok(())
}

/// Downloads a stamp image if it's missing but has a remote URL
///
/// # Arguments
/// * `image_path` - The local image path (relative to album images directory)
/// * `remote_url` - The remote URL to download from
/// * `album_name` - The album name for constructing the full local path
///
/// # Returns
/// * `Result<String, Box<dyn std::error::Error>>` - The final image path or error
pub async fn download_stamp_image_if_missing(
    image_path: &str,
    remote_url: &str,
    album_name: &str,
    sg: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    if remote_url.is_empty() {
        return Ok(image_path.to_string());
    }

    let local_path = crate::file_utils::resolve_album_image_path(album_name, image_path);

    // Check if image already exists
    if local_path.exists() {
        return Ok(image_path.to_string());
    }

    // Try to download the image (instrumented)
    match download_image(remote_url, &local_path, sg).await {
        Ok(_) => {
            // Ensure file now exists
            if local_path.exists() {
                Ok(image_path.to_string())
            } else {
                Err("download reported OK but file not found".into())
            }
        }
        Err(e) => {
            warn!(
                "[DOWNLOAD_FAIL_FINAL] sg={} url={} error={}",
                sg, remote_url, e
            );
            Err(e)
        }
    }
}

/// Downloads all missing stamp images for a collection of stamps
///
/// # Arguments
/// * `stamps` - Vector of stamp data
/// * `album_name` - The album name
///
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Success or error
pub async fn download_missing_stamp_images(
    stamps: &[crate::db_utils::StampStub],
    album_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Checking for missing images to download...");

    let mut downloaded_count = 0;
    let mut total_missing = 0;

    for stamp in stamps {
        if !stamp.image_path.is_empty() && !stamp.remote_image_url.is_empty() {
            let local_path =
                crate::file_utils::resolve_album_image_path(album_name, &stamp.image_path);

            if !local_path.exists() {
                total_missing += 1;
                match download_stamp_image_if_missing(
                    &stamp.image_path,
                    &stamp.remote_image_url,
                    album_name,
                    &stamp.sg_number,
                )
                .await
                {
                    Ok(_) => {
                        downloaded_count += 1;
                    }
                    Err(e) => {
                        warn!(
                            "Failed to download image for stamp {}: {}",
                            stamp.sg_number, e
                        );
                    }
                }
            }
        }
    }

    if total_missing > 0 {
        info!(
            "Downloaded {}/{} missing images",
            downloaded_count, total_missing
        );
    } else {
        info!("All images are already available locally");
    }

    Ok(())
}

/// Normalize images in an album's `images/` directory for PDF generation.
/// This re-opens and re-saves each image which removes embedded ICC profiles
/// and other metadata that can confuse Qt's color management. Returns the
/// number of files normalized (successful rewrites).
pub fn normalize_album_images_for_pdf(
    album_name: &str,
) -> Result<usize, Box<dyn std::error::Error>> {
    let images_dir = crate::file_utils::resolve_album_dir(album_name).join("images");
    if !images_dir.exists() {
        return Ok(0);
    }

    let mut count = 0usize;
    for entry in std::fs::read_dir(&images_dir)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        // Only attempt common image extensions
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            let lower = ext.to_ascii_lowercase();
            if lower != "png"
                && lower != "jpg"
                && lower != "jpeg"
                && lower != "webp"
                && lower != "gif"
            {
                continue;
            }
        } else {
            continue;
        }

        // Try to open and re-save the image. Use a temp file then rename to avoid corruption.
        match image::open(&path) {
            Ok(img) => {
                // Convert to RGBA8 then save as the same extension to strip metadata.
                let rgba = img.to_rgba8();
                let tmp = path.with_extension("tmpimg");
                if rgba.save(&tmp).is_ok() {
                    // Replace original file atomically
                    if std::fs::rename(&tmp, &path).is_ok() {
                        count += 1;
                    } else {
                        // Fallback: try copy + remove
                        if std::fs::copy(&tmp, &path).is_ok() {
                            let _ = std::fs::remove_file(&tmp);
                            count += 1;
                        } else {
                            let _ = std::fs::remove_file(&tmp);
                        }
                    }
                } else {
                    let _ = std::fs::remove_file(&tmp);
                }
            }
            Err(e) => {
                log::warn!("normalize image open failed {}: {}", path.display(), e);
            }
        }
    }

    Ok(count)
}
