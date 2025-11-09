use crate::wikipedia::models::ContentSource;
use rusqlite::{params, Connection};

/// Database-based content management for Wikipedia and Grokipedia
/// Store content in cache database for integration with album generation
pub fn fetch_and_cache_content(
    topic: &str,
    cache_db_path: &str,
    fetcher: &dyn crate::stampworld::fetch::Fetcher,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "DEBUG: fetch_and_cache_content called with topic='{}', db='{}'",
        topic, cache_db_path
    );
    log::info!(
        "Starting fetch_and_cache_content for topic: '{}', db: '{}'",
        topic,
        cache_db_path
    );

    // Create cache database if it doesn't exist
    let conn = Connection::open(cache_db_path)?;
    log::info!("Database opened successfully");

    // Create table for content if not exists (with migration support)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS wikipedia_content (
            id INTEGER PRIMARY KEY,
            topic TEXT UNIQUE,
            content TEXT,
            source TEXT,
            fetched_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    log::info!("Table creation/check completed");

    // Add source column if it doesn't exist (migration) - check first
    let has_source_column = conn
        .prepare("PRAGMA table_info(wikipedia_content)")?
        .query_map([], |row| {
            let name: String = row.get(1)?;
            Ok(name)
        })?
        .any(|name| name.as_ref().map_or(false, |n| n == "source"));

    if !has_source_column {
        let _ = conn.execute(
            "ALTER TABLE wikipedia_content ADD COLUMN source TEXT DEFAULT 'wikipedia'",
            [],
        );
        log::info!("Added source column to wikipedia_content table");
    }

    // Check if we already have content for this topic
    let mut stmt = conn.prepare("SELECT content FROM wikipedia_content WHERE topic = ?1")?;
    let existing: Result<String, rusqlite::Error> =
        stmt.query_row(params![topic], |row| row.get(0));

    if existing.is_ok() {
        log::info!("Content for '{}' already cached", topic);
        return Ok(());
    }

    log::info!("No cached content found, attempting to fetch from Grokipedia first");

    // Try Grokipedia first
    match crate::wikipedia::fetchers::fetch_grokipedia_content(topic, fetcher) {
        Ok(content) => {
            log::info!(
                "Grokipedia fetch successful, content length: {}",
                content.len()
            );
            // Store Grokipedia content
            let result = conn.execute(
                "INSERT OR REPLACE INTO wikipedia_content (topic, content, source) VALUES (?1, ?2, ?3)",
                params![topic, content, ContentSource::Grokipedia.as_str()],
            );
            match result {
                Ok(_) => {
                    log::info!("Successfully cached Grokipedia content for '{}'", topic);
                    return Ok(());
                }
                Err(e) => {
                    log::info!("Failed to store Grokipedia content: {}", e);
                    return Err(format!("Database error: {}", e).into());
                }
            }
        }
        Err(e) => {
            log::info!(
                "Grokipedia fetch failed for '{}': {}, trying Wikipedia",
                topic,
                e
            );
        }
    }

    log::info!("Attempting Wikipedia fallback");

    // Fall back to Wikipedia
    match crate::wikipedia::fetchers::fetch_wikipedia_content(topic, fetcher) {
        Ok(content) => {
            log::info!(
                "Wikipedia fetch successful, content length: {}",
                content.len()
            );
            // Store Wikipedia content
            let result = conn.execute(
                "INSERT OR REPLACE INTO wikipedia_content (topic, content, source) VALUES (?1, ?2, ?3)",
                params![topic, content, ContentSource::Wikipedia.as_str()],
            );
            match result {
                Ok(_) => {
                    log::info!("Successfully cached Wikipedia content for '{}'", topic);
                    Ok(())
                }
                Err(e) => {
                    log::info!("Failed to store Wikipedia content: {}", e);
                    Err(format!("Database error: {}", e).into())
                }
            }
        }
        Err(e) => {
            log::info!("Both Grokipedia and Wikipedia failed for {}: {}", topic, e);
            Err(format!("Both Grokipedia and Wikipedia failed for {}: {}", topic, e).into())
        }
    }
}

/// Retrieve cached content for a topic from database
pub fn get_cached_content(
    topic: &str,
    cache_db_path: &str,
) -> Result<Option<(String, ContentSource)>, Box<dyn std::error::Error>> {
    let conn = Connection::open(cache_db_path)?;

    let mut stmt =
        conn.prepare("SELECT content, source FROM wikipedia_content WHERE topic = ?1")?;
    let result: Result<(String, String), rusqlite::Error> = stmt.query_row(params![topic], |row| {
        Ok((
            row.get(0)?,
            row.get(1).unwrap_or_else(|_| "wikipedia".to_string()),
        ))
    });

    match result {
        Ok((content, source_str)) => {
            let source = if source_str == "grokipedia" {
                ContentSource::Grokipedia
            } else {
                ContentSource::Wikipedia
            };
            Ok(Some((content, source)))
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.into()),
    }
}
