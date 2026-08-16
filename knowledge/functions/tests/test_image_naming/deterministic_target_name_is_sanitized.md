---
type: Rust Function
title: deterministic_target_name_is_sanitized
resource: tests/test_image_naming.rs#L7-L29
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_utils/text_utils/sanitize_filename
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn deterministic_target_name_is_sanitized()`

# Calls

- [sanitize_filename](../../../functions/src/album_utils/text_utils/sanitize_filename.md)