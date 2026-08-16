//! Database operations for stamp management.
//!
//! This module contains functions for inserting, updating, and querying stamp data.

use log::{debug, error};
use regex::Regex;
use rusqlite::{params, Connection, OptionalExtension, Result};

use crate::db_utils::StampStub;
use stamp_album_db::StampInsert as DbStampInsert;

fn cleanup_stampworld_identity_duplicates(
    conn: &Connection,
    keep_rowid: i64,
    country: &str,
    stampworld_image_url: &str,
) -> Result<usize> {
    if stampworld_image_url.trim().is_empty() {
        return Ok(0);
    }

    let mut stmt = conn.prepare(
        "SELECT rowid
         FROM stamps
         WHERE country = ?1
           AND TRIM(COALESCE(stampworld_image_url, '')) = ?2
           AND rowid != ?3",
    )?;
    let rowids = stmt
        .query_map(params![country, stampworld_image_url, keep_rowid], |r| r.get::<_, i64>(0))?
        .collect::<std::result::Result<Vec<_>, _>>()?;

    let mut deleted = 0usize;
    for rid in rowids {
        let n = conn.execute("DELETE FROM stamps WHERE rowid = ?1", params![rid])?;
        if n > 0 {
            deleted += n;
        }
    }

    if deleted > 0 {
        debug!(
            "Removed {} duplicate StampWorld identity row(s) for country='{}' sw_img='{}' (keeper rowid={})",
            deleted, country, stampworld_image_url, keep_rowid
        );
    }
    Ok(deleted)
}

#[allow(clippy::too_many_arguments)]
pub fn insert_stamp(
    conn: &Connection,
    country: &str,
    series: &str,
    year: i32,
    month: &str,
    month_num: &str,
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
    let si = DbStampInsert {
        country: country.to_string(),
        series: series.to_string(),
        year,
        month: month.to_string(),
        month_num: month_num.to_string(),
        day: day.to_string(),
        sg_number: sg_number.to_string(),
        type_: type_.to_string(),
        denomination: denomination.to_string(),
        color: color.to_string(),
        description: description.to_string(),
        stamps_issued: stamps_issued.to_string(),
        mint_condition: mint_condition.to_string(),
        unused: unused.to_string(),
        used: used.to_string(),
        letter_fdc: letter_fdc.to_string(),
        currency: currency.to_string(),
        image_path: image_path.to_string(),
        remote_image_url: remote_image_url.to_string(),
        original_image_url: original_image_url.to_string(),
        watermark: watermark.to_string(),
        design: design.to_string(),
        engraving: engraving.to_string(),
        perforation: perforation.to_string(),
        context: context.to_string(),
    };
    stamp_album_db::insert_stamp_from_struct(conn, &si)
}

/// Owned struct grouping all stamp insert parameters to reduce function arity.
/// New callers should prefer constructing this and calling `insert_stamp_from_struct`.
pub struct StampInsert {
    pub country: String,
    pub series: String,
    pub year: i32,
    pub month: String,
    pub month_num: String,
    pub day: String,
    pub sg_number: String,
    pub type_: String,
    pub denomination: String,
    pub color: String,
    pub description: String,
    pub stamps_issued: String,
    pub mint_condition: String,
    pub unused: String,
    pub used: String,
    pub letter_fdc: String,
    pub currency: String,
    pub image_path: String,
    pub remote_image_url: String,
    pub original_image_url: String,
    pub watermark: String,
    pub design: String,
    pub engraving: String,
    pub perforation: String,
    pub context: String,
}

/// Convenience wrapper: build arguments from a `StampInsert` and call the
/// existing `insert_stamp`. This allows incremental migration of callers.
pub fn insert_stamp_from_struct(conn: &Connection, si: &StampInsert) -> Result<bool> {
    insert_stamp(
        conn,
        &si.country,
        &si.series,
        si.year,
        &si.month,
        &si.month_num,
        &si.day,
        &si.sg_number,
        &si.type_,
        &si.denomination,
        &si.color,
        &si.description,
        &si.stamps_issued,
        &si.mint_condition,
        &si.unused,
        &si.used,
        &si.letter_fdc,
        &si.currency,
        &si.image_path,
        &si.remote_image_url,
        &si.original_image_url,
        &si.watermark,
        &si.design,
        &si.engraving,
        &si.perforation,
        &si.context,
    )
}

/// Insert a minimal placeholder stamp row when the collector encountered a grouped
/// range on the listing page and the user requested placeholder expansion.
pub fn insert_stamp_placeholder(
    conn: &Connection,
    country: &str,
    series: &str,
    year: i32,
    sg_number: &str,
) -> Result<bool> {
    // Check existence
    let exists: i64 = conn.query_row(
        "SELECT COUNT(*) FROM stamps WHERE country = ?1 AND series = ?2 AND sg_number = ?3",
        params![country, series, sg_number],
        |r| r.get(0),
    )?;
    if exists > 0 {
        debug!(
            "Placeholder not inserted because row exists: {}|{}|{}",
            country, series, sg_number
        );
        return Ok(false);
    }
    let uniq_key = format!("{}|{}|{}", country.to_lowercase(), series, sg_number);
    // Insert with a minimal non-NULL `type` so consumers (album generator)
    // that expect a text value do not panic. Also set description/context
    // to empty strings. We deliberately avoid touching image fields.
    let sql = "INSERT INTO stamps (country, series, year, month, month_num, day, sg_number, type, denomination, color, description, context, uniq_key) VALUES (?1, ?2, ?3, '', 0, '', ?4, 'PLACEHOLDER', '', '', '', '', ?5)";
    match conn.execute(sql, params![country, series, year, sg_number, uniq_key]) {
        Ok(_) => {
            debug!(
                "Inserted placeholder for {}|{}|{}",
                country, series, sg_number
            );
            Ok(true)
        }
        Err(e) => {
            error!(
                "Failed to insert placeholder {}|{}|{}: {}",
                country, series, sg_number, e
            );
            Err(e)
        }
    }
}

/// Get stamps by country and series.
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
    let mut stmt = conn.prepare(
        "SELECT id, country, series, year, month, month_num, day, description, image_path, remote_image_url, perforation, context FROM stamps WHERE country = ?1 AND series = ?2 ORDER BY year"
    )?;
    let stamp_iter = stmt.query_map(params![country, series], |row| {
        Ok((
            row.get(0)?,              // id
            row.get(1)?,              // country
            row.get(2)?,              // series
            row.get(3)?,              // year
            row.get(4)?,              // month
            row.get::<_, i32>(5)?,    // month_num
            row.get(6)?,              // day
            row.get::<_, String>(7)?, // description
            row.get(8)?,              // image_path
            row.get(9)?,              // remote_image_url
            row.get(10)?,             // perforation
            row.get(11)?,             // context
        ))
    })?;
    let mut results = Vec::new();
    for stamp in stamp_iter {
        results.push(stamp?);
    }
    Ok(results)
}

/// Get stamps by series and year.
pub fn get_stamps_by_series(conn: &Connection, series: &str, year: &str) -> Result<Vec<StampStub>> {
    stamp_album_db::get_stamps_by_series(conn, series, year)
}

/// Get all series from the database, with optional ordering.
///
/// This function supports different ordering options:
/// - None: alphabetical ordering (delegates to stamp_album_db)
/// - Some("first_sg"): order by the minimum sg_number per series (ascending)
/// - Some("first_sg_desc"): order by the minimum sg_number per series (descending)
pub fn get_all_series(
    conn: &rusqlite::Connection,
    series_order: Option<&str>,
) -> rusqlite::Result<Vec<String>> {
    // Support legacy `series_order` options used by callers/tests.
    // - None / Some("first_date"): order by earliest year/month/day in series
    // - Some("first_sg"): order by the numeric minimum of sg_number per series (ascending)
    // - Some("first_sg_desc"): same but descending
    if let Some(order) = series_order {
        match order {
            "first_date" => {
                let mut stmt = conn.prepare(
                    "SELECT series
                     FROM stamps
                     GROUP BY series
                     ORDER BY MIN(
                                printf(
                                    '%04d-%02d-%02d',
                                    CASE WHEN COALESCE(year, 0) > 0 THEN year ELSE 9999 END,
                                    CASE WHEN COALESCE(month_num, 0) > 0 THEN month_num ELSE 99 END,
                                    CASE WHEN CAST(COALESCE(day, '') AS INTEGER) > 0 THEN CAST(day AS INTEGER) ELSE 99 END
                                )
                              ) ASC,
                              series COLLATE NOCASE ASC",
                )?;
                let rows = stmt.query_map([], |r| r.get::<_, String>(0))?;
                let mut res = Vec::new();
                for s in rows {
                    res.push(s?);
                }
                return Ok(res);
            }
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
                // Unknown ordering - fall back to first_date behavior.
                return get_all_series(conn, Some("first_date"));
            }
        }
    }
    // No explicit order requested: default to first-date chronology.
    get_all_series(conn, Some("first_date"))
}
