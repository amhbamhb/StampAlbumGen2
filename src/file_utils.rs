//! # File Utilities Module
//!
//! This module provides file system utilities for the StampAlbumGen2 application,
//! including path management, file operations, and configuration loading.

use crate::types::ALBUM_DB_PATH;

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