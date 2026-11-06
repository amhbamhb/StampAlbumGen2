---
type: Rust Method
title: capture_generation_date
resource: src/native_pdf/document.rs#L159-L172
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/native_pdf/config/NativePdfConfig/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/renderer/write_page_geometry_proof
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/renderer/write_stamp_image_proof
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/renderer/write_stamp_box_geometry_proof
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn capture_generation_date(&self) -> Option<chrono::NaiveDate>`

# Called by

- [parse](../../../../../functions/src/native_pdf/config/NativePdfConfig/parse.md)
- [write_page_geometry_proof](../../../../../functions/src/native_pdf/renderer/write_page_geometry_proof.md)
- [write_stamp_image_proof](../../../../../functions/src/native_pdf/renderer/write_stamp_image_proof.md)
- [write_stamp_box_geometry_proof](../../../../../functions/src/native_pdf/renderer/write_stamp_box_geometry_proof.md)