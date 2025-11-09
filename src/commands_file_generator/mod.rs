//! # Commands File Generator Module
//!
//! This module contains functionality for generating command files and handling
//! stamp album data collection, image processing, and album creation.
//!
//! The module is organized into focused submodules:
//! - `image_utils`: Image downloading and processing utilities
//! - `data_collection`: Stamp data collection from various sources
//! - `album_creation`: Album file generation and writing
//! - `touchstamps`: Touchstamps-specific functionality
//! - `utils`: General utility functions
//! - `generator`: Core CommandsFileGenerator struct and methods

pub mod album_creation;
pub mod data_collection;
pub mod generator;
pub mod image_utils;
pub mod touchstamps;
pub mod utils;

// Re-export commonly used items for backward compatibility
pub use album_creation::*;
pub use data_collection::*;
pub use generator::CommandsFileGenerator;
pub use image_utils::*;
pub use touchstamps::*;
pub use utils::*;
