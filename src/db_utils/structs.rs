//! Data structures for database utilities.
//!
//! This module contains the core data structures used throughout the database
//! utilities, including stamp and series representations.

/// Represents a series stub with basic metadata.
#[derive(Debug)]
pub struct SeriesStub {
    pub name: String,
    pub year: i32,
    pub month: String,
    pub day: String,
    pub is_perforated: bool,
    pub perforation: String,
}

/// Represents a stamp stub with comprehensive metadata.
///
/// This struct serves as a superset of fields used across modules.
/// Some code refers to alternate names; providing both helps during
/// the merge and keeps compatibility.
#[derive(Debug)]
pub struct StampStub {
    pub year: i32,
    pub month: String,
    pub month_num: i32,
    pub day: String,
    pub sg_number: String,
    pub r#type: String,
    pub stamp_type: String,
    pub denomination: String,
    pub color: String,
    pub description: String,
    pub desc: String,
    pub catalog_num: String,
    pub stamps_issued: String,
    pub mint_condition: String,
    pub unused: String,
    pub used: String,
    pub letter_fdc: String,
    pub currency: String,
    pub image_path: String,
    pub remote_image_url: String,
    pub original_image_url: String,
    pub image_width_px: Option<i32>,
    pub image_height_px: Option<i32>,
    pub watermark: String,
    pub design: String,
    pub engraving: String,
    pub perforation: String,
    pub series: String,
    pub context: String,
    pub multi_perf: bool,
}
