---
type: Rust Function
title: contrasting_color
resource: src/placeholder_image.rs#L45-L51
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/placeholder_image/is_color_dark
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/placeholder_image/generate_placeholder_for_stamp_with_bg
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/placeholder_image_contrast/test_contrasting_color_returns_white_for_dark
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn contrasting_color(color: Rgba<u8>) -> Rgba<u8>`

# Calls

- [is_color_dark](../../../functions/src/placeholder_image/is_color_dark.md)

# Called by

- [generate_placeholder_for_stamp_with_bg](../../../functions/src/placeholder_image/generate_placeholder_for_stamp_with_bg.md)
- [test_contrasting_color_returns_white_for_dark](../../../functions/tests/placeholder_image_contrast/test_contrasting_color_returns_white_for_dark.md)