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
            stampworld_image_url TEXT,
            colnect_page_url TEXT,
            colnect_image_url TEXT,
            colnect_id TEXT,
            colnect_name TEXT,
            colnect_catalog_codes TEXT,
            colnect_country TEXT,
            colnect_width TEXT,
            colnect_height TEXT,
            colnect_format TEXT,
            colnect_emission TEXT,
            colnect_printing TEXT,
            colnect_paper TEXT,
            colnect_gum TEXT,
            colnect_ord_mi TEXT,
            colnect_ord_sn TEXT,
            colnect_ord_yt TEXT,
            colnect_ord_sg TEXT,
            colnect_expiry_date TEXT,
            colnect_print_run TEXT,
            colnect_face_value TEXT,
            colnect_currency TEXT,
            colnect_colors TEXT,
            colnect_parent_item TEXT,
            colnect_variant TEXT,
            colnect_overprint_variant TEXT,
            colnect_score TEXT,
            colnect_accuracy TEXT,
            colnect_list TEXT,
            colnect_quantity TEXT,
            colnect_condition TEXT,
            colnect_public_note TEXT,
            colnect_private_note TEXT,
            image_width_px INTEGER,
            image_height_px INTEGER,
            watermark TEXT,
            design TEXT,
            engraving TEXT,
            perforation TEXT,
            context TEXT,
            uniq_key TEXT DEFAULT '',
            issue_location TEXT DEFAULT '',
            quantity_printed TEXT DEFAULT ''
        )",
        [],
    )?;

    // Create colnect_stamps table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS colnect_stamps (
            id INTEGER PRIMARY KEY,
            sg_number TEXT,
            colnect_id TEXT UNIQUE,
            name TEXT,
            catalog_codes TEXT,
            country TEXT,
            width TEXT,
            height TEXT,
            watermark TEXT,
            format TEXT,
            emission TEXT,
            perforation TEXT,
            printing TEXT,
            paper TEXT,
            gum TEXT,
            ord_mi TEXT,
            ord_sn TEXT,
            ord_yt TEXT,
            ord_sg TEXT,
            themes TEXT,
            colnect_link TEXT,
            description TEXT,
            issued_on TEXT,
            expiry_date TEXT,
            print_run TEXT,
            face_value TEXT,
            currency TEXT,
            colors TEXT,
            designers TEXT,
            printers TEXT,
            series TEXT,
            parent_item TEXT,
            variant TEXT,
            overprint_variant TEXT,
            score TEXT,
            accuracy TEXT,
            list TEXT,
            quantity TEXT,
            condition TEXT,
            public_note TEXT,
            private_note TEXT,
            is_deleted INTEGER DEFAULT 0,
            deleted_at TEXT DEFAULT '',
            is_processed INTEGER DEFAULT 0
        )",
        [],
    )?;

    // Create index on sg_number for faster joins
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_colnect_stamps_sg_number ON colnect_stamps (sg_number)",
        [],
    )?;

    // Create colnect_catalog_codes table for storing individual catalog codes
    conn.execute(
        "CREATE TABLE IF NOT EXISTS colnect_catalog_codes (
            id INTEGER PRIMARY KEY,
            colnect_id TEXT,
            catalog_type TEXT,
            code TEXT,
            UNIQUE(colnect_id, catalog_type)
        )",
        [],
    )?;

    // Create index on colnect_id for faster lookups
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_colnect_catalog_codes_colnect_id ON colnect_catalog_codes (colnect_id)",
        [],
    )?;

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
            design_size TEXT,
            FOREIGN KEY(series_id) REFERENCES us1847_series(id),
            UNIQUE(scott_number)
        )",
        [],
    )?;

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

    crate::db_utils::migrations::ensure_schema_migrations_table(&conn)?;
    crate::db_utils::migrations::run_pending_schema_migrations(&conn)?;
    debug!("Database schema ensured via numbered migrations.");

    Ok(conn)
}
