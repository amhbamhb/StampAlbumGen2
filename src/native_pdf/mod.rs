//! Incremental native layout primitives. Not yet an application PDF backend.

pub mod album;
pub mod albumeasy;
pub mod colour;
pub mod commands;
pub mod config;
pub mod date;
pub mod document;
mod numbering;
pub mod pagination;
pub mod proof;
pub mod renderer;
pub mod stamp_box;
pub mod stamp_heading;
pub mod stamp_image;
pub mod stamp_inner_border;
pub mod stamp_rows;
pub mod stamp_text;
pub mod text;
pub mod units;

pub type Result<T> = std::result::Result<T, String>;
