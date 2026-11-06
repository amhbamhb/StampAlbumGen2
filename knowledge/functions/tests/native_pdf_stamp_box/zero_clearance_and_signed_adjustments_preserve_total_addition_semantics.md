---
type: Rust Function
title: zero_clearance_and_signed_adjustments_preserve_total_addition_semantics
resource: tests/native_pdf_stamp_box.rs#L62-L79
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/stamp_box/StampBoxGeometry/from_total_additions
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

`fn zero_clearance_and_signed_adjustments_preserve_total_addition_semantics() -> Result<(), String>`

# Calls

- [from_total_additions](../../../functions/src/native_pdf/stamp_box/StampBoxGeometry/from_total_additions.md)
- [assert_close](../../../functions/tests/native_pdf_stamp_box/assert_close.md)
- [horizontal_clearance_mm](../../../functions/src/native_pdf/stamp_box/StampBoxGeometry/horizontal_clearance_mm.md)
- [vertical_clearance_mm](../../../functions/src/native_pdf/stamp_box/StampBoxGeometry/vertical_clearance_mm.md)