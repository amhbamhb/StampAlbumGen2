---
type: Rust Function
title: rasterize
resource: src/flagdb/assets.rs#L67-L87
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/flagdb/assets/ensure_flag
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn rasterize(bytes: &[u8], size: &str) -> AssetResult<Vec<u8>>`

# Called by

- [ensure_flag](../../../../functions/src/flagdb/assets/ensure_flag.md)