pub mod cli;
pub mod colnect;
pub mod config;
pub mod flagdb;
pub mod modes;
pub mod progress;
pub mod runtime_defaults;
pub mod review_ui;
pub mod setup;
pub mod validation;
pub mod wikipedia;
pub use crate::config::{load_slug_overrides, Config, HeaderFormatting};
// Main library file for StampAlbumGen2
// External crates

// Image and font handling

// HTML parsing and HTTP requests

// Add more modules as needed

pub mod image_utils;
// pub mod html_utils;
pub mod commands_file_generator;
pub mod stampworld;
// pub mod stamp_dimensions;
pub mod file_utils;
// pub mod text_utils;
pub mod album_globals;
pub mod album_utils;
pub mod placeholder_image;
pub mod types;
pub mod us1847;
// db utilities refactored into modular structure
pub mod db_utils;
pub mod image_downloader;
pub mod sample_images;

// Re-export commonly used functions and types
// Re-export album generation wrapper
pub use album_globals::generate_album_output;
// pub use stamp_dimensions::*;
pub use file_utils::*;
// pub use text_utils::*;
pub use album_utils::*;
pub use types::{ImageStatus, OSType, SeriesInfo};

// Re-export selected DB helpers for convenience tests and external callers
pub use db_utils::{
    insert_stamp, insert_stamp_placeholder, migrate_fix_block_metadata, migrate_stamps_table,
};
