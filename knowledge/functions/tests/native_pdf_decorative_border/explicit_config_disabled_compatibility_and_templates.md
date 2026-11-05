---
type: Rust Function
title: explicit_config_disabled_compatibility_and_templates
resource: tests/native_pdf_decorative_border.rs#L348-L429
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/config/NativePdfConfig/from_toml_in
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/err
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/config/NativePdfConfig/from_toml
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn explicit_config_disabled_compatibility_and_templates() -> Result<()>`

# Calls

- [from_toml_in](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml_in.md)
- [err](../../../functions/src/external_album/err.md)
- [from_toml](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml.md)