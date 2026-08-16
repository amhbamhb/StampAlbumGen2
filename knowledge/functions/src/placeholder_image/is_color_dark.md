---
type: Rust Function
title: is_color_dark
resource: src/placeholder_image.rs#L37-L40
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/placeholder_image/luminance
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/placeholder_image/contrasting_color
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/placeholder_image/generate_placeholder_for_stamp_with_bg
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/placeholder_image/ensure_placeholder_image_for_stamp_with_hint
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/placeholder_image_contrast/test_luminance_and_is_color_dark
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_color_dark(color: Rgba<u8>) -> bool`

# Calls

- [luminance](../../../functions/src/placeholder_image/luminance.md)

# Called by

- [contrasting_color](../../../functions/src/placeholder_image/contrasting_color.md)
- [generate_placeholder_for_stamp_with_bg](../../../functions/src/placeholder_image/generate_placeholder_for_stamp_with_bg.md)
- [ensure_placeholder_image_for_stamp_with_hint](../../../functions/src/placeholder_image/ensure_placeholder_image_for_stamp_with_hint.md)
- [test_luminance_and_is_color_dark](../../../functions/tests/placeholder_image_contrast/test_luminance_and_is_color_dark.md)