//! Database update utilities.
//!
//! This module contains functions for updating database records.

use rusqlite::{Connection, Result, params};

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

/// Inspect `local_image_path` and persist its pixel dimensions to the DB if present.
///
/// Returns Ok(true) if an UPDATE was performed (rows > 0), Ok(false) if no image
/// was found or there was nothing to update, or Err on SQL error.
pub fn persist_image_dimensions_from_file(
    conn: &Connection,
    country: &str,
    series: &str,
    sg_number: &str,
    local_image_path: &str,
) -> Result<bool> {
    if local_image_path.is_empty() {
        return Ok(false);
    }
    if let Some((w_px, h_px)) = crate::image_utils::get_image_dimensions(local_image_path) {
        let w = Some(w_px as i32);
        let h = Some(h_px as i32);
        let rows = update_image_dimensions(conn, country, series, sg_number, w, h)?;
        Ok(rows > 0)
    } else {
        Ok(false)
    }
}
