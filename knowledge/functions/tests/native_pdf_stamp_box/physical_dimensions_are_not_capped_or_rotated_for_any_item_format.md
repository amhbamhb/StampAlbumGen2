---
type: Rust Function
title: physical_dimensions_are_not_capped_or_rotated_for_any_item_format
resource: tests/native_pdf_stamp_box.rs#L36-L59
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/stamp_box/StampBoxGeometry/from_total_additions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_box/StampBoxGeometry/frame_bounds
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_stamp_box/assert_close
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_box/StampBoxGeometry/horizontal_clearance_mm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_box/StampBoxGeometry/vertical_clearance_mm
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn physical_dimensions_are_not_capped_or_rotated_for_any_item_format() -> Result<(), String>`

# Calls

- [from_total_additions](../../../functions/src/native_pdf/stamp_box/StampBoxGeometry/from_total_additions.md)
- [frame_bounds](../../../functions/src/native_pdf/stamp_box/StampBoxGeometry/frame_bounds.md)
- [assert_close](../../../functions/tests/native_pdf_stamp_box/assert_close.md)
- [horizontal_clearance_mm](../../../functions/src/native_pdf/stamp_box/StampBoxGeometry/horizontal_clearance_mm.md)
- [vertical_clearance_mm](../../../functions/src/native_pdf/stamp_box/StampBoxGeometry/vertical_clearance_mm.md)