use log::{info, warn};
use std::error::Error;

/// Ingest cached Wikipedia/Grokipedia content into the album database.
///
/// This function is intentionally conservative: it will not overwrite an
/// existing database that already contains stamps for the given `topic`.
/// If the album DB is empty for the country/topic, it will create a small
/// set of placeholder/Wikipedia-derived rows so downstream album generation
/// has some data to render and subsequent runs will operate from the DB.
pub fn ingest_wikipedia_to_db(
    topic: &str,
    cache_db_path: &str,
    album_db_path: &str,
    max_entries: usize,
    series_prefix: &str,
) -> Result<usize, Box<dyn Error>> {
    info!(
        "ingest_wikipedia_to_db: topic='{}' cache_db='{}' album_db='{}'",
        topic, cache_db_path, album_db_path
    );

    // Open or initialize album DB (ensures stamps table exists)
    let conn = crate::db_utils::init_db(album_db_path)?;

    // Check whether the DB already has entries for this country/topic
    let existing_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM stamps WHERE country = ?1",
        rusqlite::params![topic],
        |r| r.get(0),
    )?;

    if existing_count > 0 {
        info!(
            "Album DB '{}' already contains {} stamp row(s) for '{}'; skipping ingest",
            album_db_path, existing_count, topic
        );
        return Ok(0);
    }

    // Get cached content (must be present already via fetch_and_cache_content)
    match crate::wikipedia::get_cached_content(topic, cache_db_path)? {
        Some((content, source)) => {
            let src_name = match source {
                crate::wikipedia::ContentSource::Grokipedia => "Grokipedia",
                crate::wikipedia::ContentSource::Wikipedia => "Wikipedia",
            };
            info!(
                "Found cached {} content for '{}', length={}",
                src_name,
                topic,
                content.len()
            );

            // Split into paragraphs (simple heuristic) and insert up to max_entries
            let paragraphs: Vec<&str> = content
                .split("\n\n")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect();

            if paragraphs.is_empty() {
                warn!(
                    "Cached content for '{}' contained no paragraphs; nothing to ingest",
                    topic
                );
                return Ok(0);
            }

            let mut inserted = 0usize;
            let series_prefix = {
                let trimmed = series_prefix.trim();
                if trimmed.is_empty() {
                    "Wikipedia import"
                } else {
                    trimmed
                }
            };
            let series_name = format!("{} ({})", topic, series_prefix);

            for (i, para) in paragraphs.iter().enumerate() {
                if inserted >= max_entries {
                    break;
                }

                // Create a lightweight sg_number for Wikipedia rows to avoid collisions
                let sg = format!("W{}", i + 1);

                // Truncate paragraph to a reasonable length for description
                let description = if para.len() > 1024 {
                    format!("{}...", &para[..1024])
                } else {
                    para.to_string()
                };

                match crate::db_utils::insert_stamp(
                    &conn,
                    topic,              // country
                    &series_name,       // series
                    0,                  // year unknown
                    "",                 // month
                    "0",                // _month_num
                    "",                 // day
                    &sg,                // sg_number
                    "WIKIPEDIA",        // type
                    "",                 // denomination
                    "",                 // color
                    &description,       // description
                    "",                 // stamps_issued
                    "",                 // mint_condition
                    "",                 // unused
                    "",                 // used
                    "",                 // letter_fdc
                    "",                 // currency
                    "",                 // image_path
                    "",                 // remote_image_url
                    "",                 // original_image_url
                    "",                 // watermark
                    "",                 // design
                    "",                 // engraving
                    "",                 // perforation
                    series_prefix,      // context
                ) {
                    Ok(true) => {
                        inserted += 1;
                    }
                    Ok(false) => {
                        // already exists or no-op; continue
                    }
                    Err(e) => {
                        warn!(
                            "Failed to insert Wikipedia-derived row for '{}': {}",
                            topic, e
                        );
                    }
                }
            }

            info!(
                "Ingest complete: inserted {} rows into {}",
                inserted, album_db_path
            );
            Ok(inserted)
        }
        None => {
            warn!(
                "No cached Wikipedia/Grokipedia content found for '{}'; nothing to ingest",
                topic
            );
            Ok(0)
        }
    }
}
