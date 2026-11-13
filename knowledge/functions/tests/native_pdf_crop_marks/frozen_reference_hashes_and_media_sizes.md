---
type: Rust Function
title: frozen_reference_hashes_and_media_sizes
resource: tests/native_pdf_crop_marks.rs#L289-L311
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/digest
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_crop_marks/cases
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/units/points_to_mm
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn frozen_reference_hashes_and_media_sizes() -> Result<()>`

# Calls

- [digest](../../../functions/src/external_album/digest.md)
- [cases](../../../functions/tests/native_pdf_crop_marks/cases.md)
- [open](../../../functions/src/external_album/open.md)
- [points_to_mm](../../../functions/src/native_pdf/units/points_to_mm.md)