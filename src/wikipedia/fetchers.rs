use crate::stampworld::fetch::Fetcher;

/// Fetch content from Grokipedia
pub fn fetch_grokipedia_content(
    topic: &str,
    fetcher: &dyn Fetcher,
) -> Result<String, Box<dyn std::error::Error>> {
    let topic_enc = topic.replace(' ', "_");
    let url = format!(
        "https://grokipedia.com/page/Postage_stamps_and_postal_history_of_{}",
        topic_enc
    );
    log::info!("Fetching Grokipedia content for '{}' -> {}", topic, url);

    let (status, body) = fetcher.fetch_text(&url)?;

    if !status.is_success() {
        return Err(format!("Grokipedia returned {} for {}", status, topic).into());
    }

    // Check if it's a "Page Not Found" response
    if body.contains("Page Not Found") || body.contains("not found") {
        return Err(format!("Grokipedia page not found for {}", topic).into());
    }

    // Parse HTML and extract clean text content
    crate::wikipedia::parsers::parse_grokipedia_html(&body)
}

/// Fetch content from Wikipedia (full page with sections)
pub fn fetch_wikipedia_content(
    topic: &str,
    fetcher: &dyn Fetcher,
) -> Result<String, Box<dyn std::error::Error>> {
    let topic_enc = topic.replace(' ', "_");
    let url = format!(
        "https://en.wikipedia.org/w/api.php?action=parse&page=Postage_stamps_and_postal_history_of_{}&format=json&prop=text",
        topic_enc
    );
    log::info!("Fetching Wikipedia content for '{}' -> {}", topic, url);

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
        crate::wikipedia::parsers::parse_wikipedia_html(text)
    } else {
        Err("No content found in Wikipedia response".into())
    }
}
