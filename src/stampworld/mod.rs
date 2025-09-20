//! # StampWorld Module
//!
//! This module contains all StampWorld-specific functionality for StampAlbumGen2,
//! including data collection, album generation, and global state management.
//!
//! ## Organization
//!
//! - `data_collector.rs`: StampWorld data collection and processing
//! - `state.rs`: Global state and constants for StampWorld operations
//! - `mod.rs`: Module declarations and public exports
//!
//! Note: Album generation functionality has been consolidated into album_globals.rs
//! for unified processing across all data sources.

pub mod data_collector;
pub mod state;

// Re-export the main functions for easy access
pub use data_collector::collect_stampworld_data;
pub use state::*;

// Album generation functions are now available from album_globals
pub use crate::album_globals::{generate_album_output_from_stamps, write_globals, load_stamps_from_db};