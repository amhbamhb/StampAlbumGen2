---
type: Rust Function
title: write_stamp_box_geometry_proof
resource: src/native_pdf/renderer.rs#L53-L72
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/stamp_image/StampImageProofSettings/with_stamp_box
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/renderer/write_proof
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/capture_generation_date
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/native_pdf_stamp_image/illustration_scale_changes_pdf_image_but_not_known_size_or_frame
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn write_stamp_box_geometry_proof( settings: &PageSettings, proof: &GeometryProofSettings, stamp: &StampImageProofSettings, geometry: &StampBoxGeometry, output: &Path, ) -> Result<()>`

# Calls

- [with_stamp_box](../../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/with_stamp_box.md)
- [write_proof](../../../../functions/src/native_pdf/renderer/write_proof.md)
- [capture_generation_date](../../../../functions/src/native_pdf/document/PageSettings/capture_generation_date.md)

# Called by

- [illustration_scale_changes_pdf_image_but_not_known_size_or_frame](../../../../functions/tests/native_pdf_stamp_image/illustration_scale_changes_pdf_image_but_not_known_size_or_frame.md)