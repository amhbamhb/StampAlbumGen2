---
type: Rust Function
title: position_text
resource: src/native_pdf/stamp_image.rs#L197-L260
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/stamp_image/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn position_text( settings: &StampTextProofSettings, bounds: Bounds, gap_mm: f64, above: bool, page: &PageLayout, ) -> Result<PreparedStampText>`

# Calls

- [contains](../../../../functions/src/native_pdf/stamp_image/contains.md)

# Called by

- [prepare](../../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare.md)