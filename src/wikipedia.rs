use log::info;
use reqwest::blocking::Client;
use std::fs;
use std::path::Path;

/// Fetch a short extract for a given topic from Wikipedia's REST API and save it
/// to the album directory as wiki_summary.txt. Returns Ok(path) on success.
pub fn fetch_and_save_summary(topic: &str, album_dir: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::builder().user_agent("StampAlbumGen2/1.0").build()?;
    // Simple Wikipedia REST summary endpoint. Topic needs percent-encoding for path.
    // Wikipedia page titles use underscores for spaces and are case-sensitive;
    // for a simple heuristic replace spaces with underscores.
    let topic_enc = topic.replace(' ', "_");
    let url = format!("https://en.wikipedia.org/api/rest_v1/page/summary/{}", topic_enc);
    info!("Fetching Wikipedia summary for '{}' -> {}", topic, url);
    let resp = client.get(&url).send()?;
    if !resp.status().is_success() {
        return Err(format!("Wikipedia returned {} for {}", resp.status(), topic).into());
    }
    let j: serde_json::Value = resp.json()?;
    if let Some(extract) = j.get("extract").and_then(|v| v.as_str()) {
        fs::create_dir_all(album_dir)?;
        let path = Path::new(album_dir).join("wiki_summary.txt");
        fs::write(&path, extract)?;
        return Ok(path.to_string_lossy().to_string());
    }
    Err("No extract found in Wikipedia response".into())
}
