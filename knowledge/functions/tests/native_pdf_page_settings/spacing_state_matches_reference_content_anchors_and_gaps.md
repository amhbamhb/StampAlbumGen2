---
type: Rust Function
title: spacing_state_matches_reference_content_anchors_and_gaps
resource: tests/native_pdf_page_settings.rs#L127-L164
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
  - target: functions/src/native_pdf/document/PageSettings/spacing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_spacing/Spacing/vertical_mm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_spacing/Spacing/next_top
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn spacing_state_matches_reference_content_anchors_and_gaps() -> Result<()>`

# Calls

- [as_str](../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [points_to_mm](../../../functions/src/native_pdf/units/points_to_mm.md)
- [spacing](../../../functions/src/native_pdf/document/PageSettings/spacing.md)
- [vertical_mm](../../../functions/src/native_pdf/commands/page/album_pages_spacing/Spacing/vertical_mm.md)
- [next_top](../../../functions/src/native_pdf/commands/page/album_pages_spacing/Spacing/next_top.md)