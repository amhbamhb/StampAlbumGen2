//! # File Utilities Module
//!
//! This module provides file system utilities for the StampAlbumGen2 application,
//! including path management, file operations, and configuration loading.

use std::path::Path;
use log::warn;
use crate::globals::ALBUM_DB_PATH;

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

/// Loads configuration from file
pub fn load_config(config_path: &str) -> crate::config::Config {
    use std::fs;

    match fs::read_to_string(config_path) {
        Ok(content) => {
            match toml::from_str(&content) {
                Ok(config) => config,
                Err(e) => {
                    warn!("Failed to parse config file {}: {}. Using defaults.", config_path, e);
                    crate::config::Config::default()
                }
            }
        }
        Err(e) => {
            warn!("Failed to read config file {}: {}. Using defaults.", config_path, e);
            crate::config::Config::default()
        }
    }
}

/// Gets image dimensions from file
pub fn get_image_dimensions(image_path: &str) -> Option<(u32, u32)> {
    use std::fs::File;
    use std::io::BufReader;

    if !Path::new(image_path).exists() {
        return None;
    }

    match File::open(image_path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            match image::load(reader, image::ImageFormat::Jpeg) {
                Ok(img) => Some((img.width(), img.height())),
                Err(_) => None,
            }
        }
        Err(_) => None,
    }
}