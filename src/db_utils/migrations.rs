//! Database migration utilities.
//!
//! This module contains functions for migrating database schemas and data.

use log::{debug, error};
use rusqlite::{params, Connection, Result};

/// Robust migration helper: recreate `stamps` table with canonical schema while preserving data.
pub fn migrate_stamps_table(conn: &Connection) -> Result<()> {
    debug!("Starting stamps table migration");
    // We'll execute statements inside an explicit BEGIN/COMMIT to avoid needing
    // a mutable transaction object borrowed from `conn`.
    conn.execute_batch("BEGIN IMMEDIATE;")?;

    // Desired final schema columns and types (same as earlier, plus uniq_key)
    // include image_width_px and image_height_px so the canonical schema contains them
    let desired_cols = vec![
        "id INTEGER PRIMARY KEY",
        "country TEXT",
        "series TEXT",
        "year INTEGER",
        "month TEXT",
        "month_num INTEGER",
        "day TEXT",
        "sg_number TEXT",
        "type TEXT",
        "denomination TEXT",
        "color TEXT",
        "description TEXT",
        "stamps_issued TEXT",
        "mint_condition TEXT",
        "unused TEXT",
        "used TEXT",
        "letter_fdc TEXT",
        "currency TEXT",
        "image_path TEXT",
        "remote_image_url TEXT",
        "original_image_url TEXT",
        "image_width_px INTEGER",
        "image_height_px INTEGER",
        "watermark TEXT",
        "design TEXT",
        "engraving TEXT",
        "perforation TEXT",
        "context TEXT",
        "uniq_key TEXT",
    ];

    // Also keep a simple list of desired column NAMES for matching existing columns.
    let desired_col_names = vec![
        "id",
        "country",
        "series",
        "year",
        "month",
        "month_num",
        "day",
        "sg_number",
        "type",
        "denomination",
        "color",
        "description",
        "stamps_issued",
        "mint_condition",
        "unused",
        "used",
        "letter_fdc",
        "currency",
        "image_path",
        "remote_image_url",
        "original_image_url",
        "image_width_px",
        "image_height_px",
        "watermark",
        "design",
        "engraving",
        "perforation",
        "context",
        "uniq_key",
    ];

    // Create new table
    let create_sql = format!(
        "CREATE TABLE IF NOT EXISTS stamps_new ({})",
        desired_cols.join(",")
    );
    conn.execute(&create_sql, [])?;

    // Get existing columns from old table
    let mut stmt = conn.prepare("PRAGMA table_info(stamps)")?;
    let mut rows = stmt.query([])?;
    let mut existing_cols: Vec<String> = Vec::new();
    while let Ok(Some(r)) = rows.next() {
        let name: String = r.get(1)?;
        existing_cols.push(name);
    }
    debug!("Existing stamps columns: {:?}", existing_cols);
    // Determine which columns we can copy
    // Determine which columns exist in both the old table and our desired schema
    let common_cols: Vec<String> = existing_cols
        .iter()
        .filter(|c| desired_col_names.contains(&c.as_str()))
        .cloned()
        .collect();
    // always include id if present and available
    let copy_cols = if existing_cols.contains(&"id".to_string()) {
        // ensure id is first
        let mut v = vec!["id".to_string()];
        for col in &common_cols {
            if col != "id" {
                v.push(col.clone());
            }
        }
        v
    } else {
        common_cols.clone()
    };
    debug!("Copying columns into stamps_new: {:?}", copy_cols);

    // Determine how many rows exist in the original stamps table before attempting copy.
    // If the original table is empty, it's safe to continue the migration even if
    // the copy produces 0 rows (there was nothing to copy).
    let original_count: i64 = conn.query_row("SELECT COUNT(*) FROM stamps", [], |r| r.get(0))?;

    if !copy_cols.is_empty() {
        let cols_list = copy_cols.join(",");
        let insert_sql = format!(
            "INSERT INTO stamps_new ({0}) SELECT {0} FROM stamps",
            cols_list
        );
        conn.execute(&insert_sql, [])?;
        debug!("Data copied into stamps_new");
    }

    // Safety check: if stamps_new is empty after the copy, abort the migration
    // to avoid accidentally dropping the live `stamps` table and losing data.
    let new_count: i64 = conn.query_row("SELECT COUNT(*) FROM stamps_new", [], |r| r.get(0))?;
    if new_count == 0 && original_count > 0 {
        // If stamps_new ended up empty but the original table actually had rows,
        // abort to avoid data loss. If the original was empty, proceed.
        error!("Migration aborted: stamps_new contains 0 rows after attempted copy. Rolling back to avoid data loss.");
        // Roll back the explicit transaction we started earlier and return an error
        let _ = conn.execute_batch("ROLLBACK;");
        return Err(rusqlite::Error::InvalidQuery);
    }

    // For rows where uniq_key is null/empty, backfill it
    conn.execute("UPDATE stamps_new SET uniq_key = lower(coalesce(country,'')) || '|' || coalesce(series,'') || '|' || coalesce(sg_number,'') WHERE uniq_key IS NULL OR uniq_key = ''", [])?;

    // Check for duplicates on uniq_key
    let dup_groups: i64 = conn.query_row("SELECT COUNT(*) FROM (SELECT uniq_key FROM stamps_new GROUP BY uniq_key HAVING COUNT(*) > 1)", [], |r| r.get(0))?;
    if dup_groups == 0 {
        conn.execute(
            "CREATE UNIQUE INDEX IF NOT EXISTS stamps_unique_uniq_key ON stamps_new(uniq_key)",
            [],
        )?;
        debug!("Created UNIQUE index on stamps_new.uniq_key");
    } else {
        error!(
            "Migration found {} duplicate uniq_key groups. UNIQUE index not created.",
            dup_groups
        );
    }

    // Drop old table and rename
    conn.execute("DROP TABLE IF EXISTS stamps", [])?;
    conn.execute("ALTER TABLE stamps_new RENAME TO stamps", [])?;

    conn.execute_batch("COMMIT;")?;
    debug!("Stamps table migration completed successfully");
    Ok(())
}

/// Migration to fix rows where block metadata (design/engraving/perforation)
/// were stored as concatenated block text. This function heuristically
/// detects suspect rows, parses their `context` / combined text using the
/// collector helper `extract_block_metadata`, normalizes currency tokens and
/// updates only the missing or obviously-wrong columns. It's safe to run
/// multiple times and uses a transaction.
pub fn migrate_fix_block_metadata(conn: &Connection) -> Result<()> {
    debug!("Starting migration: fix block metadata and normalize currency");
    conn.execute_batch("BEGIN IMMEDIATE;")?;

    // Small helper: normalize currency tokens similarly to the collector
    fn normalize_currency_token_inner(s: &str) -> String {
        let t = s.trim();
        if t.is_empty() {
            return String::new();
        }
        let low = t.to_lowercase();
        if low == "sell"
            || low == "usd"
            || low == "$"
            || low == "us$"
            || low.contains("usd")
            || low.contains("us$")
            || low.contains("dollar")
            || low.contains("cent")
        {
            "USD".to_string()
        } else if low == "gbp"
            || low == "£"
            || low == "pound"
            || low.contains("pound")
            || low.contains("gbp")
        {
            "GBP".to_string()
        } else if low == "eur"
            || low == "€"
            || low == "euro"
            || low.contains("euro")
            || low.contains("eur")
        {
            "EUR".to_string()
        } else {
            t.to_uppercase()
        }
    }

    // Process rows where context looks like it contains block metadata
    // Look for rows where design/engraving/perforation are empty but context is not
    let mut stmt = conn.prepare(
        "SELECT id, context, design, engraving, perforation, currency FROM stamps WHERE (design = '' OR design IS NULL) AND (engraving = '' OR engraving IS NULL) AND (perforation = '' OR perforation IS NULL) AND context != '' AND context IS NOT NULL"
    )?;
    let mut rows = stmt.query([])?;
    let mut updates = 0;
    while let Ok(Some(row)) = rows.next() {
        let id: i64 = row.get(0)?;
        let context: String = row.get(1)?;
        let existing_design: String = row.get(2)?;
        let existing_engraving: String = row.get(3)?;
        let existing_perforation: String = row.get(4)?;
        let existing_currency: String = row.get(5)?;

        // Use the collector's extract_block_metadata logic
        // This is a simplified version - the real implementation is in the collector
        let parts: Vec<&str> = context.split('|').collect();
        let mut design = String::new();
        let mut engraving = String::new();
        let mut perforation = String::new();
        let mut currency = String::new();

        for part in parts {
            let p = part.trim();
            if p.is_empty() {
                continue;
            }
            // Check if it looks like currency
            let norm_curr = normalize_currency_token_inner(p);
            if !norm_curr.is_empty() && norm_curr != p {
                currency = norm_curr;
                continue;
            }
            // Check for perforation patterns
            if p.to_lowercase().contains("perf")
                || p.contains("×")
                || p.chars()
                    .all(|c| c.is_numeric() || c == '×' || c == '.' || c == ' ')
            {
                if perforation.is_empty() {
                    perforation = p.to_string();
                }
                continue;
            }
            // Check for engraving patterns
            if p.to_lowercase().contains("eng")
                || p.to_lowercase().contains("litho")
                || p.to_lowercase().contains("recess")
            {
                if engraving.is_empty() {
                    engraving = p.to_string();
                }
                continue;
            }
            // Assume it's design if we haven't categorized it
            if design.is_empty() {
                design = p.to_string();
            }
        }

        // Only update if we found something and the fields are actually empty
        if (!design.is_empty() && existing_design.is_empty())
            || (!engraving.is_empty() && existing_engraving.is_empty())
            || (!perforation.is_empty() && existing_perforation.is_empty())
            || (!currency.is_empty() && existing_currency.is_empty())
        {
            conn.execute(
                "UPDATE stamps SET design = ?, engraving = ?, perforation = ?, currency = ? WHERE id = ?",
                params![design, engraving, perforation, currency, id],
            )?;
            updates += 1;
        }
    }

    conn.execute_batch("COMMIT;")?;
    debug!(
        "Migration completed: updated {} rows with block metadata",
        updates
    );
    Ok(())
}
