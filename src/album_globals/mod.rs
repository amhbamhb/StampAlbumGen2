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
    let path = generator_new::generate_album_output_conn(conn, album_name, start_year, end_year);
    if path.exists() {
        if let Ok(meta) = std::fs::metadata(&path) {
            if meta.len() > 0 {
                return path;
            }
        }
    }
    // Fallback to original implementation
    generator::generate_album_output_conn(conn, album_name, page_callback, start_year, end_year)
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
    let path = generator_new::generate_album_output(db_path, album_name, start_year, end_year);
    if path.exists() {
        if let Ok(meta) = std::fs::metadata(&path) {
            if meta.len() > 0 {
                return path;
            }
        }
    }
    generator::generate_album_output(db_path, album_name, page_callback, start_year, end_year)
}
