---
type: Rust Function
title: ensure_flag_image
resource: src/flagdb.rs#L189-L193
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/flagdb/ensure_flag_image_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/flagdb/copy_flag_for_album
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn ensure_flag_image(alpha2: &str, size: &str) -> Result<String, Box<dyn std::error::Error>>`

# Calls

- [ensure_flag_image_with_fetcher](../../../functions/src/flagdb/ensure_flag_image_with_fetcher.md)

# Called by

- [copy_flag_for_album](../../../functions/src/flagdb/copy_flag_for_album.md)