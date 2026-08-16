//! Sorting utilities for database operations.
//!
//! This module contains functions for sorting stamp data structures.

use crate::db_utils::StampStub;

/// Sorts a vector of StampStub in place.
///
/// This function delegates to the stamp_album_db crate for the actual sorting logic.
pub fn sort_stampstubs_inplace(v: &mut [StampStub]) {
    stamp_album_db::sort_stampstubs_inplace(v)
}
