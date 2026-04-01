use crate::progress::ProgressIndicator;

/// Parse Colnect CSV filename to extract country and year
/// Format: en_stamps_csv_list_country_13-Australia_year_1975.csv
fn parse_colnect_csv_filename(csv_path: &str) -> Option<(String, String)> {
    let filename = std::path::Path::new(csv_path).file_name()?.to_str()?;

    // Extract country: look for "country_XX-CountryName"
    let country = if let Some(country_start) = filename.find("country_") {
        let after_country = &filename[country_start + 8..]; // Skip "country_"
        if let Some(dash_pos) = after_country.find('-') {
            let after_dash = &after_country[dash_pos + 1..];
            if let Some(underscore_pos) = after_dash.find('_') {
                Some(after_dash[..underscore_pos].to_string())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    // Extract year: look for "year_YYYY"
    let year = if let Some(year_start) = filename.find("year_") {
        let after_year = &filename[year_start + 5..]; // Skip "year_"
        if let Some(dot_pos) = after_year.find('.') {
            Some(after_year[..dot_pos].to_string())
        } else {
            None
        }
    } else {
        None
    };

    match (country, year) {
        (Some(c), Some(y)) => Some((c, y)),
        _ => None,
    }
}

fn canonical_album_dir_name(name: &str) -> String {
    name.trim().replace(' ', "-")
}

fn has_downloaded_images(images_dir: &std::path::Path) -> bool {
    let Ok(entries) = std::fs::read_dir(images_dir) else {
        return false;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Some(file_name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if file_name.starts_with("no_image") {
            continue;
        }
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        if matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "webp") {
            return true;
        }
    }
    false
}

fn is_colnect_export_filename(path: &std::path::Path) -> bool {
    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
        return false;
    };
    name.starts_with("en_stamps_csv_list_countr") && name.ends_with(".csv")
}

fn collect_colnect_csv_files_rec(
    root: &std::path::Path,
    out: &mut Vec<std::path::PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    if !root.exists() {
        return Ok(());
    }
    for entry in std::fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_colnect_csv_files_rec(&path, out)?;
        } else if is_colnect_export_filename(&path) {
            out.push(path);
        }
    }
    Ok(())
}

fn country_slug(s: &str) -> String {
    canonical_album_dir_name(s).to_lowercase()
}

fn file_sha256(path: &std::path::Path) -> Result<String, Box<dyn std::error::Error>> {
    use sha2::{Digest, Sha256};
    use std::io::Read;

    let mut file = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 8192];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hasher.finalize().iter().map(|b| format!("{:02x}", b)).collect())
}

fn ensure_colnect_import_history_table(
    conn: &rusqlite::Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS colnect_import_history (
            id INTEGER PRIMARY KEY,
            country_key TEXT NOT NULL,
            file_path TEXT NOT NULL,
            file_name TEXT NOT NULL,
            sha256 TEXT NOT NULL,
            file_size INTEGER,
            modified_time TEXT,
            status TEXT NOT NULL,
            started_at TEXT,
            finished_at TEXT,
            rows_ingested INTEGER,
            error_message TEXT
        )",
        [],
    )?;
    conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_colnect_import_history_country_sha
         ON colnect_import_history (country_key, sha256)",
        [],
    )?;
    Ok(())
}

fn has_import_checksum(
    conn: &rusqlite::Connection,
    country_key: &str,
    sha256: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare(
        "SELECT 1 FROM colnect_import_history WHERE country_key = ?1 AND sha256 = ?2 AND status = 'success' LIMIT 1",
    )?;
    let mut rows = stmt.query([country_key, sha256])?;
    Ok(rows.next()?.is_some())
}

fn record_import_history(
    conn: &rusqlite::Connection,
    country_key: &str,
    file_path: &std::path::Path,
    sha256: &str,
    status: &str,
    rows_ingested: i64,
    error_message: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let meta = std::fs::metadata(file_path).ok();
    let file_size = meta.as_ref().map(|m| m.len() as i64);
    let modified_time = meta
        .and_then(|m| m.modified().ok())
        .map(|t| format!("{:?}", t));
    let file_name = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string();
    let file_path_str = file_path.to_string_lossy().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT OR REPLACE INTO colnect_import_history (
            id, country_key, file_path, file_name, sha256, file_size, modified_time,
            status, started_at, finished_at, rows_ingested, error_message
        ) VALUES (
            (SELECT id FROM colnect_import_history WHERE country_key = ?1 AND sha256 = ?2),
            ?1, ?3, ?4, ?2, ?5, ?6, ?7, ?8, ?8, ?9, ?10
        )",
        rusqlite::params![
            country_key,
            sha256,
            file_path_str,
            file_name,
            file_size,
            modified_time,
            status,
            now,
            rows_ingested,
            error_message
        ],
    )?;
    Ok(())
}

fn infer_country_from_stamps(stamps: &[crate::colnect::parser::ColnectStamp]) -> Option<String> {
    stamps.iter().find_map(|s| {
        let c = s.country.trim();
        if c.is_empty() {
            None
        } else {
            Some(c.to_string())
        }
    })
}

fn move_to_country_inbox(
    file_path: &std::path::Path,
    country: &str,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    // Keep country-specific Colnect source files with the country album DB.
    // Example: Albums/Netherlands/colnect/<csv-file>
    let country_dir = std::path::Path::new("Albums")
        .join(canonical_album_dir_name(country))
        .join("colnect");
    std::fs::create_dir_all(&country_dir)?;
    let file_name = file_path
        .file_name()
        .ok_or("Missing file name")?
        .to_os_string();
    let target = country_dir.join(file_name);
    if file_path == target {
        return Ok(target);
    }
    match std::fs::rename(file_path, &target) {
        Ok(_) => Ok(target),
        Err(_) => {
            std::fs::copy(file_path, &target)?;
            std::fs::remove_file(file_path)?;
            Ok(target)
        }
    }
}

fn run_colnect_inbox_mode(
    cli: &crate::cli::Cli,
    show_progress: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    use log::{error, info, warn};

    let progress = ProgressIndicator::new_with_config(show_progress);
    let inbox_root = std::path::PathBuf::from(
        cli.colnect_inbox
            .as_ref()
            .ok_or("Missing --colnect-inbox path")?,
    );
    if !inbox_root.exists() {
        eprintln!(
            "Colnect inbox '{}' does not exist. Creating it now (equivalent to answering yes).",
            inbox_root.display()
        );
        if let Err(e) = std::fs::create_dir_all(&inbox_root) {
            return Err(format!(
                "Failed to create Colnect inbox path '{}': {}",
                inbox_root.display(),
                e
            )
            .into());
        }
    } else if !inbox_root.is_dir() {
        return Err(format!(
            "Colnect inbox path is not a directory: {}",
            inbox_root.display()
        )
        .into());
    }

    progress.set_message(&format!(
        "🔎 Scanning Colnect inbox: {}",
        inbox_root.display()
    ));
    let mut files = Vec::new();
    collect_colnect_csv_files_rec(&inbox_root, &mut files)?;
    files.sort();

    let country_filter = cli.colnect_country.as_ref().map(|s| country_slug(s));
    if let Some(filter) = country_filter.as_ref() {
        files.retain(|p| {
            parse_colnect_csv_filename(&p.to_string_lossy())
                .map(|(country, _)| country_slug(&country) == *filter)
                .unwrap_or(false)
        });
    }

    if files.is_empty() {
        return Err("No matching Colnect CSV files found in inbox.".into());
    }

    let mut imported = 0usize;
    let mut skipped_duplicates = 0usize;
    let mut skipped_missing_db = 0usize;
    let mut failed = 0usize;
    let mut imported_countries = std::collections::BTreeSet::new();

    for mut csv_file in files {
        progress.set_message(&format!("📥 Processing {}", csv_file.display()));

        let mut country = parse_colnect_csv_filename(&csv_file.to_string_lossy()).map(|(c, _)| c);

        if cli.colnect_auto_route {
            if let Some(c) = country.clone() {
                csv_file = move_to_country_inbox(&csv_file, &c)?;
            }
        }

        let stamps = match crate::colnect::parser::parse_csv(&csv_file) {
            Ok(s) => s,
            Err(e) => {
                failed += 1;
                error!("Failed to parse {}: {}", csv_file.display(), e);
                continue;
            }
        };

        if country.is_none() {
            country = infer_country_from_stamps(&stamps);
            if cli.colnect_auto_route {
                if let Some(c) = country.clone() {
                    csv_file = move_to_country_inbox(&csv_file, &c)?;
                }
            }
        }

        let country = match country {
            Some(c) => c,
            None => {
                failed += 1;
                warn!(
                    "Could not infer country for file {}; skipping",
                    csv_file.display()
                );
                continue;
            }
        };

        let album_country = canonical_album_dir_name(&country);
        let db_path = format!("Albums/{0}/{0}.db", album_country);
        let db_exists = std::path::Path::new(&db_path).exists();
        if !db_exists && !cli.colnect_create_country_db {
            skipped_missing_db += 1;
            warn!(
                "Skipping {} because target DB does not exist and --colnect-create-country-db is false: {}",
                csv_file.display(),
                db_path
            );
            continue;
        }

        let conn = crate::db_utils::init_db(&db_path)?;
        ensure_colnect_import_history_table(&conn)?;

        let checksum = file_sha256(&csv_file)?;
        let country_key = country_slug(&country);
        if cli.colnect_skip_duplicates
            && !cli.colnect_force_reimport
            && has_import_checksum(&conn, &country_key, &checksum)?
        {
            skipped_duplicates += 1;
            info!(
                "Skipping duplicate file for country {}: {}",
                country,
                csv_file.display()
            );
            continue;
        }

        let default_catalog_code =
            crate::config::load_config("config.toml").effective_default_catalog_code();
        match crate::colnect::ingest::ingest_colnect_data_with_mode(
            &conn,
            &stamps,
            &default_catalog_code,
            false,
        ) {
            Ok(_) => {
                info!(
                    "Review-first mode: Colnect ingest kept in colnect_stamps only for {}",
                    country
                );
                imported += 1;
                imported_countries.insert(album_country.clone());
                let _ = record_import_history(
                    &conn,
                    &country_key,
                    &csv_file,
                    &checksum,
                    "success",
                    stamps.len() as i64,
                    None,
                );
                info!(
                    "Imported {} rows into {} from {}",
                    stamps.len(),
                    db_path,
                    csv_file.display()
                );
            }
            Err(e) => {
                failed += 1;
                let _ = record_import_history(
                    &conn,
                    &country_key,
                    &csv_file,
                    &checksum,
                    "failed",
                    0,
                    Some(&e.to_string()),
                );
                error!(
                    "Failed ingest for {} ({}): {}",
                    country,
                    csv_file.display(),
                    e
                );
            }
        }
    }

    let mut generated = 0usize;
    let mut generate_failed = 0usize;
    if cli.colnect_generate_after_import {
        for album_country in &imported_countries {
            let db_path = format!("Albums/{0}/{0}.db", album_country);
            progress.set_message(&format!(
                "📖 Generating album output for imported country: {}",
                album_country
            ));

            match crate::db_utils::init_db(&db_path) {
                Ok(conn) => {
                    let output_path = crate::album_globals::generate_album_output_conn(
                        &conn,
                        album_country,
                        None::<fn(usize)>,
                        Some(cli.start_year),
                        Some(cli.end_year),
                    );
                    generated += 1;
                    info!(
                        "Generated album for {}: {}",
                        album_country,
                        output_path.display()
                    );
                }
                Err(e) => {
                    generate_failed += 1;
                    error!(
                        "Failed to open DB for album generation {} ({}): {}",
                        album_country, db_path, e
                    );
                }
            }
        }
    }

    let summary = if cli.colnect_generate_after_import {
        format!(
            "Colnect inbox complete: imported={}, skipped_duplicates={}, skipped_missing_db={}, failed={}, generated_albums={}, generate_failed={}",
            imported, skipped_duplicates, skipped_missing_db, failed, generated, generate_failed
        )
    } else {
        format!(
            "Colnect inbox complete: imported={}, skipped_duplicates={}, skipped_missing_db={}, failed={}",
            imported, skipped_duplicates, skipped_missing_db, failed
        )
    };

    progress.finish_with_message(&summary);
    Ok(())
}

/// Merge Colnect data into stamps table by inserting only missing variants.
/// This runs even when StampWorld rows already exist so booklet/sheet variants
/// from Colnect can appear in output.
#[allow(dead_code)]
fn sync_missing_stamps_from_colnect(
    conn: &rusqlite::Connection,
) -> Result<usize, Box<dyn std::error::Error>> {
    use log::info;

    fn extract_year(issued_on: &str) -> Option<i32> {
        issued_on.get(0..4)?.parse::<i32>().ok()
    }

    fn numeric_tokens(v: &str) -> Vec<String> {
        let re = regex::Regex::new(r"[0-9]+(?:[\.,][0-9]+)?")
            .expect("numeric token regex should compile");
        re.find_iter(v)
            .map(|m| m.as_str().replace(',', "."))
            .collect()
    }

    fn score_candidate(
        col_tokens: &[String],
        cand_tokens: &[String],
        col_series: &str,
        cand_series: &str,
        cand_image_path: &str,
    ) -> i32 {
        if col_tokens.is_empty() || cand_tokens.is_empty() {
            return 0;
        }

        let mut score = 0;
        if col_tokens == cand_tokens {
            score += 100;
        } else {
            let overlap = col_tokens
                .iter()
                .filter(|t| cand_tokens.iter().any(|c| c == *t))
                .count() as i32;
            score += overlap * 20;
        }

        let col_series_l = col_series.to_lowercase();
        let cand_series_l = cand_series.to_lowercase();
        if !col_series_l.is_empty() && !cand_series_l.is_empty() {
            if cand_series_l.contains(&col_series_l) || col_series_l.contains(&cand_series_l) {
                score += 10;
            }
        }

        if !cand_image_path.trim().is_empty() {
            score += 5;
        }

        score
    }

    fn remap_colnect_sg_to_existing_stamps(
        conn: &rusqlite::Connection,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        use log::{debug, info};

        let mut col_stmt = conn.prepare(
            "SELECT rowid, COALESCE(country, ''), COALESCE(series, ''), COALESCE(issued_on, ''),
                    COALESCE(face_value, ''), COALESCE(sg_number, '')
             FROM colnect_stamps",
        )?;

        let col_rows = col_stmt.query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
            ))
        })?;

        let mut update_stmt =
            conn.prepare("UPDATE colnect_stamps SET sg_number = ?1 WHERE rowid = ?2")?;
        let mut remapped = 0usize;

        for row in col_rows {
            let (rowid, country, col_series, issued_on, face_value, current_sg) = row?;
            let Some(year) = extract_year(&issued_on) else {
                continue;
            };
            let col_tokens = numeric_tokens(&face_value);
            if col_tokens.is_empty() {
                continue;
            }

            let mut cand_stmt = if country.trim().is_empty() {
                conn.prepare(
                    "SELECT COALESCE(sg_number, ''), COALESCE(denomination, ''),
                            COALESCE(series, ''), COALESCE(image_path, '')
                     FROM stamps
                     WHERE year = ?1 AND TRIM(COALESCE(sg_number, '')) != ''",
                )?
            } else {
                conn.prepare(
                    "SELECT COALESCE(sg_number, ''), COALESCE(denomination, ''),
                            COALESCE(series, ''), COALESCE(image_path, '')
                     FROM stamps
                     WHERE year = ?1
                       AND LOWER(TRIM(COALESCE(country, ''))) = LOWER(TRIM(?2))
                       AND TRIM(COALESCE(sg_number, '')) != ''",
                )?
            };

            let cands: Vec<(String, String, String, String)> = if country.trim().is_empty() {
                cand_stmt
                    .query_map(rusqlite::params![year], |r| {
                        Ok((
                            r.get::<_, String>(0)?,
                            r.get::<_, String>(1)?,
                            r.get::<_, String>(2)?,
                            r.get::<_, String>(3)?,
                        ))
                    })?
                    .collect::<Result<Vec<_>, _>>()?
            } else {
                cand_stmt
                    .query_map(rusqlite::params![year, country], |r| {
                        Ok((
                            r.get::<_, String>(0)?,
                            r.get::<_, String>(1)?,
                            r.get::<_, String>(2)?,
                            r.get::<_, String>(3)?,
                        ))
                    })?
                    .collect::<Result<Vec<_>, _>>()?
            };

            let mut best_score = i32::MIN;
            let mut best_sg = String::new();
            let mut tie_count = 0usize;

            for (cand_sg, cand_denom, cand_series, cand_image_path) in cands {
                let cand_tokens = numeric_tokens(&cand_denom);
                let score = score_candidate(
                    &col_tokens,
                    &cand_tokens,
                    &col_series,
                    &cand_series,
                    &cand_image_path,
                );

                if score > best_score {
                    best_score = score;
                    best_sg = cand_sg;
                    tie_count = 1;
                } else if score == best_score {
                    tie_count += 1;
                }
            }

            // Require a strong-enough match and no ambiguity.
            if best_score >= 100 && tie_count == 1 && !best_sg.is_empty() && best_sg != current_sg {
                update_stmt.execute(rusqlite::params![best_sg, rowid])?;
                remapped += 1;
            } else if best_score > 0 {
                debug!(
                    "Skipped ambiguous/weak Colnect remap rowid={} sg='{}' score={} ties={}",
                    rowid, current_sg, best_score, tie_count
                );
            }
        }

        info!(
            "Remapped {} Colnect rows onto existing stamps using year+denomination matching",
            remapped
        );
        Ok(remapped)
    }

    fn backfill_colnect_links_on_existing_stamps(
        conn: &rusqlite::Connection,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        // Populate remote/original URLs for existing stamp rows so downstream
        // fallback image fetch can resolve Colnect page images even when rows
        // already existed before current Colnect ingest. Also backfill missing
        // Colnect metadata columns without overriding existing non-empty values.
        let updated = conn.execute(
            "UPDATE stamps
             SET remote_image_url = CASE
                    WHEN TRIM(COALESCE(remote_image_url, '')) = '' THEN (
                        SELECT COALESCE(c.colnect_link, '')
                        FROM colnect_stamps c
                        WHERE c.sg_number = stamps.sg_number
                          AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                        LIMIT 1
                    )
                    ELSE remote_image_url
                 END,
                 original_image_url = CASE
                    WHEN TRIM(COALESCE(original_image_url, '')) = '' THEN (
                        SELECT COALESCE(c.colnect_link, '')
                        FROM colnect_stamps c
                        WHERE c.sg_number = stamps.sg_number
                          AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                        LIMIT 1
                    )
                    ELSE original_image_url
                                 END,
                                 colnect_page_url = CASE
                                        WHEN TRIM(COALESCE(colnect_page_url, '')) = '' THEN (
                                                SELECT COALESCE(c.colnect_link, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_page_url
                                 END,
                 colnect_image_url = CASE
                                        WHEN TRIM(COALESCE(colnect_image_url, '')) = '' THEN (
                                                SELECT COALESCE(c.colnect_link, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_image_url
                                 END,
                 colnect_id = CASE
                                        WHEN TRIM(COALESCE(colnect_id, '')) = '' THEN (
                                                SELECT COALESCE(c.colnect_id, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_id
                                 END,
                 colnect_name = CASE
                                        WHEN TRIM(COALESCE(colnect_name, '')) = '' THEN (
                                                SELECT COALESCE(c.name, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_name
                                 END,
                 colnect_catalog_codes = CASE
                                        WHEN TRIM(COALESCE(colnect_catalog_codes, '')) = '' THEN (
                                                SELECT COALESCE(c.catalog_codes, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_catalog_codes
                                 END,
                 colnect_country = CASE
                                        WHEN TRIM(COALESCE(colnect_country, '')) = '' THEN (
                                                SELECT COALESCE(c.country, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_country
                                 END,
                 colnect_width = CASE
                                        WHEN TRIM(COALESCE(colnect_width, '')) = '' THEN (
                                                SELECT COALESCE(c.width, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_width
                                 END,
                 colnect_height = CASE
                                        WHEN TRIM(COALESCE(colnect_height, '')) = '' THEN (
                                                SELECT COALESCE(c.height, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_height
                                 END,
                 colnect_format = CASE
                                        WHEN TRIM(COALESCE(colnect_format, '')) = '' THEN (
                                                SELECT COALESCE(c.format, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_format
                                 END,
                 colnect_emission = CASE
                                        WHEN TRIM(COALESCE(colnect_emission, '')) = '' THEN (
                                                SELECT COALESCE(c.emission, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_emission
                                 END,
                 colnect_printing = CASE
                                        WHEN TRIM(COALESCE(colnect_printing, '')) = '' THEN (
                                                SELECT COALESCE(c.printing, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_printing
                                 END,
                 colnect_paper = CASE
                                        WHEN TRIM(COALESCE(colnect_paper, '')) = '' THEN (
                                                SELECT COALESCE(c.paper, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_paper
                                 END,
                 colnect_gum = CASE
                                        WHEN TRIM(COALESCE(colnect_gum, '')) = '' THEN (
                                                SELECT COALESCE(c.gum, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_gum
                                 END,
                 colnect_ord_mi = CASE
                                        WHEN TRIM(COALESCE(colnect_ord_mi, '')) = '' THEN (
                                                SELECT COALESCE(c.ord_mi, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_ord_mi
                                 END,
                 colnect_ord_sn = CASE
                                        WHEN TRIM(COALESCE(colnect_ord_sn, '')) = '' THEN (
                                                SELECT COALESCE(c.ord_sn, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_ord_sn
                                 END,
                 colnect_ord_yt = CASE
                                        WHEN TRIM(COALESCE(colnect_ord_yt, '')) = '' THEN (
                                                SELECT COALESCE(c.ord_yt, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_ord_yt
                                 END,
                 colnect_ord_sg = CASE
                                        WHEN TRIM(COALESCE(colnect_ord_sg, '')) = '' THEN (
                                                SELECT COALESCE(c.ord_sg, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_ord_sg
                                 END,
                 colnect_expiry_date = CASE
                                        WHEN TRIM(COALESCE(colnect_expiry_date, '')) = '' THEN (
                                                SELECT COALESCE(c.expiry_date, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_expiry_date
                                 END,
                 colnect_print_run = CASE
                                        WHEN TRIM(COALESCE(colnect_print_run, '')) = '' THEN (
                                                SELECT COALESCE(c.print_run, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_print_run
                                 END,
                 colnect_face_value = CASE
                                        WHEN TRIM(COALESCE(colnect_face_value, '')) = '' THEN (
                                                SELECT COALESCE(c.face_value, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_face_value
                                 END,
                 colnect_currency = CASE
                                        WHEN TRIM(COALESCE(colnect_currency, '')) = '' THEN (
                                                SELECT COALESCE(c.currency, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_currency
                                 END,
                 colnect_colors = CASE
                                        WHEN TRIM(COALESCE(colnect_colors, '')) = '' THEN (
                                                SELECT COALESCE(c.colors, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_colors
                                 END,
                 colnect_parent_item = CASE
                                        WHEN TRIM(COALESCE(colnect_parent_item, '')) = '' THEN (
                                                SELECT COALESCE(c.parent_item, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_parent_item
                                 END,
                 colnect_variant = CASE
                                        WHEN TRIM(COALESCE(colnect_variant, '')) = '' THEN (
                                                SELECT COALESCE(c.variant, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_variant
                                 END,
                 colnect_overprint_variant = CASE
                                        WHEN TRIM(COALESCE(colnect_overprint_variant, '')) = '' THEN (
                                                SELECT COALESCE(c.overprint_variant, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_overprint_variant
                                 END,
                 colnect_score = CASE
                                        WHEN TRIM(COALESCE(colnect_score, '')) = '' THEN (
                                                SELECT COALESCE(c.score, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_score
                                 END,
                 colnect_accuracy = CASE
                                        WHEN TRIM(COALESCE(colnect_accuracy, '')) = '' THEN (
                                                SELECT COALESCE(c.accuracy, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_accuracy
                                 END,
                 colnect_list = CASE
                                        WHEN TRIM(COALESCE(colnect_list, '')) = '' THEN (
                                                SELECT COALESCE(c.list, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_list
                                 END,
                 colnect_quantity = CASE
                                        WHEN TRIM(COALESCE(colnect_quantity, '')) = '' THEN (
                                                SELECT COALESCE(c.quantity, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_quantity
                                 END,
                 colnect_condition = CASE
                                        WHEN TRIM(COALESCE(colnect_condition, '')) = '' THEN (
                                                SELECT COALESCE(c.condition, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_condition
                                 END,
                 colnect_public_note = CASE
                                        WHEN TRIM(COALESCE(colnect_public_note, '')) = '' THEN (
                                                SELECT COALESCE(c.public_note, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_public_note
                                 END,
                 colnect_private_note = CASE
                                        WHEN TRIM(COALESCE(colnect_private_note, '')) = '' THEN (
                                                SELECT COALESCE(c.private_note, '')
                                                FROM colnect_stamps c
                                                WHERE c.sg_number = stamps.sg_number
                                                    AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
                                                LIMIT 1
                                        )
                                        ELSE colnect_private_note
                                 END
             WHERE EXISTS (
                 SELECT 1
                 FROM colnect_stamps c
                 WHERE c.sg_number = stamps.sg_number
                   AND CAST(substr(c.issued_on, 1, 4) AS INTEGER) = stamps.year
             )",
            [],
        )?;
        Ok(updated)
    }

    // First align Colnect sg_number values with existing StampWorld stamps when
    // a strong year+denomination match exists. This keeps image-backed base
    // rows and lets Colnect enrichment attach to those rows.
    let remapped = remap_colnect_sg_to_existing_stamps(conn)?;
    let links_backfilled = backfill_colnect_links_on_existing_stamps(conn)?;

    let default_catalog_code =
        crate::config::load_config("config.toml").effective_default_catalog_code();

    // Insert missing Colnect rows into stamps using year+catalog key dedupe.
    // Keep existing StampWorld rows and only add missing variants.
    let inserted = conn.execute(
        "INSERT INTO stamps (
            country, series, year, month, month_num, day, sg_number, catalog_code, type,
            denomination, color, description,
            stamps_issued, mint_condition, unused, used, letter_fdc,
            currency, image_path, remote_image_url, original_image_url,
            stampworld_image_url, colnect_page_url, colnect_image_url,
            colnect_id, colnect_name, colnect_catalog_codes, colnect_country,
            colnect_width, colnect_height, colnect_format, colnect_emission,
            colnect_printing, colnect_paper, colnect_gum, colnect_ord_mi,
            colnect_ord_sn, colnect_ord_yt, colnect_ord_sg, colnect_expiry_date,
            colnect_print_run, colnect_face_value, colnect_currency, colnect_colors,
            colnect_parent_item, colnect_variant, colnect_overprint_variant,
            colnect_score, colnect_accuracy, colnect_list, colnect_quantity,
            colnect_condition, colnect_public_note, colnect_private_note,
            watermark, design, engraving, perforation, context,
            issue_location, quantity_printed
        )
        SELECT 
            COALESCE(NULLIF(country, ''), '') as country,
            COALESCE(NULLIF(series, ''), name) as series,
            CAST(substr(issued_on, 1, 4) AS INTEGER) as year,
            CASE CAST(substr(issued_on, 6, 2) AS INTEGER)
                WHEN 1 THEN 'January' WHEN 2 THEN 'February' WHEN 3 THEN 'March'
                WHEN 4 THEN 'April' WHEN 5 THEN 'May' WHEN 6 THEN 'June'
                WHEN 7 THEN 'July' WHEN 8 THEN 'August' WHEN 9 THEN 'September'
                WHEN 10 THEN 'October' WHEN 11 THEN 'November' WHEN 12 THEN 'December'
                ELSE '' END as month,
            CAST(substr(issued_on, 6, 2) AS INTEGER) as month_num,
            CASE
                WHEN length(issued_on) >= 10 THEN substr(issued_on, 9, 2)
                ELSE ''
            END as day,
            sg_number,
            ?1 as catalog_code,
            COALESCE(NULLIF(format, ''), 'Stamp') as type,
            face_value as denomination,
            colors as color,
            COALESCE(NULLIF(description, ''), name) as description,
            print_run as stamps_issued,
            '' as mint_condition,
            '' as unused,
            '' as used,
            '' as letter_fdc,
            currency,
            '' as image_path,
            colnect_link as remote_image_url,
            colnect_link as original_image_url,
            '' as stampworld_image_url,
            colnect_link as colnect_page_url,
            colnect_link as colnect_image_url,
            COALESCE(colnect_id, '') as colnect_id,
            COALESCE(name, '') as colnect_name,
            COALESCE(catalog_codes, '') as colnect_catalog_codes,
            COALESCE(country, '') as colnect_country,
            COALESCE(width, '') as colnect_width,
            COALESCE(height, '') as colnect_height,
            COALESCE(format, '') as colnect_format,
            COALESCE(emission, '') as colnect_emission,
            COALESCE(printing, '') as colnect_printing,
            COALESCE(paper, '') as colnect_paper,
            COALESCE(gum, '') as colnect_gum,
            COALESCE(ord_mi, '') as colnect_ord_mi,
            COALESCE(ord_sn, '') as colnect_ord_sn,
            COALESCE(ord_yt, '') as colnect_ord_yt,
            COALESCE(ord_sg, '') as colnect_ord_sg,
            COALESCE(expiry_date, '') as colnect_expiry_date,
            COALESCE(print_run, '') as colnect_print_run,
            COALESCE(face_value, '') as colnect_face_value,
            COALESCE(currency, '') as colnect_currency,
            COALESCE(colors, '') as colnect_colors,
            COALESCE(parent_item, '') as colnect_parent_item,
            COALESCE(variant, '') as colnect_variant,
            COALESCE(overprint_variant, '') as colnect_overprint_variant,
            COALESCE(score, '') as colnect_score,
            COALESCE(accuracy, '') as colnect_accuracy,
            COALESCE(list, '') as colnect_list,
            COALESCE(quantity, '') as colnect_quantity,
            COALESCE(condition, '') as colnect_condition,
            COALESCE(public_note, '') as colnect_public_note,
            COALESCE(private_note, '') as colnect_private_note,
            COALESCE(watermark, '') as watermark,
            designers as design,
            printers as engraving,
            perforation as perforation,
            themes as context,
            '' as issue_location,
            print_run as quantity_printed
        FROM colnect_stamps
        WHERE sg_number IS NOT NULL
          AND TRIM(sg_number) != ''
          AND CAST(substr(issued_on, 1, 4) AS INTEGER) > 0
          AND NOT EXISTS (
              SELECT 1
              FROM stamps s
              WHERE s.sg_number = colnect_stamps.sg_number
                AND s.year = CAST(substr(colnect_stamps.issued_on, 1, 4) AS INTEGER)
                    )",
        rusqlite::params![default_catalog_code],
    )?;

    info!(
        "Inserted {} missing stamp variants from colnect_stamps (after {} remaps, {} link backfills)",
        inserted, remapped, links_backfilled
    );
    Ok(inserted)
}

/// Run the database-only mode (generate album from existing database)
pub fn run_database_mode(
    cli: &crate::cli::Cli,
    show_progress: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    use log::{error, info, warn};

    if cli.source == "colnectexport" && cli.colnect_inbox.is_some() {
        return run_colnect_inbox_mode(cli, show_progress);
    }

    // Output-only mode: use the provided db_path and album_name
    let progress = ProgressIndicator::new_with_config(show_progress);
    progress.set_message("🔍 Initializing database mode...");

    // Infer db_path from CSV filename if not provided
    let inferred_db_path;
    let db_path = if let Some(path) = cli.db_path.as_ref() {
        path
    } else if let Some(csv_path) = cli.colnect_csv.as_ref() {
        // Parse CSV filename to extract country and year
        // Format: en_stamps_csv_list_country_13-Australia_year_1975.csv
        progress.set_message("🔍 Inferring database path from CSV filename...");
        let inferred_country = cli
            .site_name
            .clone()
            .or_else(|| parse_colnect_csv_filename(csv_path).map(|(country, _year)| country))
            .or_else(|| cli.album_name.clone());

        if let Some(country) = inferred_country {
            let canonical = canonical_album_dir_name(&country);
            inferred_db_path = format!("Albums/{0}/{0}.db", canonical);
            info!("Inferred database path: {}", inferred_db_path);
            &inferred_db_path
        } else {
            progress.finish_with_error("Could not infer database path from CSV filename");
            return Err("No --db-path provided and could not infer from CSV filename. Please provide --db-path or use a standard Colnect CSV filename format.".into());
        }
    } else {
        progress.finish_with_error("No database path provided");
        return Err("No --db-path provided. Please specify --db-path for database mode.".into());
    };

    progress.set_message(&format!("📁 Checking database path: {}", db_path));

    // Ensure parent directory exists and attempt to create the DB file if missing
    if let Some(parent) = std::path::Path::new(&db_path).parent() {
        if !parent.exists() {
            progress.set_message(&format!(
                "📁 Creating parent directory: {}",
                parent.display()
            ));
            if let Err(e) = std::fs::create_dir_all(parent) {
                progress.finish_with_error(&format!("Failed to create parent directory: {}", e));
                eprintln!(
                    "Failed to create parent directory for DB '{}': {}",
                    parent.display(),
                    e
                );
                return Err(e.into());
            }
        }
    }
    if !std::path::Path::new(&db_path).exists() {
        progress.set_message("📝 Creating new database file...");
        match std::fs::File::create(&db_path) {
            Ok(_) => (),
            Err(e) => {
                progress.finish_with_error(&format!("Failed to create DB file: {}", e));
                eprintln!("Failed to create DB file '{}': {}", db_path, e);
                return Err(e.into());
            }
        }
    }

    // Try opening the DB now to detect corrupt/non-sqlite files early and
    // provide a friendly message with suggestions rather than panicking later.
    // Open the database once and hand a reference to the generator.
    progress.set_message("🔗 Opening database connection...");
    let conn = match crate::db_utils::init_db(&db_path) {
        Ok(c) => c,
        Err(e) => {
            progress.finish_with_error(&format!("Database connection failed: {}", e));
            eprintln!("Failed to open database '{}': {}", db_path, e);
            eprintln!("Possible causes:");
            eprintln!("  - The file is not a SQLite database or is corrupted");
            eprintln!("  - File permissions prevent opening the file");
            eprintln!("  - Path points to a directory rather than a file");
            eprintln!("Possible fixes:");
            eprintln!(
                "  - Check the file: 'file {}' or try 'sqlite3 {} .dump' to inspect",
                db_path, db_path
            );
            eprintln!(
                "  - Remove or move the corrupt file so the program can create a new DB: rm {}",
                db_path
            );
            eprintln!("  - Ensure the path is correct and you have read/write permissions");
            return Err(e.into());
        }
    };
    // Determine album output/storage base from DB path so generated TXT and
    // image folder remain aligned across StampWorld and Colnect runs.
    let db_stem = std::path::Path::new(db_path)
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "Album".to_string());
    if let Some(name) = &cli.album_name {
        if canonical_album_dir_name(name).to_lowercase() != db_stem.to_lowercase() {
            warn!(
                "album-name '{}' differs from DB base '{}'; using '{}' so output and images stay in one folder",
                name,
                db_stem,
                db_stem
            );
            eprintln!(
                "Warning: --album-name '{}' differs from DB base '{}'. Using '{}' to keep output and images together.",
                name, db_stem, db_stem
            );
        }
    }
    let album_name = db_stem;

    // If Colnect is being ingested before StampWorld images exist, continue
    // but warn clearly so users understand why album pages may have placeholders.
    if cli.colnect_csv.is_some() {
        let images_dir = std::path::Path::new(db_path)
            .parent()
            .map(|p| p.join("images"))
            .unwrap_or_else(|| std::path::PathBuf::from("images"));
        if !has_downloaded_images(&images_dir) {
            let msg = format!(
                "⚠️ Colnect import is running before StampWorld images were found in '{}'. Recommended flow: run StampWorld first, then Colnect to enrich metadata.",
                images_dir.display()
            );
            warn!("{}", msg);
            progress.set_message(&msg);
            eprintln!("{}", msg);
        }
    }

    // Ingest Colnect CSV if provided (for colnectexport source)
    if let Some(csv_path) = &cli.colnect_csv {
        progress.set_message(&format!("📥 Ingesting Colnect CSV: {}", csv_path));
        match crate::colnect::parser::parse_csv(csv_path) {
            Ok(stamps) => {
                let default_catalog_code =
                    crate::config::load_config("config.toml").effective_default_catalog_code();
                match crate::colnect::ingest::ingest_colnect_data_with_default_catalog(
                    &conn,
                    &stamps,
                    &default_catalog_code,
                ) {
                    Ok(_) => info!("✅ Successfully ingested Colnect data"),
                    Err(e) => error!("❌ Failed to ingest Colnect data: {}", e),
                }
            }
            Err(e) => error!("❌ Failed to parse Colnect CSV: {}", e),
        }

        progress.set_message(
            "🧾 Colnect import complete (review-first mode: no automatic writes into stamps)",
        );
        info!(
            "Review-first mode enabled: skipped Colnect variant sync and fallback image updates to stamps"
        );
    }

    progress.set_message(&format!("📖 Generating album output for: {}", album_name));

    // Temporary unconditional marker to help automated runs detect that
    // we've reached the album-generation step and to print the env flags
    // relevant to wiki metric emission. This is intentionally a stdout
    // println! so it will appear in captured logs even if the logger
    // is configured to write to files.
    println!(
        "FORCE_WIKI_PAGE_METRICS_RUN album={} WIKI_PAGE_DEBUG={} FORCE_WIKI_PAGE_METRICS={}",
        album_name,
        std::env::var("WIKI_PAGE_DEBUG").unwrap_or_else(|_| "".to_string()),
        std::env::var("FORCE_WIKI_PAGE_METRICS").unwrap_or_else(|_| "".to_string())
    );

    let output_path = crate::album_globals::generate_album_output_conn(
        &conn,
        &album_name,
        None::<fn(usize)>,
        Some(cli.start_year),
        Some(cli.end_year),
    );

    progress.finish_with_message(&format!(
        "Album '{}' generated successfully! 📄 File: {}",
        album_name,
        output_path.display()
    ));

    Ok(())
}

/// Run the scraping mode (scrape StampWorld and generate album)
pub fn run_scraping_mode(
    cli: &crate::cli::Cli,
    show_progress: bool,
    source: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::commands_file_generator::CommandsFileGenerator;
    use log::{error, info, warn};

    let progress = ProgressIndicator::new_with_config(show_progress);
    progress.set_message("🌐 Preparing StampWorld scraping mode...");

    // Require site_name for scraping. If the caller explicitly passed
    // --site-name but set it to the literal value 'stampworld' (the
    // default/source name), prefer the provided --album-name when
    // available. This prevents constructing URLs like
    // '/en/stamps/stampworld/...' when the intended album slug is the
    // album name (e.g. 'New-Brunswick'). If neither is provided, exit.
    let site_name = if let Some(s) = cli.site_name.clone() {
        if s.to_lowercase() == "stampworld" {
            if let Some(a) = cli.album_name.clone() {
                a.clone()
            } else {
                s
            }
        } else {
            s
        }
    } else if let Some(a) = cli.album_name.clone() {
        a.clone()
    } else {
        progress.finish_with_error("Missing site name for scraping");
        eprintln!("You started the program in scraping mode but did not provide the site name.");
        eprintln!(
            "Please provide the site using --site-name (for example: --site-name 'Great-Britain') or supply --album-name so the program can infer the site."
        );
        eprintln!("If you're not sure what to put, here's the full help text:\n");
        // This would need to be handled by the caller
        return Err("Missing site name for scraping".into());
    };
    let requested_album_name = cli.album_name.clone().unwrap_or_else(|| site_name.clone());

    let album_name = if source == "us1847" {
        "US-".to_string() + &requested_album_name
    } else {
        requested_album_name
    };

    // Always keep scrape DB/images rooted by country/site so subsequent Colnect
    // enrichment resolves to the same folder.
    let storage_base_name = if source == "us1847" {
        album_name.clone()
    } else {
        site_name.clone()
    };
    let canonical_storage_dir = canonical_album_dir_name(&storage_base_name);

    if source != "us1847"
        && canonical_album_dir_name(&album_name).to_lowercase()
            != canonical_storage_dir.to_lowercase()
    {
        warn!(
            "album-name '{}' differs from site-name '{}'; storage/output will use '{}' so StampWorld and Colnect share one folder",
            album_name,
            site_name,
            canonical_storage_dir
        );
        eprintln!(
            "Warning: --album-name '{}' differs from --site-name '{}'. Using '{}' as shared storage base.",
            album_name, site_name, canonical_storage_dir
        );
    }

    progress.set_message(&format!(
        "📋 Setting up scrape: {} → {} ({}..{})",
        site_name, album_name, cli.start_year, cli.end_year
    ));
    progress.set_message(
        &format!(
            "Starting StampWorld scrape for site='{}' album='{}' years={}..{} overwrite_images={} overwrite_album={}",
            site_name,
            album_name,
            cli.start_year,
            cli.end_year,
            cli.overwrite_images,
            cli.overwrite_album
        )
    );

    // Build a CommandsFileGenerator and invoke the collector
    progress.set_message("⚙️ Configuring data collector...");
    let mut cfg = CommandsFileGenerator::default();
    cfg.album_name = album_name.clone();
    cfg.country = site_name.clone();
    // Keep scrape outputs in a canonical country/site-based folder.
    cfg.album_path = format!("Albums/{}", canonical_storage_dir);
    cfg.image_path = format!("Albums/{}/images", canonical_storage_dir);
    cfg.year = cli.start_year.to_string();

    // Build base URL
    progress.set_message(&format!(
        "🔗 Building {} URL...",
        if source == "us1847" {
            "1847US"
        } else {
            "StampWorld"
        }
    ));
    let base_url = if let Some(b) = &cli.base_url {
        b.clone()
    } else if source == "us1847" {
        // For 1847us.com, require explicit base_url since each series has its own URL
        progress.finish_with_error("Missing base URL for 1847US scraping");
        eprintln!("For 1847US scraping, you must provide the specific series URL using --base-url");
        eprintln!("Examples:");
        eprintln!("  --base-url 'https://1847us.com/byyear/1955-year/'");
        eprintln!("  --base-url 'https://1847us.com/1975identifier/'");
        return Err("Missing base URL for 1847US scraping".into());
    } else {
        // StampWorld URL construction logic...
        // Allow users to provide explicit overrides via `slug_overrides.toml`.
        // This file (looked up like `config.toml`) may contain a table
        // `[slug_overrides]` mapping the provided site_name to a canonical
        // StampWorld slug (which may contain '!' or other punctuation).
        let overrides = crate::config::load_slug_overrides("slug_overrides.toml");
        // Helper to percent-encode a slug for a path component
        // Prefer hyphen-separated slugs for multi-word site/album names
        let encode = |s: &str| s.replace(' ', "-").replace('!', "%21").replace('/', "%2F");

        // If the caller requested the full open range (0..9999) treat this
        // as an intent to fetch the album/main slug page rather than the
        // 'Postage stamps' year-range listing. In that case construct URLs
        // like: https://www.stampworld.com/en/stamps/<slug>/?page=
        let use_main_slug = cli.start_year == 0 && cli.end_year == 9999;

        if let Some(canon) = overrides.get(&site_name) {
            // If there's an explicit override, use it verbatim (but still
            // percent-encode reserved characters for the URL path).
            let encoded = encode(canon);
            // Avoid printing slug override details in normal runs.
            if use_main_slug {
                format!("https://www.stampworld.com/en/stamps/{}/?page=", encoded)
            } else {
                format!(
                    "https://www.stampworld.com/en/stamps/{}/Postage%20stamps/{}-{}?page=",
                    encoded, cli.start_year, cli.end_year
                )
            }
        } else {
            // Minimal percent-encoding for common problematic characters
            // that appear in StampWorld site names (space, '!' and '/').
            let encoded_site = encode(&site_name);
            if use_main_slug {
                format!(
                    "https://www.stampworld.com/en/stamps/{}/?page=",
                    encoded_site
                )
            } else {
                format!(
                    "https://www.stampworld.com/en/stamps/{}/Postage%20stamps/{}-{}?page=",
                    encoded_site, cli.start_year, cli.end_year
                )
            }
        }
    };
    // Load full config (so flag/wiki toggles are available)
    progress.set_message("⚙️ Loading configuration file...");
    let cfg_file = crate::config::load_config("config.toml");

    // Keep Colnect inbox locations ready during StampWorld runs so follow-up
    // Colnect imports can drop files into known paths without manual setup.
    let default_colnect_inbox = format!("Albums/{}/colnect", canonical_storage_dir);
    if let Some(configured_inbox) = cfg_file.effective_colnect_inbox_path() {
        let configured_path = std::path::Path::new(configured_inbox);
        if !configured_path.exists() {
            std::fs::create_dir_all(configured_path)?;
            info!(
                "Created configured Colnect inbox directory: {}",
                configured_path.display()
            );
        }
    }
    let default_path = std::path::Path::new(&default_colnect_inbox);
    let configured_matches_default = cfg_file
        .effective_colnect_inbox_path()
        .map(|p| {
            std::path::Path::new(p).to_string_lossy().to_lowercase()
                == default_path.to_string_lossy().to_lowercase()
        })
        .unwrap_or(false);
    if !configured_matches_default && !default_path.exists() {
        std::fs::create_dir_all(default_path)?;
        info!(
            "Created default Colnect inbox directory: {}",
            default_path.display()
        );
    }

    // Compose the DB path early and set it for the collector so both scraper
    // and generator operate on the same DB file under Albums/{Album}/{Album}.db.
    progress.set_message("💾 Setting up database path...");
    let db_path = format!(
        "{}/{}.db",
        cfg.album_path,
        canonical_storage_dir.replace('/', "-")
    );
    crate::file_utils::set_db_path_from_cli(&db_path);
    // Propagate the CLI overwrite flag into global state so generators
    // can create backups when the user explicitly requested overwrites.
    crate::file_utils::set_overwrite_album_from_cli(cli.overwrite_album);
    progress.set_message(&format!("Stamp Database path set to {}", db_path));

    progress.set_message(&format!("🌐 Starting data collection from: {}", base_url));
    let pages = if source == "us1847" {
        // For 1847us.com, we process a single URL directly
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .expect("Failed to build reqwest client");

        // Initialize DB
        let conn = match crate::db_utils::init_db(&db_path) {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to initialize DB {}: {}", db_path, e);
                return Err(e.into());
            }
        };

        match crate::us1847::collector::process_1847us_page(
            &client,
            &conn,
            &base_url,
            &site_name,
            cli.start_year,
            cli.end_year,
            &cfg.image_path,
            cli.overwrite_images,
        ) {
            Ok(stamp_count) => {
                progress.set_message(&format!(
                    "✅ 1847US scraping complete! Found {} stamps.",
                    stamp_count
                ));
                Ok(vec![format!(
                    "Processed {} stamps from {}",
                    stamp_count, base_url
                )])
            }
            Err(e) => {
                error!("Failed to process 1847us page {}: {}", base_url, e);
                Err(e)
            }
        }
    } else {
        // Call StampWorld collector
        cfg.collect_stampworld_data(
            &base_url,
            cli.overwrite_images,
            None::<&dyn Fn(usize)>,
            cli.max_pages,
            cli.stop_after_no_hits,
            cli.start_year,
            cli.end_year,
        )
    };
    match pages {
        Ok(pages) => {
            progress.set_message(&format!(
                "✅ Scraping complete! Found {} new stamp entries from web page(s).",
                pages.len()
            ));

            // If no pages/stamps were found, optionally try to ingest cached
            // Wikipedia/Grokipedia content into the album DB so the first
            // generation can produce something usable. This implements the
            // "Wikipedia-as-fallback" behavior: only run when the collector
            // returned zero results.
            if pages.len() == 0 {
                let fetch_wiki = cfg_file
                    .fetch_wikipedia
                    .unwrap_or_else(|| cfg_file.include_wikipedia_content.unwrap_or(false));
                if fetch_wiki {
                    progress.set_message("ℹ️ No stamps found in scraping; attempting Wikipedia->DB ingest fallback...");
                    let cache_db_path = cfg_file
                        .wikipedia_cache_db_path
                        .as_deref()
                        .unwrap_or("wikipedia_cache.db");
                    // Call ingest with a conservative max entries to avoid huge imports
                    match crate::wikipedia::ingest_wikipedia_to_db(
                        &site_name,
                        cache_db_path,
                        &db_path,
                        12,
                    ) {
                        Ok(inserted) => {
                            progress.set_message(&format!(
                                "🔁 Wikipedia ingest inserted {} placeholder rows",
                                inserted
                            ));
                        }
                        Err(e) => {
                            progress.set_message("⚠️ Wikipedia ingest failed");
                            eprintln!("Wikipedia ingest error: {}", e);
                        }
                    }
                }
            }

            // After scraping, generate album output from the same DB
            // Open the DB that the collector used and call the conn-taking API so
            // both scraper and generator share the same connection lifecycle.
            progress.set_message("🔗 Opening database for album generation...");
            match crate::db_utils::init_db(&db_path) {
                Ok(conn) => {
                    // Attempt to copy a flag into the album images dir if enabled in config
                    let print_flag = cfg_file.print_flag_cover.unwrap_or(true);
                    if print_flag {
                        progress.set_message("🏳️ Downloading country flag...");
                        let album_images = cfg.image_path.clone();
                        match crate::flagdb::copy_flag_for_album(&site_name, &album_images, "w320")
                        {
                            Ok(p) => info!("Copied flag into album images: {}", p),
                            Err(e) => info!(
                                "No flag found for '{}' or download failed; continuing. error={}",
                                site_name, e
                            ),
                        }
                    }
                    // Optional: fetch Wikipedia summary if requested by config
                    let fetch_wiki = cfg_file
                        .fetch_wikipedia
                        .unwrap_or_else(|| cfg_file.include_wikipedia_content.unwrap_or(false));
                    if fetch_wiki {
                        progress.set_message("📚 Fetching Wikipedia content...");
                        let cache_db_path = cfg_file
                            .wikipedia_cache_db_path
                            .as_deref()
                            .unwrap_or("wikipedia_cache.db");
                        let fetcher = crate::stampworld::fetch::ReqwestFetcher::new_for_api();
                        match crate::wikipedia::fetch_and_cache_content(
                            &site_name,
                            cache_db_path,
                            &fetcher,
                        ) {
                            Ok(_) => info!(
                                "✅ Successfully cached Wikipedia content for '{}'",
                                site_name
                            ),
                            Err(e) => {
                                info!("❌ Wikipedia fetch failed for {}: {}", site_name, e);
                                // Log the full error for debugging
                                error!("Wikipedia fetch error details: {:?}", e);
                            }
                        }
                    }

                    // Ingest Colnect CSV if provided
                    if let Some(csv_path) = &cli.colnect_csv {
                        progress.set_message(&format!("📥 Ingesting Colnect CSV: {}", csv_path));
                        match crate::colnect::parser::parse_csv(csv_path) {
                            Ok(stamps) => {
                                let default_catalog_code =
                                    crate::config::load_config("config.toml")
                                        .effective_default_catalog_code();
                                match crate::colnect::ingest::ingest_colnect_data_with_default_catalog(
                                    &conn,
                                    &stamps,
                                    &default_catalog_code,
                                ) {
                                    Ok(_) => info!("✅ Successfully ingested Colnect data"),
                                    Err(e) => error!("❌ Failed to ingest Colnect data: {}", e),
                                }
                            }
                            Err(e) => error!("❌ Failed to parse Colnect CSV: {}", e),
                        }

                        info!(
                            "Review-first mode enabled: Colnect ingest kept in colnect_stamps only"
                        );
                    }

                    progress.set_message(&format!(
                        "📖 Generating album output for: {}",
                        canonical_storage_dir
                    ));
                    let output_path = crate::album_globals::generate_album_output_conn(
                        &conn,
                        &canonical_storage_dir,
                        None::<fn(usize)>,
                        Some(cli.start_year),
                        Some(cli.end_year),
                    );

                    progress.finish_with_message(&format!(
                        "Album '{}' completed successfully! 🎉 📄 File: {}",
                        canonical_storage_dir,
                        output_path.display()
                    ));
                }
                Err(e) => {
                    progress.finish_with_error(&format!("Database connection failed: {}", e));
                    eprintln!(
                        "Failed to open database for generation '{}': {}",
                        db_path, e
                    );
                    return Err(e.into());
                }
            }
        }
        Err(e) => {
            progress.finish_with_error(&format!("Data collection failed: {}", e));
            eprintln!("Failed to collect stampworld data: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
