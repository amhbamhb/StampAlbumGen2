---
type: Rust Function
title: rendered_paths_match_margin_and_spacing_fixtures
resource: tests/native_pdf_page_settings.rs#L231-L338
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_settings/args
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/spacing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_spacing/Spacing/horizontal_mm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/units/mm_to_points
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/units/points_to_mm
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn rendered_paths_match_margin_and_spacing_fixtures() -> Result<()>`

# Calls

- [as_str](../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [args](../../../functions/tests/native_pdf_page_settings/args.md)
- [spacing](../../../functions/src/native_pdf/document/PageSettings/spacing.md)
- [horizontal_mm](../../../functions/src/native_pdf/commands/page/album_pages_spacing/Spacing/horizontal_mm.md)
- [mm_to_points](../../../functions/src/native_pdf/units/mm_to_points.md)
- [points_to_mm](../../../functions/src/native_pdf/units/points_to_mm.md)