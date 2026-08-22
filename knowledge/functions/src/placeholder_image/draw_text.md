---
type: Rust Function
title: draw_text
resource: src/placeholder_image.rs#L272-L373
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/placeholder_image/load_fallback_font
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/placeholder_image/generate_placeholder_for_stamp_with_bg
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/placeholder_image/generate_placeholder_image_with_options
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/placeholder_image/ensure_placeholder_image_for_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/placeholder_image/ensure_placeholder_image_for_stamp_with_hint
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn draw_text( img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, text: &str, center_x: i32, center_y: i32, font_size: f32, color: Rgba<u8>, font_override: Option<&str>, )`

# Calls

- [load_fallback_font](../../../functions/src/placeholder_image/load_fallback_font.md)
- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)

# Called by

- [generate_placeholder_for_stamp_with_bg](../../../functions/src/placeholder_image/generate_placeholder_for_stamp_with_bg.md)
- [generate_placeholder_image_with_options](../../../functions/src/placeholder_image/generate_placeholder_image_with_options.md)
- [ensure_placeholder_image_for_stamp](../../../functions/src/placeholder_image/ensure_placeholder_image_for_stamp.md)
- [ensure_placeholder_image_for_stamp_with_hint](../../../functions/src/placeholder_image/ensure_placeholder_image_for_stamp_with_hint.md)