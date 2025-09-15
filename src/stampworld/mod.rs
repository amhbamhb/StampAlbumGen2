//! # StampWorld Module
//!
//! This module contains all StampWorld-specific functionality for StampAlbumGen2,
//! including data collection, album generation, and global state management.
//!
//! ## Organization
//!
//! - `album_generator.rs`: Core album generation logic with proper page layout
//! - `data_collector.rs`: StampWorld data collection and processing
//! - `globals.rs`: Global state and constants for StampWorld operations
//! - `mod.rs`: Module declarations and public exports

pub mod album_generator;
pub mod data_collector;
pub mod globals;

// Re-export the main functions for easy access
pub use album_generator::{generate_album_output_from_stamps, write_globals, load_stamps_from_db};
pub use data_collector::collect_stampworld_data;
pub use globals::*;