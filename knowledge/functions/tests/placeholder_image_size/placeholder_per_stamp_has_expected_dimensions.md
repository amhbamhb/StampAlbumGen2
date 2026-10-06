---
type: Rust Function
title: placeholder_per_stamp_has_expected_dimensions
resource: tests/placeholder_image_size.rs#L4-L24
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/placeholder_image/generate_placeholder_image_with_options
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn placeholder_per_stamp_has_expected_dimensions()`

# Calls

- [generate_placeholder_image_with_options](../../../functions/src/placeholder_image/generate_placeholder_image_with_options.md)
- [open](../../../functions/src/external_album/open.md)