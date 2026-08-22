//! Database migration utilities.
//!
//! This module contains functions for migrating database schemas and data.

use log::{debug, error};
use rusqlite::{params, Connection, Result};
use std::collections::HashSet;

const LATEST_SCHEMA_VERSION: i64 = 4;

// Type alias for a migration entry: (version, name, function)
type MigrationEntry = (i64, &'static str, fn(&Connection) -> Result<()>);

const CANONICAL_STAMPS_COLUMNS: [(&str, &str); 65] = [
    ("id", "INTEGER PRIMARY KEY"),
    ("country", "TEXT"),
    ("series", "TEXT"),
    ("year", "INTEGER"),
    ("month", "TEXT"),
    ("month_num", "INTEGER"),
    ("day", "TEXT"),
    ("sg_number", "TEXT"),
    ("catalog_code", "TEXT DEFAULT ''"),
    ("type", "TEXT"),
    ("denomination", "TEXT"),
    ("color", "TEXT"),
    ("description", "TEXT"),
    ("stamps_issued", "TEXT"),
    ("mint_condition", "TEXT"),
    ("unused", "TEXT"),
    ("used", "TEXT"),
    ("letter_fdc", "TEXT"),
    ("currency", "TEXT"),
    ("image_path", "TEXT"),
    ("remote_image_url", "TEXT"),
    ("original_image_url", "TEXT"),
    ("stampworld_image_url", "TEXT DEFAULT ''"),
    ("colnect_page_url", "TEXT DEFAULT ''"),
    ("colnect_image_url", "TEXT DEFAULT ''"),
    ("colnect_id", "TEXT DEFAULT ''"),
    ("colnect_name", "TEXT DEFAULT ''"),
    ("colnect_catalog_codes", "TEXT DEFAULT ''"),
    ("colnect_country", "TEXT DEFAULT ''"),
    ("colnect_width", "TEXT DEFAULT ''"),
    ("colnect_height", "TEXT DEFAULT ''"),
    ("colnect_format", "TEXT DEFAULT ''"),
    ("colnect_emission", "TEXT DEFAULT ''"),
    ("colnect_printing", "TEXT DEFAULT ''"),
    ("colnect_paper", "TEXT DEFAULT ''"),
    ("colnect_gum", "TEXT DEFAULT ''"),
    ("colnect_ord_mi", "TEXT DEFAULT ''"),
    ("colnect_ord_sn", "TEXT DEFAULT ''"),
    ("colnect_ord_yt", "TEXT DEFAULT ''"),
    ("colnect_ord_sg", "TEXT DEFAULT ''"),
    ("colnect_expiry_date", "TEXT DEFAULT ''"),
    ("colnect_print_run", "TEXT DEFAULT ''"),
    ("colnect_face_value", "TEXT DEFAULT ''"),
    ("colnect_currency", "TEXT DEFAULT ''"),
    ("colnect_colors", "TEXT DEFAULT ''"),
    ("colnect_parent_item", "TEXT DEFAULT ''"),
    ("colnect_variant", "TEXT DEFAULT ''"),
    ("colnect_overprint_variant", "TEXT DEFAULT ''"),
    ("colnect_score", "TEXT DEFAULT ''"),
    ("colnect_accuracy", "TEXT DEFAULT ''"),
    ("colnect_list", "TEXT DEFAULT ''"),
    ("colnect_quantity", "TEXT DEFAULT ''"),
    ("colnect_condition", "TEXT DEFAULT ''"),
    ("colnect_public_note", "TEXT DEFAULT ''"),
    ("colnect_private_note", "TEXT DEFAULT ''"),
    ("image_width_px", "INTEGER"),
    ("image_height_px", "INTEGER"),
    ("watermark", "TEXT"),
    ("design", "TEXT"),
    ("engraving", "TEXT"),
    ("perforation", "TEXT"),
    ("context", "TEXT"),
    ("uniq_key", "TEXT DEFAULT ''"),
    ("issue_location", "TEXT DEFAULT ''"),
    ("quantity_printed", "TEXT DEFAULT ''"),
];

const COLNECT_STAMPS_MIGRATION_COLUMNS: [(&str, &str); 41] = [
    ("name", "TEXT"),
    ("catalog_codes", "TEXT"),
    ("country", "TEXT"),
    ("width", "TEXT"),
    ("height", "TEXT"),
    ("watermark", "TEXT"),
    ("format", "TEXT"),
    ("emission", "TEXT"),
    ("perforation", "TEXT"),
    ("printing", "TEXT"),
    ("paper", "TEXT"),
    ("gum", "TEXT"),
    ("ord_mi", "TEXT"),
    ("ord_sn", "TEXT"),
    ("ord_yt", "TEXT"),
    ("ord_sg", "TEXT"),
    ("themes", "TEXT"),
    ("colnect_link", "TEXT"),
    ("description", "TEXT"),
    ("issued_on", "TEXT"),
    ("expiry_date", "TEXT"),
    ("print_run", "TEXT"),
    ("face_value", "TEXT"),
    ("currency", "TEXT"),
    ("colors", "TEXT"),
    ("designers", "TEXT"),
    ("printers", "TEXT"),
    ("series", "TEXT"),
    ("parent_item", "TEXT"),
    ("variant", "TEXT"),
    ("overprint_variant", "TEXT"),
    ("score", "TEXT"),
    ("accuracy", "TEXT"),
    ("list", "TEXT"),
    ("quantity", "TEXT"),
    ("condition", "TEXT"),
    ("public_note", "TEXT"),
    ("private_note", "TEXT"),
    ("is_deleted", "INTEGER DEFAULT 0"),
    ("deleted_at", "TEXT DEFAULT ''"),
    ("is_processed", "INTEGER DEFAULT 0"),
];

fn table_column_names(conn: &Connection, table_name: &str) -> Result<Vec<String>> {
    let pragma = format!("PRAGMA table_info({})", table_name);
    let mut stmt = conn.prepare(&pragma)?;
    let mut rows = stmt.query([])?;
    let mut columns = Vec::new();
    while let Ok(Some(row)) = rows.next() {
        columns.push(row.get::<_, String>(1)?);
    }
    Ok(columns)
}

fn table_column_name_set(conn: &Connection, table_name: &str) -> Result<HashSet<String>> {
    Ok(table_column_names(conn, table_name)?.into_iter().collect())
}

fn ensure_columns(conn: &Connection, table_name: &str, columns: &[(&str, &str)]) -> Result<()> {
    let existing = table_column_name_set(conn, table_name)?;
    for (name, definition) in columns {
        if existing.contains(*name) {
            continue;
        }
        let sql = format!(
            "ALTER TABLE {} ADD COLUMN {} {}",
            table_name, name, definition
        );
        conn.execute(&sql, [])?;
        debug!("Added missing {} column '{}'", table_name, name);
    }
    Ok(())
}

fn cleanup_legacy_stampworld_identity_duplicates(conn: &Connection) -> Result<usize> {
    let mut group_stmt = conn.prepare(
        "SELECT lower(coalesce(country, '')) AS country_key,
                trim(coalesce(stampworld_image_url, '')) AS sw_img,
                min(rowid) AS keep_rowid,
                count(*) AS cnt
         FROM stamps
         WHERE trim(coalesce(stampworld_image_url, '')) != ''
         GROUP BY lower(coalesce(country, '')), trim(coalesce(stampworld_image_url, ''))
         HAVING count(*) > 1",
    )?;

    let groups = group_stmt
        .query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, i64>(2)?,
                r.get::<_, i64>(3)?,
            ))
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

    let mut deleted = 0usize;
    for (country_key, sw_img, keep_rowid, cnt) in groups {
        let n = conn.execute(
            "DELETE FROM stamps
             WHERE rowid != ?1
               AND lower(coalesce(country, '')) = ?2
               AND trim(coalesce(stampworld_image_url, '')) = ?3",
            rusqlite::params![keep_rowid, country_key, sw_img],
        )?;
        if n > 0 {
            deleted += n;
            debug!(
                "Deduped {} legacy row(s) for country='{}' stampworld_image_url='{}' (kept rowid={}, group_size={})",
                n, country_key, sw_img, keep_rowid, cnt
            );
        }
    }

    if deleted > 0 {
        let _ = conn.execute(
            "UPDATE stamps
             SET uniq_key = lower(coalesce(country,'')) || '|' || coalesce(series,'') || '|' || coalesce(sg_number,'')
             WHERE uniq_key IS NULL OR uniq_key = ''",
            [],
        );
    }

    Ok(deleted)
}

pub fn ensure_schema_migrations_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    Ok(())
}

pub fn run_pending_schema_migrations(conn: &Connection) -> Result<()> {
    ensure_schema_migrations_table(conn)?;

    let mut stmt = conn.prepare("SELECT version FROM schema_migrations")?;
    let applied = stmt
        .query_map([], |row| row.get::<_, i64>(0))?
        .collect::<std::result::Result<HashSet<_>, _>>()?;

    let migrations: [MigrationEntry; 4] = [
        (
            1,
            "stamps_schema_alignment",
            migration_001_stamps_schema_alignment,
        ),
        (
            2,
            "stamps_uniq_key_and_indexes",
            migration_002_stamps_uniq_key_and_indexes,
        ),
        (
            3,
            "colnect_schema_alignment",
            migration_003_colnect_schema_alignment,
        ),
        (
            4,
            "us1847_compat_columns",
            migration_004_us1847_compat_columns,
        ),
    ];

    for (version, name, migrate) in migrations {
        if applied.contains(&version) {
            continue;
        }

        debug!("Applying schema migration {} ({})", version, name);
        migrate(conn)?;
        conn.execute(
            "INSERT INTO schema_migrations (version, name) VALUES (?1, ?2)",
            params![version, name],
        )?;
        debug!("Applied schema migration {} ({})", version, name);
    }

    // Always run runtime compatibility repair, even when all migration
    // versions were already marked as applied.
    enforce_runtime_schema_compatibility(conn)?;

    debug!("Schema is at version {}", LATEST_SCHEMA_VERSION);
    Ok(())
}

fn migration_001_stamps_schema_alignment(conn: &Connection) -> Result<()> {
    ensure_columns(conn, "stamps", &CANONICAL_STAMPS_COLUMNS[1..])?;

    let existing = table_column_name_set(conn, "stamps")?;
    let desired: HashSet<String> = CANONICAL_STAMPS_COLUMNS
        .iter()
        .map(|(name, _)| (*name).to_string())
        .collect();

    if existing != desired {
        debug!(
            "Stamps schema still differs from canonical after additive repair. Running safe rebuild. existing={:?}, desired={:?}",
            existing, desired
        );
        migrate_stamps_table(conn)?;
    }

    Ok(())
}

fn migration_002_stamps_uniq_key_and_indexes(conn: &Connection) -> Result<()> {
    match cleanup_legacy_stampworld_identity_duplicates(conn) {
        Ok(n) if n > 0 => debug!(
            "Removed {} legacy duplicate StampWorld identity row(s) during schema migration",
            n
        ),
        Ok(_) => {}
        Err(e) => error!(
            "Failed legacy StampWorld identity dedupe during schema migration: {}",
            e
        ),
    }

    conn.execute(
        "UPDATE stamps
         SET uniq_key = lower(coalesce(country,'')) || '|' || coalesce(series,'') || '|' || coalesce(sg_number,'')
         WHERE uniq_key IS NULL OR uniq_key = ''",
        [],
    )?;

    let dup_groups: i64 = conn.query_row(
        "SELECT COUNT(*)
         FROM (
             SELECT uniq_key
             FROM stamps
             GROUP BY uniq_key
             HAVING COUNT(*) > 1
         )",
        [],
        |r| r.get(0),
    )?;

    if dup_groups == 0 {
        conn.execute(
            "CREATE UNIQUE INDEX IF NOT EXISTS stamps_unique_uniq_key ON stamps(uniq_key)",
            [],
        )?;
    } else {
        error!(
            "Found {} duplicate uniq_key group(s). UNIQUE index not created.",
            dup_groups
        );
    }

    Ok(())
}

fn migration_003_colnect_schema_alignment(conn: &Connection) -> Result<()> {
    ensure_columns(conn, "colnect_stamps", &COLNECT_STAMPS_MIGRATION_COLUMNS)
}

fn migration_004_us1847_compat_columns(conn: &Connection) -> Result<()> {
    ensure_columns(conn, "us1847_stamps", &[("color", "TEXT DEFAULT ''")])?;
    ensure_columns(
        conn,
        "us1847_series",
        &[
            ("design_size_portrait", "TEXT DEFAULT ''"),
            ("design_size_landscape", "TEXT DEFAULT ''"),
        ],
    )?;
    conn.execute(
        "UPDATE us1847_stamps SET color = '' WHERE color IS NULL",
        [],
    )?;
    Ok(())
}

fn enforce_runtime_schema_compatibility(conn: &Connection) -> Result<()> {
    ensure_columns(conn, "stamps", &CANONICAL_STAMPS_COLUMNS[1..])?;
    ensure_columns(conn, "colnect_stamps", &COLNECT_STAMPS_MIGRATION_COLUMNS)?;
    ensure_columns(conn, "us1847_stamps", &[("color", "TEXT DEFAULT ''")])?;
    ensure_columns(
        conn,
        "us1847_series",
        &[
            ("design_size_portrait", "TEXT DEFAULT ''"),
            ("design_size_landscape", "TEXT DEFAULT ''"),
        ],
    )?;

    conn.execute(
        "UPDATE stamps
         SET uniq_key = lower(coalesce(country,'')) || '|' || coalesce(series,'') || '|' || coalesce(sg_number,'')
         WHERE uniq_key IS NULL OR uniq_key = ''",
        [],
    )?;

    let dup_groups: i64 = conn.query_row(
        "SELECT COUNT(*)
         FROM (
             SELECT uniq_key
             FROM stamps
             GROUP BY uniq_key
             HAVING COUNT(*) > 1
         )",
        [],
        |r| r.get(0),
    )?;
    if dup_groups == 0 {
        conn.execute(
            "CREATE UNIQUE INDEX IF NOT EXISTS stamps_unique_uniq_key ON stamps(uniq_key)",
            [],
        )?;
    }

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_colnect_stamps_sg_number ON colnect_stamps (sg_number)",
        [],
    )?;

    Ok(())
}

/// Robust migration helper: recreate `stamps` table with canonical schema while preserving data.
pub fn migrate_stamps_table(conn: &Connection) -> Result<()> {
    debug!("Starting stamps table migration");
    // We'll execute statements inside an explicit BEGIN/COMMIT to avoid needing
    // a mutable transaction object borrowed from `conn`.
    conn.execute_batch("BEGIN IMMEDIATE;")?;

    // Desired final schema columns and types (same as earlier, plus uniq_key)
    // include image_width_px and image_height_px so the canonical schema contains them
    let desired_cols: Vec<String> = CANONICAL_STAMPS_COLUMNS
        .iter()
        .map(|(name, definition)| format!("{} {}", name, definition))
        .collect();

    // Also keep a simple list of desired column NAMES for matching existing columns.
    let desired_col_names: Vec<&str> = CANONICAL_STAMPS_COLUMNS
        .iter()
        .map(|(name, _)| *name)
        .collect();

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

#[cfg(test)]
mod tests {
    use super::{ensure_schema_migrations_table, run_pending_schema_migrations};
    use rusqlite::Connection;

    fn column_names(conn: &Connection, table_name: &str) -> Vec<String> {
        let pragma = format!("PRAGMA table_info({})", table_name);
        let mut stmt = conn.prepare(&pragma).expect("prepare pragma");
        stmt.query_map([], |row| row.get::<_, String>(1))
            .expect("query table info")
            .collect::<std::result::Result<Vec<_>, _>>()
            .expect("collect columns")
    }

    #[test]
    fn run_pending_schema_migrations_adds_missing_columns_and_records_versions() {
        let conn = Connection::open_in_memory().expect("in-memory db");
        conn.execute(
            "CREATE TABLE stamps (
                id INTEGER PRIMARY KEY,
                country TEXT,
                series TEXT,
                year INTEGER,
                sg_number TEXT,
                image_path TEXT,
                stampworld_image_url TEXT
            )",
            [],
        )
        .expect("create legacy stamps");
        conn.execute(
            "CREATE TABLE colnect_stamps (
                id INTEGER PRIMARY KEY,
                sg_number TEXT,
                colnect_id TEXT UNIQUE
            )",
            [],
        )
        .expect("create legacy colnect_stamps");
        conn.execute("CREATE TABLE us1847_series (id INTEGER PRIMARY KEY)", [])
            .expect("create legacy us1847_series");
        conn.execute("CREATE TABLE us1847_stamps (id INTEGER PRIMARY KEY)", [])
            .expect("create legacy us1847_stamps");

        ensure_schema_migrations_table(&conn).expect("create schema migrations table");
        run_pending_schema_migrations(&conn).expect("run migrations");

        let versions: i64 = conn
            .query_row("SELECT COUNT(*) FROM schema_migrations", [], |row| {
                row.get(0)
            })
            .expect("count migrations");
        assert_eq!(versions, 4);

        let stamps_columns = column_names(&conn, "stamps");
        assert!(stamps_columns.iter().any(|name| name == "uniq_key"));
        assert!(stamps_columns.iter().any(|name| name == "issue_location"));
        assert!(stamps_columns.iter().any(|name| name == "quantity_printed"));

        let colnect_columns = column_names(&conn, "colnect_stamps");
        assert!(colnect_columns.iter().any(|name| name == "series"));
        assert!(colnect_columns.iter().any(|name| name == "is_processed"));

        let us1847_series_columns = column_names(&conn, "us1847_series");
        assert!(us1847_series_columns
            .iter()
            .any(|name| name == "design_size_portrait"));

        let us1847_stamp_columns = column_names(&conn, "us1847_stamps");
        assert!(us1847_stamp_columns.iter().any(|name| name == "color"));
    }

    #[test]
    fn run_pending_schema_migrations_rebuilds_unexpected_stamps_schema() {
        let conn = Connection::open_in_memory().expect("in-memory db");
        conn.execute(
            "CREATE TABLE stamps (
                id INTEGER PRIMARY KEY,
                country TEXT,
                series TEXT,
                year INTEGER,
                sg_number TEXT,
                odd_legacy_column TEXT
            )",
            [],
        )
        .expect("create divergent stamps");
        conn.execute(
            "CREATE TABLE colnect_stamps (
                id INTEGER PRIMARY KEY,
                sg_number TEXT,
                colnect_id TEXT UNIQUE
            )",
            [],
        )
        .expect("create legacy colnect_stamps");
        conn.execute("CREATE TABLE us1847_series (id INTEGER PRIMARY KEY)", [])
            .expect("create legacy us1847_series");
        conn.execute("CREATE TABLE us1847_stamps (id INTEGER PRIMARY KEY)", [])
            .expect("create legacy us1847_stamps");

        ensure_schema_migrations_table(&conn).expect("create schema migrations table");
        run_pending_schema_migrations(&conn).expect("run migrations");

        let stamps_columns = column_names(&conn, "stamps");
        assert!(!stamps_columns
            .iter()
            .any(|name| name == "odd_legacy_column"));
        assert!(stamps_columns.iter().any(|name| name == "quantity_printed"));
        assert!(stamps_columns.iter().any(|name| name == "uniq_key"));
    }

    #[test]
    fn run_pending_schema_migrations_repairs_preapplied_but_drifted_schema() {
        let conn = Connection::open_in_memory().expect("in-memory db");
        conn.execute(
            "CREATE TABLE stamps (
                id INTEGER PRIMARY KEY,
                country TEXT,
                series TEXT,
                year INTEGER,
                sg_number TEXT,
                image_path TEXT
            )",
            [],
        )
        .expect("create legacy stamps");
        conn.execute(
            "CREATE TABLE colnect_stamps (
                id INTEGER PRIMARY KEY,
                sg_number TEXT,
                colnect_id TEXT UNIQUE
            )",
            [],
        )
        .expect("create legacy colnect_stamps");
        conn.execute("CREATE TABLE us1847_series (id INTEGER PRIMARY KEY)", [])
            .expect("create legacy us1847_series");
        conn.execute("CREATE TABLE us1847_stamps (id INTEGER PRIMARY KEY)", [])
            .expect("create legacy us1847_stamps");

        ensure_schema_migrations_table(&conn).expect("create schema migrations table");
        conn.execute(
            "INSERT INTO schema_migrations(version, name) VALUES
                (1, 'stamps_schema_alignment'),
                (2, 'stamps_uniq_key_and_indexes'),
                (3, 'colnect_schema_alignment'),
                (4, 'us1847_compat_columns')",
            [],
        )
        .expect("seed schema versions");

        run_pending_schema_migrations(&conn).expect("run migrations with pre-applied versions");

        let stamps_columns = column_names(&conn, "stamps");
        assert!(stamps_columns.iter().any(|name| name == "catalog_code"));
        assert!(stamps_columns.iter().any(|name| name == "uniq_key"));

        let colnect_columns = column_names(&conn, "colnect_stamps");
        assert!(colnect_columns.iter().any(|name| name == "series"));

        let us1847_stamp_columns = column_names(&conn, "us1847_stamps");
        assert!(us1847_stamp_columns.iter().any(|name| name == "color"));
    }
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
            .split(&['|', '\n'][..])
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
