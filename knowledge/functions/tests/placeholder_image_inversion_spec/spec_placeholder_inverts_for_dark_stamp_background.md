---
type: Rust Function
title: spec_placeholder_inverts_for_dark_stamp_background
resource: tests/placeholder_image_inversion_spec.rs#L11-L59
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/placeholder_image/generate_placeholder_for_stamp_with_bg
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/placeholder_image/luminance
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn spec_placeholder_inverts_for_dark_stamp_background()`

# Calls

- [generate_placeholder_for_stamp_with_bg](../../../functions/src/placeholder_image/generate_placeholder_for_stamp_with_bg.md)
- [luminance](../../../functions/src/placeholder_image/luminance.md)