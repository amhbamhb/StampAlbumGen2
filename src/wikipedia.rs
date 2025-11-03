use crate::config::Config;
use crate::stampworld::fetch::{Fetcher, ReqwestFetcher};
use log::info;
use rusqlite::{params, Connection};
use std::fs;
use std::path::Path;

/// Database-based Wikipedia content management
/// Store Wikipedia content in cache database for integration with album generation
pub fn fetch_and_cache_wikipedia_content(
    topic: &str,
    cache_db_path: &str,
    fetcher: &dyn Fetcher,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create cache database if it doesn't exist
    let conn = Connection::open(cache_db_path)?;

    // Create table for Wikipedia content if not exists
    conn.execute(
        "CREATE TABLE IF NOT EXISTS wikipedia_content (
            id INTEGER PRIMARY KEY,
            topic TEXT UNIQUE,
            content TEXT,
            fetched_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    // Check if we already have content for this topic
    let mut stmt = conn.prepare("SELECT content FROM wikipedia_content WHERE topic = ?1")?;
    let existing: Result<String, rusqlite::Error> =
        stmt.query_row(params![topic], |row| row.get(0));

    if existing.is_ok() {
        info!("Wikipedia content for '{}' already cached", topic);
        return Ok(());
    }

    // Fetch new content from Wikipedia
    let topic_enc = topic.replace(' ', "_");
    let url = format!(
        "https://en.wikipedia.org/api/rest_v1/page/summary/{}",
        topic_enc
    );
    info!("Fetching Wikipedia content for '{}' -> {}", topic, url);

    let (status, body) = fetcher.fetch_text(&url)?;
    if !status.is_success() {
        return Err(format!("Wikipedia returned {} for {}", status, topic).into());
    }

    let j: serde_json::Value = serde_json::from_str(&body)?;
    if let Some(extract) = j.get("extract").and_then(|v| v.as_str()) {
        // Store in database
        conn.execute(
            "INSERT OR REPLACE INTO wikipedia_content (topic, content) VALUES (?1, ?2)",
            params![topic, extract],
        )?;
        info!("Cached Wikipedia content for '{}'", topic);
        Ok(())
    } else {
        Err("No extract found in Wikipedia response".into())
    }
}

/// Retrieve Wikipedia content for a topic from cache database
pub fn get_cached_wikipedia_content(
    topic: &str,
    cache_db_path: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let conn = Connection::open(cache_db_path)?;

    let mut stmt = conn.prepare("SELECT content FROM wikipedia_content WHERE topic = ?1")?;
    let result: Result<String, rusqlite::Error> = stmt.query_row(params![topic], |row| row.get(0));

    match result {
        Ok(content) => Ok(Some(content)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

/// Generate AlbumEasy PAGE_TEXT commands for Wikipedia content
pub fn format_wikipedia_content_for_album(content: &str, config: &Config) -> Vec<String> {
    let mut lines = Vec::new();

    // Get configuration values with defaults
    let font = config.content_wiki_font.as_deref().unwrap_or("HN");
    let font_size = config.content_wiki_font_size.unwrap_or(12);
    let max_lines = config.content_wiki_max_lines_per_page.unwrap_or(5);
    let chars_per_line = config.content_wiki_estimated_chars_per_line.unwrap_or(250);

    // Break content into paragraphs
    let paragraphs: Vec<&str> = content
        .split('\n')
        .filter(|p| !p.trim().is_empty())
        .collect();

    if paragraphs.is_empty() {
        return lines;
    }

    // Add section header for Wikipedia content
    let header_font = config
        .wikipedia_section_title_font
        .as_deref()
        .unwrap_or("HB");
    let header_size = config.wikipedia_section_title_font_size.unwrap_or(14);
    lines.push(format!(
        "PAGE_TEXT ({} {} \"Postal History\" 3.0)",
        header_font, header_size
    ));

    let mut lines_used = 1; // Header counts as one line

    // Process paragraphs
    for paragraph in paragraphs {
        if lines_used >= max_lines {
            break; // Respect max lines per page limit
        }

        // Break long paragraphs into chunks that fit
        let words: Vec<&str> = paragraph.split_whitespace().collect();
        let mut current_line = String::new();

        for word in words {
            let test_line = if current_line.is_empty() {
                word.to_string()
            } else {
                format!("{} {}", current_line, word)
            };

            if test_line.len() <= chars_per_line {
                current_line = test_line;
            } else {
                // Current line is ready, emit it
                if !current_line.is_empty() {
                    lines.push(format!(
                        "PAGE_TEXT ({} {} \"{}\" 1.0)",
                        font,
                        font_size,
                        current_line.replace('"', "\\\"")
                    ));
                    lines_used += 1;
                    if lines_used >= max_lines {
                        break;
                    }
                }
                current_line = word.to_string();
            }
        }

        // Emit the last line of this paragraph
        if !current_line.is_empty() && lines_used < max_lines {
            lines.push(format!(
                "PAGE_TEXT ({} {} \"{}\" 1.0)",
                font,
                font_size,
                current_line.replace('"', "\\\"")
            ));
            lines_used += 1;
        }

        if lines_used >= max_lines {
            break;
        }
    }

    lines
}

/// Legacy functions for backwards compatibility

/// Fetch a short extract for a given topic from Wikipedia's REST API and save it
/// to the album directory as wiki_summary.txt. Returns Ok(path) on success.
/// Backwards-compatible wrapper: constructs a ReqwestFetcher and calls
/// `fetch_and_save_summary_with_fetcher`.
pub fn fetch_and_save_summary(
    topic: &str,
    album_dir: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let fetcher = ReqwestFetcher::new();
    fetch_and_save_summary_with_fetcher(topic, album_dir, &fetcher)
}

/// Fetch a short extract for a given topic using the provided `Fetcher` and save it
/// to the album directory as `wiki_summary.txt`. Returns Ok(path) on success.
pub fn fetch_and_save_summary_with_fetcher(
    topic: &str,
    album_dir: &str,
    fetcher: &dyn Fetcher,
) -> Result<String, Box<dyn std::error::Error>> {
    // Simple Wikipedia REST summary endpoint. Topic needs percent-encoding for path.
    // Wikipedia page titles use underscores for spaces and are case-sensitive;
    // for a simple heuristic replace spaces with underscores.
    let topic_enc = topic.replace(' ', "_");
    let url = format!(
        "https://en.wikipedia.org/api/rest_v1/page/summary/{}",
        topic_enc
    );
    info!("Fetching Wikipedia summary for '{}' -> {}", topic, url);

    let (status, body) = fetcher.fetch_text(&url)?;
    if !status.is_success() {
        return Err(format!("Wikipedia returned {} for {}", status, topic).into());
    }
    let j: serde_json::Value = serde_json::from_str(&body)?;
    if let Some(extract) = j.get("extract").and_then(|v| v.as_str()) {
        fs::create_dir_all(album_dir)?;
        let path = Path::new(album_dir).join("wiki_summary.txt");
        fs::write(&path, extract)?;
        return Ok(path.to_string_lossy().to_string());
    }
    Err("No extract found in Wikipedia response".into())
}
