// Forward these helpers to the refactor staging area so callers can keep
// using `crate::album_globals::helpers::{...}` while the implementations
// live inside `generator_new` during the refactor.
pub use crate::album_globals::generator_new::helpers::{
    build_date_page_text_centre_template, build_engraving_page_text_centre_template,
    build_page_text_template, build_series_page_text_centre_template,
    build_watermark_page_text_centre_template, emit_row_start_es,
};
