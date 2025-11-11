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
    // Return Some(normalized_currency) for recognized currency-like tokens,
    // otherwise return None. Previously this helper returned the uppercased
    // input for unrecognized tokens which caused nearly every non-empty
    // block line to be misclassified as a currency because the returned
    // string differed from the original. Using Option prevents that.
    fn normalize_currency_token_inner(s: &str) -> Option<String> {
        let t = s.trim();
        if t.is_empty() {
            return None;
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
            Some("USD".to_string())
        } else if low == "gbp"
            || low == "£"
            || low == "pound"
            || low.contains("pound")
            || low.contains("gbp")
        {
            Some("GBP".to_string())
        } else if low == "eur"
            || low == "€"
            || low == "euro"
            || low.contains("euro")
            || low.contains("eur")
        {
            Some("EUR".to_string())
        } else {
            None
        }
    }

    // Process rows where block metadata may be embedded. We select rows where
    // either the `context` is non-empty (collector often stored blocks there)
    // or any of the design/engraving/perforation columns look like they
    // contain a labelled block (contain a ':' character). This covers cases
    // where earlier collectors accidentally wrote the whole block into one
    // column (e.g. "Design: X\nEngraving: Y\nPerforation: Z").
    let mut stmt = conn.prepare(
        "SELECT id, context, design, engraving, perforation, currency FROM stamps WHERE (context != '' AND context IS NOT NULL) OR (design LIKE '%:%') OR (engraving LIKE '%:%') OR (perforation LIKE '%:%')"
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

        // Consolidate potential sources of the block text. Prefer `context`
        // when available because it's the canonical place collectors stored
        // block text, but also include the other fields in case the block was
        // accidentally stored there.
        let mut combined = String::new();
        if !context.trim().is_empty() {
            combined.push_str(&context);
        }
        if !existing_design.trim().is_empty() {
            if !combined.is_empty() {
                combined.push('|');
            }
            combined.push_str(&existing_design);
        }
        if !existing_engraving.trim().is_empty() {
            if !combined.is_empty() {
                combined.push('|');
            }
            combined.push_str(&existing_engraving);
        }
        if !existing_perforation.trim().is_empty() {
            if !combined.is_empty() {
                combined.push('|');
            }
            combined.push_str(&existing_perforation);
        }

        // Split the combined block on pipe and newlines so each labelled line
        // becomes an individual part we can classify. Collect into owned
        // Strings to avoid borrowing from a temporary created by `replace`.
        let parts: Vec<String> = combined
            .replace('\r', "")
            .split(|c| c == '|' || c == '\n')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let mut design = String::new();
        let mut engraving = String::new();
        let mut perforation = String::new();
        let mut currency = String::new();

        for part in parts.iter() {
            let p = part.as_str().trim();
            if p.is_empty() {
                continue;
            }
            // Check if it looks like currency
            // Only treat the part as a currency when the helper recognizes it
            // as a known currency token (e.g. "sell" -> "USD"). This avoids
            // misclassifying generic labelled lines like "Design: X" as
            // currency due to case-normalization differences.
            if let Some(norm_curr) = normalize_currency_token_inner(p) {
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
                    // If the part contains a colon like "Perforation: 12" prefer
                    // the RHS after the colon.
                    if let Some(idx) = p.find(':') {
                        perforation = p[idx + 1..].trim().to_string();
                    } else {
                        perforation = p.to_string();
                    }
                }
                continue;
            }
            // Check for engraving patterns
            if p.to_lowercase().contains("eng")
                || p.to_lowercase().contains("litho")
                || p.to_lowercase().contains("recess")
            {
                if engraving.is_empty() {
                    if let Some(idx) = p.find(':') {
                        engraving = p[idx + 1..].trim().to_string();
                    } else {
                        engraving = p.to_string();
                    }
                }
                continue;
            }
            // Assume it's design if we haven't categorized it
            if design.is_empty() {
                if let Some(idx) = p.find(':') {
                    design = p[idx + 1..].trim().to_string();
                } else {
                    design = p.to_string();
                }
            }
        }

        // Decide whether to replace fields. We will replace design/engraving/
        // perforation when the existing value looks like a concatenated block
        // (contains ':' or newlines) or is empty. For currency, normalize even
        // if a token was present (e.g. "Sell" -> "USD").
        let replace_design = (!design.is_empty())
            && (existing_design.is_empty()
                || existing_design.contains(':')
                || existing_design.contains('\n')
                || existing_design.trim() == context.trim());
        let replace_engraving = (!engraving.is_empty())
            && (existing_engraving.is_empty()
                || existing_engraving.contains(':')
                || existing_engraving.contains('\n')
                || existing_engraving.trim() == context.trim());
        let replace_perforation = (!perforation.is_empty())
            && (existing_perforation.is_empty()
                || existing_perforation.contains(':')
                || existing_perforation.contains('\n')
                || existing_perforation.trim() == context.trim());

        // For currency: prefer an explicitly-extracted token from the block parts
        // if available; otherwise, attempt to normalize an existing currency
        // value (e.g. "Sell" -> "USD"). If neither yields a recognized token
        // preserve the existing value.
        let new_currency = if !currency.is_empty() {
            currency.clone()
        } else if let Some(norm) = normalize_currency_token_inner(&existing_currency) {
            norm
        } else {
            existing_currency.clone()
        };

        let replace_currency = new_currency.to_lowercase() != existing_currency.to_lowercase();

        let new_design = if replace_design {
            design
        } else {
            existing_design
        };
        let new_engraving = if replace_engraving {
            engraving
        } else {
            existing_engraving
        };
        let new_perforation = if replace_perforation {
            perforation
        } else {
            existing_perforation
        };

        if replace_design || replace_engraving || replace_perforation || replace_currency {
            conn.execute(
                "UPDATE stamps SET design = ?, engraving = ?, perforation = ?, currency = ? WHERE id = ?",
                params![new_design, new_engraving, new_perforation, new_currency, id],
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
