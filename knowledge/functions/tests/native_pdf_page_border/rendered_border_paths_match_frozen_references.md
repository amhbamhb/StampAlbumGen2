---
type: Rust Function
title: rendered_border_paths_match_frozen_references
resource: tests/native_pdf_page_border.rs#L211-L272
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/units/points_to_mm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_border/path_bounds
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn rendered_border_paths_match_frozen_references() -> Result<()>`

# Calls

- [as_str](../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [points_to_mm](../../../functions/src/native_pdf/units/points_to_mm.md)
- [path_bounds](../../../functions/tests/native_pdf_page_border/path_bounds.md)