#![allow(dead_code)]
// Returns a list of distinct series names (ordered by series name)
pub fn get_all_series(conn: &rusqlite::Connection) -> rusqlite::Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT DISTINCT series FROM stamps ORDER BY series")?;
    let series_iter = stmt.query_map([], |row| {
        Ok(row.get::<_, String>(0)?)
    })?;
    let mut result = Vec::new();
    for s in series_iter {
        result.push(s?);
    }
    Ok(result)
}

// Helper: extract leading integer prefix from sg_number
fn leading_int(s: &str) -> Option<i64> {
    let s = s.trim();
    let mut digits = String::new();
    for ch in s.chars() {
        if ch.is_ascii_digit() {
            digits.push(ch);
        } else {
            break;
        }
    }
    if digits.is_empty() {
        None
    } else {
        digits.parse::<i64>().ok()
    }
}

// Create a suffix key that normalizes letters to lower-case and maps
// punctuation like '*' to a high-sort char so letters sort before '*'.
fn suffix_key(s: &str) -> String {
    let s = s.trim();
    // drop leading digits
    let mut chars = s.chars();
    while let Some(c) = chars.clone().next() {
        if c.is_ascii_digit() {
            chars.next();
        } else {
            break;
        }
    }
    let rem: String = chars.collect();
    rem.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                c.to_ascii_lowercase()
            } else if c == '*' {
                '~' // map '*' to a high char so it sorts after letters
            } else {
                // keep other characters but normalize to lowercase where applicable
                c.to_ascii_lowercase()
            }
        })
        .collect()
}

// Public helper so tests can call the same sorting logic without requiring DB IO
pub fn sort_stampstubs_inplace(v: &mut Vec<StampStub>) {
    v.sort_by(|a, b| {
        // Compare year first
        let year_cmp = a.year.cmp(&b.year);
        if year_cmp != std::cmp::Ordering::Equal {
            return year_cmp;
        }
        match (leading_int(&a.sg_number), leading_int(&b.sg_number)) {
            (Some(ai), Some(bi)) => {
                let cmp = ai.cmp(&bi);
                if cmp != std::cmp::Ordering::Equal {
                    return cmp;
                }
                // same numeric prefix: compare suffix keys, empty suffix sorts first
                let a_suf = suffix_key(&a.sg_number);
                let b_suf = suffix_key(&b.sg_number);
                let a_empty = a_suf.is_empty();
                let b_empty = b_suf.is_empty();
                if a_empty != b_empty {
                    return if a_empty { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater };
                }
                // tie-breaker: compare normalized suffix then full string
                a_suf.cmp(&b_suf).then_with(|| a.sg_number.cmp(&b.sg_number))
            }
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.sg_number.cmp(&b.sg_number),
        }
    });
}

#[derive(Debug)]
pub struct SeriesStub {
    pub name: String,
    pub year: i32,
    pub month: String,
    pub day: String,
    pub is_perforated: bool,
    pub perforation: String,
}

// Returns a list of stamps for a given series and year
pub fn get_stamps_by_series(conn: &rusqlite::Connection, series: &str, year: &str) -> rusqlite::Result<Vec<StampStub>> {
    // Decide which query to run and collect rows into a Vec<StampStub>
    let mut result: Vec<StampStub> = Vec::new();
    if year.trim().is_empty() || year == "0" {
        let mut stmt = conn.prepare("SELECT year, month, month_num, day, sg_number, type, denomination, color, description, stamps_issued, image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, series, currency, context, mint_condition, unused, used, letter_fdc FROM stamps WHERE series = ?1 ORDER BY year, sg_number")?;
        let rows = stmt.query_map([series], |row| -> std::result::Result<StampStub, rusqlite::Error> {
            Ok(StampStub {
                year: row.get::<_, i32>(0)?,
                month: row.get(1)?,
                month_num: row.get::<_, i32>(2)?,
                day: row.get(3)?,
                sg_number: row.get(4)?,
                r#type: row.get(5)?,
                stamp_type: row.get(5)?,
                denomination: row.get(6)?,
                color: row.get(7)?,
                description: row.get::<_, String>(8)?,
                desc: row.get(8)?,
                catalog_num: row.get(4)?,
                stamps_issued: row.get(9)?,
                image_path: row.get(10)?,
                remote_image_url: row.get(11)?,
                original_image_url: row.get(12)?,
                watermark: row.get(13)?,
                design: row.get(14)?,
                engraving: row.get(15)?,
                perforation: row.get(16)?,
                series: row.get(17)?,
                currency: row.get(18)?,
                context: row.get(19)?,
                mint_condition: row.get(20)?,
                unused: row.get(21)?,
                used: row.get(22)?,
                letter_fdc: row.get(23)?,
                multi_perf: false,
            })
        })?;
        for s in rows {
            result.push(s?);
        }
    } else {
        let mut stmt = conn.prepare("SELECT year, month, month_num, day, sg_number, type, denomination, color, description, stamps_issued, image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, series, currency, context, mint_condition, unused, used, letter_fdc FROM stamps WHERE series = ?1 AND year = ?2 ORDER BY sg_number")?;
        let rows = stmt.query_map([series, year], |row| -> std::result::Result<StampStub, rusqlite::Error> {
            Ok(StampStub {
                year: row.get::<_, i32>(0)?,
                month: row.get(1)?,
                month_num: row.get::<_, i32>(2)?,
                day: row.get(3)?,
                sg_number: row.get(4)?,
                r#type: row.get(5)?,
                stamp_type: row.get(5)?,
                denomination: row.get(6)?,
                color: row.get(7)?,
                description: row.get::<_, String>(8)?,
                desc: row.get(8)?,
                catalog_num: row.get(4)?,
                stamps_issued: row.get(9)?,
                image_path: row.get(10)?,
                remote_image_url: row.get(11)?,
                original_image_url: row.get(12)?,
                watermark: row.get(13)?,
                design: row.get(14)?,
                engraving: row.get(15)?,
                perforation: row.get(16)?,
                series: row.get(17)?,
                currency: row.get(18)?,
                context: row.get(19)?,
                mint_condition: row.get(20)?,
                unused: row.get(21)?,
                used: row.get(22)?,
                letter_fdc: row.get(23)?,
                multi_perf: false,
            })
        })?;
        for s in rows {
            result.push(s?);
        }
    }
    // apply sorting to the result vector (module-level helper)
    sort_stampstubs_inplace(&mut result);

    Ok(result)
}

#[derive(Debug, Clone)]
pub struct StampStub {
    // Superset of fields used across modules. Some code refers to alternate names;
    // providing both helps during the merge and keeps compatibility.
    pub year: i32,
    pub month: String,
    pub month_num: i32,
    pub day: String,
    pub sg_number: String,
    pub r#type: String,
    pub stamp_type: String,
    pub denomination: String,
    pub color: String,
    pub description: String,
    pub desc: String,
    pub catalog_num: String,
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
    pub series: String,
    pub context: String,
    pub multi_perf: bool,
}
use rusqlite::{params, Connection, Result};
use log::{debug, error};

// Robust migration helper: recreate `stamps` table with canonical schema while preserving data.
pub fn migrate_stamps_table(conn: &Connection) -> Result<()> {
    debug!("Starting stamps table migration");
    // We'll execute statements inside an explicit BEGIN/COMMIT to avoid needing
    // a mutable transaction object borrowed from `conn`.
    conn.execute_batch("BEGIN IMMEDIATE;")?;

    // Desired final schema columns and types (same as earlier, plus uniq_key)
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
        "watermark TEXT",
        "design TEXT",
        "engraving TEXT",
        "perforation TEXT",
        "context TEXT",
        "uniq_key TEXT",
    ];

    // Also keep a simple list of desired column NAMES for matching existing columns.
    let desired_col_names = vec![
        "id", "country", "series", "year", "month", "month_num", "day",
        "sg_number", "type", "denomination", "color", "description",
        "stamps_issued", "mint_condition", "unused", "used", "letter_fdc",
        "currency", "image_path", "remote_image_url", "original_image_url",
        "watermark", "design", "engraving", "perforation", "context", "uniq_key",
    ];

    // Create new table
    let create_sql = format!("CREATE TABLE IF NOT EXISTS stamps_new ({})", desired_cols.join(","));
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
    let common_cols: Vec<String> = existing_cols.iter().filter(|c| desired_col_names.contains(&c.as_str())).cloned().collect();
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

    if !copy_cols.is_empty() {
        let cols_list = copy_cols.join(",");
        let insert_sql = format!("INSERT INTO stamps_new ({0}) SELECT {0} FROM stamps", cols_list);
        conn.execute(&insert_sql, [])?;
        debug!("Data copied into stamps_new");
    }

    // Safety check: if stamps_new is empty after the copy, abort the migration
    // to avoid accidentally dropping the live `stamps` table and losing data.
    let new_count: i64 = conn.query_row("SELECT COUNT(*) FROM stamps_new", [], |r| r.get(0))?;
    if new_count == 0 {
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
        conn.execute("CREATE UNIQUE INDEX IF NOT EXISTS stamps_unique_uniq_key ON stamps_new(uniq_key)", [])?;
        debug!("Created UNIQUE index on stamps_new.uniq_key");
    } else {
        error!("Migration found {} duplicate uniq_key groups. UNIQUE index not created.", dup_groups);
    }

    // Drop old table and rename
    conn.execute("DROP TABLE IF EXISTS stamps", [])?;
    conn.execute("ALTER TABLE stamps_new RENAME TO stamps", [])?;

    conn.execute_batch("COMMIT;")?;
    debug!("Stamps table migration completed successfully");
    Ok(())
}

pub fn init_db(db_path: &str) -> Result<Connection> {
    // Ensure parent directory for the database exists so SQLite can create the file
    if let Some(parent) = std::path::Path::new(db_path).parent() {
        if !parent.as_os_str().is_empty() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                error!("Failed to create parent directory for DB {}: {}", parent.display(), e);
                // Continue and attempt to open the DB - the original behaviour was to fail here;
                // returning the error from Connection::open will surface the problem to callers.
            }
        }
    }
    let conn = Connection::open(db_path)?;
    // Log the path opened and some filesystem metadata to help debug DB path mismatches
    let canonical = std::fs::canonicalize(db_path).unwrap_or_else(|_| std::path::PathBuf::from(db_path));
    debug!("Opened database at {} (canonical: {})", db_path, canonical.display());
    match std::fs::metadata(&canonical) {
        Ok(md) => debug!("Database file exists. size={} bytes, readonly={}", md.len(), md.permissions().readonly()),
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
            watermark TEXT,
            design TEXT,
            engraving TEXT,
            perforation TEXT,
            context TEXT
        )",
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
            debug!("Stamps table columns before possible migration: {:?} (count: {})", col_names, col_count);
        }
    }
    // Expected columns (including uniq_key)
    let expected_col_count = 27;
    if col_count > 0 && col_count != expected_col_count {
        debug!("Stamps table column count ({}) does not match expected ({}). Running safe migration.", col_count, expected_col_count);
        match migrate_stamps_table(&conn) {
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
                match conn.execute("CREATE UNIQUE INDEX IF NOT EXISTS stamps_unique_uniq_key ON stamps(uniq_key);", []) {
                    Ok(_) => debug!("Created UNIQUE index stamps_unique_uniq_key"),
                    Err(e) => error!("Failed to create UNIQUE index stamps_unique_uniq_key: {}", e),
                }
            } else {
                error!("Found {} duplicate uniq_key group(s). UNIQUE index not created. Please deduplicate rows before enabling this constraint.", dup_groups);
            }
        }
        Err(e) => {
            error!("Failed to check for duplicate uniq_key groups: {}. Skipping UNIQUE index creation.", e);
        }
    }

    Ok(conn)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_stub(sg: &str, year: i32) -> StampStub {
        StampStub {
            year,
            month: String::new(),
            month_num: 0,
            day: String::new(),
            sg_number: sg.to_string(),
            r#type: String::new(),
            stamp_type: String::new(),
            denomination: String::new(),
            color: String::new(),
            description: String::new(),
            desc: String::new(),
            catalog_num: sg.to_string(),
            stamps_issued: String::new(),
            mint_condition: String::new(),
            unused: String::new(),
            used: String::new(),
            letter_fdc: String::new(),
            currency: String::new(),
            image_path: String::new(),
            remote_image_url: String::new(),
            original_image_url: String::new(),
            watermark: String::new(),
            design: String::new(),
            engraving: String::new(),
            perforation: String::new(),
            series: String::new(),
            context: String::new(),
            multi_perf: false,
        }
    }

    #[test]
    fn test_numeric_prefix_ordering() {
        let mut v = vec![
            make_stub("10", 1948),
            make_stub("1", 1948),
            make_stub("2", 1948),
            make_stub("4AT", 1948),
            make_stub("4", 1948),
            make_stub("4*", 1948),
            make_stub("A1", 1948),
        ];
        sort_stampstubs_inplace(&mut v);
        let order: Vec<String> = v.into_iter().map(|s| s.sg_number).collect();
        assert_eq!(order, vec!["1", "2", "4", "4AT", "4*", "10", "A1"]);
    }

    #[test]
    fn test_different_years_preserve_year_sort() {
        let mut v = vec![make_stub("1", 1949), make_stub("2", 1948), make_stub("10", 1950)];
        sort_stampstubs_inplace(&mut v);
        let order: Vec<(i32, String)> = v.into_iter().map(|s| (s.year, s.sg_number)).collect();
        assert_eq!(order, vec![(1948, "2".to_string()), (1949, "1".to_string()), (1950, "10".to_string())]);
    }
}

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
    // Check if stamp already exists
    let exists_sql = "SELECT COUNT(*), image_path, remote_image_url FROM stamps WHERE country = ?1 AND series = ?2 AND sg_number = ?3";
    let mut stmt = conn.prepare(exists_sql)?;
    let (count, _existing_image_path, existing_remote_url): (i64, String, String) = stmt.query_row(params![country, series, sg_number], |row| {
        Ok((row.get(0)?, row.get(1).unwrap_or(String::new()), row.get(2).unwrap_or(String::new())))
    })?;
    
    if count > 0 {
        // We may need to update an existing row if the caller provided a new
        // remote_image_url (or original_image_url). Previously we only updated
        // when the existing row contained a placeholder image. That prevented
        // backfilling remote_image_url for rows that already had an image_path
        // but no remote URL recorded. Update when a new remote URL is provided.
    let has_new_remote_url = !remote_image_url.is_empty() && remote_image_url != existing_remote_url;

        if has_new_remote_url {
            debug!("Stamp exists and a new remote_image_url was provided (country='{}', series='{}', sg_number='{}'), updating remote_url/filenames", country, series, sg_number);

            // Only overwrite image_path when the existing value is empty or is a placeholder
            let update_sql = "UPDATE stamps SET 
                remote_image_url = ?1,
                original_image_url = ?2,
                image_path = CASE WHEN (image_path IS NULL OR image_path = '' OR image_path LIKE 'images/no_image%') AND (?3 != '') THEN ?3 ELSE image_path END
                WHERE country = ?4 AND series = ?5 AND sg_number = ?6";

            match conn.execute(update_sql, params![remote_image_url, original_image_url, image_path, country, series, sg_number]) {
                Ok(n) => {
                    debug!("Updated existing stamp remote/original/image_path (rows={}) for {} {} {}", n, country, series, sg_number);
                    return Ok(true);
                }
                Err(e) => {
                    error!("Failed to update existing stamp remote/original url: {} {} {} | Error: {}", country, series, sg_number, e);
                    return Err(e.into());
                }
            }
        }

        debug!("Stamp already exists with valid data (country='{}', series='{}', sg_number='{}'), skipping", country, series, sg_number);
        return Ok(false);
    }
    
    debug!("--- STAMP INSERT ATTEMPT ---");
    debug!("[insert_stamp] country='{}'", country);
    debug!("[insert_stamp] series='{}'", series);
    debug!("[insert_stamp] year={}", year);
    debug!("[insert_stamp] month='{}'", month);
    debug!("[insert_stamp] day='{}'", day);
    debug!("[insert_stamp] sg_number='{}'", sg_number);
    debug!("[insert_stamp] type_='{}'", type_);
    debug!("[insert_stamp] denomination='{}'", denomination);
    debug!("[insert_stamp] color='{}'", color);
    debug!("[insert_stamp] description='{}'", description);
    debug!("[insert_stamp] stamps_issued='{}'", stamps_issued);
    debug!("[insert_stamp] mint_condition='{}'", mint_condition);
    debug!("[insert_stamp] unused='{}'", unused);
    debug!("[insert_stamp] used='{}'", used);
    debug!("[insert_stamp] letter_fdc='{}'", letter_fdc);
    debug!("[insert_stamp] currency='{}'", currency);
    debug!("[insert_stamp] image_path='{}'", image_path);
    debug!("[insert_stamp] remote_image_url='{}'", remote_image_url);
    debug!("[insert_stamp] watermark='{}'", watermark);
    debug!("[insert_stamp] design='{}'", design);
    debug!("[insert_stamp] engraving='{}'", engraving);
    debug!("[insert_stamp] perforation='{}'", perforation);
    debug!("[insert_stamp] context='{}'", context);
    let month_clean = match month.trim().to_lowercase().as_str() {
        "wm" | "" => "",
        "january" | "jan" => "January",
        "february" | "feb" => "February",
        "march" | "mar" => "March",
        "april" | "apr" => "April",
        "may" => "May",
        "june" | "jun" => "June",
        "july" | "jul" => "July",
        "august" | "aug" => "August",
        "september" | "sep" | "sept" => "September",
        "october" | "oct" => "October",
        "november" | "nov" => "November",
        "december" | "dec" => "December",
        _ => "",
    };
    let month_num = match month_clean {
        "January" => 1,
        "February" => 2,
        "March" => 3,
        "April" => 4,
        "May" => 5,
        "June" => 6,
        "July" => 7,
        "August" => 8,
        "September" => 9,
        "October" => 10,
        "November" => 11,
        "December" => 12,
        _ => 0,
    };
    let sql = "INSERT INTO stamps (country, series, year, month, month_num, day, sg_number, type, denomination, color, description, stamps_issued, mint_condition, unused, used, letter_fdc, currency, image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, context) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25)";
    let params_vec = vec![
        country.to_string(),
        series.to_string(),
        year.to_string(),
        month_clean.to_string(),
        month_num.to_string(), // ensure month_num is a String
        day.to_string(),
        sg_number.to_string(),
        type_.to_string(),
        denomination.to_string(),
        color.to_string(),
        description.to_string(),
        stamps_issued.to_string(),
        mint_condition.to_string(),
        unused.to_string(),
        used.to_string(),
        letter_fdc.to_string(),
        currency.to_string(),
        image_path.to_string(),
        remote_image_url.to_string(),
        original_image_url.to_string(),
        watermark.to_string(),
        design.to_string(),
        engraving.to_string(),
        perforation.to_string(),
        context.to_string()
    ];
        debug!("SQL: {}", sql);
        debug!("Params: {:?}", params_vec);
    let mut attempts = 0;
    loop {
        attempts += 1;
    match conn.execute(sql, params![country, series, year, month_clean, month_num, day, sg_number, type_, denomination, color, description, stamps_issued, mint_condition, unused, used, letter_fdc, currency, image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, context]) {
    // Arguments now match the new signature and DB columns
            Ok(_) => {
                    debug!("Insert succeeded on attempt {}", attempts);
                    // Log the last insert row id for tracing
                    let last_id = conn.last_insert_rowid();
                    debug!("Last insert rowid: {}", last_id);
                break;
            }
                Err(e) => {
                    error!("Insert failed on attempt {}: {}", attempts, e);
                if attempts >= 2 {
                        error!("Giving up after {} attempts. Params: {:?}", attempts, params_vec);
                    break;
                } else {
                        debug!("Retrying insert...");
                }
            }        }
    }
    // If we reach here, insert was attempted and we assume it succeeded above; return true to indicate an insert occurred
    Ok(true)
}

pub fn get_stamps_by_country_series(conn: &Connection, country: &str, series: &str) -> Result<Vec<(i32, String, String, i32, String, i32, String, String, String, String, String, String)>> {
    let mut stmt = conn.prepare(
        "SELECT id, country, series, year, month, month_num, day, description, image_path, remote_image_url, perforation, context FROM stamps WHERE country = ?1 AND series = ?2 ORDER BY year"
    )?;
    let stamp_iter = stmt.query_map(params![country, series], |row| {
        Ok((
            row.get(0)?, // id
            row.get(1)?, // country
            row.get(2)?, // series
            row.get(3)?, // year
            row.get(4)?, // month
            row.get::<_, i32>(5)?, // month_num
            row.get(6)?, // day
            row.get::<_, String>(7)?, // description
            row.get(8)?, // image_path
            row.get(9)?, // remote_image_url
            row.get(10)?, // perforation
            row.get(11)? // context
        ))
    })?;
    let mut results = Vec::new();
    for stamp in stamp_iter {
        results.push(stamp?);
    }
    Ok(results)
}