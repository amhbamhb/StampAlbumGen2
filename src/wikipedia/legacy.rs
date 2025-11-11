use crate::config::Config;
use crate::stampworld::fetch::{Fetcher, ReqwestFetcher};
use crate::wikipedia::database;
use crate::wikipedia::formatters;
use crate::wikipedia::models::ContentSource;
use std::fs;
use std::path::Path;

/// Legacy function for backwards compatibility
pub fn fetch_and_cache_wikipedia_content(
    topic: &str,
    cache_db_path: &str,
    fetcher: &dyn Fetcher,
) -> Result<(), Box<dyn std::error::Error>> {
    database::fetch_and_cache_content(topic, cache_db_path, fetcher)
}

/// Legacy function for backwards compatibility
pub fn get_cached_wikipedia_content(
    topic: &str,
    cache_db_path: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    match database::get_cached_content(topic, cache_db_path) {
        Ok(Some((content, _))) => Ok(Some(content)),
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    }
}

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
    log::info!("Fetching Wikipedia summary for '{}' -> {}", topic, url);

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
