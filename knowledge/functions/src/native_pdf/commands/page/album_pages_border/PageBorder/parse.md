---
type: Rust Method
title: parse
resource: src/native_pdf/commands/page/album_pages_border.rs#L25-L46
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/commands/arguments
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/units/mm_to_points
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn parse(arguments_mm: &[f64]) -> Result<Self>`

# Calls

- [arguments](../../../../../../../functions/src/native_pdf/commands/arguments.md)
- [mm_to_points](../../../../../../../functions/src/native_pdf/units/mm_to_points.md)