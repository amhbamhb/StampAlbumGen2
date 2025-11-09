pub mod constants;
pub mod generator;
pub mod globals;
pub mod helpers;
pub mod page_functions;
pub mod structs;

// Re-export main types and functions for convenience
pub use constants::MONTH_NAMES;
pub use generator::{generate_album_output, generate_album_output_conn};
pub use globals::{compute_usable_height, write_globals};
pub use helpers::{build_page_text_template, emit_row_start_es};
pub use page_functions::{add_stamp, new_page, new_row, new_series, new_year};
pub use structs::{GenericAttrs, Stamp};
