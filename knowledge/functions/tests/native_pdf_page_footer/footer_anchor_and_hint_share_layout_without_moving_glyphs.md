---
type: Rust Function
title: footer_anchor_and_hint_share_layout_without_moving_glyphs
resource: tests/native_pdf_page_footer.rs#L64-L102
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

`fn footer_anchor_and_hint_share_layout_without_moving_glyphs() -> Result<()>`

# Calls

- [from_toml](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml.md)
- [points_to_mm](../../../functions/src/native_pdf/units/points_to_mm.md)