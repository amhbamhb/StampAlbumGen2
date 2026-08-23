//! Database operations for stamp management.
//!
//! This module contains functions for inserting, updating, and querying stamp data.

use log::{debug, error};
use rusqlite::{params, Connection, Result};

use crate::db_utils::StampStub;
use stamp_album_db::StampInsert as DbStampInsert;

// Helper functions moved to `crates/stamp_album_db` to provide a single
// implementation shared between the DB crate and the main crate. Keeping
// them here caused duplicate definitions and dead-code warnings.

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
pub fn get_stamps_by_series(
    conn: &Connection,
    series: &str,
    _year: &str,
) -> Result<Vec<StampStub>> {
    // Query local stamps table directly. The external crate currently
    // contains a stubbed implementation that returns an empty vec, which
    // causes the generator to emit no stamps. Use a conservative select
    // mapping the common StampStub fields used by the generator.
    let mut stmt = conn.prepare(
        "SELECT year, month, month_num, day, sg_number, type, '' as stamp_type, denomination, color, description, '' as desc, '' as catalog_num, stamps_issued, mint_condition, unused, used, letter_fdc, currency, image_path, remote_image_url, original_image_url, image_width_px, image_height_px, watermark, design, engraving, perforation, series, context, issue_location, quantity_printed, 0 as multi_perf FROM stamps WHERE series = ?1 ORDER BY year"
    )?;
    let stamp_iter = stmt.query_map(params![series], |row| {
        Ok(StampStub {
            year: row.get::<_, Option<i32>>(0)?.unwrap_or_default(),
            month: row.get::<_, Option<String>>(1)?.unwrap_or_default(),
            month_num: row.get::<_, Option<i32>>(2)?.unwrap_or_default(),
            day: row.get::<_, Option<String>>(3)?.unwrap_or_default(),
            sg_number: row.get::<_, Option<String>>(4)?.unwrap_or_default(),
            r#type: row.get::<_, Option<String>>(5)?.unwrap_or_default(),
            stamp_type: row.get::<_, Option<String>>(6)?.unwrap_or_default(),
            denomination: row.get::<_, Option<String>>(7)?.unwrap_or_default(),
            color: row.get::<_, Option<String>>(8)?.unwrap_or_default(),
            description: row.get::<_, Option<String>>(9)?.unwrap_or_default(),
            desc: row.get::<_, Option<String>>(10)?.unwrap_or_default(),
            catalog_num: row.get::<_, Option<String>>(11)?.unwrap_or_default(),
            stamps_issued: row.get::<_, Option<String>>(12)?.unwrap_or_default(),
            mint_condition: row.get::<_, Option<String>>(13)?.unwrap_or_default(),
            unused: row.get::<_, Option<String>>(14)?.unwrap_or_default(),
            used: row.get::<_, Option<String>>(15)?.unwrap_or_default(),
            letter_fdc: row.get::<_, Option<String>>(16)?.unwrap_or_default(),
            currency: row.get::<_, Option<String>>(17)?.unwrap_or_default(),
            image_path: row.get::<_, Option<String>>(18)?.unwrap_or_default(),
            remote_image_url: row.get::<_, Option<String>>(19)?.unwrap_or_default(),
            original_image_url: row.get::<_, Option<String>>(20)?.unwrap_or_default(),
            image_width_px: row.get::<_, Option<i32>>(21)?,
            image_height_px: row.get::<_, Option<i32>>(22)?,
            watermark: row.get::<_, Option<String>>(23)?.unwrap_or_default(),
            design: row.get::<_, Option<String>>(24)?.unwrap_or_default(),
            engraving: row.get::<_, Option<String>>(25)?.unwrap_or_default(),
            perforation: row.get::<_, Option<String>>(26)?.unwrap_or_default(),
            series: row.get::<_, Option<String>>(27)?.unwrap_or_default(),
            context: row.get::<_, Option<String>>(28)?.unwrap_or_default(),
            issue_location: row.get::<_, Option<String>>(29)?.unwrap_or_default(),
            quantity_printed: row.get::<_, Option<String>>(30)?.unwrap_or_default(),
            multi_perf: row.get::<_, i32>(31)? != 0,
        })
    })?;
    let mut results = Vec::new();
    for s in stamp_iter {
        results.push(s?);
    }
    Ok(results)
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
