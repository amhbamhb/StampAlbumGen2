use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::stampworld::fetch::{Fetcher, ReqwestFetcher};

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
    // Try a few candidate locations for flags/flag_codes.json to be tolerant
    // when the binary is run from a different working directory. Prefer the
    // current working directory's `flags/` first, then the executable's
    // parent directory (useful when running from a dist directory that sits
    // beside the repo). If none found, fall back to the built-in map.
    let mut candidates: Vec<PathBuf> = Vec::new();
    // Environment override (highest priority)
    if let Ok(p) = std::env::var("STAMP_FLAGS_DIR") {
        candidates.push(PathBuf::from(p).join("flag_codes.json"));
    }
    // Current working directory
    candidates.push(PathBuf::from("flags/flag_codes.json"));
    // Executable parent directory
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            candidates.push(parent.join("flags/flag_codes.json"));
        }
    }
    // Parent of current working dir (common when running from a dist dir)
    if let Ok(cwd) = std::env::current_dir() {
        if let Some(parent) = cwd.parent() {
            candidates.push(parent.join("flags/flag_codes.json"));
        }
    }
    // At build time, embed the manifest dir so distributed binaries can still
    // look up the repo's flags/ directory if the source tree is present.
    candidates.push(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("flags/flag_codes.json"));

    for p in candidates.iter() {
        if p.exists() {
            if let Ok(contents) = fs::read_to_string(p) {
                if let Ok(j) = serde_json::from_str::<HashMap<String, String>>(&contents) {
                    return j
                        .into_iter()
                        .map(|(k, v)| (k.to_lowercase(), v.to_lowercase()))
                        .collect();
                }
            }
        }
    }

    // If no file found on disk, fall back to an embedded copy of the
    // flag_codes.json so the binary is usable even when run from a small
    // temporary directory. Also attempt to write the embedded JSON out to a
    // 'flags/flag_codes.json' file so subsequent runs can pick it up.
    const EMBEDDED_FLAGS_JSON: &str = include_str!("../flags/flag_codes.json");
    if let Ok(j) = serde_json::from_str::<HashMap<String, String>>(EMBEDDED_FLAGS_JSON) {
        // Attempt to write the embedded json to the preferred writable location
        // (cwd 'flags' preferred). Ignore write errors but log them at caller
        // level if needed; here we simply try the obvious locations.
        let prefer_dirs: Vec<PathBuf> = vec![
            PathBuf::from("flags"),
            // exe parent
            std::env::current_exe()
                .ok()
                .and_then(|e| e.parent().map(|p| p.to_path_buf()))
                .unwrap_or_else(|| PathBuf::from("flags")),
            // build manifest
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("flags"),
        ];
        // Try writing to the current working directory 'flags' first and
        // overwrite any existing file so tests that set a temporary cwd can
        // reliably observe the write-back. If that fails, try the other
        // preferred dirs and stop when a write succeeds.
        let cwd_flags = PathBuf::from("flags");
        if fs::create_dir_all(&cwd_flags).is_ok() {
            let out_path = cwd_flags.join("flag_codes.json");
            let _ = fs::write(&out_path, EMBEDDED_FLAGS_JSON);
        } else {
            for dir in prefer_dirs.iter() {
                if fs::create_dir_all(dir).is_ok() {
                    let out_path = dir.join("flag_codes.json");
                    // Attempt to write (overwrite) and stop on success
                    if fs::write(&out_path, EMBEDDED_FLAGS_JSON).is_ok() {
                        break;
                    }
                }
            }
        }

        return j
            .into_iter()
            .map(|(k, v)| (k.to_lowercase(), v.to_lowercase()))
            .collect();
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
    // Backwards-compatible wrapper that uses the default ReqwestFetcher.
    let fetcher = ReqwestFetcher::new();
    ensure_flag_image_with_fetcher(alpha2, size, &fetcher)
}

/// Ensure flag image exists for the given alpha2 code in flags/, download from FlagCDN if missing.
/// size example: "w80" or "w320". Returns path to downloaded file on success.
pub fn ensure_flag_image_with_fetcher(
    alpha2: &str,
    size: &str,
    fetcher: &dyn Fetcher,
) -> Result<String, Box<dyn std::error::Error>> {
    let alpha = alpha2.to_lowercase();
    let filename = format!("{}-{}.png", alpha, size);

    // Candidate directories to look for an existing flags image. Check the
    // current working directory first, then the executable parent, then the
    // parent of the cwd (useful in some layouts).
    let mut candidates: Vec<PathBuf> = Vec::new();
    // Environment override
    if let Ok(p) = std::env::var("STAMP_FLAGS_DIR") {
        candidates.push(PathBuf::from(p));
    }
    // Current working directory 'flags'
    candidates.push(PathBuf::from("flags"));
    // Executable parent 'flags'
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            candidates.push(parent.join("flags"));
        }
    }
    // Parent of cwd 'flags'
    if let Ok(cwd) = std::env::current_dir() {
        if let Some(parent) = cwd.parent() {
            candidates.push(parent.join("flags"));
        }
    }
    // Build-time manifest 'flags' (source tree)
    candidates.push(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("flags"));

    // If we already have the image in any candidate dir, return its path.
    for dir in candidates.iter() {
        let filepath = dir.join(&filename);
        if filepath.exists() {
            return Ok(filepath.to_string_lossy().to_string());
        }
    }

    // Not found locally: pick a writeable target directory to store downloaded flags.
    // Preference order: STAMP_FLAGS_DIR env, cwd 'flags', exe parent 'flags', build manifest 'flags'.
    let mut target_dir = PathBuf::from("flags");
    if let Ok(p) = std::env::var("STAMP_FLAGS_DIR") {
        let cand = PathBuf::from(p);
        if fs::create_dir_all(&cand).is_ok() {
            target_dir = cand;
        }
    }
    if !target_dir.exists() {
        if fs::create_dir_all(&target_dir).is_err() {
            if let Ok(exe) = std::env::current_exe() {
                if let Some(parent) = exe.parent() {
                    let alt = parent.join("flags");
                    if fs::create_dir_all(&alt).is_ok() {
                        target_dir = alt;
                    }
                }
            }
        }
    }

    let filepath = target_dir.join(&filename);

    // Build URL: https://flagcdn.com/[size]/[code].png where size is like w40 (width px)
    let url = format!("https://flagcdn.com/{}/{}.png", size, alpha);

    // Use the injected fetcher to retrieve the bytes. The Fetcher implementation
    // is responsible for user-agent, timeouts and retry policy.
    let (_status, _headers, content_bytes) = fetcher.fetch_bytes(&url)?;
    let mut out = fs::File::create(&filepath)?;
    // Write the downloaded bytes directly to the file
    out.write_all(&content_bytes)?;
    Ok(filepath.to_string_lossy().to_string())
}

/// Convenience: try to find a flag for the provided country name and copy it into the given album images dir
/// If no match, return Err so callers can fallback to placeholder creation.
pub fn copy_flag_for_album(
    country: &str,
    album_images_dir: &str,
    size: &str,
) -> Result<String, Box<dyn std::error::Error>> {
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
