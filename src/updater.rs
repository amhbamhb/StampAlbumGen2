use log::info;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt::Write as FmtWrite;
use std::sync::{LazyLock, RwLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub const REPO_OWNER: &str = "amhbamhb";
pub const REPO_NAME: &str = "StampAlbumGen2";
pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CACHE_TTL_SECS: u64 = 6 * 3600; // 6 hours

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateInfo {
    pub current_version: String,
    pub latest_version: String,
    pub update_available: bool,
    pub release_name: String,
    pub release_notes: String,
    pub release_url: String,
    pub published_at: String,
    pub download_url: Option<String>,
    pub sha256_url: Option<String>,
    pub asset_name: Option<String>,
    pub asset_size: Option<u64>,
    pub checked_at_timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
    size: u64,
}

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    name: Option<String>,
    body: Option<String>,
    html_url: String,
    published_at: Option<String>,
    assets: Vec<GitHubAsset>,
    #[allow(dead_code)]
    prerelease: bool,
    draft: bool,
}

static UPDATE_CACHE: LazyLock<RwLock<Option<(Instant, UpdateInfo)>>> =
    LazyLock::new(|| RwLock::new(None));

/// Parse a semver string like "1.15.0" or "v1.15.0" into (major, minor, patch).
pub fn parse_semver(v_str: &str) -> Option<(u64, u64, u64)> {
    let clean = v_str.trim().trim_start_matches('v').trim_start_matches('V');
    let mut parts = clean.split('.');
    let major = parts.next()?.parse::<u64>().ok()?;
    let minor = parts.next()?.parse::<u64>().ok()?;
    // Strip any pre-release suffixes (e.g. "0-beta" -> 0)
    let patch_str = parts.next()?.split(['-', '+']).next()?;
    let patch = patch_str.parse::<u64>().ok()?;
    Some((major, minor, patch))
}

/// Returns true if `remote` is strictly newer than `current`.
pub fn is_newer_version(current: &str, remote: &str) -> bool {
    match (parse_semver(current), parse_semver(remote)) {
        (Some(curr), Some(rem)) => rem > curr,
        _ => false,
    }
}

/// The expected binary asset name for the current runtime platform.
pub fn target_binary_name() -> &'static str {
    if cfg!(target_os = "windows") {
        "stamp_album_gen2.exe"
    } else {
        "stamp_album_gen2"
    }
}

/// The expected SHA256 checksum asset name for the current runtime platform.
pub fn target_checksum_name() -> &'static str {
    if cfg!(target_os = "windows") {
        "stamp_album_gen2.exe.sha256"
    } else {
        "stamp_album_gen2.sha256"
    }
}

/// Checks GitHub Releases for updates. Safe to call from any context (async or sync).
/// Caches successful checks for 6 hours unless `force` is true.
pub fn check_for_updates(force: bool) -> Result<UpdateInfo, String> {
    if !force {
        let cached = UPDATE_CACHE
            .read()
            .ok()
            .and_then(|g| g.clone())
            .filter(|(inst, _)| inst.elapsed() < Duration::from_secs(CACHE_TTL_SECS))
            .map(|(_, info)| info);
        if let Some(info) = cached {
            return Ok(info);
        }
    }

    // Run network I/O on a dedicated thread to ensure we never collide with Tokio runtime.
    let handle = std::thread::spawn(move || -> Result<UpdateInfo, String> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/releases/latest",
            REPO_OWNER, REPO_NAME
        );

        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(15))
            .user_agent(format!("StampAlbumGen2/{} (stampalbumgenerator2.com)", CURRENT_VERSION))
            .build()
            .map_err(|e| format!("Failed to build HTTP client: {e}"))?;

        let resp = client
            .get(&url)
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .map_err(|e| format!("Failed to connect to GitHub releases: {e}"))?;

        if !resp.status().is_success() {
            return Err(format!(
                "GitHub API returned error status: {} {}",
                resp.status().as_u16(),
                resp.status().canonical_reason().unwrap_or("")
            ));
        }

        let release: GitHubRelease = resp
            .json()
            .map_err(|e| format!("Failed to parse GitHub release JSON: {e}"))?;

        if release.draft {
            return Err("Latest release is a draft; skipping.".to_string());
        }

        let clean_tag = release.tag_name.trim_start_matches('v').to_string();
        let update_available = is_newer_version(CURRENT_VERSION, &clean_tag);

        let bin_name = target_binary_name();
        let chk_name = target_checksum_name();

        let mut download_url = None;
        let mut sha256_url = None;
        let mut asset_name = None;
        let mut asset_size = None;

        for asset in release.assets {
            if asset.name.eq_ignore_ascii_case(bin_name) {
                download_url = Some(asset.browser_download_url.clone());
                asset_name = Some(asset.name.clone());
                asset_size = Some(asset.size);
            } else if asset.name.eq_ignore_ascii_case(chk_name) {
                sha256_url = Some(asset.browser_download_url.clone());
            }
        }

        let now_ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let info = UpdateInfo {
            current_version: CURRENT_VERSION.to_string(),
            latest_version: clean_tag,
            update_available,
            release_name: release.name.unwrap_or_else(|| release.tag_name.clone()),
            release_notes: release.body.unwrap_or_default(),
            release_url: release.html_url,
            published_at: release.published_at.unwrap_or_default(),
            download_url,
            sha256_url,
            asset_name,
            asset_size,
            checked_at_timestamp: now_ts,
        };

        Ok(info)
    });

    let info = handle
        .join()
        .map_err(|_| "Update check thread panicked".to_string())??;

    if let Ok(mut guard) = UPDATE_CACHE.write() {
        *guard = Some((Instant::now(), info.clone()));
    }

    Ok(info)
}

/// Downloads, checksum-verifies, and atomically self-replaces the running executable.
pub fn apply_update() -> Result<String, String> {
    let update_info = check_for_updates(true)?;

    if !update_info.update_available {
        return Ok(format!(
            "Already running the latest version (v{}).",
            CURRENT_VERSION
        ));
    }

    let download_url = update_info
        .download_url
        .as_deref()
        .ok_or_else(|| format!("No matching binary asset ({}) found in release.", target_binary_name()))?;

    let sha256_url = update_info
        .sha256_url
        .as_deref()
        .ok_or_else(|| format!("No SHA256 checksum asset ({}) found in release.", target_checksum_name()))?;

    // Perform download and replacement in a worker thread.
    let target_version = update_info.latest_version.clone();
    let dl_url = download_url.to_string();
    let chk_url = sha256_url.to_string();

    let worker = std::thread::spawn(move || -> Result<String, String> {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(180))
            .user_agent(format!("StampAlbumGen2/{} (updater)", CURRENT_VERSION))
            .build()
            .map_err(|e| format!("Failed to create download client: {e}"))?;

        info!("Downloading checksum from {}", chk_url);
        let chk_resp = client
            .get(&chk_url)
            .send()
            .map_err(|e| format!("Failed to download checksum file: {e}"))?;

        let chk_text = chk_resp
            .text()
            .map_err(|e| format!("Failed to read checksum response: {e}"))?;

        let expected_sha256 = chk_text
            .split_whitespace()
            .next()
            .ok_or_else(|| "Empty checksum file".to_string())?
            .to_lowercase();

        if expected_sha256.len() != 64 {
            return Err(format!(
                "Invalid SHA256 format in checksum file (got {} characters, expected 64).",
                expected_sha256.len()
            ));
        }

        info!("Downloading updated binary from {}", dl_url);
        let mut bin_resp = client
            .get(&dl_url)
            .send()
            .map_err(|e| format!("Failed to download update binary: {e}"))?;

        if !bin_resp.status().is_success() {
            return Err(format!("Download failed with HTTP {}", bin_resp.status()));
        }

        // Write downloaded payload to a temporary file while hashing
        let mut temp_file = tempfile::Builder::new()
            .prefix("stamp_album_gen2_update_")
            .tempfile()
            .map_err(|e| format!("Failed to create temp file for update: {e}"))?;

        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 65536];

        use std::io::{Read, Write};
        loop {
            let n = bin_resp
                .read(&mut buffer)
                .map_err(|e| format!("Failed reading download stream: {e}"))?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
            temp_file
                .write_all(&buffer[..n])
                .map_err(|e| format!("Failed writing update to temp file: {e}"))?;
        }

        temp_file
            .flush()
            .map_err(|e| format!("Failed flushing update temp file: {e}"))?;

        let hash_bytes = hasher.finalize();
        let mut computed_sha256 = String::with_capacity(64);
        for b in hash_bytes {
            let _ = FmtWrite::write_fmt(&mut computed_sha256, format_args!("{:02x}", b));
        }

        if computed_sha256 != expected_sha256 {
            return Err(format!(
                "Checksum verification failed! Expected: {}, computed: {}. Aborting update.",
                expected_sha256, computed_sha256
            ));
        }

        info!("Checksum verified successfully ({})", computed_sha256);

        // Perform atomic self replacement
        self_replace::self_replace(temp_file.path())
            .map_err(|e| format!("Failed to replace executable: {e}"))?;

        info!("Successfully replaced binary with v{}", target_version);
        Ok(format!("Successfully updated to v{target_version}"))
    });

    worker
        .join()
        .map_err(|_| "Update worker thread panicked".to_string())?
}

/// Spawns the updated executable and exits the current process.
pub fn restart_application() -> Result<(), String> {
    let current_exe = std::env::current_exe()
        .map_err(|e| format!("Could not determine current executable path: {e}"))?;

    let args: Vec<_> = std::env::args_os().skip(1).collect();

    info!("Spawning restarted process: {:?} with args {:?}", current_exe, args);

    std::process::Command::new(current_exe)
        .args(args)
        .spawn()
        .map_err(|e| format!("Failed to spawn updated application process: {e}"))?;

    // Clean exit
    std::process::exit(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semver_parsing_and_comparison() {
        assert_eq!(parse_semver("1.14.3"), Some((1, 14, 3)));
        assert_eq!(parse_semver("v1.15.0"), Some((1, 15, 0)));
        assert_eq!(parse_semver("v2.0.0-rc1"), Some((2, 0, 0)));
        assert_eq!(parse_semver("invalid"), None);

        assert!(is_newer_version("1.14.3", "1.15.0"));
        assert!(is_newer_version("1.14.3", "v1.14.4"));
        assert!(is_newer_version("1.14.3", "2.0.0"));
        assert!(!is_newer_version("1.15.0", "1.15.0"));
        assert!(!is_newer_version("1.15.0", "1.14.9"));
        assert!(!is_newer_version("2.0.0", "1.99.99"));
    }

    #[test]
    fn target_names_match_platform() {
        let bin = target_binary_name();
        let chk = target_checksum_name();

        if cfg!(target_os = "windows") {
            assert_eq!(bin, "stamp_album_gen2.exe");
            assert_eq!(chk, "stamp_album_gen2.exe.sha256");
        } else {
            assert_eq!(bin, "stamp_album_gen2");
            assert_eq!(chk, "stamp_album_gen2.sha256");
        }
    }
}
