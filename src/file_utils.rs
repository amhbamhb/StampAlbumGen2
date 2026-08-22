//! # File Utilities Module
//!
//! This module provides file system utilities for the StampAlbumGen2 application,
//! including path management, file operations, and configuration loading.

use crate::types::ALBUM_DB_PATH;
use log::debug;
use std::path::PathBuf;

pub fn resolve_runtime_albums_root() -> PathBuf {
    if let Ok(from_env) = std::env::var("STAMP_ALBUMS_ROOT") {
        let trimmed = from_env.trim();
        if !trimmed.is_empty() {
            return PathBuf::from(trimmed);
        }
    }

    if let Some(from_cfg) =
        crate::config::load_config("config.toml").effective_bootstrap_albums_root_path()
    {
        let trimmed = from_cfg.trim();
        if !trimmed.is_empty() {
            return PathBuf::from(trimmed);
        }
    }

    PathBuf::from("Albums")
}

pub fn resolve_album_dir(album_name: &str) -> PathBuf {
    resolve_runtime_albums_root().join(album_name.replace(' ', "-"))
}

pub fn resolve_album_image_path(album_name: &str, image_path: &str) -> PathBuf {
    let clean_image_path = image_path.strip_prefix("images/").unwrap_or(image_path);

    resolve_album_dir(album_name)
        .join("images")
        .join(clean_image_path)
}

/// Sets the database path from CLI arguments
pub fn set_db_path_from_cli(path: &str) {
    *ALBUM_DB_PATH.lock().unwrap() = path.to_string();
}

/// Gets the database path from CLI arguments
pub fn get_db_path_from_cli() -> String {
    ALBUM_DB_PATH.lock().unwrap().clone()
}

/// Finds the configuration file path
pub fn find_config_path() -> String {
    use std::env;
    use std::path::PathBuf;

    // Try current directory first
    let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let config_path = current_dir.join("config.toml");

    if config_path.exists() {
        return config_path.to_string_lossy().to_string();
    }

    // Try parent directories
    let mut current = current_dir;
    for _ in 0..5 {
        current = current.parent().unwrap_or(&current).to_path_buf();
        let config_path = current.join("config.toml");
        if config_path.exists() {
            return config_path.to_string_lossy().to_string();
        }
    }

    // Default fallback
    "config.toml".to_string()
}

/// Checks if an image file exists for the given album and image path
pub fn image_exists(album_name: &str, image_path: &str) -> bool {
    // Strip "images/" prefix if present
    let clean_image_path = image_path.strip_prefix("images/").unwrap_or(image_path);

    // Construct the full path to the image
    let image_full_path = resolve_album_image_path(album_name, clean_image_path);

    image_full_path.exists()
}

/// Set the overwrite-album flag (typically from CLI)
pub fn set_overwrite_album_from_cli(flag: bool) {
    use crate::types::OVERWRITE_ALBUM;
    *OVERWRITE_ALBUM.lock().unwrap() = flag;
}

/// Get the overwrite-album flag
pub fn get_overwrite_album_from_cli() -> bool {
    use crate::types::OVERWRITE_ALBUM;
    *OVERWRITE_ALBUM.lock().unwrap()
}

/// Gets the appropriate image path, using placeholder if the original doesn't exist
pub fn get_image_path_or_placeholder(album_name: &str, image_path: &str) -> String {
    if image_path.is_empty() {
        return "images/no_image_placeholder.png".to_string();
    }

    if image_exists(album_name, image_path) {
        image_path.to_string()
    } else {
        debug!("Image {} not found, using placeholder", image_path);
        "images/no_image_placeholder.png".to_string()
    }
}
