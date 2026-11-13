---
type: Rust Function
title: copied_template_values_reach_every_existing_command
resource: tests/native_pdf_config.rs#L47-L76
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/config/NativePdfConfig/from_toml
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_size/PageSize/width_mm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_size/PageSize/height_mm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/spacing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_spacing/Spacing/horizontal_mm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_spacing/Spacing/vertical_mm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_border/PageBorder/occupancy_mm
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn copied_template_values_reach_every_existing_command() -> Result<()>`

# Calls

- [from_toml](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml.md)
- [width_mm](../../../functions/src/native_pdf/commands/page/album_pages_size/PageSize/width_mm.md)
- [height_mm](../../../functions/src/native_pdf/commands/page/album_pages_size/PageSize/height_mm.md)
- [spacing](../../../functions/src/native_pdf/document/PageSettings/spacing.md)
- [horizontal_mm](../../../functions/src/native_pdf/commands/page/album_pages_spacing/Spacing/horizontal_mm.md)
- [vertical_mm](../../../functions/src/native_pdf/commands/page/album_pages_spacing/Spacing/vertical_mm.md)
- [occupancy_mm](../../../functions/src/native_pdf/commands/page/album_pages_border/PageBorder/occupancy_mm.md)