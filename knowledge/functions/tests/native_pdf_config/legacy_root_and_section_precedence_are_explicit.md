---
type: Rust Function
title: legacy_root_and_section_precedence_are_explicit
resource: tests/native_pdf_config.rs#L119-L138
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
---

# Signature

`fn legacy_root_and_section_precedence_are_explicit() -> Result<()>`

# Calls

- [from_toml](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml.md)
- [width_mm](../../../functions/src/native_pdf/commands/page/album_pages_size/PageSize/width_mm.md)