// Forward these helpers to the refactor staging area so callers can keep
// using `crate::album_globals::helpers::{...}` while the implementations
// live inside `generator_new` during the refactor.
pub use crate::album_globals::generator_new::helpers::{
    build_page_text_template,
    emit_row_start_es,
};
