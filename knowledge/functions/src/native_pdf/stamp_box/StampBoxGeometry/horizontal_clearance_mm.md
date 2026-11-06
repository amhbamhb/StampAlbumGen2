---
type: Rust Method
title: horizontal_clearance_mm
resource: src/native_pdf/stamp_box.rs#L72-L74
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/native_pdf/stamp_image/StampImageProofSettings/with_stamp_box
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_stamp_box/total_additions_are_split_per_edge_not_applied_twice
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_stamp_box/physical_dimensions_are_not_capped_or_rotated_for_any_item_format
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_stamp_box/zero_clearance_and_signed_adjustments_preserve_total_addition_semantics
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn horizontal_clearance_mm(&self) -> f64`

# Called by

- [with_stamp_box](../../../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/with_stamp_box.md)
- [total_additions_are_split_per_edge_not_applied_twice](../../../../../functions/tests/native_pdf_stamp_box/total_additions_are_split_per_edge_not_applied_twice.md)
- [physical_dimensions_are_not_capped_or_rotated_for_any_item_format](../../../../../functions/tests/native_pdf_stamp_box/physical_dimensions_are_not_capped_or_rotated_for_any_item_format.md)
- [zero_clearance_and_signed_adjustments_preserve_total_addition_semantics](../../../../../functions/tests/native_pdf_stamp_box/zero_clearance_and_signed_adjustments_preserve_total_addition_semantics.md)