---
type: Rust Method
title: validate_numbers
resource: src/native_pdf/stamp_image.rs#L65-L94
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/units/mm_to_points
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/Bounds/inset
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn validate_numbers(&self) -> Result<()>`

# Calls

- [mm_to_points](../../../../../functions/src/native_pdf/units/mm_to_points.md)
- [inset](../../../../../functions/src/native_pdf/document/Bounds/inset.md)

# Called by

- [prepare](../../../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare.md)