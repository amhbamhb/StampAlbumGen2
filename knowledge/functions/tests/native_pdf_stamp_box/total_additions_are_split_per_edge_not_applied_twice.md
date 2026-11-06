---
type: Rust Function
title: total_additions_are_split_per_edge_not_applied_twice
resource: tests/native_pdf_stamp_box.rs#L17-L33
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

`fn total_additions_are_split_per_edge_not_applied_twice() -> Result<(), String>`

# Calls

- [from_total_additions](../../../functions/src/native_pdf/stamp_box/StampBoxGeometry/from_total_additions.md)
- [frame_bounds](../../../functions/src/native_pdf/stamp_box/StampBoxGeometry/frame_bounds.md)
- [assert_close](../../../functions/tests/native_pdf_stamp_box/assert_close.md)
- [horizontal_clearance_mm](../../../functions/src/native_pdf/stamp_box/StampBoxGeometry/horizontal_clearance_mm.md)
- [vertical_clearance_mm](../../../functions/src/native_pdf/stamp_box/StampBoxGeometry/vertical_clearance_mm.md)