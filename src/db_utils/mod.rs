//! # Database Utilities Module
//!
//! This module contains database-related utilities and operations for the stamp album generator.
//! It includes compatibility wrappers, data formatting, migrations, and core database operations.

pub mod formatting;
pub mod init;
pub mod migrations;
pub mod operations;
pub mod sort;
pub mod structs;
pub mod updates;

// Re-export commonly used items for backward compatibility
pub use formatting::*;
pub use init::*;
pub use migrations::*;
pub use operations::*;
pub use sort::*;
pub use stamp_album_db::StampStub;
pub use structs::*;
pub use updates::*;
