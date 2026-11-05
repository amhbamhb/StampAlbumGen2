---
type: Rust Function
title: toml_defaults_and_independent_border_colours_are_explicit
resource: tests/native_pdf_explicit_settings.rs#L39-L127
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
  - target: functions/src/native_pdf/document/PageSettings/set_border_colour
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn toml_defaults_and_independent_border_colours_are_explicit() -> Result<()>`

# Calls

- [from_toml](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml.md)
- [err](../../../functions/src/external_album/err.md)
- [set_border_colour](../../../functions/src/native_pdf/document/PageSettings/set_border_colour.md)