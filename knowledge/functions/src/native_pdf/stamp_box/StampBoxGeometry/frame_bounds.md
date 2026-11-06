---
type: Rust Method
title: frame_bounds
resource: src/native_pdf/stamp_box.rs#L68-L70
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/native_pdf/renderer/write_proof
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/StampImageProofSettings/with_stamp_box
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_stamp_box/total_additions_are_split_per_edge_not_applied_twice
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_stamp_box/physical_dimensions_are_not_capped_or_rotated_for_any_item_format
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn frame_bounds(&self) -> Bounds`

# Called by

- [write_proof](../../../../../functions/src/native_pdf/renderer/write_proof.md)
- [with_stamp_box](../../../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/with_stamp_box.md)
- [total_additions_are_split_per_edge_not_applied_twice](../../../../../functions/tests/native_pdf_stamp_box/total_additions_are_split_per_edge_not_applied_twice.md)
- [physical_dimensions_are_not_capped_or_rotated_for_any_item_format](../../../../../functions/tests/native_pdf_stamp_box/physical_dimensions_are_not_capped_or_rotated_for_any_item_format.md)