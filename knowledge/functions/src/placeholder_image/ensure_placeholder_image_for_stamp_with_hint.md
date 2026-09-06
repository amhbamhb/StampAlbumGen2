---
type: Rust Function
title: ensure_placeholder_image_for_stamp_with_hint
resource: src/placeholder_image.rs#L610-L811
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/placeholder_image/compute_placeholder_size_for_series
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/load_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/placeholder_image/is_color_dark
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/placeholder_image/draw_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/placeholder_image/generate_placeholder_image_with_options
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator_new/helpers/estimate_and_ensure_placeholders
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/ensure_placeholder_for_stamp_and_update_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/placeholder_regeneration/placeholder_regeneration_forced
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn ensure_placeholder_image_for_stamp_with_hint( opts: &PlaceholderImageOptions, ) -> Result<String, Box<dyn std::error::Error>>`

# Calls

- [compute_placeholder_size_for_series](../../../functions/src/placeholder_image/compute_placeholder_size_for_series.md)
- [load_config](../../../functions/src/config/load_config.md)
- [is_color_dark](../../../functions/src/placeholder_image/is_color_dark.md)
- [draw_text](../../../functions/src/placeholder_image/draw_text.md)
- [generate_placeholder_image_with_options](../../../functions/src/placeholder_image/generate_placeholder_image_with_options.md)

# Called by

- [estimate_and_ensure_placeholders](../../../functions/src/album_globals/generator_new/helpers/estimate_and_ensure_placeholders.md)
- [ensure_placeholder_for_stamp_and_update_db](../../../functions/src/album_globals/generator_new/helpers/ensure_placeholder_for_stamp_and_update_db.md)
- [placeholder_regeneration_forced](../../../functions/tests/placeholder_regeneration/placeholder_regeneration_forced.md)