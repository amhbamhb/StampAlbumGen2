//! Incremental native layout primitives. Not yet an application PDF backend.

pub mod colour;
pub mod commands;
pub mod config;
pub mod document;
pub mod renderer;
pub mod units;

pub type Result<T> = std::result::Result<T, String>;
