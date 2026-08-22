---
type: Rust Function
title: insert_stamp
resource: crates/stamp_album_db/src/lib.rs#L697-L1015
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn insert_stamp( conn: &Connection, country: &str, series: &str, year: i32, month: &str, _month_num: &str, day: &str, sg_number: &str, type_: &str, denomination: &str, color: &str, description: &str, stamps_issued: &str, mint_condition: &str, unused: &str, used: &str, letter_fdc: &str, currency: &str, image_path: &str, remote_image_url: &str, original_image_url: &str, watermark: &str, design: &str, engraving: &str, perforation: &str, context: &str, ) -> Result<bool>`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)