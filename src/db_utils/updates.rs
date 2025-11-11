//! Database update utilities.
//!
//! This module contains functions for updating database records.

use rusqlite::{params, Connection, Result};

/// Lightweight update_image_dimensions kept here (simple SQL update)
pub fn update_image_dimensions(
    conn: &Connection,
    country: &str,
    series: &str,
    sg_number: &str,
    width: Option<i32>,
    height: Option<i32>,
) -> Result<usize> {
    let sql = "UPDATE stamps SET image_width_px = ?1, image_height_px = ?2 WHERE country = ?3 AND series = ?4 AND sg_number = ?5";
    conn.execute(sql, params![width, height, country, series, sg_number])
}
