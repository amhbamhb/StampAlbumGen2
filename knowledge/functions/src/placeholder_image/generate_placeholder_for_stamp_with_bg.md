---
type: Rust Function
title: generate_placeholder_for_stamp_with_bg
resource: src/placeholder_image.rs#L98-L183
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/placeholder_image/compute_placeholder_size_for_series
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/placeholder_image/contrasting_color
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/placeholder_image/is_color_dark
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/placeholder_image/draw_text
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/placeholder_image_inversion_spec/spec_placeholder_inverts_for_dark_stamp_background
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/placeholder_image_series_size/placeholder_series_size_matches_hint
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn generate_placeholder_for_stamp_with_bg<P: AsRef<std::path::Path>>( _album_name: &str, sg_number: &str, year: &str, stamp_bg_color: Option<Rgba<u8>>, series_hint: Option<&str>, out_path: P, ) -> Result<(), Box<dyn std::error::Error>>`

# Calls

- [compute_placeholder_size_for_series](../../../functions/src/placeholder_image/compute_placeholder_size_for_series.md)
- [contrasting_color](../../../functions/src/placeholder_image/contrasting_color.md)
- [is_color_dark](../../../functions/src/placeholder_image/is_color_dark.md)
- [draw_text](../../../functions/src/placeholder_image/draw_text.md)

# Called by

- [spec_placeholder_inverts_for_dark_stamp_background](../../../functions/tests/placeholder_image_inversion_spec/spec_placeholder_inverts_for_dark_stamp_background.md)
- [placeholder_series_size_matches_hint](../../../functions/tests/placeholder_image_series_size/placeholder_series_size_matches_hint.md)