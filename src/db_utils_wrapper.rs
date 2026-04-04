//! Compatibility wrapper that delegates heavy DB operations to
//! `crates/stamp_album_db` while preserving a small compatibility surface for
//! the main crate (so other modules can continue to call `crate::db_utils::*`).

use rusqlite::{Connection, Result};
pub use stamp_album_db::StampStub;

/// Initialize DB using the DB crate implementation
pub fn init_db(db_path: &str) -> Result<Connection> {
    stamp_album_db::init_db(db_path)
}

/// Run stamps table migration (delegates to DB crate)
pub fn migrate_stamps_table(conn: &Connection) -> Result<()> {
    stamp_album_db::migrate_stamps_table(conn)
}

/// Run block metadata migration (delegates to DB crate)
pub fn migrate_fix_block_metadata(conn: &Connection) -> Result<()> {
    stamp_album_db::migrate_fix_block_metadata(conn)
}

/// Delegate insert_stamp to the DB crate (keeps same signature)
pub fn insert_stamp(
    conn: &Connection,
    country: &str,
    series: &str,
    year: i32,
    month: &str,
    _month_num: &str,
    day: &str,
    sg_number: &str,
    type_: &str,
    denomination: &str,
    color: &str,
    description: &str,
    stamps_issued: &str,
    mint_condition: &str,
    unused: &str,
    used: &str,
    letter_fdc: &str,
    currency: &str,
    image_path: &str,
    remote_image_url: &str,
    original_image_url: &str,
    watermark: &str,
    design: &str,
    engraving: &str,
    perforation: &str,
    context: &str,
) -> Result<bool> {
    stamp_album_db::insert_stamp(
        conn,
        country,
        series,
        year,
        month,
        _month_num,
        day,
        sg_number,
        type_,
        denomination,
        color,
        description,
        stamps_issued,
        mint_condition,
        unused,
        used,
        letter_fdc,
        currency,
        image_path,
        remote_image_url,
        original_image_url,
        watermark,
        design,
        engraving,
        perforation,
        context,
    )
}

/// Insert a minimal placeholder stamp row when the collector encountered a grouped
/// range on the listing page and the user requested placeholder expansion.
/// Returns Ok(true) if inserted, Ok(false) if row already exists.
pub fn insert_stamp_placeholder(
    conn: &rusqlite::Connection,
    country: &str,
    series: &str,
    year: i32,
    sg_number: &str,
) -> rusqlite::Result<bool> {
    // Delegate to stamp_album_db implementation
    stamp_album_db::insert_stamp_placeholder(conn, country, series, year, sg_number)
}

/// Delegate get_stamps_by_series to crate
pub fn get_stamps_by_series(conn: &Connection, series: &str, year: &str) -> Result<Vec<StampStub>> {
    stamp_album_db::get_stamps_by_series(conn, series, year)
}

/// Compatibility wrapper for get_all_series; keeps the same signature used by callers.
pub fn get_all_series(
    conn: &rusqlite::Connection,
    _series_order: Option<&str>,
) -> rusqlite::Result<Vec<String>> {
    // Support legacy `series_order` options used by callers/tests.
    // - None: delegate to the DB crate (alphabetical)
    // - Some("first_sg"): order by the numeric minimum of sg_number per series (ascending)
    // - Some("first_sg_desc"): same but descending
    if let Some(order) = _series_order {
        match order {
            "first_sg" => {
                let mut stmt = conn.prepare(
                    "SELECT series
                     FROM stamps
                     GROUP BY series
                     ORDER BY COALESCE(MIN(NULLIF(year, 0)), 9999) ASC,
                              MIN(CAST(sg_number AS INTEGER)) ASC,
                              series COLLATE NOCASE ASC",
                )?;
                let rows = stmt.query_map([], |r| r.get::<_, String>(0))?;
                let mut res = Vec::new();
                for s in rows {
                    res.push(s?);
                }
                return Ok(res);
            }
            "first_sg_desc" => {
                let mut stmt = conn.prepare(
                    "SELECT series
                     FROM stamps
                     GROUP BY series
                     ORDER BY COALESCE(MIN(NULLIF(year, 0)), 9999) ASC,
                              MIN(CAST(sg_number AS INTEGER)) DESC,
                              series COLLATE NOCASE ASC",
                )?;
                let rows = stmt.query_map([], |r| r.get::<_, String>(0))?;
                let mut res = Vec::new();
                for s in rows {
                    res.push(s?);
                }
                return Ok(res);
            }
            _ => {
                // Unknown ordering - fall back to crate default alphabetical
                return stamp_album_db::get_all_series(conn);
            }
        }
    }
    // No ordering requested: delegate to the DB crate which returns Vec<String>.
    stamp_album_db::get_all_series(conn)
}

/// Delegate get_stamps_by_country_series to crate
pub fn get_stamps_by_country_series(
    conn: &Connection,
    country: &str,
    series: &str,
) -> Result<
    Vec<(
        i32,
        String,
        String,
        i32,
        String,
        i32,
        String,
        String,
        String,
        String,
        String,
        String,
    )>,
> {
    stamp_album_db::get_stamps_by_country_series(conn, country, series)
}

/// Keep local implementation for formatting stamps_issued (was used widely)
pub fn format_stamps_issued(s: &str) -> String {
    stamp_album_db::format_stamps_issued(s)
}

/// Keep the sort helper available by delegating to the crate implementation
pub fn sort_stampstubs_inplace(v: &mut Vec<StampStub>) {
    stamp_album_db::sort_stampstubs_inplace(v)
}

/// Lightweight update_image_dimensions kept here (simple SQL update)
pub fn update_image_dimensions(
    conn: &rusqlite::Connection,
    country: &str,
    series: &str,
    sg_number: &str,
    width: Option<i32>,
    height: Option<i32>,
) -> rusqlite::Result<usize> {
    let sql = "UPDATE stamps SET image_width_px = ?1, image_height_px = ?2 WHERE country = ?3 AND series = ?4 AND sg_number = ?5";
    conn.execute(
        sql,
        rusqlite::params![width, height, country, series, sg_number],
    )
}
