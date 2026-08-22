---
type: Rust Function
title: generate_placeholder_image_with_options
resource: src/placeholder_image.rs#L396-L454
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/placeholder_image/draw_text
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/placeholder_image/generate_placeholder_image
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/placeholder_image/ensure_placeholder_image_for_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/placeholder_image/ensure_placeholder_image_for_stamp_with_hint
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/placeholder_image_size/placeholder_per_stamp_has_expected_dimensions
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn generate_placeholder_image_with_options( width: u32, height: u32, output_path: &Path, options: Option<PlaceholderOptions>, ) -> Result<(), Box<dyn std::error::Error>>`

# Calls

- [draw_text](../../../functions/src/placeholder_image/draw_text.md)

# Called by

- [generate_placeholder_image](../../../functions/src/placeholder_image/generate_placeholder_image.md)
- [ensure_placeholder_image_for_album](../../../functions/src/placeholder_image/ensure_placeholder_image_for_album.md)
- [ensure_placeholder_image_for_stamp_with_hint](../../../functions/src/placeholder_image/ensure_placeholder_image_for_stamp_with_hint.md)
- [placeholder_per_stamp_has_expected_dimensions](../../../functions/tests/placeholder_image_size/placeholder_per_stamp_has_expected_dimensions.md)