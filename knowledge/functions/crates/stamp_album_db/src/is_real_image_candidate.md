---
type: Rust Function
title: is_real_image_candidate
resource: crates/stamp_album_db/src/lib.rs#L574-L577
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/crates/stamp_album_db/src/insert_stamp_from_struct
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_real_image_candidate(path: &str) -> bool`

# Called by

- [insert_stamp_from_struct](../../../../functions/crates/stamp_album_db/src/insert_stamp_from_struct.md)