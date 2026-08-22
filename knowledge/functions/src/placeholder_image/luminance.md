---
type: Rust Function
title: luminance
resource: src/placeholder_image.rs#L16-L33
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/placeholder_image/is_color_dark
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/placeholder_image_inversion_spec/spec_placeholder_inverts_for_dark_stamp_background
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn luminance(color: Rgba<u8>) -> f32`

# Called by

- [is_color_dark](../../../functions/src/placeholder_image/is_color_dark.md)
- [spec_placeholder_inverts_for_dark_stamp_background](../../../functions/tests/placeholder_image_inversion_spec/spec_placeholder_inverts_for_dark_stamp_background.md)