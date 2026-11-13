---
type: Rust Method
title: with_stamp_box
resource: src/native_pdf/stamp_image.rs#L58-L63
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/stamp_box/StampBoxGeometry/frame_bounds
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_box/StampBoxGeometry/horizontal_clearance_mm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_box/StampBoxGeometry/vertical_clearance_mm
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/native_pdf/renderer/write_stamp_box_geometry_proof
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_stamp_image/illustration_scale_changes_pdf_image_but_not_known_size_or_frame
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_stamp_box(mut self, geometry: &StampBoxGeometry) -> Self`

# Calls

- [frame_bounds](../../../../../functions/src/native_pdf/stamp_box/StampBoxGeometry/frame_bounds.md)
- [horizontal_clearance_mm](../../../../../functions/src/native_pdf/stamp_box/StampBoxGeometry/horizontal_clearance_mm.md)
- [vertical_clearance_mm](../../../../../functions/src/native_pdf/stamp_box/StampBoxGeometry/vertical_clearance_mm.md)

# Called by

- [write_stamp_box_geometry_proof](../../../../../functions/src/native_pdf/renderer/write_stamp_box_geometry_proof.md)
- [illustration_scale_changes_pdf_image_but_not_known_size_or_frame](../../../../../functions/tests/native_pdf_stamp_image/illustration_scale_changes_pdf_image_but_not_known_size_or_frame.md)