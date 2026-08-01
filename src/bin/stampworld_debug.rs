use std::env;
use std::fs;
use std::path::Path;

use reqwest::StatusCode;

use stamp_album_gen2::types::ImageStatus;

/// A tiny local fetcher used for testing which returns the contents of a
/// provided local HTML file for any URL. Falls back to a real ReqwestFetcher
/// when no local file is configured.
struct LocalFetcher {
    local_html: Option<String>,
}

impl LocalFetcher {
    fn new_from_path(path: &str) -> Self {
        if Path::new(path).exists() {
            match fs::read_to_string(path) {
                Ok(s) => Self {
                    local_html: Some(s),
                },
                Err(_) => Self { local_html: None },
            }
        } else {
            Self { local_html: None }
        }
    }
}

impl stamp_album_gen2::stampworld::fetch::Fetcher for LocalFetcher {
    fn fetch_text(&self, _url: &str) -> Result<(StatusCode, String), reqwest::Error> {
        if let Some(s) = &self.local_html {
            Ok((StatusCode::OK, s.clone()))
        } else {
            // Delegate to ReqwestFetcher when no local HTML available
            let r = stamp_album_gen2::stampworld::fetch::ReqwestFetcher::new();
            r.fetch_text(_url)
        }
    }

    fn fetch_bytes(
        &self,
        url: &str,
    ) -> Result<(StatusCode, reqwest::header::HeaderMap, Vec<u8>), reqwest::Error> {
        // Not needed for this debug run; delegate to ReqwestFetcher
        let r = stamp_album_gen2::stampworld::fetch::ReqwestFetcher::new_for_images();
        r.fetch_bytes(url)
    }
}

fn month_number(_m: &str) -> String {
    // Keep this minimal for debug: return 0 for unknown months
    match _m.to_lowercase().as_str() {
        "january" | "jan" => "1".to_string(),
        "february" | "feb" => "2".to_string(),
        "march" | "mar" => "3".to_string(),
        "april" | "apr" => "4".to_string(),
        "may" => "5".to_string(),
        "june" | "jun" => "6".to_string(),
        "july" | "jul" => "7".to_string(),
        "august" | "aug" => "8".to_string(),
        "september" | "sep" => "9".to_string(),
        "october" | "oct" => "10".to_string(),
        "november" | "nov" => "11".to_string(),
        "december" | "dec" => "12".to_string(),
        _ => "0".to_string(),
    }
}

fn main() {
    // Allow overriding HTML path via env var DEBUG_HTML_PATH; default to temp dir saved file
    let html_path = env::var("DEBUG_HTML_PATH").unwrap_or_else(|_| {
        let temp_path = std::env::temp_dir().join("stampworld_can_1848_1910_page1.html");
        temp_path.to_string_lossy().to_string()
    });
    println!("[debug] Using HTML path: {}", html_path);

    let fetcher = LocalFetcher::new_from_path(&html_path);

    // Use a temporary images directory so we don't write into Albums during debug
    let images_dir_path = std::env::temp_dir().join("stampworld_debug_images/Canada/images");
    let images_dir = images_dir_path.to_string_lossy().to_string();
    let parent = images_dir_path.parent().unwrap();
    if !parent.exists() {
        let _ = fs::create_dir_all(parent);
    }

    // Simple get_image stub that doesn't download images and returns non-existent
    let get_image = |url: &str, sg: &str, _overwrite: bool| -> ImageStatus {
        println!("[debug] get_image called for sg={} url={}", sg, url);
        ImageStatus::new(url, false, 0, 0, "")
    };

    // Sequencer required by API
    let mut seq: i32 = -1;

    // Build base_url for Canada 1848-1910 (same as modes.rs does)
    let base_url = "https://www.stampworld.com/en/stamps/Canada/Postage%20stamps/1848-1910?page=";

    println!(
        "[debug] Starting collector (debug mode). base_url={}",
        base_url
    );

    let pages = stamp_album_gen2::stampworld::data_collector::collect_stampworld_data_with_fetcher(
        &fetcher,
        &mut seq,
        "_",
        "Canada",
        &images_dir,
        &month_number,
        &get_image,
        base_url,
        1848,
        1910,
        false,
        Some(&|p: usize| println!("[debug] page callback: page={}", p)),
        1, // limit to 1 page for quick debug
        3,
    );

    println!("[debug] Collector returned {} STAMP lines", pages.len());
    for (i, line) in pages.iter().enumerate().take(20) {
        println!("[debug] line {}: {}", i + 1, line);
    }

    println!(
        "[debug] Finished debug run. Temporary DB placed under {} if inserts occurred.",
        std::env::temp_dir()
            .join("stampworld_debug_images")
            .display()
    );
}
