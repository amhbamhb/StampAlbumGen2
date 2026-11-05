---
type: Rust Function
title: multiline_padding_and_minimum_hint_share_resolved_measurements
resource: tests/native_pdf_page_header.rs#L88-L117
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/config/NativePdfConfig/from_toml
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/units/points_to_mm
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn multiline_padding_and_minimum_hint_share_resolved_measurements() -> Result<()>`

# Calls

- [from_toml](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml.md)
- [points_to_mm](../../../functions/src/native_pdf/units/points_to_mm.md)