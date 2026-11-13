---
type: Rust Method
title: set_header
resource: src/native_pdf/document.rs#L145-L147
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/native_pdf/config/NativePdfConfig/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_number/late_width_collision_preflight_preserves_existing_output
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn set_header(&mut self, header: AlbumPagesHeader)`

# Called by

- [parse](../../../../../functions/src/native_pdf/config/NativePdfConfig/parse.md)
- [late_width_collision_preflight_preserves_existing_output](../../../../../functions/tests/native_pdf_header_number/late_width_collision_preflight_preserves_existing_output.md)