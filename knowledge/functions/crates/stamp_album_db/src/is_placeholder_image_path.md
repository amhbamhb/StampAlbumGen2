---
type: Rust Function
title: is_placeholder_image_path
resource: crates/stamp_album_db/src/lib.rs#L579-L581
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/stamp_image/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/crates/stamp_album_db/src/insert_stamp_from_struct
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_placeholder_image_path(path: &str) -> bool`

# Calls

- [contains](../../../../functions/src/native_pdf/stamp_image/contains.md)

# Called by

- [insert_stamp_from_struct](../../../../functions/crates/stamp_album_db/src/insert_stamp_from_struct.md)