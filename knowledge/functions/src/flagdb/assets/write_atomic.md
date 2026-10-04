---
type: Rust Function
title: write_atomic
resource: src/flagdb/assets.rs#L57-L65
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/flagdb/assets/ensure_flag
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/assets/copy_to_album
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn write_atomic(path: &Path, bytes: &[u8]) -> AssetResult<()>`

# Called by

- [ensure_flag](../../../../functions/src/flagdb/assets/ensure_flag.md)
- [copy_to_album](../../../../functions/src/flagdb/assets/copy_to_album.md)