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
    let status = Command::new(cli_path)
        .arg("-c")
        .arg(output_txt_path)
        .status();
    match status {
        Ok(s) if s.success() => {
            log::info!(
                "AlbumEasy CLI completed successfully for {}",
                output_txt_path.display()
            );
            if expected_pdf_path.exists() {
                Some(expected_pdf_path)
            } else {
                log::warn!(
                    "AlbumEasy CLI reported success for {} but no PDF was found at {}",
                    output_txt_path.display(),
                    expected_pdf_path.display()
                );
                None
            }
        }
        Ok(s) => {
            log::warn!(
                "AlbumEasy CLI exited with status {} for {}",
                s,
                output_txt_path.display()
            );
            None
        }
        Err(e) => {
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
    // Call refactored implementation without moving the caller's callback.
    let path = generator_new::generate_album_output_conn::<F>(
        conn, album_name, None, start_year, end_year,
    );
    if path.exists() {
        if let Ok(meta) = std::fs::metadata(&path) {
            if meta.len() > 0 {
                maybe_generate_album_pdf(&path);
                return path;
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
    maybe_generate_album_pdf(&path);
    path
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
                maybe_generate_album_pdf(&path);
                return path;
            }
        }
    }
    let path =
        generator::generate_album_output(db_path, album_name, page_callback, start_year, end_year);
    maybe_generate_album_pdf(&path);
    path
}
