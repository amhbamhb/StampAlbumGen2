// HTML utilities for StampAlbumGen2
use scraper::Html;
use reqwest::blocking::Client;

pub fn fetch_html(url: &str) -> Result<Html, reqwest::Error> {
    println!("[DEBUG] Fetching HTML from URL: {}", url);
    let client = Client::new();
    let res = client.get(url).send()?;
    println!("[DEBUG] HTTP status: {}", res.status());
    let body = res.text()?;
    println!("[DEBUG] HTML fetched, length: {}", body.len());
    Ok(Html::parse_document(&body))
}

// Add more HTML-related functions as needed
