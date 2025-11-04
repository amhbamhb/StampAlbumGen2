use crate::config::Config;
use crate::stampworld::fetch::{Fetcher, ReqwestFetcher};
use log::info;
use rusqlite::{params, Connection};
use scraper::{Html, Selector};
use std::fs;
use std::path::Path;

/// Source of the content
#[derive(Debug, Clone)]
pub enum ContentSource {
    Grokipedia,
    Wikipedia,
}

impl ContentSource {
    pub fn as_str(&self) -> &str {
        match self {
            ContentSource::Grokipedia => "grokipedia",
            ContentSource::Wikipedia => "wikipedia",
        }
    }
}

/// Database-based content management for Wikipedia and Grokipedia
/// Store content in cache database for integration with album generation
pub fn fetch_and_cache_content(
    topic: &str,
    cache_db_path: &str,
    fetcher: &dyn Fetcher,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "DEBUG: fetch_and_cache_content called with topic='{}', db='{}'",
        topic, cache_db_path
    );
    info!(
        "Starting fetch_and_cache_content for topic: '{}', db: '{}'",
        topic, cache_db_path
    );

    // Create cache database if it doesn't exist
    let conn = Connection::open(cache_db_path)?;
    info!("Database opened successfully");

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
    info!("Table creation/check completed");

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
        info!("Added source column to wikipedia_content table");
    }

    // Check if we already have content for this topic
    let mut stmt = conn.prepare("SELECT content FROM wikipedia_content WHERE topic = ?1")?;
    let existing: Result<String, rusqlite::Error> =
        stmt.query_row(params![topic], |row| row.get(0));

    if existing.is_ok() {
        info!("Content for '{}' already cached", topic);
        return Ok(());
    }

    info!("No cached content found, attempting to fetch from Grokipedia first");

    // Try Grokipedia first
    match fetch_grokipedia_content(topic, fetcher) {
        Ok(content) => {
            info!(
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
                    info!("Successfully cached Grokipedia content for '{}'", topic);
                    return Ok(());
                }
                Err(e) => {
                    info!("Failed to store Grokipedia content: {}", e);
                    return Err(format!("Database error: {}", e).into());
                }
            }
        }
        Err(e) => {
            info!(
                "Grokipedia fetch failed for '{}': {}, trying Wikipedia",
                topic, e
            );
        }
    }

    info!("Attempting Wikipedia fallback");

    // Fall back to Wikipedia
    match fetch_wikipedia_content(topic, fetcher) {
        Ok(content) => {
            info!(
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
                    info!("Successfully cached Wikipedia content for '{}'", topic);
                    Ok(())
                }
                Err(e) => {
                    info!("Failed to store Wikipedia content: {}", e);
                    Err(format!("Database error: {}", e).into())
                }
            }
        }
        Err(e) => {
            info!("Both Grokipedia and Wikipedia failed for {}: {}", topic, e);
            Err(format!("Both Grokipedia and Wikipedia failed for {}: {}", topic, e).into())
        }
    }
}

/// Fetch content from Grokipedia
fn fetch_grokipedia_content(
    topic: &str,
    fetcher: &dyn Fetcher,
) -> Result<String, Box<dyn std::error::Error>> {
    let topic_enc = topic.replace(' ', "_");
    let url = format!(
        "https://grokipedia.com/page/Postage_stamps_and_postal_history_of_{}",
        topic_enc
    );
    info!("Fetching Grokipedia content for '{}' -> {}", topic, url);

    let (status, body) = fetcher.fetch_text(&url)?;

    if !status.is_success() {
        return Err(format!("Grokipedia returned {} for {}", status, topic).into());
    }

    // Check if it's a "Page Not Found" response
    if body.contains("Page Not Found") || body.contains("not found") {
        return Err(format!("Grokipedia page not found for {}", topic).into());
    }

    // Parse HTML and extract clean text content
    parse_grokipedia_html(&body)
}

/// Fetch content from Wikipedia (full page with sections)
fn fetch_wikipedia_content(
    topic: &str,
    fetcher: &dyn Fetcher,
) -> Result<String, Box<dyn std::error::Error>> {
    let topic_enc = topic.replace(' ', "_");
    let url = format!(
        "https://en.wikipedia.org/w/api.php?action=parse&page=Postage_stamps_and_postal_history_of_{}&format=json&prop=text",
        topic_enc
    );
    info!("Fetching Wikipedia content for '{}' -> {}", topic, url);

    let (status, body) = fetcher.fetch_text(&url)?;
    if !status.is_success() {
        return Err(format!("Wikipedia returned {} for {}", status, topic).into());
    }

    let j: serde_json::Value = serde_json::from_str(&body)?;
    if let Some(text) = j
        .get("parse")
        .and_then(|p| p.get("text"))
        .and_then(|t| t.get("*"))
        .and_then(|v| v.as_str())
    {
        // Parse HTML and extract clean text content with sections
        parse_wikipedia_html(text)
    } else {
        Err("No content found in Wikipedia response".into())
    }
}

/// Parse Grokipedia HTML and extract clean text content
fn parse_grokipedia_html(html: &str) -> Result<String, Box<dyn std::error::Error>> {
    let document = Html::parse_document(html);

    // Try to find the main content area
    let content_selector = Selector::parse("article").unwrap();
    let mut content = String::new();

    if let Some(article) = document.select(&content_selector).next() {
        // Extract text from paragraphs and headings
        let paragraph_selector = Selector::parse("p, h1, h2, h3, h4, h5, h6").unwrap();

        for element in article.select(&paragraph_selector) {
            let tag_name = element.value().name();
            let text = element
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();

            if !text.is_empty() {
                // Add section headers for h1-h6
                if tag_name.starts_with('h') && tag_name.len() == 2 {
                    let level = tag_name.chars().nth(1).unwrap().to_digit(10).unwrap_or(1);
                    content.push_str(&format!("{} {}\n\n", "#".repeat(level as usize), text));
                } else {
                    content.push_str(&format!("{}\n\n", text));
                }
            }
        }
    }

    if content.is_empty() {
        return Err("No content found in Grokipedia HTML".into());
    }

    Ok(content)
}

/// Parse Wikipedia HTML and extract clean text content with sections
fn parse_wikipedia_html(html: &str) -> Result<String, Box<dyn std::error::Error>> {
    let document = Html::parse_document(html);

    // Find the main content area
    let content_selector = Selector::parse(".mw-content-ltr").unwrap();
    let mut content = String::new();

    if let Some(content_div) = document.select(&content_selector).next() {
        // Extract sections and paragraphs
        let section_selector = Selector::parse("h2, h3, h4, h5, h6, p").unwrap();

        for element in content_div.select(&section_selector) {
            let tag_name = element.value().name();
            let text = element
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();

            // Skip empty elements and references
            if text.is_empty() || text.starts_with('[') && text.ends_with(']') {
                continue;
            }

            // Skip edit links and other Wikipedia artifacts
            if text.contains("edit") && text.len() < 20 {
                continue;
            }

            // Add section headers
            if tag_name.starts_with('h') && tag_name.len() == 2 {
                let level = tag_name.chars().nth(1).unwrap().to_digit(10).unwrap_or(2);
                content.push_str(&format!("{} {}\n\n", "#".repeat(level as usize), text));
            } else {
                // Regular paragraph
                content.push_str(&format!("{}\n\n", text));
            }
        }
    }

    if content.is_empty() {
        return Err("No content found in Wikipedia HTML".into());
    }

    Ok(content)
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

/// Generate AlbumEasy PAGE_TEXT_PARAGRAPH commands for content
pub fn format_content_for_album(
    content: &str,
    source: &ContentSource,
    config: &Config,
) -> Vec<String> {
    let mut lines = Vec::new();

    // Get configuration values with defaults
    let paragraph_font = config.wikipedia_paragraph_font.as_deref().unwrap_or("HN");
    let paragraph_font_size = config.wikipedia_paragraph_font_size.unwrap_or(12);
    let paragraph_alignment = config
        .wikipedia_paragraph_alignment
        .as_deref()
        .unwrap_or("JUSTIFIED");
    let paragraph_padding = config.wikipedia_paragraph_padding.unwrap_or(10.0);
    let paragraph_vspace = config.wikipedia_paragraph_vspace.unwrap_or(5.0);
    let paragraph_right_padding = config.wikipedia_paragraph_right_padding.unwrap_or(10.0);

    let section_title_font = config
        .wikipedia_section_title_font
        .as_deref()
        .unwrap_or("HB");
    let section_title_font_size = config.wikipedia_section_title_font_size.unwrap_or(14);

    // Add section header indicating source
    let source_name = match source {
        ContentSource::Grokipedia => "Grokipedia",
        ContentSource::Wikipedia => "Wikipedia",
    };

    lines.push(format!(
        "PAGE_TEXT ({} {} \"{} Postal History\" 3.0)",
        section_title_font, section_title_font_size, source_name
    ));

    // Process content line by line, handling section headers and paragraphs
    let content_lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < content_lines.len() {
        let line = content_lines[i].trim();

        if line.is_empty() {
            i += 1;
            continue;
        }

        // Check if this is a section header (starts with #)
        if line.starts_with('#') {
            let header_text = line.trim_start_matches('#').trim();
            if !header_text.is_empty() {
                // Add section header
                lines.push(format!(
                    "PAGE_TEXT ({} {} \"{}\" 3.0)",
                    section_title_font, section_title_font_size, header_text
                ));
            }
        } else {
            // This is a paragraph - collect all consecutive non-empty lines
            let mut paragraph_lines = Vec::new();
            while i < content_lines.len() && !content_lines[i].trim().is_empty() {
                paragraph_lines.push(content_lines[i].trim());
                i += 1;
            }

            if !paragraph_lines.is_empty() {
                let paragraph_text = paragraph_lines.join(" ");

                // Start paragraph with formatting
                lines.push(format!(
                    "PAGE_TEXT_PARAGRAPH_START({} {} {} {:.1} {:.1} {:.1})",
                    paragraph_font,
                    paragraph_font_size,
                    paragraph_alignment,
                    paragraph_padding,
                    paragraph_vspace,
                    paragraph_right_padding
                ));

                // Add the paragraph content, escaping quotes
                let escaped_content = paragraph_text.replace('"', "\\\"");
                lines.push(escaped_content);

                // End paragraph
                lines.push("PAGE_TEXT_PARAGRAPH_END".to_string());
            }
        }

        i += 1;
    }

    lines
}

/// Legacy function for backwards compatibility
pub fn format_wikipedia_content_for_album(content: &str, config: &Config) -> Vec<String> {
    format_content_for_album(content, &ContentSource::Wikipedia, config)
}

/// Legacy function for backwards compatibility
pub fn fetch_and_cache_wikipedia_content(
    topic: &str,
    cache_db_path: &str,
    fetcher: &dyn Fetcher,
) -> Result<(), Box<dyn std::error::Error>> {
    fetch_and_cache_content(topic, cache_db_path, fetcher)
}

/// Legacy function for backwards compatibility
pub fn get_cached_wikipedia_content(
    topic: &str,
    cache_db_path: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    match get_cached_content(topic, cache_db_path) {
        Ok(Some((content, _))) => Ok(Some(content)),
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    }
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
    let fetcher = ReqwestFetcher::new_for_api();
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

/// Estimate the height (in mm) required to display text content on a page
/// This includes calculations for font size, line wrapping, and spacing
pub fn estimate_text_height(text: &str, config: &Config) -> f64 {
    // Get configuration values
    let page_width = config.album_page_width_mm.unwrap_or(216.0) as f64;
    let text_line_height = config.text_line_height_mm.unwrap_or(2.6) as f64;

    // Estimate characters per line based on font size
    // This is a rough approximation - smaller fonts fit more characters per line
    let paragraph_font_size = config.wikipedia_paragraph_font_size.unwrap_or(12) as f64;
    let chars_per_line = (page_width / (paragraph_font_size * 0.6)).floor() as usize;

    // Count actual lines and estimate wrapped lines
    let mut total_lines = 0;
    for line in text.lines() {
        let line_len = line.len();
        if line_len == 0 {
            total_lines += 1; // Empty lines still take space
        } else {
            // Estimate how many display lines this text line will wrap to
            let wrapped_lines = ((line_len as f64) / (chars_per_line as f64)).ceil() as usize;
            total_lines += wrapped_lines.max(1);
        }
    }

    // Add spacing between paragraphs (vspace)
    let paragraph_vspace = config.wikipedia_paragraph_vspace.unwrap_or(5.0) as f64;
    let num_paragraphs = text.lines().filter(|l| !l.trim().is_empty()).count() as f64;

    // Calculate total height with conservative multiplier
    let text_height = (total_lines as f64) * text_line_height;
    let spacing_height = num_paragraphs * paragraph_vspace;
    let total_estimated_height = text_height + spacing_height;

    // Add 20% buffer for estimation errors
    total_estimated_height * 1.2
}

/// Estimate the height of a section header
pub fn estimate_header_height(header_text: &str, config: &Config) -> f64 {
    let text_line_height = config.text_line_height_mm.unwrap_or(2.6) as f64;

    // Headers typically take 1-2 lines with some extra spacing
    let header_lines = ((header_text.len() as f64) / 40.0).ceil() as f64; // Rough estimate
    let estimated_height = header_lines * text_line_height * 1.2; // 20% extra for header spacing

    // Add 30% buffer for headers (they can be more variable)
    estimated_height * 1.3
}
