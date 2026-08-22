---
type: Rust Function
title: estimate_real_dimensions
resource: src/image_utils.rs#L92-L105
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/image_utils/get_image_dimensions
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/estimate_and_ensure_placeholders
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_utils/dimensions/calculate_stamp_dimensions
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn estimate_real_dimensions( image_path: &str, mm_per_pixel_w: f32, mm_per_pixel_h: f32, ) -> Option<(f32, f32)>`

# Calls

- [get_image_dimensions](../../../functions/src/image_utils/get_image_dimensions.md)

# Called by

- [generate_album_output_conn](../../../functions/src/album_globals/generator/generate_album_output_conn.md)
- [estimate_and_ensure_placeholders](../../../functions/src/album_globals/generator_new/helpers/estimate_and_ensure_placeholders.md)
- [calculate_stamp_dimensions](../../../functions/src/album_utils/dimensions/calculate_stamp_dimensions.md)