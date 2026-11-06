---
type: Rust Function
title: validate_bounds
resource: src/native_pdf/stamp_box.rs#L81-L99
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/document/Bounds/inset
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/units/mm_to_points
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/native_pdf/stamp_box/StampBoxGeometry/from_total_additions
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn validate_bounds(bounds: Bounds) -> Result<()>`

# Calls

- [inset](../../../../functions/src/native_pdf/document/Bounds/inset.md)
- [mm_to_points](../../../../functions/src/native_pdf/units/mm_to_points.md)

# Called by

- [from_total_additions](../../../../functions/src/native_pdf/stamp_box/StampBoxGeometry/from_total_additions.md)