---
type: Rust Function
title: insert_stamp
resource: src/db_utils/operations.rs#L16-L72
generated:
  by: okf-rs/0.4.0
---

# Signature

`pub fn insert_stamp( conn: &Connection, country: &str, series: &str, year: i32, month: &str, month_num: &str, day: &str, sg_number: &str, type_: &str, denomination: &str, color: &str, description: &str, stamps_issued: &str, mint_condition: &str, unused: &str, used: &str, letter_fdc: &str, currency: &str, image_path: &str, remote_image_url: &str, original_image_url: &str, watermark: &str, design: &str, engraving: &str, perforation: &str, context: &str, ) -> Result<bool>`