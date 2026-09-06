// Minimal, self-contained DB utilities moved out of the main crate.
use log::{debug, error};
use regex::Regex;
use rusqlite::{params, Connection, OptionalExtension, Result};

// The long-argument `insert_stamp` implementation has been consolidated
// into `insert_stamp_from_struct` to provide a single-struct API and avoid
// high-arity function signatures that trigger clippy warnings.
pub fn insert_stamp_from_struct(conn: &Connection, si: &StampInsert) -> Result<bool> {
    // Extract fields from the struct for local manipulation
    let country = si.country.as_str();
    let series = si.series.as_str();
    let year = si.year;
    let month = si.month.as_str();
    let _month_num = si.month_num.as_str();
    let day = si.day.as_str();
    let sg_number = si.sg_number.as_str();
    let type_ = si.type_.as_str();
    let denomination = si.denomination.as_str();
    let color = si.color.as_str();
    let description = si.description.as_str();
    let stamps_issued = si.stamps_issued.as_str();
    let mint_condition = si.mint_condition.as_str();
    let unused = si.unused.as_str();
    let used = si.used.as_str();
    let letter_fdc = si.letter_fdc.as_str();
    let currency = si.currency.as_str();
    let image_path = si.image_path.as_str();
    let remote_image_url = si.remote_image_url.as_str();
    let original_image_url = si.original_image_url.as_str();
    let watermark = si.watermark.as_str();
    let design = si.design.as_str();
    let engraving = si.engraving.as_str();
    let perforation = si.perforation.as_str();
    let context = si.context.as_str();

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
    let inferred_catalog_code = if !inferred_stampworld_image_url.trim().is_empty() {
        "sw".to_string()
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

    let existing_select = "SELECT rowid, image_path, remote_image_url, original_image_url, description, stamps_issued, mint_condition, unused, used, letter_fdc, currency, watermark, design, engraving, perforation, context, COALESCE(catalog_code, ''), COALESCE(stampworld_image_url, ''), COALESCE(colnect_page_url, ''), COALESCE(colnect_image_url, '') FROM stamps WHERE {predicate} LIMIT 1";
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
                row.get::<_, Option<String>>(19)?.unwrap_or_default(),
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
        existing_catalog_code,
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
            "catalog_code",
            &existing_catalog_code,
            &inferred_catalog_code
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
            catalog_code = CASE WHEN (catalog_code IS NULL OR catalog_code = '') AND (?17 != '') THEN ?17 ELSE catalog_code END,
            stampworld_image_url = CASE WHEN (stampworld_image_url IS NULL OR stampworld_image_url = '') AND (?18 != '') THEN ?18 ELSE stampworld_image_url END,
            colnect_page_url = CASE WHEN (colnect_page_url IS NULL OR colnect_page_url = '') AND (?19 != '') THEN ?19 ELSE colnect_page_url END,
            colnect_image_url = CASE WHEN (colnect_image_url IS NULL OR colnect_image_url = '') AND (?20 != '') THEN ?20 ELSE colnect_image_url END
            WHERE country = ?21 AND year = ?22 AND sg_number = ?23";

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
                inferred_catalog_code,
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
    let insert_sql = "INSERT INTO stamps (country, series, year, month, month_num, day, sg_number, type, denomination, color, description, stamps_issued, mint_condition, unused, used, letter_fdc, currency, image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, context, uniq_key, catalog_code, stampworld_image_url, colnect_page_url, colnect_image_url) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30)";
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
            inferred_catalog_code,
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

/// Minimal owned struct grouping stamp insert parameters.
/// Kept here so the DB crate exposes a simple API independent of
/// the main crate's `src/db_utils/operations.rs` representation.
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

/// Lightweight stamp representation exported by this crate.
#[derive(Debug, Clone)]
pub struct StampStub {
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
    pub image_width_px: Option<i32>,
    pub image_height_px: Option<i32>,
    pub watermark: String,
    pub design: String,
    pub engraving: String,
    pub perforation: String,
    pub series: String,
    pub context: String,
    pub issue_location: String,
    pub quantity_printed: String,
    pub multi_perf: bool,
}

/// Return stamps for a given series/year. Minimal implementation for now.
pub fn get_stamps_by_series(
    _conn: &Connection,
    _series: &str,
    _year: &str,
) -> Result<Vec<StampStub>> {
    Ok(Vec::new())
}

/// Return all series names. Minimal implementation.
pub fn get_all_series(_conn: &Connection) -> Result<Vec<String>> {
    Ok(Vec::new())
}

/// Sort stamp stubs in place. No-op placeholder.
fn leading_int_and_suffix(s: &str) -> (Option<i64>, String) {
    let mut chars = s.chars();
    let mut num_str = String::new();
    while let Some(c) = chars.next() {
        if c.is_ascii_digit() {
            num_str.push(c);
        } else {
            // first non-digit - remainder is the rest of the string starting at this char
            let mut rem = String::new();
            rem.push(c);
            rem.extend(chars);
            return (
                if num_str.is_empty() {
                    None
                } else {
                    num_str.parse::<i64>().ok()
                },
                rem,
            );
        }
    }
    // consumed all chars; whole string may be numeric
    if num_str.is_empty() {
        (None, String::new())
    } else {
        (num_str.parse::<i64>().ok(), String::new())
    }
}

/// Sort stamp stubs in place using natural ordering for numeric prefixes.
pub fn sort_stampstubs_inplace(v: &mut [StampStub]) {
    v.sort_by(|a, b| {
        let (ai, asuf) = leading_int_and_suffix(&a.sg_number);
        let (bi, bsuf) = leading_int_and_suffix(&b.sg_number);
        match (ai, bi) {
            (Some(na), Some(nb)) => {
                if na != nb {
                    return na.cmp(&nb);
                }
                // same leading number: empty suffix sorts before non-empty
                if asuf.is_empty() && !bsuf.is_empty() {
                    return std::cmp::Ordering::Less;
                }
                if !asuf.is_empty() && bsuf.is_empty() {
                    return std::cmp::Ordering::Greater;
                }
                // Prefer alphabetic suffixes before non-alphabetic (e.g. "AT" before "*")
                let a_first_alpha = asuf
                    .chars()
                    .next()
                    .map(|c| c.is_alphabetic())
                    .unwrap_or(false);
                let b_first_alpha = bsuf
                    .chars()
                    .next()
                    .map(|c| c.is_alphabetic())
                    .unwrap_or(false);
                if a_first_alpha && !b_first_alpha {
                    return std::cmp::Ordering::Less;
                }
                if !a_first_alpha && b_first_alpha {
                    return std::cmp::Ordering::Greater;
                }
                return asuf.cmp(&bsuf);
            }
            (Some(_), None) => return std::cmp::Ordering::Less,
            (None, Some(_)) => return std::cmp::Ordering::Greater,
            (None, None) => return a.sg_number.cmp(&b.sg_number),
        }
    });
}

// Lightweight helper stubs used by insert logic. These mirror small
// utilities from the main crate but kept local so the DB crate builds.
fn is_stampworld_image_url(url: &str) -> bool {
    url.contains("stampworld") || url.contains("/stamps/")
}

fn is_colnect_page_url(url: &str) -> bool {
    url.contains("colnect.com/stamps/stamp/")
}

fn is_colnect_image_url(url: &str) -> bool {
    url.contains("colnect.com") && url.contains("/images/")
}

fn is_real_image_candidate(path: &str) -> bool {
    let t = path.trim();
    !t.is_empty() && !t.starts_with("data:")
}

fn is_placeholder_image_path(path: &str) -> bool {
    path.to_lowercase().contains("no_image")
}

fn cleanup_stampworld_identity_duplicates(
    _conn: &Connection,
    _existing_rowid: i64,
    _country: &str,
    _url: &str,
) -> Result<()> {
    // no-op placeholder for now
    Ok(())
}
pub fn init_db(db_path: &str) -> Result<Connection> {
    if let Some(parent) = std::path::Path::new(db_path).parent() {
        if !parent.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all(parent);
        }
    }
    let conn = Connection::open(db_path)?;
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
            catalog_code TEXT DEFAULT '',
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
            context TEXT,
            uniq_key TEXT
        )",
        [],
    )?;
    Ok(conn)
}

// Stubs for heavy migrations; keep compatibility but avoid depending on main crate internals
pub fn migrate_stamps_table(conn: &Connection) -> Result<()> {
    debug!("Running migrate_stamps_table");
    // Check existing columns
    let mut stmt = conn.prepare("PRAGMA table_info('stamps')")?;
    let cols = stmt.query_map([], |row| row.get::<_, String>(1))?;
    let mut existing: Vec<String> = Vec::new();
    for c in cols {
        existing.push(c?);
    }

    conn.execute_batch("BEGIN IMMEDIATE;")?;
    if !existing.iter().any(|c| c == "image_width_px") {
        debug!("Adding column image_width_px");
        conn.execute("ALTER TABLE stamps ADD COLUMN image_width_px INTEGER;", [])?;
    }
    if !existing.iter().any(|c| c == "image_height_px") {
        debug!("Adding column image_height_px");
        conn.execute("ALTER TABLE stamps ADD COLUMN image_height_px INTEGER;", [])?;
    }
    conn.execute_batch("COMMIT;")?;
    debug!("migrate_stamps_table complete");
    Ok(())
}
pub fn migrate_fix_block_metadata(conn: &Connection) -> Result<()> {
    debug!("Starting migration: fix block metadata and normalize currency (db crate)");
    conn.execute_batch("BEGIN IMMEDIATE;")?;

    // Local extractor that looks for lines like "Design: ...", "Engraving: ...", "Perforation: ..."
    fn extract_block_metadata_local(details: &[String]) -> (String, String, String, String) {
        let combined = details.join("\n");
        let mut design = String::new();
        let mut engraving = String::new();
        let mut perforation = String::new();
        let mut watermark = String::new();

        let re_design = Regex::new(r"(?i)design:\s*(.+)").ok();
        let re_engr = Regex::new(r"(?i)engraving:\s*(.+)").ok();
        let re_perf = Regex::new(r"(?i)perforation:\s*([0-9]+)").ok();
        let re_wm = Regex::new(r"(?i)watermark:\s*(.+)").ok();

        if let Some(re) = re_design {
            if let Some(cap) = re.captures(&combined) {
                if let Some(m) = cap.get(1) {
                    design = m.as_str().trim().to_string();
                }
            }
        }
        if let Some(re) = re_engr {
            if let Some(cap) = re.captures(&combined) {
                if let Some(m) = cap.get(1) {
                    engraving = m.as_str().trim().to_string();
                }
            }
        }
        if let Some(re) = re_perf {
            if let Some(cap) = re.captures(&combined) {
                if let Some(m) = cap.get(1) {
                    perforation = m.as_str().trim().to_string();
                }
            }
        }
        if let Some(re) = re_wm {
            if let Some(cap) = re.captures(&combined) {
                if let Some(m) = cap.get(1) {
                    watermark = m.as_str().trim().to_string();
                }
            }
        }

        (watermark, design, engraving, perforation)
    }

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
        {
            return "USD".to_string();
        }
        if low == "eur" || low.contains("€") {
            return "EUR".to_string();
        }
        if low == "gbp" || low.contains("£") {
            return "GBP".to_string();
        }
        t.to_uppercase()
    }

    // Scan rows and find suspects
    let mut stmt =
        conn.prepare("SELECT id, design, engraving, perforation, context, currency FROM stamps")?;
    let mut rows = stmt.query([])?;
    let mut updates: Vec<(i64, String, String, String, String, String)> = Vec::new();
    while let Ok(Some(r)) = rows.next() {
        let id: i64 = r.get(0)?;
        let design: String = r.get::<_, Option<String>>(1)?.unwrap_or_default();
        let engraving: String = r.get::<_, Option<String>>(2)?.unwrap_or_default();
        let perforation: String = r.get::<_, Option<String>>(3)?.unwrap_or_default();
        let context: String = r.get::<_, Option<String>>(4)?.unwrap_or_default();
        let currency: String = r.get::<_, Option<String>>(5)?.unwrap_or_default();

        let suspect = |s: &str| {
            let low = s.to_lowercase();
            low.contains("design:")
                || low.contains("engraving:")
                || low.contains("perforation:")
                || s.contains('\n')
                || s.contains('\r')
        };
        if suspect(&design) || suspect(&engraving) || suspect(&perforation) {
            let mut details: Vec<String> = Vec::new();
            if !context.trim().is_empty() {
                details.push(context.clone());
            }
            if !design.trim().is_empty() {
                details.push(design.clone());
            }
            if !engraving.trim().is_empty() {
                details.push(engraving.clone());
            }
            if !perforation.trim().is_empty() {
                details.push(perforation.clone());
            }
            let (wm_new, design_new, engr_new, perf_new) = extract_block_metadata_local(&details);
            let currency_new = normalize_currency_token_inner(&currency);
            updates.push((id, wm_new, design_new, engr_new, perf_new, currency_new));
        }
    }

    let mut updated_count: i64 = 0;
    for (id, _, _, _, _, _) in updates.iter() {
        let id = *id;
        let row: (String, String, String, String, String) = conn.query_row(
            "SELECT watermark, design, engraving, perforation, currency FROM stamps WHERE id = ?1",
            params![id],
            |r| {
                Ok((
                    r.get::<_, Option<String>>(0)?.unwrap_or_default(),
                    r.get::<_, Option<String>>(1)?.unwrap_or_default(),
                    r.get::<_, Option<String>>(2)?.unwrap_or_default(),
                    r.get::<_, Option<String>>(3)?.unwrap_or_default(),
                    r.get::<_, Option<String>>(4)?.unwrap_or_default(),
                ))
            },
        )?;
        let combined = vec![row.0.clone(), row.1.clone(), row.2.clone(), row.3.clone()];
        let (wm_new, design_new, engr_new, perf_new) = extract_block_metadata_local(&combined);
        let currency_new = normalize_currency_token_inner(&row.4);

        let mut set_clauses: Vec<String> = Vec::new();
        let mut values: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        if !wm_new.is_empty() {
            set_clauses.push("watermark = ?".to_string());
            values.push(Box::new(wm_new));
        }
        if !design_new.is_empty() {
            set_clauses.push("design = ?".to_string());
            values.push(Box::new(design_new));
        }
        if !engr_new.is_empty() {
            set_clauses.push("engraving = ?".to_string());
            values.push(Box::new(engr_new));
        }
        if !perf_new.is_empty() {
            set_clauses.push("perforation = ?".to_string());
            values.push(Box::new(perf_new));
        }
        if !currency_new.is_empty() {
            set_clauses.push("currency = ?".to_string());
            values.push(Box::new(currency_new));
        }

        if !set_clauses.is_empty() {
            let sql = format!("UPDATE stamps SET {} WHERE id = ?", set_clauses.join(", "));
            values.push(Box::new(id));
            let refs: Vec<&dyn rusqlite::ToSql> = values
                .iter()
                .map(|b| &**b as &dyn rusqlite::ToSql)
                .collect();
            if let Ok(rows) = conn.execute(&sql, rusqlite::params_from_iter(refs)) {
                updated_count += rows as i64;
            }
        }
    }

    conn.execute_batch("COMMIT;")?;
    debug!(
        "Completed migration: fix block metadata; updated {} rows",
        updated_count
    );
    Ok(())
}

#[allow(clippy::too_many_arguments)]
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

    let exists_sql = "SELECT COUNT(*), image_path, remote_image_url, description, stamps_issued, mint_condition, unused, used, letter_fdc, currency, watermark, design, engraving, perforation, context FROM stamps WHERE country = ?1 AND series = ?2 AND sg_number = ?3";
    let mut stmt = conn.prepare(exists_sql)?;
    let row_tuple = stmt.query_row(params![country, series, sg_number], |row| {
        Ok((
            row.get::<_, i64>(0)?,
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
    ) = row_tuple;

    if count > 0 {
        // trace removed
        let is_empty = |s: &str| s.trim().is_empty();
        let mut needs_update = false;
        let has_new_remote_url =
            !remote_image_url.trim().is_empty() && remote_image_url != existing_remote_url;
        if has_new_remote_url {
            needs_update = true;
        }
        if !image_path.trim().is_empty()
            && (is_empty(&existing_image_path)
                || existing_image_path.starts_with("images/no_image"))
        {
            needs_update = true;
        }
        if is_empty(&existing_description) && !description.trim().is_empty() {
            needs_update = true;
        }
        if is_empty(&existing_stamps_issued) && !stamps_issued.trim().is_empty() {
            needs_update = true;
        }
        if is_empty(&existing_mint_condition) && !mint_condition.trim().is_empty() {
            needs_update = true;
        }
        if is_empty(&existing_unused) && !unused.trim().is_empty() {
            needs_update = true;
        }
        if is_empty(&existing_used) && !used.trim().is_empty() {
            needs_update = true;
        }
        if is_empty(&existing_letter_fdc) && !letter_fdc.trim().is_empty() {
            needs_update = true;
        }
        if is_empty(&existing_currency) && !currency.trim().is_empty() {
            needs_update = true;
        }
        if is_empty(&existing_watermark) && !watermark.trim().is_empty() {
            needs_update = true;
        }
        if is_empty(&existing_design) && !design.trim().is_empty() {
            needs_update = true;
        }
        if is_empty(&existing_engraving) && !engraving.trim().is_empty() {
            needs_update = true;
        }
        if is_empty(&existing_perforation) && !perforation.trim().is_empty() {
            needs_update = true;
        }
        if is_empty(&existing_context) && !context.trim().is_empty() {
            needs_update = true;
        }

        if needs_update {
            // trace removed
            let update_sql = "UPDATE stamps SET
                image_path = CASE WHEN (?1 != '') AND (image_path IS NULL OR image_path = '' OR image_path LIKE 'images/no_image%') THEN ?1 ELSE image_path END,
                remote_image_url = CASE WHEN (?2 != '') THEN ?2 ELSE remote_image_url END,
                original_image_url = CASE WHEN (?3 != '') THEN ?3 ELSE original_image_url END,
                description = CASE WHEN (description IS NULL OR description = '') AND (?4 != '') THEN ?4 ELSE description END,
                stamps_issued = CASE WHEN (stamps_issued IS NULL OR stamps_issued = '') AND (?5 != '') THEN ?5 ELSE stamps_issued END,
                mint_condition = CASE WHEN (mint_condition IS NULL OR mint_condition = '') AND (?6 != '') THEN ?6 ELSE mint_condition END,
                unused = CASE WHEN (unused IS NULL OR unused = '') AND (?7 != '') THEN ?7 ELSE unused END,
                used = CASE WHEN (used IS NULL OR used = '') AND (?8 != '') THEN ?8 ELSE used END,
                letter_fdc = CASE WHEN (letter_fdc IS NULL OR letter_fdc = '') AND (?9 != '') THEN ?9 ELSE letter_fdc END,
                currency = CASE WHEN (currency IS NULL OR currency = '') AND (?10 != '') THEN ?10 ELSE currency END,
                watermark = CASE WHEN (watermark IS NULL OR watermark = '') AND (?11 != '') THEN ?11 ELSE watermark END,
                design = CASE WHEN (design IS NULL OR design = '') AND (?12 != '') THEN ?12 ELSE design END,
                engraving = CASE WHEN (engraving IS NULL OR engraving = '') AND (?13 != '') THEN ?13 ELSE engraving END,
                perforation = CASE WHEN (perforation IS NULL OR perforation = '') AND (?14 != '') THEN ?14 ELSE perforation END,
                context = CASE WHEN (context IS NULL OR context = '') AND (?15 != '') THEN ?15 ELSE context END
                WHERE country = ?16 AND series = ?17 AND sg_number = ?18";

            let params_vec = params![
                image_path,
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
                country,
                series,
                sg_number
            ];
            match conn.execute(update_sql, params_vec) {
                Ok(rows) => {
                    if rows > 0 {
                        debug!(
                            "Updated existing stamp metadata/image for {} | {} | {} (rows={})",
                            country, series, sg_number, rows
                        );
                        return Ok(true);
                    } else {
                        // trace removed
                    }
                }
                Err(e) => {
                    error!(
                        "DB update error for {} {} {}: {}",
                        country, series, sg_number, e
                    );
                }
            }
        }

        // trace removed
        return Ok(false);
    }
    // trace removed
    // trace removed
    // trace removed
    // trace removed
    // trace removed
    // trace removed
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
    let uniq_key = format!("{}|{}|{}", country.to_lowercase(), series, sg_number);
    let sql = "INSERT INTO stamps (country, series, year, month, month_num, day, sg_number, type, denomination, color, description, stamps_issued, mint_condition, unused, used, letter_fdc, currency, image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, context, uniq_key) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26)";
    let params_vec = vec![
        country.to_string(),
        series.to_string(),
        year.to_string(),
        month_clean.to_string(),
        month_num.to_string(),
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
        context.to_string(),
    ];
    // trace removed
    // trace removed
    let mut attempts = 0;
    loop {
        attempts += 1;
        match conn.execute(
            sql,
            params![
                country,
                series,
                year,
                month_clean,
                month_num,
                day,
                sg_number,
                type_,
                denomination,
                color,
                description,
                stamps_issued,
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
                uniq_key
            ],
        ) {
            Ok(_) => {
                debug!("Insert succeeded on attempt {}", attempts);
                let last_id = conn.last_insert_rowid();
                debug!("Last insert rowid: {}", last_id);
                break;
            }
            Err(e) => {
                error!("Insert failed on attempt {}: {}", attempts, e);
                if attempts >= 2 {
                    error!(
                        "Giving up after {} attempts. Params: {:?}",
                        attempts, params_vec
                    );
                    break;
                } else {
                    // retrying insert
                }
            }
        }
    }
    Ok(true)
}

pub type StampCountrySeriesRow = (
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
);

pub fn get_stamps_by_country_series(
    conn: &Connection,
    country: &str,
    series: &str,
) -> Result<Vec<StampCountrySeriesRow>> {
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
