---
type: Rust Function
title: config_colour_is_applied_with_section_precedence_and_no_geometry_changes
resource: tests/native_pdf_border_colour.rs#L145-L197
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/config/NativePdfConfig/from_toml
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/err
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn config_colour_is_applied_with_section_precedence_and_no_geometry_changes() -> Result<()>`

# Calls

- [from_toml](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml.md)
- [err](../../../functions/src/external_album/err.md)