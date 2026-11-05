---
type: Rust Function
title: user_config_controls_actual_rendered_paths
resource: tests/native_pdf_config.rs#L259-L338
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/config/NativePdfConfig/from_toml
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_settings/args
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn user_config_controls_actual_rendered_paths() -> Result<()>`

# Calls

- [from_toml](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml.md)
- [args](../../../functions/tests/native_pdf_page_settings/args.md)