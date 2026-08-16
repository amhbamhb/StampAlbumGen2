---
type: Rust Function
title: update_image_dimensions
resource: src/db_utils/updates.rs#L8-L18
generated:
  by: okf-rs/0.4.0
---

# Signature

`pub fn update_image_dimensions( conn: &Connection, country: &str, series: &str, sg_number: &str, width: Option<i32>, height: Option<i32>, ) -> Result<usize>`