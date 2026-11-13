---
type: Rust Function
title: title_pdf_positions_match_frozen_references_and_hint_controls_guide
resource: tests/native_pdf_page_title.rs#L331-L463
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
  - target: functions/src/album_globals/selection/count
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn title_pdf_positions_match_frozen_references_and_hint_controls_guide() -> Result<()>`

# Calls

- [from_toml](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml.md)
- [args](../../../functions/tests/native_pdf_page_settings/args.md)
- [count](../../../functions/src/album_globals/selection/count.md)
- [open](../../../functions/src/external_album/open.md)