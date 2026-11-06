---
type: Rust Function
title: illustration_scale_changes_pdf_image_but_not_known_size_or_frame
resource: tests/native_pdf_stamp_image.rs#L139-L215
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/external_album/save
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_box/StampBoxGeometry/from_total_additions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/units/mm_to_points
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/StampImageProofSettings/with_stamp_box
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/renderer/write_stamp_box_geometry_proof
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_stamp_image/proof
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/selection/count
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/units/points_to_mm
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn illustration_scale_changes_pdf_image_but_not_known_size_or_frame() -> Result<()>`

# Calls

- [save](../../../functions/src/review_ui/views/external_album/save.md)
- [from_total_additions](../../../functions/src/native_pdf/stamp_box/StampBoxGeometry/from_total_additions.md)
- [mm_to_points](../../../functions/src/native_pdf/units/mm_to_points.md)
- [with_stamp_box](../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/with_stamp_box.md)
- [write_stamp_box_geometry_proof](../../../functions/src/native_pdf/renderer/write_stamp_box_geometry_proof.md)
- [proof](../../../functions/tests/native_pdf_stamp_image/proof.md)
- [open](../../../functions/src/external_album/open.md)
- [count](../../../functions/src/album_globals/selection/count.md)
- [points_to_mm](../../../functions/src/native_pdf/units/points_to_mm.md)