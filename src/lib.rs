pub mod flagdb;
pub mod wikipedia;
pub mod config;
pub use crate::config::{Config, HeaderFormatting, load_slug_overrides};
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
pub mod types;
pub mod placeholder_image;
// db utilities moved to `stamp_album_db` crate in Phase 2; keep a compatibility shim
pub mod db_utils;
pub mod image_downloader;
pub mod sample_images;
pub mod album_globals;
pub mod album_utils;

// Re-export commonly used functions and types
// Re-export album generation wrapper
pub use album_globals::generate_album_output;
// pub use stamp_dimensions::*;
pub use file_utils::*;
// pub use text_utils::*;
pub use album_utils::*;
pub use types::{ImageStatus, OSType, SeriesInfo};
