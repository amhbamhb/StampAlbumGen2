---
type: Rust Method
title: from_total_additions
resource: src/native_pdf/stamp_box.rs#L17-L62
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/stamp_box/validate_bounds
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/units/mm_to_points
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/native_pdf_stamp_box/total_additions_are_split_per_edge_not_applied_twice
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_stamp_box/physical_dimensions_are_not_capped_or_rotated_for_any_item_format
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_stamp_box/zero_clearance_and_signed_adjustments_preserve_total_addition_semantics
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_stamp_box/invalid_sizes_and_frames_that_would_shrink_known_stamps_fail_explicitly
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_stamp_image/illustration_scale_changes_pdf_image_but_not_known_size_or_frame
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn from_total_additions( physical_bounds: Bounds, extra_width_mm: f64, extra_height_mm: f64, stamp_box_adjust_mm: f64, ) -> Result<Self>`

# Calls

- [validate_bounds](../../../../../functions/src/native_pdf/stamp_box/validate_bounds.md)
- [mm_to_points](../../../../../functions/src/native_pdf/units/mm_to_points.md)

# Called by

- [total_additions_are_split_per_edge_not_applied_twice](../../../../../functions/tests/native_pdf_stamp_box/total_additions_are_split_per_edge_not_applied_twice.md)
- [physical_dimensions_are_not_capped_or_rotated_for_any_item_format](../../../../../functions/tests/native_pdf_stamp_box/physical_dimensions_are_not_capped_or_rotated_for_any_item_format.md)
- [zero_clearance_and_signed_adjustments_preserve_total_addition_semantics](../../../../../functions/tests/native_pdf_stamp_box/zero_clearance_and_signed_adjustments_preserve_total_addition_semantics.md)
- [invalid_sizes_and_frames_that_would_shrink_known_stamps_fail_explicitly](../../../../../functions/tests/native_pdf_stamp_box/invalid_sizes_and_frames_that_would_shrink_known_stamps_fail_explicitly.md)
- [illustration_scale_changes_pdf_image_but_not_known_size_or_frame](../../../../../functions/tests/native_pdf_stamp_image/illustration_scale_changes_pdf_image_but_not_known_size_or_frame.md)