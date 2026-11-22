use log::info;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt::Write as FmtWrite;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, RwLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub const REPO_OWNER: &str = "amhbamhb";
pub const REPO_NAME: &str = "StampAlbumGen2";
pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CACHE_TTL_SECS: u64 = 6 * 3600; // 6 hours

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UpdateState {
    Idle,
    Downloading,
    Verifying,
    Replacing,
    Restarting,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProgressStatus {
    pub state: UpdateState,
    pub message: String,
    pub percent: Option<u8>,
    pub downloaded_bytes: u64,
    pub total_bytes: Option<u64>,
    pub error: Option<String>,
}

static UPDATE_PROGRESS: LazyLock<RwLock<UpdateProgressStatus>> = LazyLock::new(|| {
    RwLock::new(UpdateProgressStatus {
        state: UpdateState::Idle,
        message: String::new(),
        percent: None,
        downloaded_bytes: 0,
        total_bytes: None,
        error: None,
    })
});

pub fn get_update_progress() -> UpdateProgressStatus {
    UPDATE_PROGRESS
        .read()
        .map(|g| g.clone())
        .unwrap_or_else(|_| UpdateProgressStatus {
            state: UpdateState::Idle,
            message: String::new(),
            percent: None,
            downloaded_bytes: 0,
            total_bytes: None,
            error: None,
        })
}

pub fn set_update_progress(status: UpdateProgressStatus) {
    if let Ok(mut g) = UPDATE_PROGRESS.write() {
        *g = status;
    }
}

/// Returns the active version of the application, allowing override via STAMP_ALBUM_CURRENT_VERSION for testing.
pub fn current_version() -> String {
    std::env::var("STAMP_ALBUM_CURRENT_VERSION").unwrap_or_else(|_| CURRENT_VERSION.to_string())
}

/// Returns the release API endpoint, allowing override via STAMP_ALBUM_UPDATE_URL for testing.
pub fn release_api_url() -> String {
    std::env::var("STAMP_ALBUM_UPDATE_URL").unwrap_or_else(|_| {
        format!(
            "https://api.github.com/repos/{}/{}/releases/latest",
            REPO_OWNER, REPO_NAME
        )
    })
}

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
        let url = release_api_url();
        let cur_ver = current_version();

        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(15))
            .user_agent(format!(
                "StampAlbumGen2/{} (stampalbumgenerator2.com)",
                cur_ver
            ))
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
        let update_available = is_newer_version(&cur_ver, &clean_tag);

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
            current_version: cur_ver,
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

/// Starts updating the application in a background thread if not already running.
pub fn start_background_update() -> Result<(), String> {
    let progress = get_update_progress();
    if matches!(
        progress.state,
        UpdateState::Downloading
            | UpdateState::Verifying
            | UpdateState::Replacing
            | UpdateState::Restarting
    ) {
        return Ok(());
    }

    set_update_progress(UpdateProgressStatus {
        state: UpdateState::Downloading,
        message: "Initiating update download...".to_string(),
        percent: Some(0),
        downloaded_bytes: 0,
        total_bytes: None,
        error: None,
    });

    std::thread::spawn(move || {
        if let Err(e) = run_update_pipeline(true) {
            log::error!("Background update failed: {}", e);
            set_update_progress(UpdateProgressStatus {
                state: UpdateState::Failed,
                message: format!("Update failed: {e}"),
                percent: None,
                downloaded_bytes: 0,
                total_bytes: None,
                error: Some(e),
            });
        }
    });

    Ok(())
}

/// Executes download, checksum verification, atomic binary replacement, and optionally triggers restart.
pub fn run_update_pipeline(trigger_restart: bool) -> Result<String, String> {
    let update_info = check_for_updates(true)?;

    if !update_info.update_available {
        let msg = format!(
            "Already running the latest version (v{}).",
            current_version()
        );
        set_update_progress(UpdateProgressStatus {
            state: UpdateState::Idle,
            message: msg.clone(),
            percent: Some(100),
            downloaded_bytes: 0,
            total_bytes: None,
            error: None,
        });
        return Ok(msg);
    }

    let download_url = update_info.download_url.as_deref().ok_or_else(|| {
        format!(
            "No matching binary asset ({}) found in release.",
            target_binary_name()
        )
    })?;

    let sha256_url = update_info.sha256_url.as_deref().ok_or_else(|| {
        format!(
            "No SHA256 checksum asset ({}) found in release.",
            target_checksum_name()
        )
    })?;

    let target_version = update_info.latest_version.clone();
    let cur_ver = current_version();
    let dl_url = download_url.to_string();
    let chk_url = sha256_url.to_string();
    let asset_size = update_info.asset_size;

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(180))
        .user_agent(format!("StampAlbumGen2/{} (updater)", cur_ver))
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

    let total_bytes = bin_resp.content_length().or(asset_size);

    set_update_progress(UpdateProgressStatus {
        state: UpdateState::Downloading,
        message: if let Some(tot) = total_bytes {
            format!(
                "Downloading update (0.0 MB / {:.1} MB)...",
                tot as f64 / 1_048_576.0
            )
        } else {
            "Downloading update binary...".to_string()
        },
        percent: Some(0),
        downloaded_bytes: 0,
        total_bytes,
        error: None,
    });

    let mut temp_file = tempfile::Builder::new()
        .prefix("stamp_album_gen2_update_")
        .tempfile()
        .map_err(|e| format!("Failed to create temp file for update: {e}"))?;

    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 65536];
    let mut downloaded_bytes: u64 = 0;
    let mut last_progress_report = Instant::now();

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
        downloaded_bytes += n as u64;

        if last_progress_report.elapsed() >= Duration::from_millis(200) {
            last_progress_report = Instant::now();
            let percent = total_bytes.map(|tot| {
                if tot > 0 {
                    ((downloaded_bytes as f64 / tot as f64) * 100.0).min(99.0) as u8
                } else {
                    0
                }
            });
            let msg = if let Some(tot) = total_bytes {
                format!(
                    "Downloading update: {:.1} MB of {:.1} MB ({}%)",
                    downloaded_bytes as f64 / 1_048_576.0,
                    tot as f64 / 1_048_576.0,
                    percent.unwrap_or(0)
                )
            } else {
                format!(
                    "Downloading update: {:.1} MB downloaded",
                    downloaded_bytes as f64 / 1_048_576.0
                )
            };
            set_update_progress(UpdateProgressStatus {
                state: UpdateState::Downloading,
                message: msg,
                percent,
                downloaded_bytes,
                total_bytes,
                error: None,
            });
        }
    }

    temp_file
        .flush()
        .map_err(|e| format!("Failed flushing update temp file: {e}"))?;

    set_update_progress(UpdateProgressStatus {
        state: UpdateState::Verifying,
        message: "Verifying SHA256 checksum...".to_string(),
        percent: Some(99),
        downloaded_bytes,
        total_bytes,
        error: None,
    });

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

    set_update_progress(UpdateProgressStatus {
        state: UpdateState::Replacing,
        message: "Installing update...".to_string(),
        percent: Some(99),
        downloaded_bytes,
        total_bytes,
        error: None,
    });

    let target_exe = std::env::current_exe().ok();

    self_replace::self_replace(temp_file.path())
        .map_err(|e| format!("Failed to replace executable: {e}"))?;

    info!("Successfully replaced binary with v{}", target_version);

    if trigger_restart {
        set_update_progress(UpdateProgressStatus {
            state: UpdateState::Restarting,
            message: format!("Successfully updated to v{target_version}! Restarting..."),
            percent: Some(100),
            downloaded_bytes,
            total_bytes,
            error: None,
        });

        // Give the UI a moment to read the restarting status
        std::thread::sleep(Duration::from_millis(1500));
        let _ = restart_application_internal(target_exe.as_deref());
    }

    Ok(format!("Successfully updated to v{target_version}"))
}

/// Downloads, checksum-verifies, and atomically self-replaces the running executable synchronously.
pub fn apply_update() -> Result<String, String> {
    run_update_pipeline(false)
}

/// Strips Windows verbatim prefix (\\?\ or \\?\UNC\) so that paths are compatible with cmd.exe and start.
pub fn strip_verbatim_path_prefix(path: &str) -> String {
    if let Some(stripped) = path.strip_prefix(r"\\?\UNC\") {
        format!(r"\\{}", stripped)
    } else if let Some(stripped) = path.strip_prefix(r"\\?\") {
        stripped.to_string()
    } else {
        path.to_string()
    }
}

/// If the executable path was captured after self-replace relocated it to a temporary
/// name (e.g. `.<hint>.<hash>.__relocated__.exe`), restores the canonical executable path.
pub fn sanitize_executable_path(path: &Path) -> PathBuf {
    let file_name = match path.file_name().and_then(|f| f.to_str()) {
        Some(name) => name,
        None => return path.to_path_buf(),
    };

    if file_name.ends_with(".__relocated__.exe")
        || file_name.ends_with(".__temp__.exe")
        || file_name.ends_with(".__selfdelete__.exe")
    {
        let trimmed = file_name.trim_start_matches('.');
        if let Some(hint) = trimmed.split('.').next().filter(|s| !s.is_empty()) {
            let canonical_name = format!("{hint}.exe");
            if let Some(parent) = path.parent() {
                return parent.join(canonical_name);
            }
        }
    }
    path.to_path_buf()
}

/// Determines the restart arguments to pass to the spawned process, ensuring that
/// if the process was running in Review UI mode by default (without explicit CLI flags),
/// `--review-ui` is explicitly preserved on restart even without a console or GUI trigger.
pub fn build_restart_arguments<I, S>(raw_args: I) -> Vec<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut args: Vec<String> = raw_args
        .into_iter()
        .map(|s| s.as_ref().to_string())
        .collect();

    if !args.iter().any(|a| a == "--review-ui" || a == "-r") {
        let has_non_review_command = args.iter().any(|a| {
            a == "--help"
                || a == "-h"
                || a == "--version"
                || a == "-V"
                || a == "--update"
                || a == "--check-update"
                || a == "--source"
                || a == "-s"
                || a == "--init-config"
                || a == "--force-init-config"
                || a == "--install-desktop-launcher"
                || a == "--uninstall-desktop-launcher"
        });
        if !has_non_review_command {
            args.push("--review-ui".to_string());
        }
    }

    args
}

/// Builds the batch script content used on Windows to safely detach and restart the application.
pub fn build_windows_restart_script(exe_path: &Path, args: &[String]) -> String {
    let exe_str = strip_verbatim_path_prefix(&exe_path.to_string_lossy());
    let mut exe_cmd = format!("start \"\" \"{}\"", exe_str);
    for arg in args {
        let sanitized: String = arg
            .chars()
            .filter(|c| !matches!(c, '"' | '&' | '|' | '<' | '>' | '^' | '%' | '\r' | '\n'))
            .collect();
        if sanitized.contains(' ') {
            exe_cmd.push_str(&format!(" \"{}\"", sanitized));
        } else {
            exe_cmd.push_str(&format!(" {}", sanitized));
        }
    }
    format!(
        "@echo off\r\nping 127.0.0.1 -n 3 >nul\r\n{}\r\n(goto) 2>nul & del \"%~f0\"\r\n",
        exe_cmd
    )
}

/// Spawns the updated executable and exits the current process cleanly with socket detachment.
pub fn restart_application() -> Result<(), String> {
    restart_application_internal(None)
}

pub(crate) fn restart_application_internal(target_exe: Option<&Path>) -> Result<(), String> {
    let current_exe = match target_exe {
        Some(path) => sanitize_executable_path(path),
        None => {
            let exe = std::env::current_exe()
                .map_err(|e| format!("Could not determine current executable path: {e}"))?;
            sanitize_executable_path(&exe)
        }
    };

    let raw_args: Vec<String> = std::env::args_os()
        .skip(1)
        .map(|a| a.to_string_lossy().to_string())
        .collect();
    let args = build_restart_arguments(raw_args);

    info!(
        "Scheduling application restart for {:?} with args {:?}",
        current_exe, args
    );

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        let script_content = build_windows_restart_script(&current_exe, &args);
        let temp_bat = std::env::temp_dir().join(format!(
            "stampalbum_restart_{}_{}.bat",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis())
                .unwrap_or(0)
        ));

        if std::fs::write(&temp_bat, script_content).is_ok() {
            let _ = std::process::Command::new("cmd")
                .args(["/c", &temp_bat.to_string_lossy()])
                .creation_flags(0x08000000) // CREATE_NO_WINDOW
                .spawn();
        } else {
            // Fallback: PowerShell command if batch file writing failed
            let exe_str = strip_verbatim_path_prefix(&current_exe.to_string_lossy());
            let mut ps_cmd = format!(
                "Start-Sleep -Seconds 2; Start-Process -FilePath '{}'",
                exe_str.replace('\'', "''")
            );
            if !args.is_empty() {
                let arg_items: Vec<String> = args
                    .iter()
                    .map(|a| format!("'{}'", a.replace('\'', "''")))
                    .collect();
                ps_cmd.push_str(&format!(" -ArgumentList @({})", arg_items.join(",")));
            }
            let _ = std::process::Command::new("powershell")
                .args([
                    "-NoProfile",
                    "-NonInteractive",
                    "-WindowStyle",
                    "Hidden",
                    "-Command",
                    &ps_cmd,
                ])
                .creation_flags(0x08000000)
                .spawn();
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let exe_str = current_exe.to_string_lossy().to_string();
        let mut sh_script = "sleep 2; exec \"$0\"".to_string();
        let mut sh_args = vec![exe_str];
        for (i, arg) in args.into_iter().enumerate() {
            sh_script.push_str(&format!(" \"${}\"", i + 1));
            sh_args.push(arg);
        }
        let _ = std::process::Command::new("sh")
            .arg("-c")
            .arg(&sh_script)
            .args(&sh_args)
            .spawn();
    }

    // Clean exit of the existing process so ports and files are freed before child launches
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

    #[test]
    fn test_strip_verbatim_path_prefix() {
        assert_eq!(
            strip_verbatim_path_prefix(r"\\?\C:\Users\ray1\repo\app.exe"),
            r"C:\Users\ray1\repo\app.exe"
        );
        assert_eq!(
            strip_verbatim_path_prefix(r"\\?\UNC\server\share\app.exe"),
            r"\\server\share\app.exe"
        );
        assert_eq!(
            strip_verbatim_path_prefix(r"C:\Users\ray1\repo\app.exe"),
            r"C:\Users\ray1\repo\app.exe"
        );
        assert_eq!(
            strip_verbatim_path_prefix("/usr/local/bin/stamp_album_gen2"),
            "/usr/local/bin/stamp_album_gen2"
        );
    }

    #[test]
    fn test_build_restart_arguments_defaults_to_review_ui() {
        let args: Vec<String> = vec![];
        let res = build_restart_arguments(args);
        assert_eq!(res, vec!["--review-ui".to_string()]);

        let res_existing = build_restart_arguments(vec!["--review-ui".to_string()]);
        assert_eq!(res_existing, vec!["--review-ui".to_string()]);

        let res_short = build_restart_arguments(vec!["-r".to_string()]);
        assert_eq!(res_short, vec!["-r".to_string()]);
    }

    #[test]
    fn test_build_restart_arguments_preserves_scraping_and_cli() {
        let res = build_restart_arguments(vec![
            "--source".to_string(),
            "stampworld".to_string(),
            "--album-name".to_string(),
            "Malta".to_string(),
        ]);
        assert_eq!(
            res,
            vec![
                "--source".to_string(),
                "stampworld".to_string(),
                "--album-name".to_string(),
                "Malta".to_string(),
            ]
        );

        let res_help = build_restart_arguments(vec!["--help".to_string()]);
        assert_eq!(res_help, vec!["--help".to_string()]);

        let res_update = build_restart_arguments(vec!["--update".to_string()]);
        assert_eq!(res_update, vec!["--update".to_string()]);
    }

    #[test]
    fn test_build_restart_arguments_preserves_custom_db() {
        let res = build_restart_arguments(vec![
            "--db-path".to_string(),
            "Albums/Malta/Malta.db".to_string(),
        ]);
        assert_eq!(
            res,
            vec![
                "--db-path".to_string(),
                "Albums/Malta/Malta.db".to_string(),
                "--review-ui".to_string(),
            ]
        );
    }

    #[test]
    fn test_build_windows_restart_script() {
        let exe = Path::new(r"C:\Program Files\StampAlbum\stamp_album_gen2.exe");
        let args = vec![
            "--review-ui".to_string(),
            "Albums/My Album/My Album.db".to_string(),
        ];
        let script = build_windows_restart_script(exe, &args);

        assert!(script.starts_with("@echo off\r\nping 127.0.0.1 -n 3 >nul\r\n"));
        assert!(script.contains(
            r#"start "" "C:\Program Files\StampAlbum\stamp_album_gen2.exe" --review-ui "Albums/My Album/My Album.db""#
        ));
        assert!(script.ends_with("(goto) 2>nul & del \"%~f0\"\r\n"));
        // Ensure no escaped quotes like \"\" exist
        assert!(!script.contains(r#"\""#));
    }

    #[test]
    fn test_sanitize_executable_path() {
        let normal = Path::new(r"C:\Apps\StampAlbum\stamp_album_gen2.exe");
        assert_eq!(sanitize_executable_path(normal), normal);

        let relocated = Path::new(
            r"C:\Apps\StampAlbum\.stamp_album_gen2.0123456789abcdef0123456789abcdef.__relocated__.exe",
        );
        assert_eq!(
            sanitize_executable_path(relocated),
            PathBuf::from(r"C:\Apps\StampAlbum\stamp_album_gen2.exe")
        );

        let temp = Path::new(
            r"C:\Apps\StampAlbum\.stamp_album_gen2.0123456789abcdef0123456789abcdef.__temp__.exe",
        );
        assert_eq!(
            sanitize_executable_path(temp),
            PathBuf::from(r"C:\Apps\StampAlbum\stamp_album_gen2.exe")
        );
    }
}
