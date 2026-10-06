pub mod commemorative_generator;
pub mod constants;
pub mod generator;
pub mod generator_new;
pub mod globals;
pub mod helpers;
pub mod page_functions;
pub mod selection;
pub mod structs;

// Re-export main types and functions for convenience
pub use constants::MONTH_NAMES;
// Re-export the generator API from `generator_new` so callers use the
// refactored implementation while the original `generator` module remains
// available for reference during the incremental refactor.
// Note: do not re-export `generator_new` symbols directly here; we provide
// safe-forwarding wrapper functions below that prefer the refactored
// implementation but fall back to the original implementation when needed.
pub use globals::{compute_usable_height, write_globals};
pub use helpers::{build_page_text_template, emit_row_start_es};
pub use page_functions::{add_stamp, new_page, new_row, new_series, new_year};
pub use structs::{GenericAttrs, Stamp};

use rusqlite::Connection;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant, SystemTime};

static PDF_DX_LOGGING_ENABLED: AtomicBool = AtomicBool::new(false);

pub fn set_pdf_dx_logging_enabled(enabled: bool) {
    PDF_DX_LOGGING_ENABLED.store(enabled, Ordering::Release);
}

pub fn pdf_dx_logging_enabled() -> bool {
    PDF_DX_LOGGING_ENABLED.load(Ordering::Acquire)
}

fn wait_for_generated_pdf_with_timing(
    expected_pdf_path: &Path,
    previous_modified: Option<SystemTime>,
    timeout: Duration,
    poll_every: Duration,
) -> bool {
    let started = Instant::now();
    loop {
        if let Ok(meta) = std::fs::metadata(expected_pdf_path) {
            if meta.len() > 0 {
                let is_newer = match (previous_modified, meta.modified()) {
                    (Some(prev), Ok(current)) => current > prev,
                    (None, Ok(_)) => true,
                    (_, Err(_)) => true,
                };
                if is_newer {
                    return true;
                }
            }
        }
        let remaining = timeout.saturating_sub(started.elapsed());
        if remaining.is_zero() {
            return false;
        }
        std::thread::sleep(poll_every.min(remaining));
    }
}

fn wait_for_generated_pdf(
    expected_pdf_path: &Path,
    previous_modified: Option<SystemTime>,
    timeout: Duration,
) -> bool {
    wait_for_generated_pdf_with_timing(
        expected_pdf_path,
        previous_modified,
        timeout,
        Duration::from_millis(200),
    )
}

#[derive(Debug, Clone)]
pub struct GeneratedAlbumArtifacts {
    pub output_txt_path: PathBuf,
    pub generated_pdf_path: Option<PathBuf>,
    /// Human-readable reason when PDF generation was attempted but failed.
    pub pdf_failure_reason: Option<String>,
}

fn albumeasy_diagnostics(stdout: &[u8], stderr: &[u8]) -> String {
    [stdout, stderr]
        .into_iter()
        .map(|bytes| String::from_utf8_lossy(bytes).trim().to_string())
        .filter(|text| !text.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn albumeasy_reported_error(diagnostics: &str) -> bool {
    diagnostics.lines().any(|line| {
        let lower = line.trim().to_ascii_lowercase();
        lower.starts_with("error:") || lower.starts_with("error in line ")
    })
}

fn maybe_generate_album_pdf_with_settings(
    output_txt_path: &Path,
    generate_pdf: bool,
    cli_path: Option<&str>,
    pdf_timeout: Duration,
) -> (Option<PathBuf>, Option<String>) {
    if !generate_pdf {
        return (None, None);
    }

    let Some(cli_path) = cli_path else {
        log::warn!(
            "AlbumEasy PDF generation is enabled but 'albumeasy_cli_path' is not set in config"
        );
        return (
            None,
            Some("albumeasy_cli_path is not set in config".to_string()),
        );
    };

    let expected_pdf_path = output_txt_path.with_extension("pdf");
    let previous_modified = std::fs::metadata(&expected_pdf_path)
        .ok()
        .and_then(|m| m.modified().ok());
    if pdf_dx_logging_enabled() {
        log::info!(
            "[pdf-dx] invoking AlbumEasy CLI='{}' output_txt='{}' expected_pdf='{}' previous_pdf_mtime_present={}",
            cli_path,
            output_txt_path.display(),
            expected_pdf_path.display(),
            previous_modified.is_some()
        );
    }

    let started = Instant::now();
    let mut cmd = Command::new(cli_path);
    cmd.arg("-c");

    // Prefer invoking AlbumEasy from the output directory and passing only
    // the TXT file name. This avoids path-space breakage in external wrapper
    // scripts that do not quote positional parameters correctly.
    let txt_arg = if let (Some(parent), Some(file_name)) =
        (output_txt_path.parent(), output_txt_path.file_name())
    {
        cmd.current_dir(parent);
        PathBuf::from(file_name)
    } else {
        output_txt_path.to_path_buf()
    };

    if pdf_dx_logging_enabled() {
        log::info!(
            "[pdf-dx] albumeasy invocation cwd='{}' txt_arg='{}'",
            cmd.get_current_dir()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "<process-cwd>".to_string()),
            txt_arg.display()
        );
    }

    let result = cmd.arg(&txt_arg).output();
    match result {
        Ok(output) => {
            let s = output.status;
            let diagnostics = albumeasy_diagnostics(&output.stdout, &output.stderr);
            if !diagnostics.is_empty() {
                log::info!(
                    "AlbumEasy output for {}:\n{}",
                    output_txt_path.display(),
                    diagnostics
                );
            }
            if !s.success() || albumeasy_reported_error(&diagnostics) {
                let reason = format!(
                    "AlbumEasy rendering failed (status {}) for '{}': {}",
                    s,
                    output_txt_path.display(),
                    if diagnostics.is_empty() {
                        "No diagnostic output was returned."
                    } else {
                        &diagnostics
                    },
                );
                log::warn!("{}", reason);
                return (None, Some(reason));
            }
            if pdf_dx_logging_enabled() {
                log::info!(
                    "[pdf-dx] AlbumEasy CLI succeeded in {} ms for '{}'",
                    started.elapsed().as_millis(),
                    output_txt_path.display()
                );
            }
            log::info!(
                "AlbumEasy CLI completed successfully for {}",
                output_txt_path.display()
            );
            if wait_for_generated_pdf(&expected_pdf_path, previous_modified, pdf_timeout) {
                if pdf_dx_logging_enabled() {
                    let meta = std::fs::metadata(&expected_pdf_path).ok();
                    let bytes = meta.as_ref().map(|m| m.len()).unwrap_or(0);
                    log::info!(
                        "[pdf-dx] generated PDF ready path='{}' size_bytes={} total_elapsed_ms={}",
                        expected_pdf_path.display(),
                        bytes,
                        started.elapsed().as_millis()
                    );
                }
                (Some(expected_pdf_path), None)
            } else {
                if pdf_dx_logging_enabled() {
                    log::warn!(
                        "[pdf-dx] timeout waiting for fresh PDF path='{}' timeout_ms={} total_elapsed_ms={}",
                        expected_pdf_path.display(),
                        pdf_timeout.as_millis(),
                        started.elapsed().as_millis()
                    );
                }
                log::warn!(
                    "AlbumEasy CLI reported success for {} but a newly generated PDF was not observed at {} within timeout",
                    output_txt_path.display(),
                    expected_pdf_path.display()
                );
                let mut reason = format!(
                    "AlbumEasy ran but no PDF appeared at '{}' within {} seconds",
                    expected_pdf_path.display(),
                    pdf_timeout.as_secs()
                );
                if !diagnostics.is_empty() {
                    reason.push_str("\nAlbumEasy output:\n");
                    reason.push_str(&diagnostics);
                }
                (None, Some(reason))
            }
        }
        Err(e) => {
            if pdf_dx_logging_enabled() {
                log::warn!(
                    "[pdf-dx] failed to launch AlbumEasy CLI='{}' elapsed_ms={} output_txt='{}' error='{}'",
                    cli_path,
                    started.elapsed().as_millis(),
                    output_txt_path.display(),
                    e
                );
            }
            log::warn!(
                "Failed to run AlbumEasy CLI '{}' for {}: {}",
                cli_path,
                output_txt_path.display(),
                e
            );
            (
                None,
                Some(format!(
                    "Failed to launch AlbumEasy CLI '{}': {}",
                    cli_path, e
                )),
            )
        }
    }
}

fn generated_artifacts_from_path(
    output_txt_path: PathBuf,
    generate_pdf_override: Option<bool>,
) -> GeneratedAlbumArtifacts {
    let cfg = crate::config::load_effective_config("config.toml");
    let cli_path = cfg.effective_albumeasy_cli_path().map(str::to_owned);
    let generate_pdf = generate_pdf_override.unwrap_or_else(|| cfg.albumeasy_auto_pdf_enabled());
    let pdf_timeout = Duration::from_secs(u64::from(cfg.albumeasy_pdf_timeout_secs()));
    let (generated_pdf_path, pdf_failure_reason) = maybe_generate_album_pdf_with_settings(
        &output_txt_path,
        generate_pdf,
        cli_path.as_deref(),
        pdf_timeout,
    );

    GeneratedAlbumArtifacts {
        output_txt_path,
        generated_pdf_path,
        pdf_failure_reason,
    }
}

// Safe forwarding wrappers: prefer the refactored `generator_new` implementation
// but fall back to the original `generator` implementation if the
// refactored function produced no usable output (this keeps tests and
// callers working while `generator_new` is incrementally completed).
fn imported_artifacts(
    conn: &Connection,
    generate_pdf_override: Option<bool>,
) -> Option<GeneratedAlbumArtifacts> {
    let result = match crate::external_album::is_imported(conn) {
        Ok(false) => return None,
        Ok(true) => crate::external_album::generate_artifacts(conn, generate_pdf_override),
        Err(error) => Err(error),
    };
    Some(result.unwrap_or_else(|error| {
        log::error!("Saved AlbumEasy layout generation failed: {error}");
        GeneratedAlbumArtifacts {
            output_txt_path: PathBuf::new(),
            generated_pdf_path: None,
            pdf_failure_reason: Some(error),
        }
    }))
}

pub async fn generate_album_output_conn<F>(
    conn: &Connection,
    album_name: &str,
    page_callback: Option<F>,
    start_year: Option<i32>,
    end_year: Option<i32>,
) -> std::path::PathBuf
where
    F: FnMut(usize),
{
    generate_album_output_conn_with_pdf_override(
        conn,
        album_name,
        page_callback,
        start_year,
        end_year,
        None,
    )
    .await
    .output_txt_path
}

pub async fn generate_album_output_conn_with_pdf_override<F>(
    conn: &Connection,
    album_name: &str,
    page_callback: Option<F>,
    start_year: Option<i32>,
    end_year: Option<i32>,
    generate_pdf_override: Option<bool>,
) -> GeneratedAlbumArtifacts
where
    F: FnMut(usize),
{
    if let Some(artifacts) = imported_artifacts(conn, generate_pdf_override) {
        return artifacts;
    }
    // The refactored implementation is still a stub; preserve the callback
    // for the original implementation until the refactor can generate output.
    let path = generator_new::generate_album_output_conn::<F>(
        conn, album_name, None, start_year, end_year,
    );

    if path.exists() {
        if let Ok(meta) = std::fs::metadata(&path) {
            if meta.len() > 0 {
                return generated_artifacts_from_path(path, generate_pdf_override);
            }
        }
    }

    // Fallback to original implementation
    let path = generator::generate_album_output_conn(
        conn,
        album_name,
        page_callback,
        start_year,
        end_year,
    )
    .await;
    generated_artifacts_from_path(path, generate_pdf_override)
}

/// Generate a Review PDF selection, rejecting empty results before any output writes.
/// Unfiltered callers retain the existing generator forwarding behavior.
pub async fn generate_album_output_conn_with_filters(
    conn: &Connection,
    album_name: &str,
    start_year: Option<i32>,
    end_year: Option<i32>,
    generate_pdf_override: Option<bool>,
    filters: &selection::ColnectPdfFilters,
) -> Result<GeneratedAlbumArtifacts, String> {
    filters.validate()?;
    if crate::external_album::is_imported(conn)? {
        if !filters.is_all() {
            return Err("Catalogue filters cannot be applied to saved AlbumEasy layouts".into());
        }
        return crate::external_album::generate_artifacts(conn, generate_pdf_override);
    }
    if filters.is_all() {
        return Ok(generate_album_output_conn_with_pdf_override(
            conn,
            album_name,
            None::<fn(usize)>,
            start_year,
            end_year,
            generate_pdf_override,
        )
        .await);
    }
    let prepared = selection::prepare_pdf_stamps(conn, filters, start_year, end_year)?;
    Ok(generate_album_output_conn_with_prepared_pdf(
        conn,
        album_name,
        start_year,
        end_year,
        generate_pdf_override,
        prepared,
    )
    .await)
}

pub(crate) async fn generate_album_output_conn_with_prepared_pdf(
    conn: &Connection,
    album_name: &str,
    start_year: Option<i32>,
    end_year: Option<i32>,
    generate_pdf_override: Option<bool>,
    prepared: selection::PreparedPdfStamps,
) -> GeneratedAlbumArtifacts {
    if let Some(artifacts) = imported_artifacts(conn, generate_pdf_override) {
        return artifacts;
    }
    let path = generator::generate_album_output_conn_prepared(
        conn,
        album_name,
        None::<fn(usize)>,
        start_year,
        end_year,
        Some(prepared),
    )
    .await;
    generated_artifacts_from_path(path, generate_pdf_override)
}

pub async fn generate_album_output<F>(
    db_path: &str,
    album_name: &str,
    page_callback: Option<F>,
    start_year: Option<i32>,
    end_year: Option<i32>,
) -> std::path::PathBuf
where
    F: FnMut(usize),
{
    if let Ok(conn) =
        Connection::open_with_flags(db_path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)
    {
        if let Some(artifacts) = imported_artifacts(&conn, None) {
            return artifacts.output_txt_path;
        }
    }
    // The refactored implementation is still a stub; preserve the callback
    // for the original implementation until the refactor can generate output.
    let path =
        generator_new::generate_album_output::<F>(db_path, album_name, None, start_year, end_year);
    if path.exists() {
        if let Ok(meta) = std::fs::metadata(&path) {
            if meta.len() > 0 {
                return generated_artifacts_from_path(path, None).output_txt_path;
            }
        }
    }
    let path =
        generator::generate_album_output(db_path, album_name, page_callback, start_year, end_year)
            .await;
    generated_artifacts_from_path(path, None).output_txt_path
}

#[cfg(test)]
mod tests {
    use super::wait_for_generated_pdf_with_timing;
    use std::fs;
    use std::time::{Duration, SystemTime};

    #[test]
    fn albumeasy_errors_are_detected_in_both_output_streams() {
        let output = super::albumeasy_diagnostics(
            b"Generating the Album ...\n",
            b"Error in line 74: images/coat_of_arms.png was not found.\n",
        );
        assert!(output.contains("Generating the Album"));
        assert!(super::albumeasy_reported_error(&output));
        assert!(super::albumeasy_reported_error(" ERROR: Invalid album"));
        assert!(!super::albumeasy_reported_error("Generated successfully"));
    }

    #[cfg(unix)]
    #[test]
    fn albumeasy_success_exit_with_render_error_fails_without_pdf_timeout() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().expect("renderer test directory");
        let launcher = dir.path().join("AlbumEasy test.sh");
        fs::write(
            &launcher,
            include_str!("../../tests/fixtures/albumeasy_render_error.sh"),
        )
        .expect("write test launcher");
        fs::set_permissions(&launcher, fs::Permissions::from_mode(0o755))
            .expect("launcher permissions");
        let txt = dir.path().join("album.txt");
        fs::write(&txt, b"").expect("test album");
        let started = std::time::Instant::now();
        let (pdf, failure) = super::maybe_generate_album_pdf_with_settings(
            &txt,
            true,
            launcher.to_str(),
            Duration::from_secs(30),
        );
        assert!(pdf.is_none());
        let reason = failure.expect("render error");
        assert!(reason.contains("Error in line 74"));
        assert!(reason.contains("images/coat_of_arms.png"));
        assert!(started.elapsed() < Duration::from_secs(5));
        assert!(!reason.contains("within 30 seconds"));
    }

    #[cfg(unix)]
    #[test]
    fn albumeasy_captured_output_preserves_successful_pdf_generation() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().expect("renderer test directory");
        let launcher = dir.path().join("AlbumEasy success.sh");
        fs::write(
            &launcher,
            include_str!("../../tests/fixtures/albumeasy_render_success.sh"),
        )
        .expect("write test launcher");
        fs::set_permissions(&launcher, fs::Permissions::from_mode(0o755))
            .expect("launcher permissions");
        let txt = dir.path().join("album with spaces.txt");
        fs::write(&txt, b"").expect("test album");
        let (pdf, failure) = super::maybe_generate_album_pdf_with_settings(
            &txt,
            true,
            launcher.to_str(),
            Duration::from_secs(1),
        );
        assert_eq!(pdf, Some(txt.with_extension("pdf")));
        assert_eq!(failure, None);
    }

    #[test]
    fn wait_for_generated_pdf_rejects_missing_empty_and_stale_files() {
        let dir = tempfile::tempdir().expect("PDF test directory");
        let path = dir.path().join("album.pdf");
        let timeout = Duration::from_millis(30);
        let poll = Duration::from_millis(100);
        assert!(!wait_for_generated_pdf_with_timing(
            &path, None, timeout, poll
        ));
        fs::write(&path, b"").expect("empty PDF");
        assert!(!wait_for_generated_pdf_with_timing(
            &path, None, timeout, poll
        ));
        fs::write(&path, b"old PDF").expect("stale PDF");
        let modified = fs::metadata(&path)
            .expect("metadata")
            .modified()
            .expect("mtime");
        assert!(!wait_for_generated_pdf_with_timing(
            &path,
            Some(modified),
            timeout,
            poll
        ));
        assert!(wait_for_generated_pdf_with_timing(
            &path, None, timeout, poll
        ));
    }

    #[cfg(unix)]
    #[test]
    fn albumeasy_pdf_timeout_failure_reports_actual_wait() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().expect("launcher test directory");
        let launcher = dir.path().join("AlbumEasy test.sh");
        fs::write(
            &launcher,
            include_str!("../../tests/fixtures/albumeasy_success.sh"),
        )
        .expect("write test launcher");
        fs::set_permissions(&launcher, fs::Permissions::from_mode(0o755))
            .expect("launcher permissions");
        let txt = dir.path().join("album.txt");
        fs::write(&txt, b"").expect("test album");
        let (pdf, failure) = super::maybe_generate_album_pdf_with_settings(
            &txt,
            true,
            launcher.to_str(),
            Duration::from_secs(1),
        );
        assert!(pdf.is_none());
        assert_eq!(
            failure,
            Some(format!(
                "AlbumEasy ran but no PDF appeared at '{}' within 1 seconds",
                txt.with_extension("pdf").display(),
            ))
        );
    }

    #[test]
    fn wait_for_generated_pdf_detects_delayed_new_write() {
        let temp_dir = tempfile::tempdir().expect("tempdir should be created");
        let pdf_path = temp_dir.path().join("album.pdf");

        fs::write(&pdf_path, b"old").expect("seed file should be written");
        let previous_modified = fs::metadata(&pdf_path)
            .ok()
            .and_then(|m| m.modified().ok())
            .or(Some(SystemTime::now()));

        std::thread::sleep(Duration::from_millis(150));
        let delayed_path = pdf_path.clone();
        let writer = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(250));
            fs::write(delayed_path, b"new-content").expect("delayed file write should succeed");
        });

        let observed = wait_for_generated_pdf_with_timing(
            &pdf_path,
            previous_modified,
            Duration::from_secs(2),
            Duration::from_millis(50),
        );

        writer.join().expect("delayed writer thread should finish");
        assert!(
            observed,
            "expected delayed PDF write to be detected within timeout"
        );
    }
}
