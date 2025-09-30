// Light wrapper crate exposing DB helpers previously in `src/db_utils.rs`
pub use rusqlite::{Connection, Result};

mod db_utils_core {
    // We'll include the original source of db_utils.rs here. Keep the module small and re-export types.
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../src/db_utils.rs"));
}

// Re-export commonly used symbols for compatibility
pub use db_utils_core::{get_all_series, get_stamps_by_series, StampStub, sort_stampstubs_inplace, migrate_stamps_table, migrate_fix_block_metadata};

// Provide a convenience init function if the original crate exposed it (init_db)
// The original repo had an `init_db` function elsewhere; if present in the included file it will be available.
pub use db_utils_core::init_db;
