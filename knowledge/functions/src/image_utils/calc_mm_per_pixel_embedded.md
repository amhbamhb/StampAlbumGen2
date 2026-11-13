---
type: Rust Function
title: calc_mm_per_pixel_embedded
resource: src/image_utils.rs#L65-L76
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn_prepared
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_utils/dimensions/calculate_stamp_dimensions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/embedded_reference_scale_matches_great_britain_albumeasy_footprints
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/placeholder_image/compute_placeholder_size_for_series
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn calc_mm_per_pixel_embedded(real_width_mm: f32, real_height_mm: f32) -> Option<(f32, f32)>`

# Called by

- [generate_album_output_conn_prepared](../../../functions/src/album_globals/generator/generate_album_output_conn_prepared.md)
- [calculate_stamp_dimensions](../../../functions/src/album_utils/dimensions/calculate_stamp_dimensions.md)
- [embedded_reference_scale_matches_great_britain_albumeasy_footprints](../../../functions/src/native_pdf/stamp_image/embedded_reference_scale_matches_great_britain_albumeasy_footprints.md)
- [compute_placeholder_size_for_series](../../../functions/src/placeholder_image/compute_placeholder_size_for_series.md)