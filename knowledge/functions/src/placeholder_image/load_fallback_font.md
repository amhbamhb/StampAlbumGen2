---
type: Rust Function
title: load_fallback_font
resource: src/placeholder_image.rs#L216-L269
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/placeholder_image/draw_text
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn load_fallback_font(font_override: Option<&str>) -> Option<FontArc>`

# Calls

- [open](../../../functions/src/external_album/open.md)

# Called by

- [draw_text](../../../functions/src/placeholder_image/draw_text.md)