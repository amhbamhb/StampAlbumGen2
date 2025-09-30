use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::io::{self};

use reqwest::blocking::Client;

/// Small local flag DB: map lowercase country name -> ISO alpha-2 code
/// We'll initialize it with a compact set and allow loading from a JSON file
/// under `flags/flag_codes.json` to allow updates.

pub fn default_flag_map() -> HashMap<String, String> {
    // Minimal initial set; we'll try to be forgiving with lookups
    let mut m = HashMap::new();
    m.insert("united states".to_string(), "us".to_string());
    m.insert("usa".to_string(), "us".to_string());
    m.insert("canada".to_string(), "ca".to_string());
    m.insert("great britain".to_string(), "gb".to_string());
    m.insert("united kingdom".to_string(), "gb".to_string());
    m.insert("israel".to_string(), "il".to_string());
    m.insert("australia".to_string(), "au".to_string());
    m.insert("germany".to_string(), "de".to_string());
    m.insert("france".to_string(), "fr".to_string());
    // ... add more as needed, but primary approach is to load a more complete list from flags/flag_codes.json
    m
}

/// Attempt to load an override JSON file at flags/flag_codes.json. The file should be a JSON map
/// of "country name" -> "alpha2 code". If not present, fall back to default_flag_map.
pub fn load_flag_map_from_fs() -> HashMap<String, String> {
    let p = Path::new("flags/flag_codes.json");
    if p.exists() {
        if let Ok(contents) = fs::read_to_string(p) {
            if let Ok(j) = serde_json::from_str::<HashMap<String, String>>(&contents) {
                return j.into_iter().map(|(k, v)| (k.to_lowercase(), v.to_lowercase())).collect();
            }
        }
    }
    default_flag_map()
}

/// Given a country name, attempt to return an alpha-2 code (lowercase) using simple normalization.
pub fn country_to_code(country: &str) -> Option<String> {
    let map = load_flag_map_from_fs();
    let key = country.trim().to_lowercase();
    if let Some(code) = map.get(&key) {
        return Some(code.clone());
    }
    // Try simplified remappings (strip punctuation, common words)
    let key2 = key.replace('\'', "").replace('-', " ").replace(',', "");
    if let Some(code) = map.get(&key2) {
        return Some(code.clone());
    }

    // Fallback: some albums are sub-national regions (provinces/states) like "New Brunswick"
    // Provide a small mapping from common region names to their parent country codes so a
    // sensible flag (e.g. Canada) can be used for the cover page when the album is regional.
    let mut region_map: HashMap<String, String> = HashMap::new();
    // Canadian provinces (common names)
    region_map.insert("new brunswick".to_string(), "ca".to_string());
    region_map.insert("british columbia".to_string(), "ca".to_string());
    region_map.insert("nova scotia".to_string(), "ca".to_string());
    region_map.insert("prince edward island".to_string(), "ca".to_string());
    region_map.insert("ontario".to_string(), "ca".to_string());
    region_map.insert("quebec".to_string(), "ca".to_string());
    region_map.insert("manitoba".to_string(), "ca".to_string());
    region_map.insert("saskatchewan".to_string(), "ca".to_string());
    region_map.insert("newfoundland".to_string(), "ca".to_string());

    // US states (examples) - fallback to 'us' for obvious matches
    region_map.insert("alabama".to_string(), "us".to_string());
    region_map.insert("alaska".to_string(), "us".to_string());
    region_map.insert("california".to_string(), "us".to_string());
    region_map.insert("new york".to_string(), "us".to_string());

    // Try lookup in region_map using key and key2
    if let Some(code) = region_map.get(&key) {
        return Some(code.clone());
    }
    if let Some(code) = region_map.get(&key2) {
        return Some(code.clone());
    }

    None
}

/// Ensure flag image exists for the given alpha2 code in flags/, download from FlagCDN if missing.
/// size example: "w80" or "w320". Returns path to downloaded file on success.
pub fn ensure_flag_image(alpha2: &str, size: &str) -> Result<String, Box<dyn std::error::Error>> {
    let alpha = alpha2.to_lowercase();
    let dir = Path::new("flags");
    fs::create_dir_all(dir)?;
    let filename = format!("{}-{}.png", alpha, size);
    let filepath = dir.join(&filename);
    if filepath.exists() {
        return Ok(filepath.to_string_lossy().to_string());
    }

    // Build URL: https://flagcdn.com/[size]/[code].png where size is like w40 (width px)
    let url = format!("https://flagcdn.com/{}/{}.png", size, alpha);

    let client = Client::builder().user_agent("StampAlbumGen2/1.0").build()?;
    let resp = client.get(&url).send()?;
    if !resp.status().is_success() {
        return Err(format!("Failed to fetch {}: status {}", url, resp.status()).into());
    }
    let mut out = fs::File::create(&filepath)?;
    let content = resp.bytes()?;
    io::copy(&mut content.as_ref(), &mut out)?;
    Ok(filepath.to_string_lossy().to_string())
}

/// Convenience: try to find a flag for the provided country name and copy it into the given album images dir
/// If no match, return Err so callers can fallback to placeholder creation.
pub fn copy_flag_for_album(country: &str, album_images_dir: &str, size: &str) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(code) = country_to_code(country) {
        let downloaded = ensure_flag_image(&code, size)?;
        let dest = Path::new(album_images_dir).join("flag_placeholder.png");
        fs::create_dir_all(album_images_dir)?;
        fs::copy(&downloaded, &dest)?;
        Ok(dest.to_string_lossy().to_string())
    } else {
        Err("No country code match".into())
    }
}

// Inline tests moved to `tests/flagdb.rs`.
