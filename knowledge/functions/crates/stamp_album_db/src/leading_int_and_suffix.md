---
type: Rust Function
title: leading_int_and_suffix
resource: crates/stamp_album_db/src/lib.rs#L505-L532
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/crates/stamp_album_db/src/sort_stampstubs_inplace
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn leading_int_and_suffix(s: &str) -> (Option<i64>, String)`

# Called by

- [sort_stampstubs_inplace](../../../../functions/crates/stamp_album_db/src/sort_stampstubs_inplace.md)