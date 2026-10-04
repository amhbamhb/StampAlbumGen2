---
type: Rust Function
title: failed_http_and_invalid_images_continue_to_flagcdn_without_caching_errors
resource: src/flagdb/assets.rs#L289-L318
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/flagdb/assets/png
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/assets/ensure_flag
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn failed_http_and_invalid_images_continue_to_flagcdn_without_caching_errors()`

# Calls

- [png](../../../../functions/src/flagdb/assets/png.md)
- [ensure_flag](../../../../functions/src/flagdb/assets/ensure_flag.md)