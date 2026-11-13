---
type: Rust Function
title: explicit_file_changes_preserve_precision_even_margins_and_native_output
resource: tests/native_pdf_config.rs#L79-L116
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/commands/page/album_pages_size/PageSize/width_mm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_size/PageSize/height_mm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/spacing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_spacing/Spacing/next_top
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn explicit_file_changes_preserve_precision_even_margins_and_native_output() -> Result<()>`

# Calls

- [width_mm](../../../functions/src/native_pdf/commands/page/album_pages_size/PageSize/width_mm.md)
- [height_mm](../../../functions/src/native_pdf/commands/page/album_pages_size/PageSize/height_mm.md)
- [spacing](../../../functions/src/native_pdf/document/PageSettings/spacing.md)
- [next_top](../../../functions/src/native_pdf/commands/page/album_pages_spacing/Spacing/next_top.md)
- [open](../../../functions/src/external_album/open.md)