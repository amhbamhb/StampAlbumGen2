//! Incremental native layout primitives. Not yet an application PDF backend.

pub mod colour;
pub mod commands;
pub mod config;
pub mod date;
pub mod document;
mod numbering;
pub mod proof;
pub mod renderer;
pub mod stamp_box;
pub mod stamp_image;
pub mod text;
pub mod units;

pub type Result<T> = std::result::Result<T, String>;
