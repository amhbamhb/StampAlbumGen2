---
type: Rust Function
title: four_page_sizes_render_and_read_back
resource: tests/native_pdf_page_settings.rs#L66-L95
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/wikipedia/models/ContentSource/as_str
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

`fn four_page_sizes_render_and_read_back() -> Result<()>`

# Calls

- [as_str](../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [open](../../../functions/src/external_album/open.md)
- [points_to_mm](../../../functions/src/native_pdf/units/points_to_mm.md)