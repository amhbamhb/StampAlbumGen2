//! # Image Downloader Module
//!
//! This module provides functionality to download stamp images from remote URLs
//! when they are missing from the local filesystem.

use std::fs;
use std::path::Path;
use std::time::Duration;
use reqwest;

/// Downloads an image from a remote URL to the local filesystem
///
/// # Arguments
/// * `url` - The remote URL of the image
/// * `local_path` - The local path where to save the image
///
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Success or error
pub fn download_image(url: &str, local_path: &Path, sg: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("[DOWNLOAD_ATTEMPT] sg={} url={} -> {}", sg, url, local_path.display());

    // Create the directory if it doesn't exist
    if let Some(parent) = local_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Build a small client that looks like a browser to reduce anti-bot responses
    let mut default_headers = reqwest::header::HeaderMap::new();
    default_headers.insert(
        reqwest::header::USER_AGENT,
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0 Safari/537.36".parse().unwrap(),
    );
    default_headers.insert(
        reqwest::header::REFERER,
        "https://www.google.com/".parse().unwrap(),
    );

    let client = reqwest::blocking::Client::builder()
        .default_headers(default_headers)
        .redirect(reqwest::redirect::Policy::limited(10))
        .timeout(Duration::from_secs(30))
        .build()?;

    let resp = match client.get(url).send() {
        Ok(r) => r,
        Err(e) => {
            println!("[DOWNLOAD_FAIL] sg={} url={} error=failed_request:{}", sg, url, e);
            return Err(Box::new(e));
        }
    };

    let status = resp.status();
    if !status.is_success() {
        println!("[DOWNLOAD_FAIL] sg={} url={} error=status:{}", sg, url, status.as_u16());
        return Err(format!("HTTP status {}", status.as_u16()).into());
    }

    // Check content-type header to avoid saving HTML error pages
    if let Some(ct) = resp.headers().get(reqwest::header::CONTENT_TYPE) {
        if let Ok(ct_str) = ct.to_str() {
            if !ct_str.starts_with("image/") {
                println!("[DOWNLOAD_FAIL] sg={} url={} error=unexpected_content_type:{}", sg, url, ct_str);
                return Err(format!("unexpected content-type: {}", ct_str).into());
            }
        }
    }

    let bytes = match resp.bytes() {
        Ok(b) => b,
        Err(e) => {
            println!("[DOWNLOAD_FAIL] sg={} url={} error=read_bytes:{}", sg, url, e);
            return Err(Box::new(e));
        }
    };

    // Validate bytes decode as an image before writing to disk
    if image::load_from_memory(&bytes).is_err() {
        println!("[DOWNLOAD_FAIL] sg={} url={} error=invalid_image_bytes size={}", sg, url, bytes.len());
        return Err("downloaded bytes did not decode as an image".into());
    }

    // Save to file
    if let Err(e) = fs::write(local_path, &bytes) {
        println!("[DOWNLOAD_FAIL] sg={} url={} error=write_file:{}", sg, url, e);
        return Err(Box::new(e));
    }

    println!("[DOWNLOAD_OK] sg={} url={} status={} saved={} size={}", sg, url, status.as_u16(), local_path.display(), bytes.len());
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
pub fn download_stamp_image_if_missing(
    image_path: &str,
    remote_url: &str,
    album_name: &str,
    sg: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    if remote_url.is_empty() {
        return Ok(image_path.to_string());
    }

    // Construct the full local path
    let album_dir = Path::new("Albums").join(album_name.replace(' ', "-"));
    let images_dir = album_dir.join("images");
    
    // Strip "images/" prefix if present
    let clean_image_path = if image_path.starts_with("images/") {
        &image_path[7..] // Remove "images/" prefix
    } else {
        image_path
    };
    
    let local_path = images_dir.join(clean_image_path);

    // Check if image already exists
    if local_path.exists() {
        return Ok(image_path.to_string());
    }

    // Try to download the image (instrumented)
    match download_image(remote_url, &local_path, sg) {
        Ok(_) => {
            // Ensure file now exists
            if local_path.exists() {
                Ok(image_path.to_string())
            } else {
                Err("download reported OK but file not found".into())
            }
        }
        Err(e) => {
            println!("[DOWNLOAD_FAIL_FINAL] sg={} url={} error={}", sg, remote_url, e);
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
pub fn download_missing_stamp_images(
    stamps: &[crate::db_utils::StampStub],
    album_name: &str
) -> Result<(), Box<dyn std::error::Error>> {
    println!("[INFO] Checking for missing images to download...");

    let mut downloaded_count = 0;
    let mut total_missing = 0;

    for stamp in stamps {
        if !stamp.image_path.is_empty() && !stamp.remote_image_url.is_empty() {
            // Strip "images/" prefix if present for path checking
            let clean_image_path = if stamp.image_path.starts_with("images/") {
                &stamp.image_path[7..] // Remove "images/" prefix
            } else {
                &stamp.image_path
            };
            
            // Check if image exists
            let album_dir = Path::new("Albums").join(album_name.replace(' ', "-"));
            let images_dir = album_dir.join("images");
            let local_path = images_dir.join(clean_image_path);

            if !local_path.exists() {
                total_missing += 1;
                match download_stamp_image_if_missing(&stamp.image_path, &stamp.remote_image_url, album_name, &stamp.sg_number) {
                    Ok(_) => {
                        downloaded_count += 1;
                    }
                    Err(e) => {
                        println!("[WARN] Failed to download image for stamp {}: {}", stamp.sg_number, e);
                    }
                }
            }
        }
    }

    if total_missing > 0 {
        println!("[INFO] Downloaded {}/{} missing images", downloaded_count, total_missing);
    } else {
        println!("[INFO] All images are already available locally");
    }

    Ok(())
}