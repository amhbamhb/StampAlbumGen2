//! Database operations for stamp management.
//!
//! This module contains functions for inserting, updating, and querying stamp data.

use log::{debug, error};
use regex::Regex;
use rusqlite::{params, Connection, OptionalExtension, Result};

use crate::db_utils::StampStub;

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
        .query_map(params![country, stampworld_image_url, keep_rowid], |r| {
            r.get::<_, i64>(0)
        })?
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

fn is_placeholder_image_path(path: &str) -> bool {
    let p = path.trim().to_ascii_lowercase();
    p.starts_with("images/no_image") || p.contains("placeholder")
}

fn is_real_image_candidate(path: &str) -> bool {
    let p = path.trim();
    !p.is_empty() && !is_placeholder_image_path(p)
}

fn is_image_url(url: &str) -> bool {
    let u = url.trim().to_ascii_lowercase();
    if u.is_empty() {
        return false;
    }
    let has_scheme = u.starts_with("http://") || u.starts_with("https://");
    let has_ext = [".jpg", ".jpeg", ".png", ".webp"]
        .iter()
        .any(|e| u.contains(e));
    has_scheme && has_ext
}

fn is_stampworld_image_url(url: &str) -> bool {
    let u = url.trim().to_ascii_lowercase();
    u.contains("stampworld.com") && is_image_url(&u)
}

fn is_colnect_page_url(url: &str) -> bool {
    url.trim()
        .to_ascii_lowercase()
        .contains("colnect.com/stamps/stamp/")
}

fn is_colnect_image_url(url: &str) -> bool {
    let u = url.trim().to_ascii_lowercase();
    u.contains("colnect.com") && is_image_url(&u)
}

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
    let inferred_stampworld_image_url = if is_stampworld_image_url(remote_image_url) {
        remote_image_url.trim().to_string()
    } else {
        String::new()
    };
    let inferred_colnect_page_url = if is_colnect_page_url(remote_image_url) {
        remote_image_url.trim().to_string()
    } else if is_colnect_page_url(original_image_url) {
        original_image_url.trim().to_string()
    } else {
        String::new()
    };
    let inferred_colnect_image_url = if is_colnect_image_url(remote_image_url) {
        remote_image_url.trim().to_string()
    } else {
        String::new()
    };
    let uniq_key = format!("{}|{}|{}", country.to_lowercase(), series, sg_number);

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
    // should backfill missing columns. Prefer exact country+series+sg matching,
    // but fall back to stable StampWorld identity keys so a later Colnect review
    // edit of series/SG does not cause the same StampWorld row to be inserted again.
    let existing_select = "SELECT rowid, image_path, remote_image_url, original_image_url, description, stamps_issued, mint_condition, unused, used, letter_fdc, currency, watermark, design, engraving, perforation, context, COALESCE(stampworld_image_url, ''), COALESCE(colnect_page_url, ''), COALESCE(colnect_image_url, '') FROM stamps WHERE {predicate} LIMIT 1";
    let load_existing = |predicate: &str, bind: &[&dyn rusqlite::ToSql]| {
        let sql = existing_select.replace("{predicate}", predicate);
        let mut stmt = conn.prepare(&sql)?;
        stmt.query_row(bind, |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, Option<String>>(1)?.unwrap_or_default(),
                row.get::<_, Option<String>>(2)?.unwrap_or_default(),
                row.get::<_, Option<String>>(3)?.unwrap_or_default(),
                row.get::<_, Option<String>>(4)?.unwrap_or_default(),
                row.get::<_, Option<String>>(5)?.unwrap_or_default(),
                row.get::<_, Option<String>>(6)?.unwrap_or_default(),
                row.get::<_, Option<String>>(7)?.unwrap_or_default(),
                row.get::<_, Option<String>>(8)?.unwrap_or_default(),
                row.get::<_, Option<String>>(9)?.unwrap_or_default(),
                row.get::<_, Option<String>>(10)?.unwrap_or_default(),
                row.get::<_, Option<String>>(11)?.unwrap_or_default(),
                row.get::<_, Option<String>>(12)?.unwrap_or_default(),
                row.get::<_, Option<String>>(13)?.unwrap_or_default(),
                row.get::<_, Option<String>>(14)?.unwrap_or_default(),
                row.get::<_, Option<String>>(15)?.unwrap_or_default(),
                row.get::<_, Option<String>>(16)?.unwrap_or_default(),
                row.get::<_, Option<String>>(17)?.unwrap_or_default(),
                row.get::<_, Option<String>>(18)?.unwrap_or_default(),
            ))
        })
        .optional()
    };

    let existing = load_existing(
        "country = ?1 AND series = ?2 AND sg_number = ?3",
        &[&country, &series, &sg_number],
    )?
    .or_else(|| {
        if inferred_stampworld_image_url.trim().is_empty() {
            None
        } else {
            load_existing(
                "country = ?1 AND TRIM(COALESCE(stampworld_image_url, '')) = ?2",
                &[&country, &inferred_stampworld_image_url],
            )
            .ok()
            .flatten()
        }
    })
    .or_else(|| {
        if !is_real_image_candidate(image_path) {
            None
        } else {
            load_existing(
                "country = ?1 AND TRIM(COALESCE(image_path, '')) = ?2",
                &[&country, &image_path],
            )
            .ok()
            .flatten()
        }
    });

    if let Some((
        existing_rowid,
        existing_image_path,
        existing_remote_url,
        existing_original_url,
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
        existing_stampworld_image_url,
        existing_colnect_page_url,
        existing_colnect_image_url,
    )) = existing
    {
        // Heal previously duplicated rows from older import behavior.
        if !inferred_stampworld_image_url.trim().is_empty() {
            if let Err(e) = cleanup_stampworld_identity_duplicates(
                conn,
                existing_rowid,
                country,
                &inferred_stampworld_image_url,
            ) {
                error!(
                    "Failed duplicate StampWorld identity cleanup for rowid={} sw_img='{}': {}",
                    existing_rowid, inferred_stampworld_image_url, e
                );
            }
        }

        // Stamp exists; check if we need to backfill any missing columns
        let mut update_needed = false;
        let mut update_sql = "UPDATE stamps SET ".to_string();
        let mut params_vec = Vec::new();

        macro_rules! add_update_if_missing {
            ($field:expr, $existing:expr, $new_val:expr) => {
                if $existing.trim().is_empty() && !$new_val.trim().is_empty() {
                    if update_needed {
                        update_sql.push_str(", ");
                    }
                    update_sql.push_str(&format!("{} = ?{}", $field, params_vec.len() + 1));
                    params_vec.push($new_val.to_string());
                    update_needed = true;
                }
            };
        }

        // Prefer real image paths over placeholder/empty image paths.
        if is_real_image_candidate(image_path)
            && (existing_image_path.trim().is_empty()
                || is_placeholder_image_path(&existing_image_path))
        {
            if update_needed {
                update_sql.push_str(", ");
            }
            update_sql.push_str(&format!("image_path = ?{}", params_vec.len() + 1));
            params_vec.push(image_path.to_string());
            update_needed = true;
        }

        // If current remote URL is empty or still a Colnect page URL, prefer
        // a concrete non-empty URL from new ingestion (e.g. StampWorld image URL).
        let existing_remote_is_colnect_page =
            existing_remote_url.contains("colnect.com/stamps/stamp/");
        if !remote_image_url.trim().is_empty()
            && (existing_remote_url.trim().is_empty() || existing_remote_is_colnect_page)
        {
            if update_needed {
                update_sql.push_str(", ");
            }
            update_sql.push_str(&format!("remote_image_url = ?{}", params_vec.len() + 1));
            params_vec.push(remote_image_url.to_string());
            update_needed = true;
        }

        add_update_if_missing!(
            "stampworld_image_url",
            &existing_stampworld_image_url,
            &inferred_stampworld_image_url
        );
        add_update_if_missing!(
            "colnect_page_url",
            &existing_colnect_page_url,
            &inferred_colnect_page_url
        );
        add_update_if_missing!(
            "colnect_image_url",
            &existing_colnect_image_url,
            &inferred_colnect_image_url
        );

        add_update_if_missing!(
            "original_image_url",
            &existing_original_url,
            original_image_url
        );
        add_update_if_missing!("description", &existing_description, description);
        add_update_if_missing!(
            "stamps_issued",
            &existing_stamps_issued,
            &stamps_issued_normalized
        );
        add_update_if_missing!("mint_condition", &existing_mint_condition, mint_condition);
        add_update_if_missing!("unused", &existing_unused, unused);
        add_update_if_missing!("used", &existing_used, used);
        add_update_if_missing!("letter_fdc", &existing_letter_fdc, letter_fdc);
        add_update_if_missing!("currency", &existing_currency, currency);
        add_update_if_missing!("watermark", &existing_watermark, watermark);
        add_update_if_missing!("design", &existing_design, design);
        add_update_if_missing!("engraving", &existing_engraving, engraving);
        add_update_if_missing!("perforation", &existing_perforation, perforation);
        add_update_if_missing!("context", &existing_context, context);

        if update_needed {
            update_sql.push_str(" WHERE rowid = ?");
            params_vec.push(existing_rowid.to_string());

            debug!(
                "Updating existing stamp rowid={} for {}|{}|{}",
                existing_rowid, country, series, sg_number
            );
            let mut update_stmt = conn.prepare(&update_sql)?;
            update_stmt.execute(rusqlite::params_from_iter(params_vec))?;
            return Ok(true);
        } else {
            debug!(
                "Stamp already exists and no updates needed for rowid={} using stable StampWorld identity: {}|{}|{}",
                existing_rowid, country, series, sg_number
            );
            return Ok(false);
        }
    }

    // Reconciliation fallback for Colnect-first/StampWorld-second flow:
    // if no exact (country+series+sg) match exists, try matching by
    // (country+year+sg) across any series and upgrade that row instead of
    // inserting a duplicate image-less variant.
    let alt_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM stamps WHERE country = ?1 AND year = ?2 AND sg_number = ?3",
        params![country, year, sg_number],
        |r| r.get(0),
    )?;
    if alt_count > 0 {
        let image_for_update = if is_real_image_candidate(image_path) {
            image_path
        } else {
            ""
        };
        let update_sql = "UPDATE stamps SET
            series = CASE
                WHEN TRIM(COALESCE(series, '')) = '' THEN ?1
                WHEN (TRIM(COALESCE(image_path, '')) = '' OR LOWER(image_path) LIKE 'images/no_image%') THEN ?1
                ELSE series
            END,
            image_path = CASE
                WHEN (?2 != '') AND (TRIM(COALESCE(image_path, '')) = '' OR LOWER(image_path) LIKE 'images/no_image%') THEN ?2
                ELSE image_path
            END,
            remote_image_url = CASE
                WHEN (?3 != '') AND (TRIM(COALESCE(remote_image_url, '')) = '' OR remote_image_url LIKE '%colnect.com/stamps/stamp/%') THEN ?3
                ELSE remote_image_url
            END,
            original_image_url = CASE
                WHEN (?4 != '') AND TRIM(COALESCE(original_image_url, '')) = '' THEN ?4
                ELSE original_image_url
            END,
            description = CASE WHEN (description IS NULL OR description = '') AND (?5 != '') THEN ?5 ELSE description END,
            stamps_issued = CASE WHEN (stamps_issued IS NULL OR stamps_issued = '') AND (?6 != '') THEN ?6 ELSE stamps_issued END,
            mint_condition = CASE WHEN (mint_condition IS NULL OR mint_condition = '') AND (?7 != '') THEN ?7 ELSE mint_condition END,
            unused = CASE WHEN (unused IS NULL OR unused = '') AND (?8 != '') THEN ?8 ELSE unused END,
            used = CASE WHEN (used IS NULL OR used = '') AND (?9 != '') THEN ?9 ELSE used END,
            letter_fdc = CASE WHEN (letter_fdc IS NULL OR letter_fdc = '') AND (?10 != '') THEN ?10 ELSE letter_fdc END,
            currency = CASE WHEN (currency IS NULL OR currency = '') AND (?11 != '') THEN ?11 ELSE currency END,
            watermark = CASE WHEN (watermark IS NULL OR watermark = '') AND (?12 != '') THEN ?12 ELSE watermark END,
            design = CASE WHEN (design IS NULL OR design = '') AND (?13 != '') THEN ?13 ELSE design END,
            engraving = CASE WHEN (engraving IS NULL OR engraving = '') AND (?14 != '') THEN ?14 ELSE engraving END,
            perforation = CASE WHEN (perforation IS NULL OR perforation = '') AND (?15 != '') THEN ?15 ELSE perforation END,
            context = CASE WHEN (context IS NULL OR context = '') AND (?16 != '') THEN ?16 ELSE context END,
            stampworld_image_url = CASE WHEN (stampworld_image_url IS NULL OR stampworld_image_url = '') AND (?17 != '') THEN ?17 ELSE stampworld_image_url END,
            colnect_page_url = CASE WHEN (colnect_page_url IS NULL OR colnect_page_url = '') AND (?18 != '') THEN ?18 ELSE colnect_page_url END,
            colnect_image_url = CASE WHEN (colnect_image_url IS NULL OR colnect_image_url = '') AND (?19 != '') THEN ?19 ELSE colnect_image_url END
            WHERE country = ?20 AND year = ?21 AND sg_number = ?22";

        let rows = conn.execute(
            update_sql,
            params![
                series,
                image_for_update,
                remote_image_url,
                original_image_url,
                description,
                stamps_issued_normalized,
                mint_condition,
                unused,
                used,
                letter_fdc,
                currency,
                watermark,
                design,
                engraving,
                perforation,
                context,
                inferred_stampworld_image_url,
                inferred_colnect_page_url,
                inferred_colnect_image_url,
                country,
                year,
                sg_number
            ],
        )?;
        if rows > 0 {
            debug!(
                "Reconciled existing stamp by country+year+sg: {}|{}|{}|{}",
                country, year, series, sg_number
            );
            return Ok(true);
        }
    }

    // Stamp does not exist; insert new record
    let insert_sql = "INSERT INTO stamps (country, series, year, month, month_num, day, sg_number, type, denomination, color, description, stamps_issued, mint_condition, unused, used, letter_fdc, currency, image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, context, uniq_key, stampworld_image_url, colnect_page_url, colnect_image_url) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29)";
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
            context,
            uniq_key,
            inferred_stampworld_image_url,
            inferred_colnect_page_url,
            inferred_colnect_image_url
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
    // - None: delegate to the DB crate (alphabetical)
    // - Some("first_sg"): order by the numeric minimum of sg_number per series (ascending)
    // - Some("first_sg_desc"): same but descending
    if let Some(order) = series_order {
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
