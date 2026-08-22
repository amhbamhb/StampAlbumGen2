---
type: Rust Function
title: update_image_dimensions
resource: src/db_utils_wrapper.rs#L211-L224
generated:
  by: okf-rs/0.4.0
---

# Signature

`pub fn update_image_dimensions( conn: &rusqlite::Connection, country: &str, series: &str, sg_number: &str, width: Option<i32>, height: Option<i32>, ) -> rusqlite::Result<usize>`