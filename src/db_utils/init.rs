//! Database initialization utilities.
//!
//! This module contains functions for initializing and setting up the database.

use log::{debug, error};
use rusqlite::{Connection, Result};

/// Initialize the database connection and ensure the stamps table exists.
///
/// This function creates the database file if it doesn't exist, ensures the
/// parent directory exists, creates the stamps table with the proper schema,
/// and runs any necessary migrations.
pub fn init_db(db_path: &str) -> Result<Connection> {
    // Ensure parent directory for the database exists so SQLite can create the file
    if let Some(parent) = std::path::Path::new(db_path).parent() {
        if !parent.as_os_str().is_empty() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                error!(
                    "Failed to create parent directory for DB {}: {}",
                    parent.display(),
                    e
                );
                // Continue and attempt to open the DB - the original behaviour was to fail here;
                // returning the error from Connection::open will surface the problem to callers.
            }
        }
    }
    let conn = Connection::open(db_path)?;
    // Log the path opened and some filesystem metadata to help debug DB path mismatches
    let canonical =
        std::fs::canonicalize(db_path).unwrap_or_else(|_| std::path::PathBuf::from(db_path));
    debug!(
        "Opened database at {} (canonical: {})",
        db_path,
        canonical.display()
    );
    match std::fs::metadata(&canonical) {
        Ok(md) => debug!(
            "Database file exists. size={} bytes, readonly={}",
            md.len(),
            md.permissions().readonly()
        ),
        Err(_) => debug!("Database file does not exist yet: {}", canonical.display()),
    }
    // Always create the table first
    conn.execute(
        "CREATE TABLE IF NOT EXISTS stamps (
            id INTEGER PRIMARY KEY,
            country TEXT,
            series TEXT,
            year INTEGER,
            month TEXT,
            month_num INTEGER,
            day TEXT,
            sg_number TEXT,
            type TEXT,
            denomination TEXT,
            color TEXT,
            description TEXT,
            stamps_issued TEXT,
            mint_condition TEXT,
            unused TEXT,
            used TEXT,
            letter_fdc TEXT,
            currency TEXT,
            image_path TEXT,
            remote_image_url TEXT,
            original_image_url TEXT,
            image_width_px INTEGER,
            image_height_px INTEGER,
            watermark TEXT,
            design TEXT,
            engraving TEXT,
            perforation TEXT,
            context TEXT
        )",
        [],
    )?;

    // Create colnect_stamps table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS colnect_stamps (
            id INTEGER PRIMARY KEY,
            sg_number TEXT,
            colnect_id TEXT UNIQUE,
            catalog_codes TEXT,
            width TEXT,
            height TEXT,
            printing TEXT,
            paper TEXT,
            gum TEXT,
            themes TEXT,
            colnect_link TEXT,
            description TEXT,
            issued_on TEXT,
            print_run TEXT,
            face_value TEXT,
            currency TEXT,
            colors TEXT,
            designers TEXT,
            printers TEXT
        )",
        [],
    )?;

    // Create index on sg_number for faster joins
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_colnect_stamps_sg_number ON colnect_stamps (sg_number)",
        [],
    )?;

    // Print column names and count after creation
    {
        let pragma = conn.prepare("PRAGMA table_info(stamps)");
        if let Ok(mut stmt) = pragma {
            let mut rows = stmt.query([])?;
            let mut col_names = Vec::new();
            while let Ok(Some(row)) = rows.next() {
                let name: String = row.get(1)?;
                col_names.push(name);
            }
            // eprintln!("[DEBUG] Stamps table columns after creation: {:?} (count: {})", col_names, col_names.len());
        }
    }
    // Now check column count. If it doesn't match our expected schema we should
    // run a safe migration rather than blindly dropping the table which loses
    // existing data. The desired schema currently has 27 columns (including
    // the 'uniq_key' column).
    let mut col_count = 0;
    let mut col_names = Vec::new();
    {
        let pragma = conn.prepare("PRAGMA table_info(stamps)");
        if let Ok(mut stmt) = pragma {
            let mut rows = stmt.query([])?;
            while let Ok(Some(row)) = rows.next() {
                let name: String = row.get(1)?;
                col_names.push(name.clone());
                col_count += 1;
            }
            debug!(
                "Stamps table columns before possible migration: {:?} (count: {})",
                col_names, col_count
            );
        }
    }
    // Expected columns (including uniq_key). We've added image_width_px and image_height_px so the expected count increases by 2.
    let expected_col_count = 29;
    if col_count > 0 && col_count != expected_col_count {
        debug!(
            "Stamps table column count ({}) does not match expected ({}). Running safe migration.",
            col_count, expected_col_count
        );
        match crate::db_utils::migrations::migrate_stamps_table(&conn) {
            Ok(_) => debug!("Safe migration completed from init_db."),
            Err(e) => error!("Safe migration failed from init_db: {}", e),
        }
    }
    // Set this to true to enable debug output, false to disable
    const DEBUG: bool = true;
    if DEBUG {
        debug!("Stamps table ensured.");
    }

    // Migration: add a `uniq_key` column (if missing), backfill it with a
    // canonical dedupe key, then create a UNIQUE index on that column if there
    // are no duplicate keys. This approach is safer when series/country text
    // normalization is required and lets us inspect/backfill before enforcing
    // the constraint.
    // 1) Add column if missing
    let mut has_uniq_col = false;
    {
        let mut stmt = conn.prepare("PRAGMA table_info(stamps)")?;
        let mut rows = stmt.query([])?;
        while let Ok(Some(row)) = rows.next() {
            let name: String = row.get(1)?;
            if name == "uniq_key" {
                has_uniq_col = true;
                break;
            }
        }
    }
    if !has_uniq_col {
        debug!("Adding column 'uniq_key' to stamps table");
        match conn.execute("ALTER TABLE stamps ADD COLUMN uniq_key TEXT", []) {
            Ok(_) => debug!("Added uniq_key column."),
            Err(e) => error!("Failed to add uniq_key column: {}", e),
        }
    }

    // 2) Backfill uniq_key using a normalized concatenation; use COALESCE to
    // guard against NULLs and lower() for basic normalization.
    match conn.execute(
        "UPDATE stamps SET uniq_key = lower(coalesce(country,'')) || '|' || coalesce(series,'') || '|' || coalesce(sg_number,'') WHERE uniq_key IS NULL OR uniq_key = ''",
        [],
    ) {
        Ok(n) => debug!("Backfilled uniq_key for {} rows", n),
        Err(e) => error!("Failed to backfill uniq_key: {}", e),
    }

    // 3) Check for duplicates on uniq_key; only create UNIQUE index if none exist
    match conn.query_row(
        "SELECT COUNT(*) FROM (SELECT uniq_key FROM stamps GROUP BY uniq_key HAVING COUNT(*) > 1);",
        [],
        |r| r.get::<_, i64>(0),
    ) {
        Ok(dup_groups) => {
            if dup_groups == 0 {
                match conn.execute(
                    "CREATE UNIQUE INDEX IF NOT EXISTS stamps_unique_uniq_key ON stamps(uniq_key);",
                    [],
                ) {
                    Ok(_) => debug!("Created UNIQUE index stamps_unique_uniq_key"),
                    Err(e) => error!(
                        "Failed to create UNIQUE index stamps_unique_uniq_key: {}",
                        e
                    ),
                }
            } else {
                error!("Found {} duplicate uniq_key group(s). UNIQUE index not created. Please deduplicate rows before enabling this constraint.", dup_groups);
            }
        }
        Err(e) => {
            error!("Failed to check for duplicate uniq_key groups: {}. Skipping UNIQUE index creation.", e);
        }
    }

    // Create us1847_series table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS us1847_series (
            id INTEGER PRIMARY KEY,
            series_name TEXT,
            series_url TEXT UNIQUE,
            description TEXT,
            president TEXT,
            postmaster_general TEXT,
            domestic_letter_rate TEXT,
            postcard_rate TEXT,
            airmail_rate TEXT,
            printing_info TEXT,
            tagging_info TEXT,
            size_info TEXT,
            design_size_portrait TEXT,
            design_size_landscape TEXT
        )",
        [],
    )?;

    // Create us1847_stamps table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS us1847_stamps (
            id INTEGER PRIMARY KEY,
            series_id INTEGER,
            scott_number TEXT,
            denomination TEXT,
            name TEXT,
            color TEXT,
            place_issued TEXT,
            issue_date TEXT,
            quantity_issued TEXT,
            designer TEXT,
            vignette TEXT,
            frame TEXT,
            lettering TEXT,
            engraver TEXT,
            image_url TEXT,
            printer TEXT,
            perforation TEXT,
            printing_details TEXT,
            tagging TEXT,
            collectible_varieties TEXT,
            usage TEXT,
            description TEXT,
            stamp_type TEXT,
            FOREIGN KEY(series_id) REFERENCES us1847_series(id),
            UNIQUE(scott_number)
        )",
        [],
    )?;

    // Add color column if it doesn't exist (for backward compatibility)
    conn.execute("ALTER TABLE us1847_stamps ADD COLUMN color TEXT", []).ok();
    conn.execute("UPDATE us1847_stamps SET color = '' WHERE color IS NULL", []).ok();

    // Create us1847_variations table (keeping for future use with varieties)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS us1847_variations (
            id INTEGER PRIMARY KEY,
            stamp_id INTEGER,
            variation_type TEXT,
            description TEXT,
            FOREIGN KEY(stamp_id) REFERENCES us1847_stamps(id)
        )",
        [],
    )?;

    Ok(conn)
}
