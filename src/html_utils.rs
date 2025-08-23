// HTML utilities for StampAlbumGen2
use scraper::{Html, Selector};
use reqwest::blocking::Client;
use reqwest::StatusCode;

pub fn fetch_html(url: &str) -> Result<Html, reqwest::Error> {
    let client = Client::new();
    let res = client.get(url).send()?;
    let body = res.text()?;
    Ok(Html::parse_document(&body))
}

// Add more HTML-related functions as needed
