---
type: Rust Function
title: is_missing
resource: src/flagdb/assets.rs#L216-L220
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/flagdb/assets/ensure_flag
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/assets/local_album_flag
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_missing(error: &(dyn std::error::Error + 'static)) -> bool`

# Called by

- [ensure_flag](../../../../functions/src/flagdb/assets/ensure_flag.md)
- [local_album_flag](../../../../functions/src/flagdb/assets/local_album_flag.md)