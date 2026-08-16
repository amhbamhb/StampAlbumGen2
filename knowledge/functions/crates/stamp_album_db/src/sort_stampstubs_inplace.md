---
type: Rust Function
title: sort_stampstubs_inplace
resource: crates/stamp_album_db/src/lib.rs#L324-L356
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/crates/stamp_album_db/src/leading_int
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/crates/stamp_album_db/src/suffix_key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn sort_stampstubs_inplace(v: &mut [StampStub])`

# Calls

- [leading_int](../../../../functions/crates/stamp_album_db/src/leading_int.md)
- [suffix_key](../../../../functions/crates/stamp_album_db/src/suffix_key.md)