pub mod commemorative_generator;
pub mod constants;
pub mod generator;
pub mod generator_new;
pub mod globals;
pub mod helpers;
pub mod page_functions;
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
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
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
        std::thread::sleep(poll_every);
    }
    false
}

fn wait_for_generated_pdf(expected_pdf_path: &Path, previous_modified: Option<SystemTime>) -> bool {
    wait_for_generated_pdf_with_timing(
        expected_pdf_path,
        previous_modified,
        Duration::from_secs(8),
        Duration::from_millis(200),
    )
}

#[derive(Debug, Clone)]
pub struct GeneratedAlbumArtifacts {
    pub output_txt_path: PathBuf,
    pub generated_pdf_path: Option<PathBuf>,
}

fn maybe_generate_album_pdf_with_settings(
    output_txt_path: &Path,
    generate_pdf: bool,
    cli_path: Option<&str>,
) -> Option<PathBuf> {
    if !generate_pdf {
        return None;
    }

    let Some(cli_path) = cli_path else {
        log::warn!(
            "AlbumEasy PDF generation is enabled but 'albumeasy_cli_path' is not set in config"
        );
        return None;
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
    let status = Command::new(cli_path)
        .arg("-c")
        .arg(output_txt_path)
        .status();
    match status {
        Ok(s) if s.success() => {
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
            if wait_for_generated_pdf(&expected_pdf_path, previous_modified) {
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
                Some(expected_pdf_path)
            } else {
                if pdf_dx_logging_enabled() {
                    log::warn!(
                        "[pdf-dx] timeout waiting for fresh PDF path='{}' timeout_ms={} total_elapsed_ms={}",
                        expected_pdf_path.display(),
                        Duration::from_secs(8).as_millis(),
                        started.elapsed().as_millis()
                    );
                }
                log::warn!(
                    "AlbumEasy CLI reported success for {} but a newly generated PDF was not observed at {} within timeout",
                    output_txt_path.display(),
                    expected_pdf_path.display()
                );
                None
            }
        }
        Ok(s) => {
            if pdf_dx_logging_enabled() {
                log::warn!(
                    "[pdf-dx] AlbumEasy CLI failed status='{}' elapsed_ms={} output_txt='{}'",
                    s,
                    started.elapsed().as_millis(),
                    output_txt_path.display()
                );
            }
            log::warn!(
                "AlbumEasy CLI exited with status {} for {}",
                s,
                output_txt_path.display()
            );
            None
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
            None
        }
    }
}

fn generated_artifacts_from_path(
    output_txt_path: PathBuf,
    generate_pdf_override: Option<bool>,
) -> GeneratedAlbumArtifacts {
    let cfg = crate::config::load_config("config.toml");
    let cli_path = cfg.effective_albumeasy_cli_path().map(str::to_owned);
    let generate_pdf = generate_pdf_override.unwrap_or_else(|| cfg.albumeasy_auto_pdf_enabled());
    let generated_pdf_path =
        maybe_generate_album_pdf_with_settings(&output_txt_path, generate_pdf, cli_path.as_deref());

    GeneratedAlbumArtifacts {
        output_txt_path,
        generated_pdf_path,
    }
}

// Safe forwarding wrappers: prefer the refactored `generator_new` implementation
// but fall back to the original `generator` implementation if the
// refactored function produced no usable output (this keeps tests and
// callers working while `generator_new` is incrementally completed).
pub fn generate_album_output_conn<F>(
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
    .output_txt_path
}

pub fn generate_album_output_conn_with_pdf_override<F>(
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
    // Call refactored implementation without moving the caller's callback.
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
    );
    generated_artifacts_from_path(path, generate_pdf_override)
}

pub fn generate_album_output<F>(
    db_path: &str,
    album_name: &str,
    page_callback: Option<F>,
    start_year: Option<i32>,
    end_year: Option<i32>,
) -> std::path::PathBuf
where
    F: FnMut(usize),
{
    // Call refactored implementation without moving the caller's callback.
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
        generator::generate_album_output(db_path, album_name, page_callback, start_year, end_year);
    generated_artifacts_from_path(path, None).output_txt_path
}

#[cfg(test)]
mod tests {
    use super::wait_for_generated_pdf_with_timing;
    use std::fs;
    use std::time::{Duration, SystemTime};

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
