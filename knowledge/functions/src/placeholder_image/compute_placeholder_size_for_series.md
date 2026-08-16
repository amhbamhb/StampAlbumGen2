---
type: Rust Function
title: compute_placeholder_size_for_series
resource: src/placeholder_image.rs#L55-L90
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/load_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/image_utils/calc_mm_per_pixel_embedded
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/sample_images/get_scaling_for_series
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/placeholder_image/generate_placeholder_for_stamp_with_bg
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/placeholder_image/ensure_placeholder_image_for_stamp_with_hint
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/placeholder_image_series_size/placeholder_series_size_matches_hint
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn compute_placeholder_size_for_series(series: &str) -> (u32, u32)`

# Calls

- [load_config](../../../functions/src/config/load_config.md)
- [calc_mm_per_pixel_embedded](../../../functions/src/image_utils/calc_mm_per_pixel_embedded.md)
- [get_scaling_for_series](../../../functions/src/sample_images/get_scaling_for_series.md)

# Called by

- [generate_placeholder_for_stamp_with_bg](../../../functions/src/placeholder_image/generate_placeholder_for_stamp_with_bg.md)
- [ensure_placeholder_image_for_stamp_with_hint](../../../functions/src/placeholder_image/ensure_placeholder_image_for_stamp_with_hint.md)
- [placeholder_series_size_matches_hint](../../../functions/tests/placeholder_image_series_size/placeholder_series_size_matches_hint.md)