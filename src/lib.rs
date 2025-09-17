mod config;
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
pub mod db_utils;
pub mod sample_images;
pub mod album_globals;

// Re-export commonly used functions and types
pub use album_globals::generate_album_output_from_stamps;
// pub use stamp_dimensions::*;
pub use file_utils::*;
// pub use text_utils::*;
pub use types::{ImageStatus, OSType, SeriesInfo, ImageStatus as GlobalImageStatus};
