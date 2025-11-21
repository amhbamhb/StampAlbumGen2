//! Image utilities for downloading and processing stamp images.
//!
//! This module contains functions for downloading images from various sources
//! (Colnect, StampWorld, etc.) and handling image-related operations.

use crate::types::ImageStatus;
use log::{debug, error};

/// Represents series information including dimensions and perforation details.
#[derive(Debug, Default)]
pub struct SeriesInfo {
    pub count: i32,
    pub multi_perf: bool,
    pub width: String,
    pub height: String,
}

/// Native blocking downloader helper: streams to a temp file and atomically
/// renames into place. Retries on transient failures.
pub fn download_to_path(
    url: &str,
    dest: &std::path::Path,
    _no_ssl_check: bool,
) -> Result<(), String> {
    use reqwest::blocking::Client;
    use std::io::{self};
    use std::time::Duration;
    use tempfile::NamedTempFile;

    let client_builder = Client::builder().timeout(Duration::from_secs(30));
    let client = client_builder
        .build()
        .map_err(|e| format!("failed to build client: {}", e))?;

    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("failed to create dir {}: {}", parent.display(), e))?;
    }

    let mut last_err: Option<String> = None;
    for attempt in 1..=3 {
        match client.get(url).send() {
            Ok(mut resp) => {
                if !resp.status().is_success() {
                    last_err = Some(format!("HTTP {}", resp.status()));
                } else {
                    match NamedTempFile::new_in(dest.parent().unwrap_or(std::path::Path::new(".")))
                    {
                        Ok(mut tmp) => {
                            let copy_res = io::copy(&mut resp, &mut tmp);
                            if let Err(e) = copy_res {
                                last_err = Some(format!("io copy error: {}", e));
                            } else if let Err(e) = tmp.persist(dest) {
                                last_err = Some(format!("persist error: {}", e));
                            } else {
                                return Ok(());
                            }
                        }
                        Err(e) => last_err = Some(format!("tempfile create error: {}", e)),
                    }
                }
            }
            Err(e) => last_err = Some(format!("request error: {}", e)),
        }
        std::thread::sleep(Duration::from_millis(500 * attempt));
    }
    Err(last_err.unwrap_or_else(|| "unknown error".to_string()))
}

/// Represents the status of an image operation (stub)
pub fn get_colnect_image(images_directory: &str, url: &str, name: &str) -> ImageStatus {
    let clean_name = name.replace(' ', "");
    let file_path = format!("{}/{}.jpg", images_directory, clean_name);
    get_image(url, name, &file_path)
}

/// Stub for get_image, to be implemented
pub fn get_image(_url: &str, _name: &str, _file_path: &str) -> ImageStatus {
    // Debug: Show the process of accessing the external website and getting the image
    debug!("Attempting to fetch image from URL: {}", _url);
    debug!("Image will be saved as: {}", _file_path);
    // Ensure the destination directory exists to avoid "No such file or directory" errors
    if let Some(parent) = std::path::Path::new(_file_path).parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            error!(
                "Failed to create image directory {}: {}",
                parent.display(),
                e
            );
            // proceed anyway; write will likely fail and be logged below
        }
    }
    // Example: Use reqwest to fetch the image
    match reqwest::blocking::get(_url) {
        Ok(resp) => {
            debug!("HTTP status: {}", resp.status());
            if resp.status().is_success() {
                match resp.bytes() {
                    Ok(bytes) => {
                        match std::fs::write(_file_path, &bytes) {
                            Ok(_) => debug!("Image saved successfully."),
                            Err(e) => error!("Failed to save image: {}", e),
                        }
                        ImageStatus {
                            link: _url.to_string(),
                            exists: true,
                            width: 0,
                            height: 0,
                            filename: _name.to_string(),
                        }
                    }
                    Err(e) => {
                        error!("Failed to read image bytes: {}", e);
                        ImageStatus {
                            link: _url.to_string(),
                            exists: false,
                            width: 0,
                            height: 0,
                            filename: _name.to_string(),
                        }
                    }
                }
            } else {
                error!("HTTP request failed with status: {}", resp.status());
                ImageStatus {
                    link: _url.to_string(),
                    exists: false,
                    width: 0,
                    height: 0,
                    filename: _name.to_string(),
                }
            }
        }
        Err(e) => {
            error!("HTTP request error: {}", e);
            crate::types::ImageStatus {
                link: _url.to_string(),
                exists: false,
                width: 0,
                height: 0,
                filename: String::new(),
            }
        }
    }
}
