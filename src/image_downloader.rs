//! # Image Downloader Module
//!
//! This module provides functionality to download stamp images from remote URLs
//! when they are missing from the local filesystem.

use std::fs;
use std::path::Path;
use reqwest;

/// Downloads an image from a remote URL to the local filesystem
///
/// # Arguments
/// * `url` - The remote URL of the image
/// * `local_path` - The local path where to save the image
///
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Success or error
pub fn download_image(url: &str, local_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("[INFO] Downloading image from: {}", url);

    // Create the directory if it doesn't exist
    if let Some(parent) = local_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Download the image
    let response = reqwest::blocking::get(url)?;
    let bytes = response.bytes()?;

    // Save to file
    fs::write(local_path, &bytes)?;

    println!("[INFO] Successfully downloaded image to: {}", local_path.display());
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
    album_name: &str
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

    // Try to download the image
    match download_image(remote_url, &local_path) {
        Ok(_) => Ok(image_path.to_string()),
        Err(e) => {
            println!("[WARN] Failed to download image from {}: {}", remote_url, e);
            // Return the original path even if download failed
            // The placeholder system will handle missing images
            Ok(image_path.to_string())
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
                match download_stamp_image_if_missing(&stamp.image_path, &stamp.remote_image_url, album_name) {
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