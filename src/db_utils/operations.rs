//! Database operations for stamp management.
//!
//! This module contains functions for inserting, updating, and querying stamp data.

use log::{debug, error};
use regex::Regex;
use rusqlite::{params, Connection, Result};

use crate::db_utils::StampStub;

/// Insert a stamp into the database.
///
/// This function handles the complex logic of inserting or updating stamp records,
/// including normalization of data and handling existing records.
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
    // Normalize stamps_issued: if dots are used as thousand separators and no commas are present, replace them with commas
    let mut stamps_issued_normalized = stamps_issued.trim().to_string();
    if stamps_issued_normalized.starts_with('(') && stamps_issued_normalized.ends_with(')') {
        stamps_issued_normalized =
            stamps_issued_normalized[1..stamps_issued_normalized.len() - 1].to_string();
    }
    let re_non = Regex::new(r"[^0-9\.,]").unwrap();
    stamps_issued_normalized = re_non
        .replace_all(&stamps_issued_normalized, "")
        .to_string();
    if stamps_issued_normalized.contains('.') && !stamps_issued_normalized.contains(',') {
        stamps_issued_normalized = stamps_issued_normalized.replace('.', ",");
    }
    // Check if stamp already exists and fetch existing metadata to decide if we
    // should backfill missing columns. We will only fill columns that are NULL
    // or empty and where the caller provided a non-empty value.
    let exists_sql = "SELECT COUNT(*), image_path, remote_image_url, description, stamps_issued, mint_condition, unused, used, letter_fdc, currency, watermark, design, engraving, perforation, context FROM stamps WHERE country = ?1 AND series = ?2 AND sg_number = ?3";
    let mut stmt = conn.prepare(exists_sql)?;
    let (
        count,
        existing_image_path,
        existing_remote_url,
        existing_description,
        existing_stamps_issued,
        existing_mint_condition,
        existing_unused,
        existing_used,
        existing_letter_fdc,
        existing_currency,
        existing_watermark,
        existing_design,
        existing_engraving,
        existing_perforation,
        existing_context,
    ): (
        i64,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    ) = stmt.query_row(params![country, series, sg_number], |row| {
        Ok((
            row.get(0)?,
            row.get(1).unwrap_or(String::new()),
            row.get(2).unwrap_or(String::new()),
            row.get(3).unwrap_or(String::new()),
            row.get(4).unwrap_or(String::new()),
            row.get(5).unwrap_or(String::new()),
            row.get(6).unwrap_or(String::new()),
            row.get(7).unwrap_or(String::new()),
            row.get(8).unwrap_or(String::new()),
            row.get(9).unwrap_or(String::new()),
            row.get(10).unwrap_or(String::new()),
            row.get(11).unwrap_or(String::new()),
            row.get(12).unwrap_or(String::new()),
            row.get(13).unwrap_or(String::new()),
            row.get(14).unwrap_or(String::new()),
        ))
    })?;
    if count > 0 {
        // Stamp exists; check if we need to backfill any missing columns
        let mut update_needed = false;
        let mut update_sql = "UPDATE stamps SET ".to_string();
        let mut params_vec = Vec::new();
        let mut param_count = 0;

        // Helper to add update clause if value is missing and we have a non-empty value
        let mut add_update = |field: &str, existing: &str, new_val: &str| {
            if existing.trim().is_empty() && !new_val.trim().is_empty() {
                if update_needed {
                    update_sql.push_str(", ");
                }
                update_sql.push_str(&format!("{} = ?{}", field, param_count + 1));
                params_vec.push(new_val.to_string());
                param_count += 1;
                update_needed = true;
            }
        };

        add_update("image_path", &existing_image_path, image_path);
        add_update("remote_image_url", &existing_remote_url, remote_image_url);
        add_update("description", &existing_description, description);
        add_update(
            "stamps_issued",
            &existing_stamps_issued,
            &stamps_issued_normalized,
        );
        add_update("mint_condition", &existing_mint_condition, mint_condition);
        add_update("unused", &existing_unused, unused);
        add_update("used", &existing_used, used);
        add_update("letter_fdc", &existing_letter_fdc, letter_fdc);
        add_update("currency", &existing_currency, currency);
        add_update("watermark", &existing_watermark, watermark);
        add_update("design", &existing_design, design);
        add_update("engraving", &existing_engraving, engraving);
        add_update("perforation", &existing_perforation, perforation);
        add_update("context", &existing_context, context);

        if update_needed {
            update_sql.push_str(" WHERE country = ?");
            params_vec.push(country.to_string());
            update_sql.push_str(" AND series = ?");
            params_vec.push(series.to_string());
            update_sql.push_str(" AND sg_number = ?");
            params_vec.push(sg_number.to_string());

            debug!(
                "Updating existing stamp: {}|{}|{}",
                country, series, sg_number
            );
            let mut update_stmt = conn.prepare(&update_sql)?;
            update_stmt.execute(rusqlite::params_from_iter(params_vec))?;
            return Ok(true);
        } else {
            debug!(
                "Stamp already exists and no updates needed: {}|{}|{}",
                country, series, sg_number
            );
            return Ok(false);
        }
    }

    // Stamp does not exist; insert new record
    let insert_sql = "INSERT INTO stamps (country, series, year, month, month_num, day, sg_number, type, denomination, color, description, stamps_issued, mint_condition, unused, used, letter_fdc, currency, image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, context) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25)";
    let month_num: i32 = _month_num.parse().unwrap_or(0);
    match conn.execute(
        insert_sql,
        params![
            country,
            series,
            year,
            month,
            month_num,
            day,
            sg_number,
            type_,
            denomination,
            color,
            description,
            stamps_issued_normalized,
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
            context
        ],
    ) {
        Ok(_) => {
            debug!("Inserted new stamp: {}|{}|{}", country, series, sg_number);
            Ok(true)
        }
        Err(e) => {
            error!(
                "Failed to insert stamp {}|{}|{}: {}",
                country, series, sg_number, e
            );
            Err(e)
        }
    }
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
    // Insert with a minimal non-NULL `type` so consumers (album generator)
    // that expect a text value do not panic. Also set description/context
    // to empty strings. We deliberately avoid touching image fields.
    let sql = "INSERT INTO stamps (country, series, year, month, month_num, day, sg_number, type, denomination, color, description, context) VALUES (?1, ?2, ?3, '', 0, '', ?4, 'PLACEHOLDER', '', '', '', '')";
    match conn.execute(sql, params![country, series, year, sg_number]) {
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
    // - None: delegate to the DB crate (alphabetical)
    // - Some("first_sg"): order by the numeric minimum of sg_number per series (ascending)
    // - Some("first_sg_desc"): same but descending
    if let Some(order) = series_order {
        match order {
            "first_sg" => {
                let mut stmt = conn.prepare(
                    "SELECT series FROM stamps GROUP BY series ORDER BY MIN(CAST(sg_number AS INTEGER)) ASC",
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
                    "SELECT series FROM stamps GROUP BY series ORDER BY MIN(CAST(sg_number AS INTEGER)) DESC",
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
